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

mod workdir;
use workdir::servers::Servers;
use workdir::settings::Settings;

mod errors;
use errors::kind::ErrorKind;

mod context;
mod server;
mod utils;
mod v2ray;

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Servers {},
    Status {},
    Work {},
    Start {},
    Stop {},
    Init {},
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

    #[clap(short = 'd', long = "dir", default_value = "~/.v2up")]
    dir: String,

    #[clap(subcommand)]
    command: Option<Commands>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // check workdir
    let workdir = match workdir::dir::Dir::new(cli.dir) {
        Ok(dir) => dir,
        Err(err) => match err.kind {
            _ => {
                println!("check workdir error: {}", err);
                std::process::exit(-1)
            }
        },
    };

    // check if workdir available
    // if workdir not exist, inform the user to run v2up init
    match workdir.available() {
        Ok(_) => {}
        Err(err) => match err.kind {
            errors::kind::ErrorKind::WorkdirUninitialized => match cli.command {
                Some(Commands::Init {}) => match workdir.init() {
                    Ok(_) => {
                        println!("workdir initialized");
                    }
                    Err(err) => {
                        println!("init working dir {} err: {}", workdir.path(), err);
                    }
                },
                _ => {
                    println!("v2up working directory is not initialized, you should run `v2up init` to initialize it.")
                }
            },
            _ => {
                println!("v2up working directory is unavailable: {}", err);
            }
        },
    };

    // workdir OK, we could load all the settings &
    // load settings
    let result = Settings::load(&workdir);
    if result.is_err() {
        println!("err on loading settings: {}", result.err().unwrap().message);
        std::process::exit(-1);
    }
    let settings = &mut result.unwrap();

    // load servers
    let servers = &mut Servers::from(workdir.path()).expect("err on loading servers");

    // create v2ray process
    let v2ray_cmd = &mut Command::new(settings.v2ray.bin.as_str());
    v2ray_cmd.args(["-config", "/Users/larry/.v2up/v2ray.json", "&"]);
    let v2ray_process =
        &mut utils::process::Process::new(v2ray_cmd, "/Users/larry/.v2up/v2ray.pid");

    // create v2ray process
    let worker_cmd = &mut Command::new("v2up");
    worker_cmd.args(["work"]);
    let worker_process =
        &mut utils::process::Process::new(worker_cmd, "/Users/larry/.v2up/worker.pid");

    // new v2ray config
    let result = v2ray::config::Config::load("/Users/larry/.v2up/v2ray.json");
    if result.is_err() {
        println!("err on loading config: {}", result.err().unwrap().message);
        std::process::exit(-1)
    }

    let config = &mut result.unwrap();

    // create context
    let mut ctx = context::Context {
        dir: &workdir,
        settings: settings,
        config: config,
        v2ray_process: v2ray_process,
        worker_process: worker_process,
        servers: servers,
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
            ctx.v2ray_process.stop();
            // stop v2up worker
            ctx.worker_process.stop();

            // remove pac
            Command::new("networksetup")
                .args(["-setautoproxystate", "Wi-Fi", "off"])
                .output()
                .expect("failed to disable pac");
        }
        Some(Commands::Init {}) => {
            // just do nothing, init is done before
        }
        Some(Commands::Subscriptions { command }) => {
            subscriptions::exec(&mut ctx, command).unwrap()
        }
        None => {}
    }
    Ok(())
}
