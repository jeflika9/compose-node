#![cfg_attr(not(feature = "std"), no_std)]

use codec::Codec;
// use sp_std::vec::Vec;
use compose_primitives::NamedContract;

sp_api::decl_runtime_apis! {
	/// The API to interact with contracts without using executive.
	pub trait ComposeRuntimeApi</* automatically added by decl_runtime_apis: Block, */ AccountId, Balance, BlockNumber, Hash> where
		// Block: BlockT automatically added by decl_runtime_apis; see "fn extend_generics_with_block"
		AccountId: Codec,
		Balance: Codec,
		BlockNumber: Codec,
		Hash: Codec,
	{
		// /// Perform a call from a specified account to a given contract.
		// ///
		// /// See [`compose::named_call`].
		// fn named_call(
		// 	origin: AccountId,
		// 	contract_path: &str,
		// 	value: Balance,
		// 	gas_limit: u64,
		// 	input_data: Vec<u8>,
		// ) -> ContractExecResult;

		// /// Instantiate a new contract.
		// ///
		// /// See [`compose::named_instantiate`].
		// fn named_instantiate(
		// 	origin: AccountId,
		// 	endowment: Balance,
		// 	gas_limit: u64,
        //     contract_path: &str,
		// 	code: Code<Hash>,
		// 	data: Vec<u8>,
		// 	salt: Vec<u8>,
		// ) -> ContractInstantiateResult<AccountId, BlockNumber>;

		/// Query the key details on a given contract, so that it may be called.
		///
		/// Returns `Some(NamedContract)` if the contract exists at the given path and `None` if it doesn't.
		/// See [`compose::get_named_contract`].
		fn get_named_contract(
			contract_path: Box<String>,
		) -> Option<NamedContract<AccountId>>;
	}
}
