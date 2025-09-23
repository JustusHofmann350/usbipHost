use std::process::{Command, Stdio};
use std::io::Read;

#[derive(Debug, Clone)]
pub struct UsbipDevice {
    pub busid: String,
    pub vidpid: String,
    pub device: String,
    pub state: String,
    pub persisted: bool,
}

pub fn list_devices() -> Vec<UsbipDevice> {
    let output = Command::new("usbipd")
        .args(&["list"])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .and_then(|mut child| {
            let mut out = String::new();
            if let Some(mut stdout) = child.stdout.take() {
                let _ = stdout.read_to_string(&mut out);
            }
            let _ = child.wait();
            Ok(out)
        }).unwrap_or_default();

    let mut out_devices = Vec::new();
    let mut section = "";
    for line in output.lines() {
        let l = line.trim();
        if l.is_empty() { continue; }
        if l.starts_with("Connected:") {
            section = "connected"; continue;
        }
        if l.starts_with("Persisted:") {
            section = "persisted"; continue;
        }
        if l.starts_with("BUSID") { continue; }
        let cols: Vec<&str> = l.split_whitespace().collect();
        if cols.len() < 4 { continue; }
        let dev = UsbipDevice {
            busid: cols[0].to_string(),
            vidpid: cols[1].to_string(),
            device: cols[2..cols.len()-1].join(" "),
            state: cols[cols.len()-1].to_string(),
            persisted: section == "persisted",
        };
        out_devices.push(dev);
    }
    out_devices
}
pub fn bind_device(busid: &str) -> Result<(), String> {
    let output = Command::new("usbipd")
        .args(&["bind", "-f", "-b", busid])
        .output()
        .map_err(|e| e.to_string())?;
    if output.status.success() { Ok(()) }
    else { Err(String::from_utf8_lossy(&output.stderr).into_owned()) }
}
pub fn unbind_device(busid: &str) -> Result<(), String> {
    let output = Command::new("usbipd")
        .args(&["unbind", "-b", busid])
        .output()
        .map_err(|e| e.to_string())?;
    if output.status.success() { Ok(()) }
    else { Err(String::from_utf8_lossy(&output.stderr).into_owned()) }
}