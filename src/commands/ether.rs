//! 'tmkms ehter' CLI (sub)commands

//mod import;
//mod keygen;

mod import;
mod start;

//use self::{import::ImportCommand, keygen::KeygenCommand};
use abscissa_core::{Command, Help, Options, Runnable};
use crate::commands::ether::import::ImportCommand;
use crate::commands::ether::start::StartCommand;

/// 'ether' command: provides subcommands for local Ethereum signer
#[derive(Command, Debug, Options, Runnable)]
pub enum EtherCommand {
    #[options(help = "show help for the 'ether' subcommand")]
    Help(Help<Self>),
    #[options(help = "import Ethereum key")]
    Import(ImportCommand),
    #[options(help = "Start JSON-RPC Ethereum signing service")]
    Start(StartCommand),
}
