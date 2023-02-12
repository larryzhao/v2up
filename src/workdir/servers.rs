use crate::errors::kind::ErrorKind;
use crate::errors::Error;
use crate::v2ray::server::ServerType;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;

use super::dir::Dir;
use std::fs::File;
use std::slice::Iter;

#[derive(Deserialize, Serialize, Debug)]
pub struct Server {
    pub group: String,
    pub server: ServerType,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Servers {
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    pub filepath: String,

    servers: Vec<Server>,
}

impl Servers {
    pub fn from_workdir(workdir: &Dir) -> Result<Servers, Error> {
        let filepath = workdir.filepath("servers.yaml");
        let servers_file = File::open(filepath.as_str()).unwrap();
        match serde_yaml::from_reader::<&std::fs::File, Servers>(&servers_file) {
            Ok(mut servers) => {
                servers.filepath = filepath;
                Ok(servers)
            }
            Err(err) => Err(Error {
                kind: ErrorKind::LoadSettingsError,
                message: format!("load servers.yaml err: {}", err),
            }),
        }
    }

    pub fn update_by_group_name(
        &mut self,
        group: &str,
        servers: &Vec<ServerType>,
    ) -> Result<bool, Error> {
        // if new servers is empty, then do nothing
        if servers.is_empty() {
            return Ok(false);
        }

        // 1. remove all servers with group name
        let mut indices_to_remove: Vec<usize> = vec![];
        for (idx, s) in self.servers.iter().enumerate() {
            if s.group.eq(group) {
                continue;
            }
            indices_to_remove.push(idx);
        }

        for i in indices_to_remove.iter().rev() {
            self.servers.remove(*i);
        }

        // 2. add new servers
        for server in servers.iter() {
            self.servers.push(Server {
                group: String::from(group),
                server: (*server).clone(),
            })
        }

        Ok(true)
    }

    pub fn save(&self) -> Result<(), Error> {
        // let servers_file = File::open(self.filepath.as_str()).expect("fail to open servers.yaml");
        let servers_file = OpenOptions::new()
            .write(true)
            .open(self.filepath.as_str())
            .expect("fail to open servers.yaml");
        serde_yaml::to_writer(servers_file, self)
            .expect("fail to encode and write to servers.yaml");
        Ok(())
    }

    pub fn iter(&self) -> Iter<Server> {
        self.servers.iter()
    }

    pub fn get(&self, idx: usize) -> Option<&Server> {
        self.servers.get(idx)
    }
}
