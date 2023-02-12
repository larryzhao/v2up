use serde::{Deserialize, Serialize};
use std::fs;

use crate::errors::kind::ErrorKind;
use crate::errors::Error;
use crate::v2ray::server::*;
use std::fs::File;
use std::time::SystemTime;

use super::dir::Dir;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct V2Ray {
    pub bin: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Log {
    pub location: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Subscription {
    pub name: String,
    pub url: String,
    pub added_at: chrono::DateTime<chrono::Local>,
    pub last_updated_at: chrono::DateTime<chrono::Local>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    pub filepath: String,

    pub v2ray: V2Ray,
    pub log: Log,
    pub subscriptions: Vec<Subscription>,
}

impl Settings {
    pub fn add_subscription(&mut self, subscription: Subscription) -> Result<(), Error> {
        self.subscriptions.push(subscription);
        self.save()
    }

    pub fn save(&self) -> Result<(), Error> {
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

    pub fn v2ray_binary(&self) -> &str {
        if self.v2ray.bin.is_empty() {
            return "/usr/local/bin/v2up";
        }
        return self.v2ray.bin.as_str();
    }

    pub fn from_workdir(workdir: &Dir) -> Result<Settings, Error> {
        // read and deserialize settings.yaml
        let filepath = workdir.filepath("settings.yaml");
        let settings_file = File::open(filepath.as_str()).unwrap();
        match serde_yaml::from_reader::<&std::fs::File, Settings>(&settings_file) {
            Ok(mut settings) => {
                settings.filepath = filepath;
                return Ok(settings);
            }
            Err(err) => {
                return Err(Error {
                    kind: ErrorKind::LoadSettingsError,
                    message: format!("load settings.yaml err: {}", err),
                });
            }
        };
    }
}
