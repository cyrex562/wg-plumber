use clap::{Args, Subcommand};

use crate::Config;

#[derive(Args,Clone)]
pub struct DeleteArgs{
    #[command(subcommand)]
    command: DeleteCommand,
}

#[derive(Subcommand,Clone)]
pub enum DeleteCommand {
    /// delete interface
    Interface(DeleteInterfaceArgs),
}

#[derive(Args,Clone)]
pub struct DeleteInterfaceArgs {
    #[arg()]
    interface: String,
    #[command(subcommand)]
    command: DeleteInterfaceCommand,
}

#[derive(Subcommand,Clone)]
pub enum DeleteInterfaceCommand {
    /// delete interface peer
    Peer(DeleteInterfacePeerArgs),
}

#[derive(Args,Clone)]
pub struct DeleteInterfacePeerArgs {
    /// peer public key
    #[arg()]
    public_key: String,
}

pub fn delete(config: &Config, args: &DeleteArgs) -> anyhow::Result<()> {
    match args.command {
        DeleteCommand::Interface(arg) => {
            delete_interface(config, &arg)
        },
    }
}

fn delete_interface(config: &Config, arg: &DeleteInterfaceArgs) -> Result<(), anyhow::Error> {
    todo!()
}