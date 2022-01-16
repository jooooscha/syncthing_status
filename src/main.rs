use reqwest::{blocking::Client, Error};
use serde::{Deserialize, Serialize};

use config::Config;
use output::format_output;
use crate::api_handler::Folder;

mod api_handler;
mod config;
mod output;

#[derive(Default)]
pub(crate) struct Device {
    config: Config,
    folder: Vec<Folder>,
}

impl Device {
}

fn main() {
    /* let devices = get_devices(); */

    let config_list = Config::load();
    for device in config_list {
         
    }

    // the output string
    let mut file_output = String::new();
    let mut bar_output = String::new();

    //iterate over devices
    for device in devices.iter() {
        let is_last_item: bool = device == &devices[devices.len() - 1];

        let folders = device.get_folders();
        let (file_string, bar_string) = format_output(&folders, &device.short_name, &device.name, is_last_item);

        file_output += &file_string;
        bar_output += &bar_string;
    }

    //write string and return `Result<>`
    println!("{}", bar_output);
    /* write_to_file(&file_output) */
}

/* fn write_to_file(string: &String) -> std::io::Result<()> {
 *     // write output content
 *     let mut file = File::create(PATH.to_owned() + "output")?;
 *     file.write_all(&string.as_bytes())?;
 *
 *     //write date file
 *     let date = Local::now().format("Last Check: %A - %T");
 *     let mut file = File::create(PATH.to_owned() + "time")?;
 *     file.write_all(&date.to_string().as_bytes())?;
 *
 *     Ok(())
 * } */
