//! Ethereum transaction signing configuration

use std::convert::TryInto;
use ethereum_tx_sign;
use ethereum_tx_sign::RawTransaction;
use std::path::Path;
use std::fs;
use tendermint::error::{Error, Kind};
use ethereum_types::H256;
use serde::{Deserialize, Serialize};
use anomaly::format_err;
use secp256k1::key::{SecretKey, PublicKey};

const ETH_MAIN_NET_ID: u32 = 1;
const ETH_ROPSTEIN_NET_ID: u32 = 3;

#[derive(Deserialize, Serialize)]
pub struct EthTxSigner {
    private_key: H256,
}

impl EthTxSigner {
    pub fn new(raw_key: &[u8]) -> Result<Self, Error> {
        let raw_private_key: &[u8; 32] = raw_key.try_into()?;
        let private_key: H256 = raw_private_key.try_into()?;
        Ok(Self { private_key })
    }

    pub fn parse_json<T: AsRef<str>>(json_string: T) -> Result<Self, Error> {
        let result = serde_json::from_str::<Self>(json_string.as_ref())?;
        Ok(result)
    }
    pub fn load_json_file<P>(path: &P) -> Result<Self, Error>
        where
            P: AsRef<Path>,
    {
        let json_string = fs::read_to_string(path).map_err(|e| {
            format_err!(
                Kind::Parse,
                "couldn't open {}: {}",
                path.as_ref().display(),
                e
            )
        })?;

        Self::parse_json(json_string)
    }

    /// Local raw transaction signing, does not connect to any remotes.
    pub fn sign_eth_transaction(&self, tx: RawTransaction) -> Vec<u8> {
        let private_key = self.get_private_key();
        let private_key: [u8; 32] = private_key
            .try_into()
            .unwrap_or_else(|_| panic!("Ethereum private key must be 256 bits"));
        let private_key = H256(private_key);
        let signature = tx.sign(&private_key, &ETH_ROPSTEIN_NET_ID);
        signature
    }
}

/// Helper trait to get signer's credentials.
pub trait GetSignerCredentials {
    /// Get raw private key
    fn get_private_key(&self) -> Vec<u8>;
    /// Get raw public key
    fn get_public_key(&self) -> Vec<u8>;
}

impl GetSignerCredentials for EthTxSigner{
    fn get_private_key(&self) -> Vec<u8> {
        self.private_key.as_bytes().to_vec()
    }

    fn get_public_key(&self) -> Vec<u8> {
        let context = secp256k1::Secp256k1::new();
        let private_key_bytes = self.private_key.0;
        let secret_key = SecretKey::from_slice(&private_key_bytes).unwrap();
        let public_key = PublicKey::from_secret_key(&context, &secret_key);
        let pubkey_serialized = public_key.serialize();
        pubkey_serialized.to_vec()
    }
}

#[test]
pub fn test_ser_eth_key() {
    let mut data: [u8; 32] = Default::default();
    data.copy_from_slice(&hex::decode(
        "2a3526dd05ad2ebba87673f711ef8c336115254ef8fcd38c4d8166db9a8120e4"
    ).unwrap());
    let private_key = ethereum_types::H256(data);
    let signer = EthTxSigner{private_key};
    let s = serde_json::to_string_pretty(&signer).unwrap();
    println!("{}", s);
}

#[test]
pub fn test_sign() {
    const ETH_CHAIN_ID: u32 = 3; // 1 for mainnet, 3 for ropstein
    let ganache = Ganache::new().spawn();
    let wallet = LocalWallet::new(&mut thread_rng());
    let wallet_ganache = ganache.keys()[0].clone();

    let wallet_pkey = wallet.get_private_key();

    let tx = ethereum_tx_sign::RawTransaction {
        nonce: ethereum_types::U256::from(0),
        to: Some(ethereum_types::H160::zero()),
        value: Default::default(),
        gas_price: ethereum_types::U256::from(10000),
        gas: ethereum_types::U256::from(21240),
        data: hex::decode(
            "7f7465737432000000000000000000000000000000000000000000000000000000600057"
        ).unwrap()
    };

    let mut data: [u8; 32] = Default::default();
    data.copy_from_slice(&hex::decode(
        "2a3526dd05ad2ebba87673f711ef8c336115254ef8fcd38c4d8166db9a8120e4"
    ).unwrap());
    let private_key = ethereum_types::H256(data);
    let raw_rlp_bytes = tx.sign(&private_key, &ETH_CHAIN_ID);

    let result = "f885808227108252f894000000000000000000000000000000000000000080a\
    47f746573743200000000000000000000000000000000000000000000000000\
    00006000572aa0b4e0309bc4953b1ca0c7eb7c0d15cc812eb4417cbd759aa09\
    3d38cb72851a14ca036e4ee3f3dbb25d6f7b8bd4dac0b4b5c717708d20ae6ff\
    08b6f71cbf0b9ad2f4";

    println!("Foo");
    assert_eq!(result, hex::encode(raw_rlp_bytes));
}

#[test]
pub fn test_sign_transfer() {
    const ETH_CHAIN_ID: u32 = 3; // 1 for mainnet, 3 for ropstein
    let ganache = Ganache::new().spawn();
    let wallet = LocalWallet::new(&mut thread_rng());
    let wallet_ganache = ganache.keys()[0].clone();

    let wallet_pkey = wallet.get_private_key();

    let tx = ethereum_tx_sign::RawTransaction {
        nonce: ethereum_types::U256::from(0),
        to: Some(ethereum_types::H160::zero()),
        value: Default::default(),
        gas_price: ethereum_types::U256::from(10000),
        gas: ethereum_types::U256::from(21240),
        data: hex::decode(
            "7f7465737432000000000000000000000000000000000000000000000000000000600057"
        ).unwrap()
    };

    let mut data: [u8; 32] = Default::default();
    data.copy_from_slice(&hex::decode(
        "2a3526dd05ad2ebba87673f711ef8c336115254ef8fcd38c4d8166db9a8120e4"
    ).unwrap());
    let private_key = ethereum_types::H256(data);
    let raw_rlp_bytes = tx.sign(&private_key, &ETH_CHAIN_ID);
    let provider = Provider::<Http>::try_from(ganache.endpoint()).unwrap();
    // connect the wallet to the provider
    let client = Arc::new(SignerMiddleware::new(provider, wallet));
    let tx: ethers::types::Transaction = Transaction{
        hash: Default::default(),
        nonce: Default::default(),
        block_hash: None,
        block_number: None,
        transaction_index: None,
        from: Default::default(),
        to: None,
        value: Default::default(),
        gas_price: Default::default(),
        gas: Default::default(),
        input: Default::default(),
        v: Default::default(),
        r: Default::default(),
        s: Default::default()
    };

    let h = hex::encode(raw_rlp_bytes);

    println!("{}", h);
    println!("{}", ganache.endpoint());
}