use std::fmt;

use anyhow::{Context, Result};

pub fn list_network_interfaces() -> Result<Vec<String>> {
    let dirs =
        std::fs::read_dir("/sys/class/net").context("Failed to read network interface list")?;

    Ok(dirs
        .filter_map(|dir| dir.ok())
        .filter_map(|dir| dir.file_name().into_string().ok())
        .collect())
}

#[derive(PartialEq, Eq, Debug)]
pub enum InterfaceType {
    Wireless,
    Ethernet,
    Loopback,
    Unknown,
}

impl fmt::Display for InterfaceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn get_interface_type(interface: &str) -> InterfaceType {
    if interface.starts_with("wlp") {
        InterfaceType::Wireless
    } else if interface.starts_with("eth") || interface.starts_with("enp") {
        InterfaceType::Ethernet
    } else if interface.starts_with("lo") {
        InterfaceType::Loopback
    } else {
        InterfaceType::Unknown
    }
}

/// wireless first, then ethernet, then rest
pub fn sort_interface_list(interfaces: &mut Vec<String>) {
    interfaces.sort_by(|a, b| {
        let a_type = get_interface_type(a);
        let b_type = get_interface_type(b);

        if a_type == b_type {
            a.cmp(b)
        } else {
            match (a_type, b_type) {
                (InterfaceType::Wireless, _) => std::cmp::Ordering::Less,
                (_, InterfaceType::Wireless) => std::cmp::Ordering::Greater,
                (InterfaceType::Ethernet, _) => std::cmp::Ordering::Less,
                (_, InterfaceType::Ethernet) => std::cmp::Ordering::Greater,
                (InterfaceType::Loopback, _) => std::cmp::Ordering::Less,
                (_, InterfaceType::Loopback) => std::cmp::Ordering::Greater,
                (InterfaceType::Unknown, _) => std::cmp::Ordering::Less,
            }
        }
    });
}

pub fn get_interface_speed(interface: &str) -> Result<u64> {
    let path = format!("/sys/class/net/{}/speed", interface);
    let speed = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read speed for interface {}", interface))?;

    Ok(speed.trim().parse::<u64>()?)
}

pub fn get_interface_rx_bits(interface: &str) -> Result<u64> {
    let path = format!("/sys/class/net/{}/statistics/rx_bytes", interface);
    let rx_bytes = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read rx_bytes for interface {}", interface))?;

    Ok(rx_bytes.trim().parse::<u64>()? * 8)
}

pub fn get_interface_tx_bits(interface: &str) -> Result<u64> {
    let path = format!("/sys/class/net/{}/statistics/tx_bytes", interface);
    let tx_bytes = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read tx_bytes for interface {}", interface))
        .unwrap();

    Ok(tx_bytes.trim().parse::<u64>()? * 8)
}

pub fn get_interface_packets(interface: &str) -> Result<u64> {
    let path = format!("/sys/class/net/{}/statistics/rx_packets", interface);
    let rx_packets = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read rx_packets for interface {}", interface))?;

    Ok(rx_packets.trim().parse::<u64>()?)
}

pub fn get_interface_address(interface: &str) -> Result<String> {
    let path = format!("/sys/class/net/{}/address", interface);
    let address = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read address for interface {}", interface))?;

    Ok(address.trim().to_string())
}

pub fn get_interface_rx_bytes_delta(interface: &str, duration: std::time::Duration) -> Result<u64> {
    let start = get_interface_rx_bits(interface)?;
    std::thread::sleep(duration);
    let end = get_interface_rx_bits(interface)?;

    Ok(end - start)
}

pub fn get_interface_tx_bytes_delta(interface: &str, duration: std::time::Duration) -> Result<u64> {
    let start = get_interface_tx_bits(interface)?;
    std::thread::sleep(duration);
    let end = get_interface_tx_bits(interface)?;

    Ok(end - start)
}
