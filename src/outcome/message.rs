use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "msg", rename_all = "snake_case")]
pub enum AppMessage {
    UsingSourcePath { path: String },
    UsingTargetPath { path: String },
    PackageSkipped { package: String },
    LinkingPackage { package: String },
    LinkCreated { from: String, to: String },
}

impl AppMessage {
    pub fn to_json(&self) -> String {
        // EXPECT: infallible serialization.
        serde_json::to_string(self).expect("serialize_fail")
    }
    pub fn emit(&self) {
        eprintln!("{}", self.to_json());
    }
}
