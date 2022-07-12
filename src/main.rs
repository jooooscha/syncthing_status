use config::Config;
use crate::api_handler::Rest;
use api_handler::*;
use std::collections::HashMap;

mod api_handler;
mod config;

#[derive(Default)]
struct System {
    folder: HashMap<String, Vec<Folder>>,
}

impl System {
    fn summary(&self) {
        let mut output = "👍";
        for (name, folder_list) in self.folder.iter() {
            for folder in folder_list.iter() {
                if folder.state != "idle" {
                    output = "👎";
                }
            }
            print!("{}: {}", name, output);
        }
        println!();
    }
}

#[derive(Default)]
struct Folder {
    #[allow(dead_code)]
    id: FolderId,
    #[allow(dead_code)]
    label: FolderLabel,
    #[allow(dead_code)]
    paused: bool,
    state: State
}

impl Folder {
    fn from(f: api_handler::Folder, state: DbStatus) -> Self {
        Self {
            id: f.id,
            label: f.label,
            paused: f.paused,
            state: state.state,
        }
    }
}

fn main() {

    let mut system = System::default();

    let config = Config::load();
    for device in config.into_iter() {
        let name = device.short_name.clone();
        let rest = Rest::new(device);

        let mut folder_list = Vec::new();
        for folder in rest.system_config().folders.into_iter() {
            let db_state = rest.db_status(&folder.id);

            let local_folder = Folder::from(folder, db_state);
            folder_list.push(local_folder);

        }

        system.folder.insert(name, folder_list);
    }

    // OUTPUT

    system.summary();

    // the output string
/*     let mut file_output = String::new();
 *     let mut bar_output = String::new();
 *
 *     //iterate over devices
 *     for device in devices.iter() {
 *         let is_last_item: bool = device == &devices[devices.len() - 1];
 *
 *         let folders = device.get_folders();
 *         let (file_string, bar_string) = format_output(&folders, &device.short_name, &device.name, is_last_item);
 *
 *         file_output += &file_string;
 *         bar_output += &bar_string;
 *     }
 *
 *     println!("{}", bar_output); */
}
