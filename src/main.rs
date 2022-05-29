#[macro_use]
extern crate rocket;
extern crate core;

use clap::Parser;
use clap::Subcommand;
use std::io;

mod commands;
use commands::servers;
use commands::subscriptions;
use commands::work;

mod errors;

mod v2ray;

mod context;
mod settings;
mod server;

use settings::Settings;

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Servers {},
    Work {},
    Subscriptions {
        #[clap(subcommand)]
        command: subscriptions::Commands,
    },
}

#[derive(Parser)]
#[clap(author, about=None)]
struct Cli {
    #[clap(short = 'v', long = "verbose")]
    verbose: bool,

    #[clap(subcommand)]
    command: Option<Commands>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // load settings
    let result = Settings::load("/Users/larry/.v2up/settings.yaml");
    if result.is_err() {
        println!("err on loading settings: {}", result.err().unwrap().message);
        std::process::exit(-1);
    }
    let settings = &mut result.unwrap();

    // load v2ray process
    let process = &mut v2ray::process::Process::new();

    // new v2ray config
    let result = v2ray::config::Config::load("/Users/larry/.v2up/v2ray.json");
    if result.is_err() {
        println!("err on loading config: {}", result.err().unwrap().message);
        std::process::exit(-1)
    }

    let config = &mut result.unwrap();

    // create context
    let mut ctx = context::Context {
        settings: settings,
        config: config,
        process: process,
    };

    match &cli.command {
        Some(Commands::Servers {}) => {
            let result = servers::exec(&mut ctx);
        }
        Some(Commands::Work {}) => {
            work::exec(&mut ctx);
        }
        Some(Commands::Subscriptions { command }) => {
            subscriptions::exec(&mut ctx, command).unwrap()
        }
        None => {}
    }

    println!("verbose: {}", cli.verbose);
    Ok(())
}
