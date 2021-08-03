// This file is part of Substrate.

// Copyright (C) 2020-2021 Parity Technologies (UK) Ltd.
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

//! Compose primitive types.

#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]

use frame_support::RuntimeDebug;
// use sp_runtime::traits::{self, Saturating, One};
use codec::{Codec, Encode, Decode};
// use jsonrpc_core::{Result};
use sp_std::fmt;
use sp_std::prelude::*;

#[cfg(feature = "std")]
use serde::{Serialize, Deserialize};

#[cfg(feature = "std")]
mod as_string {
	use super::*;
	use serde::{Serializer, Deserializer, ser::Error};

	pub fn serialize<S: Serializer>(bytes: &Vec<u8>, serializer: S) -> Result<S::Ok, S::Error> {
		std::str::from_utf8(bytes)
			.map_err(|e| S::Error::custom(format!("Debug buffer contains invalid UTF8: {}", e)))?
			.serialize(serializer)
	}

	pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<u8>, D::Error> {
		Ok(String::deserialize(deserializer)?.into_bytes())
	}
}

/// Compose operation error.
#[derive(Eq, PartialEq, Encode, Decode, RuntimeDebug)]
pub enum Error {
	/// Contract not found at path
	NotFound(Vec<u8>),
}

// ### RPC REQUIRED ###
impl Error {
	#![allow(unused_variables)]
	/// Consume given error `e` with `self` and generate a native log entry with error details.
	pub fn log_error(self, e: impl fmt::Debug) -> Self {
		log::error!(
			target: "runtime::compose",
			"[{:?}] Compose error: {:?}",
			self,
			e,
		);
		self
	}

	/// Consume given error `e` with `self` and generate a native log entry with error details.
	pub fn log_debug(self, e: impl fmt::Debug) -> Self {
		log::debug!(
			target: "runtime::compose",
			"[{:?}] Compose error: {:?}",
			self,
			e,
		);
		self
	}
}

/// The information about the contract at a given path required to make calls on the contract
#[derive(Eq, PartialEq, Encode, Decode, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct NamedContract<AccountId, Hash> {
	#[cfg_attr(feature = "std", serde(with = "as_string"))]
	/// The string representation of the path
    pub path: Vec<u8>,
	/// The owner of the contract
	pub owner:  AccountId,
	/// The hash of the contract code (if the path does not point to a contract
	/// [e.g., if the path is a domain without route], then code_hash is set to None)
    pub code_hash: Option<Hash>,
}

// pub type NamedContractResult<AccountId, Hash> = Result<NamedContract<AccountId, Hash>>;

// ### RPC REQUIRED ###
sp_api::decl_runtime_apis! {
	/// The API to interact with contracts without using executive.
	pub trait ComposeApi</* automatically added by decl_runtime_apis: Block, */ AccountId, Balance, BlockNumber, Hash> where
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
			contract_path: Vec<u8>,
		) -> Option<NamedContract<AccountId, Hash>>;
	}
}

// #[cfg(test)]
// mod tests {
// 	use super::*;

// 	use codec::Decode;
// 	use sp_core::H256;
// 	use sp_runtime::traits::Keccak256;

// 	pub(crate) fn hex(s: &str) -> H256 {
// 		s.parse().unwrap()
// 	}

// 	type Test = DataOrHash<Keccak256, String>;
// 	type TestCompact = Compact<Keccak256, (Test, Test)>;
// 	type TestProof = Proof<<Keccak256 as traits::Hash>::Output>;

// 	#[test]
// 	fn should_encode_decode_proof() {
// 		// given
// 		let proof: TestProof = Proof {
// 			leaf_index: 5,
// 			leaf_count: 10,
// 			items: vec![
// 				hex("c3e7ba6b511162fead58f2c8b5764ce869ed1118011ac37392522ed16720bbcd"),
// 				hex("d3e7ba6b511162fead58f2c8b5764ce869ed1118011ac37392522ed16720bbcd"),
// 				hex("e3e7ba6b511162fead58f2c8b5764ce869ed1118011ac37392522ed16720bbcd"),
// 			],
// 		};

// 		// when
// 		let encoded = codec::Encode::encode(&proof);
// 		let decoded = TestProof::decode(&mut &*encoded);

// 		// then
// 		assert_eq!(decoded, Ok(proof));
// 	}

// 	#[test]
// 	fn should_encode_decode_correctly_if_no_compact() {
// 		// given
// 		let cases = vec![
// 			Test::Data("Hello World!".into()),
// 			Test::Hash(hex("c3e7ba6b511162fead58f2c8b5764ce869ed1118011ac37392522ed16720bbcd")),
// 			Test::Data("".into()),
// 			Test::Data("3e48d6bcd417fb22e044747242451e2c0f3e602d1bcad2767c34808621956417".into()),
// 		];

// 		// when
// 		let encoded = cases
// 			.iter()
// 			.map(codec::Encode::encode)
// 			.collect::<Vec<_>>();

// 		let decoded = encoded
// 			.iter()
// 			.map(|x| Test::decode(&mut &**x))
// 			.collect::<Vec<_>>();

// 		// then
// 		assert_eq!(decoded, cases.into_iter().map(Result::<_, codec::Error>::Ok).collect::<Vec<_>>());
// 		// check encoding correctness
// 		assert_eq!(&encoded[0], &hex_literal::hex!("00343048656c6c6f20576f726c6421"));
// 		assert_eq!(
// 			encoded[1].as_slice(),
// 			hex_literal::hex!(
// 				"01c3e7ba6b511162fead58f2c8b5764ce869ed1118011ac37392522ed16720bbcd"
// 			).as_ref()
// 		);
// 	}

// 	#[test]
// 	fn should_return_the_hash_correctly() {
// 		// given
// 		let a = Test::Data("Hello World!".into());
// 		let b = Test::Hash(hex("c3e7ba6b511162fead58f2c8b5764ce869ed1118011ac37392522ed16720bbcd"));

// 		// when
// 		let a = a.hash();
// 		let b = b.hash();

// 		// then
// 		assert_eq!(a, hex("a9c321be8c24ba4dc2bd73f5300bde67dc57228ab8b68b607bb4c39c5374fac9"));
// 		assert_eq!(b, hex("c3e7ba6b511162fead58f2c8b5764ce869ed1118011ac37392522ed16720bbcd"));
// 	}

// 	#[test]
// 	fn compact_should_work() {
// 		// given
// 		let a = Test::Data("Hello World!".into());
// 		let b = Test::Data("".into());

// 		// when
// 		let c: TestCompact = Compact::new((a.clone(), b.clone()));
// 		let d: TestCompact = Compact::new((
// 			Test::Hash(a.hash()),
// 			Test::Hash(b.hash()),
// 		));

// 		// then
// 		assert_eq!(c.hash(), d.hash());
// 	}

// 	#[test]
// 	fn compact_should_encode_decode_correctly() {
// 		// given
// 		let a = Test::Data("Hello World!".into());
// 		let b = Test::Data("".into());

// 		let c: TestCompact = Compact::new((a.clone(), b.clone()));
// 		let d: TestCompact = Compact::new((
// 			Test::Hash(a.hash()),
// 			Test::Hash(b.hash()),
// 		));
// 		let cases = vec![c, d.clone()];

// 		// when
// 		let encoded_compact = cases
// 			.iter()
// 			.map(|c| c.using_encoded(|x| x.to_vec(), true))
// 			.collect::<Vec<_>>();

// 		let encoded = cases
// 			.iter()
// 			.map(|c| c.using_encoded(|x| x.to_vec(), false))
// 			.collect::<Vec<_>>();

// 		let decoded_compact = encoded_compact
// 			.iter()
// 			.map(|x| TestCompact::decode(&mut &**x))
// 			.collect::<Vec<_>>();

// 		let decoded = encoded
// 			.iter()
// 			.map(|x| TestCompact::decode(&mut &**x))
// 			.collect::<Vec<_>>();

// 		// then
// 		assert_eq!(decoded, cases.into_iter().map(Result::<_, codec::Error>::Ok).collect::<Vec<_>>());

// 		assert_eq!(decoded_compact, vec![Ok(d.clone()), Ok(d.clone())]);
// 	}

// 	#[test]
// 	fn opaque_leaves_should_be_full_leaf_compatible() {
// 		// given
// 		let a = Test::Data("Hello World!".into());
// 		let b = Test::Data("".into());

// 		let c: TestCompact = Compact::new((a.clone(), b.clone()));
// 		let d: TestCompact = Compact::new((
// 			Test::Hash(a.hash()),
// 			Test::Hash(b.hash()),
// 		));
// 		let cases = vec![c, d.clone()];

// 		let encoded_compact = cases
// 			.iter()
// 			.map(|c| c.using_encoded(|x| x.to_vec(), true))
// 			.map(OpaqueLeaf::from_encoded_leaf)
// 			.collect::<Vec<_>>();

// 		let opaque = cases
// 			.iter()
// 			.map(OpaqueLeaf::from_leaf)
// 			.collect::<Vec<_>>();

// 		// then
// 		assert_eq!(
// 			encoded_compact,
// 			opaque,
// 		);
// 	}

// 	#[test]
// 	fn encode_opaque_leaf_should_be_scale_compatible() {
// 		use codec::Encode;

// 		// given
// 		let a = Test::Data("Hello World!".into());
// 		let case1 = EncodableOpaqueLeaf::from_leaf(&a);
// 		let case2 = EncodableOpaqueLeaf::from_opaque_leaf(OpaqueLeaf(a.encode()));
// 		let case3 = a.encode().encode();

// 		// when
// 		let encoded = vec![&case1, &case2]
// 			.into_iter()
// 			.map(|x| x.encode())
// 			.collect::<Vec<_>>();
// 		let decoded = vec![&*encoded[0], &*encoded[1], &*case3]
// 			.into_iter()
// 			.map(|x| EncodableOpaqueLeaf::decode(&mut &*x))
// 			.collect::<Vec<_>>();

// 		// then
// 		assert_eq!(case1, case2);
// 		assert_eq!(encoded[0], encoded[1]);
// 		// then encoding should also match double-encoded leaf.
// 		assert_eq!(encoded[0], case3);

// 		assert_eq!(decoded[0], decoded[1]);
// 		assert_eq!(decoded[1], decoded[2]);
// 		assert_eq!(decoded[0], Ok(case2));
// 		assert_eq!(decoded[1], Ok(case1));
// 	}
// }
