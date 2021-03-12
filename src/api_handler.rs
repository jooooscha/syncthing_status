use reqwest::{blocking::Client, Error};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Eq, PartialEq)]
pub struct Device {
    pub url: String,
    pub name: String,
    pub api_key: String,
}

#[derive(Serialize, Deserialize)]
struct DeviceResponse {
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
pub struct Folder {
    pub id: String,
    pub label: String,
    pub state: String,
    pub errors: u32,
}

impl Device {
    // returns all folders from 'Device' as a Vec<Folder>
    pub fn get_folders(&self) -> Result<Vec<Folder>, Error> {
        let folders__ = request_folder(&self.url, &self.api_key)?;

        let mut folder_complete: Vec<Folder> = Vec::new();

        for (id, label) in folders__.into_iter() {
            let folder_info = request_folder_info(&id, &self.url, &self.api_key)?;

            let folder = Folder {
                id,
                label,
                state: folder_info.state,
                errors: folder_info.errors,
            };

            folder_complete.push(folder);
        }

        Ok(folder_complete)
    }
}
// Request all folders of a device
fn request_folder(device_url: &String, api_key: &String) -> Result<HashMap<String, String>, Error> {
    let url = format!("{}{}", device_url, "/rest/system/config");
    let body = make_request(&url, api_key)?;

    let device_response: DeviceResponse = serde_json::from_str(&body).unwrap();
    let mut folders: HashMap<String, String> = HashMap::new();

    for folder in device_response.folders.into_iter() {
        folders.insert(folder.id, folder.label);
    }

    Ok(folders)
}

// Request information about a folder
fn request_folder_info(
    folder_id: &String,
    device_url: &String,
    api_key: &String,
) -> Result<FolderInfo, Error> {
    let url = format!("{}{}{}", device_url, "/rest/db/status?folder=", folder_id);
    let body = make_request(&url, api_key)?;
    let folder_info: FolderInfo = serde_json::from_str(&body).unwrap();
    Ok(folder_info)
}

fn make_request(url: &String, api_key: &String) -> Result<String, Error> {
    let client = Client::builder().build()?;

    let response = client
        .get(url)
        .header("X-API-Key", api_key)
        .send()?
        .text()?;

    Ok(response)
}
