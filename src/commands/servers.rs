use crate::context::Context;
use crate::v2ray::server::Server;
use crate::errors::Error;

use std::{io, sync::mpsc, thread, time::Duration};
use dialoguer::{theme::ColorfulTheme, Select};

pub fn exec(ctx: &mut Context) -> Result<(), Error> {
    let mut selections : Vec<&str> = vec![];
    
    for server in &ctx.settings.subscriptions[0].servers {
        match server {
            Server::Vmess(server) => selections.push(server.name.as_str())
        }
    }

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick a server").default(0).items(&selections[..]).interact().unwrap(); 

    let server = &ctx.settings.subscriptions[0].servers[selection];
    let result = ctx.config.use_server(server);
    if result.is_err() {

    }

    ctx.process.restart(ctx.settings.v2ray_binary());
    match server {
        Server::Vmess(server) => {
            println!("use server: {}, {}", server.name, server.address);
        }
    }

    return Ok(())
}