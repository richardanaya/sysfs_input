use std::fs::{self, read_to_string};
use std::path::Path;

#[derive(Debug)]
pub struct InputCapability {
    name: String,
    raw_value: String,
}

#[derive(Debug)]
pub struct InputDevice {
    name: String,
    file_name: String,
    capabilities: Vec<InputCapability>,
    properties: String,
}

#[derive(Debug)]
pub struct EventDevice {
    file_name: String,
    device_name: String,
}

#[derive(Debug)]
pub enum Device {
    Input(InputDevice),
    Event(EventDevice),
}

fn read_first_line_of_file(path: &str) -> Result<String, std::io::Error> {
    let cap_value = read_to_string(path)?;
    let mut cap_lines = cap_value.lines();
    Ok(cap_lines.next().unwrap().to_string())
}

pub fn input_devices() -> Result<Vec<Device>, std::io::Error> {
    let mut v = vec![];
    for entry in fs::read_dir("/sys/class/input/")? {
        let entry = entry?;
        let name = entry.file_name().into_string().unwrap();
        if name.starts_with("input") {
            let mut caps = vec![];
            let caps_path = format!("/sys/class/input/{}/capabilities", name);
            if Path::new(&caps_path).exists() {
                for caps_entry in fs::read_dir(&caps_path)? {
                    let caps_entry = caps_entry?;
                    let cap_name = caps_entry.file_name().into_string().unwrap();

                    caps.push(InputCapability {
                        name: cap_name.clone(),
                        raw_value: read_first_line_of_file(&format!(
                            "/sys/class/input/{}/capabilities/{}",
                            name, cap_name
                        ))?,
                    });
                }
                v.push(Device::Input(InputDevice {
                    name: read_first_line_of_file(&format!("/sys/class/input/{}/name", name))?,
                    file_name: entry.file_name().into_string().unwrap(),
                    capabilities: caps,
                    properties: read_first_line_of_file(&format!(
                        "/sys/class/input/{}/properties",
                        name
                    ))?,
                }))
            }
        } else if name.starts_with("event") {
            let link = fs::read_link(&format!(
                "/sys/class/input/{}/device",
                name
            ))?;
            v.push(Device::Event(EventDevice {
                file_name: entry.file_name().into_string().unwrap(),
                device_name: link.file_name().unwrap().to_os_string().into_string().unwrap(),
            }))
        }
    }
    Ok(v)
}
