use beerus_core::lightclient::beerus::BeerusLightClient;
/// The RPC module for the Ethereum protocol required by Kakarot.
use jsonrpsee::{
    core::{async_trait, RpcResult as Result},
    proc_macros::rpc,
};
// use std::str::FromStr;

pub struct BeerusRpc {
    _beerus: BeerusLightClient,
}

#[rpc(server, client)]
trait BeerusApi {
    // Ethereum methods

    // // Returns information about a block by hash
    // #[method(name = "eth_getBlockByHash")]
    // async fn eth_get_block_by_hash(
    //     &self,
    //     hash: &str,
    //     full_tx: &str,
    // ) -> Result<Option<serde_json::Value>>;

    // Returns sync status
    #[method(name = "eth_syncing")]
    async fn eth_syncing(&self) -> Result<bool>;

    // Returns the chain ID of the current network
    #[method(name = "eth_chainId")]
    async fn eth_chain_id(&self) -> Result<u64>;

    // Returns the number of most recent block
    #[method(name = "eth_blockNumber")]
    async fn eth_block_number(&self) -> Result<u64>;

    // Starknet methods

    // Return the currently configured Starknet chain id
    #[method(name = "stark_chainId")]
    async fn stark_chain_id(&self) -> Result<String>;

    // Get the most recent accepted block number
    #[method(name = "stark_blockNumber")]
    async fn stark_block_number(&self) -> Result<u64>;
}

#[async_trait]
impl BeerusApiServer for BeerusRpc {
    // Ethereum functions

    // async fn eth_get_block_by_hash(
    //     &self,
    //     hash: &str,
    //     full_tx: &str,
    // ) -> Result<Option<serde_json::Value>> {
    //     let hash: Vec<u8> = hash[2..]
    //         .chars()
    //         .map(|c| u8::from_str_radix(&c.to_string(), 16).unwrap())
    //         .collect();
    //     let full_tx = bool::from_str(full_tx).unwrap();
    //     let block = self
    //         ._beerus
    //         .ethereum_lightclient
    //         .read()
    //         .await
    //         .get_block_by_hash(&hash, full_tx)
    //         .await
    //         .unwrap();
    //     Ok(block)
    // }

    async fn eth_syncing(&self) -> Result<bool> {
        Ok(true)
    }

    async fn eth_chain_id(&self) -> Result<u64> {
        let chain_id = self
            ._beerus
            .ethereum_lightclient
            .read()
            .await
            .chain_id()
            .await;
        Ok(chain_id)
    }

    async fn eth_block_number(&self) -> Result<u64> {
        let block_number = self
            ._beerus
            .ethereum_lightclient
            .read()
            .await
            .get_block_number()
            .await
            .unwrap();
        Ok(block_number)
    }

    // Starknet functions
    async fn stark_chain_id(&self) -> Result<String> {
        let chain_id = self
            ._beerus
            .starknet_lightclient
            .chain_id()
            .await
            .unwrap()
            .to_string();
        Ok(chain_id)
    }

    async fn stark_block_number(&self) -> Result<u64> {
        let block_number = self
            ._beerus
            .starknet_lightclient
            .block_number()
            .await
            .unwrap();
        Ok(block_number)
    }
}

impl BeerusRpc {
    pub fn new(beerus: BeerusLightClient) -> Self {
        Self { _beerus: beerus }
    }
}
