extern crate core;
extern crate rocket;
extern crate termion;

use clap::Parser;
use clap::Subcommand;
use std::process::Command;

mod commands;
use commands::servers;
use commands::start;
use commands::status;
use commands::subscriptions;
use commands::work;

mod errors;
mod utils;

mod v2ray;

mod context;
mod server;
mod settings;

use settings::Settings;

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Servers {},
    Status {},
    Work {},
    Start {},
    Stop {},
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

    // create v2ray process
    let cmd = &mut Command::new(settings.v2ray.bin.as_str());
    cmd.args(["-config", "/Users/larry/.v2up/v2ray.json", "&"]);
    let process = &mut utils::process::Process::new(cmd, "/Users/larry/.v2up/v2ray.pid");

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
        Some(Commands::Status {}) => {
            status::exec(&ctx);
        }
        Some(Commands::Work {}) => {
            work::exec(&mut ctx);
        }
        Some(Commands::Start {}) => {
            start::exec(&mut ctx);
        }
        Some(Commands::Stop {}) => {
            // stop v2ray
            ctx.process.stop();
            // stop v2up worker
            // remove pac
            Command::new("networksetup")
                .args(["-setautoproxystate", "Wi-Fi", "off"])
                .output()
                .expect("failed to disable pac");
        }
        Some(Commands::Subscriptions { command }) => {
            subscriptions::exec(&mut ctx, command).unwrap()
        }
        None => {}
    }
    Ok(())
}
