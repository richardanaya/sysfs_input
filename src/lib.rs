use std::fs;

#[derive(Debug)]
pub struct InputDevice {
    name: String
}

pub fn input_devices() -> Result<Vec<InputDevice>,std::io::Error> {
    let mut v = vec![];
    for entry in fs::read_dir("/sys/class/input/")? {
        let entry = entry?;
        v.push(InputDevice {
            name:entry.file_name().into_string().unwrap(),
        })
    }
    Ok(v)
}
