use reqwest::{Client, Response};
use crate::config::Config;

pub use types::*;
pub mod types;

pub struct Api {
    pub client: Client,
    pub config: Config,
}

impl Api {
    pub fn new(config: Config) -> Self {
        let client = Client::builder()
            .danger_accept_invalid_certs(true)
            .connect_timeout(std::time::Duration::from_millis(5000))
            .build().unwrap();

        Self {
            client,
            config,
        }
    }
    pub async fn system_config(&self) -> Result<SystemConfig, reqwest::Error> {
        self.request(SYSTEM_CONFIG).await?.json().await
    }

    pub async fn db_status(&self, id: &FolderId) -> Result<DbState, reqwest::Error> {
        let url = format!("{}{}", DB_STATUS, id);
        self.request(&url).await?.json().await
    }

    async fn request(&self, url: &str) -> Result<Response, reqwest::Error> {
        let url = format!("{}{}", self.config.url, url);
        self.client.get(&url)
            .header("X-API-Key", &self.config.api_key)
            .send().await
    }
}

//////////////////////

/* #[derive(Serialize, Deserialize)]
 * struct FolderId {
 *     id: String,
 *     label: String,
 * }
 *
 * #[derive(Serialize, Deserialize)]
 * struct FolderState {
 *     state: String,
 *     errors: u32,
 * }
 *
 * #[derive(Serialize, Deserialize)]
 * pub struct Folder {
 *     id: FolderId,
 *     state: FolderState,
 * }
 *
 * impl Folder {
 *     pub fn load(config: &Config) -> Vec<Self> {
 *         let url = format!("{}{}", config.url, CONFIG_PATH);
 *         let body = request(&url, &config.api_key);
 *         let folders: Vec<FolderId> = serde_json::from_str::<Vec<FolderId>>(&body).unwrap();
 *
 *         let mut ret = Vec::new();
 *         for id in folders.into_iter() {
 *             let url = format!("{}{}{}", config.url, "/rest/db/status?folder=", id.id);
 *             let folder_info: FolderState = serde_json::from_str(&body).unwrap();
 *             let state: FolderState = serde_json::from_str(&body).unwrap();
 *             let folder = Self { id, state };
 *             ret.push(folder);
 *         }
 *
 *         ret
 *     }
 *
 *     pub fn id(&self) -> String {
 *         self.id.id
 *     }
 *     pub fn label(&self) -> String {
 *         self.id.label
 *     }
 *     pub fn state(&self) -> String {
 *         self.state.state
 *     }
 *     pub fn errors(&self) -> u32 {
 *         self.state.errors
 *     }
 * }
 *
 * fn request(url: &String, api_key: &String) -> String {
 *     let client = Client::builder()
 *         .danger_accept_invalid_certs(true)
 *         .build().unwrap();
 *
 *     let response = client
 *         .get(url)
 *         .header("X-API-Key", api_key)
 *         .send().unwrap()
 *         .text().unwrap();
 *
 *     response
 * } */

/* #[derive(Serialize, Deserialize)]
 * pub struct DirList {
 *     pub dirs: Vec<FolderId>,
 * } */

/* impl DirList {
 *     pub fn fetch(config: &Config) -> Result<Self, Error> {
 *         let url = format!("{}{}", config.url, CONFIG_PATH);
 *         let body = request(&url, &config.api_key).unwrap();
 *         serde_json::from_str::<Self>(&body)
 *     }
 * } */

/* impl Device {
 *     // returns all folders from 'Device' as a Vec<Folder>
 *     pub fn get_folders(&self) -> Result<Vec<FolderId>, Box<dyn std::error::Error>> {
 *         let folders__ = request_folder(&self.url, &self.api_key)?;
 *
 *         let mut folder_complete: Vec<FolderId> = Vec::new();
 *
 *         for (id, label) in folders__.into_iter() {
 *             let folder_info = request_folder_info(&id, &self.url, &self.api_key)?;
 *
 *             let folder = FolderId {
 *                 id,
 *                 label,
 *                 state: folder_info.state,
 *                 errors: folder_info.errors,
 *             };
 *
 *             folder_complete.push(folder);
 *         }
 *
 *         Ok(folder_complete)
 *     }
 * }
 * // Request all folders of a device
 * fn request_folder(device_url: &String, api_key: &String) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
 *     let url = format!("{}{}", device_url, "/rest/system/config");
 *     let body = request(&url, api_key)?;
 *
 *     let device_response: Folders = serde_json::from_str(&body)?;
 *     let mut folders: HashMap<String, String> = HashMap::new();
 *
 *     for folder in device_response.folders.into_iter() {
 *         folders.insert(folder.id, folder.label);
 *     }
 *
 *     Ok(folders)
 * }
 *
 * // Request information about a folder
 * fn request_folder_info(
 *     folder_id: &String,
 *     device_url: &String,
 *     api_key: &String,
 * ) -> Result<FolderInfo, Error> {
 *     let url = format!("{}{}{}", device_url, "/rest/db/status?folder=", folder_id);
 *     let body = request(&url, api_key)?;
 *     let folder_info: FolderInfo = serde_json::from_str(&body).unwrap();
 *     Ok(folder_info)
 * } */
