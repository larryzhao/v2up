use crate::errors::Error;
use crate::v2ray::server::ServerType;
use crate::{errors::kind::ErrorKind, workdir::servers::Server};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fs;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    pub filepath: String,

    pub log: Log,
    pub inbounds: Vec<Inbound>,
    pub outbounds: Vec<Outbound>,
    pub dns: Dns,
    pub routing: Routing,
    pub transport: Transport,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Log {
    pub error: String,
    pub loglevel: String,
    pub access: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Inbound {
    pub listen: String,
    pub protocol: String,
    pub settings: Settings,
    pub port: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub udp: Option<bool>,
    pub auth: Option<String>,
    pub timeout: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Outbound {
    pub mux: Option<Mux>,
    pub protocol: String,
    pub stream_settings: Option<StreamSettings>,
    pub tag: String,
    pub settings: OutboundSettings,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mux {
    pub enabled: bool,
    pub concurrency: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamSettings {
    pub network: String,
    pub security: String,
    pub tls_settings: Option<TLSSettings>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TLSSettings {
    pub server_name: String,
    pub allow_insecure_ciphers: bool,
    pub allow_insecure: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TcpSettings {
    pub header: Header,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Header {
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OutboundSettings {
    pub vnext: Option<Vec<Vnext>>,
    pub servers: Option<Vec<ServerTrojan>>,
}

impl OutboundSettings {
    pub fn address(&self) -> String {
        match &self.vnext {
            Some(servers) => return servers[0].address.clone(),
            None => match &self.servers {
                Some(servers) => return servers[0].address.clone(),
                None => return String::from("none"),
            },
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SettingsVmess {
    #[serde(default)]
    pub vnext: Vec<Vnext>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SettingsTrojan {
    #[serde(default)]
    pub servers: Vec<ServerTrojan>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerTrojan {
    pub address: String,
    pub port: i32,
    pub password: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Vnext {
    pub address: String,
    #[serde(skip_serializing_if = "<[_]>::is_empty")]
    pub users: Vec<User>,
    pub port: i32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub alter_id: i32,
    // pub level: i32,
    pub security: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dns {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Routing {
    pub settings: Settings3,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings3 {
    pub domain_strategy: String,
    pub rules: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transport {}

impl Config {
    pub fn use_server(&mut self, server: &ServerType) -> Result<(), Error> {
        for outbound in &mut self.outbounds {
            if !outbound.tag.eq("proxy") {
                continue;
            }

            let o = server.to_outbound();
            outbound.mux = o.mux;
            outbound.settings = o.settings;
            outbound.stream_settings = o.stream_settings;
            outbound.protocol = o.protocol;
            outbound.tag = o.tag;
        }
        self.save()
    }

    pub fn save(&self) -> Result<(), Error> {
        let val = json!(self);
        val.to_string();
        match fs::write(self.filepath.as_str(), val.to_string()) {
            Ok(_val) => Ok(()),
            Err(err) => Err(Error {
                kind: ErrorKind::WriteFileError,
                message: format!("save v2ray config file err: {}", err),
            }),
        }
    }

    pub fn load(filepath: &str) -> Result<Config, Error> {
        let r = fs::read_to_string(filepath);
        if r.is_err() {
            return Err(Error {
                kind: ErrorKind::ReadFileError,
                message: format!(
                    "err on reading v2ray config file {}, {}",
                    filepath.clone(),
                    "hello"
                ),
            });
        }

        let file_content = r.unwrap();
        let r = serde_json::from_str(file_content.as_str());
        if r.is_err() {
            return Err(Error {
                kind: ErrorKind::ParseJSONError,
                message: format!("parse json err: {}", r.err().unwrap()),
            });
        }

        let mut config: Config = r.unwrap();
        config.filepath = String::from(filepath);

        return Ok(config);
    }
}

const INITIAL_CONFIG: &str = r#"{
    "log": {
      "error": "",
      "loglevel": "info",
      "access": ""
    },
    "inbounds": [
      {
        "listen": "127.0.0.1",
        "protocol": "socks",
        "settings": {
          "udp": false,
          "auth": "noauth"
        },
        "port": "6153"
      },
      {
        "listen": "127.0.0.1",
        "protocol": "http",
        "settings": {
          "timeout": 360
        },
        "port": "6152"
      }
    ],
    "outbounds": [
      {
        "mux": {
          "enabled": false,
          "concurrency": 8
        },
        "protocol": "vmess",
        "streamSettings": {
          "network": "tcp",
          "tcpSettings": {
            "header": {
              "type": "none"
            }
          },
          "security": "none"
        },
        "tag": "proxy",
        "settings": {
          "vnext": [
            {
              "address": "example.org",
              "users": [
                {
                  "id": "example-id",
                  "alterId": 2,
                  "level": 0,
                  "security": "aes-128-gcm"
                }
              ],
              "port": 15109
            }
          ]
        }
      },
      {
        "tag": "direct",
        "protocol": "freedom",
        "settings": {
          "domainStrategy": "UseIP",
          "userLevel": 0
        }
      },
      {
        "tag": "block",
        "protocol": "blackhole",
        "settings": {
          "response": {
            "type": "none"
          }
        }
      }
    ],
    "dns": {},
    "routing": {
      "settings": {
        "domainStrategy": "AsIs",
        "rules": []
      }
    },
    "transport": {}
  }"#;
