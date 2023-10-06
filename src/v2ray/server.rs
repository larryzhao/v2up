use regex::Regex;
use rocket::data::N;
use serde::{Deserialize, Serialize};
use std::str;

use crate::errors::kind::ErrorKind;
use crate::errors::Error;
use crate::v2ray::config::Outbound;
use crate::v2ray::config::OutboundSettings;
use crate::v2ray::config::SettingsTrojan;
use crate::v2ray::config::SettingsVmess;
use crate::v2ray::config::User;
use crate::v2ray::config::Vnext;

use super::config::ServerTrojan;
use super::config::StreamSettings;
use super::config::TLSSettings;
use super::config::WSSettings;
use super::config::WSSettingsHeaders;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum ServerType {
    #[serde(rename = "vmess")]
    Vmess(VmessServer),
    #[serde(rename = "trojan")]
    Trojan(TrojanServer),
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

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct TrojanServer {
    pub name: String,
    pub address: String,
    pub port: i32,
    pub password: String,
    pub sni: String,
    pub allow_insecure: bool,
    #[serde(default)]
    pub network: String,
    #[serde(default)]
    pub path: String,
}

impl TrojanServer {
    pub fn new() -> TrojanServer {
        return TrojanServer {
            name: String::from(""),
            address: String::from(""),
            port: 0,
            password: String::from(""),
            sni: String::from(""),
            allow_insecure: false,
            network: String::from(""),
            path: String::from(""),
        };
    }
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
    pub fn to_outbound(&self) -> Outbound {
        match self {
            ServerType::Trojan(server) => {
                return Outbound {
                    mux: None,
                    protocol: String::from("trojan"),
                    settings: OutboundSettings {
                        servers: Some(vec![ServerTrojan {
                            address: server.address.clone(),
                            port: server.port,
                            password: server.password.clone(),
                        }]),
                        vnext: None,
                    },
                    stream_settings: Some(StreamSettings {
                        network: match server.network.as_str() {
                            "ws" => String::from("ws"),
                            _ => String::from("tcp"),
                        },
                        security: String::from("tls"),
                        tls_settings: Some(TLSSettings {
                            server_name: server.sni.clone(),
                            allow_insecure: server.allow_insecure,
                            allow_insecure_ciphers: server.allow_insecure,
                        }),
                        ws_settings: match server.network.as_str() {
                            "ws" => Some(WSSettings {
                                path: server.path.clone(),
                                headers: WSSettingsHeaders {
                                    host: server.sni.clone(),
                                },
                            }),
                            _ => None,
                        },
                    }),
                    tag: String::from("proxy"),
                };
            }
            ServerType::Vmess(server) => {
                return Outbound {
                    mux: None,
                    protocol: String::from("vmess"),
                    tag: String::from("proxy"),
                    settings: OutboundSettings {
                        vnext: Some(vec![Vnext {
                            address: server.address.clone(),
                            port: server.port,
                            users: vec![User {
                                id: server.user_id.clone(),
                                alter_id: server.alter_id,
                                // level: 0,
                                security: String::from("aes-128-gcm"),
                            }],
                        }]),
                        servers: None,
                    },
                    stream_settings: Some(StreamSettings {
                        network: String::from("tcp"),
                        security: String::from("none"),
                        tls_settings: None,
                        ws_settings: None,
                    }),
                };
            }
        }
    }
}

pub fn from_str(server_url: &str) -> Result<ServerType, Error> {
    let parts: Vec<&str> = server_url.split("://").collect();

    return match parts[0] {
        "vmess" => parse_vmess_server(parts[1]),
        "trojan" => parse_trojan_server(parts[1]),
        _ => Err(Error {
            kind: ErrorKind::UnknownServerProtocol,
            message: format!("unknown server protocol: {}, {}", parts[0], server_url),
        }),
    };
}

fn parse_trojan_server(data: &str) -> Result<ServerType, Error> {
    let server = TrojanServer::new();

    // data: trojan://31b98cae-da2d-4456-b351-f91838313f0a@jp1.lxjc.app:443?allowInsecure=0&peer=16-163-218-240.nhost.00cdn.com&sni=16-163-218-240.nhost.00cdn.com#%E5%89%A9%E4%BD%99%E6%B5%81%E9%87%8F%EF%BC%9A99.89%20GB
    let re = Regex::new("trojan://(.*)@(.*):(.*)?(.*)#(.*)").unwrap();

    if !re.is_match(data) {
        return Err(Error {
            kind: ErrorKind::UnknownServerProtocol,
            message: format!("err trojan server url: {}", data),
        });
    }

    for cap in re.captures_iter(data) {
        println!("{}", &cap[0]);
    }

    // handle part2

    Ok(ServerType::Trojan(server))
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
