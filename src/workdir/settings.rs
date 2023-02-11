use serde::{Deserialize, Serialize};
use std::{fs, path};

use crate::errors::kind::ErrorKind;
use crate::errors::Error;
use crate::v2ray::server::*;
use std::fs::File;
use std::path::PathBuf;
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
    pub last_polled_at: chrono::DateTime<chrono::Local>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    pub filepath: String,

    pub v2ray: V2Ray,
    pub log: Log,

    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    pub subscriptions: Vec<Subscription>,
}

impl Settings {
    pub fn add_subscription(&mut self, subscription: Subscription) -> Result<(), Error> {
        self.subscriptions.push(subscription);
        self.save()
    }

    pub fn update_subscription_servers(
        &mut self,
        name: &str,
        servers: &Vec<ServerType>,
    ) -> Result<(), Error> {
        let mut sub_idx: usize = 0;

        for i in 0..self.subscriptions.len() {
            if self.subscriptions[i].name.as_str().eq(name) {
                sub_idx = i;
                break;
            }
        }

        self.subscriptions[sub_idx].last_polled_at = chrono::DateTime::from(SystemTime::now());
        // self.subscriptions[sub_idx].servers = (*servers).clone();
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

    pub fn load(workdir: &Dir) -> Result<Settings, Error> {
        // read and deserialize settings.yaml
        let settings_file = File::open(workdir.filepath("settings.yaml").as_str()).unwrap();
        let settings: Settings = match serde_yaml::from_reader(&settings_file) {
            Ok(settings) => settings,
            Err(err) => {
                return Err(Error {
                    kind: ErrorKind::LoadSettingsError,
                    message: format!("load settings.yaml err: {}", err),
                });
            }
        };

        return Ok(settings);

        // let r = fs::read_to_string(&settings_path);
        // if r.is_err() {
        //     return Err(Error {
        //         kind: ErrorKind::ReadFileError,
        //         message: format!(
        //             "err on reading settings file {}, {}",
        //             settings_path.to_str().unwrap(),
        //             r.err().unwrap()
        //         ),
        //     });
        // }

        // let content = r.unwrap();
        // let r = serde_yaml::from_str(&content);
        // if r.is_err() {
        //     return Err(Error {
        //         kind: ErrorKind::ParseYAMLError,
        //         message: format!("parse yaml err: {}", r.err().unwrap()),
        //     });
        // }

        // let mut settings: Settings = r.unwrap();
        // settings.filepath = String::from(filepath);
        // return Ok(settings);
    }
}
