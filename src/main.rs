use std::{any, path::PathBuf};

use clap::{command, Args, Parser, Subcommand};
use serde::Deserialize;

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
    peers: Vec<Peer>,
}

#[derive(Deserialize)]
struct Interface {}

#[derive(Deserialize)]
struct Peer {}

#[derive(Subcommand)]
enum Commands {
    /// create a interface
    CreateInterface(CreateInterfaceArgs),
    /// update a interface
    UpdateInterface(UpdateInterfaceArgs),
    /// delete a interface
    DeleteInterface(DeleteInterfaceArgs),
    /// start a connection
    StartConnection(StartConnectionArgs),
    /// stop a interface
    StopConnection(StopConnectionArgs),
    /// start all interfaces
    StartAllConnections(StartAllConnectionsArgs),
    /// stop all interfaces
    StopAllConnections(StopAllConnectionsArgs),
    /// create a peer
    CreatePeer(CreatePeerArgs),
    /// update a peer
    UpdatePeer(UpdatePeerArgs),
    /// delete a peer
    DeletePeer(DeletePeerArgs),
    /// list peers
    ListConnectionPeers(ListConnectionPeersArgs),
    /// list all peers for all interfaces
    ListAllConnectionPeers(ListAllConnectionPeersArgs),
    /// run as a server
    Server(ServerArgs),
    /// Single Connection Status
    ConnectionStatus(ConnectionStatusArgs),
    /// All Connection statuses
    AllConnectionStatuses(AllConnectionStatusesArgs),
}

#[derive(Args)]
struct ConnectionStatusArgs {}

#[derive(Args)]
struct AllConnectionStatusesArgs {}

#[derive(Args)]
struct StartConnectionArgs {}

#[derive(Args)]
struct StopConnectionArgs {}

#[derive(Args)]
struct StartAllConnectionsArgs {}

#[derive(Args)]
struct DeletePeerArgs {}

#[derive(Args)]
struct ListAllConnectionPeersArgs {}

#[derive(Args)]
struct ListConnectionPeersArgs {}

#[derive(Args)]
struct ServerArgs {}

#[derive(Args)]
struct StopAllConnectionsArgs {}

#[derive(Args)]
struct CreateInterfaceArgs {}

#[derive(Args)]
struct UpdateInterfaceArgs {}

#[derive(Args)]
struct DeleteInterfaceArgs {}

#[derive(Args)]
struct CreatePeerArgs {}

#[derive(Args)]
struct UpdatePeerArgs {}

fn create_interface(config: &Config, args: &CreateInterfaceArgs) -> anyhow::Result<()> {
    Ok(())
}

fn delete_interface(config: &Config, args: &DeleteInterfaceArgs) -> anyhow::Result<()> {
    Ok(())
}

fn start_connection(config: &Config, args: &StartConnectionArgs) -> anyhow::Result<()> {
    Ok(())
}

fn stop_connection(config: &Config, args: &StopConnectionArgs) -> anyhow::Result<()> {
    Ok(())
}

fn start_all_connections(config: &Config, args: &StartAllConnectionsArgs) -> anyhow::Result<()> {
    Ok(())
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
        Commands::CreateInterface(args) => {
            println!("CreateInterface");
            create_interface(&config, args)?;
        }
        Commands::UpdateInterface(args) => {
            println!("UpdateInterface");
            update_interface(&config, args)?;
        }
        Commands::DeleteInterface(args) => {
            println!("DeleteInterface");
            delete_interface(&config, args)?;
        }
        Commands::StartConnection(args) => {
            println!("StartConnection");
            start_connection(&config, args)?;
        }
        Commands::StopConnection(args) => {
            println!("StopConnection");
            stop_connection(&config, args)?;
        }
        Commands::StartAllConnections(args) => {
            println!("StartAllConnections");
            start_all_connections(&config, args)?;
        }
        Commands::StopAllConnections(args) => {
            println!("StopAllConnections");
            stop_all_connections(&config, args)?;
        }
        Commands::CreatePeer(args) => {
            println!("CreatePeer");
            create_peer(&config, args)?;
        }

        Commands::UpdatePeer(args) => {
            println!("UpdatePeer");
            update_peer(&config, args)?;
        }
        Commands::DeletePeer(args) => {
            println!("DeletePeer");
            delete_peer(&config, args)?;
        }
        Commands::ListConnectionPeers(args) => {
            println!("ListConnectionPeers");
            list_connection_peers(&config, args)?;
        }
        Commands::ListAllConnectionPeers(args) => {
            println!("ListAllConnectionPeers");
            list_all_connection_peers(&config, args)?;
        }
        Commands::Server(args) => {
            println!("Server");
            run_server(&config, args)?;
        }
        Commands::ConnectionStatus(args) => {
            println!("ConnectionStatus");
            connection_status(&config, args)?;
        }
        Commands::AllConnectionStatuses(args) => {
            println!("AllConnectionStatuses");
            all_connection_statuses(&config, args)?;
        }
    };
    Ok(())
}

fn all_connection_statuses(
    config: &Config,
    args: &AllConnectionStatusesArgs,
) -> anyhow::Result<()> {
    todo!()
}

fn connection_status(config: &Config, args: &ConnectionStatusArgs) -> anyhow::Result<()> {
    todo!()
}

fn run_server(config: &Config, args: &ServerArgs) -> anyhow::Result<()> {
    todo!()
}

fn list_all_connection_peers(
    config: &Config,
    args: &ListAllConnectionPeersArgs,
) -> anyhow::Result<()> {
    todo!()
}

fn stop_all_connections(config: &Config, args: &StopAllConnectionsArgs) -> anyhow::Result<()> {
    todo!()
}

fn list_connection_peers(config: &Config, args: &ListConnectionPeersArgs) -> anyhow::Result<()> {
    todo!()
}

fn delete_peer(config: &Config, args: &DeletePeerArgs) -> anyhow::Result<()> {
    todo!()
}

fn update_peer(config: &Config, args: &UpdatePeerArgs) -> anyhow::Result<()> {
    todo!()
}

fn create_peer(config: &Config, args: &CreatePeerArgs) -> anyhow::Result<()> {
    todo!()
}

fn update_interface(config: &Config, args: &UpdateInterfaceArgs) -> anyhow::Result<()> {
    todo!()
}
