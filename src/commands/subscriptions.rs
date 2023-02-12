extern crate base64;

use crate::context::Context;
use crate::errors;
use crate::errors::Error;
use crate::v2ray;
use crate::v2ray::server::*;
use crate::workdir::settings::Subscription;
use std::time::Duration;

use clap::Subcommand;
use std::str;
use std::time::SystemTime;

use reqwest::blocking::ClientBuilder;

#[derive(Subcommand)]
pub enum Commands {
    Add { name: String, url: String },
    Remove { name: String },
    // List {},
    Update {},
}

pub fn exec(ctx: &mut Context, commands: &Commands) -> Result<(), Error> {
    return match commands {
        Commands::Add { name, url } => add(ctx, name.as_str(), url.as_str()),
        Commands::Remove { name } => remove(ctx, name.as_str()),
        Commands::Update {} => update(ctx),
    };
}

pub fn add(ctx: &mut Context, name: &str, url: &str) -> Result<(), Error> {
    let now = SystemTime::now();

    ctx.settings.add_subscription(Subscription {
        name: String::from(name),
        url: String::from(url),
        added_at: chrono::DateTime::from(now),
        last_updated_at: chrono::DateTime::from(std::time::UNIX_EPOCH),
    })
}

pub fn update(ctx: &mut Context) -> Result<(), Error> {
    for sub in &mut ctx.settings.subscriptions {
        match fetch(sub.url.as_str()) {
            Ok(servers) => {
                match ctx
                    .servers
                    .update_by_group_name(sub.name.as_str(), &servers)
                {
                    Ok(changed) => {
                        if changed {
                            sub.last_updated_at = chrono::DateTime::from(SystemTime::now())
                        }
                    }
                    Err(err) => {
                        println!("update servers by subscription {} err: {}", sub.name, err)
                    }
                }
                continue;
            }
            Err(err) => {
                println!("request {} err: {}", sub.url, err);
                continue;
            }
        }
    }

    ctx.servers.save().expect("fail to save servers.yaml");
    ctx.settings.save().expect("fail to save settings.yaml");
    Ok(())
}

fn fetch(url: &str) -> Result<Vec<ServerType>, Error> {
    let client = ClientBuilder::new()
        .no_proxy()
        .timeout(Duration::new(30, 0))
        .build()
        .unwrap();

    let result = client.get(url).send();
    if result.is_err() {
        return match result.err() {
            Some(err) => {
                eprintln!("{}", err);
                return Err(Error {
                    kind: errors::kind::ErrorKind::HTTPRequestError,
                    message: format!("get {} with unknown err", url),
                });
            }
            None => Err(Error {
                kind: errors::kind::ErrorKind::HTTPRequestError,
                message: format!("get {} with unknown err", url),
            }),
        };
    }

    let result = result.unwrap().text();
    if result.is_err() {
        return match result.err() {
            Some(err) => Err(Error {
                kind: errors::kind::ErrorKind::HTTPRequestError,
                message: format!("read body err: {}", err),
            }),
            None => Err(Error {
                kind: errors::kind::ErrorKind::HTTPRequestError,
                message: format!("read body with unknown err"),
            }),
        };
    }

    let body = result.unwrap();
    let mut servers = vec![];

    let result = base64::decode(body);
    if result.is_err() {
        return Err(Error {
            kind: errors::kind::ErrorKind::Base64DecodeError,
            message: format!(
                "decode subscription content base64 err: {}",
                result.err().unwrap()
            ),
        });
    }

    let bytes = result.unwrap();
    let servers_data = match str::from_utf8(bytes.as_slice()) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    let server_entries = servers_data.split("\r\n");
    for entry in server_entries {
        if entry.trim().is_empty() {
            continue;
        }

        match v2ray::server::from_str(entry) {
            Ok(server) => servers.push(server),
            Err(err) => {
                println!(
                    "parsing server entry: {} with error: {}",
                    entry, err.message
                );
                continue;
            }
        }
    }

    Ok(servers)
}

pub fn remove(ctx: &Context, name: &str) -> Result<(), Error> {
    Ok(())
}
