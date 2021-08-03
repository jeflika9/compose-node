use std::sync::Arc;
use std::convert::{TryFrom};
use compose_primitives::*;
use compose_runtime_api::*;
use jsonrpc_core::{Result};
use jsonrpc_derive::rpc;
use codec::Codec;
use sp_api::ProvideRuntimeApi;
use sp_core::{Bytes, H256};
use sp_blockchain::HeaderBackend;
use sp_rpc::number::NumberOrHex;
use sp_runtime::{
	generic::BlockId,
	traits::{Block as BlockT, Header as HeaderT},
};

/// Contracts RPC methods.
#[rpc]
pub trait ComposeRPCApi<BlockHash, BlockNumber, AccountId, Balance, Hash> {
	// /// Executes a call to a contract.
	// ///
	// /// This call is performed locally without submitting any transactions. Thus executing this
	// /// won't change any state. Nonetheless, the calling state-changing contracts is still possible.
	// ///
	// /// This method is useful for calling getter-like methods on contracts.
	// #[rpc(name = "compose_namedCall")]
	// fn named_call(
	// 	&self,
	// 	call_request: CallRequest<AccountId>,
	// 	at: Option<BlockHash>,
	// ) -> Result<ContractExecResult>;

	// /// Instantiate a new contract.
	// ///
	// /// This call is performed locally without submitting any transactions. Thus the contract
	// /// is not actually created.
	// ///
	// /// This method is useful for UIs to dry-run contract instantiations.
	// #[rpc(name = "compose_instantiate")]
	// fn named_instantiate(
	// 	&self,
	// 	instantiate_request: InstantiateRequest<AccountId, Hash>,
	// 	at: Option<BlockHash>,
	// ) -> Result<ContractInstantiateResult<AccountId, BlockNumber>>;

	/// Returns the value under a specified storage `key` in a contract given by `address` param,
	/// or `None` if it is not set.
	#[rpc(name = "compose_getNamedContract")]
	fn get_named_contract(
		&self,
        contract_path: Box<String>,
		at: Option<BlockHash>,
	) -> Result<NamedContract<AccountId>>;

}

/// An implementation of contract specific RPC methods.
pub struct NamedContracts<C, B> {
	client: Arc<C>,
	_marker: std::marker::PhantomData<B>,
}

impl<C, Block, AccountId, Balance, Hash>
	ComposeRPCApi<
		<Block as BlockT>::Hash,
		<<Block as BlockT>::Header as HeaderT>::Number,
		AccountId,
		Balance,
		Hash,
	> for NamedContracts<C, Block>
where
	Block: BlockT,
	C: Send + Sync + 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
	C::Api: ComposeRuntimeApi<
		Block,
		AccountId,
		Balance,
		<<Block as BlockT>::Header as HeaderT>::Number,
		Hash,
	>,
	AccountId: Codec,
	Balance: Codec + TryFrom<NumberOrHex>,
	Hash: Codec,
{
	// fn named_call(
	// 	&self,
	// 	call_request: CallRequest<AccountId>,
	// 	at: Option<<Block as BlockT>::Hash>,
	// ) -> Result<ContractExecResult> {
	// 	let api = self.client.runtime_api();
	// 	let at = BlockId::hash(at.unwrap_or_else(||
	// 		// If the block hash is not supplied assume the best block.
	// 		self.client.info().best_hash));

	// 	let CallRequest {
	// 		origin,
	// 		dest,
	// 		value,
	// 		gas_limit,
	// 		input_data,
	// 	} = call_request;

	// 	let value: Balance = decode_hex(value, "balance")?;
	// 	let gas_limit: Weight = decode_hex(gas_limit, "weight")?;
	// 	limit_gas(gas_limit)?;

	// 	let exec_result = api
	// 		// .call(&at, origin, dest, value, gas_limit, input_data.to_vec())
    //         .named_call()
	// 		.map_err(runtime_error_into_rpc_err)?;

	// 	Ok(exec_result)
	// }

	// fn named_instantiate(
	// 	&self,
	// 	instantiate_request: InstantiateRequest<AccountId, Hash>,
	// 	at: Option<<Block as BlockT>::Hash>,
	// ) -> Result<ContractInstantiateResult<AccountId, <<Block as BlockT>::Header as HeaderT>::Number>> {
	// 	let api = self.client.runtime_api();
	// 	let at = BlockId::hash(at.unwrap_or_else(||
	// 		// If the block hash is not supplied assume the best block.
	// 		self.client.info().best_hash));

	// 	let InstantiateRequest {
	// 		origin,
	// 		endowment,
	// 		gas_limit,
	// 		code,
	// 		data,
	// 		salt,
	// 	} = instantiate_request;

	// 	let endowment: Balance = decode_hex(endowment, "balance")?;
	// 	let gas_limit: Weight = decode_hex(gas_limit, "weight")?;
	// 	limit_gas(gas_limit)?;

	// 	let exec_result = api
	// 		// .instantiate(&at, origin, endowment, gas_limit, code, data.to_vec(), salt.to_vec())
    //         .named_instantiate()
	// 		.map_err(runtime_error_into_rpc_err)?;

	// 	Ok(exec_result)
	// }

	fn get_named_contract(
		&self,
		contract_path: Box<String>,
		at: Option<<Block as BlockT>::Hash>,
	) -> Result<NamedContract<AccountId>> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

		let result = api
			.get_named_contract(&at, contract_path)
			.map_err(runtime_error_into_rpc_err)?;
			// .map_err(NamedContractError)?
			// .map(Bytes);

		Ok(result.unwrap())
	}
}
