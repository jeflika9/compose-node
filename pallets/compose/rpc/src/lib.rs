// This file is part of Substrate.

// Copyright (C) 2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![warn(missing_docs)]

//! Node-specific RPC methods for interaction with Merkle Mountain Range pallet.

use std::sync::Arc;

use codec::{Codec}; //, Encode};
use jsonrpc_core::{Error, ErrorCode, Result};
use jsonrpc_derive::rpc;
use std::convert::{TryFrom}; //, TryInto};
// use serde::{Deserialize, Serialize};
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
// use sp_core::Bytes;
use sp_rpc::number::NumberOrHex;
use sp_runtime::{
	// generic::BlockId,
	traits::{
		Block as BlockT,
		Header as HeaderT,
	},
};

// ### RPC REQUIRED ###
use compose_primitives::{
	NamedContract,
	Error as ComposeError,
};

// ### RPC REQUIRED ###
pub use compose_primitives::ComposeApi as ComposeRuntimeApi;

/// Compose RPC methods.
#[rpc]
pub trait ComposeApi<BlockHash, BlockNumber, AccountId, Balance, Hash> {
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
        contract_path: Vec<u8>,
	) -> Result<NamedContract<AccountId, Hash>>;
}

// ### RPC REQUIRED ###
/// An implementation of MMR specific RPC methods.
pub struct Compose<C, B> {
	client: Arc<C>,
	_marker: std::marker::PhantomData<B>,
}

// ### RPC REQUIRED ###
impl<C, B> Compose<C, B> {
	/// Create new `Mmr` with the given reference to the client.
	pub fn new(client: Arc<C>) -> Self {
		Self {
			client,
			_marker: Default::default(),
		}
	}
}

// ### RPC REQUIRED ###

impl<C, Block, AccountId, Balance, Hash> 
	ComposeApi<
		<Block as BlockT>::Hash,
		<<Block as BlockT>::Header as HeaderT>::Number,
		AccountId,
		Balance,
		Hash,
	> for Compose<C, Block>
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
	// fn generate_proof(
	// 	&self,
	// 	leaf_index: u64,
	// 	at: Option<<Block as BlockT>::Hash>,
	// ) -> Result<LeafProof<<Block as BlockT>::Hash>> {
	// 	let api = self.client.runtime_api();
	// 	let block_hash = at.unwrap_or_else(||
	// 		// If the block hash is not supplied assume the best block.
	// 		self.client.info().best_hash
	// 	);

	// 	let (leaf, proof) = api
	// 		.generate_proof_with_context(
	// 			&BlockId::hash(block_hash),
	// 			sp_core::ExecutionContext::OffchainCall(None),
	// 			leaf_index,
	// 		)
	// 		.map_err(runtime_error_into_rpc_error)?
	// 		.map_err(mmr_error_into_rpc_error)?;

	// 	Ok(LeafProof::new(block_hash, leaf, proof))
	// }

	fn get_named_contract(&self, contract_path: Vec<u8>) -> Result<NamedContract<AccountId, Hash>> {
		Err(compose_error_into_rpc_error(ComposeError::NotFound(contract_path)))
		// Ok(NamedContract::<AccountId, Hash> {
		// 	path: contract_path,
		// 	id: AccountId::new(0u32),
		// 	code: Hash::new(0u32),
		// })
	}
}

// const RUNTIME_ERROR: i64 = 9000;
const COMPOSE_ERROR: i64 = 9010;

// ### RPC REQUIRED ###
/// Converts a mmr-specific error into an RPC error.
fn compose_error_into_rpc_error(err: ComposeError) -> Error {
	match err {
		ComposeError::NotFound(contract_path) => Error {
			code: ErrorCode::ServerError(COMPOSE_ERROR + 1),
			message: format!("Contract at {:?} was not found", contract_path).into(),
			data: Some(format!("Contract at {:?} was not found", contract_path).into()),
		},
		// _ => Error {
		// 	code: ErrorCode::ServerError(COMPOSE_ERROR + 2),
		// 	message: "Unexpected Compose error".into(),
		// 	data: Some(format!("{:?}", err).into()),
		// },
	}
}

// /// Converts a runtime trap into an RPC error.
// fn runtime_error_into_rpc_error(err: impl std::fmt::Debug) -> Error {
// 	Error {
// 		code: ErrorCode::ServerError(RUNTIME_ERROR),
// 		message: "Runtime trapped".into(),
// 		data: Some(format!("{:?}", err).into()),
// 	}
// }

// #[cfg(test)]
// mod tests {
// 	use super::*;
// 	use sp_core::H256;

// 	#[test]
// 	fn should_serialize_leaf_proof() {
// 		// given
// 		let leaf = vec![1_u8, 2, 3, 4];
// 		let proof = Proof {
// 			leaf_index: 1,
// 			leaf_count: 9,
// 			items: vec![H256::repeat_byte(1), H256::repeat_byte(2)],
// 		};

// 		let leaf_proof = LeafProof::new(H256::repeat_byte(0), leaf, proof);

// 		// when
// 		let actual = serde_json::to_string(&leaf_proof).unwrap();

// 		// then
// 		assert_eq!(
// 			actual,
// 			r#"{"blockHash":"0x0000000000000000000000000000000000000000000000000000000000000000","leaf":"0x1001020304","proof":"0x010000000000000009000000000000000801010101010101010101010101010101010101010101010101010101010101010202020202020202020202020202020202020202020202020202020202020202"}"#
// 		);
// 	}

// 	#[test]
// 	fn should_deserialize_leaf_proof() {
// 		// given
// 		let expected = LeafProof {
// 			block_hash: H256::repeat_byte(0),
// 			leaf: Bytes(vec![1_u8, 2, 3, 4].encode()),
// 			proof: Bytes(Proof {
// 				leaf_index: 1,
// 				leaf_count: 9,
// 				items: vec![H256::repeat_byte(1), H256::repeat_byte(2)],
// 			}.encode()),
// 		};

// 		// when
// 		let actual: LeafProof<H256> = serde_json::from_str(r#"{
// 			"blockHash":"0x0000000000000000000000000000000000000000000000000000000000000000",
// 			"leaf":"0x1001020304",
// 			"proof":"0x010000000000000009000000000000000801010101010101010101010101010101010101010101010101010101010101010202020202020202020202020202020202020202020202020202020202020202"
// 		}"#).unwrap();

// 		// then
// 		assert_eq!(actual, expected);

// 	}
// }
