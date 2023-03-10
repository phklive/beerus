use beerus_core::lightclient::beerus::BeerusLightClient;
use ethers::{
    types::{Address, Transaction},
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

    // // Returns the number of transactions in a block matching the given block hash
    // #[method(name = "eth_getBlockTransactionCountByHash")]
    // async fn eth_get_block_transaction_count_by_hash(&self, hash: &str) -> Result<u64>;

    // Returns the number of uncles in a block from a block matching the given block number
    #[method(name = "eth_getBlockTransactionCountByNumber")]
    async fn eth_get_block_transaction_count_by_number(&self, block_tag: &str) -> Result<u64>;

    // // Returns the number of uncles in a block from a block matching the given block hash
    // #[method(name = "eth_getUncleCountByBlockHash")]
    // async fn eth_get_uncle_count_by_block_hash(&self, hash: &str) -> Result<u64>;

    // // Returns the number of transactions in a block matching the given block number
    // #[method(name = "eth_getUncleCountByBlockNumber")]
    // async fn eth_get_uncle_count_by_block_number(&self, block_tag: &str) -> Result<u64>;

    // Returns the chain ID of the current network
    #[method(name = "eth_chainId")]
    async fn eth_chain_id(&self) -> Result<u64>;

    // // Returns an object with data about the sync status or false
    // #[method(name = "eth_syncing")]
    // async fn eth_syncing(&self) -> Result<SyncingStatus>;

    // // Returns the client coinbase address
    // #[method(name = "eth_coinbase")]
    // async fn eth_coinbase(&self) -> Result<Address>;

    // // Returns a list of addresses owned by client
    // #[method(name = "eth_accounts")]
    // async fn eth_accounts(&self) -> Result<bool>;

    // Returns the number of most recent block
    #[method(name = "eth_blockNumber")]
    async fn eth_block_number(&self) -> Result<u64>;

    // TODO: impliment
    // // Executes a new message call immediately without creating a transaction on the blockchain
    // #[method(name = "eth_call")]
    // async fn eth_call(&self, opts: CallOpts, block_tag: &str) -> Result<String>;

    // Generates and returns an estimate of how much gas is necessary to allow the transaction to complete
    #[method(name = "eth_estimateGas")]
    async fn eth_estimate_gas(&self, opts: CallOpts) -> Result<String>;

    // // Generates an access list for a transaction
    // #[method(name = "eth_createAccessList")]
    // async fn eth_createAccessList(&self) -> Result<bool>;

    // Returns the current price per gas in wei
    #[method(name = "eth_gasPrice")]
    async fn eth_gas_price(&self) -> Result<String>;

    // Returns the current maxPriorityFeePerGas per gas in wei
    #[method(name = "eth_maxPriorityFeePerGas")]
    async fn eth_max_priority_fee_per_gas(&self) -> Result<String>;

    // // Transaction fee history
    // #[method(name = "eth_feeHistory")]
    // async fn eth_feeHistory(&self) -> Result<bool>;

    // // Creates a filter object, based on filter options, to notify when the state changes (logs)
    // #[method(name = "eth_newFilter")]
    // async fn eth_newFilter(&self) -> Result<bool>;

    // // Creates a filter in the node, to notify when a new block arrives
    // #[method(name = "eth_newBlockFilter")]
    // async fn eth_newBlockFilter(&self) -> Result<bool>;

    // // Creates a filter in the node, to notify when new pending transactions arrive
    // #[method(name = "eth_newPendingTransactionFilter")]
    // async fn eth_newPendingTransactionFilter(&self) -> Result<bool>;

    // // Uninstalls a filter with given id
    // #[method(name = "eth_uninstallFilter")]
    // async fn eth_uninstallFilter(&self) -> Result<bool>;

    // // Polling method for a filter, which returns an array of logs which occured since last poll
    // #[method(name = "eth_getFilterChanges")]
    // async fn eth_getFilterChanges(&self) -> Result<bool>;

    // // Returns an array of all logs matching filter with given id
    // #[method(name = "eth_getFilterLogs")]
    // async fn eth_getFilterLogs(&self) -> Result<bool>;

    // // Returns an array of all logs matching filter with given id
    // #[method(name = "eth_getLogs")]
    // async fn eth_getLogs(&self) -> Result<bool>;

    // // Returns whether the client is actively mining new blocks
    // #[method(name = "eth_mining")]
    // async fn eth_mining(&self) -> Result<bool>;

    // // Returns the number of hashes per second that the node is mining with
    // #[method(name = "eth_hashrate")]
    // async fn eth_hashrate(&self) -> Result<bool>;

    // // Returns the hash of the current block, the seedhash and the boundary condition to be met ("target")
    // #[method(name = "eth_getWork")]
    // async fn eth_getWork(&self) -> Result<bool>;

    // // Used for submitting a proof of work solution
    // #[method(name = "eth_submitWork")]
    // async fn eth_submitWork(&self) -> Result<bool>;

    // // Used for submitting mining hash rate
    // #[method(name = "eth_submitHashRate")]
    // async fn eth_submitHashRate(&self) -> Result<bool>;

    // // Returns an EIP-191 signature over the provided data
    // #[method(name = "eth_sign")]
    // async fn eth_sign(&self) -> Result<bool>;

    // // Returns an RLP-encoded transaction signed by the specified account
    // #[method(name = "eth_signTransaction")]
    // async fn eth_signTransaction(&self) -> Result<bool>;

    // Returns the balance of the account of given address
    #[method(name = "eth_getBalance")]
    async fn eth_get_balance(&self, address: &str) -> Result<String>;

    // // Returns the value from a storage position at given address
    // #[method(name = "eth_getStorageAt")]
    // async fn eth_getStorageAt(&self) -> Result<bool>;

    // Returns the number of transactions sent from an address
    #[method(name = "eth_getTransactionCount")]
    async fn eth_get_transaction_count(&self, address: &str) -> Result<u64>;

    // Returns code at given address
    #[method(name = "eth_getCode")]
    async fn eth_get_code(&self, address: &str) -> Result<Vec<u8>>;

    // // Returns the meerkle proof for a given account and optionally some storage keys
    // #[method(name = "eth_getProof")]
    // async fn eth_getProof(&self) -> Result<bool>;

    // // Signs and submits a transaction
    // #[method(name = "eth_sendTransaction")]
    // async fn eth_sendTransaction(&self) -> Result<bool>;

    // // Submits a raw transaction
    // #[method(name = "eth_sendRawTransaction")]
    // async fn eth_sendRawTransaction(&self) -> Result<bool>;

    // // Returns information about a transaction requested by transaction hash
    // #[method(name = "eth_getTransactionByHash")]
    // async fn eth_getTransactionByHash(&self) -> Result<bool>;

    // Returns information about a transaction by block hash and transaction index position
    #[method(name = "eth_getTransactionByBlockHashAndIndex")]
    async fn eth_get_transaction_by_block_hash_and_index(
        &self,
        hash: &str,
        index: u64,
    ) -> Result<Transaction>;

    // Returns information about a transaction by block number and transaction index position
    #[method(name = "eth_getTransactionByBlockNumberAndIndex")]
    async fn eth_get_transaction_by_block_number_and_index(
        &self,
        block_tag: &str,
        index: u64,
    ) -> Result<Transaction>;

    // // Returns the receipt of a transaction by transaction hash
    // #[method(name = "eth_getTransactionReceipt")]
    // async fn eth_getTransactionReceipt(&self) -> Result<bool>;

    // // Returns an RLP-encoded header
    // #[method(name = "debug_getRawHeader")]
    // async fn debug_getRawHeader(&self) -> Result<bool>;

    // // Returns an RLP-encoded block
    // #[method(name = "debug_getRawBlock")]
    // async fn debug_getRawBlock(&self) -> Result<bool>;

    // // Returns an array of EIP-2718 binary encoded transactions
    // #[method(name = "debug_getRawTransaction")]
    // async fn debug_getRawTransaction(&self) -> Result<bool>;

    // // Returns an array of EIP-2718 binary encoded receipts
    // #[method(name = "debug_getRawReceipts")]
    // async fn debug_getRawReceipts(&self) -> Result<bool>;

    // // Returns an array of recent bad blocks that the client has seen on the network
    // #[method(name = "debug_getBadBlocks")]
    // async fn debug_getBadBlocks(&self) -> Result<bool>;
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

    // // TODO: implement coinbase on ethereum lightclient
    // async fn eth_coinbase(&self) -> Result<Address> {
    //     let coinbase = self._beerus.ethereum_lightclient.read().await;
    //     Ok(coinbase)
    // }

    // // TODO: impliment syncing on ethereum lightclient
    // async fn eth_syncing(&self) -> Result<bool> {
    // }

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

    // // TODO: Implement syncing on ethereum lightclient
    // async fn eth_syncing(&self) -> Result<SyncingStatus> {
    //     let syncing = self._beerus.ethereum_lightclient.read().await
    // }

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

    // TODO: impliment
    // async fn eth_call(&self, opts: CallOpts, block_tag: &str) -> Result<String> {
    //     let block_tag: String = serde_json::to_string(&block_tag).unwrap();
    //     let block_tag: BlockTag = serde_json::from_str(block_tag.as_str()).unwrap();
    //     let call = self
    //         ._beerus
    //         .ethereum_lightclient
    //         .read()
    //         .await
    //         .call(&opts, block_tag)
    //         .await
    //         .unwrap();
    //     Ok(call)
    // }

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

    async fn eth_get_transaction_by_block_hash_and_index(
        &self,
        hash: &str,
        index: u64,
    ) -> Result<Transaction> {
        let hash: Vec<u8> = hash[2..]
            .chars()
            .map(|c| u8::from_str_radix(&c.to_string(), 16).unwrap())
            .collect();

        let block = self
            ._beerus
            .ethereum_lightclient
            .read()
            .await
            .get_block_by_hash(&hash, true)
            .await
            .unwrap();

        let transactions = match block {
            Some(x) => {
                let transactions = x.transactions;
                match transactions {
                    helios::prelude::Transactions::Full(x) => x,
                    helios::prelude::Transactions::Hashes(_) => vec![],
                }
            }
            None => vec![],
        };

        Ok(transactions[index as usize].clone())
    }

    async fn eth_get_transaction_by_block_number_and_index(
        &self,
        block_tag: &str,
        index: u64,
    ) -> Result<Transaction> {
        let block_tag: String = serde_json::to_string(&block_tag).unwrap();
        let block_tag: BlockTag = serde_json::from_str(block_tag.as_str()).unwrap();

        let block = self
            ._beerus
            .ethereum_lightclient
            .read()
            .await
            .get_block_by_number(block_tag, true)
            .await
            .unwrap();

        let transactions = match block {
            Some(x) => {
                let transactions = x.transactions;
                match transactions {
                    helios::prelude::Transactions::Full(x) => x,
                    helios::prelude::Transactions::Hashes(_) => vec![],
                }
            }
            None => vec![],
        };

        Ok(transactions[index as usize].clone())
    }
}

impl BeerusRpc {
    pub fn new(beerus: BeerusLightClient) -> Self {
        Self { _beerus: beerus }
    }
}
