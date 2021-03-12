use crate::api_handler::*;
use dirs::home_dir;
use std::{fs::File, io::prelude::*};

const DEVICES_PATH: &str = ".config/syncthing_status/devices.yml";

pub fn get_devices() -> Vec<Device> {
    let mut path = home_dir().unwrap();
    path.push(DEVICES_PATH);

    match File::open(path) {
        Ok(mut file) => {
            let mut reader = String::new();
            file.read_to_string(&mut reader).unwrap();
            let devices: Vec<Device> = match serde_yaml::from_str(&reader) {
                Ok(d) => d,
                Err(_) => panic!("Could not read device-file"),
            };

            devices
        }
        Err(_) => panic!("Could not find device-file"),
    }
}
