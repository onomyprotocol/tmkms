//! `tmkms ether import` command

use crate::other_signers::eth_signer::{EthTxSigner, GetSignerCredentials};
use crate::{config::provider::softsign::KeyFormat, key_utils, prelude::*};
use abscissa_core::{status_err, Command, Options, Runnable};
use std::{path::PathBuf, process};

/// `import` command: import an ethereum private key. Gets .json, stores raw as base64
#[derive(Command, Debug, Default, Options)]
pub struct ImportCommand {
    #[options(
        short = "f",
        help = "key format to import: 'json' or 'raw' (default 'json')"
    )]
    format: Option<String>,

    #[options(free, help = "[INPUT] and [OUTPUT] paths for key")]
    paths: Vec<PathBuf>,
}

impl Runnable for ImportCommand {
    fn run(&self) {
        if self.paths.len() != 2 {
            status_err!("expected 2 arguments, got {}", self.paths.len());
            eprintln!("\nUsage: tmkms ether import [priv_key_json.json] [priv_key_base64.json]");
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

        let private_key = EthTxSigner::load_json_file(input_path).unwrap_or_else(|e| {
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
