static CONNECTION_ERROR: &str = "👎";
static CONNECTION_GOOD: &str = "👍";
static SCANNING: &str = "📀";
static SYNCING: &str = "💾";
// static PREPARING: &str = "💿";
static PREPARING: &str = SYNCING;

use crate::api_handler::*;

pub fn format_output(
        folders: &Result<Vec<Folder>, Box<dyn std::error::Error>>,
        name: &String,
        long_name: &String,
        is_last: bool,
    ) -> (String, String) {

    // let mut status = String::from("Ok");
    let mut status = CONNECTION_GOOD.to_string();
    let mut file_string: String;

    let ln = format!("{} ", &long_name);
    file_string = format!("-------------------- {:-<25}\n", ln);

    match folders {
        Ok(folders) => {
            for f in folders.iter() {
                file_string = format!(
                    "{}{:>13}: {:<10} | Errors: {} \n",
                    file_string, f.label, f.state, f.errors
                );

                if &f.state != "idle" && &f.state != "" {
                    // emtpy string is when folder is paused
                    status = match f.state.clone().as_str() {
                        "scanning" => SCANNING.to_string(),
                        "sync-preparing" => PREPARING.to_string(),
                        "syncing" => SYNCING.to_string(),
                        _ => f.state.clone(),
                    };
                    /* status = f.state.clone() // only keep last not-idle status */
                }
            }
        }
        Err(_) => {
            file_string += &CONNECTION_ERROR.to_string();
            status = CONNECTION_ERROR.to_string();
        }
    }

    if is_last {
        (file_string, format!("{}: {}", name, status))
    } else {
        (file_string + "\n", format!("{}: {} - ", name, status))
    }
}
