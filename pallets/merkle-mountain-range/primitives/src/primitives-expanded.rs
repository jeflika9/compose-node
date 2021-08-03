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

//! Merkle Mountain Range primitive types.

#![warn(missing_docs)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;

use frame_support::RuntimeDebug;
use sp_runtime::traits::{self, Saturating, One};
use sp_std::fmt;

/// A provider of the MMR's leaf data.
pub trait LeafDataProvider {
    /// A type that should end up in the leaf of MMR.
    type LeafData: FullLeaf + codec::Decode;

    /// The method to return leaf data that should be placed
    /// in the leaf node appended MMR at this block.
    ///
    /// This is being called by the `on_initialize` method of
    /// this pallet at the very beginning of each block.
    fn leaf_data()
    -> Self::LeafData;
}

impl LeafDataProvider for () {
    type LeafData = ();

    fn leaf_data() -> Self::LeafData { () }
}

/// The most common use case for MMRs is to store historical block hashes,
/// so that any point in time in the future we can receive a proof about some past
/// blocks without using excessive on-chain storage.
///
/// Hence we implement the [LeafDataProvider] for [frame_system::Pallet]. Since the
/// current block hash is not available (since the block is not finished yet),
/// we use the `parent_hash` here along with parent block number.
impl <T: frame_system::Config> LeafDataProvider for frame_system::Pallet<T> {
    type LeafData =
     (<T as frame_system::Config>::BlockNumber,
      <T as frame_system::Config>::Hash);

    fn leaf_data() -> Self::LeafData {
        (Self::block_number().saturating_sub(One::one()), Self::parent_hash())
    }
}

/// New MMR root notification hook.
pub trait OnNewRoot<Hash> {
    /// Function called by the pallet in case new MMR root has been computed.
    fn on_new_root(root: &Hash);
}

/// No-op implementation of [OnNewRoot].
impl <Hash> OnNewRoot<Hash> for () {
    fn on_new_root(_root: &Hash) { }
}

/// A full leaf content stored in the offchain-db.
pub trait FullLeaf: Clone + PartialEq + fmt::Debug {
    /// Encode the leaf either in it's full or compact form.
    ///
    /// NOTE the encoding returned here MUST be `Decode`able into `FullLeaf`.
    fn using_encoded<R, F: FnOnce(&[u8]) -> R>(&self, f: F, compact: bool)
    -> R;
}

impl <T: codec::Encode + codec::Decode + Clone + PartialEq + fmt::Debug>
 FullLeaf for T {
    fn using_encoded<R, F: FnOnce(&[u8]) -> R>(&self, f: F, _compact: bool)
     -> R {
        codec::Encode::using_encoded(self, f)
    }
}

/// An element representing either full data or it's hash.
///
/// See [Compact] to see how it may be used in practice to reduce the size
/// of proofs in case multiple [LeafDataProvider]s are composed together.
/// This is also used internally by the MMR to differentiate leaf nodes (data)
/// and inner nodes (hashes).
///
/// [DataOrHash::hash] method calculates the hash of this element in it's compact form,
/// so should be used instead of hashing the encoded form (which will always be non-compact).
pub enum DataOrHash<H: traits::Hash, L> {

    /// Arbitrary data in it's full form.
    Data(L),

    /// A hash of some data.
    Hash(H::Output),
}
impl <H: traits::Hash, L> core::fmt::Debug for DataOrHash<H, L> where
 H: core::fmt::Debug, L: core::fmt::Debug {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Data(ref a0) =>
            fmt.debug_tuple("DataOrHash::Data").field(a0).finish(),
            Self::Hash(ref a0) =>
            fmt.debug_tuple("DataOrHash::Hash").field(a0).finish(),
            _ => Ok(()),
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <H: ::core::clone::Clone + traits::Hash, L: ::core::clone::Clone>
 ::core::clone::Clone for DataOrHash<H, L> where
 H::Output: ::core::clone::Clone {
    #[inline]
    fn clone(&self) -> DataOrHash<H, L> {
        match (&*self,) {
            (&DataOrHash::Data(ref __self_0),) =>
            DataOrHash::Data(::core::clone::Clone::clone(&(*__self_0))),
            (&DataOrHash::Hash(ref __self_0),) =>
            DataOrHash::Hash(::core::clone::Clone::clone(&(*__self_0))),
        }
    }
}
impl <H: traits::Hash, L> ::core::marker::StructuralPartialEq for
 DataOrHash<H, L> {
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <H: ::core::cmp::PartialEq + traits::Hash, L: ::core::cmp::PartialEq>
 ::core::cmp::PartialEq for DataOrHash<H, L> where
 H::Output: ::core::cmp::PartialEq {
    #[inline]
    fn eq(&self, other: &DataOrHash<H, L>) -> bool {
        {
            let __self_vi = ::core::intrinsics::discriminant_value(&*self);
            let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&DataOrHash::Data(ref __self_0),
                     &DataOrHash::Data(ref __arg_1_0)) =>
                    (*__self_0) == (*__arg_1_0),
                    (&DataOrHash::Hash(ref __self_0),
                     &DataOrHash::Hash(ref __arg_1_0)) =>
                    (*__self_0) == (*__arg_1_0),
                    _ => unsafe { ::core::intrinsics::unreachable() }
                }
            } else { false }
        }
    }
    #[inline]
    fn ne(&self, other: &DataOrHash<H, L>) -> bool {
        {
            let __self_vi = ::core::intrinsics::discriminant_value(&*self);
            let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&DataOrHash::Data(ref __self_0),
                     &DataOrHash::Data(ref __arg_1_0)) =>
                    (*__self_0) != (*__arg_1_0),
                    (&DataOrHash::Hash(ref __self_0),
                     &DataOrHash::Hash(ref __arg_1_0)) =>
                    (*__self_0) != (*__arg_1_0),
                    _ => unsafe { ::core::intrinsics::unreachable() }
                }
            } else { true }
        }
    }
}

impl <H: traits::Hash, L> From<L> for DataOrHash<H, L> {
    fn from(l: L) -> Self { Self::Data(l) }
}

mod encoding {
    use super::*;

    /// A helper type to implement [codec::Codec] for [DataOrHash].
    enum Either<A, B> { Left(A), Right(B), }
    const _: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate codec as _parity_scale_codec;
            impl <A, B> _parity_scale_codec::Encode for Either<A, B> where
             A: _parity_scale_codec::Encode, A: _parity_scale_codec::Encode,
             B: _parity_scale_codec::Encode, B: _parity_scale_codec::Encode {
                fn encode_to<__CodecOutputEdqy: _parity_scale_codec::Output +
                             ?Sized>(&self,
                                     __codec_dest_edqy:
                                         &mut __CodecOutputEdqy) {
                    match *self {
                        Either::Left(ref aa) => {
                            __codec_dest_edqy.push_byte(0usize as
                                                            ::core::primitive::u8);
                            _parity_scale_codec::Encode::encode_to(aa,
                                                                   __codec_dest_edqy);
                        }
                        Either::Right(ref aa) => {
                            __codec_dest_edqy.push_byte(1usize as
                                                            ::core::primitive::u8);
                            _parity_scale_codec::Encode::encode_to(aa,
                                                                   __codec_dest_edqy);
                        }
                        _ => (),
                    }
                }
            }
            impl <A, B> _parity_scale_codec::EncodeLike for Either<A, B> where
             A: _parity_scale_codec::Encode, A: _parity_scale_codec::Encode,
             B: _parity_scale_codec::Encode, B: _parity_scale_codec::Encode {
            }
        };
    const _: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate codec as _parity_scale_codec;
            impl <A, B> _parity_scale_codec::Decode for Either<A, B> where
             A: _parity_scale_codec::Decode, A: _parity_scale_codec::Decode,
             B: _parity_scale_codec::Decode, B: _parity_scale_codec::Decode {
                fn decode<__CodecInputEdqy: _parity_scale_codec::Input>(__codec_input_edqy:
                                                                            &mut __CodecInputEdqy)
                 -> ::core::result::Result<Self, _parity_scale_codec::Error> {
                    match __codec_input_edqy.read_byte().map_err(|e|
                                                                     e.chain("Could not decode `Either`, failed to read variant byte"))?
                        {
                        __codec_x_edqy if
                        __codec_x_edqy == 0usize as ::core::primitive::u8 => {
                            ::core::result::Result::Ok(Either::<A,
                                                                B>::Left({
                                                                             let __codec_res_edqy =
                                                                                 <A
                                                                                     as
                                                                                     _parity_scale_codec::Decode>::decode(__codec_input_edqy);
                                                                             match __codec_res_edqy
                                                                                 {
                                                                                 ::core::result::Result::Err(e)
                                                                                 =>
                                                                                 return ::core::result::Result::Err(e.chain("Could not decode `Either::Left.0`")),
                                                                                 ::core::result::Result::Ok(__codec_res_edqy)
                                                                                 =>
                                                                                 __codec_res_edqy,
                                                                             }
                                                                         }))
                        }
                        __codec_x_edqy if
                        __codec_x_edqy == 1usize as ::core::primitive::u8 => {
                            ::core::result::Result::Ok(Either::<A,
                                                                B>::Right({
                                                                              let __codec_res_edqy =
                                                                                  <B
                                                                                      as
                                                                                      _parity_scale_codec::Decode>::decode(__codec_input_edqy);
                                                                              match __codec_res_edqy
                                                                                  {
                                                                                  ::core::result::Result::Err(e)
                                                                                  =>
                                                                                  return ::core::result::Result::Err(e.chain("Could not decode `Either::Right.0`")),
                                                                                  ::core::result::Result::Ok(__codec_res_edqy)
                                                                                  =>
                                                                                  __codec_res_edqy,
                                                                              }
                                                                          }))
                        }
                        _ =>
                        ::core::result::Result::Err("Could not decode `Either`, variant doesn\'t exist".into()),
                    }
                }
            }
        };

    impl <H: traits::Hash, L: FullLeaf> codec::Encode for DataOrHash<H, L> {
        fn encode_to<T: codec::Output + ?Sized>(&self, dest: &mut T) {
            match self {
                Self::Data(l) =>
                l.using_encoded(|data|
                                    Either::<&[u8],
                                             &H::Output>::Left(data).encode_to(dest),
                                false),
                Self::Hash(h) =>
                Either::<&[u8], &H::Output>::Right(h).encode_to(dest),
            }
        }
    }

    impl <H: traits::Hash, L: FullLeaf + codec::Decode> codec::Decode for
     DataOrHash<H, L> {
        fn decode<I: codec::Input>(value: &mut I)
         -> Result<Self, codec::Error> {
            let decoded: Either<Vec<u8>, H::Output> = Either::decode(value)?;
            Ok(match decoded {
                   Either::Left(l) => DataOrHash::Data(L::decode(&mut &*l)?),
                   Either::Right(r) => DataOrHash::Hash(r),
               })
        }
    }
}

impl <H: traits::Hash, L: FullLeaf> DataOrHash<H, L> {
    /// Retrieve a hash of this item.
    ///
    /// Depending on the node type it's going to either be a contained value for [DataOrHash::Hash]
    /// node, or a hash of SCALE-encoded [DataOrHash::Data] data.
    pub fn hash(&self) -> H::Output {
        match *self {
            Self::Data(ref leaf) =>
            leaf.using_encoded(<H as traits::Hash>::hash, true),
            Self::Hash(ref hash) => hash.clone(),
        }
    }
}

/// A composition of multiple leaf elements with compact form representation.
///
/// When composing together multiple [LeafDataProvider]s you will end up with
/// a tuple of `LeafData` that each element provides.
///
/// However this will cause the leaves to have significant size, while for some
/// use cases it will be enough to prove only one element of the tuple.
/// That's the rationale for [Compact] struct. We wrap each element of the tuple
/// into [DataOrHash] and each tuple element is hashed first before constructing
/// the final hash of the entire tuple. This allows you to replace tuple elements
/// you don't care about with their hashes.
pub struct Compact<H, T> {
    /// Internal tuple representation.
    pub tuple: T,
    _hash: sp_std::marker::PhantomData<H>,
}
impl <H, T> core::fmt::Debug for Compact<H, T> where H: core::fmt::Debug,
 T: core::fmt::Debug {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        fmt.debug_struct("Compact").field("tuple",
                                          &self.tuple).field("_hash",
                                                             &self._hash).finish()
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <H: ::core::clone::Clone, T: ::core::clone::Clone> ::core::clone::Clone
 for Compact<H, T> {
    #[inline]
    fn clone(&self) -> Compact<H, T> {
        match *self {
            Compact { tuple: ref __self_0_0, _hash: ref __self_0_1 } =>
            Compact{tuple: ::core::clone::Clone::clone(&(*__self_0_0)),
                    _hash: ::core::clone::Clone::clone(&(*__self_0_1)),},
        }
    }
}
impl <H, T> ::core::marker::StructuralPartialEq for Compact<H, T> { }
#[automatically_derived]
#[allow(unused_qualifications)]
impl <H: ::core::cmp::PartialEq, T: ::core::cmp::PartialEq>
 ::core::cmp::PartialEq for Compact<H, T> {
    #[inline]
    fn eq(&self, other: &Compact<H, T>) -> bool {
        match *other {
            Compact { tuple: ref __self_1_0, _hash: ref __self_1_1 } =>
            match *self {
                Compact { tuple: ref __self_0_0, _hash: ref __self_0_1 } =>
                (*__self_0_0) == (*__self_1_0) &&
                    (*__self_0_1) == (*__self_1_1),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &Compact<H, T>) -> bool {
        match *other {
            Compact { tuple: ref __self_1_0, _hash: ref __self_1_1 } =>
            match *self {
                Compact { tuple: ref __self_0_0, _hash: ref __self_0_1 } =>
                (*__self_0_0) != (*__self_1_0) ||
                    (*__self_0_1) != (*__self_1_1),
            },
        }
    }
}

impl <H, T> sp_std::ops::Deref for Compact<H, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target { &self.tuple }
}

impl <H, T> Compact<H, T> {
    /// Create a new [Compact] wrapper for a tuple.
    pub fn new(tuple: T) -> Self { Self{tuple, _hash: Default::default(),} }
}

impl <H, T: codec::Decode> codec::Decode for Compact<H, T> {
    fn decode<I: codec::Input>(value: &mut I) -> Result<Self, codec::Error> {
        T::decode(value).map(Compact::new)
    }
}

macro_rules! impl_leaf_data_for_tuple {
    ($ ($ name : ident : $ id : tt), +) =>
    {
        /// [FullLeaf] implementation for `Compact<H, (DataOrHash<H, Tuple>, ...)>`
        impl < H, $ ($ name), + > FullLeaf for Compact < H,
        ($ (DataOrHash < H, $ name >,) +) > where H : traits :: Hash, $
        ($ name : FullLeaf), +
        {
            fn using_encoded < R, F : FnOnce(& [u8]) -> R >
            (& self, f : F, compact : bool) -> R
            {
                if compact
                {
                    codec :: Encode ::
                    using_encoded(&
                                  ($
                                   (DataOrHash :: < H, $ name > ::
                                    Hash(self.tuple.$ id.hash()),) +), f)
                } else { codec :: Encode :: using_encoded(& self.tuple, f) }
            }
        }
        /// [LeafDataProvider] implementation for `Compact<H, (DataOrHash<H, Tuple>, ...)>`
        ///
        /// This provides a compact-form encoding for tuples wrapped in [Compact].
        impl < H, $ ($ name), + > LeafDataProvider for Compact < H,
        ($ ($ name,) +) > where H : traits :: Hash, $
        ($ name : LeafDataProvider), +
        {
            type LeafData = Compact < H,
            ($ (DataOrHash < H, $ name :: LeafData >,) +), > ; fn leaf_data()
            -> Self :: LeafData
            {
                let tuple = ($ (DataOrHash :: Data($ name :: leaf_data()),) +)
                ; Compact :: new(tuple)


            }
        } /// [LeafDataProvider] implementation for `(Tuple, ...)`
        ///
        /// This provides regular (non-compactable) composition of [LeafDataProvider]s.
        impl < $ ($ name), + > LeafDataProvider for($ ($ name,) +)
        where($ ($ name :: LeafData,) +) : FullLeaf, $
        ($ name : LeafDataProvider), +
        {
            type LeafData = ($ ($ name :: LeafData,) +) ; fn leaf_data() ->
            Self :: LeafData
            {
                ($ ($ name :: leaf_data(),) +)


            }
        }
    }
}
/// [FullLeaf] implementation for `Compact<H, (DataOrHash<H, Tuple>, ...)>`
impl <H, A> FullLeaf for Compact<H, (DataOrHash<H, A>,)> where
 H: traits::Hash, A: FullLeaf {
    fn using_encoded<R, F: FnOnce(&[u8]) -> R>(&self, f: F, compact: bool)
     -> R {
        if compact {
            codec::Encode::using_encoded(&(DataOrHash::<H,
                                                        A>::Hash(self.tuple.0.hash()),),
                                         f)
        } else { codec::Encode::using_encoded(&self.tuple, f) }
    }
}
/// [LeafDataProvider] implementation for `Compact<H, (DataOrHash<H, Tuple>, ...)>`
///
/// This provides a compact-form encoding for tuples wrapped in [Compact].
impl <H, A> LeafDataProvider for Compact<H, (A,)> where H: traits::Hash,
 A: LeafDataProvider {
    type LeafData = Compact<H, (DataOrHash<H, A::LeafData>,)>;
    fn leaf_data() -> Self::LeafData {
        let tuple = (DataOrHash::Data(A::leaf_data()),);
        Compact::new(tuple)
    }
}
/// [LeafDataProvider] implementation for `(Tuple, ...)`
///
/// This provides regular (non-compactable) composition of [LeafDataProvider]s.
impl <A> LeafDataProvider for (A,) where (A::LeafData,): FullLeaf,
 A: LeafDataProvider {
    type LeafData = (A::LeafData,);
    fn leaf_data() -> Self::LeafData { (A::leaf_data(),) }
}
/// [FullLeaf] implementation for `Compact<H, (DataOrHash<H, Tuple>, ...)>`
impl <H, A, B> FullLeaf for Compact<H, (DataOrHash<H, A>, DataOrHash<H, B>)>
 where H: traits::Hash, A: FullLeaf, B: FullLeaf {
    fn using_encoded<R, F: FnOnce(&[u8]) -> R>(&self, f: F, compact: bool)
     -> R {
        if compact {
            codec::Encode::using_encoded(&(DataOrHash::<H,
                                                        A>::Hash(self.tuple.0.hash()),
                                           DataOrHash::<H,
                                                        B>::Hash(self.tuple.1.hash())),
                                         f)
        } else { codec::Encode::using_encoded(&self.tuple, f) }
    }
}
/// [LeafDataProvider] implementation for `Compact<H, (DataOrHash<H, Tuple>, ...)>`
///
/// This provides a compact-form encoding for tuples wrapped in [Compact].
impl <H, A, B> LeafDataProvider for Compact<H, (A, B)> where H: traits::Hash,
 A: LeafDataProvider, B: LeafDataProvider {
    type LeafData =
     Compact<H, (DataOrHash<H, A::LeafData>, DataOrHash<H, B::LeafData>)>;
    fn leaf_data() -> Self::LeafData {
        let tuple =
            (DataOrHash::Data(A::leaf_data()),
             DataOrHash::Data(B::leaf_data()));
        Compact::new(tuple)
    }
}
/// [LeafDataProvider] implementation for `(Tuple, ...)`
///
/// This provides regular (non-compactable) composition of [LeafDataProvider]s.
impl <A, B> LeafDataProvider for (A, B) where
 (A::LeafData, B::LeafData): FullLeaf, A: LeafDataProvider,
 B: LeafDataProvider {
    type LeafData = (A::LeafData, B::LeafData);
    fn leaf_data() -> Self::LeafData { (A::leaf_data(), B::leaf_data()) }
}
/// [FullLeaf] implementation for `Compact<H, (DataOrHash<H, Tuple>, ...)>`
impl <H, A, B, C> FullLeaf for
 Compact<H, (DataOrHash<H, A>, DataOrHash<H, B>, DataOrHash<H, C>)> where
 H: traits::Hash, A: FullLeaf, B: FullLeaf, C: FullLeaf {
    fn using_encoded<R, F: FnOnce(&[u8]) -> R>(&self, f: F, compact: bool)
     -> R {
        if compact {
            codec::Encode::using_encoded(&(DataOrHash::<H,
                                                        A>::Hash(self.tuple.0.hash()),
                                           DataOrHash::<H,
                                                        B>::Hash(self.tuple.1.hash()),
                                           DataOrHash::<H,
                                                        C>::Hash(self.tuple.2.hash())),
                                         f)
        } else { codec::Encode::using_encoded(&self.tuple, f) }
    }
}
/// [LeafDataProvider] implementation for `Compact<H, (DataOrHash<H, Tuple>, ...)>`
///
/// This provides a compact-form encoding for tuples wrapped in [Compact].
impl <H, A, B, C> LeafDataProvider for Compact<H, (A, B, C)> where
 H: traits::Hash, A: LeafDataProvider, B: LeafDataProvider,
 C: LeafDataProvider {
    type LeafData =
     Compact<H,
             (DataOrHash<H, A::LeafData>, DataOrHash<H, B::LeafData>,
              DataOrHash<H, C::LeafData>)>;
    fn leaf_data() -> Self::LeafData {
        let tuple =
            (DataOrHash::Data(A::leaf_data()),
             DataOrHash::Data(B::leaf_data()),
             DataOrHash::Data(C::leaf_data()));
        Compact::new(tuple)
    }
}
/// [LeafDataProvider] implementation for `(Tuple, ...)`
///
/// This provides regular (non-compactable) composition of [LeafDataProvider]s.
impl <A, B, C> LeafDataProvider for (A, B, C) where
 (A::LeafData, B::LeafData, C::LeafData): FullLeaf, A: LeafDataProvider,
 B: LeafDataProvider, C: LeafDataProvider {
    type LeafData = (A::LeafData, B::LeafData, C::LeafData);
    fn leaf_data() -> Self::LeafData {
        (A::leaf_data(), B::leaf_data(), C::leaf_data())
    }
}
/// [FullLeaf] implementation for `Compact<H, (DataOrHash<H, Tuple>, ...)>`
impl <H, A, B, C, D> FullLeaf for
 Compact<H,
         (DataOrHash<H, A>, DataOrHash<H, B>, DataOrHash<H, C>,
          DataOrHash<H, D>)> where H: traits::Hash, A: FullLeaf, B: FullLeaf,
 C: FullLeaf, D: FullLeaf {
    fn using_encoded<R, F: FnOnce(&[u8]) -> R>(&self, f: F, compact: bool)
     -> R {
        if compact {
            codec::Encode::using_encoded(&(DataOrHash::<H,
                                                        A>::Hash(self.tuple.0.hash()),
                                           DataOrHash::<H,
                                                        B>::Hash(self.tuple.1.hash()),
                                           DataOrHash::<H,
                                                        C>::Hash(self.tuple.2.hash()),
                                           DataOrHash::<H,
                                                        D>::Hash(self.tuple.3.hash())),
                                         f)
        } else { codec::Encode::using_encoded(&self.tuple, f) }
    }
}
/// [LeafDataProvider] implementation for `Compact<H, (DataOrHash<H, Tuple>, ...)>`
///
/// This provides a compact-form encoding for tuples wrapped in [Compact].
impl <H, A, B, C, D> LeafDataProvider for Compact<H, (A, B, C, D)> where
 H: traits::Hash, A: LeafDataProvider, B: LeafDataProvider,
 C: LeafDataProvider, D: LeafDataProvider {
    type LeafData =
     Compact<H,
             (DataOrHash<H, A::LeafData>, DataOrHash<H, B::LeafData>,
              DataOrHash<H, C::LeafData>, DataOrHash<H, D::LeafData>)>;
    fn leaf_data() -> Self::LeafData {
        let tuple =
            (DataOrHash::Data(A::leaf_data()),
             DataOrHash::Data(B::leaf_data()),
             DataOrHash::Data(C::leaf_data()),
             DataOrHash::Data(D::leaf_data()));
        Compact::new(tuple)
    }
}
/// [LeafDataProvider] implementation for `(Tuple, ...)`
///
/// This provides regular (non-compactable) composition of [LeafDataProvider]s.
impl <A, B, C, D> LeafDataProvider for (A, B, C, D) where
 (A::LeafData, B::LeafData, C::LeafData, D::LeafData): FullLeaf,
 A: LeafDataProvider, B: LeafDataProvider, C: LeafDataProvider,
 D: LeafDataProvider {
    type LeafData = (A::LeafData, B::LeafData, C::LeafData, D::LeafData);
    fn leaf_data() -> Self::LeafData {
        (A::leaf_data(), B::leaf_data(), C::leaf_data(), D::leaf_data())
    }
}
/// [FullLeaf] implementation for `Compact<H, (DataOrHash<H, Tuple>, ...)>`
impl <H, A, B, C, D, E> FullLeaf for
 Compact<H,
         (DataOrHash<H, A>, DataOrHash<H, B>, DataOrHash<H, C>,
          DataOrHash<H, D>, DataOrHash<H, E>)> where H: traits::Hash,
 A: FullLeaf, B: FullLeaf, C: FullLeaf, D: FullLeaf, E: FullLeaf {
    fn using_encoded<R, F: FnOnce(&[u8]) -> R>(&self, f: F, compact: bool)
     -> R {
        if compact {
            codec::Encode::using_encoded(&(DataOrHash::<H,
                                                        A>::Hash(self.tuple.0.hash()),
                                           DataOrHash::<H,
                                                        B>::Hash(self.tuple.1.hash()),
                                           DataOrHash::<H,
                                                        C>::Hash(self.tuple.2.hash()),
                                           DataOrHash::<H,
                                                        D>::Hash(self.tuple.3.hash()),
                                           DataOrHash::<H,
                                                        E>::Hash(self.tuple.4.hash())),
                                         f)
        } else { codec::Encode::using_encoded(&self.tuple, f) }
    }
}
/// [LeafDataProvider] implementation for `Compact<H, (DataOrHash<H, Tuple>, ...)>`
///
/// This provides a compact-form encoding for tuples wrapped in [Compact].
impl <H, A, B, C, D, E> LeafDataProvider for Compact<H, (A, B, C, D, E)> where
 H: traits::Hash, A: LeafDataProvider, B: LeafDataProvider,
 C: LeafDataProvider, D: LeafDataProvider, E: LeafDataProvider {
    type LeafData =
     Compact<H,
             (DataOrHash<H, A::LeafData>, DataOrHash<H, B::LeafData>,
              DataOrHash<H, C::LeafData>, DataOrHash<H, D::LeafData>,
              DataOrHash<H, E::LeafData>)>;
    fn leaf_data() -> Self::LeafData {
        let tuple =
            (DataOrHash::Data(A::leaf_data()),
             DataOrHash::Data(B::leaf_data()),
             DataOrHash::Data(C::leaf_data()),
             DataOrHash::Data(D::leaf_data()),
             DataOrHash::Data(E::leaf_data()));
        Compact::new(tuple)
    }
}
/// [LeafDataProvider] implementation for `(Tuple, ...)`
///
/// This provides regular (non-compactable) composition of [LeafDataProvider]s.
impl <A, B, C, D, E> LeafDataProvider for (A, B, C, D, E) where
 (A::LeafData, B::LeafData, C::LeafData, D::LeafData, E::LeafData): FullLeaf,
 A: LeafDataProvider, B: LeafDataProvider, C: LeafDataProvider,
 D: LeafDataProvider, E: LeafDataProvider {
    type LeafData =
     (A::LeafData, B::LeafData, C::LeafData, D::LeafData, E::LeafData);
    fn leaf_data() -> Self::LeafData {
        (A::leaf_data(), B::leaf_data(), C::leaf_data(), D::leaf_data(),
         E::leaf_data())
    }
}



/// A MMR proof data for one of the leaves.
pub struct Proof<Hash> {
    /// The index of the leaf the proof is for.
    pub leaf_index: u64,
    /// Number of leaves in MMR, when the proof was generated.
    pub leaf_count: u64,
    /// Proof elements (hashes of siblings of inner nodes on the path to the leaf).
    pub items: Vec<Hash>,
}
const _: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate codec as _parity_scale_codec;
        impl <Hash> _parity_scale_codec::Encode for Proof<Hash> where
         Vec<Hash>: _parity_scale_codec::Encode,
         Vec<Hash>: _parity_scale_codec::Encode {
            fn encode_to<__CodecOutputEdqy: _parity_scale_codec::Output +
                         ?Sized>(&self,
                                 __codec_dest_edqy: &mut __CodecOutputEdqy) {
                _parity_scale_codec::Encode::encode_to(&self.leaf_index,
                                                       __codec_dest_edqy);
                _parity_scale_codec::Encode::encode_to(&self.leaf_count,
                                                       __codec_dest_edqy);
                _parity_scale_codec::Encode::encode_to(&self.items,
                                                       __codec_dest_edqy);
            }
        }
        impl <Hash> _parity_scale_codec::EncodeLike for Proof<Hash> where
         Vec<Hash>: _parity_scale_codec::Encode,
         Vec<Hash>: _parity_scale_codec::Encode {
        }
    };
const _: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate codec as _parity_scale_codec;
        impl <Hash> _parity_scale_codec::Decode for Proof<Hash> where
         Vec<Hash>: _parity_scale_codec::Decode,
         Vec<Hash>: _parity_scale_codec::Decode {
            fn decode<__CodecInputEdqy: _parity_scale_codec::Input>(__codec_input_edqy:
                                                                        &mut __CodecInputEdqy)
             -> ::core::result::Result<Self, _parity_scale_codec::Error> {
                ::core::result::Result::Ok(Proof::<Hash>{leaf_index:
                                                             {
                                                                 let __codec_res_edqy =
                                                                     <u64 as
                                                                         _parity_scale_codec::Decode>::decode(__codec_input_edqy);
                                                                 match __codec_res_edqy
                                                                     {
                                                                     ::core::result::Result::Err(e)
                                                                     =>
                                                                     return ::core::result::Result::Err(e.chain("Could not decode `Proof::leaf_index`")),
                                                                     ::core::result::Result::Ok(__codec_res_edqy)
                                                                     =>
                                                                     __codec_res_edqy,
                                                                 }
                                                             },
                                                         leaf_count:
                                                             {
                                                                 let __codec_res_edqy =
                                                                     <u64 as
                                                                         _parity_scale_codec::Decode>::decode(__codec_input_edqy);
                                                                 match __codec_res_edqy
                                                                     {
                                                                     ::core::result::Result::Err(e)
                                                                     =>
                                                                     return ::core::result::Result::Err(e.chain("Could not decode `Proof::leaf_count`")),
                                                                     ::core::result::Result::Ok(__codec_res_edqy)
                                                                     =>
                                                                     __codec_res_edqy,
                                                                 }
                                                             },
                                                         items:
                                                             {
                                                                 let __codec_res_edqy =
                                                                     <Vec<Hash>
                                                                         as
                                                                         _parity_scale_codec::Decode>::decode(__codec_input_edqy);
                                                                 match __codec_res_edqy
                                                                     {
                                                                     ::core::result::Result::Err(e)
                                                                     =>
                                                                     return ::core::result::Result::Err(e.chain("Could not decode `Proof::items`")),
                                                                     ::core::result::Result::Ok(__codec_res_edqy)
                                                                     =>
                                                                     __codec_res_edqy,
                                                                 }
                                                             },})
            }
        }
    };
impl <Hash> core::fmt::Debug for Proof<Hash> where Hash: core::fmt::Debug {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        fmt.debug_struct("Proof").field("leaf_index",
                                        &self.leaf_index).field("leaf_count",
                                                                &self.leaf_count).field("items",
                                                                                        &self.items).finish()
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <Hash: ::core::clone::Clone> ::core::clone::Clone for Proof<Hash> {
    #[inline]
    fn clone(&self) -> Proof<Hash> {
        match *self {
            Proof {
            leaf_index: ref __self_0_0,
            leaf_count: ref __self_0_1,
            items: ref __self_0_2 } =>
            Proof{leaf_index: ::core::clone::Clone::clone(&(*__self_0_0)),
                  leaf_count: ::core::clone::Clone::clone(&(*__self_0_1)),
                  items: ::core::clone::Clone::clone(&(*__self_0_2)),},
        }
    }
}
impl <Hash> ::core::marker::StructuralPartialEq for Proof<Hash> { }
#[automatically_derived]
#[allow(unused_qualifications)]
impl <Hash: ::core::cmp::PartialEq> ::core::cmp::PartialEq for Proof<Hash> {
    #[inline]
    fn eq(&self, other: &Proof<Hash>) -> bool {
        match *other {
            Proof {
            leaf_index: ref __self_1_0,
            leaf_count: ref __self_1_1,
            items: ref __self_1_2 } =>
            match *self {
                Proof {
                leaf_index: ref __self_0_0,
                leaf_count: ref __self_0_1,
                items: ref __self_0_2 } =>
                (*__self_0_0) == (*__self_1_0) &&
                    (*__self_0_1) == (*__self_1_1) &&
                    (*__self_0_2) == (*__self_1_2),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &Proof<Hash>) -> bool {
        match *other {
            Proof {
            leaf_index: ref __self_1_0,
            leaf_count: ref __self_1_1,
            items: ref __self_1_2 } =>
            match *self {
                Proof {
                leaf_index: ref __self_0_0,
                leaf_count: ref __self_0_1,
                items: ref __self_0_2 } =>
                (*__self_0_0) != (*__self_1_0) ||
                    (*__self_0_1) != (*__self_1_1) ||
                    (*__self_0_2) != (*__self_1_2),
            },
        }
    }
}
impl <Hash> ::core::marker::StructuralEq for Proof<Hash> { }
#[automatically_derived]
#[allow(unused_qualifications)]
impl <Hash: ::core::cmp::Eq> ::core::cmp::Eq for Proof<Hash> {
    #[inline]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::core::cmp::AssertParamIsEq<u64>;
            let _: ::core::cmp::AssertParamIsEq<u64>;
            let _: ::core::cmp::AssertParamIsEq<Vec<Hash>>;
        }
    }
}

/// Merkle Mountain Range operation error.
pub enum Error {

    /// Error while pushing new node.
    Push,

    /// Error getting the new root.
    GetRoot,

    /// Error commiting changes.
    Commit,

    /// Error during proof generation.
    GenerateProof,

    /// Proof verification error.
    Verify,

    /// Leaf not found in the storage.
    LeafNotFound,
}
impl core::fmt::Debug for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Push => fmt.debug_tuple("Error::Push").finish(),
            Self::GetRoot => fmt.debug_tuple("Error::GetRoot").finish(),
            Self::Commit => fmt.debug_tuple("Error::Commit").finish(),
            Self::GenerateProof =>
            fmt.debug_tuple("Error::GenerateProof").finish(),
            Self::Verify => fmt.debug_tuple("Error::Verify").finish(),
            Self::LeafNotFound =>
            fmt.debug_tuple("Error::LeafNotFound").finish(),
            _ => Ok(()),
        }
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
                    Error::Push => {
                        __codec_dest_edqy.push_byte(0usize as
                                                        ::core::primitive::u8);
                    }
                    Error::GetRoot => {
                        __codec_dest_edqy.push_byte(1usize as
                                                        ::core::primitive::u8);
                    }
                    Error::Commit => {
                        __codec_dest_edqy.push_byte(2usize as
                                                        ::core::primitive::u8);
                    }
                    Error::GenerateProof => {
                        __codec_dest_edqy.push_byte(3usize as
                                                        ::core::primitive::u8);
                    }
                    Error::Verify => {
                        __codec_dest_edqy.push_byte(4usize as
                                                        ::core::primitive::u8);
                    }
                    Error::LeafNotFound => {
                        __codec_dest_edqy.push_byte(5usize as
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
                        ::core::result::Result::Ok(Error::Push)
                    }
                    __codec_x_edqy if
                    __codec_x_edqy == 1usize as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(Error::GetRoot)
                    }
                    __codec_x_edqy if
                    __codec_x_edqy == 2usize as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(Error::Commit)
                    }
                    __codec_x_edqy if
                    __codec_x_edqy == 3usize as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(Error::GenerateProof)
                    }
                    __codec_x_edqy if
                    __codec_x_edqy == 4usize as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(Error::Verify)
                    }
                    __codec_x_edqy if
                    __codec_x_edqy == 5usize as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(Error::LeafNotFound)
                    }
                    _ =>
                    ::core::result::Result::Err("Could not decode `Error`, variant doesn\'t exist".into()),
                }
            }
        }
    };
impl ::core::marker::StructuralPartialEq for Error { }
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::PartialEq for Error {
    #[inline]
    fn eq(&self, other: &Error) -> bool {
        {
            let __self_vi = ::core::intrinsics::discriminant_value(&*self);
            let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) { _ => true, }
            } else { false }
        }
    }
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

impl Error {
    #![allow(unused_variables)]
    /// Consume given error `e` with `self` and generate a native log entry with error details.
    pub fn log_error(self, e: impl fmt::Debug) -> Self {










        // wrap into `OpaqueLeaf` type








        // given

        // when

        // then

        // given

        // when


        // then
        // check encoding correctness

        // given

        // when

        // then

        // given

        // when

        // then

        // given


        // when




        // then


        // given




        // then


        // given

        // when

        // then
        // then encoding should also match double-encoded leaf.

        {
            let lvl = ::log::Level::Error;
            if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                ::log::__private_api_log(::core::fmt::Arguments::new_v1(&["[",
                                                                          "] MMR error: "],
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
                                         &("runtime::mmr",
                                           "pallet_mmr_primitives",
                                           "pallets/merkle-mountain-range/primitives/src/lib.rs",
                                           310u32));
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
                                                                          "] MMR error: "],
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
                                         &("runtime::mmr",
                                           "pallet_mmr_primitives",
                                           "pallets/merkle-mountain-range/primitives/src/lib.rs",
                                           321u32));
            }
        };
        self
    }
}
/// A helper type to allow using arbitrary SCALE-encoded leaf data in the RuntimeApi.
///
/// The point is to be able to verify MMR proofs from external MMRs, where we don't
/// know the exact leaf type, but it's enough for us to have it SCALE-encoded.
///
/// Note the leaf type should be encoded in its compact form when passed through this type.
/// See [FullLeaf] documentation for details.
///
/// This type does not implement SCALE encoding/decoding on purpose to avoid confusion,
/// it would have to be SCALE-compatible with the concrete leaf type, but due to SCALE limitations
/// it's not possible to know how many bytes the encoding of concrete leaf type uses.
pub struct OpaqueLeaf(
                      /// Raw bytes of the leaf type encoded in its compact form.
                      ///
                      /// NOTE it DOES NOT include length prefix (like `Vec<u8>` encoding would).
                      #[serde(with = "sp_core::bytes")]
                      pub Vec<u8>);
impl core::fmt::Debug for OpaqueLeaf {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        fmt.debug_tuple("OpaqueLeaf").field(&self.0).finish()
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for OpaqueLeaf {
    #[inline]
    fn clone(&self) -> OpaqueLeaf {
        match *self {
            OpaqueLeaf(ref __self_0_0) =>
            OpaqueLeaf(::core::clone::Clone::clone(&(*__self_0_0))),
        }
    }
}
impl ::core::marker::StructuralPartialEq for OpaqueLeaf { }
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::PartialEq for OpaqueLeaf {
    #[inline]
    fn eq(&self, other: &OpaqueLeaf) -> bool {
        match *other {
            OpaqueLeaf(ref __self_1_0) =>
            match *self {
                OpaqueLeaf(ref __self_0_0) => (*__self_0_0) == (*__self_1_0),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &OpaqueLeaf) -> bool {
        match *other {
            OpaqueLeaf(ref __self_1_0) =>
            match *self {
                OpaqueLeaf(ref __self_0_0) => (*__self_0_0) != (*__self_1_0),
            },
        }
    }
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
        impl _serde::Serialize for OpaqueLeaf {
            fn serialize<__S>(&self, __serializer: __S)
             -> _serde::__private::Result<__S::Ok, __S::Error> where
             __S: _serde::Serializer {
                _serde::Serializer::serialize_newtype_struct(__serializer,
                                                             "OpaqueLeaf",
                                                             {
                                                                 struct __SerializeWith<'__a> {
                                                                     values: (&'__a Vec<u8>,),
                                                                     phantom: _serde::__private::PhantomData<OpaqueLeaf>,
                                                                 }
                                                                 impl <'__a>
                                                                  _serde::Serialize
                                                                  for
                                                                  __SerializeWith<'__a>
                                                                  {
                                                                     fn serialize<__S>(&self,
                                                                                       __s:
                                                                                           __S)
                                                                      ->
                                                                          _serde::__private::Result<__S::Ok,
                                                                                                    __S::Error>
                                                                      where
                                                                      __S: _serde::Serializer {
                                                                         sp_core::bytes::serialize(self.values.0,
                                                                                                   __s)
                                                                     }
                                                                 }
                                                                 &__SerializeWith{values:
                                                                                      (&self.0,),
                                                                                  phantom:
                                                                                      _serde::__private::PhantomData::<OpaqueLeaf>,}
                                                             })
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
        impl <'de> _serde::Deserialize<'de> for OpaqueLeaf {
            fn deserialize<__D>(__deserializer: __D)
             -> _serde::__private::Result<Self, __D::Error> where
             __D: _serde::Deserializer<'de> {
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<OpaqueLeaf>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl <'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = OpaqueLeaf;
                    fn expecting(&self,
                                 __formatter:
                                     &mut _serde::__private::Formatter)
                     -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter,
                                                                "tuple struct OpaqueLeaf")
                    }
                    #[inline]
                    fn visit_newtype_struct<__E>(self, __e: __E)
                     -> _serde::__private::Result<Self::Value, __E::Error>
                     where __E: _serde::Deserializer<'de> {
                        let __field0: Vec<u8> =
                            match sp_core::bytes::deserialize(__e) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                        _serde::__private::Ok(OpaqueLeaf(__field0))
                    }
                    #[inline]
                    fn visit_seq<__A>(self, mut __seq: __A)
                     -> _serde::__private::Result<Self::Value, __A::Error>
                     where __A: _serde::de::SeqAccess<'de> {
                        let __field0 =
                            match {
                                      struct __DeserializeWith<'de> {
                                          value: Vec<u8>,
                                          phantom: _serde::__private::PhantomData<OpaqueLeaf>,
                                          lifetime: _serde::__private::PhantomData<&'de ()>,
                                      }
                                      impl <'de> _serde::Deserialize<'de> for
                                       __DeserializeWith<'de> {
                                          fn deserialize<__D>(__deserializer:
                                                                  __D)
                                           ->
                                               _serde::__private::Result<Self,
                                                                         __D::Error>
                                           where
                                           __D: _serde::Deserializer<'de> {
                                              _serde::__private::Ok(__DeserializeWith{value:
                                                                                          match sp_core::bytes::deserialize(__deserializer)
                                                                                              {
                                                                                              _serde::__private::Ok(__val)
                                                                                              =>
                                                                                              __val,
                                                                                              _serde::__private::Err(__err)
                                                                                              =>
                                                                                              {
                                                                                                  return _serde::__private::Err(__err);
                                                                                              }
                                                                                          },
                                                                                      phantom:
                                                                                          _serde::__private::PhantomData,
                                                                                      lifetime:
                                                                                          _serde::__private::PhantomData,})
                                          }
                                      }
                                      _serde::__private::Option::map(match _serde::de::SeqAccess::next_element::<__DeserializeWith<'de>>(&mut __seq)
                                                                         {
                                                                         _serde::__private::Ok(__val)
                                                                         =>
                                                                         __val,
                                                                         _serde::__private::Err(__err)
                                                                         => {
                                                                             return _serde::__private::Err(__err);
                                                                         }
                                                                     },
                                                                     |__wrap|
                                                                         __wrap.value)
                                  } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(_serde::de::Error::invalid_length(0usize,
                                                                                                    &"tuple struct OpaqueLeaf with 1 element"));
                                }
                            };
                        _serde::__private::Ok(OpaqueLeaf(__field0))
                    }
                }
                _serde::Deserializer::deserialize_newtype_struct(__deserializer,
                                                                 "OpaqueLeaf",
                                                                 __Visitor{marker:
                                                                               _serde::__private::PhantomData::<OpaqueLeaf>,
                                                                           lifetime:
                                                                               _serde::__private::PhantomData,})
            }
        }
    };
impl OpaqueLeaf {
    /// Convert a concrete MMR leaf into an opaque type.
    pub fn from_leaf<T: FullLeaf>(leaf: &T) -> Self {
        let encoded_leaf = leaf.using_encoded(|d| d.to_vec(), true);
        OpaqueLeaf::from_encoded_leaf(encoded_leaf)
    }
    /// Create a `OpaqueLeaf` given raw bytes of compact-encoded leaf.
    pub fn from_encoded_leaf(encoded_leaf: Vec<u8>) -> Self {
        OpaqueLeaf(encoded_leaf)
    }
    /// Attempt to decode the leaf into expected concrete type.
    pub fn try_decode<T: codec::Decode>(&self) -> Option<T> {
        codec::Decode::decode(&mut &*self.0).ok()
    }
}
impl FullLeaf for OpaqueLeaf {
    fn using_encoded<R, F: FnOnce(&[u8]) -> R>(&self, f: F, _compact: bool)
     -> R {
        f(&self.0)
    }
}
/// A type-safe wrapper for the concrete leaf type.
///
/// This structure serves merely to avoid passing raw `Vec<u8>` around.
/// It must be `Vec<u8>`-encoding compatible.
///
/// It is different from [`OpaqueLeaf`], because it does implement `Codec`
/// and the encoding has to match raw `Vec<u8>` encoding.
pub struct EncodableOpaqueLeaf(pub Vec<u8>);
const _: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate codec as _parity_scale_codec;
        impl _parity_scale_codec::Encode for EncodableOpaqueLeaf {
            fn encode_to<__CodecOutputEdqy: _parity_scale_codec::Output +
                         ?Sized>(&self,
                                 __codec_dest_edqy: &mut __CodecOutputEdqy) {
                _parity_scale_codec::Encode::encode_to(&&self.0,
                                                       __codec_dest_edqy)
            }
            fn encode(&self) -> _parity_scale_codec::alloc::vec::Vec<u8> {
                _parity_scale_codec::Encode::encode(&&self.0)
            }
            fn using_encoded<R,
                             F: ::core::ops::FnOnce(&[::core::primitive::u8])
                             -> R>(&self, f: F) -> R {
                _parity_scale_codec::Encode::using_encoded(&&self.0, f)
            }
        }
        impl _parity_scale_codec::EncodeLike for EncodableOpaqueLeaf { }
    };
const _: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate codec as _parity_scale_codec;
        impl _parity_scale_codec::Decode for EncodableOpaqueLeaf {
            fn decode<__CodecInputEdqy: _parity_scale_codec::Input>(__codec_input_edqy:
                                                                        &mut __CodecInputEdqy)
             -> ::core::result::Result<Self, _parity_scale_codec::Error> {
                ::core::result::Result::Ok(EncodableOpaqueLeaf({
                                                                   let __codec_res_edqy =
                                                                       <Vec<u8>
                                                                           as
                                                                           _parity_scale_codec::Decode>::decode(__codec_input_edqy);
                                                                   match __codec_res_edqy
                                                                       {
                                                                       ::core::result::Result::Err(e)
                                                                       =>
                                                                       return ::core::result::Result::Err(e.chain("Could not decode `EncodableOpaqueLeaf.0`")),
                                                                       ::core::result::Result::Ok(__codec_res_edqy)
                                                                       =>
                                                                       __codec_res_edqy,
                                                                   }
                                                               }))
            }
        }
    };
impl core::fmt::Debug for EncodableOpaqueLeaf {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        fmt.debug_tuple("EncodableOpaqueLeaf").field(&self.0).finish()
    }
}
impl ::core::marker::StructuralPartialEq for EncodableOpaqueLeaf { }
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::PartialEq for EncodableOpaqueLeaf {
    #[inline]
    fn eq(&self, other: &EncodableOpaqueLeaf) -> bool {
        match *other {
            EncodableOpaqueLeaf(ref __self_1_0) =>
            match *self {
                EncodableOpaqueLeaf(ref __self_0_0) =>
                (*__self_0_0) == (*__self_1_0),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &EncodableOpaqueLeaf) -> bool {
        match *other {
            EncodableOpaqueLeaf(ref __self_1_0) =>
            match *self {
                EncodableOpaqueLeaf(ref __self_0_0) =>
                (*__self_0_0) != (*__self_1_0),
            },
        }
    }
}
impl ::core::marker::StructuralEq for EncodableOpaqueLeaf { }
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::Eq for EncodableOpaqueLeaf {
    #[inline]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {
        { let _: ::core::cmp::AssertParamIsEq<Vec<u8>>; }
    }
}
impl EncodableOpaqueLeaf {
    /// Convert a concrete leaf into encodable opaque version.
    pub fn from_leaf<T: FullLeaf>(leaf: &T) -> Self {
        let opaque = OpaqueLeaf::from_leaf(leaf);
        Self::from_opaque_leaf(opaque)
    }
    /// Given an opaque leaf, make it encodable.
    pub fn from_opaque_leaf(opaque: OpaqueLeaf) -> Self { Self(opaque.0) }
    /// Try to convert into a [OpaqueLeaf].
    pub fn into_opaque_leaf(self) -> OpaqueLeaf {
        OpaqueLeaf::from_encoded_leaf(self.0)
    }
}
#[doc(hidden)]
mod sp_api_hidden_includes_DECL_RUNTIME_APIS {
    pub extern crate sp_api as sp_api;
}
#[doc(hidden)]
#[allow(dead_code)]
#[allow(deprecated)]
pub mod runtime_decl_for_MmrApi {
    use super::*;
    #[doc = " API to interact with MMR pallet."]
    pub trait MmrApi<Block: self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::BlockT,
                     Hash: codec::Codec> {
        #[doc = " Generate MMR proof for a leaf under given index."]
        fn generate_proof(leaf_index: u64)
        -> Result<(EncodableOpaqueLeaf, Proof<Hash>), Error>;
        #[doc = " Verify MMR proof against on-chain MMR."]
        #[doc = ""]
        #[doc =
          " Note this function will use on-chain MMR root hash and check if the proof"]
        #[doc = " matches the hash."]
        #[doc =
          " See [Self::verify_proof_stateless] for a stateless verifier."]
        fn verify_proof(leaf: EncodableOpaqueLeaf, proof: Proof<Hash>)
        -> Result<(), Error>;
        #[doc = " Verify MMR proof against given root hash."]
        #[doc = ""]
        #[doc =
          " Note this function does not require any on-chain storage - the"]
        #[doc = " proof is verified against given MMR root hash."]
        #[doc = ""]
        #[doc =
          " The leaf data is expected to be encoded in it\'s compact form."]
        fn verify_proof_stateless(root: Hash, leaf: EncodableOpaqueLeaf,
                                  proof: Proof<Hash>)
        -> Result<(), Error>;
    }
    pub const VERSION: u32 = 1u32;
    pub const ID: [u8; 8] =
        [145u8, 213u8, 223u8, 24u8, 176u8, 210u8, 207u8, 88u8];
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
    pub fn generate_proof_native_call_generator<'a,
                                                ApiImpl: MmrApi<Block, Hash>,
                                                NodeBlock: self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::BlockT,
                                                Block: self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::BlockT +
                                                'a, Hash: codec::Codec +
                                                'a>(leaf_index: u64)
     ->
         impl FnOnce()
         ->
             std::result::Result<Result<(EncodableOpaqueLeaf, Proof<Hash>),
                                        Error>,
                                 self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ApiError> +
         'a {
        move || { let res = ApiImpl::generate_proof(leaf_index); Ok(res) }
    }
    #[cfg(any(feature = "std", test))]
    pub fn verify_proof_native_call_generator<'a,
                                              ApiImpl: MmrApi<Block, Hash>,
                                              NodeBlock: self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::BlockT,
                                              Block: self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::BlockT +
                                              'a, Hash: codec::Codec +
                                              'a>(leaf: EncodableOpaqueLeaf,
                                                  proof: Proof<Hash>)
     ->
         impl FnOnce()
         ->
             std::result::Result<Result<(), Error>,
                                 self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ApiError> +
         'a {
        move || { let res = ApiImpl::verify_proof(leaf, proof); Ok(res) }
    }
    #[cfg(any(feature = "std", test))]
    pub fn verify_proof_stateless_native_call_generator<'a,
                                                        ApiImpl: MmrApi<Block,
                                                                        Hash>,
                                                        NodeBlock: self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::BlockT,
                                                        Block: self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::BlockT +
                                                        'a,
                                                        Hash: codec::Codec +
                                                        'a>(root: Hash,
                                                            leaf:
                                                                EncodableOpaqueLeaf,
                                                            proof:
                                                                Proof<Hash>)
     ->
         impl FnOnce()
         ->
             std::result::Result<Result<(), Error>,
                                 self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ApiError> +
         'a {
        move ||
            {
                let res = ApiImpl::verify_proof_stateless(root, leaf, proof);
                Ok(res)
            }
    }
    #[cfg(any(feature = "std", test))]
    pub fn generate_proof_call_api_at<R: self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::Encode +
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
                                                                                        "MmrApi_generate_proof",
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
    #[cfg(any(feature = "std", test))]
    pub fn verify_proof_call_api_at<R: self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::Encode +
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
                                                                                        "MmrApi_verify_proof",
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
    #[cfg(any(feature = "std", test))]
    pub fn verify_proof_stateless_call_api_at<R: self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::Encode +
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
                                                                                        "MmrApi_verify_proof_stateless",
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
#[doc = " API to interact with MMR pallet."]
#[cfg(any(feature = "std", test))]
pub trait MmrApi<Block: self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::BlockT,
                 Hash: codec::Codec>: self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::Core<Block> {
    #[doc = " Generate MMR proof for a leaf under given index."]
    fn generate_proof(&self,
                      __runtime_api_at_param__:
                          &self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::BlockId<Block>,
                      leaf_index: u64)
     ->
         std::result::Result<Result<(EncodableOpaqueLeaf, Proof<Hash>),
                                    Error>,
                             self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ApiError> {
        let runtime_api_impl_params_encoded =
            self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::Encode::encode(&(&leaf_index));
        self.MmrApi_generate_proof_runtime_api_impl(__runtime_api_at_param__,
                                                    self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ExecutionContext::OffchainCall(None),
                                                    Some((leaf_index)),
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
                                                                                                          <Result<(EncodableOpaqueLeaf,
                                                                                                                   Proof<Hash>),
                                                                                                                  Error>
                                                                                                              as
                                                                                                              self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::Decode>::decode(&mut &r[..]).map_err(|err|
                                                                                                                                                                                                               self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ApiError::FailedToDecodeReturnValue{function:
                                                                                                                                                                                                                                                                                                               "generate_proof",
                                                                                                                                                                                                                                                                                                           error:
                                                                                                                                                                                                                                                                                                               err,})
                                                                                                      }
                                                                                                  })
    }
    #[doc = " Generate MMR proof for a leaf under given index."]
    fn generate_proof_with_context(&self,
                                   __runtime_api_at_param__:
                                       &self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::BlockId<Block>,
                                   context:
                                       self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ExecutionContext,
                                   leaf_index: u64)
     ->
         std::result::Result<Result<(EncodableOpaqueLeaf, Proof<Hash>),
                                    Error>,
                             self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ApiError> {
        let runtime_api_impl_params_encoded =
            self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::Encode::encode(&(&leaf_index));
        self.MmrApi_generate_proof_runtime_api_impl(__runtime_api_at_param__,
                                                    context,
                                                    Some((leaf_index)),
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
                                                                                                          <Result<(EncodableOpaqueLeaf,
                                                                                                                   Proof<Hash>),
                                                                                                                  Error>
                                                                                                              as
                                                                                                              self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::Decode>::decode(&mut &r[..]).map_err(|err|
                                                                                                                                                                                                               self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ApiError::FailedToDecodeReturnValue{function:
                                                                                                                                                                                                                                                                                                               "generate_proof",
                                                                                                                                                                                                                                                                                                           error:
                                                                                                                                                                                                                                                                                                               err,})
                                                                                                      }
                                                                                                  })
    }
    #[doc(hidden)]
    fn MmrApi_generate_proof_runtime_api_impl(&self,
                                              at:
                                                  &self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::BlockId<Block>,
                                              context:
                                                  self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ExecutionContext,
                                              params: Option<(u64)>,
                                              params_encoded: Vec<u8>)
    ->
        std::result::Result<self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::NativeOrEncoded<Result<(EncodableOpaqueLeaf,
                                                                                                            Proof<Hash>),
                                                                                                           Error>>,
                            self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ApiError>;
    #[doc = " Verify MMR proof against on-chain MMR."]
    #[doc = ""]
    #[doc =
      " Note this function will use on-chain MMR root hash and check if the proof"]
    #[doc = " matches the hash."]
    #[doc = " See [Self::verify_proof_stateless] for a stateless verifier."]
    fn verify_proof(&self,
                    __runtime_api_at_param__:
                        &self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::BlockId<Block>,
                    leaf: EncodableOpaqueLeaf, proof: Proof<Hash>)
     ->
         std::result::Result<Result<(), Error>,
                             self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ApiError> {
        let runtime_api_impl_params_encoded =
            self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::Encode::encode(&(&leaf,
                                                                                     &proof));
        self.MmrApi_verify_proof_runtime_api_impl(__runtime_api_at_param__,
                                                  self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ExecutionContext::OffchainCall(None),
                                                  Some((leaf, proof)),
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
                                                                                                        <Result<(),
                                                                                                                Error>
                                                                                                            as
                                                                                                            self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::Decode>::decode(&mut &r[..]).map_err(|err|
                                                                                                                                                                                                             self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ApiError::FailedToDecodeReturnValue{function:
                                                                                                                                                                                                                                                                                                             "verify_proof",
                                                                                                                                                                                                                                                                                                         error:
                                                                                                                                                                                                                                                                                                             err,})
                                                                                                    }
                                                                                                })
    }
    #[doc = " Verify MMR proof against on-chain MMR."]
    #[doc = ""]
    #[doc =
      " Note this function will use on-chain MMR root hash and check if the proof"]
    #[doc = " matches the hash."]
    #[doc = " See [Self::verify_proof_stateless] for a stateless verifier."]
    fn verify_proof_with_context(&self,
                                 __runtime_api_at_param__:
                                     &self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::BlockId<Block>,
                                 context:
                                     self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ExecutionContext,
                                 leaf: EncodableOpaqueLeaf,
                                 proof: Proof<Hash>)
     ->
         std::result::Result<Result<(), Error>,
                             self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ApiError> {
        let runtime_api_impl_params_encoded =
            self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::Encode::encode(&(&leaf,
                                                                                     &proof));
        self.MmrApi_verify_proof_runtime_api_impl(__runtime_api_at_param__,
                                                  context,
                                                  Some((leaf, proof)),
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
                                                                                                        <Result<(),
                                                                                                                Error>
                                                                                                            as
                                                                                                            self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::Decode>::decode(&mut &r[..]).map_err(|err|
                                                                                                                                                                                                             self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ApiError::FailedToDecodeReturnValue{function:
                                                                                                                                                                                                                                                                                                             "verify_proof",
                                                                                                                                                                                                                                                                                                         error:
                                                                                                                                                                                                                                                                                                             err,})
                                                                                                    }
                                                                                                })
    }
    #[doc(hidden)]
    fn MmrApi_verify_proof_runtime_api_impl(&self,
                                            at:
                                                &self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::BlockId<Block>,
                                            context:
                                                self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ExecutionContext,
                                            params:
                                                Option<(EncodableOpaqueLeaf,
                                                        Proof<Hash>)>,
                                            params_encoded: Vec<u8>)
    ->
        std::result::Result<self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::NativeOrEncoded<Result<(),
                                                                                                           Error>>,
                            self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ApiError>;
    #[doc = " Verify MMR proof against given root hash."]
    #[doc = ""]
    #[doc = " Note this function does not require any on-chain storage - the"]
    #[doc = " proof is verified against given MMR root hash."]
    #[doc = ""]
    #[doc = " The leaf data is expected to be encoded in it\'s compact form."]
    fn verify_proof_stateless(&self,
                              __runtime_api_at_param__:
                                  &self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::BlockId<Block>,
                              root: Hash, leaf: EncodableOpaqueLeaf,
                              proof: Proof<Hash>)
     ->
         std::result::Result<Result<(), Error>,
                             self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ApiError> {
        let runtime_api_impl_params_encoded =
            self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::Encode::encode(&(&root,
                                                                                     &leaf,
                                                                                     &proof));
        self.MmrApi_verify_proof_stateless_runtime_api_impl(__runtime_api_at_param__,
                                                            self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ExecutionContext::OffchainCall(None),
                                                            Some((root, leaf,
                                                                  proof)),
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
                                                                                                                  <Result<(),
                                                                                                                          Error>
                                                                                                                      as
                                                                                                                      self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::Decode>::decode(&mut &r[..]).map_err(|err|
                                                                                                                                                                                                                       self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ApiError::FailedToDecodeReturnValue{function:
                                                                                                                                                                                                                                                                                                                       "verify_proof_stateless",
                                                                                                                                                                                                                                                                                                                   error:
                                                                                                                                                                                                                                                                                                                       err,})
                                                                                                              }
                                                                                                          })
    }
    #[doc = " Verify MMR proof against given root hash."]
    #[doc = ""]
    #[doc = " Note this function does not require any on-chain storage - the"]
    #[doc = " proof is verified against given MMR root hash."]
    #[doc = ""]
    #[doc = " The leaf data is expected to be encoded in it\'s compact form."]
    fn verify_proof_stateless_with_context(&self,
                                           __runtime_api_at_param__:
                                               &self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::BlockId<Block>,
                                           context:
                                               self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ExecutionContext,
                                           root: Hash,
                                           leaf: EncodableOpaqueLeaf,
                                           proof: Proof<Hash>)
     ->
         std::result::Result<Result<(), Error>,
                             self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ApiError> {
        let runtime_api_impl_params_encoded =
            self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::Encode::encode(&(&root,
                                                                                     &leaf,
                                                                                     &proof));
        self.MmrApi_verify_proof_stateless_runtime_api_impl(__runtime_api_at_param__,
                                                            context,
                                                            Some((root, leaf,
                                                                  proof)),
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
                                                                                                                  <Result<(),
                                                                                                                          Error>
                                                                                                                      as
                                                                                                                      self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::Decode>::decode(&mut &r[..]).map_err(|err|
                                                                                                                                                                                                                       self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ApiError::FailedToDecodeReturnValue{function:
                                                                                                                                                                                                                                                                                                                       "verify_proof_stateless",
                                                                                                                                                                                                                                                                                                                   error:
                                                                                                                                                                                                                                                                                                                       err,})
                                                                                                              }
                                                                                                          })
    }
    #[doc(hidden)]
    fn MmrApi_verify_proof_stateless_runtime_api_impl(&self,
                                                      at:
                                                          &self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::BlockId<Block>,
                                                      context:
                                                          self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ExecutionContext,
                                                      params:
                                                          Option<(Hash,
                                                                  EncodableOpaqueLeaf,
                                                                  Proof<Hash>)>,
                                                      params_encoded: Vec<u8>)
    ->
        std::result::Result<self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::NativeOrEncoded<Result<(),
                                                                                                           Error>>,
                            self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ApiError>;
}
#[cfg(any(feature = "std", test))]
impl <Block: self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::BlockT,
      Hash: codec::Codec>
 self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::RuntimeApiInfo for
 MmrApi<Block, Hash> {
    const ID: [u8; 8] =
        [145u8, 213u8, 223u8, 24u8, 176u8, 210u8, 207u8, 88u8];
    const VERSION: u32 = 1u32;
}
