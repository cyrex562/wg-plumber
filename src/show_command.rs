use anyhow::bail;
use clap::{Args, Subcommand, Arg};

use crate::Config;


#[derive(Args, Clone)]
pub struct ShowArgs{
    #[command(subcommand)]
    command: ShowCommand,
}



#[derive(Subcommand, Clone)]
pub enum ShowCommand {
    /// Show interface
    Interface(ShowInterfaceArgs),
}

#[derive(Args, Clone)]
pub struct ShowInterfaceArgs {
    #[arg()]
    interface: String,
    #[command(subcommand)]
    command: Option<ShowInterfaceCommand>,
}

#[derive(Subcommand, Clone)]
pub enum ShowInterfaceCommand {
    /// interface public key
    PublicKey,
    /// interface private key
    PrivateKey,
    /// pre-shared keys
    PresharedKeys,
    /// endpoints
    Endpoints,
    /// allowed ips
    AllowedIps,
    /// latest handshakes
    Handshakes,
    /// transfers
    Transfers,
    /// keepalives
    Keepalives,
    /// listen port for interface
    ListenPort,
    /// dump current state
    Dump
}

fn show(config: &Config, args: &ShowArgs) -> anyhow::Result<()> {
    match args.command {
        ShowCommand::Interface(arg) => {
            show_interface(config, &arg)
        },
    }
}

fn show_interface(config: &Config, args: &ShowInterfaceArgs) -> anyhow::Result<()> {

    if args.interface == "brief" {
        println!("Show interface brief");
        return show_interface_brief(config);
    }

    if args.command.is_some() {
        return match args.command.unwrap() {
            ShowInterfaceCommand::PublicKey => {
                println!("Show interface public key");
                show_ifc_pub_key(config, &args.interface)
            },
            ShowInterfaceCommand::PrivateKey => {
                println!("Show interface private key");
                show_ifc_priv_key(config, &args.interface)
            },
            ShowInterfaceCommand::PresharedKeys => {
                println!("Show interface preshared keys");
                show_ifc_psks(config, &args.interface)
            },
            ShowInterfaceCommand::Endpoints => {
                println!("Show interface endpoints");
                show_ifc_endpoints(config, &args.interface)
            },
            ShowInterfaceCommand::AllowedIps => {
                println!("Show interface allowed ips");
                show_ifc_allowed_ips(config, &args.interface)
            },
            ShowInterfaceCommand::Handshakes => {
                println!("Show interface handshakes");
                show_ifc_handshakes(config, &args.interface)
            },
            ShowInterfaceCommand::Transfers => {
                println!("Show interface transfers");
                show_ifc_transfers(config, &args.interface)
            },
            ShowInterfaceCommand::Keepalives => {
                println!("Show interface keepalives");
                show_ifc_keepalives(config, &args.interface)
            },
            ShowInterfaceCommand::ListenPort => todo!(),
            ShowInterfaceCommand::Dump => todo!(),
        }
    }

    bail!("no subcommand specified for show interface command")
}

fn show_ifc_keepalives(config: &Config, interface: &str) -> Result<(), anyhow::Error> {
    todo!()
}

fn show_ifc_transfers(config: &Config, interface: &str) -> Result<(), anyhow::Error> {
    todo!()
}

fn show_ifc_handshakes(config: &Config, interface: &str) -> Result<(), anyhow::Error> {
    todo!()
}

fn show_ifc_allowed_ips(config: &Config, interface: &str) -> Result<(), anyhow::Error> {
    todo!()
}

fn show_ifc_endpoints(config: &Config, interface: &str) -> Result<(), anyhow::Error> {
    todo!()
}

fn show_ifc_psks(config: &Config, interface: &str) -> Result<(), anyhow::Error> {
    todo!()
}

fn show_ifc_priv_key(config: &Config, interface: &str) -> Result<(), anyhow::Error> {
    todo!()
}

fn show_interface_brief(config: &Config) -> anyhow::Result<()> {
    println!("Show interface brief");
    todo!()
}

fn show_ifc_pub_key(config: &Config, ifc_name: &str) -> anyhow::Result<()> {
    println!("Show interface public key");
    todo!()
}