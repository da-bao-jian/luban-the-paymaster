use std::sync::Arc;
use ethers::{
    providers::{Middleware, Provider},
    types::{Address, U256},
};

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
}




// pub use crate::eth::EthApiServerImpl;
// use aa_bundler_primitives::{
//     UserOperation, UserOperationByHash, UserOperationGasEstimation, UserOperationHash,
//     UserOperationPartial, UserOperationReceipt,
// };
// use ethers::types::{Address, U64};
// use jsonrpsee::{core::RpcResult, proc_macros::rpc};

// #[rpc(server, namespace = "eth")]
// pub trait EthApi {
//     #[method(name = "chainId")]
//     async fn chain_id(&self) -> RpcResult<U64>;

//     #[method(name = "supportedEntryPoints")]
//     async fn supported_entry_points(&self) -> RpcResult<Vec<String>>;

//     #[method(name = "sendUserOperation")]
//     async fn send_user_operation(
//         &self,
//         user_operation: UserOperation,
//         entry_point: Address,
//     ) -> RpcResult<UserOperationHash>;

//     #[method(name = "estimateUserOperationGas")]
//     async fn estimate_user_operation_gas(
//         &self,
//         user_operation: UserOperationPartial,
//         entry_point: Address,
//     ) -> RpcResult<UserOperationGasEstimation>;

//     #[method(name = "getUserOperationReceipt")]
//     async fn get_user_operation_receipt(
//         &self,
//         user_operation_hash: String,
//     ) -> RpcResult<Option<UserOperationReceipt>>;

//     #[method(name = "getUserOperationByHash")]
//     async fn get_user_operation_by_hash(
//         &self,
//         user_operation_hash: String,
//     ) -> RpcResult<Option<UserOperationByHash>>;
// }
