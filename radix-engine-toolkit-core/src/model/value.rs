//! This module implements the [Value] struct as well as all of its related methods for conversion
//! and validation.

use radix_transaction::manifest::ast::Value as AstValue;
use sbor::type_id::*;
use scrypto::data::ScryptoCustomTypeId;
use scrypto::prelude::{
    scrypto_decode, scrypto_encode, Blob, Decimal, EcdsaSecp256k1PublicKey,
    EcdsaSecp256k1Signature, EddsaEd25519PublicKey, EddsaEd25519Signature, Expression, Hash,
    NonFungibleAddress, NonFungibleId, PreciseDecimal, ScryptoCustomValue, ScryptoValue,
};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr, TryFromInto};

use crate::error::Error;
use crate::model::address::*;
use crate::model::identifier::{BucketId, Identifier, ProofId};
use crate::traits::ValidateWithContext;

use super::NodeIdentifier;

// ======
// Value
// ======

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq, Clone)]
#[serde(tag = "type")]
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

    Enum {
        variant: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        fields: Option<Vec<Value>>,
    },

    Array {
        element_type: ValueKind,
        elements: Vec<Value>,
    },
    Tuple {
        elements: Vec<Value>,
    },

    // Scrypto Values
    KeyValueStore {
        identifier: NodeIdentifier,
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
        identifier: NodeIdentifier,
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
    SystemAddress {
        address: NetworkAwareSystemAddress,
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
        identifier: BucketId,
    },
    Proof {
        identifier: ProofId,
    },
    Vault {
        identifier: NodeIdentifier,
    },
    NonFungibleId {
        #[serde(flatten)]
        #[serde_as(as = "TryFromInto<crate::model::helper::NonFungibleIdProxy>")]
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

            Self::Enum { .. } => ValueKind::Enum,

            Self::Array { .. } => ValueKind::Array,
            Self::Tuple { .. } => ValueKind::Tuple,

            Self::Decimal { .. } => ValueKind::Decimal,
            Self::PreciseDecimal { .. } => ValueKind::PreciseDecimal,

            Self::Component { .. } => ValueKind::Component,
            Self::PackageAddress { .. } => ValueKind::PackageAddress,
            Self::ComponentAddress { .. } => ValueKind::ComponentAddress,
            Self::ResourceAddress { .. } => ValueKind::ResourceAddress,
            Self::SystemAddress { .. } => ValueKind::SystemAddress,

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

    // ===========
    // Validation
    // ===========

    fn validate_kind(&self, expected_kind: ValueKind) -> Result<(), Error> {
        if self.kind() == expected_kind {
            Ok(())
        } else {
            Err(Error::InvalidType {
                expected_types: vec![expected_kind],
                actual_type: self.kind(),
            })
        }
    }

    fn validate_if_collection(&self) -> Result<(), Error> {
        match self {
            Self::Array {
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
            // Not a collection. No validation required.
            _ => Ok(()),
        }
    }

    fn validate_address_network_id(&self, expected_network_id: u8) -> Result<(), Error> {
        let network_id = match self {
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

    // ============
    // Conversions
    // ============

    pub fn from_ast_value(ast_value: &AstValue, bech32_coder: &Bech32Coder) -> Result<Self, Error> {
        let value = match ast_value {
            AstValue::Unit => Self::Unit,
            AstValue::Bool(value) => Self::Bool { value: *value },

            AstValue::I8(value) => Self::I8 { value: *value },
            AstValue::I16(value) => Self::I16 { value: *value },
            AstValue::I32(value) => Self::I32 { value: *value },
            AstValue::I64(value) => Self::I64 { value: *value },
            AstValue::I128(value) => Self::I128 { value: *value },

            AstValue::U8(value) => Self::U8 { value: *value },
            AstValue::U16(value) => Self::U16 { value: *value },
            AstValue::U32(value) => Self::U32 { value: *value },
            AstValue::U64(value) => Self::U64 { value: *value },
            AstValue::U128(value) => Self::U128 { value: *value },

            AstValue::String(value) => Self::String {
                value: value.clone(),
            },

            AstValue::Enum(variant, fields) => Self::Enum {
                variant: variant.clone(),
                fields: {
                    let fields = fields
                        .iter()
                        .map(|v| Self::from_ast_value(v, bech32_coder))
                        .collect::<Result<Vec<Value>, _>>()?;
                    match fields.len() {
                        0 => None,
                        _ => Some(fields),
                    }
                },
            },

            AstValue::Array(ast_type, elements) => Self::Array {
                element_type: (*ast_type).into(),
                elements: elements
                    .iter()
                    .map(|v| Self::from_ast_value(v, bech32_coder))
                    .collect::<Result<Vec<Value>, _>>()?,
            },
            AstValue::Tuple(elements) => Self::Tuple {
                elements: elements
                    .iter()
                    .map(|v| Self::from_ast_value(v, bech32_coder))
                    .collect::<Result<Vec<Value>, _>>()?,
            },

            AstValue::Decimal(value) => {
                if let AstValue::String(value) = &**value {
                    Self::Decimal {
                        value: value.parse()?,
                    }
                } else {
                    Err(Error::UnexpectedContents {
                        kind_being_parsed: ValueKind::Decimal,
                        allowed_children_kinds: vec![ValueKind::String],
                        found_child_kind: value.kind().into(),
                    })?
                }
            }
            AstValue::PreciseDecimal(value) => {
                if let AstValue::String(value) = &**value {
                    Self::PreciseDecimal {
                        value: value.parse()?,
                    }
                } else {
                    Err(Error::UnexpectedContents {
                        kind_being_parsed: ValueKind::PreciseDecimal,
                        allowed_children_kinds: vec![ValueKind::String],
                        found_child_kind: value.kind().into(),
                    })?
                }
            }

            AstValue::Component(value) => {
                if let AstValue::String(value) = &**value {
                    Self::Component {
                        identifier: value.parse()?,
                    }
                } else {
                    Err(Error::UnexpectedContents {
                        kind_being_parsed: ValueKind::Component,
                        allowed_children_kinds: vec![ValueKind::String],
                        found_child_kind: value.kind().into(),
                    })?
                }
            }
            AstValue::PackageAddress(value) => {
                if let AstValue::String(value) = &**value {
                    Self::PackageAddress {
                        address: NetworkAwarePackageAddress {
                            network_id: bech32_coder.network_id(),
                            address: bech32_coder
                                .decoder
                                .validate_and_decode_package_address(value)?,
                        },
                    }
                } else {
                    Err(Error::UnexpectedContents {
                        kind_being_parsed: ValueKind::PackageAddress,
                        allowed_children_kinds: vec![ValueKind::String],
                        found_child_kind: value.kind().into(),
                    })?
                }
            }
            AstValue::ComponentAddress(value) => {
                if let AstValue::String(value) = &**value {
                    Self::ComponentAddress {
                        address: NetworkAwareComponentAddress {
                            network_id: bech32_coder.network_id(),
                            address: bech32_coder
                                .decoder
                                .validate_and_decode_component_address(value)?,
                        },
                    }
                } else {
                    Err(Error::UnexpectedContents {
                        kind_being_parsed: ValueKind::ComponentAddress,
                        allowed_children_kinds: vec![ValueKind::String],
                        found_child_kind: value.kind().into(),
                    })?
                }
            }
            AstValue::ResourceAddress(value) => {
                if let AstValue::String(value) = &**value {
                    Self::ResourceAddress {
                        address: NetworkAwareResourceAddress {
                            network_id: bech32_coder.network_id(),
                            address: bech32_coder
                                .decoder
                                .validate_and_decode_resource_address(value)?,
                        },
                    }
                } else {
                    Err(Error::UnexpectedContents {
                        kind_being_parsed: ValueKind::ResourceAddress,
                        allowed_children_kinds: vec![ValueKind::String],
                        found_child_kind: value.kind().into(),
                    })?
                }
            }
            AstValue::SystemAddress(value) => {
                if let AstValue::String(value) = &**value {
                    Self::SystemAddress {
                        address: NetworkAwareSystemAddress {
                            network_id: bech32_coder.network_id(),
                            address: bech32_coder
                                .decoder
                                .validate_and_decode_system_address(value)?,
                        },
                    }
                } else {
                    Err(Error::UnexpectedContents {
                        kind_being_parsed: ValueKind::SystemAddress,
                        allowed_children_kinds: vec![ValueKind::String],
                        found_child_kind: value.kind().into(),
                    })?
                }
            }

            AstValue::Hash(value) => {
                if let AstValue::String(value) = &**value {
                    Self::Hash {
                        value: value.parse()?,
                    }
                } else {
                    Err(Error::UnexpectedContents {
                        kind_being_parsed: ValueKind::Hash,
                        allowed_children_kinds: vec![ValueKind::String],
                        found_child_kind: value.kind().into(),
                    })?
                }
            }

            AstValue::Bucket(value) => {
                if let AstValue::U32(value) = &**value {
                    Self::Bucket {
                        identifier: Identifier::U32(*value).into(),
                    }
                } else if let AstValue::String(value) = &**value {
                    Self::Bucket {
                        identifier: Identifier::String(value.clone()).into(),
                    }
                } else {
                    Err(Error::UnexpectedContents {
                        kind_being_parsed: ValueKind::Bucket,
                        allowed_children_kinds: vec![ValueKind::U32, ValueKind::String],
                        found_child_kind: value.kind().into(),
                    })?
                }
            }
            AstValue::Proof(value) => {
                if let AstValue::U32(value) = &**value {
                    Self::Proof {
                        identifier: Identifier::U32(*value).into(),
                    }
                } else if let AstValue::String(value) = &**value {
                    Self::Proof {
                        identifier: Identifier::String(value.clone()).into(),
                    }
                } else {
                    Err(Error::UnexpectedContents {
                        kind_being_parsed: ValueKind::Proof,
                        allowed_children_kinds: vec![ValueKind::U32, ValueKind::String],
                        found_child_kind: value.kind().into(),
                    })?
                }
            }

            AstValue::NonFungibleId(value) => {
                if let AstValue::String(value) = &**value {
                    Self::NonFungibleId {
                        value: value.parse()?,
                    }
                } else {
                    Err(Error::UnexpectedContents {
                        kind_being_parsed: ValueKind::NonFungibleId,
                        allowed_children_kinds: vec![ValueKind::String],
                        found_child_kind: value.kind().into(),
                    })?
                }
            }
            AstValue::NonFungibleAddress(value) => {
                if let AstValue::String(value) = &**value {
                    Self::NonFungibleAddress {
                        address: value.parse()?,
                    }
                } else {
                    Err(Error::UnexpectedContents {
                        kind_being_parsed: ValueKind::NonFungibleAddress,
                        allowed_children_kinds: vec![ValueKind::String],
                        found_child_kind: value.kind().into(),
                    })?
                }
            }

            AstValue::Blob(value) => {
                if let AstValue::String(value) = &**value {
                    Self::Blob {
                        hash: value.parse()?,
                    }
                } else {
                    Err(Error::UnexpectedContents {
                        kind_being_parsed: ValueKind::Blob,
                        allowed_children_kinds: vec![ValueKind::String],
                        found_child_kind: value.kind().into(),
                    })?
                }
            }
            AstValue::Expression(value) => {
                if let AstValue::String(value) = &**value {
                    Self::Expression {
                        value: value.parse()?,
                    }
                } else {
                    Err(Error::UnexpectedContents {
                        kind_being_parsed: ValueKind::Expression,
                        allowed_children_kinds: vec![ValueKind::String],
                        found_child_kind: value.kind().into(),
                    })?
                }
            }

            AstValue::Vault(value) => {
                if let AstValue::String(value) = &**value {
                    Self::Vault {
                        identifier: value.parse()?,
                    }
                } else {
                    Err(Error::UnexpectedContents {
                        kind_being_parsed: ValueKind::Vault,
                        allowed_children_kinds: vec![ValueKind::String],
                        found_child_kind: value.kind().into(),
                    })?
                }
            }
            AstValue::KeyValueStore(value) => {
                if let AstValue::String(value) = &**value {
                    Self::KeyValueStore {
                        identifier: value.parse()?,
                    }
                } else {
                    Err(Error::UnexpectedContents {
                        kind_being_parsed: ValueKind::KeyValueStore,
                        allowed_children_kinds: vec![ValueKind::String],
                        found_child_kind: value.kind().into(),
                    })?
                }
            }
            AstValue::EcdsaSecp256k1PublicKey(value) => {
                if let AstValue::String(value) = &**value {
                    Self::EcdsaSecp256k1PublicKey {
                        public_key: value.parse()?,
                    }
                } else {
                    Err(Error::UnexpectedContents {
                        kind_being_parsed: ValueKind::EcdsaSecp256k1PublicKey,
                        allowed_children_kinds: vec![ValueKind::String],
                        found_child_kind: value.kind().into(),
                    })?
                }
            }
            AstValue::EcdsaSecp256k1Signature(value) => {
                if let AstValue::String(value) = &**value {
                    Self::EcdsaSecp256k1Signature {
                        signature: value.parse()?,
                    }
                } else {
                    Err(Error::UnexpectedContents {
                        kind_being_parsed: ValueKind::EcdsaSecp256k1Signature,
                        allowed_children_kinds: vec![ValueKind::String],
                        found_child_kind: value.kind().into(),
                    })?
                }
            }
            AstValue::EddsaEd25519PublicKey(value) => {
                if let AstValue::String(value) = &**value {
                    Self::EddsaEd25519PublicKey {
                        public_key: value.parse()?,
                    }
                } else {
                    Err(Error::UnexpectedContents {
                        kind_being_parsed: ValueKind::EddsaEd25519PublicKey,
                        allowed_children_kinds: vec![ValueKind::String],
                        found_child_kind: value.kind().into(),
                    })?
                }
            }
            AstValue::EddsaEd25519Signature(value) => {
                if let AstValue::String(value) = &**value {
                    Self::EddsaEd25519Signature {
                        signature: value.parse()?,
                    }
                } else {
                    Err(Error::UnexpectedContents {
                        kind_being_parsed: ValueKind::EddsaEd25519Signature,
                        allowed_children_kinds: vec![ValueKind::String],
                        found_child_kind: value.kind().into(),
                    })?
                }
            }
        };
        Ok(value)
    }

    pub fn to_ast_value(&self, bech32_coder: &Bech32Coder) -> Result<AstValue, Error> {
        let ast_value = match self {
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

            Value::Enum { variant, fields } => AstValue::Enum(
                variant.clone(),
                fields
                    .clone()
                    .unwrap_or_default()
                    .iter()
                    .map(|v| v.to_ast_value(bech32_coder))
                    .collect::<Result<Vec<AstValue>, _>>()?,
            ),

            Value::Array {
                element_type,
                elements,
            } => {
                self.validate_if_collection()?;
                AstValue::Array(
                    (*element_type).into(),
                    elements
                        .iter()
                        .map(|id| id.to_ast_value(bech32_coder))
                        .collect::<Result<Vec<AstValue>, Error>>()?,
                )
            }
            Value::Tuple { elements } => AstValue::Tuple(
                elements
                    .iter()
                    .map(|v| v.to_ast_value(bech32_coder))
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
                    bech32_coder
                        .encoder
                        .encode_package_address_to_string(&value.address),
                )))
            }
            Value::ComponentAddress { address: value } => {
                AstValue::ComponentAddress(Box::new(AstValue::String(
                    bech32_coder
                        .encoder
                        .encode_component_address_to_string(&value.address),
                )))
            }
            Value::ResourceAddress { address: value } => {
                AstValue::ResourceAddress(Box::new(AstValue::String(
                    bech32_coder
                        .encoder
                        .encode_resource_address_to_string(&value.address),
                )))
            }
            Value::SystemAddress { address: value } => {
                AstValue::SystemAddress(Box::new(AstValue::String(
                    bech32_coder
                        .encoder
                        .encode_system_address_to_string(&value.address),
                )))
            }

            Value::Hash { value } => AstValue::Hash(Box::new(AstValue::String(value.to_string()))),
            Value::Bucket { identifier } => AstValue::Bucket(Box::new(match identifier.0 {
                Identifier::String(ref string) => AstValue::String(string.clone()),
                Identifier::U32(number) => AstValue::U32(number),
            })),
            Value::Proof { identifier } => AstValue::Proof(Box::new(match identifier.0 {
                Identifier::String(ref string) => AstValue::String(string.clone()),
                Identifier::U32(number) => AstValue::U32(number),
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

            Value::Component { identifier } => {
                AstValue::Component(Box::new(AstValue::String(identifier.to_string())))
            }
            Value::Vault { identifier } => {
                AstValue::Vault(Box::new(AstValue::String(identifier.to_string())))
            }
            Value::KeyValueStore { identifier } => {
                AstValue::KeyValueStore(Box::new(AstValue::String(identifier.to_string())))
            }

            Value::EcdsaSecp256k1PublicKey { public_key } => AstValue::EcdsaSecp256k1PublicKey(
                Box::new(AstValue::String(public_key.to_string())),
            ),
            Value::EcdsaSecp256k1Signature { signature } => {
                AstValue::EcdsaSecp256k1Signature(Box::new(AstValue::String(signature.to_string())))
            }

            Value::EddsaEd25519PublicKey { public_key } => {
                AstValue::EddsaEd25519PublicKey(Box::new(AstValue::String(public_key.to_string())))
            }
            Value::EddsaEd25519Signature { signature } => {
                AstValue::EddsaEd25519Signature(Box::new(AstValue::String(signature.to_string())))
            }
        };
        Ok(ast_value)
    }

    fn to_scrypto_value(&self) -> Result<ScryptoValue, Error> {
        let scrypto_value = match self {
            Value::Unit => ScryptoValue::Unit,
            Value::Bool { value } => ScryptoValue::Bool { value: *value },

            Value::U8 { value } => ScryptoValue::U8 { value: *value },
            Value::U16 { value } => ScryptoValue::U16 { value: *value },
            Value::U32 { value } => ScryptoValue::U32 { value: *value },
            Value::U64 { value } => ScryptoValue::U64 { value: *value },
            Value::U128 { value } => ScryptoValue::U128 { value: *value },

            Value::I8 { value } => ScryptoValue::I8 { value: *value },
            Value::I16 { value } => ScryptoValue::I16 { value: *value },
            Value::I32 { value } => ScryptoValue::I32 { value: *value },
            Value::I64 { value } => ScryptoValue::I64 { value: *value },
            Value::I128 { value } => ScryptoValue::I128 { value: *value },

            Value::String { value } => ScryptoValue::String {
                value: value.clone(),
            },
            Value::Enum { variant, fields } => ScryptoValue::Enum {
                discriminator: variant.clone(),
                fields: fields
                    .clone()
                    .unwrap_or_default()
                    .into_iter()
                    .map(|x| x.to_scrypto_value())
                    .collect::<Result<Vec<_>, _>>()?,
            },
            Value::Array {
                element_type,
                elements,
            } => ScryptoValue::Array {
                element_type_id: (*element_type).into(),
                elements: elements
                    .clone()
                    .into_iter()
                    .map(|x| x.to_scrypto_value())
                    .collect::<Result<Vec<_>, _>>()?,
            },
            Value::Tuple { elements } => ScryptoValue::Tuple {
                fields: elements
                    .clone()
                    .into_iter()
                    .map(|x| x.to_scrypto_value())
                    .collect::<Result<Vec<_>, _>>()?,
            },
            Value::KeyValueStore { identifier } => ScryptoValue::Custom {
                value: ScryptoCustomValue::KeyValueStore(identifier.to_bytes()),
            },

            Value::Decimal { value } => ScryptoValue::Custom {
                value: ScryptoCustomValue::Decimal(*value),
            },
            Value::PreciseDecimal { value } => ScryptoValue::Custom {
                value: ScryptoCustomValue::PreciseDecimal(*value),
            },
            Value::Component { identifier } => ScryptoValue::Custom {
                value: ScryptoCustomValue::Component(identifier.to_bytes()),
            },
            Value::ComponentAddress { address } => ScryptoValue::Custom {
                value: ScryptoCustomValue::ComponentAddress(address.address),
            },
            Value::PackageAddress { address } => ScryptoValue::Custom {
                value: ScryptoCustomValue::PackageAddress(address.address),
            },
            Value::ResourceAddress { address } => ScryptoValue::Custom {
                value: ScryptoCustomValue::ResourceAddress(address.address),
            },
            Value::SystemAddress { address } => ScryptoValue::Custom {
                value: ScryptoCustomValue::SystemAddress(address.address),
            },

            Value::Hash { value } => ScryptoValue::Custom {
                value: ScryptoCustomValue::Hash(*value),
            },

            Value::EcdsaSecp256k1PublicKey { public_key } => ScryptoValue::Custom {
                value: ScryptoCustomValue::EcdsaSecp256k1PublicKey(*public_key),
            },
            Value::EddsaEd25519PublicKey { public_key } => ScryptoValue::Custom {
                value: ScryptoCustomValue::EddsaEd25519PublicKey(*public_key),
            },

            Value::EcdsaSecp256k1Signature { signature } => ScryptoValue::Custom {
                value: ScryptoCustomValue::EcdsaSecp256k1Signature(*signature),
            },
            Value::EddsaEd25519Signature { signature } => ScryptoValue::Custom {
                value: ScryptoCustomValue::EddsaEd25519Signature(*signature),
            },

            Value::Bucket { identifier } => ScryptoValue::Custom {
                value: match identifier.0 {
                    Identifier::U32(numeric_identifier) => {
                        ScryptoCustomValue::Bucket(numeric_identifier)
                    }
                    Identifier::String(_) => {
                        return Err(Error::SborEncodeError(
                            "Unable to encode a Bucket with a string identifier".into(),
                        ));
                    }
                },
            },
            Value::Proof { identifier } => ScryptoValue::Custom {
                value: match identifier.0 {
                    Identifier::U32(numeric_identifier) => {
                        ScryptoCustomValue::Proof(numeric_identifier)
                    }
                    Identifier::String(_) => {
                        return Err(Error::SborEncodeError(
                            "Unable to encode a Proof with a string identifier".into(),
                        ));
                    }
                },
            },
            Value::Vault { identifier } => ScryptoValue::Custom {
                value: ScryptoCustomValue::Vault(identifier.to_bytes()),
            },

            Value::NonFungibleId { value } => ScryptoValue::Custom {
                value: ScryptoCustomValue::NonFungibleId(value.clone()),
            },
            Value::NonFungibleAddress { address } => ScryptoValue::Custom {
                value: ScryptoCustomValue::NonFungibleAddress(address.clone()),
            },

            Value::Blob { hash } => ScryptoValue::Custom {
                value: ScryptoCustomValue::Blob(hash.clone()),
            },
            Value::Expression { value } => ScryptoValue::Custom {
                value: ScryptoCustomValue::Expression(value.clone()),
            },
        };
        Ok(scrypto_value)
    }

    fn from_scrypto_value(scrypto_value: &ScryptoValue, network_id: u8) -> Self {
        match scrypto_value {
            ScryptoValue::Unit => Value::Unit,
            ScryptoValue::Bool { value } => Value::Bool { value: *value },

            ScryptoValue::U8 { value } => Value::U8 { value: *value },
            ScryptoValue::U16 { value } => Value::U16 { value: *value },
            ScryptoValue::U32 { value } => Value::U32 { value: *value },
            ScryptoValue::U64 { value } => Value::U64 { value: *value },
            ScryptoValue::U128 { value } => Value::U128 { value: *value },

            ScryptoValue::I8 { value } => Value::I8 { value: *value },
            ScryptoValue::I16 { value } => Value::I16 { value: *value },
            ScryptoValue::I32 { value } => Value::I32 { value: *value },
            ScryptoValue::I64 { value } => Value::I64 { value: *value },
            ScryptoValue::I128 { value } => Value::I128 { value: *value },

            ScryptoValue::String { value } => Value::String {
                value: value.clone(),
            },

            ScryptoValue::Enum {
                discriminator,
                fields,
            } => Value::Enum {
                variant: discriminator.clone(),
                fields: if fields.is_empty() {
                    None
                } else {
                    Some(
                        fields
                            .clone()
                            .into_iter()
                            .map(|x| Self::from_scrypto_value(&x, network_id))
                            .collect(),
                    )
                },
            },
            ScryptoValue::Array {
                element_type_id,
                elements,
            } => Value::Array {
                element_type: (*element_type_id).into(),
                elements: elements
                    .clone()
                    .into_iter()
                    .map(|x| Self::from_scrypto_value(&x, network_id))
                    .collect(),
            },
            ScryptoValue::Tuple { fields } => Value::Tuple {
                elements: fields
                    .clone()
                    .into_iter()
                    .map(|x| Self::from_scrypto_value(&x, network_id))
                    .collect(),
            },

            ScryptoValue::Custom { value } => match value {
                ScryptoCustomValue::PackageAddress(address) => Value::PackageAddress {
                    address: NetworkAwarePackageAddress {
                        network_id,
                        address: *address,
                    },
                },
                ScryptoCustomValue::ComponentAddress(address) => Value::ComponentAddress {
                    address: NetworkAwareComponentAddress {
                        network_id,
                        address: *address,
                    },
                },
                ScryptoCustomValue::ResourceAddress(address) => Value::ResourceAddress {
                    address: NetworkAwareResourceAddress {
                        network_id,
                        address: *address,
                    },
                },
                ScryptoCustomValue::SystemAddress(address) => Value::SystemAddress {
                    address: NetworkAwareSystemAddress {
                        network_id,
                        address: *address,
                    },
                },

                ScryptoCustomValue::Component(node_id) => Value::Component {
                    identifier: NodeIdentifier::from_bytes(*node_id),
                },
                ScryptoCustomValue::KeyValueStore(node_id) => Value::KeyValueStore {
                    identifier: NodeIdentifier::from_bytes(*node_id),
                },
                ScryptoCustomValue::Vault(node_id) => Value::Vault {
                    identifier: NodeIdentifier::from_bytes(*node_id),
                },
                ScryptoCustomValue::Bucket(identifier) => Value::Bucket {
                    identifier: Identifier::U32(*identifier).into(),
                },
                ScryptoCustomValue::Proof(identifier) => Value::Proof {
                    identifier: Identifier::U32(*identifier).into(),
                },

                ScryptoCustomValue::Expression(value) => Value::Expression {
                    value: value.clone(),
                },
                ScryptoCustomValue::Blob(value) => Value::Blob {
                    hash: value.clone(),
                },
                ScryptoCustomValue::Hash(value) => Value::Hash { value: *value },

                ScryptoCustomValue::EcdsaSecp256k1PublicKey(value) => {
                    Value::EcdsaSecp256k1PublicKey { public_key: *value }
                }
                ScryptoCustomValue::EddsaEd25519PublicKey(value) => {
                    Value::EddsaEd25519PublicKey { public_key: *value }
                }
                ScryptoCustomValue::EcdsaSecp256k1Signature(value) => {
                    Value::EcdsaSecp256k1Signature { signature: *value }
                }
                ScryptoCustomValue::EddsaEd25519Signature(value) => {
                    Value::EddsaEd25519Signature { signature: *value }
                }

                ScryptoCustomValue::Decimal(value) => Value::Decimal { value: *value },
                ScryptoCustomValue::PreciseDecimal(value) => {
                    Value::PreciseDecimal { value: *value }
                }

                ScryptoCustomValue::NonFungibleId(value) => Value::NonFungibleId {
                    value: value.clone(),
                },
                ScryptoCustomValue::NonFungibleAddress(value) => Value::NonFungibleAddress {
                    address: value.clone(),
                },
            },
        }
    }

    // ===========================
    // SBOR Encoding and Decoding
    // ===========================

    pub fn encode(&self) -> Result<Vec<u8>, Error> {
        let scrypto_value = self.to_scrypto_value()?;
        scrypto_encode(&scrypto_value).map_err(|err| err.into())
    }

    pub fn decode(bytes: &[u8], network_id: u8) -> Result<Self, Error> {
        let scrypto_value = scrypto_decode::<ScryptoValue>(bytes)?;
        Ok(Self::from_scrypto_value(&scrypto_value, network_id))
    }
}

// ===========
// Validation
// ===========

impl ValidateWithContext<(u8, Option<ValueKind>)> for Value {
    fn validate(&self, (network_id, expected_kind): (u8, Option<ValueKind>)) -> Result<(), Error> {
        self.validate_if_collection()?;
        self.validate_address_network_id(network_id)?;
        if let Some(expected_kind) = expected_kind {
            self.validate_kind(expected_kind)?;
        };
        Ok(())
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

    Enum,

    Array,
    Tuple,

    Decimal,
    PreciseDecimal,

    Component,
    PackageAddress,
    ComponentAddress,
    ResourceAddress,
    SystemAddress,

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

            Self::Enum => TYPE_ENUM,

            Self::Array => TYPE_ARRAY,
            Self::Tuple => TYPE_TUPLE,

            Self::KeyValueStore => ScryptoCustomTypeId::KeyValueStore.as_u8(),

            Self::Decimal => ScryptoCustomTypeId::Decimal.as_u8(),
            Self::PreciseDecimal => ScryptoCustomTypeId::PreciseDecimal.as_u8(),

            Self::Component => ScryptoCustomTypeId::Component.as_u8(),
            Self::PackageAddress => ScryptoCustomTypeId::PackageAddress.as_u8(),
            Self::ResourceAddress => ScryptoCustomTypeId::ResourceAddress.as_u8(),
            Self::ComponentAddress => ScryptoCustomTypeId::ComponentAddress.as_u8(),
            Self::SystemAddress => ScryptoCustomTypeId::SystemAddress.as_u8(),

            Self::Hash => ScryptoCustomTypeId::Hash.as_u8(),

            Self::Bucket => ScryptoCustomTypeId::Bucket.as_u8(),
            Self::Proof => ScryptoCustomTypeId::Proof.as_u8(),
            Self::Vault => ScryptoCustomTypeId::Vault.as_u8(),

            Self::NonFungibleId => ScryptoCustomTypeId::NonFungibleId.as_u8(),
            Self::NonFungibleAddress => ScryptoCustomTypeId::NonFungibleAddress.as_u8(),

            Self::EcdsaSecp256k1PublicKey => ScryptoCustomTypeId::EcdsaSecp256k1PublicKey.as_u8(),
            Self::EcdsaSecp256k1Signature => ScryptoCustomTypeId::EcdsaSecp256k1Signature.as_u8(),
            Self::EddsaEd25519PublicKey => ScryptoCustomTypeId::EddsaEd25519PublicKey.as_u8(),
            Self::EddsaEd25519Signature => ScryptoCustomTypeId::EddsaEd25519Signature.as_u8(),

            Self::Blob => ScryptoCustomTypeId::Blob.as_u8(),
            Self::Expression => ScryptoCustomTypeId::Expression.as_u8(),
        }
    }

    pub fn from_type_id(type_id: u8) -> Result<Self, Error> {
        let value_kind = match type_id {
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

            TYPE_ENUM => Self::Enum,

            TYPE_ARRAY => Self::Array,
            TYPE_TUPLE => Self::Tuple,

            type_id => match ScryptoCustomTypeId::from_u8(type_id) {
                Some(scrypto_type) => match scrypto_type {
                    ScryptoCustomTypeId::Decimal => Self::Decimal,
                    ScryptoCustomTypeId::PreciseDecimal => Self::PreciseDecimal,
                    ScryptoCustomTypeId::PackageAddress => Self::PackageAddress,
                    ScryptoCustomTypeId::ResourceAddress => Self::ResourceAddress,
                    ScryptoCustomTypeId::ComponentAddress => Self::ComponentAddress,
                    ScryptoCustomTypeId::SystemAddress => Self::SystemAddress,
                    ScryptoCustomTypeId::Hash => Self::Hash,
                    ScryptoCustomTypeId::Bucket => Self::Bucket,
                    ScryptoCustomTypeId::Proof => Self::Proof,
                    ScryptoCustomTypeId::NonFungibleId => Self::NonFungibleId,
                    ScryptoCustomTypeId::NonFungibleAddress => Self::NonFungibleAddress,
                    ScryptoCustomTypeId::Component => Self::Component,
                    ScryptoCustomTypeId::Vault => Self::Vault,
                    ScryptoCustomTypeId::EcdsaSecp256k1PublicKey => Self::EcdsaSecp256k1PublicKey,
                    ScryptoCustomTypeId::EcdsaSecp256k1Signature => Self::EcdsaSecp256k1Signature,
                    ScryptoCustomTypeId::EddsaEd25519PublicKey => Self::EddsaEd25519PublicKey,
                    ScryptoCustomTypeId::EddsaEd25519Signature => Self::EddsaEd25519Signature,
                    ScryptoCustomTypeId::KeyValueStore => Self::KeyValueStore,
                    ScryptoCustomTypeId::Blob => Self::Blob,
                    ScryptoCustomTypeId::Expression => Self::Expression,
                },
                None => return Err(Error::UnknownTypeId { type_id }),
            },
        };
        Ok(value_kind)
    }
}

impl From<ValueKind> for radix_transaction::manifest::ast::Type {
    fn from(value: ValueKind) -> radix_transaction::manifest::ast::Type {
        match value {
            ValueKind::Unit => radix_transaction::manifest::ast::Type::Unit,

            ValueKind::Bool => radix_transaction::manifest::ast::Type::Bool,
            ValueKind::I8 => radix_transaction::manifest::ast::Type::I8,
            ValueKind::I16 => radix_transaction::manifest::ast::Type::I16,
            ValueKind::I32 => radix_transaction::manifest::ast::Type::I32,
            ValueKind::I64 => radix_transaction::manifest::ast::Type::I64,
            ValueKind::I128 => radix_transaction::manifest::ast::Type::I128,

            ValueKind::U8 => radix_transaction::manifest::ast::Type::U8,
            ValueKind::U16 => radix_transaction::manifest::ast::Type::U16,
            ValueKind::U32 => radix_transaction::manifest::ast::Type::U32,
            ValueKind::U64 => radix_transaction::manifest::ast::Type::U64,
            ValueKind::U128 => radix_transaction::manifest::ast::Type::U128,

            ValueKind::String => radix_transaction::manifest::ast::Type::String,

            ValueKind::Enum => radix_transaction::manifest::ast::Type::Enum,

            ValueKind::Array => radix_transaction::manifest::ast::Type::Array,
            ValueKind::Tuple => radix_transaction::manifest::ast::Type::Tuple,

            ValueKind::Decimal => radix_transaction::manifest::ast::Type::Decimal,
            ValueKind::PreciseDecimal => radix_transaction::manifest::ast::Type::PreciseDecimal,

            ValueKind::PackageAddress => radix_transaction::manifest::ast::Type::PackageAddress,
            ValueKind::ComponentAddress => radix_transaction::manifest::ast::Type::ComponentAddress,
            ValueKind::ResourceAddress => radix_transaction::manifest::ast::Type::ResourceAddress,
            ValueKind::SystemAddress => radix_transaction::manifest::ast::Type::SystemAddress,

            ValueKind::Hash => radix_transaction::manifest::ast::Type::Hash,

            ValueKind::Bucket => radix_transaction::manifest::ast::Type::Bucket,
            ValueKind::Proof => radix_transaction::manifest::ast::Type::Proof,

            ValueKind::NonFungibleId => radix_transaction::manifest::ast::Type::NonFungibleId,
            ValueKind::NonFungibleAddress => {
                radix_transaction::manifest::ast::Type::NonFungibleAddress
            }

            ValueKind::Blob => radix_transaction::manifest::ast::Type::Blob,
            ValueKind::Expression => radix_transaction::manifest::ast::Type::Expression,

            ValueKind::Component => radix_transaction::manifest::ast::Type::Component,
            ValueKind::KeyValueStore => radix_transaction::manifest::ast::Type::KeyValueStore,
            ValueKind::Vault => radix_transaction::manifest::ast::Type::Vault,
            ValueKind::EcdsaSecp256k1PublicKey => {
                radix_transaction::manifest::ast::Type::EcdsaSecp256k1PublicKey
            }
            ValueKind::EcdsaSecp256k1Signature => {
                radix_transaction::manifest::ast::Type::EcdsaSecp256k1Signature
            }
            ValueKind::EddsaEd25519PublicKey => {
                radix_transaction::manifest::ast::Type::EddsaEd25519PublicKey
            }
            ValueKind::EddsaEd25519Signature => {
                radix_transaction::manifest::ast::Type::EddsaEd25519Signature
            }
        }
    }
}

impl From<radix_transaction::manifest::ast::Type> for ValueKind {
    fn from(value: radix_transaction::manifest::ast::Type) -> ValueKind {
        match value {
            radix_transaction::manifest::ast::Type::Unit => Self::Unit,
            radix_transaction::manifest::ast::Type::Bool => Self::Bool,

            radix_transaction::manifest::ast::Type::I8 => Self::I8,
            radix_transaction::manifest::ast::Type::I16 => Self::I16,
            radix_transaction::manifest::ast::Type::I32 => Self::I32,
            radix_transaction::manifest::ast::Type::I64 => Self::I64,
            radix_transaction::manifest::ast::Type::I128 => Self::I128,
            radix_transaction::manifest::ast::Type::U8 => Self::U8,
            radix_transaction::manifest::ast::Type::U16 => Self::U16,
            radix_transaction::manifest::ast::Type::U32 => Self::U32,
            radix_transaction::manifest::ast::Type::U64 => Self::U64,
            radix_transaction::manifest::ast::Type::U128 => Self::U128,

            radix_transaction::manifest::ast::Type::String => Self::String,

            radix_transaction::manifest::ast::Type::Enum => Self::Enum,

            radix_transaction::manifest::ast::Type::Array => Self::Array,
            radix_transaction::manifest::ast::Type::Tuple => Self::Tuple,

            radix_transaction::manifest::ast::Type::Decimal => Self::Decimal,
            radix_transaction::manifest::ast::Type::PreciseDecimal => Self::PreciseDecimal,

            radix_transaction::manifest::ast::Type::Component => Self::Component,
            radix_transaction::manifest::ast::Type::PackageAddress => Self::PackageAddress,
            radix_transaction::manifest::ast::Type::ComponentAddress => Self::ComponentAddress,
            radix_transaction::manifest::ast::Type::ResourceAddress => Self::ResourceAddress,
            radix_transaction::manifest::ast::Type::SystemAddress => Self::SystemAddress,

            radix_transaction::manifest::ast::Type::Hash => Self::Hash,
            radix_transaction::manifest::ast::Type::EcdsaSecp256k1PublicKey => {
                Self::EcdsaSecp256k1PublicKey
            }
            radix_transaction::manifest::ast::Type::EcdsaSecp256k1Signature => {
                Self::EcdsaSecp256k1Signature
            }
            radix_transaction::manifest::ast::Type::EddsaEd25519PublicKey => {
                Self::EddsaEd25519PublicKey
            }
            radix_transaction::manifest::ast::Type::EddsaEd25519Signature => {
                Self::EddsaEd25519Signature
            }

            radix_transaction::manifest::ast::Type::Vault => Self::Vault,
            radix_transaction::manifest::ast::Type::KeyValueStore => Self::KeyValueStore,

            radix_transaction::manifest::ast::Type::Bucket => Self::Bucket,
            radix_transaction::manifest::ast::Type::Proof => Self::Proof,

            radix_transaction::manifest::ast::Type::NonFungibleId => Self::NonFungibleId,
            radix_transaction::manifest::ast::Type::NonFungibleAddress => Self::NonFungibleAddress,

            radix_transaction::manifest::ast::Type::Blob => Self::Blob,
            radix_transaction::manifest::ast::Type::Expression => Self::Expression,
        }
    }
}

impl From<ValueKind> for SborTypeId<ScryptoCustomTypeId> {
    fn from(value: ValueKind) -> Self {
        match value {
            ValueKind::Unit => SborTypeId::Unit,
            ValueKind::Bool => SborTypeId::Bool,

            ValueKind::U8 => SborTypeId::U8,
            ValueKind::U16 => SborTypeId::U16,
            ValueKind::U32 => SborTypeId::U32,
            ValueKind::U64 => SborTypeId::U64,
            ValueKind::U128 => SborTypeId::U128,

            ValueKind::I8 => SborTypeId::I8,
            ValueKind::I16 => SborTypeId::I16,
            ValueKind::I32 => SborTypeId::I32,
            ValueKind::I64 => SborTypeId::I64,
            ValueKind::I128 => SborTypeId::I128,

            ValueKind::String => SborTypeId::String,

            ValueKind::Enum => SborTypeId::Enum,
            ValueKind::Array => SborTypeId::Array,
            ValueKind::Tuple => SborTypeId::Tuple,

            ValueKind::Component => SborTypeId::Custom(ScryptoCustomTypeId::Component),
            ValueKind::SystemAddress => SborTypeId::Custom(ScryptoCustomTypeId::SystemAddress),
            ValueKind::PackageAddress => SborTypeId::Custom(ScryptoCustomTypeId::PackageAddress),
            ValueKind::ResourceAddress => SborTypeId::Custom(ScryptoCustomTypeId::ResourceAddress),
            ValueKind::ComponentAddress => {
                SborTypeId::Custom(ScryptoCustomTypeId::ComponentAddress)
            }

            ValueKind::Vault => SborTypeId::Custom(ScryptoCustomTypeId::Vault),
            ValueKind::Proof => SborTypeId::Custom(ScryptoCustomTypeId::Proof),
            ValueKind::Bucket => SborTypeId::Custom(ScryptoCustomTypeId::Bucket),
            ValueKind::KeyValueStore => SborTypeId::Custom(ScryptoCustomTypeId::KeyValueStore),

            ValueKind::Expression => SborTypeId::Custom(ScryptoCustomTypeId::Expression),
            ValueKind::Blob => SborTypeId::Custom(ScryptoCustomTypeId::Blob),
            ValueKind::NonFungibleAddress => {
                SborTypeId::Custom(ScryptoCustomTypeId::NonFungibleAddress)
            }

            ValueKind::Hash => SborTypeId::Custom(ScryptoCustomTypeId::Hash),
            ValueKind::EcdsaSecp256k1PublicKey => {
                SborTypeId::Custom(ScryptoCustomTypeId::EcdsaSecp256k1PublicKey)
            }
            ValueKind::EcdsaSecp256k1Signature => {
                SborTypeId::Custom(ScryptoCustomTypeId::EcdsaSecp256k1Signature)
            }
            ValueKind::EddsaEd25519PublicKey => {
                SborTypeId::Custom(ScryptoCustomTypeId::EddsaEd25519PublicKey)
            }
            ValueKind::EddsaEd25519Signature => {
                SborTypeId::Custom(ScryptoCustomTypeId::EddsaEd25519Signature)
            }
            ValueKind::Decimal => SborTypeId::Custom(ScryptoCustomTypeId::Decimal),
            ValueKind::PreciseDecimal => SborTypeId::Custom(ScryptoCustomTypeId::PreciseDecimal),
            ValueKind::NonFungibleId => SborTypeId::Custom(ScryptoCustomTypeId::NonFungibleId),
        }
    }
}

impl From<SborTypeId<ScryptoCustomTypeId>> for ValueKind {
    fn from(value: SborTypeId<ScryptoCustomTypeId>) -> Self {
        match value {
            SborTypeId::Unit => ValueKind::Unit,
            SborTypeId::Bool => ValueKind::Bool,

            SborTypeId::U8 => ValueKind::U8,
            SborTypeId::U16 => ValueKind::U16,
            SborTypeId::U32 => ValueKind::U32,
            SborTypeId::U64 => ValueKind::U64,
            SborTypeId::U128 => ValueKind::U128,

            SborTypeId::I8 => ValueKind::I8,
            SborTypeId::I16 => ValueKind::I16,
            SborTypeId::I32 => ValueKind::I32,
            SborTypeId::I64 => ValueKind::I64,
            SborTypeId::I128 => ValueKind::I128,

            SborTypeId::String => ValueKind::String,

            SborTypeId::Enum => ValueKind::Enum,
            SborTypeId::Array => ValueKind::Array,
            SborTypeId::Tuple => ValueKind::Tuple,

            SborTypeId::Custom(custom_type_id) => match custom_type_id {
                ScryptoCustomTypeId::PackageAddress => ValueKind::PackageAddress,
                ScryptoCustomTypeId::ComponentAddress => ValueKind::ComponentAddress,
                ScryptoCustomTypeId::ResourceAddress => ValueKind::ResourceAddress,
                ScryptoCustomTypeId::SystemAddress => ValueKind::SystemAddress,

                ScryptoCustomTypeId::Component => ValueKind::Component,
                ScryptoCustomTypeId::KeyValueStore => ValueKind::KeyValueStore,
                ScryptoCustomTypeId::Bucket => ValueKind::Bucket,
                ScryptoCustomTypeId::Proof => ValueKind::Proof,
                ScryptoCustomTypeId::Vault => ValueKind::Vault,

                ScryptoCustomTypeId::Expression => ValueKind::Expression,
                ScryptoCustomTypeId::Blob => ValueKind::Blob,
                ScryptoCustomTypeId::NonFungibleAddress => ValueKind::NonFungibleAddress,

                ScryptoCustomTypeId::Hash => ValueKind::Hash,
                ScryptoCustomTypeId::EcdsaSecp256k1PublicKey => ValueKind::EcdsaSecp256k1PublicKey,
                ScryptoCustomTypeId::EcdsaSecp256k1Signature => ValueKind::EcdsaSecp256k1Signature,
                ScryptoCustomTypeId::EddsaEd25519PublicKey => ValueKind::EddsaEd25519PublicKey,
                ScryptoCustomTypeId::EddsaEd25519Signature => ValueKind::EddsaEd25519Signature,
                ScryptoCustomTypeId::Decimal => ValueKind::Decimal,
                ScryptoCustomTypeId::PreciseDecimal => ValueKind::PreciseDecimal,
                ScryptoCustomTypeId::NonFungibleId => ValueKind::NonFungibleId,
            },
        }
    }
}

// =============================
// From and TryFrom Conversions
// =============================

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::String { value }
    }
}

impl From<Decimal> for Value {
    fn from(value: Decimal) -> Self {
        Value::Decimal { value }
    }
}

impl From<Blob> for Value {
    fn from(hash: Blob) -> Self {
        Value::Blob { hash }
    }
}

impl From<NonFungibleId> for Value {
    fn from(value: NonFungibleId) -> Self {
        Value::NonFungibleId { value }
    }
}

impl From<NonFungibleAddress> for Value {
    fn from(address: NonFungibleAddress) -> Self {
        Value::NonFungibleAddress { address }
    }
}

impl From<NetworkAwareComponentAddress> for Value {
    fn from(address: NetworkAwareComponentAddress) -> Value {
        Value::ComponentAddress { address }
    }
}

impl From<NetworkAwareResourceAddress> for Value {
    fn from(address: NetworkAwareResourceAddress) -> Value {
        Value::ResourceAddress { address }
    }
}

impl From<NetworkAwarePackageAddress> for Value {
    fn from(address: NetworkAwarePackageAddress) -> Value {
        Value::PackageAddress { address }
    }
}

impl TryFrom<Value> for String {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::String { value } => Ok(value),
            _ => Err(Error::InvalidType {
                expected_types: vec![ValueKind::String],
                actual_type: value.kind(),
            }),
        }
    }
}

impl TryFrom<Value> for Decimal {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Decimal { value } => Ok(value),
            _ => Err(Error::InvalidType {
                expected_types: vec![ValueKind::Decimal],
                actual_type: value.kind(),
            }),
        }
    }
}

impl TryFrom<Value> for Blob {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Blob { hash } => Ok(hash),
            _ => Err(Error::InvalidType {
                expected_types: vec![ValueKind::Blob],
                actual_type: value.kind(),
            }),
        }
    }
}

impl TryFrom<Value> for NonFungibleAddress {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::NonFungibleAddress { address } => Ok(address),
            _ => Err(Error::InvalidType {
                expected_types: vec![ValueKind::NonFungibleAddress],
                actual_type: value.kind(),
            }),
        }
    }
}

impl TryFrom<Value> for NonFungibleId {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::NonFungibleId { value } => Ok(value),
            _ => Err(Error::InvalidType {
                expected_types: vec![ValueKind::NonFungibleId],
                actual_type: value.kind(),
            }),
        }
    }
}

impl TryFrom<Value> for NetworkAwareComponentAddress {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::ComponentAddress { address } => Ok(address),
            _ => Err(Error::InvalidType {
                expected_types: vec![ValueKind::ComponentAddress],
                actual_type: value.kind(),
            }),
        }
    }
}

impl TryFrom<Value> for NetworkAwareResourceAddress {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::ResourceAddress { address } => Ok(address),
            _ => Err(Error::InvalidType {
                expected_types: vec![ValueKind::ResourceAddress],
                actual_type: value.kind(),
            }),
        }
    }
}

impl TryFrom<Value> for NetworkAwarePackageAddress {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::PackageAddress { address } => Ok(address),
            _ => Err(Error::InvalidType {
                expected_types: vec![ValueKind::PackageAddress],
                actual_type: value.kind(),
            }),
        }
    }
}

// ===========
// Unit Tests
// ===========

#[cfg(test)]
mod tests {
    use scrypto::{
        prelude::*,
        radix_engine_interface::{address::Bech32Decoder, core::NetworkDefinition},
    };

    use super::{Value, ValueKind};
    use crate::model::address::*;

    macro_rules! test_value {
        ($string: expr, $value: expr) => {
            assert_serialization_matches($string, $value);
            assert_deserialization_matches($string, $value);
        };
    }

    fn assert_serialization_matches(string: &str, value: Value) {
        let serialized_string =
            serde_json::to_string(&value).expect("Serialization of trusted value failed");

        let string = string.replace(['\n', ' '], "");
        let serialized_string = serialized_string.replace(['\n', ' '], "");
        assert_eq!(string, serialized_string);
    }

    fn assert_deserialization_matches(string: &str, value: Value) {
        let deserialized_value = serde_json::from_str(string).expect("Deserialization failed.");
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
                "type": "Enum",
                "variant": "Component"
            }
            "#,
            Value::Enum {
                variant: "Component".into(),
                fields: None,
            }
        };
        test_value! {
            r#"
            {
                "type": "Enum",
                "variant": "Component",
                "fields": [
                    {
                        "type": "String",
                        "value": "Account"
                    }
                ]
            }
            "#,
            Value::Enum {
                variant: "Component".into(),
                fields: Some(vec![
                    Value::String { value: "Account".into() }
                ])
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
                    Value::Bucket {
                        identifier: crate::model::identifier::Identifier::String("my_xrd_bucket".into()).into()
                    }
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
                "type": "ComponentAddress",
                "address": "account_sim1qwssnwt0yzhzjydxj7u9uvnljtgaug23re8p32jrjecqajtsvr"
            }
            "#,
            Value::ComponentAddress {
                address: NetworkAwareComponentAddress {
                    network_id: 0xf2,
                    address: Bech32Decoder::new(&NetworkDefinition::simulator())
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
                    address: Bech32Decoder::new(&NetworkDefinition::simulator())
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
                    address: Bech32Decoder::new(&NetworkDefinition::simulator())
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
                value: "910edb2dabf107c7628ecdb9126535676d61bc39a843475f3057d809bfd2d65d".parse().unwrap()
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
                identifier: crate::model::identifier::Identifier::U32(192).into()
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
                identifier: crate::model::identifier::Identifier::String("HelloBucket".into()).into()
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
                identifier: crate::model::identifier::Identifier::U32(192).into()
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
                identifier: crate::model::identifier::Identifier::String("HelloProof".into()).into()
            }
        };
    }

    #[test]
    fn non_collection_validation_succeeds() {
        // Arrange
        let value = Value::Bool { value: false };

        // Act
        let result = value.validate_if_collection();

        // Assert
        assert!(matches!(result, Ok(())))
    }

    #[test]
    fn array_of_decimals_validation_succeeds() {
        // Arrange
        let value = Value::Array {
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
        let result = value.validate_if_collection();

        // Assert
        assert!(matches!(result, Ok(())))
    }

    #[test]
    fn array_of_decimal_and_precise_decimal_validation_fails() {
        // Arrange
        let value = Value::Array {
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
        let result = value.validate_if_collection();

        // Assert
        let _expected_types = vec![ValueKind::Decimal];
        assert!(matches!(
            result,
            Err(crate::error::Error::InvalidType {
                expected_types: _,
                actual_type: ValueKind::PreciseDecimal
            })
        ))
    }

    #[test]
    fn validation_of_deeply_nested_tuple_with_non_matching_types_fails() {
        // Arrange
        let value = Value::Tuple {
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
        let result = value.validate_if_collection();

        // Assert
        assert!(matches!(result, Err(_)))
    }
}
