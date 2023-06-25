use crate::bindings::entrypointgoerli::entrypointgoerli;
use aa_bundler_primitives::{
    UserOperation, UserOperationByHash, UserOperationGasEstimation, UserOperationHash,
    UserOperationPartial, UserOperationReceipt, Wallet,
};
use anyhow::Result;
use async_trait::async_trait;
use ethers::{
    prelude::SignerMiddleware,
    providers::{Middleware, Provider},
    signers::Signer,
    types::{transaction::eip2718::TypedTransaction, Address, H160, U256, U64},
};
use jsonrpsee::{
    core::RpcResult,
    proc_macros::rpc,
    tracing::info,
    types::{error::ErrorCode, ErrorObject},
};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

/// A simplified bundler implementation based on AA-Bundler
/// https://github.com/Vid201/aa-bundler
pub struct DumbBundler<M: Middleware> {
    /// The Provider that connects to Goerli
    pub eth_provider: Arc<M>,
    /// Goerli Chain ID
    pub eth_chain_id: U64,
    /// The Provider that connects to Mumbai
    pub poly_provider: Arc<M>,
    /// Mumbai Chain ID
    pub poly_chain_id: U64,
    /// Entry point address
    pub entry_point: Address,
    /// Max verification gas
    pub max_verification_gas: U256,
    /// Call gas Limit
    pub call_gas_limit: U256,
    /// Bundler wallet
    pub wallet: Wallet,
}

impl<M> DumbBundler<M>
where
    M: Middleware + 'static,
    M::Provider: Send + Sync + 'static,
{
    pub fn new(
        eth_provider: Arc<M>,
        poly_provider: Arc<M>,
        max_verification_gas: U256,
        call_gas_limit: U256,
        wallet: Wallet,
    ) -> Self {
        Self {
            eth_provider,
            eth_chain_id: U64::from(5),
            poly_provider,
            poly_chain_id: U64::from(80001),
            entry_point: H160::from_str("0x5FF137D4b0FDCD49DcA30c7CF57E578a026d2789").unwrap(),
            max_verification_gas,
            call_gas_limit,
            wallet,
        }
    }
}

/// Eth API trait ported from AA-Bundler
///  https://github.com/Vid201/aa-bundler/blob/main/crates/rpc/src/eth_api.rs
#[derive(Serialize, Deserialize, Clone)]
pub struct EstimateUserOperationGasResponse {
    pub pre_verification_gas: U256,
    pub verification_gas_limit: U256,
    pub call_gas_limit: U256,
}

#[rpc(server, namespace = "eth")]
pub trait EthApi {
    #[method(name = "chainId")]
    async fn chain_id(&self) -> RpcResult<U64>;
    #[method(name = "supportedEntryPoints")]
    async fn supported_entry_points(&self) -> RpcResult<Vec<Address>>;
    #[method(name = "sendUserOperation")]
    async fn send_user_operation(
        &self,
        user_operation: UserOperation,
        entry_point: Address,
    ) -> RpcResult<UserOperationHash>;
    #[method(name = "estimateUserOperationGas")]
    async fn estimate_user_operation_gas(
        &self,
        user_operation: UserOperation,
        entry_point: Address,
    ) -> RpcResult<EstimateUserOperationGasResponse>;
    #[method(name = "getUserOperationReceipt")]
    async fn get_user_operation_receipt(
        &self,
        user_operation_hash: UserOperationHash,
    ) -> RpcResult<Option<UserOperationReceipt>>;
}

#[async_trait]
impl<M> EthApiServer for DumbBundler<M>
where
    M: Middleware + 'static,
    M::Provider: Send + Sync,
{
    async fn chain_id(&self) -> RpcResult<U64> {
        Ok(U64::from(80001))
    }

    async fn supported_entry_points(&self) -> RpcResult<Vec<Address>> {
        Ok(vec![H160::from_str(
            "0x5FF137D4b0FDCD49DcA30c7CF57E578a026d2789",
        )
        .unwrap()])
    }

    async fn send_user_operation(
        &self,
        user_operation: UserOperation,
        entry_point: Address,
    ) -> RpcResult<UserOperationHash> {
        let wallet = Arc::new(SignerMiddleware::new(
            self.poly_provider.clone(),
            self.wallet.signer.clone(),
        ));

        let entry_point_instance =
            entrypointgoerli::entrypointgoerli::new(entry_point, wallet.clone());

        let nonce = wallet
            .clone()
            .get_transaction_count(self.wallet.signer.address(), None)
            .await
            .unwrap();

        let mut user_op_vec = Vec::new();
        user_op_vec.push(user_operation.clone());
        let mut tx: TypedTransaction = entry_point_instance
            .handle_ops(user_op_vec, self.wallet.signer.address())
            .tx
            .clone();
        tx.set_nonce(nonce).set_chain_id(U64::from(80001));

        let tx = wallet
            .send_transaction(tx, None)
            .await
            .unwrap()
            .interval(Duration::from_millis(75));
        let tx_hash = tx.tx_hash();

        return Ok(UserOperationHash(tx_hash));
    }

    async fn estimate_user_operation_gas(
        &self,
        user_operation: UserOperation,
        entry_point: Address,
    ) -> RpcResult<EstimateUserOperationGasResponse> {
        info!("{:?}", user_operation);
        info!("{:?}", entry_point);
        Ok(EstimateUserOperationGasResponse {
            pre_verification_gas: U256::from(0),
            verification_gas_limit: U256::from(0),
            call_gas_limit: U256::from(self.call_gas_limit),
        })
    }

    async fn get_user_operation_receipt(
        &self,
        user_operation_hash: UserOperationHash,
    ) -> RpcResult<Option<UserOperationReceipt>> {
        info!("{:?}", user_operation_hash);
        Ok(None)
    }
}
