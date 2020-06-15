//! Cryptographic service providers: signing backends

#[cfg(feature = "ledgertm")]
pub mod ledgertm;
#[cfg(feature = "softsign")]
pub mod softsign;
#[cfg(feature = "yubihsm")]
pub mod yubihsm;

#[cfg(feature = "ledgertm")]
use self::ledgertm::LedgerTendermintConfig;
#[cfg(feature = "softsign")]
use self::softsign::SoftsignConfig;
#[cfg(feature = "yubihsm")]
use self::yubihsm::YubihsmConfig;
use serde::Deserialize;

/// Provider configuration
#[derive(Default, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ProviderConfig {
    /// Software-backed signer
    #[cfg(feature = "softsign")]
    #[serde(default)]
    pub softsign: Vec<SoftsignConfig>,

    /// Map of yubihsm-connector labels to their configurations
    #[cfg(feature = "yubihsm")]
    #[serde(default)]
    pub yubihsm: Vec<YubihsmConfig>,

    /// Map of ledger-tm labels to their configurations
    #[cfg(feature = "ledgertm")]
    #[serde(default)]
    pub ledgertm: Vec<LedgerTendermintConfig>,
}

/// Types of cryptographic keys
// TODO(tarcieri): move this into a provider-agnostic module
#[derive(Clone, Debug, Deserialize)]
pub enum KeyType {
    /// Account keys
    #[serde(rename = "account")]
    Account,

    /// Consensus keys
    #[serde(rename = "consensus")]
    Consensus,
}

impl Default for KeyType {
    /// Backwards compat for existing configuration files
    fn default() -> Self {
        KeyType::Consensus
    }
}
