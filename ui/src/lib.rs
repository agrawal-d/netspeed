extern crate dirs;
extern crate libnetspeed;
extern crate log;
extern crate simplelog;

use anyhow::Result;
use eframe::egui;
use libnetspeed::*;
use log::*;
use std::{
    sync::{Arc, Mutex, Once},
    time::{Duration, Instant},
};

// default string for unknown address
const UNKNOWN_ADDRESS: &str = "Unknown";
const UPDATE_UNTERVAL_MS: u64 = 1000;
static SPEED_ERROR: Once = Once::new();

struct MyApp {
    interfaces: Vec<String>,
    stats: Arc<Mutex<Stats>>,
}

impl Default for MyApp {
    fn default() -> Self {
        MyApp {
            interfaces: list_network_interfaces().unwrap_or_default(),
            stats: Arc::new(Mutex::new(Stats::default())),
        }
    }
}

struct Stats {
    selected: usize,
    prev_selected: usize,
    prev_rx_bytes: u64,
    prev_tx_bytes: u64,
    rx_bytes: u64,
    tx_bytes: u64,
    record_time: Instant,
    speed: u64,
    address: String,
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            selected: 0,
            prev_selected: 0,
            prev_rx_bytes: 0,
            prev_tx_bytes: 0,
            rx_bytes: 0,
            tx_bytes: 0,
            record_time: Instant::now(),
            speed: 0,
            address: String::from(UNKNOWN_ADDRESS),
        }
    }
}

fn update_stats(interfaces: &[String], stats: &mut Arc<Mutex<Stats>>) {
    let mut stats = stats.lock().unwrap();
    let interface = &interfaces[stats.selected];

    if stats.selected != stats.prev_selected {
        stats.prev_selected = stats.selected;
        stats.prev_rx_bytes = 0;
        stats.prev_tx_bytes = 0;
        stats.rx_bytes = 0;
        stats.tx_bytes = 0;
        info!("Changed selection");
    }

    let rx_bytes = get_interface_rx_bits(interface).unwrap_or_else(|err| {
        warn!(
            "Failed to get rx_bytes for interface {}: {}",
            interface, err
        );
        0
    });
    let tx_bytes = get_interface_tx_bits(interface).unwrap_or_else(|err| {
        warn!(
            "Failed to get tx_bytes for interface {}: {}",
            interface, err
        );
        0
    });

    let address = get_interface_address(interface).unwrap_or_else(|err| {
        warn!("Failed to get address for interface {}: {}", interface, err);
        String::from(UNKNOWN_ADDRESS)
    });
    let speed = get_interface_speed(interface).unwrap_or_else(|err| {
        SPEED_ERROR.call_once(|| {
            warn!("Failed to get speed for interface {}: {}", interface, err);
        });
        0
    });

    stats.prev_rx_bytes = if stats.rx_bytes == 0 {
        rx_bytes
    } else {
        stats.rx_bytes
    };
    stats.prev_tx_bytes = if stats.tx_bytes == 0 {
        tx_bytes
    } else {
        stats.tx_bytes
    };
    stats.rx_bytes = rx_bytes;
    stats.tx_bytes = tx_bytes;
    stats.record_time = Instant::now();
    stats.speed = speed;
    stats.address = address;
}

impl MyApp {
    fn stat_list(&self, ui: &mut egui::Ui, stats: &std::sync::MutexGuard<'_, Stats>) {
        let duration: Duration = Duration::from_millis(UPDATE_UNTERVAL_MS);
        ///////////////////////////////////////////////////

        ui.label("Type");
        ui.label(format!(
            "{}",
            get_interface_type(&self.interfaces[stats.selected])
        ));
        ui.end_row();

        ///////////////////////////////////////////////////
        info!("{}", (stats.rx_bytes - stats.prev_rx_bytes) / 1000);
        ui.label("Download speed");
        ui.label(get_rate(stats.rx_bytes - stats.prev_rx_bytes, duration).to_string());
        ui.end_row();

        ///////////////////////////////////////////////////

        ui.label("Upload Speed");
        ui.label(get_rate(stats.tx_bytes - stats.prev_tx_bytes, duration).to_string());
        ui.end_row();

        ///////////////////////////////////////////////////

        ui.label("Total Downloaded");
        ui.label(bytes_to_human_readable(stats.rx_bytes).to_string());
        ui.end_row();

        ///////////////////////////////////////////////////

        ui.label("Total Uploaded");
        ui.label(bytes_to_human_readable(stats.tx_bytes).to_string());
        ui.end_row();

        ///////////////////////////////////////////////////

        ui.label("Max speed");
        ui.label(
            (if stats.speed == 0 {
                String::from("Unknown")
            } else {
                get_rate(stats.speed, Duration::from_secs(1))
            })
            .to_string(),
        );
        ui.end_row();

        ///////////////////////////////////////////////////

        ui.label("Network Address");
        let mut addr: &str = stats.address.as_str();
        ui.add(egui::TextEdit::singleline(&mut addr));
        ui.end_row();

        ///////////////////////////////////////////////////
    }
}

/// Convert bytes to human readable format with value less than 100 ( upto Tb )
fn bytes_to_human_readable(bytes: u64) -> String {
    let units = ["b", "Kb", "Mb", "Gb"];
    let mut bytes = bytes as f64;
    let mut unit = 0;

    while bytes > 100.0 && unit < units.len() - 1 {
        bytes /= 1000.0;
        unit += 1;
    }

    format!("{:.2} {}", bytes, units[unit])
}

/// Get rate in human readable format
fn get_rate(bytes: u64, duration: Duration) -> String {
    if duration.as_millis() == 0 {
        return "0.00 b/s".to_string();
    }

    let bytes = bytes as f64;
    let duration = duration.as_millis() as f64;
    let bytes_per_second: f64 = (bytes * 1000.0) / duration;
    format!("{}/s", bytes_to_human_readable(bytes_per_second as u64))
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint_after(Duration::from_millis(UPDATE_UNTERVAL_MS));
        if self.interfaces.is_empty() {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.label("No network interfaces found");
            });
            return;
        }

        let mut stats = self.stats.lock().unwrap();

        if stats.selected != stats.prev_selected {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.label("Upating...");
            });

            return;
        }

        // Interface selection
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::ComboBox::from_label("Select a network")
                .selected_text(&self.interfaces[stats.selected])
                .show_ui(ui, |ui: &mut egui::Ui| {
                    for (i, interface) in self.interfaces.iter().enumerate() {
                        ui.selectable_value(&mut stats.selected, i, interface);
                    }
                });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Grid::new("my_grid")
                .num_columns(2)
                .min_col_width(100.0)
                .spacing([40.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    self.stat_list(ui, &stats);
                });
        });
    }
}

pub fn ui() -> Result<(), eframe::Error> {
    let update_interval = Duration::from_millis(UPDATE_UNTERVAL_MS);

    let app = Box::<MyApp>::default();
    let stats = app.stats.clone();
    let interfaces = app.interfaces.clone();

    std::thread::spawn(move || loop {
        update_stats(&interfaces, &mut stats.clone());
        std::thread::sleep(update_interval);
    });

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 400.0)),
        centered: true,
        ..Default::default()
    };

    eframe::run_native("NetSpeed", options, Box::new(|_| app))
}
