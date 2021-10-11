use std::convert::From;
use std::fmt::Debug;

/// Whereas [`crate::substrate_type::SubstrateType`] is concerned with type information,
/// [`SubstrateValue`] is concerned with holding a representation of actual values
/// corresponding to each of those types.
///
/// Not all types have an similar-named value; for example, sequences and array
/// values can both be represented with [`SequenceValue`], and structs and tuple values can
/// both be represented with [`CompositeValue`]. Only enough information is preserved to
/// construct a valid value for any type that we know about, and it should be possible to
/// verify whether a value can be treated as a given [`crate::substrate_type::SubstrateType`]
/// or not.
#[derive(Clone)]
pub enum SubstrateValue {
	/// Values for a named or unnamed struct or tuple.
	Composite(CompositeValue),
	/// An enum variant.
	Variant(VariantValue),
	/// A value corresponding to a sequence or array type, or even a BitVec.
	Sequence(SequenceValue),
	/// Any of the primitive values we can have.
	Primitive(PrimitiveValue),
}

impl Debug for SubstrateValue {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Composite(val) => Debug::fmt(val, f),
			Self::Variant(val) => Debug::fmt(val, f),
			Self::Sequence(val) => Debug::fmt(val, f),
			Self::Primitive(val) => Debug::fmt(val, f),
		}
	}
}

#[derive(Clone)]
pub enum CompositeValue {
	/// Eg `{ foo: 2, bar: false }`
	Named(Vec<(String, SubstrateValue)>),
	/// Eg `(2, false)`
	Unnamed(Vec<SubstrateValue>),
}

impl Debug for CompositeValue {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			CompositeValue::Named(fields) => {
				let mut struc = f.debug_struct("");
				for (name, val) in fields {
					struc.field(name, val);
				}
				struc.finish()
			}
			CompositeValue::Unnamed(fields) => {
				let mut struc = f.debug_tuple("");
				for val in fields {
					struc.field(val);
				}
				struc.finish()
			}
		}
	}
}

impl From<CompositeValue> for SubstrateValue {
	fn from(val: CompositeValue) -> Self {
		SubstrateValue::Composite(val)
	}
}

#[derive(Clone)]
pub struct VariantValue {
	pub name: String,
	pub fields: CompositeValue,
}

impl Debug for VariantValue {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(&self.name)?;
		f.write_str(" ")?;
		Debug::fmt(&self.fields, f)
	}
}

impl From<VariantValue> for SubstrateValue {
	fn from(val: VariantValue) -> Self {
		SubstrateValue::Variant(val)
	}
}

#[derive(Clone)]
pub struct SequenceValue {
	pub values: Vec<SubstrateValue>,
}

impl Debug for SequenceValue {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_list().entries(&self.values).finish()
	}
}

impl From<SequenceValue> for SubstrateValue {
	fn from(val: SequenceValue) -> Self {
		SubstrateValue::Sequence(val)
	}
}

#[derive(Clone)]
pub enum PrimitiveValue {
	Bool(bool),
	Char(char),
	Str(String),
	U8(u8),
	U16(u16),
	U32(u32),
	U64(u64),
	U128(u128),
	U256([u8; 32]),
	I8(i8),
	I16(i16),
	I32(i32),
	I64(i64),
	I128(i128),
	I256([u8; 32]),
}

impl Debug for PrimitiveValue {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			PrimitiveValue::Bool(val) => Debug::fmt(val, f),
			PrimitiveValue::Char(val) => Debug::fmt(val, f),
			PrimitiveValue::Str(val) => Debug::fmt(val, f),
			PrimitiveValue::U8(val) => Debug::fmt(val, f),
			PrimitiveValue::U16(val) => Debug::fmt(val, f),
			PrimitiveValue::U32(val) => Debug::fmt(val, f),
			PrimitiveValue::U64(val) => Debug::fmt(val, f),
			PrimitiveValue::U128(val) => Debug::fmt(val, f),
			PrimitiveValue::I8(val) => Debug::fmt(val, f),
			PrimitiveValue::I16(val) => Debug::fmt(val, f),
			PrimitiveValue::I32(val) => Debug::fmt(val, f),
			PrimitiveValue::I64(val) => Debug::fmt(val, f),
			PrimitiveValue::I128(val) => Debug::fmt(val, f),
			PrimitiveValue::U256(val) | PrimitiveValue::I256(val) => {
				f.write_str("BigNum(")?;
				Debug::fmt(val, f)?;
				f.write_str(")")
			}
		}
	}
}

impl From<PrimitiveValue> for SubstrateValue {
	fn from(val: PrimitiveValue) -> Self {
		SubstrateValue::Primitive(val)
	}
}
