use crate::context::Context;
use crate::errors::kind::ErrorKind;
use crate::errors::Error;
use crate::v2ray::server::ServerType;

use dialoguer::{theme::ColorfulTheme, Select};

pub fn exec(ctx: &mut Context) -> Result<(), Error> {
    let mut selections: Vec<&str> = vec![];

    for server in ctx.servers.iter() {
        match &server.server {
            ServerType::Vmess(server) => selections.push(server.name.as_str()),
            ServerType::Trojan(server) => selections.push(server.name.as_str()),
        }
    }

    if selections.len() == 0 {
        println!("no servers");
        return Ok(());
    }

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick a server")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    let server = ctx
        .servers
        .get(selection)
        .expect(" server selection out of bounds");

    let result = ctx.config.use_server(&server.server);
    if result.is_err() {}

    let result = ctx.v2ray_process.restart(ctx.settings.v2ray_binary());

    if result.is_err() {
        return Err(Error {
            kind: ErrorKind::ExecuteCommandError,
            message: format!("restart v2ray err: {}", result.err().unwrap()),
        });
    }

    match &server.server {
        ServerType::Vmess(server) => {
            println!("use server: {}, {}", server.name, server.address);
        }
        ServerType::Trojan(server) => {
            println!("use server: {}, {}", server.name, server.address);
        }
    }

    return Ok(());
}
