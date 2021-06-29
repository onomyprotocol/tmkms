//! 'tmkms ether' CLI (sub)commands

mod import;
mod start;

use crate::commands::ether::{import::ImportCommand, start::StartCommand};
use abscissa_core::{Command, Help, Options, Runnable};

/// 'ether' command: provides subcommands for local Ethereum signer
#[derive(Command, Debug, Options, Runnable)]
pub enum EtherCommand {
    /// Show help for `ether` command
    #[options(help = "show help for the 'ether' subcommand")]
    Help(Help<Self>),
    /// Import Ethereum key
    #[options(help = "import Ethereum key")]
    Import(ImportCommand),
    /// Start JSON-RPC Ethereum signing service
    #[options(help = "Start JSON-RPC Ethereum signing service")]
    Start(StartCommand),
}
