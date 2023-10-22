extern crate dirs;
extern crate log;
extern crate simplelog;

use anyhow::{Context, Result};
use eframe::egui;

struct MyApp {
    interface: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            interface: String::from("/dev"),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
        });
    }
}

pub fn ui() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 400.0)),
        ..Default::default()
    };

    eframe::run_native("NetSpeed", options, Box::new(|_| Box::<MyApp>::default()))
}
