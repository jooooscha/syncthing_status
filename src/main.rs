use std::fs::File;
use std::io::prelude::*;
use chrono::Local;

mod api_handler;

const PATH: &str = "/home/joscha/.config/syncthing_status/";

mod input;
use input::get_devices;

mod output;
use output::format_output;
// constants

fn main() -> std::io::Result<()> {

    let devices = get_devices();

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
