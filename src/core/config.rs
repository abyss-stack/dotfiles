use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

use crate::outcome::{AppError, AppMessage, AppResult};

#[derive(Serialize, Deserialize)]
pub enum Strategy {
    Copy,
    Symlink,
}

#[derive(Serialize, Deserialize)]
pub struct Package {
    pub name: String,
    pub source: PathBuf,
    pub target: PathBuf,
    pub strategy: Strategy,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub packages: Vec<Package>,
}

impl Config {
    pub const TEMPLATE: &str = include_str!("../../template.json");

    pub fn load<P: AsRef<Path>>(path: P) -> AppResult<Self> {
        let path = path.as_ref();

        AppMessage::LoadingConfig {
            path: path.to_path_buf(),
        }
        .emit();

        let data = std::fs::read_to_string(path).map_err(|e| AppError::ConfigLoad {
            path: path.to_path_buf(),
            what: e.to_string(),
        })?;

        let config: Self = serde_json::from_str(&data).map_err(|e| AppError::ConfigParse {
            what: e.to_string(),
        })?;

        AppMessage::ConfigLoaded.emit();

        Ok(config)
    }
}
