use clap::{Args, Subcommand};

use crate::Config;

#[derive(Args, Clone)]
pub struct SetArgs {
    #[command(subcommand)]
    command: SetCommand,
}

#[derive(Subcommand, Clone)]
pub enum SetCommand {
    /// set interface
    Interface(SetInterfaceArgs),
}

#[derive(Args, Clone)]
pub struct SetInterfaceArgs {
    #[arg()]
    interface: String,
    #[command(subcommand)]
    command: SetInterfaceCommand,
}

#[derive(Subcommand, Clone)]
pub enum SetInterfaceCommand {
    /// set interface up
    Up,
    /// set interface down
    Down,
    /// set interface listen port
    ListenPort(ListenPortInterfaceArgs),
    /// set interface private key
    PrivateKey(PrivateKeyInterfaceArgs),
    /// set interface peer
    Peer(SetInterfacePeerArgs),
}

#[derive(Args, Clone)]
pub struct ListenPortInterfaceArgs {
    #[arg()]
    port: u16,
}

#[derive(Args, Clone)]
pub struct PrivateKeyInterfaceArgs {
    #[arg()]
    private_key: String,
}

#[derive(Args, Clone)]
pub struct SetInterfacePeerArgs {
    #[arg()]
    public_key: Option<String>,
    #[command(subcommand)]
    command: SetInterfacePeerCommand,
}

#[derive(Subcommand, Clone)]
pub enum SetInterfacePeerCommand {
    /// set private key
    PrivateKey(PrivateKeyInterfacePeerArgs),
    /// set allowed ips
    AllowedIps(AllowedIpsInterfacePeerArgs),
    /// set public key
    PublicKey(PublicKeyInterfacePeerArgs),
    /// set endpoint
    Endpoint(EndpointInterfacePeerArgs),
    /// set preshared key
    PresharedKeys(PresharedKeysInterfacePeerArgs),
    /// set persistent keepalive
    PersistentKeepalive(PersistentKeepaliveInterfacePeerArgs),
}

#[derive(Args, Clone)]
pub struct PersistentKeepaliveInterfacePeerArgs {
    #[arg()]
    interval: u16,
}

#[derive(Args, Clone)]
pub struct PrivateKeyInterfacePeerArgs {
    #[arg()]
    private_key: Option<String>,
}

#[derive(Args, Clone)]
pub struct AllowedIpsInterfacePeerArgs {
    #[arg()]
    allowed_ips: Vec<String>,
}

#[derive(Args,Clone)]
pub struct PublicKeyInterfacePeerArgs {
    #[arg()]
    public_key: String,
}

#[derive(Args,Clone)]
pub struct EndpointInterfacePeerArgs {
    #[arg()]
    address: String,
    #[arg()]
    port: u16,
}

#[derive(Args,Clone)]
pub struct PresharedKeysInterfacePeerArgs {
    #[arg()]
    preshared_key: String,
}

pub fn set(config: &Config, args: &SetArgs) -> anyhow::Result<()> {
    match args.command {
        SetCommand::Interface(arg) => {
            set_interface(config, &arg)
        },
    }
}

pub fn set_interface(config: &Config, arg: &SetInterfaceArgs) -> anyhow::Result<()> {
    match arg.command {
        SetInterfaceCommand::Up => {
            set_interface_up(config, arg)
        },
        SetInterfaceCommand::Down => {
            set_interface_down(config, arg)
        },
        SetInterfaceCommand::ListenPort(arg) => {
            set_interface_listen_port(config, &arg)
        },
        SetInterfaceCommand::PrivateKey(arg) => {
            set_interface_private_key(config, &arg)
        },
        SetInterfaceCommand::Peer(arg) => {
            set_interface_peer(config, &arg)
        },
    }
}

pub fn set_interface_up(config: &Config, arg: &SetInterfaceArgs) -> anyhow::Result<()> {
    todo!()
}

pub fn set_interface_down(config: &Config, arg: &SetInterfaceArgs) -> anyhow::Result<()> {
    todo!()
}

pub fn set_interface_listen_port(config: &Config, arg: &ListenPortInterfaceArgs) -> anyhow::Result<()> {
    todo!()
}

pub fn set_interface_peer(config: &Config, arg: &SetInterfacePeerArgs) -> anyhow::Result<()> {
    todo!()
}

pub fn set_interface_private_key(config: &Config, arg: &PrivateKeyInterfaceArgs) -> anyhow::Result<()> {
    todo!()
}