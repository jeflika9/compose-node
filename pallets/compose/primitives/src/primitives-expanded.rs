#![feature(prelude_import)]
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

#![warn(missing_docs)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;

use frame_support::RuntimeDebug;
// use sp_runtime::traits::{self, Saturating, One};
use serde::{Serialize, Deserialize};
use codec::{Codec, Encode, Decode};
use jsonrpc_core::{Result};
use sp_std::fmt;

/// Compose operation error.
pub enum Error {

    /// Contract not found
    NotFound,
}
impl ::core::marker::StructuralEq for Error { }
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::Eq for Error {
    #[inline]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () { { } }
}
impl ::core::marker::StructuralPartialEq for Error { }
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::PartialEq for Error {
    #[inline]
    fn eq(&self, other: &Error) -> bool {
        match (&*self, &*other) { _ => true, }
    }
}
const _: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate codec as _parity_scale_codec;
        impl _parity_scale_codec::Encode for Error {
            fn encode_to<__CodecOutputEdqy: _parity_scale_codec::Output +
                         ?Sized>(&self,
                                 __codec_dest_edqy: &mut __CodecOutputEdqy) {
                match *self {
                    Error::NotFound => {
                        __codec_dest_edqy.push_byte(0usize as
                                                        ::core::primitive::u8);
                    }
                    _ => (),
                }
            }
        }
        impl _parity_scale_codec::EncodeLike for Error { }
    };
const _: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate codec as _parity_scale_codec;
        impl _parity_scale_codec::Decode for Error {
            fn decode<__CodecInputEdqy: _parity_scale_codec::Input>(__codec_input_edqy:
                                                                        &mut __CodecInputEdqy)
             -> ::core::result::Result<Self, _parity_scale_codec::Error> {
                match __codec_input_edqy.read_byte().map_err(|e|
                                                                 e.chain("Could not decode `Error`, failed to read variant byte"))?
                    {
                    __codec_x_edqy if
                    __codec_x_edqy == 0usize as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(Error::NotFound)
                    }
                    _ =>
                    ::core::result::Result::Err("Could not decode `Error`, variant doesn\'t exist".into()),
                }
            }
        }
    };
impl core::fmt::Debug for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::NotFound => fmt.debug_tuple("Error::NotFound").finish(),
            _ => Ok(()),
        }
    }
}

#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct NamedContract<AccountId> {
    path: Box<String>,
    at: AccountId,
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () =
    {
        #[allow(unused_extern_crates, clippy :: useless_attribute)]
        extern crate serde as _serde;
        #[allow(unused_macros)]
        macro_rules! try {
            ($ __expr : expr) =>
            {
                match $ __expr
                {
                    _serde :: __private :: Ok(__val) => __val, _serde ::
                    __private :: Err(__err) =>
                    { return _serde :: __private :: Err(__err) ; }
                }
            }
        }
        #[automatically_derived]
        impl <AccountId> _serde::Serialize for NamedContract<AccountId> where
         AccountId: _serde::Serialize {
            fn serialize<__S>(&self, __serializer: __S)
             -> _serde::__private::Result<__S::Ok, __S::Error> where
             __S: _serde::Serializer {
                let mut __serde_state =
                    match _serde::Serializer::serialize_struct(__serializer,
                                                               "NamedContract",
                                                               false as usize
                                                                   + 1 + 1) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "path",
                                                                    &self.path)
                    {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "at",
                                                                    &self.at)
                    {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () =
    {
        #[allow(unused_extern_crates, clippy :: useless_attribute)]
        extern crate serde as _serde;
        #[allow(unused_macros)]
        macro_rules! try {
            ($ __expr : expr) =>
            {
                match $ __expr
                {
                    _serde :: __private :: Ok(__val) => __val, _serde ::
                    __private :: Err(__err) =>
                    { return _serde :: __private :: Err(__err) ; }
                }
            }
        }
        #[automatically_derived]
        impl <'de, AccountId> _serde::Deserialize<'de> for
         NamedContract<AccountId> where AccountId: _serde::Deserialize<'de> {
            fn deserialize<__D>(__deserializer: __D)
             -> _serde::__private::Result<Self, __D::Error> where
             __D: _serde::Deserializer<'de> {
                #[allow(non_camel_case_types)]
                enum __Field { __field0, __field1, }
                struct __FieldVisitor;
                impl <'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(&self,
                                 __formatter:
                                     &mut _serde::__private::Formatter)
                     -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter,
                                                                "field identifier")
                    }
                    fn visit_u64<__E>(self, __value: u64)
                     -> _serde::__private::Result<Self::Value, __E> where
                     __E: _serde::de::Error {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            _ =>
                            _serde::__private::Err(_serde::de::Error::invalid_value(_serde::de::Unexpected::Unsigned(__value),
                                                                                    &"field index 0 <= i < 2")),
                        }
                    }
                    fn visit_str<__E>(self, __value: &str)
                     -> _serde::__private::Result<Self::Value, __E> where
                     __E: _serde::de::Error {
                        match __value {
                            "path" =>
                            _serde::__private::Ok(__Field::__field0),
                            "at" => _serde::__private::Ok(__Field::__field1),
                            _ => {
                                _serde::__private::Err(_serde::de::Error::unknown_field(__value,
                                                                                        FIELDS))
                            }
                        }
                    }
                    fn visit_bytes<__E>(self, __value: &[u8])
                     -> _serde::__private::Result<Self::Value, __E> where
                     __E: _serde::de::Error {
                        match __value {
                            b"path" =>
                            _serde::__private::Ok(__Field::__field0),
                            b"at" => _serde::__private::Ok(__Field::__field1),
                            _ => {
                                let __value =
                                    &_serde::__private::from_utf8_lossy(__value);
                                _serde::__private::Err(_serde::de::Error::unknown_field(__value,
                                                                                        FIELDS))
                            }
                        }
                    }
                }
                impl <'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(__deserializer: __D)
                     -> _serde::__private::Result<Self, __D::Error> where
                     __D: _serde::Deserializer<'de> {
                        _serde::Deserializer::deserialize_identifier(__deserializer,
                                                                     __FieldVisitor)
                    }
                }
                struct __Visitor<'de, AccountId> where
                       AccountId: _serde::Deserialize<'de> {
                    marker: _serde::__private::PhantomData<NamedContract<AccountId>>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl <'de, AccountId> _serde::de::Visitor<'de> for
                 __Visitor<'de, AccountId> where
                 AccountId: _serde::Deserialize<'de> {
                    type Value = NamedContract<AccountId>;
                    fn expecting(&self,
                                 __formatter:
                                     &mut _serde::__private::Formatter)
                     -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter,
                                                                "struct NamedContract")
                    }
                    #[inline]
                    fn visit_seq<__A>(self, mut __seq: __A)
                     -> _serde::__private::Result<Self::Value, __A::Error>
                     where __A: _serde::de::SeqAccess<'de> {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<Box<String>>(&mut __seq)
                                      {
                                      _serde::__private::Ok(__val) => __val,
                                      _serde::__private::Err(__err) => {
                                          return _serde::__private::Err(__err);
                                      }
                                  } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(_serde::de::Error::invalid_length(0usize,
                                                                                                    &"struct NamedContract with 2 elements"));
                                }
                            };
                        let __field1 =
                            match match _serde::de::SeqAccess::next_element::<AccountId>(&mut __seq)
                                      {
                                      _serde::__private::Ok(__val) => __val,
                                      _serde::__private::Err(__err) => {
                                          return _serde::__private::Err(__err);
                                      }
                                  } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(_serde::de::Error::invalid_length(1usize,
                                                                                                    &"struct NamedContract with 2 elements"));
                                }
                            };
                        _serde::__private::Ok(NamedContract{path: __field0,
                                                            at: __field1,})
                    }
                    #[inline]
                    fn visit_map<__A>(self, mut __map: __A)
                     -> _serde::__private::Result<Self::Value, __A::Error>
                     where __A: _serde::de::MapAccess<'de> {
                        let mut __field0:
                                _serde::__private::Option<Box<String>> =
                            _serde::__private::None;
                        let mut __field1:
                                _serde::__private::Option<AccountId> =
                            _serde::__private::None;
                        while let _serde::__private::Some(__key) =
                                  match _serde::de::MapAccess::next_key::<__Field>(&mut __map)
                                      {
                                      _serde::__private::Ok(__val) => __val,
                                      _serde::__private::Err(__err) => {
                                          return _serde::__private::Err(__err);
                                      }
                                  } {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0)
                                       {
                                        return _serde::__private::Err(<__A::Error
                                                                          as
                                                                          _serde::de::Error>::duplicate_field("path"));
                                    }
                                    __field0 =
                                        _serde::__private::Some(match _serde::de::MapAccess::next_value::<Box<String>>(&mut __map)
                                                                    {
                                                                    _serde::__private::Ok(__val)
                                                                    => __val,
                                                                    _serde::__private::Err(__err)
                                                                    => {
                                                                        return _serde::__private::Err(__err);
                                                                    }
                                                                });
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1)
                                       {
                                        return _serde::__private::Err(<__A::Error
                                                                          as
                                                                          _serde::de::Error>::duplicate_field("at"));
                                    }
                                    __field1 =
                                        _serde::__private::Some(match _serde::de::MapAccess::next_value::<AccountId>(&mut __map)
                                                                    {
                                                                    _serde::__private::Ok(__val)
                                                                    => __val,
                                                                    _serde::__private::Err(__err)
                                                                    => {
                                                                        return _serde::__private::Err(__err);
                                                                    }
                                                                });
                                }
                            }
                        }
                        let __field0 =
                            match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None =>
                                match _serde::__private::de::missing_field("path")
                                    {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                },
                            };
                        let __field1 =
                            match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None =>
                                match _serde::__private::de::missing_field("at")
                                    {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                },
                            };
                        _serde::__private::Ok(NamedContract{path: __field0,
                                                            at: __field1,})
                    }
                }
                const FIELDS: &'static [&'static str] = &["path", "at"];
                _serde::Deserializer::deserialize_struct(__deserializer,
                                                         "NamedContract",
                                                         FIELDS,
                                                         __Visitor{marker:
                                                                       _serde::__private::PhantomData::<NamedContract<AccountId>>,
                                                                   lifetime:
                                                                       _serde::__private::PhantomData,})
            }
        }
    };
#[automatically_derived]
#[allow(unused_qualifications)]
impl <AccountId: ::core::clone::Clone> ::core::clone::Clone for
 NamedContract<AccountId> {
    #[inline]
    fn clone(&self) -> NamedContract<AccountId> {
        match *self {
            NamedContract { path: ref __self_0_0, at: ref __self_0_1 } =>
            NamedContract{path: ::core::clone::Clone::clone(&(*__self_0_0)),
                          at: ::core::clone::Clone::clone(&(*__self_0_1)),},
        }
    }
}
impl <AccountId> ::core::marker::StructuralEq for NamedContract<AccountId> { }
#[automatically_derived]
#[allow(unused_qualifications)]
impl <AccountId: ::core::cmp::Eq> ::core::cmp::Eq for NamedContract<AccountId>
 {
    #[inline]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::core::cmp::AssertParamIsEq<Box<String>>;
            let _: ::core::cmp::AssertParamIsEq<AccountId>;
        }
    }
}
impl <AccountId> ::core::marker::StructuralPartialEq for
 NamedContract<AccountId> {
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <AccountId: ::core::cmp::PartialEq> ::core::cmp::PartialEq for
 NamedContract<AccountId> {
    #[inline]
    fn eq(&self, other: &NamedContract<AccountId>) -> bool {
        match *other {
            NamedContract { path: ref __self_1_0, at: ref __self_1_1 } =>
            match *self {
                NamedContract { path: ref __self_0_0, at: ref __self_0_1 } =>
                (*__self_0_0) == (*__self_1_0) &&
                    (*__self_0_1) == (*__self_1_1),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &NamedContract<AccountId>) -> bool {
        match *other {
            NamedContract { path: ref __self_1_0, at: ref __self_1_1 } =>
            match *self {
                NamedContract { path: ref __self_0_0, at: ref __self_0_1 } =>
                (*__self_0_0) != (*__self_1_0) ||
                    (*__self_0_1) != (*__self_1_1),
            },
        }
    }
}
const _: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate codec as _parity_scale_codec;
        impl <AccountId> _parity_scale_codec::Encode for
         NamedContract<AccountId> where
         AccountId: _parity_scale_codec::Encode,
         AccountId: _parity_scale_codec::Encode {
            fn encode_to<__CodecOutputEdqy: _parity_scale_codec::Output +
                         ?Sized>(&self,
                                 __codec_dest_edqy: &mut __CodecOutputEdqy) {
                _parity_scale_codec::Encode::encode_to(&self.path,
                                                       __codec_dest_edqy);
                _parity_scale_codec::Encode::encode_to(&self.at,
                                                       __codec_dest_edqy);
            }
        }
        impl <AccountId> _parity_scale_codec::EncodeLike for
         NamedContract<AccountId> where
         AccountId: _parity_scale_codec::Encode,
         AccountId: _parity_scale_codec::Encode {
        }
    };
const _: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate codec as _parity_scale_codec;
        impl <AccountId> _parity_scale_codec::Decode for
         NamedContract<AccountId> where
         AccountId: _parity_scale_codec::Decode,
         AccountId: _parity_scale_codec::Decode {
            fn decode<__CodecInputEdqy: _parity_scale_codec::Input>(__codec_input_edqy:
                                                                        &mut __CodecInputEdqy)
             -> ::core::result::Result<Self, _parity_scale_codec::Error> {
                ::core::result::Result::Ok(NamedContract::<AccountId>{path:
                                                                          {
                                                                              let __codec_res_edqy =
                                                                                  <Box<String>
                                                                                      as
                                                                                      _parity_scale_codec::Decode>::decode(__codec_input_edqy);
                                                                              match __codec_res_edqy
                                                                                  {
                                                                                  ::core::result::Result::Err(e)
                                                                                  =>
                                                                                  return ::core::result::Result::Err(e.chain("Could not decode `NamedContract::path`")),
                                                                                  ::core::result::Result::Ok(__codec_res_edqy)
                                                                                  =>
                                                                                  __codec_res_edqy,
                                                                              }
                                                                          },
                                                                      at:
                                                                          {
                                                                              let __codec_res_edqy =
                                                                                  <AccountId
                                                                                      as
                                                                                      _parity_scale_codec::Decode>::decode(__codec_input_edqy);
                                                                              match __codec_res_edqy
                                                                                  {
                                                                                  ::core::result::Result::Err(e)
                                                                                  =>
                                                                                  return ::core::result::Result::Err(e.chain("Could not decode `NamedContract::at`")),
                                                                                  ::core::result::Result::Ok(__codec_res_edqy)
                                                                                  =>
                                                                                  __codec_res_edqy,
                                                                              }
                                                                          },})
            }
        }
    };

// ### RPC REQUIRED ###
impl Error {
    #![allow(unused_variables)]
    /// Consume given error `e` with `self` and generate a native log entry with error details.
    pub fn log_error(self, e: impl fmt::Debug) -> Self {



        // ### RPC REQUIRED ###
        /* automatically added by decl_runtime_apis: Block, */

        // Block: BlockT automatically added by decl_runtime_apis; see "fn extend_generics_with_block"
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
        {
            let lvl = ::log::Level::Error;
            if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                ::log::__private_api_log(::core::fmt::Arguments::new_v1(&["[",
                                                                          "] Compose error: "],
                                                                        &match (&self,
                                                                                &e)
                                                                             {
                                                                             (arg0,
                                                                              arg1)
                                                                             =>
                                                                             [::core::fmt::ArgumentV1::new(arg0,
                                                                                                           ::core::fmt::Debug::fmt),
                                                                              ::core::fmt::ArgumentV1::new(arg1,
                                                                                                           ::core::fmt::Debug::fmt)],
                                                                         }),
                                         lvl,
                                         &("runtime::compose",
                                           "compose_primitives",
                                           "pallets/compose/primitives/src/lib.rs",
                                           52u32));
            }
        };
        self
    }
    /// Consume given error `e` with `self` and generate a native log entry with error details.
    pub fn log_debug(self, e: impl fmt::Debug) -> Self {
        {
            let lvl = ::log::Level::Debug;
            if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                ::log::__private_api_log(::core::fmt::Arguments::new_v1(&["[",
                                                                          "] Compose error: "],
                                                                        &match (&self,
                                                                                &e)
                                                                             {
                                                                             (arg0,
                                                                              arg1)
                                                                             =>
                                                                             [::core::fmt::ArgumentV1::new(arg0,
                                                                                                           ::core::fmt::Debug::fmt),
                                                                              ::core::fmt::ArgumentV1::new(arg1,
                                                                                                           ::core::fmt::Debug::fmt)],
                                                                         }),
                                         lvl,
                                         &("runtime::compose",
                                           "compose_primitives",
                                           "pallets/compose/primitives/src/lib.rs",
                                           63u32));
            }
        };
        self
    }
}
pub type NamedContractResult<AccountId> = Result<NamedContract<AccountId>>;
#[doc(hidden)]
mod sp_api_hidden_includes_DECL_RUNTIME_APIS {
    pub extern crate sp_api as sp_api;
}
#[doc(hidden)]
#[allow(dead_code)]
#[allow(deprecated)]
pub mod runtime_decl_for_ComposeApi {
    use super::*;
    #[doc = " The API to interact with contracts without using executive."]
    pub trait ComposeApi<Block: self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::BlockT,
                         AccountId, Balance, BlockNumber, Hash> where
     AccountId: Codec, Balance: Codec, BlockNumber: Codec, Hash: Codec {
        #[doc =
          " Query the key details on a given contract, so that it may be called."]
        #[doc = ""]
        #[doc =
          " Returns `Some(NamedContract)` if the contract exists at the given path and `None` if it doesn\'t."]
        #[doc = " See [`compose::get_named_contract`]."]
        fn get_named_contract(contract_path: Box<String>)
        -> Option<NamedContract<AccountId>>;
    }
    pub const VERSION: u32 = 1u32;
    pub const ID: [u8; 8] =
        [197u8, 114u8, 54u8, 201u8, 158u8, 216u8, 171u8, 102u8];
    #[cfg(any(feature = "std", test))]
    fn convert_between_block_types<I: self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::Encode,
                                   R: self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::Decode,
                                   F: FnOnce(self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::codec::Error)
                                   ->
                                       self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ApiError>(input:
                                                                                                             &I,
                                                                                                         map_error:
                                                                                                             F)
     ->
         std::result::Result<R,
                             self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ApiError> {
        <R as
            self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::DecodeLimit>::decode_with_depth_limit(self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::MAX_EXTRINSIC_DEPTH,
                                                                                                          &mut &self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::Encode::encode(input)[..]).map_err(map_error)
    }
    #[cfg(any(feature = "std", test))]
    pub fn get_named_contract_native_call_generator<'a,
                                                    ApiImpl: ComposeApi<Block,
                                                                        AccountId,
                                                                        Balance,
                                                                        BlockNumber,
                                                                        Hash>,
                                                    NodeBlock: self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::BlockT,
                                                    Block: self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::BlockT +
                                                    'a, AccountId: 'a,
                                                    Balance: 'a,
                                                    BlockNumber: 'a,
                                                    Hash: 'a>(contract_path:
                                                                  Box<String>)
     ->
         impl FnOnce()
         ->
             std::result::Result<Option<NamedContract<AccountId>>,
                                 self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ApiError> +
         'a where AccountId: Codec, Balance: Codec, BlockNumber: Codec,
     Hash: Codec {
        move ||
            { let res = ApiImpl::get_named_contract(contract_path); Ok(res) }
    }
    #[cfg(any(feature = "std", test))]
    pub fn get_named_contract_call_api_at<R: self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::Encode +
                                          self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::Decode +
                                          PartialEq, NC: FnOnce()
                                          ->
                                              std::result::Result<R,
                                                                  self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ApiError> +
                                          std::panic::UnwindSafe,
                                          Block: self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::BlockT,
                                          T: self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::CallApiAt<Block>>(call_runtime_at:
                                                                                                                           &T,
                                                                                                                       at:
                                                                                                                           &self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::BlockId<Block>,
                                                                                                                       args:
                                                                                                                           Vec<u8>,
                                                                                                                       changes:
                                                                                                                           &std::cell::RefCell<self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::OverlayedChanges>,
                                                                                                                       storage_transaction_cache:
                                                                                                                           &std::cell::RefCell<self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::StorageTransactionCache<Block,
                                                                                                                                                                                                                               T::StateBackend>>,
                                                                                                                       native_call:
                                                                                                                           Option<NC>,
                                                                                                                       context:
                                                                                                                           self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ExecutionContext,
                                                                                                                       recorder:
                                                                                                                           &Option<self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ProofRecorder<Block>>)
     ->
         std::result::Result<self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::NativeOrEncoded<R>,
                             self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ApiError> {
        let version = call_runtime_at.runtime_version_at(at)?;
        let params =
            self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::CallApiAtParams{at,
                                                                                    function:
                                                                                        "ComposeApi_get_named_contract",
                                                                                    native_call,
                                                                                    arguments:
                                                                                        args,
                                                                                    overlayed_changes:
                                                                                        changes,
                                                                                    storage_transaction_cache,
                                                                                    context,
                                                                                    recorder,};
        call_runtime_at.call_api_at(params)
    }
}
#[doc = " The API to interact with contracts without using executive."]
#[cfg(any(feature = "std", test))]
pub trait ComposeApi<Block: self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::BlockT,
                     AccountId, Balance, BlockNumber,
                     Hash>: self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::Core<Block>
 where AccountId: Codec, Balance: Codec, BlockNumber: Codec, Hash: Codec {
    #[doc =
      " Query the key details on a given contract, so that it may be called."]
    #[doc = ""]
    #[doc =
      " Returns `Some(NamedContract)` if the contract exists at the given path and `None` if it doesn\'t."]
    #[doc = " See [`compose::get_named_contract`]."]
    fn get_named_contract(&self,
                          __runtime_api_at_param__:
                              &self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::BlockId<Block>,
                          contract_path: Box<String>)
     ->
         std::result::Result<Option<NamedContract<AccountId>>,
                             self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ApiError> {
        let runtime_api_impl_params_encoded =
            self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::Encode::encode(&(&contract_path));
        self.ComposeApi_get_named_contract_runtime_api_impl(__runtime_api_at_param__,
                                                            self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ExecutionContext::OffchainCall(None),
                                                            Some((contract_path)),
                                                            runtime_api_impl_params_encoded).and_then(|r|
                                                                                                          match r
                                                                                                              {
                                                                                                              self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::NativeOrEncoded::Native(n)
                                                                                                              =>
                                                                                                              {
                                                                                                                  Ok(n)
                                                                                                              }
                                                                                                              self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::NativeOrEncoded::Encoded(r)
                                                                                                              =>
                                                                                                              {
                                                                                                                  <Option<NamedContract<AccountId>>
                                                                                                                      as
                                                                                                                      self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::Decode>::decode(&mut &r[..]).map_err(|err|
                                                                                                                                                                                                                       self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ApiError::FailedToDecodeReturnValue{function:
                                                                                                                                                                                                                                                                                                                       "get_named_contract",
                                                                                                                                                                                                                                                                                                                   error:
                                                                                                                                                                                                                                                                                                                       err,})
                                                                                                              }
                                                                                                          })
    }
    #[doc =
      " Query the key details on a given contract, so that it may be called."]
    #[doc = ""]
    #[doc =
      " Returns `Some(NamedContract)` if the contract exists at the given path and `None` if it doesn\'t."]
    #[doc = " See [`compose::get_named_contract`]."]
    fn get_named_contract_with_context(&self,
                                       __runtime_api_at_param__:
                                           &self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::BlockId<Block>,
                                       context:
                                           self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ExecutionContext,
                                       contract_path: Box<String>)
     ->
         std::result::Result<Option<NamedContract<AccountId>>,
                             self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ApiError> {
        let runtime_api_impl_params_encoded =
            self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::Encode::encode(&(&contract_path));
        self.ComposeApi_get_named_contract_runtime_api_impl(__runtime_api_at_param__,
                                                            context,
                                                            Some((contract_path)),
                                                            runtime_api_impl_params_encoded).and_then(|r|
                                                                                                          match r
                                                                                                              {
                                                                                                              self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::NativeOrEncoded::Native(n)
                                                                                                              =>
                                                                                                              {
                                                                                                                  Ok(n)
                                                                                                              }
                                                                                                              self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::NativeOrEncoded::Encoded(r)
                                                                                                              =>
                                                                                                              {
                                                                                                                  <Option<NamedContract<AccountId>>
                                                                                                                      as
                                                                                                                      self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::Decode>::decode(&mut &r[..]).map_err(|err|
                                                                                                                                                                                                                       self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ApiError::FailedToDecodeReturnValue{function:
                                                                                                                                                                                                                                                                                                                       "get_named_contract",
                                                                                                                                                                                                                                                                                                                   error:
                                                                                                                                                                                                                                                                                                                       err,})
                                                                                                              }
                                                                                                          })
    }
    #[doc(hidden)]
    fn ComposeApi_get_named_contract_runtime_api_impl(&self,
                                                      at:
                                                          &self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::BlockId<Block>,
                                                      context:
                                                          self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ExecutionContext,
                                                      params:
                                                          Option<(Box<String>)>,
                                                      params_encoded: Vec<u8>)
    ->
        std::result::Result<self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::NativeOrEncoded<Option<NamedContract<AccountId>>>,
                            self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ApiError>;
}
#[cfg(any(feature = "std", test))]
impl <Block: self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::BlockT,
      AccountId, Balance, BlockNumber, Hash>
 self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::RuntimeApiInfo for
 ComposeApi<Block, AccountId, Balance, BlockNumber, Hash> {
    const ID: [u8; 8] =
        [197u8, 114u8, 54u8, 201u8, 158u8, 216u8, 171u8, 102u8];
    const VERSION: u32 = 1u32;
}
