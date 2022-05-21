use crate::context::Context;
use crate::errors;
use crate::errors::Error;
use clap::Subcommand;

use crate::settings::Subscription;
use std::time::SystemTime;

#[derive(Subcommand)]
pub enum Commands {
    Add { name: String, url: String },
    Remove { name: String },
    Update {},
}

pub fn exec(ctx: &mut Context, commands: &Commands) -> Result<(), Error> {
    return match commands {
        Commands::Add { name, url } => add(ctx, name.as_str(), url.as_str()),
        Commands::Remove { name } => remove(ctx, name.as_str()),
        _ => Err(errors::Error {
            kind: errors::kind::ErrorKind::CommandNotFoundError,
            message: format!("command not found"),
        }),
    };
}

pub fn add(ctx: &mut Context, name: &str, url: &str) -> Result<(), Error> {
    let now = SystemTime::now();

    ctx.settings.add_subscription(Subscription {
        name: String::from(name),
        url: String::from(url),
        added_at: chrono::DateTime::from(now),
        last_polled_at: chrono::DateTime::from(std::time::UNIX_EPOCH),
    })
}

pub fn remove(ctx: &Context, name: &str) -> Result<(), Error> {
    Ok(())
}
