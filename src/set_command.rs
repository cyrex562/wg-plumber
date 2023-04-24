use clap::{Args, Subcommand};

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
