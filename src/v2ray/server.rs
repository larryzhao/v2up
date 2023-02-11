use serde::{Deserialize, Serialize};
use std::str;

use crate::errors::kind::ErrorKind;
use crate::errors::Error;
use crate::v2ray::config::Settings2;
use crate::v2ray::config::User;
use crate::v2ray::config::Vnext;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum ServerType {
    #[serde(rename = "vmess")]
    Vmess(VmessServer),
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VmessServer {
    pub name: String,
    pub network: String,
    pub user_id: String,
    pub alter_id: i32,
    pub address: String,
    pub port: i32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct VmessServerInfo {
    pub v: String,
    pub ps: String,
    pub add: String,
    pub port: String,
    pub id: String,
    pub aid: String,
    pub net: String,
    #[serde(rename = "type")]
    pub ty: String,
    pub host: String,
    pub path: String,
    pub tls: String,
}

impl ServerType {
    pub fn to_outbound(&self) -> Settings2 {
        match self {
            ServerType::Vmess(server) => {
                return Settings2 {
                    vnext: vec![Vnext {
                        address: server.address.clone(),
                        port: server.port,
                        users: vec![User {
                            id: server.user_id.clone(),
                            alter_id: server.alter_id,
                            level: 0,
                            security: String::from("aes-128-gcm"),
                        }],
                    }],
                    domain_strategy: None,
                    response: None,
                    user_level: None,
                }
            }
        }
    }
}

pub fn from_str(server_url: &str) -> Result<ServerType, Error> {
    let parts: Vec<&str> = server_url.split("://").collect();

    return match parts[0] {
        "vmess" => parse_vmess_server(parts[1]),
        _ => Err(Error {
            kind: ErrorKind::UnknownServerProtocol,
            message: format!("unknown server protocol: {}, {}", parts[0], server_url),
        }),
    };
}

fn parse_vmess_server(data: &str) -> Result<ServerType, Error> {
    let result = base64::decode(data);
    if result.is_err() {
        return Err(Error {
            kind: ErrorKind::Base64DecodeError,
            message: format!(
                "decode server info err: {}, source: {}",
                result.err().unwrap(),
                data
            ),
        });
    }

    let bytes = result.unwrap();
    let json_data = match str::from_utf8(bytes.as_slice()) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    let result = serde_json::from_str(json_data);
    if result.is_err() {
        return Err(Error {
            kind: ErrorKind::Base64DecodeError,
            message: format!(
                "decode server info err: {}, source: {}",
                result.err().unwrap(),
                data
            ),
        });
    }

    let server_info: VmessServerInfo = result.unwrap();
    let vmess_server = VmessServer {
        name: server_info.ps,
        network: server_info.net,
        address: server_info.add,
        port: server_info.port.parse().unwrap(),
        user_id: server_info.id,
        alter_id: server_info.aid.parse().unwrap(),
    };

    Ok(ServerType::Vmess(vmess_server))
}
