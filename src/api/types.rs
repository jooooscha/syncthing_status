use serde::{Deserialize, Serialize};

pub const SYSTEM_CONFIG: &str = "/rest/system/config";
pub const DB_STATUS: &str = "/rest/db/status?folder=";

pub type FolderId = String;
pub type FolderLabel = String;

#[derive(PartialEq, Serialize, Deserialize, PartialOrd, Eq, Ord, Debug, Copy, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum State {
    Error,
    Unknown,
    Syncing,
    Scanning,
    ScanWaiting,
    SyncWaiting,
    SyncPreparing,
    Idle,
    #[serde(rename(deserialize = ""))]
    Paused,
}

impl Default for State {
    fn default() -> Self {
        Self::Idle
    }
}

impl State {
    pub fn to_emoji(&self) -> &str {
        match self {
            Self::Error => "❌",
            Self::Unknown => "🤷",
            Self::Syncing => "💾",
            Self::Scanning => "💿",
            Self::ScanWaiting
                | Self::SyncWaiting
                | Self::SyncPreparing => "🕛",
            Self::Idle => "👍",
            Self::Paused => "⏸️",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SystemConfig {
    pub folders: Vec<Folder>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Folder {
    pub id: FolderId,
    pub label: FolderLabel,
    pub paused: bool,
}

#[derive(Serialize, Deserialize)]
pub struct DbStatus {
    pub state: State,
}
