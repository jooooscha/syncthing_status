use config::Config;
use crate::api::Api;
use api::*;
use std::collections::HashMap;

mod api;
mod config;

#[derive(Default)]
struct System {
    folder: HashMap<String, Vec<Folder>>,
}

impl System {
    fn output(&self) {

        let mut string = String::new();

        for (name, folder_list) in self.folder.iter() {
            let mut states = Vec::new();

            for folder in folder_list.iter() {
                states.push(folder.state.clone());
            }
            states.sort();

            string += &format!("{}: {} ", name, states[0].to_emoji());
        }

        println!("{}", string.trim());
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
    fn from(f: api::Folder, state: DbStatus) -> Self {
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
        let rest = Api::new(device);

        let mut folder_list = Vec::new();
        let system_config = match rest.system_config() {
            Ok(c) => c,
            Err(_) => continue,
        };
        for folder in system_config.folders.into_iter() {
            let db_state = match rest.db_status(&folder.id) {
                Ok(dbs) => dbs,
                Err(_) => continue,
            };

            let local_folder = Folder::from(folder, db_state);
            folder_list.push(local_folder);

        }

        system.folder.insert(name, folder_list);
    }

    // OUTPUT

    system.output();

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
