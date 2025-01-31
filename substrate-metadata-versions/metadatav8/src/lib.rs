// Copyright 2018-2019 Parity Technologies (UK) Ltd.
// This file is part of Substrate.

// Substrate is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Substrate is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Substrate.  If not, see <http://www.gnu.org/licenses/>.

//! Decodable variant of the RuntimeMetadata.
//!
//! This really doesn't belong here, but is necessary for the moment. In the future
//! it should be removed entirely to an external module for shimming on to the
//! codec-encoded metadata.
#![allow(clippy::all)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
use codec::{Decode, Error, Input};
use codec::{Encode, Output};
use frame_metadata::decode_different::*;
use rstd::vec::Vec;
#[cfg(feature = "std")]
use serde::Serialize;

/// Curent prefix of metadata
pub const META_RESERVED: u32 = 0x6174656d; // 'meta' warn endianness

/// All the metadata about a function.
#[derive(Clone, PartialEq, Eq, Encode)]
#[cfg_attr(feature = "std", derive(Decode, Debug, Serialize))]
pub struct FunctionMetadata {
	pub name: DecodeDifferentStr,
	pub arguments: DecodeDifferentArray<FunctionArgumentMetadata>,
	pub documentation: DecodeDifferentArray<&'static str, StringBuf>,
}

/// All the metadata about a function argument.
#[derive(Clone, PartialEq, Eq, Encode)]
#[cfg_attr(feature = "std", derive(Decode, Debug, Serialize))]
pub struct FunctionArgumentMetadata {
	pub name: DecodeDifferentStr,
	pub ty: DecodeDifferentStr,
}

/// Newtype wrapper for support encoding functions (actual the result of the function).
#[derive(Clone, Eq)]
pub struct FnEncode<E>(pub fn() -> E)
where
	E: Encode + 'static;

impl<E: Encode> Encode for FnEncode<E> {
	fn encode_to<W: Output + ?Sized>(&self, dest: &mut W) {
		self.0().encode_to(dest);
	}
}

impl<E: Encode> codec::EncodeLike for FnEncode<E> {}

impl<E: Encode + PartialEq> PartialEq for FnEncode<E> {
	fn eq(&self, other: &Self) -> bool {
		self.0().eq(&other.0())
	}
}

#[cfg(feature = "std")]
impl<E: Encode + ::std::fmt::Debug> std::fmt::Debug for FnEncode<E> {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		self.0().fmt(f)
	}
}

#[cfg(feature = "std")]
impl<E: Encode + serde::Serialize> serde::Serialize for FnEncode<E> {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		self.0().serialize(serializer)
	}
}

/// All the metadata about an outer event.
#[derive(Clone, PartialEq, Eq, Encode)]
#[cfg_attr(feature = "std", derive(Decode, Debug, Serialize))]
pub struct OuterEventMetadata {
	pub name: DecodeDifferentStr,
	pub events:
		DecodeDifferentArray<(&'static str, FnEncode<&'static [EventMetadata]>), (StringBuf, Vec<EventMetadata>)>,
}

/// All the metadata about an event.
#[derive(Clone, PartialEq, Eq, Encode)]
#[cfg_attr(feature = "std", derive(Decode, Debug, Serialize))]
pub struct EventMetadata {
	pub name: DecodeDifferentStr,
	pub arguments: DecodeDifferentArray<&'static str, StringBuf>,
	pub documentation: DecodeDifferentArray<&'static str, StringBuf>,
}

/// All the metadata about one storage entry.
#[derive(Clone, PartialEq, Eq, Encode)]
#[cfg_attr(feature = "std", derive(Decode, Debug, Serialize))]
pub struct StorageEntryMetadata {
	pub name: DecodeDifferentStr,
	pub modifier: StorageEntryModifier,
	pub ty: StorageEntryType,
	pub default: ByteGetter,
	pub documentation: DecodeDifferentArray<&'static str, StringBuf>,
}

/// All the metadata about one module constant.
#[derive(Clone, PartialEq, Eq, Encode)]
#[cfg_attr(feature = "std", derive(Decode, Debug, Serialize))]
pub struct ModuleConstantMetadata {
	pub name: DecodeDifferentStr,
	pub ty: DecodeDifferentStr,
	pub value: ByteGetter,
	pub documentation: DecodeDifferentArray<&'static str, StringBuf>,
}

/// All the metadata about a module error.
#[derive(Clone, PartialEq, Eq, Encode)]
#[cfg_attr(feature = "std", derive(Decode, Debug, Serialize))]
pub struct ErrorMetadata {
	pub name: DecodeDifferentStr,
	pub documentation: DecodeDifferentArray<&'static str, StringBuf>,
}

/// All the metadata about errors in a module.
pub trait ModuleErrorMetadata {
	fn metadata() -> &'static [ErrorMetadata];
}

impl ModuleErrorMetadata for &'static str {
	fn metadata() -> &'static [ErrorMetadata] {
		&[]
	}
}

/// A technical trait to store lazy initiated vec value as static dyn pointer.
pub trait DefaultByte: Send + Sync {
	fn default_byte(&self) -> Vec<u8>;
}

/// Wrapper over dyn pointer for accessing a cached once byte value.
#[derive(Clone)]
pub struct DefaultByteGetter(pub &'static dyn DefaultByte);

/// Decode different for static lazy initiated byte value.
pub type ByteGetter = DecodeDifferent<DefaultByteGetter, Vec<u8>>;

impl Encode for DefaultByteGetter {
	fn encode_to<W: Output + ?Sized>(&self, dest: &mut W) {
		self.0.default_byte().encode_to(dest)
	}
}

impl codec::EncodeLike for DefaultByteGetter {}

impl PartialEq<DefaultByteGetter> for DefaultByteGetter {
	fn eq(&self, other: &DefaultByteGetter) -> bool {
		let left = self.0.default_byte();
		let right = other.0.default_byte();
		left.eq(&right)
	}
}

impl Eq for DefaultByteGetter {}

#[cfg(feature = "std")]
impl serde::Serialize for DefaultByteGetter {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		self.0.default_byte().serialize(serializer)
	}
}

#[cfg(feature = "std")]
impl std::fmt::Debug for DefaultByteGetter {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		self.0.default_byte().fmt(f)
	}
}

/// Hasher used by storage maps
#[derive(Clone, PartialEq, Eq, Encode)]
#[cfg_attr(feature = "std", derive(Decode, Debug, Serialize))]
pub enum StorageHasher {
	Blake2_128,
	Blake2_256,
	Twox128,
	Twox256,
	Twox64Concat,
}

/// A storage entry type.
#[derive(Clone, PartialEq, Eq, Encode)]
#[cfg_attr(feature = "std", derive(Decode, Debug, Serialize))]
pub enum StorageEntryType {
	Plain(DecodeDifferentStr),
	Map {
		hasher: StorageHasher,
		key: DecodeDifferentStr,
		value: DecodeDifferentStr,
		is_linked: bool,
	},
	DoubleMap {
		hasher: StorageHasher,
		key1: DecodeDifferentStr,
		key2: DecodeDifferentStr,
		value: DecodeDifferentStr,
		key2_hasher: StorageHasher,
	},
}

/// A storage entry modifier.
#[derive(Clone, PartialEq, Eq, Encode)]
#[cfg_attr(feature = "std", derive(Decode, Debug, Serialize))]
pub enum StorageEntryModifier {
	Optional,
	Default,
}

/// All metadata of the storage.
#[derive(Clone, PartialEq, Eq, Encode)]
#[cfg_attr(feature = "std", derive(Decode, Debug, Serialize))]
pub struct StorageMetadata {
	/// The common prefix used by all storage entries.
	pub prefix: DecodeDifferent<&'static str, StringBuf>,
	pub entries: DecodeDifferent<&'static [StorageEntryMetadata], Vec<StorageEntryMetadata>>,
}

#[derive(Eq, Encode, PartialEq)]
#[cfg_attr(feature = "std", derive(Decode, Debug, Serialize))]
/// Metadata prefixed by a u32 for reserved usage
pub struct RuntimeMetadataPrefixed(pub u32, pub RuntimeMetadata);

/// The metadata of a runtime.
/// The version ID encoded/decoded through
/// the enum nature of `RuntimeMetadata`.
#[derive(Eq, Encode, PartialEq)]
#[cfg_attr(feature = "std", derive(Decode, Debug, Serialize))]
pub enum RuntimeMetadata {
	/// Unused; enum filler.
	V0(RuntimeMetadataDeprecated),
	/// Version 1 for runtime metadata. No longer used.
	V1(RuntimeMetadataDeprecated),
	/// Version 2 for runtime metadata. No longer used.
	V2(RuntimeMetadataDeprecated),
	/// Version 3 for runtime metadata. No longer used.
	V3(RuntimeMetadataDeprecated),
	/// Version 4 for runtime metadata. No longer used.
	V4(RuntimeMetadataDeprecated),
	/// Version 5 for runtime metadata. No longer used.
	V5(RuntimeMetadataDeprecated),
	/// Version 6 for runtime metadata. No longer used.
	V6(RuntimeMetadataDeprecated),
	/// Version 7 for runtime metadata. No longer used.
	V7(RuntimeMetadataDeprecated),
	/// Version 8 for runtime metadata.
	V8(RuntimeMetadataV8),
}

/// Enum that should fail.
#[derive(Eq, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug, Serialize))]
pub enum RuntimeMetadataDeprecated {}

impl Encode for RuntimeMetadataDeprecated {
	fn encode_to<W: Output + ?Sized>(&self, _dest: &mut W) {}
}

impl codec::EncodeLike for RuntimeMetadataDeprecated {}

#[cfg(feature = "std")]
impl Decode for RuntimeMetadataDeprecated {
	fn decode<I: Input>(_input: &mut I) -> Result<Self, Error> {
		Err("Decoding is not supported".into())
	}
}

/// The metadata of a runtime.
#[derive(Eq, Encode, PartialEq)]
#[cfg_attr(feature = "std", derive(Decode, Debug, Serialize))]
pub struct RuntimeMetadataV8 {
	pub modules: DecodeDifferentArray<ModuleMetadata>,
}

/// The latest version of the metadata.
pub type RuntimeMetadataLastVersion = RuntimeMetadataV8;

/// All metadata about an runtime module.
#[derive(Clone, PartialEq, Eq, Encode)]
#[cfg_attr(feature = "std", derive(Decode, Debug, Serialize))]
pub struct ModuleMetadata {
	pub name: DecodeDifferentStr,
	pub storage: Option<DecodeDifferent<FnEncode<StorageMetadata>, StorageMetadata>>,
	pub calls: ODFnA<FunctionMetadata>,
	pub event: ODFnA<EventMetadata>,
	pub constants: DFnA<ModuleConstantMetadata>,
	pub errors: DFnA<ErrorMetadata>,
}

type ODFnA<T> = Option<DFnA<T>>;
type DFnA<T> = DecodeDifferent<FnEncode<&'static [T]>, Vec<T>>;

impl Into<frame_metadata::OpaqueMetadata> for RuntimeMetadataPrefixed {
	fn into(self) -> frame_metadata::OpaqueMetadata {
		frame_metadata::OpaqueMetadata(self.encode())
	}
}

impl Into<RuntimeMetadataPrefixed> for RuntimeMetadataLastVersion {
	fn into(self) -> RuntimeMetadataPrefixed {
		RuntimeMetadataPrefixed(META_RESERVED, RuntimeMetadata::V8(self))
	}
}
