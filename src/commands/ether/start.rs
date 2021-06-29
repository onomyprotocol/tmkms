//! `tmkms ether start` command

use crate::other_signers::eth_signer::EthTxSigner;
use crate::other_signers::rpc;
use abscissa_core::{status_err, Command, Options, Runnable};
use std::{path::PathBuf, process};

/// `start` command: starts signing service
#[derive(Command, Debug, Default, Options)]
pub struct StartCommand {
    #[options(free, help = "path to private key")]
    path: PathBuf,
}

impl Runnable for StartCommand {
    fn run(&self) {
        let signer = EthTxSigner::load_json_file(&self.path).unwrap_or_else(|e| {
            status_err!("couldn't load {}: {}", &self.path.display(), e);
            process::exit(1);
        });

        rpc::start_server(signer);
    }
}
