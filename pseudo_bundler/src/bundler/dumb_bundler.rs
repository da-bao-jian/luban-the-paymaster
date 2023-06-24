use std::sync::Arc;
use ethers::{
    providers::{Middleware, Provider},
    types::{Address, U64, U256},
};
use jsonrpsee::{
	core::RpcResult, 
	proc_macros::rpc,
	tracing::info,
	types::{
		error::{CallError, ErrorCode},
		ErrorObject
	}
};
use aa_bundler_primitives::{
    UserOperation, UserOperationByHash, UserOperationGasEstimation, UserOperationHash,
    UserOperationPartial, UserOperationReceipt,
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// A simplified bundler implementation based on AA-Bundler
/// https://github.com/Vid201/aa-bundler
pub struct DumbBundler<M: Middleware> {

	/// The Provider that connects to Goerli
	pub eth_provider: Arc<M>,
	/// Goerli Chain ID
	pub eth_chain_id: U256,
	/// The Provider that connects to Mumbai
	pub poly_provider: Arc<M>,
	/// Mumbai Chain ID
	pub poly_chain_id: U256,
	/// Entry point address
	pub entry_point: Address,
	/// Max verification gas
	pub max_verification_gas: U256,
	/// Call gas Limit
	pub call_gas_limit: u64,

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
where M: Middleware + 'static,
	M::Provider: Send + Sync,
{
	async fn chain_id(&self) -> RpcResult<U64> {
        Ok(U64::default())
    }

    async fn supported_entry_points(&self) -> RpcResult<Vec<Address>> {
        Ok(vec![Address::default()])
    }

    async fn send_user_operation(
        &self,
        user_operation: UserOperation,
        entry_point: Address,
    ) -> RpcResult<UserOperationHash> {
        info!("{:?}", user_operation);
        info!("{:?}", entry_point);
        // Ok(SendUserOperationResponse::Success(H256::default()))
        let data = serde_json::value::to_raw_value(&"{\"a\": 100, \"b\": 200}").unwrap();
        Err(jsonrpsee::core::Error::Call(CallError::Custom(
            ErrorObject::owned(
                ErrorCode::ServerError(-32000).code(),
                "Not implemented",
                Some(data),
            ),
        )))
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




