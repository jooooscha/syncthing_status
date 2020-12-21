use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use chrono::Local;

static CONNECTION_ERROR: &str = "connection error";
static PATH: &str = "/home/joscha/.config/polybar/scripts/syncthing_status/";

#[derive(Serialize, Eq, PartialEq)]
struct DeviceConfig {
    url: String,
    name: String,
    api_key: String
}

// folder deserialization
#[derive(Serialize, Deserialize)]
struct Config {
    folders: Vec<FolderName>,
}

#[derive(Serialize, Deserialize)]
struct FolderName {
    id: String,
    label: String,
}

#[derive(Serialize, Deserialize)]
struct FolderInfo {
    state: String,
    errors: u32,
}

#[derive(Serialize, Deserialize)]
struct Folder {
    id: String,
    label: String,
    state: String,
    errors: u32,
}

fn main() -> std::io::Result<()> {

    //define devices
    #[allow(non_snake_case)]
    let devices: [DeviceConfig; 2] = [
        DeviceConfig {
            url: String::from("http://127.0.0.1:8080"),
            name: String::from("laptop"),
            api_key: String::from("FLR6DH9pyGvMuweZics9LDL9Auwb3JRG"),
        },
        DeviceConfig {
            url: String::from("http://192.168.0.10:8384"),
            name: String::from("rasp"),
            api_key: String::from("2feNdfnPzCYRwCcy2cVXKVVA2VH9dubZ"),
        }
    ];

    // the output string
    let mut file_string = String::new();
    let mut bar_output = String::new();

    //iterate over devices
    for device in devices.iter() {

        let device_connected: bool;
        let long_string: String;
        let short_string: String;

        let is_last_item: bool = device == &devices[devices.len() -1];

        // fetch folders name/id
        let folders_name: HashMap<String, String> = match get_folder(&device) {
            Some(map) => {
                device_connected = true;
                map
            }
            None => {
                device_connected = false;
                HashMap::new()
            },
        };

        if device_connected {
            let mut folders: Vec<Folder> = Vec::new();

            for (id, label) in folders_name.into_iter() {

                // fetch folder information like state
                let f_info = match get_folder_info(id.clone(), &device) {
                    Some(data) => data,
                    None => continue,
                };

                // create Folder with all information combined
                let f = Folder {
                    id,
                    label,
                    state: f_info.state,
                    errors: f_info.errors,
                };

                folders.push(f);
            }

            //combine to one string
            let (formatted, short) = append_to_output_string(&folders, &device.name, is_last_item);

            long_string = formatted;
            short_string = format!("{}: {} - ", &device.name, short);

        } else {
            long_string = String::new();
            short_string = format!("{}: {} - ", &device.name, CONNECTION_ERROR.to_string());
        }

        file_string = file_string + &long_string;
        bar_output = bar_output + &short_string;
    }

    bar_output.truncate(bar_output.len() - 3);

    //write string and return `Result<>`
    println!("{}", bar_output);
    write_to_file(&file_string)
}

fn get_folder(device: &DeviceConfig) -> Option<HashMap<String, String>> {

    let mut folders: HashMap<String, String> = HashMap::new();

    let client = reqwest::blocking::Client::builder().build().ok()?;
    let body = client.get(&format!("{}{}", &device.url, "/rest/system/config"))
        .header("X-API-Key", &device.api_key)
        .send().ok()?
        .text().ok()?;

    let data: Config = serde_json::from_str(&body).unwrap();

    for f in data.folders.iter() {
        folders.insert(f.id.clone(), f.label.clone());
    }

    Some(folders)
}

fn get_folder_info(id: String, device: &DeviceConfig) -> Option<FolderInfo> {

    let client = reqwest::blocking::Client::builder().build().ok()?;
    let body = client.get(&format!("{}{}{}", &device.url, "/rest/db/status?folder=", &id))
        .header("X-API-Key", &device.api_key)
        .send().ok()?
        .text().ok()?;

    let data: FolderInfo = serde_json::from_str(&body).unwrap();

    Some(data)
}

fn append_to_output_string(folders: &Vec<Folder>, device_name: &String, is_last: bool) -> (String, String) {

    let mut string = format!("-------------------- {:-<25}\n", device_name.clone() + " ");

    let mut short_output = String::from("Up to Date");

    for f in folders.iter() {
        string = format!("{}{:>13}: {:<10} | Errors: {} \n",
            string,
            f.label,
            f.state,
            f.errors
        );

        if &f.state != "idle" {
            short_output = f.state.clone();
        }
    }

    if is_last {
        (string, short_output)
    } else {
        (string + "\n", short_output)
    }
}

fn write_to_file(string: &String) -> std::io::Result<()> {

    // write output content
    let mut file = File::create(PATH.to_owned() + "output")?;
    file.write_all(&string.as_bytes())?;

    //write date file
    let date = Local::now().format("Last Check: %A - %T");
    let mut file = File::create(PATH.to_owned() + "time")?;
    file.write_all(&date.to_string().as_bytes())?;

    Ok(())
}
