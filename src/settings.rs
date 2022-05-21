use serde::{Deserialize, Serialize};

use std::fs;

use crate::errors::kind::ErrorKind;
use crate::errors::Error;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct V2Ray {
    pub path: String,
    pub version: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Log {
    pub location: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Subscription {
    pub name: String,
    pub url: String,
    pub added_at: chrono::DateTime<chrono::Local>,
    pub last_polled_at: chrono::DateTime<chrono::Local>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    pub filepath: String,

    pub v2ray: V2Ray,
    pub log: Log,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub subscriptions: Vec<Subscription>,
}

impl Settings {
    pub fn add_subscription(&mut self, subscription: Subscription) -> Result<(), Error> {
        self.subscriptions.push(subscription);
        self.save()
    }

    pub fn save(&mut self) -> Result<(), Error> {
        let result = serde_yaml::to_string(self);
        if result.is_err() {
            let err_msg = result.err().unwrap();
            return Err(Error {
                kind: ErrorKind::EncodeYAMLError,
                message: format!("encode yaml for settings.yaml err: {}", err_msg),
            });
        }

        let content = result.unwrap();
        fs::write(self.filepath.as_str(), content);
        Ok(())
    }

    pub fn load(filepath: &str) -> Result<Settings, Error> {
        let r = fs::read_to_string(filepath);
        if r.is_err() {
            return Err(Error {
                kind: ErrorKind::ReadFileError,
                message: format!(
                    "err on reading settings file {}, {}",
                    filepath.clone(),
                    "hello"
                ),
            });
        }

        let content = r.unwrap();

        let r = serde_yaml::from_str(&content);
        if r.is_err() {
            return Err(Error {
                kind: ErrorKind::ParseYAMLError,
                message: format!("parse yaml err: {}", r.err().unwrap()),
            });
        }

        let mut settings: Settings = r.unwrap();
        settings.filepath = String::from(filepath);
        return Ok(settings);
    }
}
