use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "fail", rename_all = "snake_case")]
pub enum AppError {
    /* config */
    ConfigLoad {
        path: PathBuf,
        what: String,
    },
    ConfigParse {
        what: String,
    },

    /* symlink */
    RemoveDir {
        package: String,
        path: PathBuf,
        what: String,
    },
    RemoveFile {
        package: String,
        path: PathBuf,
        what: String,
    },
    Symlink {
        package: String,
        source: PathBuf,
        target: PathBuf,
        what: String,
    },

    /* copy */
    Copy {
        package: String,
        source: PathBuf,
        target: PathBuf,
        what: String,
    },
}

impl AppError {
    #[allow(clippy::expect_used)]
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).expect("to_json_fail")
    }

    pub fn emit(&self) {
        eprintln!("{}", self.to_json());
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_json())
    }
}

impl std::error::Error for AppError {}
