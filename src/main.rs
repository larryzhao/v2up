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
use commands::stop;
use commands::subscriptions;
use commands::work;

mod workdir;
use workdir::servers::Servers;
use workdir::settings::Settings;

mod errors;

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
    Version {},
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
    let mut settings = Settings::from_workdir(&workdir).expect("err loading settings");

    // load servers
    let servers = &mut Servers::from_workdir(&workdir).expect("err loading servers");

    // create v2ray process
    let mut v2ray_cmd = Command::new(settings.v2ray.bin.as_str());
    v2ray_cmd.args(["-config", workdir.filepath("v2ray.json").as_str(), "&"]);
    let mut v2ray_process =
        utils::process::Process::new(&mut v2ray_cmd, workdir.filepath("v2ray.pid").as_str());

    // create v2ray process
    let worker_cmd = &mut Command::new("v2up");
    worker_cmd.args(["work"]);
    let worker_process =
        &mut utils::process::Process::new(worker_cmd, workdir.filepath("worker.pid").as_str());

    // new v2ray config
    let mut v2ray_config = v2ray::config::Config::load(workdir.filepath("v2ray.json").as_str())
        .expect("err loading v2ray config");

    // create context
    let mut ctx = context::Context {
        dir: &workdir,
        settings: &mut settings,
        config: &mut v2ray_config,
        v2ray_process: &mut v2ray_process,
        worker_process: worker_process,
        servers: servers,
    };

    match &cli.command {
        Some(Commands::Servers {}) => {
            servers::exec(&mut ctx);
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
            stop::exec(&mut ctx);
        }
        Some(Commands::Init {}) => {
            // just do nothing, init is done before
        }
        Some(Commands::Version {}) => {
            println!("v2up version {}", env!("CARGO_PKG_VERSION"))
        }
        Some(Commands::Subscriptions { command }) => {
            subscriptions::exec(&mut ctx, command).unwrap();
        }
        None => {}
    }
    Ok(())
}
