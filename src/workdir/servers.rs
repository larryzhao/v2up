use crate::errors::kind::ErrorKind;
use crate::errors::Error;
use crate::v2ray::server::ServerType;
use serde::{Deserialize, Serialize};

use std::fs::File;
use std::path::PathBuf;
use std::slice::Iter;

#[derive(Deserialize, Serialize, Debug)]
pub struct Server {
    pub group: String,
    pub server: ServerType,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Servers {
    servers: Vec<Server>,
}

impl Servers {
    pub fn from_workdir(workdir_path: &str) -> Result<Servers, Error> {
        let servers_file = File::open(PathBuf::from(workdir_path).join("servers.yaml")).unwrap();
        match serde_yaml::from_reader(&servers_file) {
            Ok(servers) => Ok(servers),
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
    ) -> Result<(), Error> {
        // 1. remove all servers with group name
        let mut indices_to_remove: Vec<usize> = vec![];
        for (idx, s) in self.servers.iter().enumerate() {
            if !s.group.eq(group) {
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

        Ok(())
    }

    pub fn save(&self) -> Result<(), Error> {
        Ok(())
    }

    pub fn iter(&self) -> Iter<Server> {
        self.servers.iter()
    }

    pub fn get(&self, idx: usize) -> Option<&Server> {
        self.servers.get(idx)
    }
}
