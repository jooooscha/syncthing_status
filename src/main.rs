use config::Config;
use crate::api_handler::Rest;
use api_handler::*;

mod api_handler;
mod config;

#[derive(Default)]
struct System {
    folder: Vec<Folder>,
}

impl System {
    fn summary(&self) {
        let mut output = "👍";
        for folder in self.folder.iter() {
            if folder.state != "idle" {
                output = "👎";
            }
        }

        println!("{}", output);
    }
}

#[derive(Default)]
struct Folder {
    id: FolderId,
    label: FolderLabel,
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
        let rest = Rest::new(device);

        for folder in rest.system_config().folders.into_iter() {
            let db_state = rest.db_status(&folder.id);

            let local_folder = Folder::from(folder, db_state);
            system.folder.push(local_folder);
        }
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
