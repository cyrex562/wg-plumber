use std::path::PathBuf;
mod show_command;
mod delete_command;
mod set_command;
mod server;

use clap::{command, Args, Parser, Subcommand};
use serde::Deserialize;
use show_command::ShowArgs;
use set_command::SetArgs;
use delete_command::DeleteArgs;
use server::{server, ServerArgs};

use crate::{show_command::show, set_command::set, delete_command::delete};

#[derive(Parser)]
#[command(author,version,about,long_about=None)]
struct Cli {
    #[arg(default_value_t=String::from("config.toml"),
          help="Path to the config file",
          short='c',
          long="config")]
    config: String,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Deserialize)]
struct Config {
    interfaces: Vec<Interface>,
}

#[derive(Deserialize)]
struct Interface {
    peers: Vec<Peer>
}

#[derive(Deserialize)]
struct Peer {}

#[derive(Subcommand)]
enum Commands {
    /// show
    Show(ShowArgs),
    /// set
    Set(SetArgs),
    /// delete
    Delete(DeleteArgs),
    /// create a interface
    Server(ServerArgs),
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let config_path = PathBuf::from(cli.config);
    if config_path.exists() {
        println!("Config file {} exists", config_path.to_str().unwrap());
    } else {
        panic!(
            "Config file {} does not exist",
            config_path.to_str().unwrap()
        );
    }

    let config: Config =
        toml::from_str(std::fs::read_to_string(config_path).unwrap().as_str()).unwrap();

    match &cli.command {
        Commands::Show(args) => {
            println!("Show");
            show(&config, args)?;
        }
        Commands::Set(args) => {
            println!("Set");
            set(&config, args)?;
        }
        Commands::Delete(args) => {
            println!("Delete");
            delete(&config, args)?;
        }
        Commands::Server(args) => {
            let runtime = actix_rt::Runtime::new()?;
            runtime.block_on(async {
                server(&config, args).await;
            });
        }
    };
    Ok(())
}
