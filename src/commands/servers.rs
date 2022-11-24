use crate::context::Context;
use crate::errors::kind::ErrorKind;
use crate::errors::Error;
use crate::v2ray::server::Server;

use dialoguer::{theme::ColorfulTheme, Select};

pub fn exec(ctx: &mut Context) -> Result<(), Error> {
    let mut selections: Vec<&str> = vec![];

    for server in &ctx.settings.subscriptions[0].servers {
        match server {
            Server::Vmess(server) => selections.push(server.name.as_str()),
        }
    }

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick a server")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    let server = &ctx.settings.subscriptions[0].servers[selection];
    let result = ctx.config.use_server(server);
    if result.is_err() {}

    let result = ctx.v2ray_process.restart(ctx.settings.v2ray_binary());
    if result.is_err() {
        return Err(Error {
            kind: ErrorKind::ExecuteCommandError,
            message: format!("restart v2ray err: {}", result.err().unwrap()),
        });
    }

    match server {
        Server::Vmess(server) => {
            println!("use server: {}, {}", server.name, server.address);
        }
    }

    return Ok(());
}
