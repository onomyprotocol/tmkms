//! `tmkms ether import` command

use crate::{config::provider::softsign::KeyFormat, key_utils, prelude::*};
use std::{path::PathBuf, process};
use abscissa_core::{Command, Options, Runnable, status_err};
use crate::other_signers::eth_signer::{EthTxSigner, GetSignerCredentials};
use crate::other_signers::*;

/// `import` command: import an ethereum keypair
#[derive(Command, Debug, Default, Options)]
pub struct ImportCommand {
    #[options(
    short = "f",
    help = "key format to import: 'json' or 'raw' (default 'json')"
    )]
    format: Option<String>,

    #[options(free, help = "[INPUT] and [OUTPUT] paths for key generation")]
    paths: Vec<PathBuf>,
}

impl Runnable for ImportCommand {
    fn run(&self) {
        if self.paths.len() != 2 {
            status_err!("expected 2 arguments, got {}", self.paths.len());
            eprintln!("\nUsage: tmkms softsign import [priv_validator.json] [output.key]");
            process::exit(1);
        }

        let input_path = &self.paths[0];
        let output_path = &self.paths[1];

        let format = self
            .format
            .as_ref()
            .map(|f| {
                f.parse::<KeyFormat>().unwrap_or_else(|e| {
                    status_err!("{} (must be 'json' or 'raw')", e);
                    process::exit(1);
                })
            })
            .unwrap_or(KeyFormat::Json);

        if format != KeyFormat::Json {
            status_err!("invalid format: {:?} (must be 'json')", format);
            process::exit(1);
        }

        let private_key = EthTxSigner::load_json_file(input_path)
            .unwrap_or_else(|e| {
                status_err!("couldn't load {}: {}", input_path.display(), e);
                process::exit(1);
            });

        key_utils::write_base64_secret(output_path, &private_key.get_private_key()).unwrap_or_else(
            |e| {
                status_err!("{}", e);
                process::exit(1);
            },
        );

        info!("Imported ETH private key to {}", output_path.display());
    }
}