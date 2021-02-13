static CONNECTION_ERROR: &str = "No con.";

use reqwest::Error;
use crate::api_handler::*;

pub fn format_output(folders: &Result<Vec<Folder>, Error>, device_name: &String, is_last: bool) -> (String, String) {

    let name = device_name;
    let mut status = String::from("Up to date");
    let mut file_string: String;

    file_string = format!("-------------------- {:-<25} \n", &name);

    match folders {
        Ok(folders) => {
            for f in folders.iter() {
                file_string = format!("{}{:>13}: {:<10} | Errors: {} \n", file_string, f.label, f.state, f.errors);

                if &f.state != "idle" && &f.state != "" { // emtpy string is when folder is paused
                    status = f.state.clone() // only keep last not-idle status
                }
            }
        },
        Err(_) => {
            file_string += &CONNECTION_ERROR.to_string();
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
