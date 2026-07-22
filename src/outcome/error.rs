use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "err", rename_all = "snake_case")]
pub enum AppError {
    HomeDirNotFound,
    CurrentDirNotFound,
    CanonicalizeError { what: String },
    LinkingError { from: String, what: String },
}

impl AppError {
    pub fn to_json(&self) -> String {
        // EXPECT: infallible serialization.
        serde_json::to_string(self).expect("serialize_fail")
    }
    pub fn emit(&self) {
        eprintln!("{}", self.to_json());
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_json())
    }
}

impl std::error::Error for AppError {}
