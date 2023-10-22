use anyhow::{Context, Result};

pub fn list_network_interfaces() -> Result<Vec<String>> {
    let dirs =
        std::fs::read_dir("/sys/class/net").context("Failed to read network interface list")?;

    Ok(dirs
        .filter_map(|dir| dir.ok())
        .filter_map(|dir| dir.file_name().into_string().ok())
        .collect())
}

pub fn list_wireless_interfaces() -> Result<Vec<String>> {
    let interfaces = list_network_interfaces()?;

    Ok(interfaces
        .into_iter()
        .filter(|interface| interface.starts_with("wlp"))
        .collect())
}

pub fn list_ethernet_interfaces() -> Result<Vec<String>> {
    let interfaces = list_network_interfaces()?;

    Ok(interfaces
        .into_iter()
        .filter(|interface| interface.starts_with("eth") || interface.starts_with("enp"))
        .collect())
}

pub fn get_interface_speed(interface: &str) -> Result<usize> {
    let path = format!("/sys/class/net/{}/speed", interface);
    let speed = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read speed for interface {}", interface))?;

    Ok(speed.trim().parse::<usize>()?)
}

pub fn get_interface_rx_bytes(interface: &str) -> Result<usize> {
    let path = format!("/sys/class/net/{}/statistics/rx_bytes", interface);
    let rx_bytes = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read rx_bytes for interface {}", interface))?;

    Ok(rx_bytes.trim().parse::<usize>()?)
}

pub fn get_interface_tx_bytes(interface: &str) -> Result<usize> {
    let path = format!("/sys/class/net/{}/statistics/tx_bytes", interface);
    let tx_bytes = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read tx_bytes for interface {}", interface))?;

    Ok(tx_bytes.trim().parse::<usize>()?)
}

pub fn get_interface_packets(interface: &str) -> Result<usize> {
    let path = format!("/sys/class/net/{}/statistics/rx_packets", interface);
    let rx_packets = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read rx_packets for interface {}", interface))?;

    Ok(rx_packets.trim().parse::<usize>()?)
}

pub fn get_interface_address(interface: &str) -> Result<String> {
    let path = format!("/sys/class/net/{}/address", interface);
    let address = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read address for interface {}", interface))?;

    Ok(address.trim().to_string())
}

pub fn get_interface_rx_bytes_delta(
    interface: &str,
    duration: std::time::Duration,
) -> Result<usize> {
    let start = get_interface_rx_bytes(interface)?;
    std::thread::sleep(duration);
    let end = get_interface_rx_bytes(interface)?;

    Ok(end - start)
}

pub fn get_interface_tx_bytes_delta(
    interface: &str,
    duration: std::time::Duration,
) -> Result<usize> {
    let start = get_interface_tx_bytes(interface)?;
    std::thread::sleep(duration);
    let end = get_interface_tx_bytes(interface)?;

    Ok(end - start)
}
