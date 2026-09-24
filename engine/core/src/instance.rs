use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instance { pub id: String, pub name: String, pub game_directory: PathBuf, pub minecraft_version: String, pub loader: Option<String> }
