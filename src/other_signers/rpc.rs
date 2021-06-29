/// JSON-RPC interface for transaction signing
use crate::other_signers::eth_signer::EthTxSigner;

use ethereum_tx_sign::RawTransaction;
use jsonrpc_core::{futures::FutureExt, BoxFuture, IoHandler, Result};
use jsonrpc_derive::rpc;
use jsonrpc_ws_server::ServerBuilder;

use std::path::Path;
use std::pin::Pin;

/// Ethereum signer RPC trait
#[rpc]
pub trait SignEthereumRpc {
    /// Sign ethereum transaction
    #[rpc(name = "sign_eth_tx")]
    fn sign_eth_tx(&self, tx: RawTransaction) -> BoxFuture<Result<Vec<u8>>>;
}

/// RPC implementation
#[derive(Clone)]
pub struct SignEthereumImpl {
    signer: Pin<Box<EthTxSigner>>,
}

impl SignEthereumImpl {
    /// Initiate signer from raw key
    pub fn from_key(raw_key: &[u8]) -> Result<Self> {
        EthTxSigner::new(raw_key)
            .map(|s| Self { signer: s.boxed() })
            .map_err(|e| {
                let message = format!("{}", e);
                jsonrpc_core::Error::invalid_params(message)
            })
    }

    /// Initiate signer from JSON file
    pub fn from_json_file<P: AsRef<Path>>(path: &P) -> Result<Self> {
        EthTxSigner::load_json_file(path)
            .map(|s| Self { signer: s.boxed() })
            .map_err(|e| {
                let message = format!("{}", e);
                jsonrpc_core::Error::invalid_params(message)
            })
    }
}

impl SignEthereumRpc for SignEthereumImpl {
    fn sign_eth_tx(&self, tx: RawTransaction) -> BoxFuture<Result<Vec<u8>>> {
        let clone = self.clone();
        let a = async {
            let clone = clone;
            Ok(clone.signer.sign_eth_transaction(tx))
        }
        .boxed();
        a
    }
}

/// Starts JSON-RPC server
pub fn start_server(signer: EthTxSigner) {
    let mut io = IoHandler::default();
    let pinned = Box::pin(signer);
    let signer = SignEthereumImpl { signer: pinned };
    io.extend_with(signer.to_delegate());

    let server = ServerBuilder::new(io)
        .start(&"0.0.0.0:3030".parse().unwrap())
        .expect("Server must start with no issues");

    server.wait().unwrap()
}

#[test]
fn test_signing() {
    futures::executor::block_on(async {
        let mut io = IoHandler::new();
        let mut data: [u8; 32] = Default::default();
        data.copy_from_slice(
            &hex::decode("2a3526dd05ad2ebba87673f711ef8c336115254ef8fcd38c4d8166db9a8120e4")
                .unwrap(),
        );
        let signer = SignEthereumImpl::from_key(&data).unwrap();
        io.extend_with(signer.to_delegate());
        println!("Starting local server");
        let (client, server) = local::connect(io);
        let client = use_client(client).fuse();
        let server = server.fuse();

        futures::pin_mut!(client);
        futures::pin_mut!(server);

        futures::select! {
            _server = server => {},
            _client = client => {},
        }
    });
}

/// Hint: SignEthereumRpcClient is generated by [#rpc] macro.
#[test]
async fn use_client(client: SignEthereumRpcClient) {
    let msg = ethereum_tx_sign::RawTransaction {
        nonce: ethereum_types::U256::from(0),
        to: Some(ethereum_types::H160::zero()),
        value: Default::default(),
        gas_price: ethereum_types::U256::from(10000),
        gas: ethereum_types::U256::from(21240),
        data: hex::decode(
            "7f7465737432000000000000000000000000000000000000000000000000000000600057",
        )
        .unwrap(),
    };

    println!("{}", serde_json::to_string_pretty(&msg).unwrap());

    let mut data: [u8; 32] = Default::default();
    data.copy_from_slice(
        &hex::decode("2a3526dd05ad2ebba87673f711ef8c336115254ef8fcd38c4d8166db9a8120e4").unwrap(),
    );
    let private_key = ethereum_types::H256(data);
    let raw_rlp_bytes = client.sign_eth_tx(msg).await.unwrap();
    let result = "f885808227108252f894000000000000000000000000000000000000000080a\
        47f746573743200000000000000000000000000000000000000000000000000\
        00006000572aa0b4e0309bc4953b1ca0c7eb7c0d15cc812eb4417cbd759aa09\
        3d38cb72851a14ca036e4ee3f3dbb25d6f7b8bd4dac0b4b5c717708d20ae6ff\
        08b6f71cbf0b9ad2f4";

    assert_eq!(result, hex::encode(raw_rlp_bytes));
}
