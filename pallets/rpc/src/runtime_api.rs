#![cfg_attr(not(feature = "std"), no_std)]

use codec::Codec;
use sp_std::vec::Vec;
use compose_primitives::{
	GetContractResult,
};

sp_api::decl_runtime_apis! {
	/// The API to interact with contracts without using executive.
	pub trait ComposeApi<AccountId, Balance, BlockNumber, Hash> where
		AccountId: Codec,
		Balance: Codec,
		BlockNumber: Codec,
		Hash: Codec,
	{
		/// Perform a call from a specified account to a given contract.
		///
		/// See [`compose::named_call`].
		fn named_call(
			origin: AccountId,
			contract_path: &str,
			value: Balance,
			gas_limit: u64,
			input_data: Vec<u8>,
		) -> ContractExecResult;

		/// Instantiate a new contract.
		///
		/// See [`compose::named_instantiate`].
		fn named_instantiate(
			origin: AccountId,
			endowment: Balance,
			gas_limit: u64,
            contract_path: &str,
			code: Code<Hash>,
			data: Vec<u8>,
			salt: Vec<u8>,
		) -> ContractInstantiateResult<AccountId, BlockNumber>;

		/// Query a given storage key in a given contract.
		///
		/// Returns `Ok(Some(Vec<u8>))` if the storage value exists under the given key in the
		/// specified account and `Ok(None)` if it doesn't. If the account specified by the address
		/// doesn't exist, or doesn't have a contract or if the contract is a tombstone, then `Err`
		/// is returned.
		fn get_named_contract(
			contract_path: &str,
		) -> GetContractResult;
	}
}
