#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
mod common {
    use codec::{Encode, Decode};
    use serde::{Serialize, Deserialize};
    use std::convert::{TryFrom, TryInto};
    use jsonrpc_core::{Error, ErrorCode, Result};
    use frame_support::{RuntimeDebug};
    pub use pallet_contracts_primitives::{Code, ContractExecResult, ContractInstantiateResult};
    pub use pallet_contracts_rpc::{CallRequest, InstantiateRequest, Contracts};
    const RUNTIME_ERROR: i64 = 1;
    const CONTRACT_DOESNT_EXIST: i64 = 2;
    pub type Weight = u64;
    /// A rough estimate of how much gas a decent hardware consumes per second,
    /// using native execution.
    /// This value is used to set the upper bound for maximal contract calls to
    /// prevent blocking the RPC for too long.
    ///
    /// As 1 gas is equal to 1 weight we base this on the conducted benchmarks which
    /// determined runtime weights:
    /// <https://github.com/paritytech/substrate/pull/5446>
    const GAS_PER_SECOND: Weight = 1_000_000_000_000;
    /// The maximum amount of weight that the call and instantiate rpcs are allowed to consume.
    /// This puts a ceiling on the weight limit that is supplied to the rpc as an argument.
    const GAS_LIMIT: Weight = 5 * GAS_PER_SECOND;
    #[serde(rename_all = "camelCase")]
    #[serde(deny_unknown_fields)]
    pub struct NamedContract<AccountId> {
        path: Box<String>,
        at: AccountId,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<AccountId> _serde::Serialize for NamedContract<AccountId>
        where
            AccountId: _serde::Serialize,
        {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "NamedContract",
                    false as usize + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "path",
                    &self.path,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "at",
                    &self.at,
                ) {
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
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de, AccountId> _serde::Deserialize<'de> for NamedContract<AccountId>
        where
            AccountId: _serde::Deserialize<'de>,
        {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 2",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "path" => _serde::__private::Ok(__Field::__field0),
                            "at" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Err(_serde::de::Error::unknown_field(
                                __value, FIELDS,
                            )),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"path" => _serde::__private::Ok(__Field::__field0),
                            b"at" => _serde::__private::Ok(__Field::__field1),
                            _ => {
                                let __value = &_serde::__private::from_utf8_lossy(__value);
                                _serde::__private::Err(_serde::de::Error::unknown_field(
                                    __value, FIELDS,
                                ))
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de, AccountId>
                where
                    AccountId: _serde::Deserialize<'de>,
                {
                    marker: _serde::__private::PhantomData<NamedContract<AccountId>>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de, AccountId> _serde::de::Visitor<'de> for __Visitor<'de, AccountId>
                where
                    AccountId: _serde::Deserialize<'de>,
                {
                    type Value = NamedContract<AccountId>;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "struct NamedContract")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<Box<String>>(
                            &mut __seq,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct NamedContract with 2 elements",
                                ));
                            }
                        };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<AccountId>(
                            &mut __seq,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct NamedContract with 2 elements",
                                ));
                            }
                        };
                        _serde::__private::Ok(NamedContract {
                            path: __field0,
                            at: __field1,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<Box<String>> =
                            _serde::__private::None;
                        let mut __field1: _serde::__private::Option<AccountId> =
                            _serde::__private::None;
                        while let _serde::__private::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "path",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Box<String>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "at",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<AccountId>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("path") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("at") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(NamedContract {
                            path: __field0,
                            at: __field1,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &["path", "at"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "NamedContract",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<NamedContract<AccountId>>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<AccountId: ::core::clone::Clone> ::core::clone::Clone for NamedContract<AccountId> {
        #[inline]
        fn clone(&self) -> NamedContract<AccountId> {
            match *self {
                NamedContract {
                    path: ref __self_0_0,
                    at: ref __self_0_1,
                } => NamedContract {
                    path: ::core::clone::Clone::clone(&(*__self_0_0)),
                    at: ::core::clone::Clone::clone(&(*__self_0_1)),
                },
            }
        }
    }
    impl<AccountId> ::core::marker::StructuralEq for NamedContract<AccountId> {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<AccountId: ::core::cmp::Eq> ::core::cmp::Eq for NamedContract<AccountId> {
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
    impl<AccountId> ::core::marker::StructuralPartialEq for NamedContract<AccountId> {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<AccountId: ::core::cmp::PartialEq> ::core::cmp::PartialEq for NamedContract<AccountId> {
        #[inline]
        fn eq(&self, other: &NamedContract<AccountId>) -> bool {
            match *other {
                NamedContract {
                    path: ref __self_1_0,
                    at: ref __self_1_1,
                } => match *self {
                    NamedContract {
                        path: ref __self_0_0,
                        at: ref __self_0_1,
                    } => (*__self_0_0) == (*__self_1_0) && (*__self_0_1) == (*__self_1_1),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &NamedContract<AccountId>) -> bool {
            match *other {
                NamedContract {
                    path: ref __self_1_0,
                    at: ref __self_1_1,
                } => match *self {
                    NamedContract {
                        path: ref __self_0_0,
                        at: ref __self_0_1,
                    } => (*__self_0_0) != (*__self_1_0) || (*__self_0_1) != (*__self_1_1),
                },
            }
        }
    }
    const _: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate codec as _parity_scale_codec;
        impl<AccountId> _parity_scale_codec::Encode for NamedContract<AccountId>
        where
            AccountId: _parity_scale_codec::Encode,
            AccountId: _parity_scale_codec::Encode,
        {
            fn encode_to<__CodecOutputEdqy: _parity_scale_codec::Output + ?Sized>(
                &self,
                __codec_dest_edqy: &mut __CodecOutputEdqy,
            ) {
                _parity_scale_codec::Encode::encode_to(&self.path, __codec_dest_edqy);
                _parity_scale_codec::Encode::encode_to(&self.at, __codec_dest_edqy);
            }
        }
        impl<AccountId> _parity_scale_codec::EncodeLike for NamedContract<AccountId>
        where
            AccountId: _parity_scale_codec::Encode,
            AccountId: _parity_scale_codec::Encode,
        {
        }
    };
    const _: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate codec as _parity_scale_codec;
        impl<AccountId> _parity_scale_codec::Decode for NamedContract<AccountId>
        where
            AccountId: _parity_scale_codec::Decode,
            AccountId: _parity_scale_codec::Decode,
        {
            fn decode<__CodecInputEdqy: _parity_scale_codec::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> ::core::result::Result<Self, _parity_scale_codec::Error> {
                ::core::result::Result::Ok(NamedContract::<AccountId> {
                    path: {
                        let __codec_res_edqy = <Box<String> as _parity_scale_codec::Decode>::decode(
                            __codec_input_edqy,
                        );
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `NamedContract::path`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    },
                    at: {
                        let __codec_res_edqy =
                            <AccountId as _parity_scale_codec::Decode>::decode(__codec_input_edqy);
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `NamedContract::at`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    },
                })
            }
        }
    };
    pub enum NamedContractError {
        NotFound,
    }
    impl ::core::marker::StructuralEq for NamedContractError {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Eq for NamedContractError {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            {}
        }
    }
    impl ::core::marker::StructuralPartialEq for NamedContractError {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for NamedContractError {
        #[inline]
        fn eq(&self, other: &NamedContractError) -> bool {
            match (&*self, &*other) {
                _ => true,
            }
        }
    }
    const _: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate codec as _parity_scale_codec;
        impl _parity_scale_codec::Encode for NamedContractError {
            fn encode_to<__CodecOutputEdqy: _parity_scale_codec::Output + ?Sized>(
                &self,
                __codec_dest_edqy: &mut __CodecOutputEdqy,
            ) {
                match *self {
                    NamedContractError::NotFound => {
                        __codec_dest_edqy.push_byte(0usize as ::core::primitive::u8);
                    }
                    _ => (),
                }
            }
        }
        impl _parity_scale_codec::EncodeLike for NamedContractError {}
    };
    const _: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate codec as _parity_scale_codec;
        impl _parity_scale_codec::Decode for NamedContractError {
            fn decode<__CodecInputEdqy: _parity_scale_codec::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> ::core::result::Result<Self, _parity_scale_codec::Error> {
                match __codec_input_edqy.read_byte().map_err(|e| {
                    e.chain("Could not decode `NamedContractError`, failed to read variant byte")
                })? {
                    __codec_x_edqy if __codec_x_edqy == 0usize as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(NamedContractError::NotFound)
                    }
                    _ => ::core::result::Result::Err(
                        "Could not decode `NamedContractError`, variant doesn\'t exist".into(),
                    ),
                }
            }
        }
    };
    impl core::fmt::Debug for NamedContractError {
        fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self {
                Self::NotFound => fmt.debug_tuple("NamedContractError::NotFound").finish(),
                _ => Ok(()),
            }
        }
    }
    impl From<NamedContractError> for Error {
        fn from(e: NamedContractError) -> Error {
            match e {
                NotFound => Error {
                    code: ErrorCode::ServerError(CONTRACT_DOESNT_EXIST),
                    message: "The specified contract doesn't exist.".into(),
                    data: None,
                },
            }
        }
    }
    pub type NamedContractResult<AccountId> = Result<NamedContract<AccountId>>;
    /// Converts a runtime trap into an RPC error.
    pub fn runtime_error_into_rpc_err(err: impl std::fmt::Debug) -> Error {
        Error {
            code: ErrorCode::ServerError(RUNTIME_ERROR),
            message: "Runtime error".into(),
            data: Some(
                {
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &[""],
                        &match (&err,) {
                            (arg0,) => {
                                [::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt)]
                            }
                        },
                    ));
                    res
                }
                .into(),
            ),
        }
    }
    pub fn decode_hex<H: std::fmt::Debug + Copy, T: TryFrom<H>>(from: H, name: &str) -> Result<T> {
        from.try_into().map_err(|_| Error {
            code: ErrorCode::InvalidParams,
            message: {
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &["", " does not fit into the ", " type"],
                    &match (&from, &name) {
                        (arg0, arg1) => [
                            ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt),
                            ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                        ],
                    },
                ));
                res
            },
            data: None,
        })
    }
    pub fn limit_gas(gas_limit: Weight) -> Result<()> {
        if gas_limit > GAS_LIMIT {
            Err(Error {
                code: ErrorCode::InvalidParams,
                message: {
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &[
                            "Requested gas limit is greater than maximum allowed: ",
                            " > ",
                        ],
                        &match (&gas_limit, &GAS_LIMIT) {
                            (arg0, arg1) => [
                                ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                            ],
                        },
                    ));
                    res
                },
                data: None,
            })
        } else {
            Ok(())
        }
    }
}
mod runtime_api {
    use codec::Codec;
    use sp_std::vec::Vec;
    use crate::common::*;
    #[doc(hidden)]
    mod sp_api_hidden_includes_DECL_RUNTIME_APIS {
        pub extern crate sp_api as sp_api;
    }
    #[doc(hidden)]
    #[allow(dead_code)]
    #[allow(deprecated)]
    pub mod runtime_decl_for_ComposeRuntimeApi {
        use super::*;
        /// The API to interact with contracts without using executive.
        pub trait ComposeRuntimeApi<
            Block: self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::BlockT,
            AccountId,
            Balance,
            BlockNumber,
            Hash,
        >
        where
            AccountId: Codec,
            Balance: Codec,
            BlockNumber: Codec,
            Hash: Codec,
        {
            /// Query the key details on a given contract, so that it may be called.
            ///
            /// Returns `Some(NamedContract)` if the contract exists at the given path and `None` if it doesn't.
            /// See [`compose::get_named_contract`].
            fn get_named_contract(contract_path: Box<String>) -> Option<NamedContract<AccountId>>;
        }
        pub const VERSION: u32 = 1u32;
        pub const ID: [u8; 8] = [118u8, 69u8, 208u8, 84u8, 232u8, 254u8, 56u8, 104u8];
        #[cfg(any(feature = "std", test))]
        fn convert_between_block_types<
            I: self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::Encode,
            R: self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::Decode,
            F: FnOnce(
                self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::codec::Error,
            ) -> self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ApiError,
        >(
            input: &I,
            map_error: F,
        ) -> std::result::Result<R, self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ApiError>
        {
            < R as self :: sp_api_hidden_includes_DECL_RUNTIME_APIS :: sp_api :: DecodeLimit > :: decode_with_depth_limit (self :: sp_api_hidden_includes_DECL_RUNTIME_APIS :: sp_api :: MAX_EXTRINSIC_DEPTH , & mut & self :: sp_api_hidden_includes_DECL_RUNTIME_APIS :: sp_api :: Encode :: encode (input) [..]) . map_err (map_error)
        }
        #[cfg(any(feature = "std", test))]
        pub fn get_named_contract_native_call_generator<
            'a,
            ApiImpl: ComposeRuntimeApi<Block, AccountId, Balance, BlockNumber, Hash>,
            NodeBlock: self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::BlockT,
            Block: self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::BlockT + 'a,
            AccountId: 'a,
            Balance: 'a,
            BlockNumber: 'a,
            Hash: 'a,
        >(
            contract_path: Box<String>,
        ) -> impl FnOnce() -> std::result::Result<
            Option<NamedContract<AccountId>>,
            self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ApiError,
        > + 'a
        where
            AccountId: Codec,
            Balance: Codec,
            BlockNumber: Codec,
            Hash: Codec,
        {
            move || {
                let res = ApiImpl::get_named_contract(contract_path);
                Ok(res)
            }
        }
        #[cfg(any(feature = "std", test))]
        pub fn get_named_contract_call_api_at<
            R: self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::Encode
                + self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::Decode
                + PartialEq,
            NC: FnOnce() -> std::result::Result<
                    R,
                    self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ApiError,
                > + std::panic::UnwindSafe,
            Block: self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::BlockT,
            T: self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::CallApiAt<Block>,
        >(
            call_runtime_at: &T,
            at: &self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::BlockId<Block>,
            args: Vec<u8>,
            changes: &std::cell::RefCell<
                self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::OverlayedChanges,
            >,
            storage_transaction_cache: &std::cell::RefCell<
                self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::StorageTransactionCache<
                    Block,
                    T::StateBackend,
                >,
            >,
            native_call: Option<NC>,
            context: self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ExecutionContext,
            recorder: &Option<
                self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ProofRecorder<Block>,
            >,
        ) -> std::result::Result<
            self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::NativeOrEncoded<R>,
            self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ApiError,
        > {
            let version = call_runtime_at.runtime_version_at(at)?;
            let params = self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::CallApiAtParams {
                at,
                function: "ComposeRuntimeApi_get_named_contract",
                native_call,
                arguments: args,
                overlayed_changes: changes,
                storage_transaction_cache,
                context,
                recorder,
            };
            call_runtime_at.call_api_at(params)
        }
    }
    /// The API to interact with contracts without using executive.
    #[cfg(any(feature = "std", test))]
    pub trait ComposeRuntimeApi<
        Block: self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::BlockT,
        AccountId,
        Balance,
        BlockNumber,
        Hash,
    >: self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::Core<Block>
    where
        AccountId: Codec,
        Balance: Codec,
        BlockNumber: Codec,
        Hash: Codec,
    {
        /// Query the key details on a given contract, so that it may be called.
        ///
        /// Returns `Some(NamedContract)` if the contract exists at the given path and `None` if it doesn't.
        /// See [`compose::get_named_contract`].
        fn get_named_contract(
            &self,
            __runtime_api_at_param__ : & self :: sp_api_hidden_includes_DECL_RUNTIME_APIS :: sp_api :: BlockId < Block >,
            contract_path: Box<String>,
        ) -> std::result::Result<
            Option<NamedContract<AccountId>>,
            self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ApiError,
        > {
            let runtime_api_impl_params_encoded =
                self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::Encode::encode(
                    &(&contract_path),
                );
            self . ComposeRuntimeApi_get_named_contract_runtime_api_impl (__runtime_api_at_param__ , self :: sp_api_hidden_includes_DECL_RUNTIME_APIS :: sp_api :: ExecutionContext :: OffchainCall (None) , Some ((contract_path)) , runtime_api_impl_params_encoded) . and_then (| r | match r { self :: sp_api_hidden_includes_DECL_RUNTIME_APIS :: sp_api :: NativeOrEncoded :: Native (n) => { Ok (n) } self :: sp_api_hidden_includes_DECL_RUNTIME_APIS :: sp_api :: NativeOrEncoded :: Encoded (r) => { < Option < NamedContract < AccountId > > as self :: sp_api_hidden_includes_DECL_RUNTIME_APIS :: sp_api :: Decode > :: decode (& mut & r [..]) . map_err (| err | self :: sp_api_hidden_includes_DECL_RUNTIME_APIS :: sp_api :: ApiError :: FailedToDecodeReturnValue { function : "get_named_contract" , error : err , }) } })
        }
        /// Query the key details on a given contract, so that it may be called.
        ///
        /// Returns `Some(NamedContract)` if the contract exists at the given path and `None` if it doesn't.
        /// See [`compose::get_named_contract`].
        fn get_named_contract_with_context(
            &self,
            __runtime_api_at_param__ : & self :: sp_api_hidden_includes_DECL_RUNTIME_APIS :: sp_api :: BlockId < Block >,
            context: self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ExecutionContext,
            contract_path: Box<String>,
        ) -> std::result::Result<
            Option<NamedContract<AccountId>>,
            self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ApiError,
        > {
            let runtime_api_impl_params_encoded =
                self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::Encode::encode(
                    &(&contract_path),
                );
            self . ComposeRuntimeApi_get_named_contract_runtime_api_impl (__runtime_api_at_param__ , context , Some ((contract_path)) , runtime_api_impl_params_encoded) . and_then (| r | match r { self :: sp_api_hidden_includes_DECL_RUNTIME_APIS :: sp_api :: NativeOrEncoded :: Native (n) => { Ok (n) } self :: sp_api_hidden_includes_DECL_RUNTIME_APIS :: sp_api :: NativeOrEncoded :: Encoded (r) => { < Option < NamedContract < AccountId > > as self :: sp_api_hidden_includes_DECL_RUNTIME_APIS :: sp_api :: Decode > :: decode (& mut & r [..]) . map_err (| err | self :: sp_api_hidden_includes_DECL_RUNTIME_APIS :: sp_api :: ApiError :: FailedToDecodeReturnValue { function : "get_named_contract" , error : err , }) } })
        }
        #[doc(hidden)]
        fn ComposeRuntimeApi_get_named_contract_runtime_api_impl(
            &self,
            at: &self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::BlockId<Block>,
            context: self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ExecutionContext,
            params: Option<(Box<String>)>,
            params_encoded: Vec<u8>,
        ) -> std::result::Result<
            self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::NativeOrEncoded<
                Option<NamedContract<AccountId>>,
            >,
            self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ApiError,
        >;
    }
    #[cfg(any(feature = "std", test))]
    impl<
            Block: self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::BlockT,
            AccountId,
            Balance,
            BlockNumber,
            Hash,
        > self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::RuntimeApiInfo
        for ComposeRuntimeApi<Block, AccountId, Balance, BlockNumber, Hash>
    {
        const ID: [u8; 8] = [118u8, 69u8, 208u8, 84u8, 232u8, 254u8, 56u8, 104u8];
        const VERSION: u32 = 1u32;
    }
}
use std::sync::Arc;
use std::convert::{TryFrom};
use common::*;
use runtime_api::*;
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
mod rpc_impl_ComposeRPCApi {
    use jsonrpc_core as _jsonrpc_core;
    use super::*;
    /// The generated client module.
    pub mod gen_client {
        use jsonrpc_core_client as _jsonrpc_core_client;
        use super::*;
        use _jsonrpc_core::{
            Call, Error, ErrorCode, Id, MethodCall, Params, Request, Response, Version,
        };
        use _jsonrpc_core::futures::prelude::Future;
        use _jsonrpc_core::futures::sync::{mpsc, oneshot};
        use _jsonrpc_core::serde_json::{self, Value};
        use _jsonrpc_core_client::{
            RpcChannel, RpcError, RpcFuture, TypedClient, TypedSubscriptionStream,
        };
        /// The Client.
        pub struct Client<BlockHash, BlockNumber, AccountId, Balance, Hash> {
            inner: TypedClient,
            _0: std::marker::PhantomData<BlockHash>,
            _1: std::marker::PhantomData<BlockNumber>,
            _2: std::marker::PhantomData<AccountId>,
            _3: std::marker::PhantomData<Balance>,
            _4: std::marker::PhantomData<Hash>,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl<
                BlockHash: ::core::clone::Clone,
                BlockNumber: ::core::clone::Clone,
                AccountId: ::core::clone::Clone,
                Balance: ::core::clone::Clone,
                Hash: ::core::clone::Clone,
            > ::core::clone::Clone for Client<BlockHash, BlockNumber, AccountId, Balance, Hash>
        {
            #[inline]
            fn clone(&self) -> Client<BlockHash, BlockNumber, AccountId, Balance, Hash> {
                match *self {
                    Client {
                        inner: ref __self_0_0,
                        _0: ref __self_0_1,
                        _1: ref __self_0_2,
                        _2: ref __self_0_3,
                        _3: ref __self_0_4,
                        _4: ref __self_0_5,
                    } => Client {
                        inner: ::core::clone::Clone::clone(&(*__self_0_0)),
                        _0: ::core::clone::Clone::clone(&(*__self_0_1)),
                        _1: ::core::clone::Clone::clone(&(*__self_0_2)),
                        _2: ::core::clone::Clone::clone(&(*__self_0_3)),
                        _3: ::core::clone::Clone::clone(&(*__self_0_4)),
                        _4: ::core::clone::Clone::clone(&(*__self_0_5)),
                    },
                }
            }
        }
        impl<BlockHash, BlockNumber, AccountId, Balance, Hash>
            Client<BlockHash, BlockNumber, AccountId, Balance, Hash>
        where
            BlockHash: Send + Sync + 'static + _jsonrpc_core::serde::Serialize,
            BlockNumber: Send + Sync + 'static,
            AccountId: Send + Sync + 'static + _jsonrpc_core::serde::de::DeserializeOwned,
            Balance: Send + Sync + 'static,
            Hash: Send + Sync + 'static,
        {
            /// Creates a new `Client`.
            pub fn new(sender: RpcChannel) -> Self {
                Client {
                    inner: sender.into(),
                    _0: std::marker::PhantomData,
                    _1: std::marker::PhantomData,
                    _2: std::marker::PhantomData,
                    _3: std::marker::PhantomData,
                    _4: std::marker::PhantomData,
                }
            }
            /// Returns the value under a specified storage `key` in a contract given by `address` param,
            /// or `None` if it is not set.
            pub fn get_named_contract(
                &self,
                contract_path: Box<String>,
                at: Option<BlockHash>,
            ) -> impl Future<Item = NamedContract<AccountId>, Error = RpcError> {
                let args = (contract_path, at);
                self.inner.call_method(
                    "compose_getNamedContract",
                    "NamedContract < AccountId >",
                    args,
                )
            }
        }
        impl<BlockHash, BlockNumber, AccountId, Balance, Hash> From<RpcChannel>
            for Client<BlockHash, BlockNumber, AccountId, Balance, Hash>
        where
            BlockHash: Send + Sync + 'static + _jsonrpc_core::serde::Serialize,
            BlockNumber: Send + Sync + 'static,
            AccountId: Send + Sync + 'static + _jsonrpc_core::serde::de::DeserializeOwned,
            Balance: Send + Sync + 'static,
            Hash: Send + Sync + 'static,
        {
            fn from(channel: RpcChannel) -> Self {
                Client::new(channel.into())
            }
        }
    }
    /// The generated server module.
    pub mod gen_server {
        use self::_jsonrpc_core::futures as _futures;
        use super::*;
        /// Contracts RPC methods.
        pub trait ComposeRPCApi<BlockHash, BlockNumber, AccountId, Balance, Hash>:
            Sized + Send + Sync + 'static
        {
            /// Returns the value under a specified storage `key` in a contract given by `address` param,
            /// or `None` if it is not set.
            fn get_named_contract(
                &self,
                contract_path: Box<String>,
                at: Option<BlockHash>,
            ) -> Result<NamedContract<AccountId>>;
            /// Create an `IoDelegate`, wiring rpc calls to the trait methods.
            fn to_delegate<M: _jsonrpc_core::Metadata>(self) -> _jsonrpc_core::IoDelegate<Self, M>
            where
                BlockHash: Send + Sync + 'static + _jsonrpc_core::serde::de::DeserializeOwned,
                BlockNumber: Send + Sync + 'static,
                AccountId: Send + Sync + 'static + _jsonrpc_core::serde::Serialize,
                Balance: Send + Sync + 'static,
                Hash: Send + Sync + 'static,
            {
                let mut del = _jsonrpc_core::IoDelegate::new(self.into());
                del.add_method("compose_getNamedContract", move |base, params| {
                    let method = &(Self::get_named_contract
                        as fn(
                            &Self,
                            Box<String>,
                            Option<BlockHash>,
                        ) -> Result<NamedContract<AccountId>>);
                    let passed_args_num = match params {
                        _jsonrpc_core::Params::Array(ref v) => Ok(v.len()),
                        _jsonrpc_core::Params::None => Ok(0),
                        _ => Err(_jsonrpc_core::Error::invalid_params(
                            "`params` should be an array",
                        )),
                    };
                    let params =
                        passed_args_num.and_then(|passed_args_num| match passed_args_num {
                            _ if passed_args_num < 1usize => {
                                Err(_jsonrpc_core::Error::invalid_params({
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &["`params` should have at least ", " argument(s)"],
                                        &match (&1usize,) {
                                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Display::fmt,
                                            )],
                                        },
                                    ));
                                    res
                                }))
                            }
                            1usize => params
                                .parse::<(Box<String>,)>()
                                .map(|(a,)| (a, None))
                                .map_err(Into::into),
                            2usize => params
                                .parse::<(Box<String>, Option<BlockHash>)>()
                                .map(|(a, b)| (a, b))
                                .map_err(Into::into),
                            _ => Err(_jsonrpc_core::Error::invalid_params_with_details(
                                {
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &["Expected from ", " to ", " parameters."],
                                        &match (&1usize, &2usize) {
                                            (arg0, arg1) => [
                                                ::core::fmt::ArgumentV1::new(
                                                    arg0,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg1,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                            ],
                                        },
                                    ));
                                    res
                                },
                                {
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &["Got: "],
                                        &match (&passed_args_num,) {
                                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Display::fmt,
                                            )],
                                        },
                                    ));
                                    res
                                },
                            )),
                        });
                    match params {
                        Ok((a, b)) => {
                            use self::_futures::{Future, IntoFuture};
                            let fut = (method)(base, a, b)
                                .into_future()
                                .map(|value| {
                                    _jsonrpc_core::to_value(value)
                                        .expect("Expected always-serializable type; qed")
                                })
                                .map_err(Into::into as fn(_) -> _jsonrpc_core::Error);
                            _futures::future::Either::A(fut)
                        }
                        Err(e) => _futures::future::Either::B(_futures::failed(e)),
                    }
                });
                del
            }
        }
    }
}
pub use self::rpc_impl_ComposeRPCApi::gen_client;
pub use self::rpc_impl_ComposeRPCApi::gen_server::ComposeRPCApi;
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
    fn get_named_contract(
        &self,
        contract_path: Box<String>,
        at: Option<<Block as BlockT>::Hash>,
    ) -> Result<NamedContract<AccountId>> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));
        let result = api
            .get_named_contract(&at, contract_path)
            .map_err(runtime_error_into_rpc_err)?;
        Ok(result.unwrap())
    }
}
#[rustc_main]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[])
}
