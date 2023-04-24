use clap::{Args, Subcommand};

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
