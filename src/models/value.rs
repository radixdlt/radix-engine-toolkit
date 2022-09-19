//! This module implements the [Value] struct as well as all of its related methods for conversion
//! and validation.
//!
//! This module implements [Value] conversions into two main types:
//!
//! 1. [transaction::manifest::ast::Value]: Conversion into and from this type is supported since
//! this the type that values need to be in for the creation of transaction manifest instructions
//! and because it is the type that values are found in when a manifest is decompiled. Therefore,
//! the functions [value_from_ast_value] and [ast_value_from_value] can be used to convert a [Value]
//! from and to [transaction::manifest::ast::Value].
//! 2. [sbor::Value]: Easy conversions from and to this type are needed since this type is needed
//! since this type is often times used as an intermediary type for the SBOR Encode and Decode
//! requests to this library. The conversion back in forth is done through the functions
//! [value_from_sbor_value] and [sbor_value_from_value].

use radix_engine::types::ScryptoType;
use sbor::type_id::*;
use sbor::{decode_any, encode_any, Value as SborValue};
use scrypto::prelude::{
    scrypto_decode, scrypto_encode, Blob, Decimal, EcdsaSecp256k1PublicKey,
    EcdsaSecp256k1Signature, EddsaEd25519PublicKey, EddsaEd25519Signature, Expression, Hash,
    NonFungibleAddress, NonFungibleId, PreciseDecimal,
};
use std::convert::{TryFrom, TryInto};
use std::str::FromStr;
use transaction::manifest::ast::Value as AstValue;

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

use crate::address::Bech32Manager;
use crate::error::Error;
use crate::models::serde::*;

// ======
// Value
// ======

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq, Clone)]
#[serde(tag = "type")]

// TODO: Consider extending the value type to support all `ScryptoType`s even if they do not have
// a manifest representation. Their manifest compatibility can be evaluated at runtime in the
// `value_from_ast_value` and `ast_value_from_value` functions.
pub enum Value {
    Unit,
    Bool {
        value: bool,
    },

    U8 {
        #[serde_as(as = "DisplayFromStr")]
        value: u8,
    },
    U16 {
        #[serde_as(as = "DisplayFromStr")]
        value: u16,
    },
    U32 {
        #[serde_as(as = "DisplayFromStr")]
        value: u32,
    },
    U64 {
        #[serde_as(as = "DisplayFromStr")]
        value: u64,
    },
    U128 {
        #[serde_as(as = "DisplayFromStr")]
        value: u128,
    },

    I8 {
        #[serde_as(as = "DisplayFromStr")]
        value: i8,
    },
    I16 {
        #[serde_as(as = "DisplayFromStr")]
        value: i16,
    },
    I32 {
        #[serde_as(as = "DisplayFromStr")]
        value: i32,
    },
    I64 {
        #[serde_as(as = "DisplayFromStr")]
        value: i64,
    },
    I128 {
        #[serde_as(as = "DisplayFromStr")]
        value: i128,
    },

    String {
        value: String,
    },

    Struct {
        fields: Vec<Value>,
    },
    Enum {
        variant_name: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        fields: Option<Vec<Value>>,
    },
    Option {
        #[serde(default)]
        value: Box<Option<Value>>,
    },
    Result {
        value: Box<Result<Value, Value>>,
    },

    Array {
        element_type: ValueKind,
        elements: Vec<Value>,
    },
    Tuple {
        elements: Vec<Value>,
    },

    List {
        element_type: ValueKind,
        elements: Vec<Value>,
    },
    Set {
        element_type: ValueKind,
        elements: Vec<Value>,
    },
    Map {
        key_type: ValueKind,
        value_type: ValueKind,
        elements: Vec<Value>,
    },

    // Scrypto Values
    KeyValueStore {
        identifier: KeyValueStoreId,
    },

    Decimal {
        #[serde_as(as = "DisplayFromStr")]
        value: Decimal,
    },
    PreciseDecimal {
        #[serde_as(as = "DisplayFromStr")]
        value: PreciseDecimal,
    },

    Component {
        address: NetworkAwareComponentAddress,
    },

    ComponentAddress {
        address: NetworkAwareComponentAddress,
    },
    ResourceAddress {
        address: NetworkAwareResourceAddress,
    },
    PackageAddress {
        address: NetworkAwarePackageAddress,
    },

    Hash {
        #[serde_as(as = "DisplayFromStr")]
        value: Hash,
    },
    EcdsaSecp256k1PublicKey {
        #[serde_as(as = "DisplayFromStr")]
        public_key: EcdsaSecp256k1PublicKey,
    },
    EcdsaSecp256k1Signature {
        #[serde_as(as = "DisplayFromStr")]
        signature: EcdsaSecp256k1Signature,
    },
    EddsaEd25519PublicKey {
        #[serde_as(as = "DisplayFromStr")]
        public_key: EddsaEd25519PublicKey,
    },
    EddsaEd25519Signature {
        #[serde_as(as = "DisplayFromStr")]
        signature: EddsaEd25519Signature,
    },

    Bucket {
        identifier: Identifier,
    },
    Proof {
        identifier: Identifier,
    },
    Vault {
        identifier: VaultId,
    },
    NonFungibleId {
        #[serde_as(as = "DisplayFromStr")]
        value: NonFungibleId,
    },
    NonFungibleAddress {
        #[serde_as(as = "DisplayFromStr")]
        address: NonFungibleAddress,
    },

    Blob {
        #[serde_as(as = "DisplayFromStr")]
        hash: Blob,
    },
    Expression {
        #[serde_as(as = "DisplayFromStr")]
        value: Expression,
    },
}

impl Value {
    pub fn kind(&self) -> ValueKind {
        match self {
            Self::Unit => ValueKind::Unit,
            Self::Bool { .. } => ValueKind::Bool,

            Self::I8 { .. } => ValueKind::I8,
            Self::I16 { .. } => ValueKind::I16,
            Self::I32 { .. } => ValueKind::I32,
            Self::I64 { .. } => ValueKind::I64,
            Self::I128 { .. } => ValueKind::I128,

            Self::U8 { .. } => ValueKind::U8,
            Self::U16 { .. } => ValueKind::U16,
            Self::U32 { .. } => ValueKind::U32,
            Self::U64 { .. } => ValueKind::U64,
            Self::U128 { .. } => ValueKind::U128,

            Self::String { .. } => ValueKind::String,

            Self::Struct { .. } => ValueKind::Struct,
            Self::Enum { .. } => ValueKind::Enum,

            Self::Option { .. } => ValueKind::Option,
            Self::Array { .. } => ValueKind::Array,
            Self::Tuple { .. } => ValueKind::Tuple,
            Self::Result { .. } => ValueKind::Result,

            Self::List { .. } => ValueKind::List,
            Self::Set { .. } => ValueKind::Set,
            Self::Map { .. } => ValueKind::Map,

            Self::Decimal { .. } => ValueKind::Decimal,
            Self::PreciseDecimal { .. } => ValueKind::PreciseDecimal,

            Self::Component { .. } => ValueKind::Component,
            Self::PackageAddress { .. } => ValueKind::PackageAddress,
            Self::ComponentAddress { .. } => ValueKind::ComponentAddress,
            Self::ResourceAddress { .. } => ValueKind::ResourceAddress,

            Self::Hash { .. } => ValueKind::Hash,

            Self::Bucket { .. } => ValueKind::Bucket,
            Self::Proof { .. } => ValueKind::Proof,
            Self::Vault { .. } => ValueKind::Vault,

            Self::NonFungibleId { .. } => ValueKind::NonFungibleId,
            Self::NonFungibleAddress { .. } => ValueKind::NonFungibleAddress,

            Self::KeyValueStore { .. } => ValueKind::KeyValueStore,

            Self::EcdsaSecp256k1PublicKey { .. } => ValueKind::EcdsaSecp256k1PublicKey,
            Self::EcdsaSecp256k1Signature { .. } => ValueKind::EcdsaSecp256k1Signature,
            Self::EddsaEd25519PublicKey { .. } => ValueKind::EddsaEd25519PublicKey,
            Self::EddsaEd25519Signature { .. } => ValueKind::EddsaEd25519Signature,

            Self::Blob { .. } => ValueKind::Blob,
            Self::Expression { .. } => ValueKind::Expression,
        }
    }

    pub fn validate_kind(&self, expected_kind: ValueKind) -> Result<(), Error> {
        if self.kind() == expected_kind {
            Ok(())
        } else {
            Err(Error::InvalidType {
                expected_type: expected_kind,
                actual_type: self.kind(),
            })
        }
    }

    pub fn validate_if_collection(&self) -> Result<(), Error> {
        match self {
            Self::Array {
                element_type,
                elements,
            }
            | Self::List {
                element_type,
                elements,
            }
            | Self::Set {
                element_type,
                elements,
            } => {
                elements
                    .iter()
                    .map(|item| match item.validate_if_collection() {
                        Ok(_) => item.validate_kind(*element_type),
                        Err(error) => Err(error),
                    })
                    .collect::<Result<Vec<()>, _>>()?;
                Ok(())
            }
            Self::Tuple { elements } => {
                elements
                    .iter()
                    .map(|item| item.validate_if_collection())
                    .collect::<Result<Vec<()>, _>>()?;
                Ok(())
            }
            Self::Map {
                key_type,
                value_type,
                elements,
            } => {
                elements
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| i % 2 == 0)
                    .map(|(_, item)| match item.validate_if_collection() {
                        Ok(_) => item.validate_kind(*key_type),
                        Err(error) => Err(error),
                    })
                    .collect::<Result<Vec<()>, _>>()?;
                elements
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| i % 2 != 0)
                    .map(|(_, item)| match item.validate_if_collection() {
                        Ok(_) => item.validate_kind(*value_type),
                        Err(error) => Err(error),
                    })
                    .collect::<Result<Vec<()>, _>>()?;
                Ok(())
            }
            // Not a collection. No validation required.
            _ => Ok(()),
        }
    }

    pub fn validate_address_network_id(&self, expected_network_id: u8) -> Result<(), Error> {
        let network_id: u8 = match self {
            Self::Component { address } => address.network_id,
            Self::ComponentAddress { address } => address.network_id,
            Self::ResourceAddress { address } => address.network_id,
            Self::PackageAddress { address } => address.network_id,
            _ => return Ok(()),
        };
        if network_id == expected_network_id {
            Ok(())
        } else {
            Err(Error::NetworkMismatchError {
                expected: expected_network_id,
                found: network_id,
            })
        }
    }

    pub fn encode(&self) -> Result<Vec<u8>, Error> {
        Ok(encode_any(&sbor_value_from_value(self)?))
    }

    pub fn decode(bytes: &[u8], network_id: u8) -> Result<Self, Error> {
        Ok(value_from_sbor_value(&decode_any(bytes)?, network_id)?)
    }
}

// ==========
// ValueKind
// ==========

#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ValueKind {
    Unit,
    Bool,

    I8,
    I16,
    I32,
    I64,
    I128,

    U8,
    U16,
    U32,
    U64,
    U128,

    String,

    Struct,
    Enum,

    Option,
    Array,
    Tuple,
    Result,

    List,
    Set,
    Map,

    Decimal,
    PreciseDecimal,

    Component,
    PackageAddress,
    ComponentAddress,
    ResourceAddress,

    Hash,

    Bucket,
    Proof,
    Vault,

    NonFungibleId,
    NonFungibleAddress,

    KeyValueStore,

    EcdsaSecp256k1PublicKey,
    EcdsaSecp256k1Signature,
    EddsaEd25519PublicKey,
    EddsaEd25519Signature,

    Blob,
    Expression,
}

impl ValueKind {
    pub fn type_id(&self) -> u8 {
        match self {
            Self::Unit => TYPE_UNIT,
            Self::Bool => TYPE_BOOL,

            Self::U8 => TYPE_U8,
            Self::U16 => TYPE_U16,
            Self::U32 => TYPE_U32,
            Self::U64 => TYPE_U64,
            Self::U128 => TYPE_U128,

            Self::I8 => TYPE_I8,
            Self::I16 => TYPE_I16,
            Self::I32 => TYPE_I32,
            Self::I64 => TYPE_I64,
            Self::I128 => TYPE_I128,

            Self::String => TYPE_STRING,

            Self::Struct => TYPE_STRUCT,
            Self::Enum => TYPE_ENUM,

            Self::Option => TYPE_OPTION,
            Self::Array => TYPE_ARRAY,
            Self::Tuple => TYPE_TUPLE,
            Self::Result => TYPE_OPTION,

            Self::List => TYPE_LIST,
            Self::Set => TYPE_SET,
            Self::Map => TYPE_MAP,

            Self::KeyValueStore => ScryptoType::KeyValueStore.id(),

            Self::Decimal => ScryptoType::Decimal.id(),
            Self::PreciseDecimal => ScryptoType::PreciseDecimal.id(),

            Self::Component => ScryptoType::Component.id(),
            Self::PackageAddress => ScryptoType::PackageAddress.id(),
            Self::ResourceAddress => ScryptoType::ResourceAddress.id(),
            Self::ComponentAddress => ScryptoType::ComponentAddress.id(),

            Self::Hash => ScryptoType::Hash.id(),

            Self::Bucket => ScryptoType::Bucket.id(),
            Self::Proof => ScryptoType::Proof.id(),
            Self::Vault => ScryptoType::Vault.id(),

            Self::NonFungibleId => ScryptoType::NonFungibleId.id(),
            Self::NonFungibleAddress => ScryptoType::NonFungibleAddress.id(),

            Self::EcdsaSecp256k1PublicKey => ScryptoType::EcdsaSecp256k1PublicKey.id(),
            Self::EcdsaSecp256k1Signature => ScryptoType::EcdsaSecp256k1Signature.id(),
            Self::EddsaEd25519PublicKey => ScryptoType::EddsaEd25519PublicKey.id(),
            Self::EddsaEd25519Signature => ScryptoType::EddsaEd25519Signature.id(),

            Self::Blob => ScryptoType::Blob.id(),
            Self::Expression => ScryptoType::Expression.id(),
        }
    }

    pub fn from_type_id(type_id: u8) -> Result<Self, Error> {
        let value_kind: Self = match type_id {
            TYPE_UNIT => Self::Unit,
            TYPE_BOOL => Self::Bool,

            TYPE_U8 => Self::U8,
            TYPE_U16 => Self::U16,
            TYPE_U32 => Self::U32,
            TYPE_U64 => Self::U64,
            TYPE_U128 => Self::U128,

            TYPE_I8 => Self::I8,
            TYPE_I16 => Self::I16,
            TYPE_I32 => Self::I32,
            TYPE_I64 => Self::I64,
            TYPE_I128 => Self::I128,

            TYPE_STRING => Self::String,

            TYPE_STRUCT => Self::Struct,
            TYPE_ENUM => Self::Enum,

            TYPE_OPTION => Self::Option,
            TYPE_ARRAY => Self::Array,
            TYPE_TUPLE => Self::Tuple,
            TYPE_RESULT => Self::Result,

            TYPE_LIST => Self::List,
            TYPE_SET => Self::Set,
            TYPE_MAP => Self::Map,

            type_id => match ScryptoType::from_id(type_id) {
                Some(scrypto_type) => match scrypto_type {
                    ScryptoType::Decimal => Self::Decimal,
                    ScryptoType::PreciseDecimal => Self::PreciseDecimal,
                    ScryptoType::PackageAddress => Self::PackageAddress,
                    ScryptoType::ResourceAddress => Self::ResourceAddress,
                    ScryptoType::ComponentAddress => Self::ComponentAddress,
                    ScryptoType::Hash => Self::Hash,
                    ScryptoType::Bucket => Self::Bucket,
                    ScryptoType::Proof => Self::Proof,
                    ScryptoType::NonFungibleId => Self::NonFungibleId,
                    ScryptoType::NonFungibleAddress => Self::NonFungibleAddress,
                    ScryptoType::Component => Self::Component,
                    ScryptoType::Vault => Self::Vault,
                    ScryptoType::EcdsaSecp256k1PublicKey => Self::EcdsaSecp256k1PublicKey,
                    ScryptoType::EcdsaSecp256k1Signature => Self::EcdsaSecp256k1Signature,
                    ScryptoType::EddsaEd25519PublicKey => Self::EddsaEd25519PublicKey,
                    ScryptoType::EddsaEd25519Signature => Self::EddsaEd25519Signature,
                    ScryptoType::KeyValueStore => Self::KeyValueStore,
                    ScryptoType::Blob => Self::Blob,
                    ScryptoType::Expression => Self::Expression,
                },
                None => return Err(Error::UnknownTypeId { type_id }),
            },
        };
        Ok(value_kind)
    }
}

impl TryInto<transaction::manifest::ast::Type> for ValueKind {
    type Error = Error;

    fn try_into(self) -> Result<transaction::manifest::ast::Type, Self::Error> {
        let value_kind = match self {
            Self::Unit => transaction::manifest::ast::Type::Unit,

            Self::Bool => transaction::manifest::ast::Type::Bool,
            Self::I8 => transaction::manifest::ast::Type::I8,
            Self::I16 => transaction::manifest::ast::Type::I16,
            Self::I32 => transaction::manifest::ast::Type::I32,
            Self::I64 => transaction::manifest::ast::Type::I64,
            Self::I128 => transaction::manifest::ast::Type::I128,

            Self::U8 => transaction::manifest::ast::Type::U8,
            Self::U16 => transaction::manifest::ast::Type::U16,
            Self::U32 => transaction::manifest::ast::Type::U32,
            Self::U64 => transaction::manifest::ast::Type::U64,
            Self::U128 => transaction::manifest::ast::Type::U128,

            Self::String => transaction::manifest::ast::Type::String,

            Self::Struct => transaction::manifest::ast::Type::Struct,
            Self::Enum => transaction::manifest::ast::Type::Enum,

            Self::Option => transaction::manifest::ast::Type::Option,
            Self::Array => transaction::manifest::ast::Type::Array,
            Self::Tuple => transaction::manifest::ast::Type::Tuple,
            Self::Result => transaction::manifest::ast::Type::Result,

            Self::List => transaction::manifest::ast::Type::List,
            Self::Set => transaction::manifest::ast::Type::Set,
            Self::Map => transaction::manifest::ast::Type::Map,

            Self::Decimal => transaction::manifest::ast::Type::Decimal,
            Self::PreciseDecimal => transaction::manifest::ast::Type::PreciseDecimal,

            Self::PackageAddress => transaction::manifest::ast::Type::PackageAddress,
            Self::ComponentAddress => transaction::manifest::ast::Type::ComponentAddress,
            Self::ResourceAddress => transaction::manifest::ast::Type::ResourceAddress,

            Self::Hash => transaction::manifest::ast::Type::Hash,

            Self::Bucket => transaction::manifest::ast::Type::Bucket,
            Self::Proof => transaction::manifest::ast::Type::Proof,

            Self::NonFungibleId => transaction::manifest::ast::Type::NonFungibleId,
            Self::NonFungibleAddress => transaction::manifest::ast::Type::NonFungibleAddress,

            Self::Blob => transaction::manifest::ast::Type::Blob,
            Self::Expression => transaction::manifest::ast::Type::Expression,

            Self::Component
            | Self::Vault
            | Self::EcdsaSecp256k1PublicKey
            | Self::EcdsaSecp256k1Signature
            | Self::EddsaEd25519PublicKey
            | Self::EddsaEd25519Signature
            | Self::KeyValueStore => {
                return Err(Error::NoManifestRepresentation { kind: self.clone() })
            }
        };
        Ok(value_kind)
    }
}
impl From<transaction::manifest::ast::Type> for ValueKind {
    fn from(value: transaction::manifest::ast::Type) -> ValueKind {
        match value {
            transaction::manifest::ast::Type::Unit => Self::Unit,
            transaction::manifest::ast::Type::Bool => Self::Bool,

            transaction::manifest::ast::Type::I8 => Self::I8,
            transaction::manifest::ast::Type::I16 => Self::I16,
            transaction::manifest::ast::Type::I32 => Self::I32,
            transaction::manifest::ast::Type::I64 => Self::I64,
            transaction::manifest::ast::Type::I128 => Self::I128,
            transaction::manifest::ast::Type::U8 => Self::U8,
            transaction::manifest::ast::Type::U16 => Self::U16,
            transaction::manifest::ast::Type::U32 => Self::U32,
            transaction::manifest::ast::Type::U64 => Self::U64,
            transaction::manifest::ast::Type::U128 => Self::U128,

            transaction::manifest::ast::Type::String => Self::String,

            transaction::manifest::ast::Type::Struct => Self::Struct,
            transaction::manifest::ast::Type::Enum => Self::Enum,

            transaction::manifest::ast::Type::Option => Self::Option,
            transaction::manifest::ast::Type::Array => Self::Array,
            transaction::manifest::ast::Type::Tuple => Self::Tuple,
            transaction::manifest::ast::Type::Result => Self::Result,

            transaction::manifest::ast::Type::List => Self::List,
            transaction::manifest::ast::Type::Set => Self::Set,
            transaction::manifest::ast::Type::Map => Self::Map,

            transaction::manifest::ast::Type::Decimal => Self::Decimal,
            transaction::manifest::ast::Type::PreciseDecimal => Self::PreciseDecimal,

            transaction::manifest::ast::Type::PackageAddress => Self::PackageAddress,
            transaction::manifest::ast::Type::ComponentAddress => Self::ComponentAddress,
            transaction::manifest::ast::Type::ResourceAddress => Self::ResourceAddress,

            transaction::manifest::ast::Type::Hash => Self::Hash,

            transaction::manifest::ast::Type::Bucket => Self::Bucket,
            transaction::manifest::ast::Type::Proof => Self::Proof,

            transaction::manifest::ast::Type::NonFungibleId => Self::NonFungibleId,
            transaction::manifest::ast::Type::NonFungibleAddress => Self::NonFungibleAddress,

            transaction::manifest::ast::Type::Blob => Self::Blob,
            transaction::manifest::ast::Type::Expression => Self::Expression,
        }
    }
}

// ==================
// Value Conversions
// ==================

pub fn ast_value_from_value(
    value: &Value,
    bech32_manager: &Bech32Manager,
) -> Result<AstValue, Error> {
    let ast_value: AstValue = match value {
        Value::Unit => AstValue::Unit,
        Value::Bool { value } => AstValue::Bool(*value),

        Value::I8 { value } => AstValue::I8(*value),
        Value::I16 { value } => AstValue::I16(*value),
        Value::I32 { value } => AstValue::I32(*value),
        Value::I64 { value } => AstValue::I64(*value),
        Value::I128 { value } => AstValue::I128(*value),

        Value::U8 { value } => AstValue::U8(*value),
        Value::U16 { value } => AstValue::U16(*value),
        Value::U32 { value } => AstValue::U32(*value),
        Value::U64 { value } => AstValue::U64(*value),
        Value::U128 { value } => AstValue::U128(*value),

        Value::String { value } => AstValue::String(value.clone()),

        Value::Struct { fields } => AstValue::Struct(
            fields
                .iter()
                .map(|v| ast_value_from_value(v, bech32_manager))
                .collect::<Result<Vec<AstValue>, _>>()?,
        ),
        Value::Enum {
            variant_name,
            fields,
        } => AstValue::Enum(
            variant_name.clone(),
            fields
                .clone()
                .unwrap_or_default()
                .iter()
                .map(|v| ast_value_from_value(v, bech32_manager))
                .collect::<Result<Vec<AstValue>, _>>()?,
        ),
        Value::Option { value } => AstValue::Option(Box::new(match &**value {
            Some(value) => Some(ast_value_from_value(&value, bech32_manager)?),
            None => None,
        })),
        Value::Result { value } => AstValue::Result(Box::new(match &**value {
            Ok(value) => Ok(ast_value_from_value(&value, bech32_manager)?),
            Err(value) => Err(ast_value_from_value(&value, bech32_manager)?),
        })),

        Value::Array {
            element_type,
            elements,
        } => AstValue::Array((*element_type).try_into()?, {
            value.validate_if_collection()?;
            elements
                .iter()
                .map(|id| ast_value_from_value(id, bech32_manager))
                .collect::<Result<Vec<AstValue>, Error>>()
        }?),
        Value::Tuple { elements } => AstValue::Tuple(
            elements
                .iter()
                .map(|v| ast_value_from_value(v, bech32_manager))
                .collect::<Result<Vec<AstValue>, _>>()?,
        ),

        Value::List {
            element_type,
            elements,
        } => AstValue::List((*element_type).try_into()?, {
            value.validate_if_collection()?;
            elements
                .iter()
                .map(|id| ast_value_from_value(id, bech32_manager))
                .collect::<Result<Vec<AstValue>, Error>>()
        }?),
        Value::Set {
            element_type,
            elements,
        } => AstValue::Set((*element_type).try_into()?, {
            value.validate_if_collection()?;
            elements
                .iter()
                .map(|id| ast_value_from_value(id, bech32_manager))
                .collect::<Result<Vec<AstValue>, Error>>()
        }?),
        Value::Map {
            key_type: keys_type,
            value_type: values_type,
            elements,
        } => AstValue::Map(
            (*keys_type).try_into()?,
            (*values_type).try_into()?,
            elements
                .iter()
                .map(|v| ast_value_from_value(v, bech32_manager))
                .collect::<Result<Vec<AstValue>, _>>()?,
        ),

        Value::Decimal { value } => {
            AstValue::Decimal(Box::new(AstValue::String(value.to_string())))
        }
        Value::PreciseDecimal { value } => {
            AstValue::PreciseDecimal(Box::new(AstValue::String(value.to_string())))
        }

        Value::PackageAddress { address: value } => {
            AstValue::PackageAddress(Box::new(AstValue::String(
                bech32_manager
                    .encoder
                    .encode_package_address(&value.address),
            )))
        }
        Value::ComponentAddress { address: value } => {
            AstValue::ComponentAddress(Box::new(AstValue::String(
                bech32_manager
                    .encoder
                    .encode_component_address(&value.address),
            )))
        }
        Value::ResourceAddress { address: value } => {
            AstValue::ResourceAddress(Box::new(AstValue::String(
                bech32_manager
                    .encoder
                    .encode_resource_address(&value.address),
            )))
        }

        Value::Hash { value } => AstValue::Hash(Box::new(AstValue::String(value.to_string()))),
        Value::Bucket { identifier: value } => AstValue::Bucket(Box::new(match value {
            Identifier::String(string) => AstValue::String(string.clone()),
            Identifier::U32(number) => AstValue::U32(*number),
        })),
        Value::Proof { identifier: value } => AstValue::Proof(Box::new(match value {
            Identifier::String(string) => AstValue::String(string.clone()),
            Identifier::U32(number) => AstValue::U32(*number),
        })),

        Value::NonFungibleId { value } => {
            AstValue::NonFungibleId(Box::new(AstValue::String(value.to_string())))
        }
        Value::NonFungibleAddress { address } => {
            AstValue::NonFungibleAddress(Box::new(AstValue::String(address.to_string())))
        }

        Value::Blob { hash } => AstValue::Blob(Box::new(AstValue::String(hash.to_string()))),
        Value::Expression { value } => {
            AstValue::Expression(Box::new(AstValue::String(value.to_string())))
        }

        Value::Component { .. }
        | Value::Vault { .. }
        | Value::EcdsaSecp256k1PublicKey { .. }
        | Value::EcdsaSecp256k1Signature { .. }
        | Value::EddsaEd25519PublicKey { .. }
        | Value::EddsaEd25519Signature { .. }
        | Value::KeyValueStore { .. } => {
            return Err(Error::NoManifestRepresentation { kind: value.kind() })
        }
    };
    Ok(ast_value)
}

pub fn value_from_ast_value(
    ast_value: &AstValue,
    bech32_manager: &Bech32Manager,
) -> Result<Value, Error> {
    let value: Value = match ast_value {
        AstValue::Unit => Value::Unit,
        AstValue::Bool(value) => Value::Bool { value: *value },

        AstValue::I8(value) => Value::I8 { value: *value },
        AstValue::I16(value) => Value::I16 { value: *value },
        AstValue::I32(value) => Value::I32 { value: *value },
        AstValue::I64(value) => Value::I64 { value: *value },
        AstValue::I128(value) => Value::I128 { value: *value },

        AstValue::U8(value) => Value::U8 { value: *value },
        AstValue::U16(value) => Value::U16 { value: *value },
        AstValue::U32(value) => Value::U32 { value: *value },
        AstValue::U64(value) => Value::U64 { value: *value },
        AstValue::U128(value) => Value::U128 { value: *value },

        AstValue::String(value) => Value::String {
            value: value.clone(),
        },

        AstValue::Struct(fields) => Value::Struct {
            fields: fields
                .iter()
                .map(|v| value_from_ast_value(v, bech32_manager))
                .collect::<Result<Vec<Value>, _>>()?,
        },
        AstValue::Enum(variant_name, fields) => Value::Enum {
            variant_name: variant_name.clone(),
            fields: {
                let fields = fields
                    .iter()
                    .map(|v| value_from_ast_value(v, bech32_manager))
                    .collect::<Result<Vec<Value>, _>>()?;
                match fields.len() {
                    0 => None,
                    _ => Some(fields),
                }
            },
        },
        AstValue::Option(value) => Value::Option {
            value: Box::new(match &**value {
                Some(value) => Some(value_from_ast_value(&value, bech32_manager)?),
                None => None,
            }),
        },
        AstValue::Result(value) => Value::Result {
            value: Box::new(match &**value {
                Ok(value) => Ok(value_from_ast_value(&value, bech32_manager)?),
                Err(value) => Err(value_from_ast_value(&value, bech32_manager)?),
            }),
        },

        AstValue::Array(ast_type, elements) => Value::Array {
            element_type: (*ast_type).into(),
            elements: elements
                .iter()
                .map(|v| value_from_ast_value(v, bech32_manager))
                .collect::<Result<Vec<Value>, _>>()?,
        },
        AstValue::Tuple(elements) => Value::Tuple {
            elements: elements
                .iter()
                .map(|v| value_from_ast_value(v, bech32_manager))
                .collect::<Result<Vec<Value>, _>>()?,
        },

        AstValue::List(ast_type, elements) => Value::List {
            element_type: (*ast_type).into(),
            elements: elements
                .iter()
                .map(|v| value_from_ast_value(v, bech32_manager))
                .collect::<Result<Vec<Value>, _>>()?,
        },
        AstValue::Set(ast_type, elements) => Value::Set {
            element_type: (*ast_type).into(),
            elements: elements
                .iter()
                .map(|v| value_from_ast_value(v, bech32_manager))
                .collect::<Result<Vec<Value>, _>>()?,
        },
        AstValue::Map(key_ast_type, value_ast_type, elements) => Value::Map {
            key_type: (*key_ast_type).into(),
            value_type: (*value_ast_type).into(),
            elements: elements
                .iter()
                .map(|v| value_from_ast_value(v, bech32_manager))
                .collect::<Result<Vec<Value>, _>>()?,
        },

        AstValue::Decimal(value) => {
            if let AstValue::String(value) = &**value {
                Value::Decimal {
                    value: Decimal::from_str(&value)?,
                }
            } else {
                Err(Error::UnexpectedContents {
                    kind: ValueKind::Decimal,
                    expected: vec![ValueKind::String],
                    found: value.kind().into(),
                })?
            }
        }
        AstValue::PreciseDecimal(value) => {
            if let AstValue::String(value) = &**value {
                Value::PreciseDecimal {
                    value: PreciseDecimal::from_str(&value)?,
                }
            } else {
                Err(Error::UnexpectedContents {
                    kind: ValueKind::PreciseDecimal,
                    expected: vec![ValueKind::String],
                    found: value.kind().into(),
                })?
            }
        }

        AstValue::PackageAddress(value) => {
            if let AstValue::String(value) = &**value {
                Value::PackageAddress {
                    address: NetworkAwarePackageAddress {
                        network_id: bech32_manager.network_id(),
                        address: bech32_manager
                            .decoder
                            .validate_and_decode_package_address(value)?,
                    },
                }
            } else {
                Err(Error::UnexpectedContents {
                    kind: ValueKind::PackageAddress,
                    expected: vec![ValueKind::String],
                    found: value.kind().into(),
                })?
            }
        }
        AstValue::ComponentAddress(value) => {
            if let AstValue::String(value) = &**value {
                Value::ComponentAddress {
                    address: NetworkAwareComponentAddress {
                        network_id: bech32_manager.network_id(),
                        address: bech32_manager
                            .decoder
                            .validate_and_decode_component_address(value)?,
                    },
                }
            } else {
                Err(Error::UnexpectedContents {
                    kind: ValueKind::ComponentAddress,
                    expected: vec![ValueKind::String],
                    found: value.kind().into(),
                })?
            }
        }
        AstValue::ResourceAddress(value) => {
            if let AstValue::String(value) = &**value {
                Value::ResourceAddress {
                    address: NetworkAwareResourceAddress {
                        network_id: bech32_manager.network_id(),
                        address: bech32_manager
                            .decoder
                            .validate_and_decode_resource_address(value)?,
                    },
                }
            } else {
                Err(Error::UnexpectedContents {
                    kind: ValueKind::ResourceAddress,
                    expected: vec![ValueKind::String],
                    found: value.kind().into(),
                })?
            }
        }

        AstValue::Hash(value) => {
            if let AstValue::String(value) = &**value {
                Value::Hash {
                    value: Hash::from_str(&value)?,
                }
            } else {
                Err(Error::UnexpectedContents {
                    kind: ValueKind::Hash,
                    expected: vec![ValueKind::String],
                    found: value.kind().into(),
                })?
            }
        }

        AstValue::Bucket(value) => {
            if let AstValue::U32(value) = &**value {
                Value::Bucket {
                    identifier: Identifier::U32(*value),
                }
            } else if let AstValue::String(value) = &**value {
                Value::Bucket {
                    identifier: Identifier::String(value.clone()),
                }
            } else {
                Err(Error::UnexpectedContents {
                    kind: ValueKind::Bucket,
                    expected: vec![ValueKind::U32, ValueKind::String],
                    found: value.kind().into(),
                })?
            }
        }
        AstValue::Proof(value) => {
            if let AstValue::U32(value) = &**value {
                Value::Proof {
                    identifier: Identifier::U32(*value),
                }
            } else if let AstValue::String(value) = &**value {
                Value::Proof {
                    identifier: Identifier::String(value.clone()),
                }
            } else {
                Err(Error::UnexpectedContents {
                    kind: ValueKind::Proof,
                    expected: vec![ValueKind::U32, ValueKind::String],
                    found: value.kind().into(),
                })?
            }
        }

        AstValue::NonFungibleId(value) => {
            if let AstValue::String(value) = &**value {
                Value::NonFungibleId {
                    value: NonFungibleId::from_str(&value)?,
                }
            } else {
                Err(Error::UnexpectedContents {
                    kind: ValueKind::NonFungibleId,
                    expected: vec![ValueKind::String],
                    found: value.kind().into(),
                })?
            }
        }
        AstValue::NonFungibleAddress(value) => {
            if let AstValue::String(value) = &**value {
                Value::NonFungibleAddress {
                    address: NonFungibleAddress::from_str(&value)?,
                }
            } else {
                Err(Error::UnexpectedContents {
                    kind: ValueKind::NonFungibleAddress,
                    expected: vec![ValueKind::String],
                    found: value.kind().into(),
                })?
            }
        }

        AstValue::Blob(value) => {
            if let AstValue::String(value) = &**value {
                Value::Blob {
                    hash: Blob::from_str(&value)?,
                }
            } else {
                Err(Error::UnexpectedContents {
                    kind: ValueKind::Blob,
                    expected: vec![ValueKind::String],
                    found: value.kind().into(),
                })?
            }
        }
        AstValue::Expression(value) => {
            if let AstValue::String(value) = &**value {
                Value::Expression {
                    value: Expression::from_str(&value)?,
                }
            } else {
                Err(Error::UnexpectedContents {
                    kind: ValueKind::Expression,
                    expected: vec![ValueKind::String],
                    found: value.kind().into(),
                })?
            }
        }
    };
    Ok(value)
}

pub fn sbor_value_from_value(value: &Value) -> Result<SborValue, Error> {
    value.validate_if_collection()?;
    let value: SborValue = match value {
        Value::Unit => SborValue::Unit,
        Value::Bool { value } => SborValue::Bool { value: *value },

        Value::U8 { value } => SborValue::U8 { value: *value },
        Value::U16 { value } => SborValue::U16 { value: *value },
        Value::U32 { value } => SborValue::U32 { value: *value },
        Value::U64 { value } => SborValue::U64 { value: *value },
        Value::U128 { value } => SborValue::U128 { value: *value },

        Value::I8 { value } => SborValue::I8 { value: *value },
        Value::I16 { value } => SborValue::I16 { value: *value },
        Value::I32 { value } => SborValue::I32 { value: *value },
        Value::I64 { value } => SborValue::I64 { value: *value },
        Value::I128 { value } => SborValue::I128 { value: *value },

        Value::String { value } => SborValue::String {
            value: value.clone(),
        },

        Value::Struct { fields } => SborValue::Struct {
            fields: fields
                .into_iter()
                .map(sbor_value_from_value)
                .collect::<Result<Vec<_>, _>>()?,
        },
        Value::Enum {
            variant_name,
            fields,
        } => SborValue::Enum {
            name: variant_name.clone(),
            fields: fields
                .clone()
                .unwrap_or_default()
                .iter()
                .map(sbor_value_from_value)
                .collect::<Result<Vec<_>, _>>()?,
        },
        Value::Option { value } => SborValue::Option {
            value: Box::new(match &**value {
                Some(value) => Some(sbor_value_from_value(&value)?),
                None => None,
            }),
        },
        Value::Result { value } => SborValue::Result {
            value: Box::new(match &**value {
                Ok(value) => Ok(sbor_value_from_value(&value)?),
                Err(value) => Err(sbor_value_from_value(&value)?),
            }),
        },

        Value::Array {
            element_type,
            elements,
        } => SborValue::Array {
            element_type_id: element_type.type_id(),
            elements: elements
                .into_iter()
                .map(sbor_value_from_value)
                .collect::<Result<Vec<_>, _>>()?,
        },
        Value::Tuple { elements } => SborValue::Tuple {
            elements: elements
                .into_iter()
                .map(sbor_value_from_value)
                .collect::<Result<Vec<_>, _>>()?,
        },
        Value::List {
            element_type,
            elements,
        } => SborValue::List {
            element_type_id: element_type.type_id(),
            elements: elements
                .into_iter()
                .map(sbor_value_from_value)
                .collect::<Result<Vec<_>, _>>()?,
        },
        Value::Set {
            element_type,
            elements,
        } => SborValue::Set {
            element_type_id: element_type.type_id(),
            elements: elements
                .into_iter()
                .map(sbor_value_from_value)
                .collect::<Result<Vec<_>, _>>()?,
        },
        Value::Map {
            key_type,
            value_type,
            elements,
        } => SborValue::Map {
            key_type_id: key_type.type_id(),
            value_type_id: value_type.type_id(),
            elements: elements
                .into_iter()
                .map(sbor_value_from_value)
                .collect::<Result<Vec<_>, _>>()?,
        },

        Value::Decimal { value } => decode_any(&scrypto_encode(value))?,
        Value::PreciseDecimal { value } => decode_any(&scrypto_encode(value))?,

        Value::Component { address } => decode_any(&scrypto_encode(
            &scrypto::prelude::Component::from(address.address),
        ))?,
        Value::ComponentAddress { address } => decode_any(&scrypto_encode(&address.address))?,
        Value::ResourceAddress { address } => decode_any(&scrypto_encode(&address.address))?,
        Value::PackageAddress { address } => decode_any(&scrypto_encode(&address.address))?,

        Value::Hash { value } => decode_any(&scrypto_encode(value))?,
        Value::Bucket { identifier } => {
            if let Identifier::U32(value) = identifier {
                Ok(decode_any(&scrypto_encode(&scrypto::prelude::Bucket(
                    *value,
                )))?)
            } else {
                // TODO: Temporary error. Need a better way to deal with this.
                Err(Error::DecodeError(
                    "Unable to encode a Bucket with a string identifier".into(),
                ))
            }?
        }
        Value::Proof { identifier } => {
            if let Identifier::U32(value) = identifier {
                Ok(decode_any(&scrypto_encode(&scrypto::prelude::Proof(
                    *value,
                )))?)
            } else {
                // TODO: Temporary error. Need a better way to deal with this.
                Err(Error::DecodeError(
                    "Unable to encode a Proof with a string identifier".into(),
                ))
            }?
        }
        Value::Vault { identifier } => {
            decode_any(&scrypto_encode(&scrypto::prelude::Vault(identifier.0)))?
        }

        Value::NonFungibleId { value } => decode_any(&scrypto_encode(value))?,
        Value::NonFungibleAddress { address } => decode_any(&scrypto_encode(address))?,

        Value::KeyValueStore { identifier } => decode_any(&scrypto_encode(
            // TODO: This might not be correct due to the generics. Required more investigation.
            &scrypto::prelude::KeyValueStore {
                id: identifier.0,
                key: std::marker::PhantomData::<()>,
                value: std::marker::PhantomData::<()>,
            },
        ))?,

        Value::EcdsaSecp256k1PublicKey { public_key } => decode_any(&scrypto_encode(public_key))?,
        Value::EcdsaSecp256k1Signature { signature } => decode_any(&scrypto_encode(signature))?,

        Value::EddsaEd25519PublicKey { public_key } => decode_any(&scrypto_encode(public_key))?,
        Value::EddsaEd25519Signature { signature } => decode_any(&scrypto_encode(signature))?,

        Value::Blob { hash } => decode_any(&scrypto_encode(hash))?,
        Value::Expression { value } => decode_any(&scrypto_encode(value))?,
    };
    Ok(value)
}

pub fn value_from_sbor_value(value: &SborValue, network_id: u8) -> Result<Value, Error> {
    let value: Value = match value {
        SborValue::Unit => Value::Unit,
        SborValue::Bool { value } => Value::Bool { value: *value },

        SborValue::U8 { value } => Value::U8 { value: *value },
        SborValue::U16 { value } => Value::U16 { value: *value },
        SborValue::U32 { value } => Value::U32 { value: *value },
        SborValue::U64 { value } => Value::U64 { value: *value },
        SborValue::U128 { value } => Value::U128 { value: *value },

        SborValue::I8 { value } => Value::I8 { value: *value },
        SborValue::I16 { value } => Value::I16 { value: *value },
        SborValue::I32 { value } => Value::I32 { value: *value },
        SborValue::I64 { value } => Value::I64 { value: *value },
        SborValue::I128 { value } => Value::I128 { value: *value },

        SborValue::String { value } => Value::String {
            value: value.clone(),
        },

        SborValue::Struct { fields } => Value::Struct {
            fields: fields
                .into_iter()
                .map(|value| value_from_sbor_value(value, network_id))
                .collect::<Result<Vec<_>, _>>()?,
        },
        SborValue::Enum { name, fields } => Value::Enum {
            variant_name: name.clone(),
            fields: match fields.len() {
                0 => None,
                _ => Some(
                    fields
                        .clone()
                        .iter()
                        .map(|value| value_from_sbor_value(value, network_id))
                        .collect::<Result<Vec<_>, _>>()?,
                ),
            },
        },

        SborValue::Option { value } => Value::Option {
            value: Box::new(match &**value {
                Some(value) => Some(value_from_sbor_value(&value, network_id)?),
                None => None,
            }),
        },
        SborValue::Result { value } => Value::Result {
            value: Box::new(match &**value {
                Ok(value) => Ok(value_from_sbor_value(&value, network_id)?),
                Err(value) => Err(value_from_sbor_value(&value, network_id)?),
            }),
        },

        SborValue::Array {
            element_type_id,
            elements,
        } => Value::Array {
            element_type: ValueKind::from_type_id(*element_type_id)?,
            elements: elements
                .into_iter()
                .map(|value| value_from_sbor_value(value, network_id))
                .collect::<Result<Vec<_>, _>>()?,
        },
        SborValue::Tuple { elements } => Value::Tuple {
            elements: elements
                .into_iter()
                .map(|value| value_from_sbor_value(value, network_id))
                .collect::<Result<Vec<_>, _>>()?,
        },
        SborValue::List {
            element_type_id,
            elements,
        } => Value::List {
            element_type: ValueKind::from_type_id(*element_type_id)?,
            elements: elements
                .into_iter()
                .map(|value| value_from_sbor_value(value, network_id))
                .collect::<Result<Vec<_>, _>>()?,
        },
        SborValue::Set {
            element_type_id,
            elements,
        } => Value::Set {
            element_type: ValueKind::from_type_id(*element_type_id)?,
            elements: elements
                .into_iter()
                .map(|value| value_from_sbor_value(value, network_id))
                .collect::<Result<Vec<_>, _>>()?,
        },
        SborValue::Map {
            key_type_id,
            value_type_id,
            elements,
        } => Value::Map {
            key_type: ValueKind::from_type_id(*key_type_id)?,
            value_type: ValueKind::from_type_id(*value_type_id)?,
            elements: elements
                .into_iter()
                .map(|value| value_from_sbor_value(value, network_id))
                .collect::<Result<Vec<_>, _>>()?,
        },

        SborValue::Custom { type_id, bytes: _ } => match ScryptoType::from_id(*type_id) {
            Some(scrypto_type) => match scrypto_type {
                ScryptoType::Decimal => Value::Decimal {
                    value: scrypto_decode(&encode_any(&value))?,
                },
                ScryptoType::PreciseDecimal => Value::PreciseDecimal {
                    value: scrypto_decode(&encode_any(&value))?,
                },
                ScryptoType::Component => {
                    let component_address_ref: &[u8] =
                        &scrypto_decode::<scrypto::prelude::ComponentAddress>(&encode_any(&value))?
                            .to_vec();

                    Value::Component {
                        address: NetworkAwareComponentAddress {
                            network_id,
                            address: scrypto::prelude::ComponentAddress::try_from(
                                component_address_ref,
                            )?,
                        },
                    }
                }
                ScryptoType::PackageAddress => Value::PackageAddress {
                    address: NetworkAwarePackageAddress {
                        network_id,
                        address: scrypto_decode(&encode_any(&value))?,
                    },
                },
                ScryptoType::ComponentAddress => Value::ComponentAddress {
                    address: NetworkAwareComponentAddress {
                        network_id,
                        address: scrypto_decode(&encode_any(&value))?,
                    },
                },
                ScryptoType::ResourceAddress => Value::ResourceAddress {
                    address: NetworkAwareResourceAddress {
                        network_id,
                        address: scrypto_decode(&encode_any(&value))?,
                    },
                },
                ScryptoType::Hash => Value::Hash {
                    value: scrypto_decode(&encode_any(&value))?,
                },
                ScryptoType::Bucket => Value::Bucket {
                    identifier: Identifier::U32(
                        scrypto_decode::<scrypto::prelude::Bucket>(&encode_any(&value))?.0,
                    ),
                },
                ScryptoType::Proof => Value::Proof {
                    identifier: Identifier::U32(
                        scrypto_decode::<scrypto::prelude::Proof>(&encode_any(&value))?.0,
                    ),
                },
                ScryptoType::Vault => Value::Vault {
                    identifier: scrypto_decode::<scrypto::prelude::Vault>(&encode_any(&value))?
                        .0
                        .into(),
                },
                ScryptoType::NonFungibleId => Value::NonFungibleId {
                    value: scrypto_decode(&encode_any(&value))?,
                },
                ScryptoType::NonFungibleAddress => Value::NonFungibleAddress {
                    address: scrypto_decode(&encode_any(&value))?,
                },
                ScryptoType::KeyValueStore => Value::KeyValueStore {
                    identifier: scrypto_decode::<scrypto::prelude::Vault>(&encode_any(&value))?
                        .0
                        .into(),
                },

                ScryptoType::EcdsaSecp256k1PublicKey => Value::EcdsaSecp256k1PublicKey {
                    public_key: scrypto_decode(&encode_any(&value))?,
                },
                ScryptoType::EcdsaSecp256k1Signature => Value::EcdsaSecp256k1Signature {
                    signature: scrypto_decode(&encode_any(&value))?,
                },
                ScryptoType::EddsaEd25519PublicKey => Value::EddsaEd25519PublicKey {
                    public_key: scrypto_decode(&encode_any(&value))?,
                },
                ScryptoType::EddsaEd25519Signature => Value::EddsaEd25519Signature {
                    signature: scrypto_decode(&encode_any(&value))?,
                },

                ScryptoType::Blob => Value::Blob {
                    hash: scrypto_decode(&encode_any(&value))?,
                },
                ScryptoType::Expression => Value::Expression {
                    value: scrypto_decode(&encode_any(&value))?,
                },
            },
            None => return Err(Error::UnknownTypeId { type_id: *type_id }),
        },
    };
    value.validate_if_collection()?;
    Ok(value)
}

// ===========
// Unit Tests
// ===========

#[cfg(test)]
mod tests {
    use super::{Value, ValueKind};
    use crate::models::serde::{
        NetworkAwareComponentAddress, NetworkAwarePackageAddress, NetworkAwareResourceAddress,
    };
    use scrypto::prelude::*;

    macro_rules! test_value {
        ($string: expr, $value: expr) => {
            assert_serialization_matches($string, $value);
            assert_deserialization_matches($string, $value);
        };
    }

    fn assert_serialization_matches(string: &str, value: Value) {
        let serialized_string: String =
            serde_json::to_string(&value).expect("Serialization of trusted value failed");

        let string = string.replace("\n", "").replace(" ", "");
        let serialized_string = serialized_string.replace("\n", "").replace(" ", "");
        assert_eq!(string, serialized_string);
    }

    fn assert_deserialization_matches(string: &str, value: Value) {
        let deserialized_value: Value =
            serde_json::from_str(string).expect("Deserialization failed.");
        assert_eq!(value, deserialized_value);
    }

    #[test]
    fn value_serialization_and_deserialization_succeeds() {
        test_value! {
            r#"
            {
                "type": "U8",
                "value": "192"
            }
            "#,
            Value::U8 { value: 192 }
        };
        test_value! {
            r#"
            {
                "type": "U16",
                "value": "18947"
            }
            "#,
            Value::U16 { value: 18947 }
        };
        test_value! {
            r#"
            {
                "type": "U32",
                "value": "1144418947"
            }
            "#,
            Value::U32 { value: 1144418947 }
        };
        test_value! {
            r#"
            {
                "type": "U64",
                "value": "114441894733333"
            }
            "#,
            Value::U64 {
                value: 114441894733333,
            }
        };
        test_value! {
            r#"
            {
                "type": "U128",
                "value": "11444189334733333"
            }
            "#,
            Value::U128 {
                value: 11444189334733333,
            }
        };

        test_value! {
            r#"
            {
                "type": "I8",
                "value": "-100"
            }
            "#,
            Value::I8 { value: -100 }
        };
        test_value! {
            r#"
            {
                "type": "I16",
                "value": "-18947"
            }
            "#,
            Value::I16 { value: -18947 }
        };
        test_value! {
            r#"
            {
                "type": "I32",
                "value": "-1144418947"
            }
            "#,
            Value::I32 { value: -1144418947 }
        };
        test_value! {
            r#"
            {
                "type": "I64",
                "value": "-114441894733333"
            }
            "#,
            Value::I64 {
                value: -114441894733333,
            }
        };
        test_value! {
            r#"
            {
                "type": "I128",
                "value": "-11444189334733333"
            }
            "#,
            Value::I128 {
                value: -11444189334733333,
            }
        };

        test_value! {
            r#"
            {
                "type": "String",
                "value": "Hello World!"
            }
            "#,
            Value::String {
                value: "Hello World!".into(),
            }
        };

        test_value! {
            r#"
            {
                "type": "Struct",
                "fields": [
                    {
                        "type": "String",
                        "value": "Hello World!"
                    },
                    {
                        "type": "U8",
                        "value": "179"
                    }
                ]
            }
            "#,
            Value::Struct {
                fields: vec![
                    Value::String {
                        value: "Hello World!".into()
                    },
                    Value::U8 {
                        value: 179
                    }
                ]
            }
        };
        test_value! {
            r#"
            {
                "type": "Enum",
                "variant_name": "Component"
            }
            "#,
            Value::Enum {
                variant_name: "Component".into(),
                fields: None,
            }
        };
        test_value! {
            r#"
            {
                "type": "Enum",
                "variant_name": "Component",
                "fields": [
                    {
                        "type": "String",
                        "value": "Account"
                    }
                ]
            }
            "#,
            Value::Enum {
                variant_name: "Component".into(),
                fields: Some(vec![
                    Value::String { value: "Account".into() }
                ])
            }
        };

        // TODO: I'm really unhappy with the way that this is serialized. I want something that
        // looks more like the enum type. Something like:
        // {
        //      "type": "Option",
        //      "variant_name": "None",
        // }
        // Need to look into how this can be done with Serde.
        test_value! {
            r#"
            {
                "type": "Option",
                "value": null
            }
            "#,
            Value::Option {
                value: Box::new(None),
            }
        };

        // TODO: I'm really unhappy with the way that this is serialized. I want something that
        // looks more like the enum type. Something like:
        // {
        //      "type": "Option",
        //      "variant_name": "Some",
        //      "field": {
        //          "type": "String"
        //          "value": "Hello World!"
        //      }
        // }
        // Need to look into how this can be done with Serde.
        test_value! {
            r#"
            {
                "type": "Option",
                "value": {
                    "type": "String",
                    "value": "Hello World!"
                }
            }
            "#,
            Value::Option {
                value: Box::new(Some(Value::String {
                    value: "Hello World!".into(),
                })),
            }
        };

        // TODO: I'm really unhappy with the way that this is serialized. I want something that
        // looks more like the enum type. Something like:
        // {
        //      "type": "Result",
        //      "variant_name": "Ok",
        //      "field": {
        //          "type": "String"
        //          "value": "Hello World!"
        //      }
        // }
        // Need to look into how this can be done with Serde.
        test_value! {
            r#"
            {
                "type": "Result",
                "value": {
                    "Ok": {
                        "type": "String",
                        "value": "This is ok"
                    }
                }
            }
            "#,
            Value::Result {
                value: Box::new(Ok(Value::String {
                    value: "This is ok".into(),
                })),
            }
        };

        test_value! {
            r#"
            {
                "type": "Array",
                "element_type": "Decimal",
                "elements": [
                    {
                        "type": "Decimal",
                        "value": "192.38"
                    },
                    {
                        "type": "Decimal",
                        "value": "10012"
                    }
                ]
            }
            "#,
            Value::Array {
                element_type: ValueKind::Decimal,
                elements: vec![
                    Value::Decimal {
                        value: dec!("192.38"),
                    },
                    Value::Decimal {
                        value: dec!("10012"),
                    },
                ],
            }
        };

        test_value! {
            r#"
            {
                "type": "Tuple",
                "elements": [
                    {
                        "type": "Decimal",
                        "value": "192.38"
                    },
                    {
                        "type": "NonFungibleId",
                        "value": "3007100000000b3ce8b6056e62b902e029623df6df5c"
                    },
                    {
                        "type": "Bucket",
                        "identifier": "my_xrd_bucket"
                    }
                ]
            }
            "#,
            Value::Tuple {
                elements: vec![
                    Value::Decimal {
                        value: dec!("192.38")
                    },
                    Value::NonFungibleId {
                        value: NonFungibleId::from_str("3007100000000b3ce8b6056e62b902e029623df6df5c").unwrap()
                    },
                    Value::Bucket {
                        identifier: crate::models::serde::Identifier::String("my_xrd_bucket".into())
                    }
                ]
            }
        };

        test_value! {
            r#"
            {
                "type": "List",
                "element_type": "Decimal",
                "elements": [
                    {
                        "type": "Decimal",
                        "value": "192.38"
                    },
                    {
                        "type": "Decimal",
                        "value": "10012"
                    }
                ]
            }
            "#,
            Value::List {
                element_type: ValueKind::Decimal,
                elements: vec![
                    Value::Decimal {
                        value: dec!("192.38"),
                    },
                    Value::Decimal {
                        value: dec!("10012"),
                    },
                ],
            }
        };

        test_value! {
            r#"
            {
                "type": "Set",
                "element_type": "Decimal",
                "elements": [
                    {
                        "type": "Decimal",
                        "value": "192.38"
                    },
                    {
                        "type": "Decimal",
                        "value": "10012"
                    }
                ]
            }
            "#,
            Value::Set {
                element_type: ValueKind::Decimal,
                elements: vec![
                    Value::Decimal {
                        value: dec!("192.38"),
                    },
                    Value::Decimal {
                        value: dec!("10012"),
                    },
                ],
            }
        };

        test_value! {
            r#"
            {
                "type": "Map",
                "key_type": "String",
                "value_type": "Decimal",
                "elements": [
                    {
                        "type": "String",
                        "value": "Toyota Camry"
                    },
                    {
                        "type": "Decimal",
                        "value": "80000"
                    },
                    
                    {
                        "type": "String",
                        "value": "Ford Raptor"
                    },
                    {
                        "type": "Decimal",
                        "value": "170000"
                    }
                ]
            }
            "#,
            Value::Map {
                key_type: ValueKind::String,
                value_type: ValueKind::Decimal,
                elements: vec![
                    Value::String { value: "Toyota Camry".into() },
                    Value::Decimal { value: dec!("80000") },

                    Value::String { value: "Ford Raptor".into() },
                    Value::Decimal { value: dec!("170000") },
                ]
            }
        };

        test_value! {
            r#"
            {
                "type": "Decimal",
                "value": "100"
            }
            "#,
            Value::Decimal {
                value: dec!("100")
            }
        };
        test_value! {
            r#"
            {
                "type": "PreciseDecimal",
                "value": "100"
            }
            "#,
            Value::PreciseDecimal {
                value: pdec!("100")
            }
        };

        test_value! {
            r#"
            {
                "type": "Component",
                "address": "account_sim1qwssnwt0yzhzjydxj7u9uvnljtgaug23re8p32jrjecqajtsvr"
            }
            "#,
            Value::Component {
                address: NetworkAwareComponentAddress {
                    network_id: 0xf2,
                    address: scrypto::address::Bech32Decoder::new(&NetworkDefinition::simulator())
                        .validate_and_decode_component_address("account_sim1qwssnwt0yzhzjydxj7u9uvnljtgaug23re8p32jrjecqajtsvr")
                        .expect("Decoding of a trusted address string failed")
                }
            }
        };
        test_value! {
            r#"
            {
                "type": "ComponentAddress",
                "address": "account_sim1qwssnwt0yzhzjydxj7u9uvnljtgaug23re8p32jrjecqajtsvr"
            }
            "#,
            Value::ComponentAddress {
                address: NetworkAwareComponentAddress {
                    network_id: 0xf2,
                    address: scrypto::address::Bech32Decoder::new(&NetworkDefinition::simulator())
                        .validate_and_decode_component_address("account_sim1qwssnwt0yzhzjydxj7u9uvnljtgaug23re8p32jrjecqajtsvr")
                        .expect("Decoding of a trusted address string failed")
                }
            }
        };
        test_value! {
            r#"
            {
                "type": "PackageAddress",
                "address": "package_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqsnznk7n"
            }
            "#,
            Value::PackageAddress {
                address: NetworkAwarePackageAddress {
                    network_id: 0xf2,
                    address: scrypto::address::Bech32Decoder::new(&NetworkDefinition::simulator())
                        .validate_and_decode_package_address("package_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqsnznk7n")
                        .expect("Decoding of a trusted address string failed")
                }
            }
        };
        test_value! {
            r#"
            {
                "type": "ResourceAddress",
                "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqshxgp7h"
            }
            "#,
            Value::ResourceAddress {
                address: NetworkAwareResourceAddress {
                    network_id: 0xf2,
                    address: scrypto::address::Bech32Decoder::new(&NetworkDefinition::simulator())
                        .validate_and_decode_resource_address("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqshxgp7h")
                        .expect("Decoding of a trusted address string failed")
                }
            }
        };

        test_value! {
            r#"
            {
                "type": "Hash",
                "value": "910edb2dabf107c7628ecdb9126535676d61bc39a843475f3057d809bfd2d65d"
            }
            "#,
            Value::Hash {
                value: Hash::from_str("910edb2dabf107c7628ecdb9126535676d61bc39a843475f3057d809bfd2d65d").unwrap()
            }
        };

        test_value! {
            r#"
            {
                "type": "Bucket",
                "identifier": 192
            }
            "#,
            Value::Bucket {
                identifier: crate::models::serde::Identifier::U32(192)
            }
        };
        test_value! {
            r#"
            {
                "type": "Bucket",
                "identifier": "HelloBucket"
            }
            "#,
            Value::Bucket {
                identifier: crate::models::serde::Identifier::String("HelloBucket".into())
            }
        };

        test_value! {
            r#"
            {
                "type": "Proof",
                "identifier": 192
            }
            "#,
            Value::Proof {
                identifier: crate::models::serde::Identifier::U32(192)
            }
        };
        test_value! {
            r#"
            {
                "type": "Proof",
                "identifier": "HelloProof"
            }
            "#,
            Value::Proof {
                identifier: crate::models::serde::Identifier::String("HelloProof".into())
            }
        };

        test_value! {
            r#"
            {
                "type": "NonFungibleId",
                "value": "3007100000000b3ce8b6056e62b902e029623df6df5c"
            }
            "#,
            Value::NonFungibleId {
                value: NonFungibleId::from_str("3007100000000b3ce8b6056e62b902e029623df6df5c").unwrap()
            }
        };
    }

    #[test]
    fn non_collection_validation_succeeds() {
        // Arrange
        let value: Value = Value::Bool { value: false };

        // Act
        let result: Result<(), crate::error::Error> = value.validate_if_collection();

        // Assert
        assert!(matches!(result, Ok(())))
    }

    #[test]
    fn array_of_decimals_validation_succeeds() {
        // Arrange
        let value: Value = Value::Array {
            element_type: ValueKind::Decimal,
            elements: vec![
                Value::Decimal { value: dec!("20") },
                Value::Decimal { value: dec!("100") },
                Value::Decimal {
                    value: dec!("192.31"),
                },
            ],
        };

        // Act
        let result: Result<(), crate::error::Error> = value.validate_if_collection();

        // Assert
        assert!(matches!(result, Ok(())))
    }

    #[test]
    fn array_of_decimal_and_precise_decimal_validation_fails() {
        // Arrange
        let value: Value = Value::Array {
            element_type: ValueKind::Decimal,
            elements: vec![
                Value::Decimal { value: dec!("20") },
                Value::Decimal { value: dec!("100") },
                Value::Decimal {
                    value: dec!("192.31"),
                },
                Value::PreciseDecimal {
                    value: pdec!("192.31"),
                },
            ],
        };

        // Act
        let result: Result<(), crate::error::Error> = value.validate_if_collection();

        // Assert
        assert!(matches!(
            result,
            Err(crate::error::Error::InvalidType {
                expected_type: ValueKind::Decimal,
                actual_type: ValueKind::PreciseDecimal
            })
        ))
    }

    #[test]
    fn validation_of_deeply_nested_tuple_with_non_matching_types_fails() {
        // Arrange
        let value: Value = Value::Tuple {
            elements: vec![
                Value::Decimal { value: dec!("10") },
                Value::PreciseDecimal { value: pdec!("10") },
                Value::String {
                    value: "Hello World!".into(),
                },
                Value::Tuple {
                    elements: vec![
                        Value::Decimal { value: dec!("10") },
                        Value::PreciseDecimal { value: pdec!("10") },
                        Value::String {
                            value: "Hello World!".into(),
                        },
                        Value::Tuple {
                            elements: vec![
                                Value::Decimal { value: dec!("10") },
                                Value::PreciseDecimal { value: pdec!("10") },
                                Value::String {
                                    value: "Hello World!".into(),
                                },
                                Value::Tuple {
                                    elements: vec![
                                        Value::Decimal { value: dec!("10") },
                                        Value::PreciseDecimal { value: pdec!("10") },
                                        Value::String {
                                            value: "Hello World!".into(),
                                        },
                                        Value::Array {
                                            element_type: ValueKind::Decimal,
                                            elements: vec![
                                                Value::Decimal { value: dec!("20") },
                                                Value::Decimal { value: dec!("100") },
                                                Value::Decimal {
                                                    value: dec!("192.31"),
                                                },
                                                Value::PreciseDecimal {
                                                    value: pdec!("192.31"),
                                                },
                                            ],
                                        },
                                    ],
                                },
                            ],
                        },
                    ],
                },
            ],
        };

        // Act
        let result: Result<(), crate::error::Error> = value.validate_if_collection();

        // Assert
        assert!(matches!(result, Err(_)))
    }

    #[test]
    fn validation_of_valid_map_succeeds() {
        // Arrange
        let value: Value = Value::Map {
            key_type: ValueKind::String,
            value_type: ValueKind::Decimal,
            elements: vec![
                Value::String {
                    value: "Toyota Camry".into(),
                },
                Value::Decimal {
                    value: dec!("80000"),
                },
                Value::String {
                    value: "Ford Raptor".into(),
                },
                Value::Decimal {
                    value: dec!("170000"),
                },
            ],
        };

        // Act
        let result: Result<(), crate::error::Error> = value.validate_if_collection();

        // Assert
        assert!(matches!(result, Ok(())))
    }
}
