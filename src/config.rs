use crate::api_handler::*;
use dirs_next::config_dir;
use serde::{Deserialize, Serialize};
use std::{
    fs::{
        OpenOptions,
        create_dir_all,
    },
    path::PathBuf,
    io::{
        prelude::*,
        ErrorKind,
    },
};

const CONFIG_DIR: &str = "syncthing_status";
const CONFIG_FILE: &str = "devices.yml";

#[derive(Serialize, Deserialize, Eq, PartialEq)]
pub(crate) struct Config {
    devices: Vec<Device>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            devices: vec![Device::default()],
        }
    }
}

#[derive(Serialize, Deserialize, Eq, PartialEq)]
pub(crate) struct Device {
    pub url: String,
    pub name: String,
    pub short_name: String,
    pub api_key: String,
}

impl Default for Device {
    fn default() -> Self {
        let url = "https://localhost:8384".to_string();
        let name = "Laptop".to_string();
        let short_name = "L".to_string();
        let api_key = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA".to_string();

        Self {
            url,
            name,
            short_name,
            api_key,
        }
    }
}

impl Config {
    pub(crate) fn load() -> Self {
        get_devices()
    }
}

/* pub fn get_devices() -> Vec<Device> {
 *     // create folder
 *     let folder = config_dir()
 *         .unwrap()
 *         .join("syncthing_status");
 *
 *     let _ = create_dir_all(&folder);
 *
 *     let path = folder.join("devices").with_extension("yml");
 *
 *     // open file
 *     let mut file = OpenOptions::new()
 *         .read(true)
 *         .write(true)
 *         .open(path);
 *
 *     match file {
 *         Ok(file) => {}
 *         Err(error) => {
 *             match error.kind() {
 *                 ErrorKind::NotFound => {
 *                     let mut file = OpenOptions::new()
 *                         .write(true)
 *                         .create(true)
 *                         .open(path)
 *                         .unwrap();
 *                 serde_yaml::to_string(&Device::default()).unwrap()
 *                 },
 *                 _ => {}
 *             }
 *         }
 *     }
 *
 *     let mut reader = String::new();
 *     file.read_to_string(&mut reader).unwrap();
 *     let devices: Vec<Device> = match serde_yaml::from_str(&reader) {
 *         Ok(d) => d,
 *         Err(e) => panic!("Could not parse device-file: {:?}", e),
 *     };
 *
 *     devices
 * } */

/// Read in config dir
pub(crate) fn get_devices() -> Config {

    let file_path = get_config_dir().join(CONFIG_FILE);

    let file_result = OpenOptions::new()
        .read(true)
        .open(file_path);

    // return content (or default on error)
    match file_result {
        Ok(mut file) => {
            let mut buffer = String::new();
            if let Err(e) = file.read_to_string(&mut buffer) {
                panic!("Data is no valid utf-8: {}", e);
            }

            return serde_yaml::from_str(&buffer).unwrap_or_default();
        }
        Err(error) => {
            match error.kind() {
                ErrorKind::NotFound => {
                    let devices = vec![Config::default()];
                    let string = serde_yaml::to_string(&devices).unwrap();
                    write_config(&string);
                    return Config { devices };
                },
                other_error => {
                    panic!("Could not read file: {:?}", other_error);
               },
            }
        },
    }
}

/// Write in config dir
fn write_config(content: &str) {
    let config_dir = get_config_dir();

    let file_path = config_dir.join(CONFIG_FILE);

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(file_path)
        .unwrap();

    let _ = file.write_all(content.as_bytes());
}

// private function to create and read config dir
fn get_config_dir() -> PathBuf {
    let path = config_dir().unwrap().join(CONFIG_DIR);
    if let Err(error) = create_dir_all(&path) {
        if error.kind() == ErrorKind::PermissionDenied {
                panic!("Permission to config dir denied");
        }
    }

    path
}
