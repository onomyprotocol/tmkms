//! Tendermint Key Management System

#![deny(unsafe_code)]
#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]

// Map type used within this application
use std::collections::BTreeMap as Map;

pub use crate::application::KmsApplication;

#[cfg(not(any(feature = "softsign", feature = "yubihsm", feature = "ledger")))]
compile_error!(
    "please enable one of the following backends with cargo's --features argument: \
     yubihsm, ledgertm, softsign (e.g. --features=yubihsm)"
);

pub mod amino_types;
pub mod application;
pub mod chain;
pub mod client;
pub mod commands;
pub mod config;
pub mod connection;
pub mod error;
pub mod key_utils;
pub mod keyring;
/// Signers for non-Tendermint transactions
pub mod other_signers;
pub mod prelude;
pub mod rpc;
pub mod session;

#[cfg(feature = "tx-signer")]
pub mod tx_signer;

#[cfg(feature = "yubihsm")]
pub mod yubihsm;
