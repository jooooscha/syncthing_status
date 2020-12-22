use std::fs::File;
use std::io::prelude::*;
use chrono::Local;
use reqwest::{
    Error
};

mod api_handler;
use api_handler::*;

// constants

static CONNECTION_ERROR: &str = "connection error";
static PATH: &str = "/home/joscha/.config/syncthing_status/";

fn main() -> std::io::Result<()> {

    //define devices
    #[allow(non_snake_case)]
    let devices: [Device; 2] = [
        Device {
            url: String::from("http://127.0.0.1:8080"),
            name: String::from("laptop"),
            api_key: String::from("FLR6DH9pyGvMuweZics9LDL9Auwb3JRG"),
        },
        Device {
            url: String::from("http://192.168.0.10:8384"),
            name: String::from("rasp"),
            api_key: String::from("2feNdfnPzCYRwCcy2cVXKVVA2VH9dubZ"),
        }
    ];

    // the output string
    let mut file_output = String::new();
    let mut bar_output = String::new();

    //iterate over devices
    for device in devices.iter() {

        let is_last_item: bool = device == &devices[devices.len() -1];

        let folders = device.get_folders();
        let (file_string, bar_string) = format_output(&folders, &device.name, is_last_item);

        file_output += &file_string;
        bar_output += &bar_string;
    }

    //write string and return `Result<>`
    println!("{}", bar_output);
    write_to_file(&file_output)
}

fn format_output(folders: &Result<Vec<Folder>, Error>, device_name: &String, is_last: bool) -> (String, String) {

    let name = device_name;
    let mut status = String::from("Up to date");
    let mut file_string: String;

    match folders {
        Ok(folders) => {
            file_string = format!("-------------------- {:-<25} \n", &name);

            for f in folders.iter() {
                file_string = format!("{}{:>13}: {:<10} | Errors: {} \n",
                                 file_string,
                                 f.label,
                                 f.state,
                                 f.errors
                                );

                if &f.state != "idle" {
                    status = f.state.clone() // only keep last not-idle status
                }
            }
        },
        Err(error) => {
            /* eprintln!("Error while requsting folders: {}", error); */
            file_string = String::new();
            status = CONNECTION_ERROR.to_string();
        }
    }

    if is_last {
        (
            file_string,
            format!("{}: {}", device_name, status)
        )
    } else {
        (
            file_string + "\n",
            format!("{}: {} - ", device_name, status)
        )
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
