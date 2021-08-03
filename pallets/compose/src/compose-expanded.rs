#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;

use sp_std::prelude::*;
use sp_std::if_std;
use frame_support::{RuntimeDebugNoBound};
use codec::{Encode, Decode};

mod serstring {



    // pub enum PathGen<'a> {
    //     /// A value domain/route combination, with domain D separate from route R.
    //     DomainRoute(&'a SerString, &'a SerString),
    //     /// A path, expressed as <domain> (if route is None) or <domain>:<route>.
    //     Path(&'a SerString),
    // }












    // let route = route.clone();



    // #[cfg(feature = "std")]





    // #[cfg(feature = "std")]



















    // #[pallet::hooks]
    // impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T>
    // 	// TODO_MAYBE_WHERE_CLAUSE
    // {
    // 	// TODO_ON_FINALIZE
    // 	// TODO_ON_INITIALIZE
    // 	// TODO_ON_RUNTIME_UPGRADE
    // 	// TODO_INTEGRITY_TEST
    // 	// TODO_OFFCHAIN_WORKER
    // }







    // we have reached a path where the caller is approved, and we have a (potentially empty) list of additional paths
    // to be approved, so break with caller approved
    // The path has already been registered but the caller is not approved on it, so they are disapproved to add a contract
    // at or below this point in the hierarchy, so leave is_approved == false and break
    // Since this path doesn't yet exist, if we find a super path where the caller is approved, then we'll want to
    // approve the caller for this sub path as well.  I.e., if caller A registers a domain "new_domain", they will
    // be the approver on "new_domain:".  If they then approve caller B on the path "new_domain:a", then if caller
    // B registers a contract at "new_domain:a/path/to/the/contract", then we want to approve caller B for that
    // path and also the paths "new_domain:a/path/to/the", "new_domain:a/path/to" and "new_domain:a/path".  We
    // don't want to approve caller B for "new_domain:a" (since they are already approved for that path) nor for "new_domain:"
    // (since caller A owns that path, only they have the right to approve caller B for it).__rust_force_expr!

    // Note that at this point, caller A does NOT have approvals on any of the sub-paths of "new_domain:a", only
    // caller B has them.  Caller B can approve caller A on those paths at their discretion, or call A can 


    use codec::{Encode, Decode, EncodeLike};
    use sp_core::RuntimeDebug;
    use sp_std::vec::Vec;
    /// A serializable string equivalent
    pub struct SerString {
        val: Vec<u8>,
    }
    impl ::core::marker::StructuralEq for SerString { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Eq for SerString {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            { let _: ::core::cmp::AssertParamIsEq<Vec<u8>>; }
        }
    }
    impl core::fmt::Debug for SerString {
        fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
            fmt.debug_struct("SerString").field("val", &self.val).finish()
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for SerString {
        #[inline]
        fn clone(&self) -> SerString {
            match *self {
                SerString { val: ref __self_0_0 } =>
                SerString{val: ::core::clone::Clone::clone(&(*__self_0_0)),},
            }
        }
    }
    impl SerString {
        pub fn from_str(s: &str) -> Self { Self{val: s.into(),} }
        pub fn to_str(&self) -> &str {
            sp_std::str::from_utf8(&self.val).unwrap()
        }
        pub fn replace(&self, from: &str, to: &str) -> SerString {
            SerString::from_str(&self.to_str().replace(from, to)[..])
        }
        pub fn to_lowercase(&self) -> SerString {
            SerString::from_str(&self.to_str().to_lowercase()[..])
        }
        pub fn split(&self, separator: &str) -> Vec<SerString> {
            self.to_str().split(separator).map(SerString::from_str).collect()
        }
        pub fn join(v: Vec<SerString>, separator: &str) -> SerString {
            Self::from_str(&v.iter().map(|s|
                                             s.to_str()).collect::<Vec<&str>>().join(separator)[..])
        }
    }
    /// Convenience macro to use the format! interface to get a SerString
    #[macro_export]
    macro_rules! format_ser_string {
        ($ ($ args : tt) *) =>
        {
            {
                SerString
                {
                    val : sp_std :: fmt :: format!
                    ($ ($ args) *).as_bytes().to_vec()
                }
            }
        } ;
    }
    impl From<&str> for SerString {
        fn from(s: &str) -> Self { SerString::from_str(s) }
    }
    impl Default for SerString {
        fn default() -> Self { Self{val: Default::default(),} }
    }
    impl PartialEq for SerString {
        fn eq(&self, other: &Self) -> bool { self.as_ref() == other.as_ref() }
    }
    impl AsRef<[u8]> for SerString {
        fn as_ref(&self) -> &[u8] { self.val.as_ref() }
    }
    impl Encode for SerString {
        fn encode(&self) -> Vec<u8> { self.val.encode() }
    }
    impl EncodeLike for SerString { }
    impl Decode for SerString {
        fn decode<I: codec::Input>(value: &mut I)
         -> Result<Self, codec::Error> {
            Decode::decode(value).map(|val| { Self{val,} })
        }
    }
    impl sp_std::fmt::Display for SerString {
        fn fmt(&self, f: &mut sp_std::fmt::Formatter) -> sp_std::fmt::Result {
            f.write_fmt(::core::fmt::Arguments::new_v1(&[""],
                                                       &match (&self.to_str(),)
                                                            {
                                                            (arg0,) =>
                                                            [::core::fmt::ArgumentV1::new(arg0,
                                                                                          ::core::fmt::Display::fmt)],
                                                        }))
        }
    }
    #[cfg(feature = "std")]
    impl serde::Serialize for SerString {
        fn serialize<S: serde::Serializer>(&self, serializer: S)
         -> Result<S::Ok, S::Error> {
            self.val.serialize(serializer)
        }
    }
    #[cfg(feature = "std")]
    impl <'de> serde::Deserialize<'de> for SerString {
        fn deserialize<D: serde::Deserializer<'de>>(de: D)
         -> Result<Self, D::Error> {
            String::deserialize(de).map(|val| { Self{val: val.into(),} })
        }
    }
    /// Create a const [`SerString`].
    #[macro_export]
    macro_rules! create_compose_str {
        ($ y : expr) => { { $ crate :: SerString :: Owned($ y) } }
    }
}
use serstring::SerString;
type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
/// The hash value of the WASM code in the smart contract.
type CodeHashOf<T> = <T as frame_system::Config>::Hash;
/// The WASM code instantiated in the smart contract.
type Code = Vec<u8>;
/// An approver is an account that has been approved by the path owner to also manage the path.  The approver's management 
/// rights are fully the same as the path owner's rights within the path's sub-domain.  E.g., if an approver is
/// given approval rights at the domain level (such as for "MyAPI"), then the approver may execute any
/// domain management calls.  If an approver is given approval rights for a given path (such as for "MyAPI:delegator"),
/// the approver may for example add or remove sub-paths (such as "MyAPI:delegator/v2.0", "MyAPI:delegator/LTS")
type Approver<T> = AccountIdOf<T>;
/// A path is a case-insensitive <domain>[:<route>/[<sub-route>/[...]]] UTF-8 string.  Examples of valid paths are:
///          MyAPI:delegator
///          myapi:adder
///          myapi:subber
///          MariasBakery:Order/Cookies/v2.1
///          mariasbakery:order/cakes/v2.0
///    		 MariasBakery                       <-- this is a pure / top-level domain
pub struct Path {
    /// <domain> as a string with original case preserved. If present any ":" will be removed
    /// (e.g., the domain is "MariasBakery" rather than "MariasBakery:")
    pub domain: SerString,
    /// <domain> as a lower-cased string.
    pub domain_lc: SerString,
    /// <route> as a string with original case preserved.  
    /// A top-level domain has a route of "".
    /// Routes are also UTF-8 strings, where sub-routes within the route are slash-separated.
    /// Routes are case-insensitive; Order/Cookies and order/cookies are considered the same route.
    /// A route of None corresponds to a pure / top-level (i.e., a route-less) domain
    pub route: SerString,
    /// <route> as a lower-cased string.
    pub route_lc: SerString,
    /// <domain>: if route is none, else <domain>:<route>, as a string with original ca
    /// s
    /// 
    /// 
    /// e
    ///  
    /// p
    /// 
    /// r
    /// eserved.
    pub path: SerString,
    /// <path> as a lower-cased string.
    pub path_lc: SerString,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for Path {
    #[inline]
    fn clone(&self) -> Path {
        match *self {
            Path {
            domain: ref __self_0_0,
            domain_lc: ref __self_0_1,
            route: ref __self_0_2,
            route_lc: ref __self_0_3,
            path: ref __self_0_4,
            path_lc: ref __self_0_5 } =>
            Path{domain: ::core::clone::Clone::clone(&(*__self_0_0)),
                 domain_lc: ::core::clone::Clone::clone(&(*__self_0_1)),
                 route: ::core::clone::Clone::clone(&(*__self_0_2)),
                 route_lc: ::core::clone::Clone::clone(&(*__self_0_3)),
                 path: ::core::clone::Clone::clone(&(*__self_0_4)),
                 path_lc: ::core::clone::Clone::clone(&(*__self_0_5)),},
        }
    }
}
impl ::core::marker::StructuralEq for Path { }
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::Eq for Path {
    #[inline]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::core::cmp::AssertParamIsEq<SerString>;
            let _: ::core::cmp::AssertParamIsEq<SerString>;
            let _: ::core::cmp::AssertParamIsEq<SerString>;
            let _: ::core::cmp::AssertParamIsEq<SerString>;
            let _: ::core::cmp::AssertParamIsEq<SerString>;
            let _: ::core::cmp::AssertParamIsEq<SerString>;
        }
    }
}
impl ::core::marker::StructuralPartialEq for Path { }
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::PartialEq for Path {
    #[inline]
    fn eq(&self, other: &Path) -> bool {
        match *other {
            Path {
            domain: ref __self_1_0,
            domain_lc: ref __self_1_1,
            route: ref __self_1_2,
            route_lc: ref __self_1_3,
            path: ref __self_1_4,
            path_lc: ref __self_1_5 } =>
            match *self {
                Path {
                domain: ref __self_0_0,
                domain_lc: ref __self_0_1,
                route: ref __self_0_2,
                route_lc: ref __self_0_3,
                path: ref __self_0_4,
                path_lc: ref __self_0_5 } =>
                (*__self_0_0) == (*__self_1_0) &&
                    (*__self_0_1) == (*__self_1_1) &&
                    (*__self_0_2) == (*__self_1_2) &&
                    (*__self_0_3) == (*__self_1_3) &&
                    (*__self_0_4) == (*__self_1_4) &&
                    (*__self_0_5) == (*__self_1_5),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &Path) -> bool {
        match *other {
            Path {
            domain: ref __self_1_0,
            domain_lc: ref __self_1_1,
            route: ref __self_1_2,
            route_lc: ref __self_1_3,
            path: ref __self_1_4,
            path_lc: ref __self_1_5 } =>
            match *self {
                Path {
                domain: ref __self_0_0,
                domain_lc: ref __self_0_1,
                route: ref __self_0_2,
                route_lc: ref __self_0_3,
                path: ref __self_0_4,
                path_lc: ref __self_0_5 } =>
                (*__self_0_0) != (*__self_1_0) ||
                    (*__self_0_1) != (*__self_1_1) ||
                    (*__self_0_2) != (*__self_1_2) ||
                    (*__self_0_3) != (*__self_1_3) ||
                    (*__self_0_4) != (*__self_1_4) ||
                    (*__self_0_5) != (*__self_1_5),
            },
        }
    }
}
const _: () =
    {
        impl core::fmt::Debug for Path {
            fn fmt(&self, fmt: &mut core::fmt::Formatter)
             -> core::fmt::Result {
                fmt.debug_struct("Path").field("domain",
                                               &self.domain).field("domain_lc",
                                                                   &self.domain_lc).field("route",
                                                                                          &self.route).field("route_lc",
                                                                                                             &self.route_lc).field("path",
                                                                                                                                   &self.path).field("path_lc",
                                                                                                                                                     &self.path_lc).finish()
            }
        }
    };
const _: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate codec as _parity_scale_codec;
        impl _parity_scale_codec::Encode for Path {
            fn encode_to<__CodecOutputEdqy: _parity_scale_codec::Output +
                         ?Sized>(&self,
                                 __codec_dest_edqy: &mut __CodecOutputEdqy) {
                _parity_scale_codec::Encode::encode_to(&self.domain,
                                                       __codec_dest_edqy);
                _parity_scale_codec::Encode::encode_to(&self.domain_lc,
                                                       __codec_dest_edqy);
                _parity_scale_codec::Encode::encode_to(&self.route,
                                                       __codec_dest_edqy);
                _parity_scale_codec::Encode::encode_to(&self.route_lc,
                                                       __codec_dest_edqy);
                _parity_scale_codec::Encode::encode_to(&self.path,
                                                       __codec_dest_edqy);
                _parity_scale_codec::Encode::encode_to(&self.path_lc,
                                                       __codec_dest_edqy);
            }
        }
        impl _parity_scale_codec::EncodeLike for Path { }
    };
const _: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate codec as _parity_scale_codec;
        impl _parity_scale_codec::Decode for Path {
            fn decode<__CodecInputEdqy: _parity_scale_codec::Input>(__codec_input_edqy:
                                                                        &mut __CodecInputEdqy)
             -> ::core::result::Result<Self, _parity_scale_codec::Error> {
                ::core::result::Result::Ok(Path{domain:
                                                    {
                                                        let __codec_res_edqy =
                                                            <SerString as
                                                                _parity_scale_codec::Decode>::decode(__codec_input_edqy);
                                                        match __codec_res_edqy
                                                            {
                                                            ::core::result::Result::Err(e)
                                                            =>
                                                            return ::core::result::Result::Err(e.chain("Could not decode `Path::domain`")),
                                                            ::core::result::Result::Ok(__codec_res_edqy)
                                                            =>
                                                            __codec_res_edqy,
                                                        }
                                                    },
                                                domain_lc:
                                                    {
                                                        let __codec_res_edqy =
                                                            <SerString as
                                                                _parity_scale_codec::Decode>::decode(__codec_input_edqy);
                                                        match __codec_res_edqy
                                                            {
                                                            ::core::result::Result::Err(e)
                                                            =>
                                                            return ::core::result::Result::Err(e.chain("Could not decode `Path::domain_lc`")),
                                                            ::core::result::Result::Ok(__codec_res_edqy)
                                                            =>
                                                            __codec_res_edqy,
                                                        }
                                                    },
                                                route:
                                                    {
                                                        let __codec_res_edqy =
                                                            <SerString as
                                                                _parity_scale_codec::Decode>::decode(__codec_input_edqy);
                                                        match __codec_res_edqy
                                                            {
                                                            ::core::result::Result::Err(e)
                                                            =>
                                                            return ::core::result::Result::Err(e.chain("Could not decode `Path::route`")),
                                                            ::core::result::Result::Ok(__codec_res_edqy)
                                                            =>
                                                            __codec_res_edqy,
                                                        }
                                                    },
                                                route_lc:
                                                    {
                                                        let __codec_res_edqy =
                                                            <SerString as
                                                                _parity_scale_codec::Decode>::decode(__codec_input_edqy);
                                                        match __codec_res_edqy
                                                            {
                                                            ::core::result::Result::Err(e)
                                                            =>
                                                            return ::core::result::Result::Err(e.chain("Could not decode `Path::route_lc`")),
                                                            ::core::result::Result::Ok(__codec_res_edqy)
                                                            =>
                                                            __codec_res_edqy,
                                                        }
                                                    },
                                                path:
                                                    {
                                                        let __codec_res_edqy =
                                                            <SerString as
                                                                _parity_scale_codec::Decode>::decode(__codec_input_edqy);
                                                        match __codec_res_edqy
                                                            {
                                                            ::core::result::Result::Err(e)
                                                            =>
                                                            return ::core::result::Result::Err(e.chain("Could not decode `Path::path`")),
                                                            ::core::result::Result::Ok(__codec_res_edqy)
                                                            =>
                                                            __codec_res_edqy,
                                                        }
                                                    },
                                                path_lc:
                                                    {
                                                        let __codec_res_edqy =
                                                            <SerString as
                                                                _parity_scale_codec::Decode>::decode(__codec_input_edqy);
                                                        match __codec_res_edqy
                                                            {
                                                            ::core::result::Result::Err(e)
                                                            =>
                                                            return ::core::result::Result::Err(e.chain("Could not decode `Path::path_lc`")),
                                                            ::core::result::Result::Ok(__codec_res_edqy)
                                                            =>
                                                            __codec_res_edqy,
                                                        }
                                                    },})
            }
        }
    };
impl Path {
    pub fn new(domain: SerString, route: SerString) -> Self {
        let domain = domain.replace(":", "");
        let domain_lc = domain.to_lowercase();
        let route_lc = route.to_lowercase();
        let path =
            SerString::from_str(&[domain.to_str(), ":",
                                  route.to_str()].concat()[..]);
        let path_lc = path.to_lowercase();
        Self{domain, domain_lc, route, route_lc, path, path_lc,}
    }
    pub fn new_from_path(path: SerString) -> Result<Path, SerString> {
        let split_path = path.split(":");
        if (split_path.len() == 1) || (split_path.len() == 2) {
            let route: SerString =
                if split_path.len() == 1 {
                    SerString::from_str("").into()
                } else { split_path[1].clone() };
            Ok(Self::new(split_path[0].clone(), route))
        } else { Err("Malformed Path".into()) }
    }
}
impl sp_std::fmt::Display for Path {
    fn fmt(&self, f: &mut sp_std::fmt::Formatter) -> sp_std::fmt::Result {
        f.write_fmt(::core::fmt::Arguments::new_v1(&["Path { ", " }"],
                                                   &match (&self.path_lc.to_str(),)
                                                        {
                                                        (arg0,) =>
                                                        [::core::fmt::ArgumentV1::new(arg0,
                                                                                      ::core::fmt::Display::fmt)],
                                                    }))
    }
}
/// The code hash for a smart contract, and optionally the underlying code of that contract
pub struct Contract<T: frame_system::Config> {
    /// Optional: hash for the code referenced by this contract.
    pub code_hash: CodeHashOf<T>,
    /// Optional: code for this contract.
    pub code: Option<Code>,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <T: ::core::clone::Clone + frame_system::Config> ::core::clone::Clone for
 Contract<T> {
    #[inline]
    fn clone(&self) -> Contract<T> {
        match *self {
            Contract { code_hash: ref __self_0_0, code: ref __self_0_1 } =>
            Contract{code_hash: ::core::clone::Clone::clone(&(*__self_0_0)),
                     code: ::core::clone::Clone::clone(&(*__self_0_1)),},
        }
    }
}
impl <T: frame_system::Config> ::core::marker::StructuralEq for Contract<T> {
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <T: ::core::cmp::Eq + frame_system::Config> ::core::cmp::Eq for
 Contract<T> {
    #[inline]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::core::cmp::AssertParamIsEq<CodeHashOf<T>>;
            let _: ::core::cmp::AssertParamIsEq<Option<Code>>;
        }
    }
}
impl <T: frame_system::Config> ::core::marker::StructuralPartialEq for
 Contract<T> {
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <T: ::core::cmp::PartialEq + frame_system::Config> ::core::cmp::PartialEq
 for Contract<T> {
    #[inline]
    fn eq(&self, other: &Contract<T>) -> bool {
        match *other {
            Contract { code_hash: ref __self_1_0, code: ref __self_1_1 } =>
            match *self {
                Contract { code_hash: ref __self_0_0, code: ref __self_0_1 }
                =>
                (*__self_0_0) == (*__self_1_0) &&
                    (*__self_0_1) == (*__self_1_1),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &Contract<T>) -> bool {
        match *other {
            Contract { code_hash: ref __self_1_0, code: ref __self_1_1 } =>
            match *self {
                Contract { code_hash: ref __self_0_0, code: ref __self_0_1 }
                =>
                (*__self_0_0) != (*__self_1_0) ||
                    (*__self_0_1) != (*__self_1_1),
            },
        }
    }
}
const _: () =
    {
        impl <T: frame_system::Config> core::fmt::Debug for Contract<T> {
            fn fmt(&self, fmt: &mut core::fmt::Formatter)
             -> core::fmt::Result {
                fmt.debug_struct("Contract").field("code_hash",
                                                   &self.code_hash).field("code",
                                                                          &self.code).finish()
            }
        }
    };
const _: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate codec as _parity_scale_codec;
        impl <T: frame_system::Config> _parity_scale_codec::Encode for
         Contract<T> where CodeHashOf<T>: _parity_scale_codec::Encode,
         CodeHashOf<T>: _parity_scale_codec::Encode {
            fn encode_to<__CodecOutputEdqy: _parity_scale_codec::Output +
                         ?Sized>(&self,
                                 __codec_dest_edqy: &mut __CodecOutputEdqy) {
                _parity_scale_codec::Encode::encode_to(&self.code_hash,
                                                       __codec_dest_edqy);
                _parity_scale_codec::Encode::encode_to(&self.code,
                                                       __codec_dest_edqy);
            }
        }
        impl <T: frame_system::Config> _parity_scale_codec::EncodeLike for
         Contract<T> where CodeHashOf<T>: _parity_scale_codec::Encode,
         CodeHashOf<T>: _parity_scale_codec::Encode {
        }
    };
const _: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate codec as _parity_scale_codec;
        impl <T: frame_system::Config> _parity_scale_codec::Decode for
         Contract<T> where CodeHashOf<T>: _parity_scale_codec::Decode,
         CodeHashOf<T>: _parity_scale_codec::Decode {
            fn decode<__CodecInputEdqy: _parity_scale_codec::Input>(__codec_input_edqy:
                                                                        &mut __CodecInputEdqy)
             -> ::core::result::Result<Self, _parity_scale_codec::Error> {
                ::core::result::Result::Ok(Contract::<T>{code_hash:
                                                             {
                                                                 let __codec_res_edqy =
                                                                     <CodeHashOf<T>
                                                                         as
                                                                         _parity_scale_codec::Decode>::decode(__codec_input_edqy);
                                                                 match __codec_res_edqy
                                                                     {
                                                                     ::core::result::Result::Err(e)
                                                                     =>
                                                                     return ::core::result::Result::Err(e.chain("Could not decode `Contract::code_hash`")),
                                                                     ::core::result::Result::Ok(__codec_res_edqy)
                                                                     =>
                                                                     __codec_res_edqy,
                                                                 }
                                                             },
                                                         code:
                                                             {
                                                                 let __codec_res_edqy =
                                                                     <Option<Code>
                                                                         as
                                                                         _parity_scale_codec::Decode>::decode(__codec_input_edqy);
                                                                 match __codec_res_edqy
                                                                     {
                                                                     ::core::result::Result::Err(e)
                                                                     =>
                                                                     return ::core::result::Result::Err(e.chain("Could not decode `Contract::code`")),
                                                                     ::core::result::Result::Ok(__codec_res_edqy)
                                                                     =>
                                                                     __codec_res_edqy,
                                                                 }
                                                             },})
            }
        }
    };
impl <T: frame_system::Config> Contract<T> {
    pub fn new(code_hash: &CodeHashOf<T>, code: &Option<Code>) -> Self {
        Self{code_hash: code_hash.clone(), code: code.clone(),}
    }
}
impl <T: frame_system::Config> sp_std::fmt::Display for Contract<T> {
    fn fmt(&self, f: &mut sp_std::fmt::Formatter) -> sp_std::fmt::Result {
        f.write_fmt(::core::fmt::Arguments::new_v1_formatted(&["Contract { ",
                                                               " }"],
                                                             &match (&self.code_hash,)
                                                                  {
                                                                  (arg0,) =>
                                                                  [::core::fmt::ArgumentV1::new(arg0,
                                                                                                ::core::fmt::Debug::fmt)],
                                                              },
                                                             &[::core::fmt::rt::v1::Argument{position:
                                                                                                 0usize,
                                                                                             format:
                                                                                                 ::core::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                     ' ',
                                                                                                                                 align:
                                                                                                                                     ::core::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                 flags:
                                                                                                                                     4u32,
                                                                                                                                 precision:
                                                                                                                                     ::core::fmt::rt::v1::Count::Implied,
                                                                                                                                 width:
                                                                                                                                     ::core::fmt::rt::v1::Count::Implied,},}]))
    }
}
/// Registered path.
pub struct RegisteredPath<T: frame_system::Config> {
    /// Owner of the path.
    pub owner: AccountIdOf<T>,
    /// Full path (i.e., <domain>:<route>).  The path for a domain-name reservation is <domain>: 
    pub path: Path,
    /// The code hash for a smart contract, and optionally the underlying code of that contract
    pub contract: Option<Contract<T>>,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <T: ::core::clone::Clone + frame_system::Config> ::core::clone::Clone for
 RegisteredPath<T> {
    #[inline]
    fn clone(&self) -> RegisteredPath<T> {
        match *self {
            RegisteredPath {
            owner: ref __self_0_0,
            path: ref __self_0_1,
            contract: ref __self_0_2 } =>
            RegisteredPath{owner: ::core::clone::Clone::clone(&(*__self_0_0)),
                           path: ::core::clone::Clone::clone(&(*__self_0_1)),
                           contract:
                               ::core::clone::Clone::clone(&(*__self_0_2)),},
        }
    }
}
impl <T: frame_system::Config> ::core::marker::StructuralEq for
 RegisteredPath<T> {
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <T: ::core::cmp::Eq + frame_system::Config> ::core::cmp::Eq for
 RegisteredPath<T> {
    #[inline]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::core::cmp::AssertParamIsEq<AccountIdOf<T>>;
            let _: ::core::cmp::AssertParamIsEq<Path>;
            let _: ::core::cmp::AssertParamIsEq<Option<Contract<T>>>;
        }
    }
}
impl <T: frame_system::Config> ::core::marker::StructuralPartialEq for
 RegisteredPath<T> {
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <T: ::core::cmp::PartialEq + frame_system::Config> ::core::cmp::PartialEq
 for RegisteredPath<T> {
    #[inline]
    fn eq(&self, other: &RegisteredPath<T>) -> bool {
        match *other {
            RegisteredPath {
            owner: ref __self_1_0,
            path: ref __self_1_1,
            contract: ref __self_1_2 } =>
            match *self {
                RegisteredPath {
                owner: ref __self_0_0,
                path: ref __self_0_1,
                contract: ref __self_0_2 } =>
                (*__self_0_0) == (*__self_1_0) &&
                    (*__self_0_1) == (*__self_1_1) &&
                    (*__self_0_2) == (*__self_1_2),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &RegisteredPath<T>) -> bool {
        match *other {
            RegisteredPath {
            owner: ref __self_1_0,
            path: ref __self_1_1,
            contract: ref __self_1_2 } =>
            match *self {
                RegisteredPath {
                owner: ref __self_0_0,
                path: ref __self_0_1,
                contract: ref __self_0_2 } =>
                (*__self_0_0) != (*__self_1_0) ||
                    (*__self_0_1) != (*__self_1_1) ||
                    (*__self_0_2) != (*__self_1_2),
            },
        }
    }
}
const _: () =
    {
        impl <T: frame_system::Config> core::fmt::Debug for RegisteredPath<T>
         {
            fn fmt(&self, fmt: &mut core::fmt::Formatter)
             -> core::fmt::Result {
                fmt.debug_struct("RegisteredPath").field("owner",
                                                         &self.owner).field("path",
                                                                            &self.path).field("contract",
                                                                                              &self.contract).finish()
            }
        }
    };
const _: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate codec as _parity_scale_codec;
        impl <T: frame_system::Config> _parity_scale_codec::Encode for
         RegisteredPath<T> where AccountIdOf<T>: _parity_scale_codec::Encode,
         AccountIdOf<T>: _parity_scale_codec::Encode,
         Option<Contract<T>>: _parity_scale_codec::Encode,
         Option<Contract<T>>: _parity_scale_codec::Encode {
            fn encode_to<__CodecOutputEdqy: _parity_scale_codec::Output +
                         ?Sized>(&self,
                                 __codec_dest_edqy: &mut __CodecOutputEdqy) {
                _parity_scale_codec::Encode::encode_to(&self.owner,
                                                       __codec_dest_edqy);
                _parity_scale_codec::Encode::encode_to(&self.path,
                                                       __codec_dest_edqy);
                _parity_scale_codec::Encode::encode_to(&self.contract,
                                                       __codec_dest_edqy);
            }
        }
        impl <T: frame_system::Config> _parity_scale_codec::EncodeLike for
         RegisteredPath<T> where AccountIdOf<T>: _parity_scale_codec::Encode,
         AccountIdOf<T>: _parity_scale_codec::Encode,
         Option<Contract<T>>: _parity_scale_codec::Encode,
         Option<Contract<T>>: _parity_scale_codec::Encode {
        }
    };
const _: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate codec as _parity_scale_codec;
        impl <T: frame_system::Config> _parity_scale_codec::Decode for
         RegisteredPath<T> where AccountIdOf<T>: _parity_scale_codec::Decode,
         AccountIdOf<T>: _parity_scale_codec::Decode,
         Option<Contract<T>>: _parity_scale_codec::Decode,
         Option<Contract<T>>: _parity_scale_codec::Decode {
            fn decode<__CodecInputEdqy: _parity_scale_codec::Input>(__codec_input_edqy:
                                                                        &mut __CodecInputEdqy)
             -> ::core::result::Result<Self, _parity_scale_codec::Error> {
                ::core::result::Result::Ok(RegisteredPath::<T>{owner:
                                                                   {
                                                                       let __codec_res_edqy =
                                                                           <AccountIdOf<T>
                                                                               as
                                                                               _parity_scale_codec::Decode>::decode(__codec_input_edqy);
                                                                       match __codec_res_edqy
                                                                           {
                                                                           ::core::result::Result::Err(e)
                                                                           =>
                                                                           return ::core::result::Result::Err(e.chain("Could not decode `RegisteredPath::owner`")),
                                                                           ::core::result::Result::Ok(__codec_res_edqy)
                                                                           =>
                                                                           __codec_res_edqy,
                                                                       }
                                                                   },
                                                               path:
                                                                   {
                                                                       let __codec_res_edqy =
                                                                           <Path
                                                                               as
                                                                               _parity_scale_codec::Decode>::decode(__codec_input_edqy);
                                                                       match __codec_res_edqy
                                                                           {
                                                                           ::core::result::Result::Err(e)
                                                                           =>
                                                                           return ::core::result::Result::Err(e.chain("Could not decode `RegisteredPath::path`")),
                                                                           ::core::result::Result::Ok(__codec_res_edqy)
                                                                           =>
                                                                           __codec_res_edqy,
                                                                       }
                                                                   },
                                                               contract:
                                                                   {
                                                                       let __codec_res_edqy =
                                                                           <Option<Contract<T>>
                                                                               as
                                                                               _parity_scale_codec::Decode>::decode(__codec_input_edqy);
                                                                       match __codec_res_edqy
                                                                           {
                                                                           ::core::result::Result::Err(e)
                                                                           =>
                                                                           return ::core::result::Result::Err(e.chain("Could not decode `RegisteredPath::contract`")),
                                                                           ::core::result::Result::Ok(__codec_res_edqy)
                                                                           =>
                                                                           __codec_res_edqy,
                                                                       }
                                                                   },})
            }
        }
    };
impl <T: frame_system::Config> RegisteredPath<T> {
    pub fn new(owner: &AccountIdOf<T>, path: &Path,
               contract: &Option<Contract<T>>) -> Self {
        Self{owner: owner.clone(),
             path: path.clone(),
             contract: contract.clone(),}
    }
}
impl <T: frame_system::Config> sp_std::fmt::Display for RegisteredPath<T> {
    fn fmt(&self, f: &mut sp_std::fmt::Formatter) -> sp_std::fmt::Result {
        match &self.contract {
            Some(contract) =>
            f.write_fmt(::core::fmt::Arguments::new_v1(&["RegisteredPath {owner: ",
                                                         ", path: ",
                                                         ", contract: ",
                                                         " }"],
                                                       &match (&self.owner,
                                                               &self.path.path_lc,
                                                               &contract.code_hash)
                                                            {
                                                            (arg0, arg1, arg2)
                                                            =>
                                                            [::core::fmt::ArgumentV1::new(arg0,
                                                                                          ::core::fmt::Debug::fmt),
                                                             ::core::fmt::ArgumentV1::new(arg1,
                                                                                          ::core::fmt::Display::fmt),
                                                             ::core::fmt::ArgumentV1::new(arg2,
                                                                                          ::core::fmt::Debug::fmt)],
                                                        })),
            None =>
            f.write_fmt(::core::fmt::Arguments::new_v1(&["RegisteredPath {owner: ",
                                                         ", path: ",
                                                         ", contract: None"],
                                                       &match (&self.owner,
                                                               &self.path.path_lc)
                                                            {
                                                            (arg0, arg1) =>
                                                            [::core::fmt::ArgumentV1::new(arg0,
                                                                                          ::core::fmt::Debug::fmt),
                                                             ::core::fmt::ArgumentV1::new(arg1,
                                                                                          ::core::fmt::Display::fmt)],
                                                        })),
        }
    }
}
pub use pallet::*;
#[doc =
  r"
			The module that hosts all the
			[FRAME](https://substrate.dev/docs/en/knowledgebase/runtime/frame)
			types needed to add this pallet to a
			[runtime](https://substrate.dev/docs/en/knowledgebase/runtime/).
			"]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use super::*;
    #[doc =
      " Configure the pallet by specifying the parameters and types on which it depends."]
    pub trait Config: frame_system::Config {
        #[doc =
          " Because this pallet emits events, it depends on the runtime\'s definition of an event."]
        type Event: From<Event<Self>> +
         IsType<<Self as frame_system::Config>::Event>;
        #[doc = " Maximum size for a domain name, in bytes"]
        type MaxDomainByteSize: Get<u32>;
        #[doc = " Maximum size for a route name, in bytes"]
        type MaxRouteByteSize: Get<u32>;
    }
    #[doc =
      r"
			The [pallet](https://substrate.dev/docs/en/knowledgebase/runtime/pallets) implementing
			the on-chain logic.
			"]
    pub struct Pallet<T>(PhantomData<T>);
    const _: () =
        {
            impl <T> core::clone::Clone for Pallet<T> {
                fn clone(&self) -> Self {
                    Self(core::clone::Clone::clone(&self.0))
                }
            }
        };
    const _: () =
        {
            impl <T> core::cmp::Eq for Pallet<T> { }
        };
    const _: () =
        {
            impl <T> core::cmp::PartialEq for Pallet<T> {
                fn eq(&self, other: &Self) -> bool {
                    true && self.0 == other.0
                }
            }
        };
    const _: () =
        {
            impl <T> core::fmt::Debug for Pallet<T> {
                fn fmt(&self, fmt: &mut core::fmt::Formatter)
                 -> core::fmt::Result {
                    fmt.debug_tuple("Pallet").field(&self.0).finish()
                }
            }
        };
    #[doc =
      " A table of registered paths (i.e., domain/route combinations).  A domain itself will always be registered with a route of None; "]
    #[doc =
      " all other registered paths will have a Some<SerString> route string"]
    #[doc = " \\[domain, route, path/owner registration info\\]"]
    #[allow(type_alias_bounds)]
    pub type RegisteredPaths<T: Config> =
     StorageDoubleMap<_GeneratedPrefixForStorageRegisteredPaths<T>,
                      Blake2_128Concat, SerString, Blake2_128Concat,
                      SerString, RegisteredPath<T>>;
    #[doc =
      " A table of registered approvers for a given path, i.e., account(s) (if any) that have been approved by the path owner to also manage the path."]
    #[doc = " \\[domain, (route, approver)\\]"]
    #[allow(type_alias_bounds)]
    pub type RegisteredApprovers<T: Config> =
     StorageDoubleMap<_GeneratedPrefixForStorageRegisteredApprovers<T>,
                      Blake2_128Concat, SerString, Blake2_128Concat,
                      (SerString, Approver<T>), ()>;
    #[doc =
      r"
			The [event](https://substrate.dev/docs/en/knowledgebase/runtime/events) emitted
			by this pallet.
			"]
    pub enum Event<T: Config> {

        #[doc = " An owner was set or reset. \\[who, path\\]"]
        OwnerSet(T::AccountId, SerString),

        #[doc(hidden)]
        #[codec(skip)]
        __Ignore(frame_support::sp_std::marker::PhantomData<(T)>,
                 frame_support::Never),
    }
    const _: () =
        {
            impl <T: Config> core::clone::Clone for Event<T> {
                fn clone(&self) -> Self {
                    match self {
                        Self::OwnerSet(ref _0, ref _1) =>
                        Self::OwnerSet(core::clone::Clone::clone(_0),
                                       core::clone::Clone::clone(_1)),
                        Self::__Ignore(ref _0, ref _1) =>
                        Self::__Ignore(core::clone::Clone::clone(_0),
                                       core::clone::Clone::clone(_1)),
                    }
                }
            }
        };
    const _: () =
        {
            impl <T: Config> core::cmp::Eq for Event<T> { }
        };
    const _: () =
        {
            impl <T: Config> core::cmp::PartialEq for Event<T> {
                fn eq(&self, other: &Self) -> bool {
                    match (self, other) {
                        (Self::OwnerSet(_0, _1),
                         Self::OwnerSet(_0_other, _1_other)) =>
                        true && _0 == _0_other && _1 == _1_other,
                        (Self::__Ignore(_0, _1),
                         Self::__Ignore(_0_other, _1_other)) =>
                        true && _0 == _0_other && _1 == _1_other,
                        (Self::OwnerSet { .. }, Self::__Ignore { .. }) =>
                        false,
                        (Self::__Ignore { .. }, Self::OwnerSet { .. }) =>
                        false,
                    }
                }
            }
        };
    const _: () =
        {
            impl <T: Config> core::fmt::Debug for Event<T> {
                fn fmt(&self, fmt: &mut core::fmt::Formatter)
                 -> core::fmt::Result {
                    match *self {
                        Self::OwnerSet(ref _0, ref _1) => {
                            fmt.debug_tuple("Event::OwnerSet").field(&_0).field(&_1).finish()
                        }
                        Self::__Ignore(ref _0, ref _1) => {
                            fmt.debug_tuple("Event::__Ignore").field(&_0).field(&_1).finish()
                        }
                    }
                }
            }
        };
    const _: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate codec as _parity_scale_codec;
            impl <T: Config> _parity_scale_codec::Encode for Event<T> where
             T::AccountId: _parity_scale_codec::Encode,
             T::AccountId: _parity_scale_codec::Encode {
                fn encode_to<__CodecOutputEdqy: _parity_scale_codec::Output +
                             ?Sized>(&self,
                                     __codec_dest_edqy:
                                         &mut __CodecOutputEdqy) {
                    match *self {
                        Event::OwnerSet(ref aa, ref ba) => {
                            __codec_dest_edqy.push_byte(0usize as
                                                            ::core::primitive::u8);
                            _parity_scale_codec::Encode::encode_to(aa,
                                                                   __codec_dest_edqy);
                            _parity_scale_codec::Encode::encode_to(ba,
                                                                   __codec_dest_edqy);
                        }
                        _ => (),
                    }
                }
            }
            impl <T: Config> _parity_scale_codec::EncodeLike for Event<T>
             where T::AccountId: _parity_scale_codec::Encode,
             T::AccountId: _parity_scale_codec::Encode {
            }
        };
    const _: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate codec as _parity_scale_codec;
            impl <T: Config> _parity_scale_codec::Decode for Event<T> where
             T::AccountId: _parity_scale_codec::Decode,
             T::AccountId: _parity_scale_codec::Decode {
                fn decode<__CodecInputEdqy: _parity_scale_codec::Input>(__codec_input_edqy:
                                                                            &mut __CodecInputEdqy)
                 -> ::core::result::Result<Self, _parity_scale_codec::Error> {
                    match __codec_input_edqy.read_byte().map_err(|e|
                                                                     e.chain("Could not decode `Event`, failed to read variant byte"))?
                        {
                        __codec_x_edqy if
                        __codec_x_edqy == 0usize as ::core::primitive::u8 => {
                            ::core::result::Result::Ok(Event::<T>::OwnerSet({
                                                                                let __codec_res_edqy =
                                                                                    <T::AccountId
                                                                                        as
                                                                                        _parity_scale_codec::Decode>::decode(__codec_input_edqy);
                                                                                match __codec_res_edqy
                                                                                    {
                                                                                    ::core::result::Result::Err(e)
                                                                                    =>
                                                                                    return ::core::result::Result::Err(e.chain("Could not decode `Event::OwnerSet.0`")),
                                                                                    ::core::result::Result::Ok(__codec_res_edqy)
                                                                                    =>
                                                                                    __codec_res_edqy,
                                                                                }
                                                                            },
                                                                            {
                                                                                let __codec_res_edqy =
                                                                                    <SerString
                                                                                        as
                                                                                        _parity_scale_codec::Decode>::decode(__codec_input_edqy);
                                                                                match __codec_res_edqy
                                                                                    {
                                                                                    ::core::result::Result::Err(e)
                                                                                    =>
                                                                                    return ::core::result::Result::Err(e.chain("Could not decode `Event::OwnerSet.1`")),
                                                                                    ::core::result::Result::Ok(__codec_res_edqy)
                                                                                    =>
                                                                                    __codec_res_edqy,
                                                                                }
                                                                            }))
                        }
                        _ =>
                        ::core::result::Result::Err("Could not decode `Event`, variant doesn\'t exist".into()),
                    }
                }
            }
        };
    #[doc =
      r"
			Custom [dispatch errors](https://substrate.dev/docs/en/knowledgebase/runtime/errors)
			of this pallet.
			"]
    pub enum Error<T> {

        #[doc(hidden)]
        __Ignore(frame_support::sp_std::marker::PhantomData<(T)>,
                 frame_support::Never),

        #[doc = " Domain already exists."]
        DomainAlreadyExists,

        #[doc = " Domain/Route Combination already exists."]
        PathAlreadyExists,

        #[doc = " Domain name is too large."]
        DomainNameTooLarge,

        #[doc = " Domain does not exist."]
        DomainNotRegistered,

        #[doc = " Domain exists but route is not registered on the domain."]
        RouteNotRegistered,

        #[doc =
          " The source of a call is not authorized to perform the call."]
        CallerNotApproved,

        #[doc =
          " The path does not follow the <domain>:<slash-separated route> convention."]
        MalformedPath,

        #[doc =
          " The domain does not follow the <domain>:<slash-separated route> convention."]
        MalformedDomain,
    }
    impl <T: Config> Pallet<T> {
        #[doc =
          " Return Some(registered path) if registered, None if not registered, or MalformedPath if the given domain and route are invalid"]
        pub fn get_registered_path_info(path: &Path)
         -> Option<RegisteredPath<T>> {
            {
                ::std::io::_print(::core::fmt::Arguments::new_v1(&["in get_registered_path_info, looking for ",
                                                                   "\n"],
                                                                 &match (&path,)
                                                                      {
                                                                      (arg0,)
                                                                      =>
                                                                      [::core::fmt::ArgumentV1::new(arg0,
                                                                                                    ::core::fmt::Display::fmt)],
                                                                  }));
            };
            RegisteredPaths::<T>::get(path.domain_lc.clone(),
                                      path.route_lc.clone())
        }
        #[doc =
          " Return true if registered, false if not (Malformed domains/routes are reported as false)"]
        pub fn is_registered_path(path: &Path) -> bool {
            match Pallet::<T>::get_registered_path_info(path) {
                Some(_) => true,
                _ => false,
            }
        }
        #[doc = " Return true if domain is registered, false if not"]
        pub fn is_registered_domain(path: &Path) -> bool {
            let route: SerString = "".into();
            match RegisteredPaths::<T>::get(path.domain_lc.clone(), route) {
                Some(_) => true,
                None => false,
            }
        }
        #[doc =
          " Return true if approver is registered for path, false if not"]
        pub fn is_registered_approver(path: &Path, approver: &Approver<T>)
         -> bool {
            match RegisteredApprovers::<T>::get(path.domain_lc.clone(),
                                                (path.route_lc.clone(),
                                                 &approver)) {
                Some(_) => true,
                _ => false,
            }
        }
    }
    impl <T: Config> Pallet<T> {
        #[doc = " Register a domain."]
        #[doc =
          " A domain must be registered before routes on that domain can be created."]
        pub fn register_domain(origin: OriginFor<T>, domain: SerString)
         -> DispatchResultWithPostInfo {
            let owner = ensure_signed(origin)?;
            let null_route = SerString::from_str("");
            let path = Path::new(domain, null_route);
            match Pallet::<T>::is_registered_path(&path) {
                false => {
                    let registered_path =
                        RegisteredPath::new(&owner, &path, &None);
                    {
                        ::std::io::_print(::core::fmt::Arguments::new_v1(&["in register_domain, Ok path, registered_path = ",
                                                                           "\n"],
                                                                         &match (&registered_path,)
                                                                              {
                                                                              (arg0,)
                                                                              =>
                                                                              [::core::fmt::ArgumentV1::new(arg0,
                                                                                                            ::core::fmt::Display::fmt)],
                                                                          }));
                    };
                    {
                        ::std::io::_print(::core::fmt::Arguments::new_v1(&["in register_domain, Adding the following K/K to RegisteredPaths = ",
                                                                           ",  ",
                                                                           "\n"],
                                                                         &match (&registered_path.path.domain_lc.clone(),
                                                                                 &registered_path.clone().path.route_lc.clone())
                                                                              {
                                                                              (arg0,
                                                                               arg1)
                                                                              =>
                                                                              [::core::fmt::ArgumentV1::new(arg0,
                                                                                                            ::core::fmt::Display::fmt),
                                                                               ::core::fmt::ArgumentV1::new(arg1,
                                                                                                            ::core::fmt::Display::fmt)],
                                                                          }));
                    };
                    RegisteredPaths::<T>::insert(registered_path.path.domain_lc.clone(),
                                                 registered_path.clone().path.route_lc.clone(),
                                                 registered_path.clone());
                    {
                        ::std::io::_print(::core::fmt::Arguments::new_v1(&["in register_domain, Adding the following K/K to RegisteredApprovers = ",
                                                                           ", (",
                                                                           ", ",
                                                                           ")\n"],
                                                                         &match (&registered_path.clone().path.domain_lc.clone(),
                                                                                 &registered_path.clone().path.route_lc.clone(),
                                                                                 &owner.clone())
                                                                              {
                                                                              (arg0,
                                                                               arg1,
                                                                               arg2)
                                                                              =>
                                                                              [::core::fmt::ArgumentV1::new(arg0,
                                                                                                            ::core::fmt::Display::fmt),
                                                                               ::core::fmt::ArgumentV1::new(arg1,
                                                                                                            ::core::fmt::Display::fmt),
                                                                               ::core::fmt::ArgumentV1::new(arg2,
                                                                                                            ::core::fmt::Debug::fmt)],
                                                                          }));
                    };
                    RegisteredApprovers::<T>::insert(registered_path.clone().path.domain_lc.clone(),
                                                     (registered_path.clone().path.route_lc.clone(),
                                                      owner.clone()), ());
                    Self::deposit_event(Event::OwnerSet(owner.clone(),
                                                        registered_path.clone().path.domain_lc.clone()));
                    Ok(Some(10_000).into())
                }
                true => Err(Error::<T>::DomainAlreadyExists)?,
            }
        }
        #[doc = " Register a contract."]
        #[doc =
          " A domain must be registered before contracts on that domain can be registered."]
        pub fn register_contract(origin: OriginFor<T>, path: SerString,
                                 code_hash: CodeHashOf<T>, code: Option<Code>)
         -> DispatchResultWithPostInfo {
            let caller: AccountIdOf<T> = ensure_signed(origin)?;
            match Path::new_from_path(path.clone()) {
                Ok(path) =>
                match Pallet::<T>::is_registered_domain(&path) {
                    true => {
                        let domain_lc = path.clone().domain_lc;
                        let routes = path.route_lc.split("/");
                        let super_paths =
                            (0..(routes.len() +
                                     1)).map(|x|
                                                 {
                                                     match x {
                                                         0 =>
                                                         Path::new_from_path(SerString::from_str(domain_lc.to_str())).unwrap(),
                                                         _ =>
                                                         Path::new_from_path(SerString::join(<[_]>::into_vec(box
                                                                                                                 [domain_lc.clone(),
                                                                                                                  SerString::join(routes[0..x].to_vec(),
                                                                                                                                  "/")]),
                                                                                             ":")).unwrap(),
                                                     }
                                                 }).rev();
                        let mut is_approved = false;
                        let mut paths_to_potentially_be_approved =
                            Vec::<Path>::new();
                        for super_path in super_paths {
                            if Pallet::<T>::is_registered_approver(&super_path,
                                                                   &caller) {
                                {
                                    ::std::io::_print(::core::fmt::Arguments::new_v1(&["in register_contract, found approved path ",
                                                                                       " for caller ",
                                                                                       "\n"],
                                                                                     &match (&super_path,
                                                                                             &caller)
                                                                                          {
                                                                                          (arg0,
                                                                                           arg1)
                                                                                          =>
                                                                                          [::core::fmt::ArgumentV1::new(arg0,
                                                                                                                        ::core::fmt::Display::fmt),
                                                                                           ::core::fmt::ArgumentV1::new(arg1,
                                                                                                                        ::core::fmt::Debug::fmt)],
                                                                                      }));
                                };
                                is_approved = true;
                                break ;
                            } else {
                                if Pallet::<T>::is_registered_path(&super_path)
                                   {
                                    {
                                        ::std::io::_print(::core::fmt::Arguments::new_v1(&["in register_contract, found a registered path ",
                                                                                           " for whic caller ",
                                                                                           " is not approved\n"],
                                                                                         &match (&super_path,
                                                                                                 &caller)
                                                                                              {
                                                                                              (arg0,
                                                                                               arg1)
                                                                                              =>
                                                                                              [::core::fmt::ArgumentV1::new(arg0,
                                                                                                                            ::core::fmt::Display::fmt),
                                                                                               ::core::fmt::ArgumentV1::new(arg1,
                                                                                                                            ::core::fmt::Debug::fmt)],
                                                                                          }));
                                    };
                                    break ;
                                } else {
                                    {
                                        ::std::io::_print(::core::fmt::Arguments::new_v1(&["in register_contract, found a path ",
                                                                                           " that potentially needs to be approved for caller ",
                                                                                           "\n"],
                                                                                         &match (&super_path,
                                                                                                 &caller)
                                                                                              {
                                                                                              (arg0,
                                                                                               arg1)
                                                                                              =>
                                                                                              [::core::fmt::ArgumentV1::new(arg0,
                                                                                                                            ::core::fmt::Display::fmt),
                                                                                               ::core::fmt::ArgumentV1::new(arg1,
                                                                                                                            ::core::fmt::Debug::fmt)],
                                                                                          }));
                                    };
                                    paths_to_potentially_be_approved.push(super_path.clone());
                                }
                            }
                        }
                        match is_approved {
                            true => {
                                {
                                    ::std::io::_print(::core::fmt::Arguments::new_v1(&["in register_contract, caller ",
                                                                                       " is approved about to approve super paths\n"],
                                                                                     &match (&caller,)
                                                                                          {
                                                                                          (arg0,)
                                                                                          =>
                                                                                          [::core::fmt::ArgumentV1::new(arg0,
                                                                                                                        ::core::fmt::Debug::fmt)],
                                                                                      }));
                                };
                                for ptba in paths_to_potentially_be_approved {
                                    {
                                        ::std::io::_print(::core::fmt::Arguments::new_v1(&["in register_contract, path ",
                                                                                           " to be approved for caller ",
                                                                                           "\n"],
                                                                                         &match (&ptba,
                                                                                                 &caller)
                                                                                              {
                                                                                              (arg0,
                                                                                               arg1)
                                                                                              =>
                                                                                              [::core::fmt::ArgumentV1::new(arg0,
                                                                                                                            ::core::fmt::Display::fmt),
                                                                                               ::core::fmt::ArgumentV1::new(arg1,
                                                                                                                            ::core::fmt::Debug::fmt)],
                                                                                          }));
                                    };
                                    let contract =
                                        match ptba == path {
                                            true =>
                                            Some(Contract::new(&code_hash,
                                                               &code)),
                                            false => None,
                                        };
                                    let registered_path =
                                        RegisteredPath::new(&caller, &ptba,
                                                            &contract);
                                    {
                                        ::std::io::_print(::core::fmt::Arguments::new_v1(&["in register_contract is_approved path, registered_path = ",
                                                                                           "\n"],
                                                                                         &match (&registered_path,)
                                                                                              {
                                                                                              (arg0,)
                                                                                              =>
                                                                                              [::core::fmt::ArgumentV1::new(arg0,
                                                                                                                            ::core::fmt::Display::fmt)],
                                                                                          }));
                                    };
                                    {
                                        ::std::io::_print(::core::fmt::Arguments::new_v1(&["in register_contract is_approved path, Adding the following K/K to RegisteredPaths = ",
                                                                                           ",  ",
                                                                                           "\n"],
                                                                                         &match (&registered_path.path.domain_lc.clone(),
                                                                                                 &registered_path.clone().path.route_lc.clone())
                                                                                              {
                                                                                              (arg0,
                                                                                               arg1)
                                                                                              =>
                                                                                              [::core::fmt::ArgumentV1::new(arg0,
                                                                                                                            ::core::fmt::Display::fmt),
                                                                                               ::core::fmt::ArgumentV1::new(arg1,
                                                                                                                            ::core::fmt::Display::fmt)],
                                                                                          }));
                                    };
                                    RegisteredPaths::<T>::insert(registered_path.path.domain_lc.clone(),
                                                                 registered_path.clone().path.route_lc.clone(),
                                                                 registered_path.clone());
                                    {
                                        ::std::io::_print(::core::fmt::Arguments::new_v1(&["in register_contract is_approved path, Adding the following K/K to RegisteredApprovers = ",
                                                                                           ",  ",
                                                                                           "\n"],
                                                                                         &match (&registered_path.clone().path.route_lc.clone(),
                                                                                                 &caller.clone())
                                                                                              {
                                                                                              (arg0,
                                                                                               arg1)
                                                                                              =>
                                                                                              [::core::fmt::ArgumentV1::new(arg0,
                                                                                                                            ::core::fmt::Display::fmt),
                                                                                               ::core::fmt::ArgumentV1::new(arg1,
                                                                                                                            ::core::fmt::Debug::fmt)],
                                                                                          }));
                                    };
                                    RegisteredApprovers::<T>::insert(registered_path.clone().path.domain_lc,
                                                                     (registered_path.clone().path.route_lc,
                                                                      caller.clone()),
                                                                     ());
                                    Self::deposit_event(Event::OwnerSet(caller.clone(),
                                                                        registered_path.clone().path.domain_lc));
                                }
                                Ok(Some(10_000).into())
                            }
                            false => {
                                {
                                    ::std::io::_print(::core::fmt::Arguments::new_v1(&["in register_contract, caller not approved, path = ",
                                                                                       ", caller = ",
                                                                                       "\n"],
                                                                                     &match (&path,
                                                                                             &caller)
                                                                                          {
                                                                                          (arg0,
                                                                                           arg1)
                                                                                          =>
                                                                                          [::core::fmt::ArgumentV1::new(arg0,
                                                                                                                        ::core::fmt::Display::fmt),
                                                                                           ::core::fmt::ArgumentV1::new(arg1,
                                                                                                                        ::core::fmt::Display::fmt)],
                                                                                      }));
                                };
                                ;
                                Err(Error::<T>::CallerNotApproved)?
                            }
                        }
                    }
                    false => {
                        {
                            ::std::io::_print(::core::fmt::Arguments::new_v1(&["in register_contract, domain not registered, path = ",
                                                                               "\n"],
                                                                             &match (&path,)
                                                                                  {
                                                                                  (arg0,)
                                                                                  =>
                                                                                  [::core::fmt::ArgumentV1::new(arg0,
                                                                                                                ::core::fmt::Display::fmt)],
                                                                              }));
                        };
                        ;
                        Err(Error::<T>::DomainNotRegistered)?
                    }
                },
                Err(_) => {
                    {
                        ::std::io::_print(::core::fmt::Arguments::new_v1(&["in register_contract, malformed domain, path = ",
                                                                           "\n"],
                                                                         &match (&path.clone(),)
                                                                              {
                                                                              (arg0,)
                                                                              =>
                                                                              [::core::fmt::ArgumentV1::new(arg0,
                                                                                                            ::core::fmt::Display::fmt)],
                                                                          }));
                    };
                    ;
                    Err(Error::<T>::MalformedDomain)?
                }
            }
        }
        pub fn add_approver(origin: OriginFor<T>, path: SerString,
                            approver: Approver<T>)
         -> DispatchResultWithPostInfo {
            let caller = ensure_signed(origin)?;
            match Path::new_from_path(path) {
                Ok(path) =>
                match Pallet::<T>::is_registered_path(&path) {
                    true =>
                    match Pallet::<T>::is_registered_approver(&path, &caller)
                        {
                        true => {
                            RegisteredApprovers::<T>::insert(path.domain_lc.clone(),
                                                             (path.route_lc.clone(),
                                                              approver), ());
                            Ok(Some(10_000).into())
                        }
                        false => Err(Error::<T>::CallerNotApproved)?,
                    },
                    false => Err(Error::<T>::DomainNotRegistered)?,
                },
                Err(_) => Err(Error::<T>::MalformedDomain)?,
            }
        }
    }
    impl <T: Config> Pallet<T> {
        #[doc(hidden)]
        pub fn module_constants_metadata()
         -> &'static [frame_support::dispatch::ModuleConstantMetadata] {
            &[{
                  #[allow(non_upper_case_types)]
                  #[allow(non_camel_case_types)]
                  struct MaxDomainByteSizeDefaultByteGetter<T>(frame_support::sp_std::marker::PhantomData<(T)>);
                  impl <T: Config> frame_support::dispatch::DefaultByte for
                   MaxDomainByteSizeDefaultByteGetter<T> {
                      fn default_byte(&self)
                       -> frame_support::sp_std::vec::Vec<u8> {
                          let value =
                              <T::MaxDomainByteSize as
                                  frame_support::traits::Get<u32>>::get();
                          frame_support::codec::Encode::encode(&value)
                      }
                  }
                  unsafe impl <T: Config> Send for
                   MaxDomainByteSizeDefaultByteGetter<T> {
                  }
                  unsafe impl <T: Config> Sync for
                   MaxDomainByteSizeDefaultByteGetter<T> {
                  }
                  frame_support::dispatch::ModuleConstantMetadata{name:
                                                                      frame_support::dispatch::DecodeDifferent::Encode("MaxDomainByteSize"),
                                                                  ty:
                                                                      frame_support::dispatch::DecodeDifferent::Encode("u32"),
                                                                  value:
                                                                      frame_support::dispatch::DecodeDifferent::Encode(frame_support::dispatch::DefaultByteGetter(&MaxDomainByteSizeDefaultByteGetter::<T>(frame_support::sp_std::marker::PhantomData))),
                                                                  documentation:
                                                                      frame_support::dispatch::DecodeDifferent::Encode(&[" Maximum size for a domain name, in bytes"]),}
              },
              {
                  #[allow(non_upper_case_types)]
                  #[allow(non_camel_case_types)]
                  struct MaxRouteByteSizeDefaultByteGetter<T>(frame_support::sp_std::marker::PhantomData<(T)>);
                  impl <T: Config> frame_support::dispatch::DefaultByte for
                   MaxRouteByteSizeDefaultByteGetter<T> {
                      fn default_byte(&self)
                       -> frame_support::sp_std::vec::Vec<u8> {
                          let value =
                              <T::MaxRouteByteSize as
                                  frame_support::traits::Get<u32>>::get();
                          frame_support::codec::Encode::encode(&value)
                      }
                  }
                  unsafe impl <T: Config> Send for
                   MaxRouteByteSizeDefaultByteGetter<T> {
                  }
                  unsafe impl <T: Config> Sync for
                   MaxRouteByteSizeDefaultByteGetter<T> {
                  }
                  frame_support::dispatch::ModuleConstantMetadata{name:
                                                                      frame_support::dispatch::DecodeDifferent::Encode("MaxRouteByteSize"),
                                                                  ty:
                                                                      frame_support::dispatch::DecodeDifferent::Encode("u32"),
                                                                  value:
                                                                      frame_support::dispatch::DecodeDifferent::Encode(frame_support::dispatch::DefaultByteGetter(&MaxRouteByteSizeDefaultByteGetter::<T>(frame_support::sp_std::marker::PhantomData))),
                                                                  documentation:
                                                                      frame_support::dispatch::DecodeDifferent::Encode(&[" Maximum size for a route name, in bytes"]),}
              }]
        }
    }
    impl <T: Config> frame_support::error::ModuleErrorMetadata for Pallet<T> {
        fn metadata() -> &'static [frame_support::error::ErrorMetadata] {
            <Error<T> as
                frame_support::error::ModuleErrorMetadata>::metadata()
        }
    }
    #[doc = r" Type alias to `Pallet`, to be used by `construct_runtime`."]
    #[doc = r""]
    #[doc = r" Generated by `pallet` attribute macro."]
    #[deprecated(note = "use `Pallet` instead")]
    #[allow(dead_code)]
    pub type Module<T> = Pallet<T>;
    impl <T: Config> frame_support::traits::GetPalletVersion for Pallet<T> {
        fn current_version() -> frame_support::traits::PalletVersion {
            frame_support::traits::PalletVersion{major: 0u16,
                                                 minor: 1u8,
                                                 patch: 0u8,}
        }
        fn storage_version() -> Option<frame_support::traits::PalletVersion> {
            let key =
                frame_support::traits::PalletVersion::storage_key::<<T as
                                                                    frame_system::Config>::PalletInfo,
                                                                    Self>().expect("Every active pallet has a name in the runtime; qed");
            frame_support::storage::unhashed::get(&key)
        }
    }
    impl <T: Config> frame_support::traits::OnGenesis for Pallet<T> {
        fn on_genesis() {
            frame_support::traits::PalletVersion{major: 0u16,
                                                 minor: 1u8,
                                                 patch:
                                                     0u8,}.put_into_storage::<<T
                                                                              as
                                                                              frame_system::Config>::PalletInfo,
                                                                              Self>();
        }
    }
    impl <T: Config> frame_support::traits::PalletInfoAccess for Pallet<T> {
        fn index() -> usize {
            <<T as frame_system::Config>::PalletInfo as
                frame_support::traits::PalletInfo>::index::<Self>().expect("Pallet is part of the runtime because pallet `Config` trait is \
						implemented by the runtime")
        }
        fn name() -> &'static str {
            <<T as frame_system::Config>::PalletInfo as
                frame_support::traits::PalletInfo>::name::<Self>().expect("Pallet is part of the runtime because pallet `Config` trait is \
						implemented by the runtime")
        }
    }
    impl <T: Config> frame_support::traits::StorageInfoTrait for Pallet<T> {
        fn storage_info()
         ->
             frame_support::sp_std::vec::Vec<frame_support::traits::StorageInfo> {
            #[allow(unused_mut)]
            let mut res = ::alloc::vec::Vec::new();
            {
                let mut storage_info =
                    <RegisteredPaths<T> as
                        frame_support::traits::PartialStorageInfoTrait>::partial_storage_info();
                res.append(&mut storage_info);
            }
            {
                let mut storage_info =
                    <RegisteredApprovers<T> as
                        frame_support::traits::PartialStorageInfoTrait>::partial_storage_info();
                res.append(&mut storage_info);
            }
            res
        }
    }
    #[doc(hidden)]
    pub mod __substrate_call_check {
        #[macro_export]
        #[doc(hidden)]
        macro_rules! __is_call_part_defined_0 {
            ($ pallet_name : ident) => { } ;
        }
        #[doc(hidden)]
        pub use __is_call_part_defined_0 as is_call_part_defined;
    }
    #[doc =
      r"Contains one variant per dispatchable that can be called by an extrinsic."]
    #[codec(encode_bound())]
    #[codec(decode_bound())]
    #[allow(non_camel_case_types)]
    pub enum Call<T: Config> {

        #[doc(hidden)]
        #[codec(skip)]
        __Ignore(frame_support::sp_std::marker::PhantomData<(T,)>,
                 frame_support::Never),

        #[doc = " Register a domain."]
        #[doc =
          " A domain must be registered before routes on that domain can be created."]
        register_domain(SerString),

        #[doc = " Register a contract."]
        #[doc =
          " A domain must be registered before contracts on that domain can be registered."]
        register_contract(SerString, CodeHashOf<T>, Option<Code>),
        add_approver(SerString, Approver<T>),
    }
    const _: () =
        {
            impl <T: Config> core::fmt::Debug for Call<T> {
                fn fmt(&self, fmt: &mut core::fmt::Formatter)
                 -> core::fmt::Result {
                    match *self {
                        Self::__Ignore(ref _0, ref _1) => {
                            fmt.debug_tuple("Call::__Ignore").field(&_0).field(&_1).finish()
                        }
                        Self::register_domain(ref _0) => {
                            fmt.debug_tuple("Call::register_domain").field(&_0).finish()
                        }
                        Self::register_contract(ref _0, ref _1, ref _2) => {
                            fmt.debug_tuple("Call::register_contract").field(&_0).field(&_1).field(&_2).finish()
                        }
                        Self::add_approver(ref _0, ref _1) => {
                            fmt.debug_tuple("Call::add_approver").field(&_0).field(&_1).finish()
                        }
                    }
                }
            }
        };
    const _: () =
        {
            impl <T: Config> core::clone::Clone for Call<T> {
                fn clone(&self) -> Self {
                    match self {
                        Self::__Ignore(ref _0, ref _1) =>
                        Self::__Ignore(core::clone::Clone::clone(_0),
                                       core::clone::Clone::clone(_1)),
                        Self::register_domain(ref _0) =>
                        Self::register_domain(core::clone::Clone::clone(_0)),
                        Self::register_contract(ref _0, ref _1, ref _2) =>
                        Self::register_contract(core::clone::Clone::clone(_0),
                                                core::clone::Clone::clone(_1),
                                                core::clone::Clone::clone(_2)),
                        Self::add_approver(ref _0, ref _1) =>
                        Self::add_approver(core::clone::Clone::clone(_0),
                                           core::clone::Clone::clone(_1)),
                    }
                }
            }
        };
    const _: () =
        {
            impl <T: Config> core::cmp::Eq for Call<T> { }
        };
    const _: () =
        {
            impl <T: Config> core::cmp::PartialEq for Call<T> {
                fn eq(&self, other: &Self) -> bool {
                    match (self, other) {
                        (Self::__Ignore(_0, _1),
                         Self::__Ignore(_0_other, _1_other)) =>
                        true && _0 == _0_other && _1 == _1_other,
                        (Self::register_domain(_0),
                         Self::register_domain(_0_other)) =>
                        true && _0 == _0_other,
                        (Self::register_contract(_0, _1, _2),
                         Self::register_contract(_0_other, _1_other,
                                                 _2_other)) =>
                        true && _0 == _0_other && _1 == _1_other &&
                            _2 == _2_other,
                        (Self::add_approver(_0, _1),
                         Self::add_approver(_0_other, _1_other)) =>
                        true && _0 == _0_other && _1 == _1_other,
                        (Self::__Ignore { .. }, Self::register_domain { .. })
                        => false,
                        (Self::__Ignore { .. }, Self::register_contract { ..
                         }) => false,
                        (Self::__Ignore { .. }, Self::add_approver { .. }) =>
                        false,
                        (Self::register_domain { .. }, Self::__Ignore { .. })
                        => false,
                        (Self::register_domain { .. },
                         Self::register_contract { .. }) => false,
                        (Self::register_domain { .. }, Self::add_approver { ..
                         }) => false,
                        (Self::register_contract { .. }, Self::__Ignore { ..
                         }) => false,
                        (Self::register_contract { .. },
                         Self::register_domain { .. }) => false,
                        (Self::register_contract { .. }, Self::add_approver {
                         .. }) => false,
                        (Self::add_approver { .. }, Self::__Ignore { .. }) =>
                        false,
                        (Self::add_approver { .. }, Self::register_domain { ..
                         }) => false,
                        (Self::add_approver { .. }, Self::register_contract {
                         .. }) => false,
                    }
                }
            }
        };
    const _: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate codec as _parity_scale_codec;
            #[allow(non_camel_case_types)]
            impl <T: Config> _parity_scale_codec::Encode for Call<T> {
                fn encode_to<__CodecOutputEdqy: _parity_scale_codec::Output +
                             ?Sized>(&self,
                                     __codec_dest_edqy:
                                         &mut __CodecOutputEdqy) {
                    match *self {
                        Call::register_domain(ref aa) => {
                            __codec_dest_edqy.push_byte(0usize as
                                                            ::core::primitive::u8);
                            _parity_scale_codec::Encode::encode_to(aa,
                                                                   __codec_dest_edqy);
                        }
                        Call::register_contract(ref aa, ref ba, ref ca) => {
                            __codec_dest_edqy.push_byte(1usize as
                                                            ::core::primitive::u8);
                            _parity_scale_codec::Encode::encode_to(aa,
                                                                   __codec_dest_edqy);
                            _parity_scale_codec::Encode::encode_to(ba,
                                                                   __codec_dest_edqy);
                            _parity_scale_codec::Encode::encode_to(ca,
                                                                   __codec_dest_edqy);
                        }
                        Call::add_approver(ref aa, ref ba) => {
                            __codec_dest_edqy.push_byte(2usize as
                                                            ::core::primitive::u8);
                            _parity_scale_codec::Encode::encode_to(aa,
                                                                   __codec_dest_edqy);
                            _parity_scale_codec::Encode::encode_to(ba,
                                                                   __codec_dest_edqy);
                        }
                        _ => (),
                    }
                }
            }
            impl <T: Config> _parity_scale_codec::EncodeLike for Call<T> { }
        };
    const _: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate codec as _parity_scale_codec;
            #[allow(non_camel_case_types)]
            impl <T: Config> _parity_scale_codec::Decode for Call<T> {
                fn decode<__CodecInputEdqy: _parity_scale_codec::Input>(__codec_input_edqy:
                                                                            &mut __CodecInputEdqy)
                 -> ::core::result::Result<Self, _parity_scale_codec::Error> {
                    match __codec_input_edqy.read_byte().map_err(|e|
                                                                     e.chain("Could not decode `Call`, failed to read variant byte"))?
                        {
                        __codec_x_edqy if
                        __codec_x_edqy == 0usize as ::core::primitive::u8 => {
                            ::core::result::Result::Ok(Call::<T>::register_domain({
                                                                                      let __codec_res_edqy =
                                                                                          <SerString
                                                                                              as
                                                                                              _parity_scale_codec::Decode>::decode(__codec_input_edqy);
                                                                                      match __codec_res_edqy
                                                                                          {
                                                                                          ::core::result::Result::Err(e)
                                                                                          =>
                                                                                          return ::core::result::Result::Err(e.chain("Could not decode `Call::register_domain.0`")),
                                                                                          ::core::result::Result::Ok(__codec_res_edqy)
                                                                                          =>
                                                                                          __codec_res_edqy,
                                                                                      }
                                                                                  }))
                        }
                        __codec_x_edqy if
                        __codec_x_edqy == 1usize as ::core::primitive::u8 => {
                            ::core::result::Result::Ok(Call::<T>::register_contract({
                                                                                        let __codec_res_edqy =
                                                                                            <SerString
                                                                                                as
                                                                                                _parity_scale_codec::Decode>::decode(__codec_input_edqy);
                                                                                        match __codec_res_edqy
                                                                                            {
                                                                                            ::core::result::Result::Err(e)
                                                                                            =>
                                                                                            return ::core::result::Result::Err(e.chain("Could not decode `Call::register_contract.0`")),
                                                                                            ::core::result::Result::Ok(__codec_res_edqy)
                                                                                            =>
                                                                                            __codec_res_edqy,
                                                                                        }
                                                                                    },
                                                                                    {
                                                                                        let __codec_res_edqy =
                                                                                            <CodeHashOf<T>
                                                                                                as
                                                                                                _parity_scale_codec::Decode>::decode(__codec_input_edqy);
                                                                                        match __codec_res_edqy
                                                                                            {
                                                                                            ::core::result::Result::Err(e)
                                                                                            =>
                                                                                            return ::core::result::Result::Err(e.chain("Could not decode `Call::register_contract.1`")),
                                                                                            ::core::result::Result::Ok(__codec_res_edqy)
                                                                                            =>
                                                                                            __codec_res_edqy,
                                                                                        }
                                                                                    },
                                                                                    {
                                                                                        let __codec_res_edqy =
                                                                                            <Option<Code>
                                                                                                as
                                                                                                _parity_scale_codec::Decode>::decode(__codec_input_edqy);
                                                                                        match __codec_res_edqy
                                                                                            {
                                                                                            ::core::result::Result::Err(e)
                                                                                            =>
                                                                                            return ::core::result::Result::Err(e.chain("Could not decode `Call::register_contract.2`")),
                                                                                            ::core::result::Result::Ok(__codec_res_edqy)
                                                                                            =>
                                                                                            __codec_res_edqy,
                                                                                        }
                                                                                    }))
                        }
                        __codec_x_edqy if
                        __codec_x_edqy == 2usize as ::core::primitive::u8 => {
                            ::core::result::Result::Ok(Call::<T>::add_approver({
                                                                                   let __codec_res_edqy =
                                                                                       <SerString
                                                                                           as
                                                                                           _parity_scale_codec::Decode>::decode(__codec_input_edqy);
                                                                                   match __codec_res_edqy
                                                                                       {
                                                                                       ::core::result::Result::Err(e)
                                                                                       =>
                                                                                       return ::core::result::Result::Err(e.chain("Could not decode `Call::add_approver.0`")),
                                                                                       ::core::result::Result::Ok(__codec_res_edqy)
                                                                                       =>
                                                                                       __codec_res_edqy,
                                                                                   }
                                                                               },
                                                                               {
                                                                                   let __codec_res_edqy =
                                                                                       <Approver<T>
                                                                                           as
                                                                                           _parity_scale_codec::Decode>::decode(__codec_input_edqy);
                                                                                   match __codec_res_edqy
                                                                                       {
                                                                                       ::core::result::Result::Err(e)
                                                                                       =>
                                                                                       return ::core::result::Result::Err(e.chain("Could not decode `Call::add_approver.1`")),
                                                                                       ::core::result::Result::Ok(__codec_res_edqy)
                                                                                       =>
                                                                                       __codec_res_edqy,
                                                                                   }
                                                                               }))
                        }
                        _ =>
                        ::core::result::Result::Err("Could not decode `Call`, variant doesn\'t exist".into()),
                    }
                }
            }
        };
    impl <T: Config> frame_support::dispatch::GetDispatchInfo for Call<T> {
        fn get_dispatch_info(&self) -> frame_support::dispatch::DispatchInfo {
            match *self {
                Self::register_domain(ref domain) => {
                    let __pallet_base_weight =
                        10_000 + T::DbWeight::get().writes(1);
                    let __pallet_weight =
                        <dyn frame_support::dispatch::WeighData<(&SerString,)>>::weigh_data(&__pallet_base_weight,
                                                                                            (domain,));
                    let __pallet_class =
                        <dyn frame_support::dispatch::ClassifyDispatch<(&SerString,)>>::classify_dispatch(&__pallet_base_weight,
                                                                                                          (domain,));
                    let __pallet_pays_fee =
                        <dyn frame_support::dispatch::PaysFee<(&SerString,)>>::pays_fee(&__pallet_base_weight,
                                                                                        (domain,));
                    frame_support::dispatch::DispatchInfo{weight:
                                                              __pallet_weight,
                                                          class:
                                                              __pallet_class,
                                                          pays_fee:
                                                              __pallet_pays_fee,}
                }
                Self::register_contract(ref path, ref code_hash, ref code) =>
                {
                    let __pallet_base_weight =
                        10_000 + T::DbWeight::get().writes(1);
                    let __pallet_weight =
                        <dyn frame_support::dispatch::WeighData<(&SerString,
                                                                 &CodeHashOf<T>,
                                                                 &Option<Code>)>>::weigh_data(&__pallet_base_weight,
                                                                                              (path,
                                                                                               code_hash,
                                                                                               code));
                    let __pallet_class =
                        <dyn frame_support::dispatch::ClassifyDispatch<(&SerString,
                                                                        &CodeHashOf<T>,
                                                                        &Option<Code>)>>::classify_dispatch(&__pallet_base_weight,
                                                                                                            (path,
                                                                                                             code_hash,
                                                                                                             code));
                    let __pallet_pays_fee =
                        <dyn frame_support::dispatch::PaysFee<(&SerString,
                                                               &CodeHashOf<T>,
                                                               &Option<Code>)>>::pays_fee(&__pallet_base_weight,
                                                                                          (path,
                                                                                           code_hash,
                                                                                           code));
                    frame_support::dispatch::DispatchInfo{weight:
                                                              __pallet_weight,
                                                          class:
                                                              __pallet_class,
                                                          pays_fee:
                                                              __pallet_pays_fee,}
                }
                Self::add_approver(ref path, ref approver) => {
                    let __pallet_base_weight =
                        10_000 + T::DbWeight::get().writes(1);
                    let __pallet_weight =
                        <dyn frame_support::dispatch::WeighData<(&SerString,
                                                                 &Approver<T>)>>::weigh_data(&__pallet_base_weight,
                                                                                             (path,
                                                                                              approver));
                    let __pallet_class =
                        <dyn frame_support::dispatch::ClassifyDispatch<(&SerString,
                                                                        &Approver<T>)>>::classify_dispatch(&__pallet_base_weight,
                                                                                                           (path,
                                                                                                            approver));
                    let __pallet_pays_fee =
                        <dyn frame_support::dispatch::PaysFee<(&SerString,
                                                               &Approver<T>)>>::pays_fee(&__pallet_base_weight,
                                                                                         (path,
                                                                                          approver));
                    frame_support::dispatch::DispatchInfo{weight:
                                                              __pallet_weight,
                                                          class:
                                                              __pallet_class,
                                                          pays_fee:
                                                              __pallet_pays_fee,}
                }
                Self::__Ignore(_, _) => {
                    {
                        ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(&["internal error: entered unreachable code: "],
                                                                                    &match (&"__Ignore cannot be used",)
                                                                                         {
                                                                                         (arg0,)
                                                                                         =>
                                                                                         [::core::fmt::ArgumentV1::new(arg0,
                                                                                                                       ::core::fmt::Display::fmt)],
                                                                                     }))
                    }
                }
            }
        }
    }
    impl <T: Config> frame_support::dispatch::GetCallName for Call<T> {
        fn get_call_name(&self) -> &'static str {
            match *self {
                Self::register_domain(..) => "register_domain",
                Self::register_contract(..) => "register_contract",
                Self::add_approver(..) => "add_approver",
                Self::__Ignore(_, _) => {
                    {
                        ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(&["internal error: entered unreachable code: "],
                                                                                    &match (&"__PhantomItem cannot be used.",)
                                                                                         {
                                                                                         (arg0,)
                                                                                         =>
                                                                                         [::core::fmt::ArgumentV1::new(arg0,
                                                                                                                       ::core::fmt::Display::fmt)],
                                                                                     }))
                    }
                }
            }
        }
        fn get_call_names() -> &'static [&'static str] {
            &["register_domain", "register_contract", "add_approver"]
        }
    }
    impl <T: Config> frame_support::traits::UnfilteredDispatchable for Call<T>
     {
        type Origin = frame_system::pallet_prelude::OriginFor<T>;
        fn dispatch_bypass_filter(self, origin: Self::Origin)
         -> frame_support::dispatch::DispatchResultWithPostInfo {
            match self {
                Self::register_domain(domain) => {
                    let __within_span__ =
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static CALLSITE:
                             ::tracing::__macro_support::MacroCallsite =
                                {
                                    use ::tracing::__macro_support::MacroCallsite;
                                    static META: ::tracing::Metadata<'static>
                                     =
                                        {
                                            ::tracing_core::metadata::Metadata::new("register_domain",
                                                                                    "compose::pallet",
                                                                                    ::tracing::Level::TRACE,
                                                                                    Some("pallets/compose/src/lib.rs"),
                                                                                    Some(187u32),
                                                                                    Some("compose::pallet"),
                                                                                    ::tracing_core::field::FieldSet::new(&[],
                                                                                                                         ::tracing_core::callsite::Identifier(&CALLSITE)),
                                                                                    ::tracing::metadata::Kind::SPAN)
                                        };
                                    MacroCallsite::new(&META)
                                };
                            let mut interest =
                                ::tracing::subscriber::Interest::never();
                            if ::tracing::Level::TRACE <=
                                   ::tracing::level_filters::STATIC_MAX_LEVEL
                                   &&
                                   ::tracing::Level::TRACE <=
                                       ::tracing::level_filters::LevelFilter::current()
                                   &&
                                   {
                                       interest = CALLSITE.interest();
                                       !interest.is_never()
                                   } && CALLSITE.is_enabled(interest) {
                                let meta = CALLSITE.metadata();
                                ::tracing::Span::new(meta,
                                                     &{
                                                          meta.fields().value_set(&[])
                                                      })
                            } else {
                                let span = CALLSITE.disabled_span();
                                { };
                                span
                            }
                        };
                    let __tracing_guard__ = __within_span__.enter();
                    <Pallet<T>>::register_domain(origin,
                                                 domain).map(Into::into).map_err(Into::into)
                }
                Self::register_contract(path, code_hash, code) => {
                    let __within_span__ =
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static CALLSITE:
                             ::tracing::__macro_support::MacroCallsite =
                                {
                                    use ::tracing::__macro_support::MacroCallsite;
                                    static META: ::tracing::Metadata<'static>
                                     =
                                        {
                                            ::tracing_core::metadata::Metadata::new("register_contract",
                                                                                    "compose::pallet",
                                                                                    ::tracing::Level::TRACE,
                                                                                    Some("pallets/compose/src/lib.rs"),
                                                                                    Some(187u32),
                                                                                    Some("compose::pallet"),
                                                                                    ::tracing_core::field::FieldSet::new(&[],
                                                                                                                         ::tracing_core::callsite::Identifier(&CALLSITE)),
                                                                                    ::tracing::metadata::Kind::SPAN)
                                        };
                                    MacroCallsite::new(&META)
                                };
                            let mut interest =
                                ::tracing::subscriber::Interest::never();
                            if ::tracing::Level::TRACE <=
                                   ::tracing::level_filters::STATIC_MAX_LEVEL
                                   &&
                                   ::tracing::Level::TRACE <=
                                       ::tracing::level_filters::LevelFilter::current()
                                   &&
                                   {
                                       interest = CALLSITE.interest();
                                       !interest.is_never()
                                   } && CALLSITE.is_enabled(interest) {
                                let meta = CALLSITE.metadata();
                                ::tracing::Span::new(meta,
                                                     &{
                                                          meta.fields().value_set(&[])
                                                      })
                            } else {
                                let span = CALLSITE.disabled_span();
                                { };
                                span
                            }
                        };
                    let __tracing_guard__ = __within_span__.enter();
                    <Pallet<T>>::register_contract(origin, path, code_hash,
                                                   code).map(Into::into).map_err(Into::into)
                }
                Self::add_approver(path, approver) => {
                    let __within_span__ =
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static CALLSITE:
                             ::tracing::__macro_support::MacroCallsite =
                                {
                                    use ::tracing::__macro_support::MacroCallsite;
                                    static META: ::tracing::Metadata<'static>
                                     =
                                        {
                                            ::tracing_core::metadata::Metadata::new("add_approver",
                                                                                    "compose::pallet",
                                                                                    ::tracing::Level::TRACE,
                                                                                    Some("pallets/compose/src/lib.rs"),
                                                                                    Some(187u32),
                                                                                    Some("compose::pallet"),
                                                                                    ::tracing_core::field::FieldSet::new(&[],
                                                                                                                         ::tracing_core::callsite::Identifier(&CALLSITE)),
                                                                                    ::tracing::metadata::Kind::SPAN)
                                        };
                                    MacroCallsite::new(&META)
                                };
                            let mut interest =
                                ::tracing::subscriber::Interest::never();
                            if ::tracing::Level::TRACE <=
                                   ::tracing::level_filters::STATIC_MAX_LEVEL
                                   &&
                                   ::tracing::Level::TRACE <=
                                       ::tracing::level_filters::LevelFilter::current()
                                   &&
                                   {
                                       interest = CALLSITE.interest();
                                       !interest.is_never()
                                   } && CALLSITE.is_enabled(interest) {
                                let meta = CALLSITE.metadata();
                                ::tracing::Span::new(meta,
                                                     &{
                                                          meta.fields().value_set(&[])
                                                      })
                            } else {
                                let span = CALLSITE.disabled_span();
                                { };
                                span
                            }
                        };
                    let __tracing_guard__ = __within_span__.enter();
                    <Pallet<T>>::add_approver(origin, path,
                                              approver).map(Into::into).map_err(Into::into)
                }
                Self::__Ignore(_, _) => {
                    let _ = origin;
                    {
                        {
                            ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(&["internal error: entered unreachable code: "],
                                                                                        &match (&"__PhantomItem cannot be used.",)
                                                                                             {
                                                                                             (arg0,)
                                                                                             =>
                                                                                             [::core::fmt::ArgumentV1::new(arg0,
                                                                                                                           ::core::fmt::Display::fmt)],
                                                                                         }))
                        }
                    };
                }
            }
        }
    }
    impl <T: Config> frame_support::dispatch::Callable<T> for Pallet<T> {
        type Call = Call<T>;
    }
    impl <T: Config> Pallet<T> {
        #[doc(hidden)]
        #[allow(dead_code)]
        pub fn call_functions()
         -> &'static [frame_support::dispatch::FunctionMetadata] {
            &[frame_support::dispatch::FunctionMetadata{name:
                                                            frame_support::dispatch::DecodeDifferent::Encode("register_domain"),
                                                        arguments:
                                                            frame_support::dispatch::DecodeDifferent::Encode(&[frame_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                     frame_support::dispatch::DecodeDifferent::Encode("domain"),
                                                                                                                                                                 ty:
                                                                                                                                                                     frame_support::dispatch::DecodeDifferent::Encode("SerString"),}]),
                                                        documentation:
                                                            frame_support::dispatch::DecodeDifferent::Encode(&[" Register a domain.",
                                                                                                               " A domain must be registered before routes on that domain can be created."]),},
              frame_support::dispatch::FunctionMetadata{name:
                                                            frame_support::dispatch::DecodeDifferent::Encode("register_contract"),
                                                        arguments:
                                                            frame_support::dispatch::DecodeDifferent::Encode(&[frame_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                     frame_support::dispatch::DecodeDifferent::Encode("path"),
                                                                                                                                                                 ty:
                                                                                                                                                                     frame_support::dispatch::DecodeDifferent::Encode("SerString"),},
                                                                                                               frame_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                     frame_support::dispatch::DecodeDifferent::Encode("code_hash"),
                                                                                                                                                                 ty:
                                                                                                                                                                     frame_support::dispatch::DecodeDifferent::Encode("CodeHashOf<T>"),},
                                                                                                               frame_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                     frame_support::dispatch::DecodeDifferent::Encode("code"),
                                                                                                                                                                 ty:
                                                                                                                                                                     frame_support::dispatch::DecodeDifferent::Encode("Option<Code>"),}]),
                                                        documentation:
                                                            frame_support::dispatch::DecodeDifferent::Encode(&[" Register a contract.",
                                                                                                               " A domain must be registered before contracts on that domain can be registered."]),},
              frame_support::dispatch::FunctionMetadata{name:
                                                            frame_support::dispatch::DecodeDifferent::Encode("add_approver"),
                                                        arguments:
                                                            frame_support::dispatch::DecodeDifferent::Encode(&[frame_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                     frame_support::dispatch::DecodeDifferent::Encode("path"),
                                                                                                                                                                 ty:
                                                                                                                                                                     frame_support::dispatch::DecodeDifferent::Encode("SerString"),},
                                                                                                               frame_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                     frame_support::dispatch::DecodeDifferent::Encode("approver"),
                                                                                                                                                                 ty:
                                                                                                                                                                     frame_support::dispatch::DecodeDifferent::Encode("Approver<T>"),}]),
                                                        documentation:
                                                            frame_support::dispatch::DecodeDifferent::Encode(&[]),}]
        }
    }
    impl <T: Config> frame_support::sp_std::fmt::Debug for Error<T> {
        fn fmt(&self, f: &mut frame_support::sp_std::fmt::Formatter<'_>)
         -> frame_support::sp_std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl <T: Config> Error<T> {
        pub fn as_u8(&self) -> u8 {
            match &self {
                Self::__Ignore(_, _) => {
                    {
                        ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(&["internal error: entered unreachable code: "],
                                                                                    &match (&"`__Ignore` can never be constructed",)
                                                                                         {
                                                                                         (arg0,)
                                                                                         =>
                                                                                         [::core::fmt::ArgumentV1::new(arg0,
                                                                                                                       ::core::fmt::Display::fmt)],
                                                                                     }))
                    }
                }
                Self::DomainAlreadyExists => 0usize as u8,
                Self::PathAlreadyExists => 1usize as u8,
                Self::DomainNameTooLarge => 2usize as u8,
                Self::DomainNotRegistered => 3usize as u8,
                Self::RouteNotRegistered => 4usize as u8,
                Self::CallerNotApproved => 5usize as u8,
                Self::MalformedPath => 6usize as u8,
                Self::MalformedDomain => 7usize as u8,
            }
        }
        pub fn as_str(&self) -> &'static str {
            match &self {
                Self::__Ignore(_, _) => {
                    {
                        ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(&["internal error: entered unreachable code: "],
                                                                                    &match (&"`__Ignore` can never be constructed",)
                                                                                         {
                                                                                         (arg0,)
                                                                                         =>
                                                                                         [::core::fmt::ArgumentV1::new(arg0,
                                                                                                                       ::core::fmt::Display::fmt)],
                                                                                     }))
                    }
                }
                Self::DomainAlreadyExists => "DomainAlreadyExists",
                Self::PathAlreadyExists => "PathAlreadyExists",
                Self::DomainNameTooLarge => "DomainNameTooLarge",
                Self::DomainNotRegistered => "DomainNotRegistered",
                Self::RouteNotRegistered => "RouteNotRegistered",
                Self::CallerNotApproved => "CallerNotApproved",
                Self::MalformedPath => "MalformedPath",
                Self::MalformedDomain => "MalformedDomain",
            }
        }
    }
    impl <T: Config> From<Error<T>> for &'static str {
        fn from(err: Error<T>) -> &'static str { err.as_str() }
    }
    impl <T: Config> From<Error<T>> for
     frame_support::sp_runtime::DispatchError {
        fn from(err: Error<T>) -> Self {
            let index =
                <<T as frame_system::Config>::PalletInfo as
                    frame_support::traits::PalletInfo>::index::<Pallet<T>>().expect("Every active module has an index in the runtime; qed")
                    as u8;
            frame_support::sp_runtime::DispatchError::Module{index,
                                                             error:
                                                                 err.as_u8(),
                                                             message:
                                                                 Some(err.as_str()),}
        }
    }
    impl <T: Config> frame_support::error::ModuleErrorMetadata for Error<T> {
        fn metadata() -> &'static [frame_support::error::ErrorMetadata] {
            &[frame_support::error::ErrorMetadata{name:
                                                      frame_support::error::DecodeDifferent::Encode("DomainAlreadyExists"),
                                                  documentation:
                                                      frame_support::error::DecodeDifferent::Encode(&[" Domain already exists."]),},
              frame_support::error::ErrorMetadata{name:
                                                      frame_support::error::DecodeDifferent::Encode("PathAlreadyExists"),
                                                  documentation:
                                                      frame_support::error::DecodeDifferent::Encode(&[" Domain/Route Combination already exists."]),},
              frame_support::error::ErrorMetadata{name:
                                                      frame_support::error::DecodeDifferent::Encode("DomainNameTooLarge"),
                                                  documentation:
                                                      frame_support::error::DecodeDifferent::Encode(&[" Domain name is too large."]),},
              frame_support::error::ErrorMetadata{name:
                                                      frame_support::error::DecodeDifferent::Encode("DomainNotRegistered"),
                                                  documentation:
                                                      frame_support::error::DecodeDifferent::Encode(&[" Domain does not exist."]),},
              frame_support::error::ErrorMetadata{name:
                                                      frame_support::error::DecodeDifferent::Encode("RouteNotRegistered"),
                                                  documentation:
                                                      frame_support::error::DecodeDifferent::Encode(&[" Domain exists but route is not registered on the domain."]),},
              frame_support::error::ErrorMetadata{name:
                                                      frame_support::error::DecodeDifferent::Encode("CallerNotApproved"),
                                                  documentation:
                                                      frame_support::error::DecodeDifferent::Encode(&[" The source of a call is not authorized to perform the call."]),},
              frame_support::error::ErrorMetadata{name:
                                                      frame_support::error::DecodeDifferent::Encode("MalformedPath"),
                                                  documentation:
                                                      frame_support::error::DecodeDifferent::Encode(&[" The path does not follow the <domain>:<slash-separated route> convention."]),},
              frame_support::error::ErrorMetadata{name:
                                                      frame_support::error::DecodeDifferent::Encode("MalformedDomain"),
                                                  documentation:
                                                      frame_support::error::DecodeDifferent::Encode(&[" The domain does not follow the <domain>:<slash-separated route> convention."]),}]
        }
    }
    #[doc(hidden)]
    pub mod __substrate_event_check {
        #[macro_export]
        #[doc(hidden)]
        macro_rules! __is_event_part_defined_1 {
            ($ pallet_name : ident) => { } ;
        }
        #[doc(hidden)]
        pub use __is_event_part_defined_1 as is_event_part_defined;
    }
    impl <T: Config> Pallet<T> {
        pub(super) fn deposit_event(event: Event<T>) {
            let event = <<T as Config>::Event as From<Event<T>>>::from(event);
            let event =
                <<T as Config>::Event as
                    Into<<T as frame_system::Config>::Event>>::into(event);
            <frame_system::Pallet<T>>::deposit_event(event)
        }
    }
    impl <T: Config> From<Event<T>> for () {
        fn from(_: Event<T>) { }
    }
    impl <T: Config> Event<T> {
        #[allow(dead_code)]
        #[doc(hidden)]
        pub fn metadata() -> &'static [frame_support::event::EventMetadata] {
            &[frame_support::event::EventMetadata{name:
                                                      frame_support::event::DecodeDifferent::Encode("OwnerSet"),
                                                  arguments:
                                                      frame_support::event::DecodeDifferent::Encode(&["AccountId",
                                                                                                      "SerString"]),
                                                  documentation:
                                                      frame_support::event::DecodeDifferent::Encode(&[" An owner was set or reset. \\[who, path\\]"]),}]
        }
    }
    impl <T: Config> Pallet<T> {
        #[doc(hidden)]
        pub fn storage_metadata()
         -> frame_support::metadata::StorageMetadata {
            frame_support::metadata::StorageMetadata{prefix:
                                                         frame_support::metadata::DecodeDifferent::Encode(<<T
                                                                                                           as
                                                                                                           frame_system::Config>::PalletInfo
                                                                                                              as
                                                                                                              frame_support::traits::PalletInfo>::name::<Pallet<T>>().expect("Every active pallet has a name in the runtime; qed")),
                                                     entries:
                                                         frame_support::metadata::DecodeDifferent::Encode(&[frame_support::metadata::StorageEntryMetadata{name:
                                                                                                                                                              frame_support::metadata::DecodeDifferent::Encode(<RegisteredPaths<T>
                                                                                                                                                                                                                   as
                                                                                                                                                                                                                   frame_support::storage::types::StorageDoubleMapMetadata>::NAME),
                                                                                                                                                          modifier:
                                                                                                                                                              <RegisteredPaths<T>
                                                                                                                                                                  as
                                                                                                                                                                  frame_support::storage::types::StorageDoubleMapMetadata>::MODIFIER,
                                                                                                                                                          ty:
                                                                                                                                                              frame_support::metadata::StorageEntryType::DoubleMap{hasher:
                                                                                                                                                                                                                       <RegisteredPaths<T>
                                                                                                                                                                                                                           as
                                                                                                                                                                                                                           frame_support::storage::types::StorageDoubleMapMetadata>::HASHER1,
                                                                                                                                                                                                                   key2_hasher:
                                                                                                                                                                                                                       <RegisteredPaths<T>
                                                                                                                                                                                                                           as
                                                                                                                                                                                                                           frame_support::storage::types::StorageDoubleMapMetadata>::HASHER2,
                                                                                                                                                                                                                   key1:
                                                                                                                                                                                                                       frame_support::metadata::DecodeDifferent::Encode("SerString"),
                                                                                                                                                                                                                   key2:
                                                                                                                                                                                                                       frame_support::metadata::DecodeDifferent::Encode("SerString"),
                                                                                                                                                                                                                   value:
                                                                                                                                                                                                                       frame_support::metadata::DecodeDifferent::Encode("RegisteredPath<T>"),},
                                                                                                                                                          default:
                                                                                                                                                              frame_support::metadata::DecodeDifferent::Encode(<RegisteredPaths<T>
                                                                                                                                                                                                                   as
                                                                                                                                                                                                                   frame_support::storage::types::StorageDoubleMapMetadata>::DEFAULT),
                                                                                                                                                          documentation:
                                                                                                                                                              frame_support::metadata::DecodeDifferent::Encode(&[" A table of registered paths (i.e., domain/route combinations).  A domain itself will always be registered with a route of None; ",
                                                                                                                                                                                                                 " all other registered paths will have a Some<SerString> route string",
                                                                                                                                                                                                                 " \\[domain, route, path/owner registration info\\]"]),},
                                                                                                            frame_support::metadata::StorageEntryMetadata{name:
                                                                                                                                                              frame_support::metadata::DecodeDifferent::Encode(<RegisteredApprovers<T>
                                                                                                                                                                                                                   as
                                                                                                                                                                                                                   frame_support::storage::types::StorageDoubleMapMetadata>::NAME),
                                                                                                                                                          modifier:
                                                                                                                                                              <RegisteredApprovers<T>
                                                                                                                                                                  as
                                                                                                                                                                  frame_support::storage::types::StorageDoubleMapMetadata>::MODIFIER,
                                                                                                                                                          ty:
                                                                                                                                                              frame_support::metadata::StorageEntryType::DoubleMap{hasher:
                                                                                                                                                                                                                       <RegisteredApprovers<T>
                                                                                                                                                                                                                           as
                                                                                                                                                                                                                           frame_support::storage::types::StorageDoubleMapMetadata>::HASHER1,
                                                                                                                                                                                                                   key2_hasher:
                                                                                                                                                                                                                       <RegisteredApprovers<T>
                                                                                                                                                                                                                           as
                                                                                                                                                                                                                           frame_support::storage::types::StorageDoubleMapMetadata>::HASHER2,
                                                                                                                                                                                                                   key1:
                                                                                                                                                                                                                       frame_support::metadata::DecodeDifferent::Encode("SerString"),
                                                                                                                                                                                                                   key2:
                                                                                                                                                                                                                       frame_support::metadata::DecodeDifferent::Encode("(SerString, Approver<T>)"),
                                                                                                                                                                                                                   value:
                                                                                                                                                                                                                       frame_support::metadata::DecodeDifferent::Encode("()"),},
                                                                                                                                                          default:
                                                                                                                                                              frame_support::metadata::DecodeDifferent::Encode(<RegisteredApprovers<T>
                                                                                                                                                                                                                   as
                                                                                                                                                                                                                   frame_support::storage::types::StorageDoubleMapMetadata>::DEFAULT),
                                                                                                                                                          documentation:
                                                                                                                                                              frame_support::metadata::DecodeDifferent::Encode(&[" A table of registered approvers for a given path, i.e., account(s) (if any) that have been approved by the path owner to also manage the path.",
                                                                                                                                                                                                                 " \\[domain, (route, approver)\\]"]),}]),}
        }
    }
    pub struct _GeneratedPrefixForStorageRegisteredPaths<T>(core::marker::PhantomData<(T,)>);
    impl <T: Config> frame_support::traits::StorageInstance for
     _GeneratedPrefixForStorageRegisteredPaths<T> {
        fn pallet_prefix() -> &'static str {
            <<T as frame_system::Config>::PalletInfo as
                frame_support::traits::PalletInfo>::name::<Pallet<T>>().expect("Every active pallet has a name in the runtime; qed")
        }
        const STORAGE_PREFIX: &'static str = "RegisteredPaths";
    }
    pub struct _GeneratedPrefixForStorageRegisteredApprovers<T>(core::marker::PhantomData<(T,)>);
    impl <T: Config> frame_support::traits::StorageInstance for
     _GeneratedPrefixForStorageRegisteredApprovers<T> {
        fn pallet_prefix() -> &'static str {
            <<T as frame_system::Config>::PalletInfo as
                frame_support::traits::PalletInfo>::name::<Pallet<T>>().expect("Every active pallet has a name in the runtime; qed")
        }
        const STORAGE_PREFIX: &'static str = "RegisteredApprovers";
    }
    #[doc(hidden)]
    pub mod __substrate_inherent_check {
        #[macro_export]
        #[doc(hidden)]
        macro_rules! __is_inherent_part_defined_2 {
            ($ pallet_name : ident) =>
            {
                compile_error!
                (concat!
                 ("`", stringify! ($ pallet_name),
                  "` does not have #[pallet::inherent] defined, perhaps you should \
				remove `Inherent` from construct_runtime?",))
                ;
            }
        }
        #[doc(hidden)]
        pub use __is_inherent_part_defined_2 as is_inherent_part_defined;
    }
    #[doc =
      r" Hidden instance generated to be internally used when module is used without"]
    #[doc = r" instance."]
    #[doc(hidden)]
    pub type __InherentHiddenInstance = ();
    pub(super) trait Store {
        type RegisteredPaths;
        type RegisteredApprovers;
    }
    impl <T: Config> Store for Pallet<T> {
        type RegisteredPaths = RegisteredPaths<T>;
        type RegisteredApprovers = RegisteredApprovers<T>;
    }
    impl <T: Config>
     frame_support::traits::Hooks<<T as frame_system::Config>::BlockNumber>
     for Pallet<T> {
    }
    impl <T: Config>
     frame_support::traits::OnFinalize<<T as
                                       frame_system::Config>::BlockNumber> for
     Pallet<T> {
        fn on_finalize(n: <T as frame_system::Config>::BlockNumber) {
            let __within_span__ =
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static CALLSITE: ::tracing::__macro_support::MacroCallsite
                     =
                        {
                            use ::tracing::__macro_support::MacroCallsite;
                            static META: ::tracing::Metadata<'static> =
                                {
                                    ::tracing_core::metadata::Metadata::new("on_finalize",
                                                                            "compose::pallet",
                                                                            ::tracing::Level::TRACE,
                                                                            Some("pallets/compose/src/lib.rs"),
                                                                            Some(187u32),
                                                                            Some("compose::pallet"),
                                                                            ::tracing_core::field::FieldSet::new(&[],
                                                                                                                 ::tracing_core::callsite::Identifier(&CALLSITE)),
                                                                            ::tracing::metadata::Kind::SPAN)
                                };
                            MacroCallsite::new(&META)
                        };
                    let mut interest =
                        ::tracing::subscriber::Interest::never();
                    if ::tracing::Level::TRACE <=
                           ::tracing::level_filters::STATIC_MAX_LEVEL &&
                           ::tracing::Level::TRACE <=
                               ::tracing::level_filters::LevelFilter::current()
                           &&
                           {
                               interest = CALLSITE.interest();
                               !interest.is_never()
                           } && CALLSITE.is_enabled(interest) {
                        let meta = CALLSITE.metadata();
                        ::tracing::Span::new(meta,
                                             &{
                                                  meta.fields().value_set(&[])
                                              })
                    } else { let span = CALLSITE.disabled_span(); { }; span }
                };
            let __tracing_guard__ = __within_span__.enter();
            <Self as
                frame_support::traits::Hooks<<T as
                                             frame_system::Config>::BlockNumber>>::on_finalize(n)
        }
    }
    impl <T: Config>
     frame_support::traits::OnIdle<<T as frame_system::Config>::BlockNumber>
     for Pallet<T> {
        fn on_idle(n: <T as frame_system::Config>::BlockNumber,
                   remaining_weight: frame_support::weights::Weight)
         -> frame_support::weights::Weight {
            <Self as
                frame_support::traits::Hooks<<T as
                                             frame_system::Config>::BlockNumber>>::on_idle(n,
                                                                                           remaining_weight)
        }
    }
    impl <T: Config>
     frame_support::traits::OnInitialize<<T as
                                         frame_system::Config>::BlockNumber>
     for Pallet<T> {
        fn on_initialize(n: <T as frame_system::Config>::BlockNumber)
         -> frame_support::weights::Weight {
            let __within_span__ =
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static CALLSITE: ::tracing::__macro_support::MacroCallsite
                     =
                        {
                            use ::tracing::__macro_support::MacroCallsite;
                            static META: ::tracing::Metadata<'static> =
                                {
                                    ::tracing_core::metadata::Metadata::new("on_initialize",
                                                                            "compose::pallet",
                                                                            ::tracing::Level::TRACE,
                                                                            Some("pallets/compose/src/lib.rs"),
                                                                            Some(187u32),
                                                                            Some("compose::pallet"),
                                                                            ::tracing_core::field::FieldSet::new(&[],
                                                                                                                 ::tracing_core::callsite::Identifier(&CALLSITE)),
                                                                            ::tracing::metadata::Kind::SPAN)
                                };
                            MacroCallsite::new(&META)
                        };
                    let mut interest =
                        ::tracing::subscriber::Interest::never();
                    if ::tracing::Level::TRACE <=
                           ::tracing::level_filters::STATIC_MAX_LEVEL &&
                           ::tracing::Level::TRACE <=
                               ::tracing::level_filters::LevelFilter::current()
                           &&
                           {
                               interest = CALLSITE.interest();
                               !interest.is_never()
                           } && CALLSITE.is_enabled(interest) {
                        let meta = CALLSITE.metadata();
                        ::tracing::Span::new(meta,
                                             &{
                                                  meta.fields().value_set(&[])
                                              })
                    } else { let span = CALLSITE.disabled_span(); { }; span }
                };
            let __tracing_guard__ = __within_span__.enter();
            <Self as
                frame_support::traits::Hooks<<T as
                                             frame_system::Config>::BlockNumber>>::on_initialize(n)
        }
    }
    impl <T: Config> frame_support::traits::OnRuntimeUpgrade for Pallet<T> {
        fn on_runtime_upgrade() -> frame_support::weights::Weight {
            let __within_span__ =
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static CALLSITE: ::tracing::__macro_support::MacroCallsite
                     =
                        {
                            use ::tracing::__macro_support::MacroCallsite;
                            static META: ::tracing::Metadata<'static> =
                                {
                                    ::tracing_core::metadata::Metadata::new("on_runtime_update",
                                                                            "compose::pallet",
                                                                            ::tracing::Level::TRACE,
                                                                            Some("pallets/compose/src/lib.rs"),
                                                                            Some(187u32),
                                                                            Some("compose::pallet"),
                                                                            ::tracing_core::field::FieldSet::new(&[],
                                                                                                                 ::tracing_core::callsite::Identifier(&CALLSITE)),
                                                                            ::tracing::metadata::Kind::SPAN)
                                };
                            MacroCallsite::new(&META)
                        };
                    let mut interest =
                        ::tracing::subscriber::Interest::never();
                    if ::tracing::Level::TRACE <=
                           ::tracing::level_filters::STATIC_MAX_LEVEL &&
                           ::tracing::Level::TRACE <=
                               ::tracing::level_filters::LevelFilter::current()
                           &&
                           {
                               interest = CALLSITE.interest();
                               !interest.is_never()
                           } && CALLSITE.is_enabled(interest) {
                        let meta = CALLSITE.metadata();
                        ::tracing::Span::new(meta,
                                             &{
                                                  meta.fields().value_set(&[])
                                              })
                    } else { let span = CALLSITE.disabled_span(); { }; span }
                };
            let __tracing_guard__ = __within_span__.enter();
            let new_storage_version =
                frame_support::traits::PalletVersion{major: 0u16,
                                                     minor: 1u8,
                                                     patch: 0u8,};
            let pallet_name =
                <<T as frame_system::Config>::PalletInfo as
                    frame_support::traits::PalletInfo>::name::<Self>().unwrap_or("<unknown pallet name>");
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level()
                   {
                    ::log::__private_api_log(::core::fmt::Arguments::new_v1(&["\u{2705} no migration for ",
                                                                              ", setting storage version to "],
                                                                            &match (&pallet_name,
                                                                                    &new_storage_version)
                                                                                 {
                                                                                 (arg0,
                                                                                  arg1)
                                                                                 =>
                                                                                 [::core::fmt::ArgumentV1::new(arg0,
                                                                                                               ::core::fmt::Display::fmt),
                                                                                  ::core::fmt::ArgumentV1::new(arg1,
                                                                                                               ::core::fmt::Debug::fmt)],
                                                                             }),
                                             lvl,
                                             &(frame_support::LOG_TARGET,
                                               "compose::pallet",
                                               "pallets/compose/src/lib.rs",
                                               187u32));
                }
            };
            let result =
                <Self as
                    frame_support::traits::Hooks<<T as
                                                 frame_system::Config>::BlockNumber>>::on_runtime_upgrade();
            new_storage_version.put_into_storage::<<T as
                                                   frame_system::Config>::PalletInfo,
                                                   Self>();
            let additional_write =
                <<T as frame_system::Config>::DbWeight as
                    frame_support::traits::Get<_>>::get().writes(1);
            result.saturating_add(additional_write)
        }
    }
    impl <T: Config>
     frame_support::traits::OffchainWorker<<T as
                                           frame_system::Config>::BlockNumber>
     for Pallet<T> {
        fn offchain_worker(n: <T as frame_system::Config>::BlockNumber) {
            <Self as
                frame_support::traits::Hooks<<T as
                                             frame_system::Config>::BlockNumber>>::offchain_worker(n)
        }
    }
    impl <T: Config> frame_support::traits::IntegrityTest for Pallet<T> {
        fn integrity_test() {
            <Self as
                frame_support::traits::Hooks<<T as
                                             frame_system::Config>::BlockNumber>>::integrity_test()
        }
    }
    #[doc(hidden)]
    pub mod __substrate_genesis_config_check {
        #[macro_export]
        #[doc(hidden)]
        macro_rules! __is_genesis_config_defined_3 {
            ($ pallet_name : ident) =>
            {
                compile_error!
                (concat!
                 ("`", stringify! ($ pallet_name),
                  "` does not have #[pallet::genesis_config] defined, perhaps you should \
								remove `Config` from construct_runtime?",))
                ;
            }
        }
        #[macro_export]
        #[doc(hidden)]
        macro_rules! __is_std_enabled_for_genesis_3 {
            ($ pallet_name : ident, $ pallet_path : expr) => { } ;
        }
        #[doc(hidden)]
        pub use __is_genesis_config_defined_3 as is_genesis_config_defined;
        #[doc(hidden)]
        pub use __is_std_enabled_for_genesis_3 as is_std_enabled_for_genesis;
    }
    #[doc(hidden)]
    pub mod __substrate_origin_check {
        #[macro_export]
        #[doc(hidden)]
        macro_rules! __is_origin_part_defined_4 {
            ($ pallet_name : ident) =>
            {
                compile_error!
                (concat!
                 ("`", stringify! ($ pallet_name),
                  "` does not have #[pallet::origin] defined, perhaps you should \
				remove `Origin` from construct_runtime?",))
                ;
            }
        }
        #[doc(hidden)]
        pub use __is_origin_part_defined_4 as is_origin_part_defined;
    }
    #[doc(hidden)]
    pub mod __substrate_validate_unsigned_check {
        #[macro_export]
        #[doc(hidden)]
        macro_rules! __is_validate_unsigned_part_defined_5 {
            ($ pallet_name : ident) =>
            {
                compile_error!
                (concat!
                 ("`", stringify! ($ pallet_name),
                  "` does not have #[pallet::validate_unsigned] defined, perhaps you should \
				remove `ValidateUnsigned` from construct_runtime?",))
                ;
            }
        }
        #[doc(hidden)]
        pub use __is_validate_unsigned_part_defined_5 as
                is_validate_unsigned_part_defined;
    }
}
