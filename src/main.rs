extern crate core;

use clap::Parser;
use clap::Subcommand;

mod commands;
use commands::servers;
use commands::subscriptions;

mod errors;

mod v2ray;

mod context;
mod settings;

use settings::Settings;

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Servers {},
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

fn main() {
    let cli = Cli::parse();

    // load settings
    let result = Settings::load("/Users/larry/.v2up/settings.yaml");
    if result.is_err() {
        println!("err on loading settings: {}", result.err().unwrap().message);
        std::process::exit(-1);
    }
    let mut settings = result.unwrap();

    // load v2ray process
    let mut process = v2ray::process::Process::new();

    // new v2ray config
    let result = v2ray::config::Config::load("/Users/larry/.v2up/v2ray.json");
    if result.is_err() {
        println!("err on loading config: {}", result.err().unwrap().message);
        std::process::exit(-1)
    }

    let mut config = result.unwrap();

    // create context
    let mut ctx = context::Context {
        settings: &mut settings,
        config: &mut config,
        process: &mut process,
    };

    match &cli.command {
        Some(Commands::Servers {}) => servers::exec(&ctx),
        Some(Commands::Subscriptions { command }) => {
            subscriptions::exec(&mut ctx, command).unwrap()
        }
        None => {}
    }

    println!("verbose: {}", cli.verbose)
}
