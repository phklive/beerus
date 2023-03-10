use beerus_core::lightclient::beerus::BeerusLightClient;
use ethers::{
    types::{Address, SyncingStatus},
    utils,
};
use helios::types::{BlockTag, CallOpts, ExecutionBlock};
/// The RPC module for the Ethereum protocol required by Kakarot.
use jsonrpsee::{
    core::{async_trait, RpcResult as Result},
    proc_macros::rpc,
};
use std::str::FromStr;

pub struct BeerusRpc {
    _beerus: BeerusLightClient,
}

#[rpc(server, client)]
trait BeerusApi {
    // Ethereum methods

    // Returns information about a block by hash
    #[method(name = "eth_getBlockByHash")]
    async fn eth_get_block_by_hash(
        &self,
        hash: &str,
        full_tx: &str,
    ) -> Result<Option<ExecutionBlock>>;

    // Returns information about a block by number
    #[method(name = "eth_getBlockByNumber")]
    async fn eth_get_block_by_number(
        &self,
        block_tag: &str,
        full_tx: &str,
    ) -> Result<Option<ExecutionBlock>>;

    // Returns the number of uncles in a block from a block matching the given block number
    #[method(name = "eth_getBlockTransactionCountByNumber")]
    async fn eth_get_block_transaction_count_by_number(&self, block_tag: &str) -> Result<u64>;

    // Returns the number of transactions in a block matching the given block hash
    #[method(name = "eth_getBlockTransactionCountByHash")]
    async fn eth_get_block_transaction_count_by_hash(&self, hash: &str) -> Result<u64>;

    // Returns the chain ID of the current network
    #[method(name = "eth_chainId")]
    async fn eth_chain_id(&self) -> Result<u64>;

    // Returns the number of most recent block
    #[method(name = "eth_blockNumber")]
    async fn eth_block_number(&self) -> Result<u64>;

    // Generates and returns an estimate of how much gas is necessary to allow the transaction to complete
    #[method(name = "eth_estimateGas")]
    async fn eth_estimate_gas(&self, opts: CallOpts) -> Result<String>;

    // Returns the current price per gas in wei
    #[method(name = "eth_gasPrice")]
    async fn eth_gas_price(&self) -> Result<String>;

    // Returns the current maxPriorityFeePerGas per gas in wei
    #[method(name = "eth_maxPriorityFeePerGas")]
    async fn eth_max_priority_fee_per_gas(&self) -> Result<String>;

    // Returns the balance of the account of given address
    #[method(name = "eth_getBalance")]
    async fn eth_get_balance(&self, address: &str) -> Result<String>;

    // Returns the number of transactions sent from an address
    #[method(name = "eth_getTransactionCount")]
    async fn eth_get_transaction_count(&self, address: &str) -> Result<u64>;

    // Returns code at given address
    #[method(name = "eth_getCode")]
    async fn eth_get_code(&self, address: &str) -> Result<Vec<u8>>;

    // Submits a raw transaction
    #[method(name = "eth_sendRawTransaction")]
    async fn eth_send_raw_transaction(&self, bytes: &str) -> Result<String>;

    // Executes a new message call immediately without creating a transaction on the blockchain
    #[method(name = "eth_call")]
    async fn eth_call(&self, opts: CallOpts, block_tag: &str) -> Result<String>;

    // // Returns an array of all logs matching filter with given id
    // #[method(name = "eth_getLogs")]
    // async fn eth_get_logs(&self, filter: Filter) -> Result<Vec<Log>>;

    // // Returns the value from a storage position at given address
    // #[method(name = "eth_getStorageAt")]
    // async fn eth_get_storage_at(
    //     &self,
    //     address: &str,
    //     slot: H256,
    //     block_tag: &str,
    // ) -> Result<String>;

    // TODO: Impliment
    // // Returns the receipt of a transaction by transaction hash
    // #[method(name = "eth_getTransactionReceipt")]
    // async fn eth_get_transaction_receipt(&self, hash: &str) -> Result<Option<TransactionReceipt>>;

    // Returns an object with data about the sync status or false
    #[method(name = "eth_syncing")]
    async fn eth_syncing(&self) -> Result<SyncingStatus>;

    // Returns the client coinbase address
    #[method(name = "eth_coinbase")]
    async fn eth_coinbase(&self) -> Result<Address>;
}

#[async_trait]
impl BeerusApiServer for BeerusRpc {
    // Ethereum functions

    async fn eth_get_block_by_hash(
        &self,
        hash: &str,
        full_tx: &str,
    ) -> Result<Option<ExecutionBlock>> {
        let hash: Vec<u8> = hash[2..]
            .chars()
            .map(|c| u8::from_str_radix(&c.to_string(), 16).unwrap())
            .collect();

        let full_tx = bool::from_str(full_tx).unwrap();

        let block = self
            ._beerus
            .ethereum_lightclient
            .read()
            .await
            .get_block_by_hash(&hash, full_tx)
            .await
            .unwrap();

        Ok(block)
    }

    async fn eth_get_block_by_number(
        &self,
        block_tag: &str,
        full_tx: &str,
    ) -> Result<Option<ExecutionBlock>> {
        let full_tx = bool::from_str(full_tx).unwrap();
        let block_tag: String = serde_json::to_string(&block_tag).unwrap();
        let block_tag: BlockTag = serde_json::from_str(block_tag.as_str()).unwrap();
        let block = self
            ._beerus
            .ethereum_lightclient
            .read()
            .await
            .get_block_by_number(block_tag, full_tx)
            .await
            .unwrap();

        Ok(block)
    }

    async fn eth_get_block_transaction_count_by_number(&self, block_tag: &str) -> Result<u64> {
        let block_tag: String = serde_json::to_string(&block_tag).unwrap();
        let block_tag: BlockTag = serde_json::from_str(block_tag.as_str()).unwrap();
        let block_transaction_count = self
            ._beerus
            .ethereum_lightclient
            .read()
            .await
            .get_block_transaction_count_by_number(block_tag)
            .await
            .unwrap();

        Ok(block_transaction_count)
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

    async fn eth_get_balance(&self, address: &str) -> Result<String> {
        // Parse the Ethereum address.
        let addr = Address::from_str(address).unwrap();
        // TODO: Make the block tag configurable.
        let block = BlockTag::Latest;
        // Query the balance of the Ethereum address.
        let balance = self
            ._beerus
            .ethereum_lightclient
            .read()
            .await
            .get_balance(&addr, block)
            .await
            .unwrap();

        // Format the balance in Ether.
        let balance_in_eth = utils::format_units(balance, "ether").unwrap();

        Ok(balance_in_eth)
    }

    // TODO: impliment on ethereum lightclient
    // async fn eth_get_transaction_receipt(&self, hash: &str) -> Result<Option<TransactionReceipt>> {
    //     let tx_receipt = self._beerus.ethereum_lightclient.read().await
    // }

    // async fn eth_get_logs(&self, filter: Filter) -> Result<Vec<Log>> {
    //     let logs = self._beerus.ethereum_lightclient.read().await.get_logs(from_block, to_block, &filter.address, &filter.topics)
    // }

    // async fn eth_get_storage_at(
    //     &self,
    //     address: &str,
    //     slot: H256,
    //     block_tag: &str,
    // ) -> Result<String> {
    //    // Parse the Ethereum address.
    //    let addr = Address::from_str(address).unwrap();
    //    // Parse block_tag
    //    let block_tag: String = serde_json::to_string(&block_tag).unwrap();
    //    let block_tag: BlockTag = serde_json::from_str(block_tag.as_str()).unwrap();
    //    let storage = self._beerus.ethereum_lightclient.read().await.get_st
    // }

    async fn eth_send_raw_transaction(&self, bytes: &str) -> Result<String> {
        // Parse bytes
        let bytes: Vec<u8> = bytes[2..]
            .chars()
            .map(|c| u8::from_str_radix(&c.to_string(), 16).unwrap())
            .collect();
        let bytes_slice: &[u8] = bytes.as_ref();
        // Send Raw Transaction.
        let response = self
            ._beerus
            .ethereum_lightclient
            .read()
            .await
            .send_raw_transaction(bytes_slice)
            .await
            .unwrap()
            .to_string();
        Ok(response)
    }

    async fn eth_get_block_transaction_count_by_hash(&self, hash: &str) -> Result<u64> {
        // Parse hash
        let hash: Vec<u8> = hash[2..]
            .chars()
            .map(|c| u8::from_str_radix(&c.to_string(), 16).unwrap())
            .collect();

        let tx_count = self
            ._beerus
            .ethereum_lightclient
            .read()
            .await
            .get_block_transaction_count_by_hash(&hash)
            .await
            .unwrap();
        Ok(tx_count)
    }

    async fn eth_get_transaction_count(&self, address: &str) -> Result<u64> {
        // Parse the Ethereum address.
        let addr = Address::from_str(address).unwrap();
        // TODO: Make the block tag configurable.
        let block = BlockTag::Latest;
        // Query the balance of the Ethereum address.
        let nonce = self
            ._beerus
            .ethereum_lightclient
            .read()
            .await
            .get_nonce(&addr, block)
            .await
            .unwrap();
        Ok(nonce)
    }

    async fn eth_get_code(&self, address: &str) -> Result<Vec<u8>> {
        // Parse the Ethereum address.
        let addr = Address::from_str(address).unwrap();
        // TODO: Make the block tag configurable.
        let block = BlockTag::Latest;
        let code = self
            ._beerus
            .ethereum_lightclient
            .read()
            .await
            .get_code(&addr, block)
            .await
            .unwrap();
        Ok(code)
    }

    async fn eth_gas_price(&self) -> Result<String> {
        let gas_price = self
            ._beerus
            .ethereum_lightclient
            .read()
            .await
            .get_gas_price()
            .await
            .unwrap()
            .to_string();
        Ok(gas_price)
    }

    async fn eth_estimate_gas(&self, opts: CallOpts) -> Result<String> {
        let gas = self
            ._beerus
            .ethereum_lightclient
            .read()
            .await
            .estimate_gas(&opts)
            .await
            .unwrap()
            .to_string();
        Ok(gas)
    }

    async fn eth_max_priority_fee_per_gas(&self) -> Result<String> {
        let max_fee = self
            ._beerus
            .ethereum_lightclient
            .read()
            .await
            .get_priority_fee()
            .await
            .unwrap()
            .to_string();
        Ok(max_fee)
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

    async fn eth_call(&self, opts: CallOpts, block_tag: &str) -> Result<String> {
        let block_tag: String = serde_json::to_string(&block_tag).unwrap();
        let block_tag: BlockTag = serde_json::from_str(block_tag.as_str()).unwrap();
        let res = self
            ._beerus
            .ethereum_lightclient
            .read()
            .await
            .call(&opts, block_tag)
            .await
            .unwrap();
        Ok(format!("0x{}", hex::encode(res)))
    }

    async fn eth_syncing(&self) -> Result<SyncingStatus> {
        let syncing = self
            ._beerus
            .ethereum_lightclient
            .read()
            .await
            .get_syncing()
            .await
            .unwrap();
        Ok(syncing)
    }

    // TODO: implement coinbase on ethereum lightclient
    async fn eth_coinbase(&self) -> Result<Address> {
        let coinbase = self
            ._beerus
            .ethereum_lightclient
            .read()
            .await
            .get_coinbase()
            .await
            .unwrap();
        Ok(coinbase)
    }
}

impl BeerusRpc {
    pub fn new(beerus: BeerusLightClient) -> Self {
        Self { _beerus: beerus }
    }
}
