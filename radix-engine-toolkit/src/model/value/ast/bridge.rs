// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use super::model::*;
use crate::error::{Error, Result};
use crate::model::address::*;
use crate::model::engine_identifier::TransientIdentifier;

use native_transaction::manifest::ast;
use native_transaction::manifest::generator::GeneratorError;
use scrypto::prelude::{
    ManifestBlobRef, ManifestCustomValueKind, ManifestExpression, ManifestValueKind,
};

impl From<ast::Type> for ManifestAstValueKind {
    fn from(value: ast::Type) -> ManifestAstValueKind {
        match value {
            ast::Type::Bool => Self::Bool,

            ast::Type::I8 => Self::I8,
            ast::Type::I16 => Self::I16,
            ast::Type::I32 => Self::I32,
            ast::Type::I64 => Self::I64,
            ast::Type::I128 => Self::I128,
            ast::Type::U8 => Self::U8,
            ast::Type::U16 => Self::U16,
            ast::Type::U32 => Self::U32,
            ast::Type::U64 => Self::U64,
            ast::Type::U128 => Self::U128,

            ast::Type::String => Self::String,

            ast::Type::Enum => Self::Enum,

            ast::Type::Array => Self::Array,
            ast::Type::Tuple => Self::Tuple,

            ast::Type::Decimal => Self::Decimal,
            ast::Type::PreciseDecimal => Self::PreciseDecimal,

            ast::Type::Address => Self::Address,
            ast::Type::PackageAddress => Self::PackageAddress,
            ast::Type::ResourceAddress => Self::ResourceAddress,
            ast::Type::ComponentAddress => Self::ComponentAddress,

            ast::Type::Bucket => Self::Bucket,
            ast::Type::Proof => Self::Proof,

            ast::Type::NonFungibleLocalId => Self::NonFungibleLocalId,
            ast::Type::NonFungibleGlobalId => Self::NonFungibleGlobalId,

            ast::Type::Blob => Self::Blob,
            ast::Type::Expression => Self::Expression,
            ast::Type::Bytes => Self::Bytes,
        }
    }
}

impl From<ManifestAstValueKind> for ast::Type {
    fn from(value: ManifestAstValueKind) -> ast::Type {
        match value {
            ManifestAstValueKind::Bool => Self::Bool,
            ManifestAstValueKind::I8 => Self::I8,
            ManifestAstValueKind::I16 => Self::I16,
            ManifestAstValueKind::I32 => Self::I32,
            ManifestAstValueKind::I64 => Self::I64,
            ManifestAstValueKind::I128 => Self::I128,

            ManifestAstValueKind::U8 => Self::U8,
            ManifestAstValueKind::U16 => Self::U16,
            ManifestAstValueKind::U32 => Self::U32,
            ManifestAstValueKind::U64 => Self::U64,
            ManifestAstValueKind::U128 => Self::U128,

            ManifestAstValueKind::String => Self::String,

            ManifestAstValueKind::Enum => Self::Enum,
            ManifestAstValueKind::Some => Self::Enum,
            ManifestAstValueKind::None => Self::Enum,
            ManifestAstValueKind::Ok => Self::Enum,
            ManifestAstValueKind::Err => Self::Enum,

            ManifestAstValueKind::Map => Self::Array,
            ManifestAstValueKind::Array => Self::Array,
            ManifestAstValueKind::Tuple => Self::Tuple,

            ManifestAstValueKind::Decimal => Self::Decimal,
            ManifestAstValueKind::PreciseDecimal => Self::PreciseDecimal,

            ManifestAstValueKind::Address => Self::Address,
            ManifestAstValueKind::PackageAddress => Self::PackageAddress,
            ManifestAstValueKind::ResourceAddress => Self::ResourceAddress,
            ManifestAstValueKind::ComponentAddress => Self::ComponentAddress,

            ManifestAstValueKind::Bucket => Self::Bucket,
            ManifestAstValueKind::Proof => Self::Proof,

            ManifestAstValueKind::NonFungibleLocalId => Self::NonFungibleLocalId,
            ManifestAstValueKind::NonFungibleGlobalId => Self::NonFungibleGlobalId,

            ManifestAstValueKind::Blob => Self::Blob,
            ManifestAstValueKind::Bytes => Self::Bytes,
            ManifestAstValueKind::Expression => Self::Expression,
        }
    }
}

impl From<ManifestValueKind> for ManifestAstValueKind {
    fn from(value: ManifestValueKind) -> ManifestAstValueKind {
        match value {
            ManifestValueKind::Bool => Self::Bool,

            ManifestValueKind::I8 => Self::I8,
            ManifestValueKind::I16 => Self::I16,
            ManifestValueKind::I32 => Self::I32,
            ManifestValueKind::I64 => Self::I64,
            ManifestValueKind::I128 => Self::I128,
            ManifestValueKind::U8 => Self::U8,
            ManifestValueKind::U16 => Self::U16,
            ManifestValueKind::U32 => Self::U32,
            ManifestValueKind::U64 => Self::U64,
            ManifestValueKind::U128 => Self::U128,

            ManifestValueKind::String => Self::String,

            ManifestValueKind::Enum => Self::Enum,

            ManifestValueKind::Map => Self::Map,
            ManifestValueKind::Array => Self::Array,
            ManifestValueKind::Tuple => Self::Tuple,

            ManifestValueKind::Custom(ManifestCustomValueKind::Decimal) => Self::Decimal,
            ManifestValueKind::Custom(ManifestCustomValueKind::PreciseDecimal) => {
                Self::PreciseDecimal
            }

            ManifestValueKind::Custom(ManifestCustomValueKind::Address) => Self::Address,

            ManifestValueKind::Custom(ManifestCustomValueKind::Bucket) => Self::Bucket,
            ManifestValueKind::Custom(ManifestCustomValueKind::Proof) => Self::Proof,

            ManifestValueKind::Custom(ManifestCustomValueKind::NonFungibleLocalId) => {
                Self::NonFungibleLocalId
            }

            ManifestValueKind::Custom(ManifestCustomValueKind::Blob) => Self::Blob,
            ManifestValueKind::Custom(ManifestCustomValueKind::Expression) => Self::Expression,
        }
    }
}

impl From<ManifestAstValueKind> for ManifestValueKind {
    fn from(value: ManifestAstValueKind) -> ManifestValueKind {
        match value {
            ManifestAstValueKind::Bool => Self::Bool,

            ManifestAstValueKind::I8 => Self::I8,
            ManifestAstValueKind::I16 => Self::I16,
            ManifestAstValueKind::I32 => Self::I32,
            ManifestAstValueKind::I64 => Self::I64,
            ManifestAstValueKind::I128 => Self::I128,
            ManifestAstValueKind::U8 => Self::U8,
            ManifestAstValueKind::U16 => Self::U16,
            ManifestAstValueKind::U32 => Self::U32,
            ManifestAstValueKind::U64 => Self::U64,
            ManifestAstValueKind::U128 => Self::U128,

            ManifestAstValueKind::String => Self::String,

            ManifestAstValueKind::Ok
            | ManifestAstValueKind::Err
            | ManifestAstValueKind::Some
            | ManifestAstValueKind::None
            | ManifestAstValueKind::Enum => Self::Enum,

            ManifestAstValueKind::Map => Self::Map,
            ManifestAstValueKind::Bytes | ManifestAstValueKind::Array => Self::Array,
            ManifestAstValueKind::NonFungibleGlobalId | ManifestAstValueKind::Tuple => Self::Tuple,

            ManifestAstValueKind::Decimal => Self::Custom(ManifestCustomValueKind::Decimal),
            ManifestAstValueKind::PreciseDecimal => {
                Self::Custom(ManifestCustomValueKind::PreciseDecimal)
            }

            ManifestAstValueKind::Address
            | ManifestAstValueKind::PackageAddress
            | ManifestAstValueKind::ResourceAddress
            | ManifestAstValueKind::ComponentAddress => {
                Self::Custom(ManifestCustomValueKind::Address)
            }

            ManifestAstValueKind::Bucket => Self::Custom(ManifestCustomValueKind::Bucket),
            ManifestAstValueKind::Proof => Self::Custom(ManifestCustomValueKind::Proof),

            ManifestAstValueKind::NonFungibleLocalId => {
                Self::Custom(ManifestCustomValueKind::NonFungibleLocalId)
            }

            ManifestAstValueKind::Blob => Self::Custom(ManifestCustomValueKind::Blob),
            ManifestAstValueKind::Expression => Self::Custom(ManifestCustomValueKind::Expression),
        }
    }
}

impl ManifestAstValue {
    pub fn to_ast_value(&self, bech32_coder: &Bech32Coder) -> Result<ast::Value> {
        let value = match self {
            ManifestAstValue::Bool { value } => ast::Value::Bool(*value),

            ManifestAstValue::I8 { value } => ast::Value::I8(*value),
            ManifestAstValue::I16 { value } => ast::Value::I16(*value),
            ManifestAstValue::I32 { value } => ast::Value::I32(*value),
            ManifestAstValue::I64 { value } => ast::Value::I64(*value),
            ManifestAstValue::I128 { value } => ast::Value::I128(*value),

            ManifestAstValue::U8 { value } => ast::Value::U8(*value),
            ManifestAstValue::U16 { value } => ast::Value::U16(*value),
            ManifestAstValue::U32 { value } => ast::Value::U32(*value),
            ManifestAstValue::U64 { value } => ast::Value::U64(*value),
            ManifestAstValue::U128 { value } => ast::Value::U128(*value),

            ManifestAstValue::String { value } => ast::Value::String(value.clone()),

            ManifestAstValue::Enum { variant, fields } => ast::Value::Enum(
                variant.resolve_discriminator()?,
                fields
                    .clone()
                    .unwrap_or_default()
                    .iter()
                    .map(|value| value.to_ast_value(bech32_coder))
                    .collect::<Result<Vec<ast::Value>>>()?,
            ),
            ManifestAstValue::Some { value } => {
                ast::Value::Some(Box::new(value.to_ast_value(bech32_coder)?))
            }
            ManifestAstValue::None => ast::Value::None,
            ManifestAstValue::Ok { value } => {
                ast::Value::Ok(Box::new(value.to_ast_value(bech32_coder)?))
            }
            ManifestAstValue::Err { value } => {
                ast::Value::Err(Box::new(value.to_ast_value(bech32_coder)?))
            }

            ManifestAstValue::Array {
                element_kind,
                elements,
            } => ast::Value::Array(
                (*element_kind).into(),
                elements
                    .iter()
                    .map(|id| id.to_ast_value(bech32_coder))
                    .collect::<Result<Vec<ast::Value>>>()?,
            ),
            ManifestAstValue::Map {
                key_value_kind,
                value_value_kind,
                entries,
            } => ast::Value::Map(
                (*key_value_kind).into(),
                (*value_value_kind).into(),
                entries
                    .iter()
                    .flat_map(|(x, y)| [x, y])
                    .map(|value| value.to_ast_value(bech32_coder))
                    .collect::<Result<Vec<ast::Value>>>()?,
            ),
            ManifestAstValue::Tuple { elements } => ast::Value::Tuple(
                elements
                    .iter()
                    .map(|v| v.to_ast_value(bech32_coder))
                    .collect::<Result<Vec<ast::Value>>>()?,
            ),

            ManifestAstValue::Decimal { value } => {
                ast::Value::Decimal(Box::new(ast::Value::String(value.to_string())))
            }
            ManifestAstValue::PreciseDecimal { value } => {
                ast::Value::PreciseDecimal(Box::new(ast::Value::String(value.to_string())))
            }

            ManifestAstValue::Address { address: value } => ast::Value::Address(Box::new(
                ast::Value::String(value.to_string_with_encoder(bech32_coder)),
            )),
            ManifestAstValue::PackageAddress { address: value } => {
                ast::Value::PackageAddress(Box::new(ast::Value::String(
                    bech32_coder.encode_package_address(value.address),
                )))
            }
            ManifestAstValue::ComponentAddress { address: value } => {
                ast::Value::ComponentAddress(Box::new(ast::Value::String(
                    bech32_coder.encode_component_address(&value.address),
                )))
            }
            ManifestAstValue::ResourceAddress { address: value } => {
                ast::Value::ResourceAddress(Box::new(ast::Value::String(
                    bech32_coder.encode_resource_address(value.address),
                )))
            }

            ManifestAstValue::Bucket { identifier } => {
                ast::Value::Bucket(Box::new(match identifier.0 {
                    TransientIdentifier::String {
                        value: ref identifier,
                    } => ast::Value::String(identifier.clone()),
                    TransientIdentifier::U32 { value: identifier } => ast::Value::U32(identifier),
                }))
            }
            ManifestAstValue::Proof { identifier } => {
                ast::Value::Proof(Box::new(match identifier.0 {
                    TransientIdentifier::String {
                        value: ref identifier,
                    } => ast::Value::String(identifier.clone()),
                    TransientIdentifier::U32 { value: identifier } => ast::Value::U32(identifier),
                }))
            }

            ManifestAstValue::NonFungibleLocalId { value } => {
                ast::Value::NonFungibleLocalId(Box::new(ast::Value::String(value.to_string())))
            }
            ManifestAstValue::NonFungibleGlobalId { address } => {
                let nf_global_id_string = format!(
                    "{}:{}",
                    bech32_coder.encode_resource_address(address.resource_address.address),
                    address.non_fungible_local_id
                );
                ast::Value::NonFungibleGlobalId(Box::new(ast::Value::String(nf_global_id_string)))
            }

            ManifestAstValue::Blob { hash } => {
                ast::Value::Blob(Box::new(ast::Value::String(hex::encode(hash.0))))
            }
            ManifestAstValue::Expression { value } => {
                ast::Value::Expression(Box::new(ast::Value::String(match value {
                    ManifestExpression::EntireWorktop => "ENTIRE_WORKTOP".into(),
                    ManifestExpression::EntireAuthZone => "ENTIRE_AUTH_ZONE".into(),
                })))
            }
            ManifestAstValue::Bytes { value } => {
                ast::Value::Bytes(Box::new(ast::Value::String(hex::encode(value))))
            }
        };
        Ok(value)
    }

    /// Converts Scrypto's tx compiler's [`ast::Value`] to a [`Value`] given a bech32 coder as
    /// context.
    pub fn from_ast_value(value: &ast::Value, bech32_coder: &Bech32Coder) -> Result<Self> {
        let parsing = ManifestAstValueKind::from(value.value_kind());
        let value = match value {
            ast::Value::Bool(value) => Self::Bool { value: *value },

            ast::Value::I8(value) => Self::I8 { value: *value },
            ast::Value::I16(value) => Self::I16 { value: *value },
            ast::Value::I32(value) => Self::I32 { value: *value },
            ast::Value::I64(value) => Self::I64 { value: *value },
            ast::Value::I128(value) => Self::I128 { value: *value },

            ast::Value::U8(value) => Self::U8 { value: *value },
            ast::Value::U16(value) => Self::U16 { value: *value },
            ast::Value::U32(value) => Self::U32 { value: *value },
            ast::Value::U64(value) => Self::U64 { value: *value },
            ast::Value::U128(value) => Self::U128 { value: *value },

            ast::Value::String(value) => Self::String {
                value: value.clone(),
            },

            ast::Value::Enum(variant, fields) => Self::Enum {
                variant: EnumDiscriminator::U8 {
                    discriminator: *variant,
                },
                fields: {
                    if fields.is_empty() {
                        None
                    } else {
                        Some(
                            fields
                                .iter()
                                .map(|value| Self::from_ast_value(value, bech32_coder))
                                .collect::<Result<Vec<ManifestAstValue>>>()?,
                        )
                    }
                },
            },

            ast::Value::Some(value) => Self::Some {
                value: Box::new(Self::from_ast_value(value, bech32_coder)?),
            },
            ast::Value::None => Self::None,
            ast::Value::Ok(value) => Self::Ok {
                value: Box::new(Self::from_ast_value(value, bech32_coder)?),
            },
            ast::Value::Err(value) => Self::Err {
                value: Box::new(Self::from_ast_value(value, bech32_coder)?),
            },

            ast::Value::Map(key_value_kind, value_value_kind, entries) => Self::Map {
                key_value_kind: (*key_value_kind).into(),
                value_value_kind: (*value_value_kind).into(),
                entries: {
                    // Ensure that we have enough elements for the window operation
                    if entries.len() % 2 != 0 {
                        Err(Error::from(GeneratorError::OddNumberOfElements))
                    } else {
                        let mut entries_vec = Vec::new();
                        for chunk in entries.chunks(2) {
                            let key = Self::from_ast_value(&chunk[0], bech32_coder)?;
                            let value = Self::from_ast_value(&chunk[1], bech32_coder)?;

                            entries_vec.push((key, value));
                        }
                        Ok(entries_vec)
                    }
                }?,
            },
            ast::Value::Array(ast_type, elements) => Self::Array {
                element_kind: (*ast_type).into(),
                elements: elements
                    .iter()
                    .map(|value| Self::from_ast_value(value, bech32_coder))
                    .collect::<Result<Vec<ManifestAstValue>>>()?,
            },
            ast::Value::Tuple(elements) => Self::Tuple {
                elements: elements
                    .iter()
                    .map(|value| Self::from_ast_value(value, bech32_coder))
                    .collect::<Result<Vec<ManifestAstValue>>>()?,
            },
            ast::Value::Decimal(value) => map_if_value_string(parsing, value, |string| {
                string
                    .parse()
                    .map(|value| Self::Decimal { value })
                    .map_err(Error::from)
            })?,
            ast::Value::PreciseDecimal(value) => map_if_value_string(parsing, value, |string| {
                string
                    .parse()
                    .map(|value| Self::PreciseDecimal { value })
                    .map_err(Error::from)
            })?,
            ast::Value::Address(address) => {
                map_if_value_string(parsing, address, |address_string| {
                    EntityAddress::from_str_with_coder(address_string, bech32_coder)
                        .map(|address| ManifestAstValue::Address { address })
                })?
            }
            ast::Value::PackageAddress(address) => {
                map_if_value_string(parsing, address, |address_string| {
                    bech32_coder
                        .decode_to_network_aware_package_address(address_string)
                        .map(|address| ManifestAstValue::PackageAddress { address })
                })?
            }
            ast::Value::ResourceAddress(address) => {
                map_if_value_string(parsing, address, |address_string| {
                    bech32_coder
                        .decode_to_network_aware_resource_address(address_string)
                        .map(|address| ManifestAstValue::ResourceAddress { address })
                })?
            }
            ast::Value::ComponentAddress(address) => {
                map_if_value_string(parsing, address, |address_string| {
                    bech32_coder
                        .decode_to_network_aware_component_address(address_string)
                        .map(|address| ManifestAstValue::ComponentAddress { address })
                })?
            }

            ast::Value::Bucket(value) => {
                if let ast::Value::U32(identifier) = &**value {
                    Self::Bucket {
                        identifier: TransientIdentifier::U32 { value: *identifier }.into(),
                    }
                } else if let ast::Value::String(identifier) = &**value {
                    Self::Bucket {
                        identifier: TransientIdentifier::String {
                            value: identifier.to_owned(),
                        }
                        .into(),
                    }
                } else {
                    Err(Error::UnexpectedAstContents {
                        parsing: ManifestAstValueKind::Bucket,
                        expected: vec![ManifestAstValueKind::U32, ManifestAstValueKind::String],
                        found: value.value_kind().into(),
                    })?
                }
            }
            ast::Value::Proof(value) => {
                if let ast::Value::U32(identifier) = &**value {
                    Self::Proof {
                        identifier: TransientIdentifier::U32 { value: *identifier }.into(),
                    }
                } else if let ast::Value::String(identifier) = &**value {
                    Self::Proof {
                        identifier: TransientIdentifier::String {
                            value: identifier.clone(),
                        }
                        .into(),
                    }
                } else {
                    Err(Error::UnexpectedAstContents {
                        parsing: ManifestAstValueKind::Proof,
                        expected: vec![ManifestAstValueKind::U32, ManifestAstValueKind::String],
                        found: value.value_kind().into(),
                    })?
                }
            }

            ast::Value::NonFungibleLocalId(value) => Self::NonFungibleLocalId {
                value: match &**value {
                    ast::Value::String(value) => value.parse()?,
                    _ => Err(Error::UnexpectedAstContents {
                        parsing: ManifestAstValueKind::NonFungibleLocalId,
                        expected: vec![ManifestAstValueKind::String],
                        found: value.value_kind().into(),
                    })?,
                },
            },
            ast::Value::NonFungibleGlobalId(value) => match &**value {
                ast::Value::String(string) => {
                    let native_global_id =
                        scrypto::prelude::NonFungibleGlobalId::try_from_canonical_string(
                            bech32_coder.decoder(),
                            string,
                        )?;
                    Self::NonFungibleGlobalId {
                        address: NonFungibleGlobalId {
                            resource_address: NetworkAwareResourceAddress {
                                network_id: bech32_coder.network_id(),
                                address: native_global_id.resource_address(),
                            },
                            non_fungible_local_id: native_global_id.local_id().clone(),
                        },
                    }
                }
                _ => Err(Error::UnexpectedAstContents {
                    parsing: ManifestAstValueKind::NonFungibleGlobalId,
                    expected: vec![ManifestAstValueKind::String],
                    found: value.value_kind().into(),
                })?,
            },

            ast::Value::Blob(value) => map_if_value_string(parsing, value, |blob_string| {
                let bytes = hex::decode(blob_string)?;
                ManifestBlobRef::try_from(bytes.as_slice())
                    .map(|manifest_blob| Self::Blob {
                        hash: manifest_blob,
                    })
                    .map_err(Error::from)
            })?,
            ast::Value::Expression(value) => map_if_value_string(
                parsing,
                value,
                |expression_string| match expression_string {
                    "ENTIRE_WORKTOP" => Ok(Self::Expression {
                        value: ManifestExpression::EntireWorktop,
                    }),
                    "ENTIRE_AUTH_ZONE" => Ok(Self::Expression {
                        value: ManifestExpression::EntireAuthZone,
                    }),
                    string => Err(Error::InvalidExpressionString {
                        found: string.to_owned(),
                        excepted: vec![
                            String::from("ENTIRE_WORKTOP"),
                            String::from("ENTIRE_AUTH_ZONE"),
                        ],
                    }),
                },
            )?,

            ast::Value::Bytes(value) => map_if_value_string(parsing, value, |string| {
                hex::decode(string)
                    .map_err(Error::from)
                    .map(|value| Self::Bytes { value })
            })?,
        };
        Ok(value)
    }
}

fn map_if_value_string<F>(
    parsing: ManifestAstValueKind,
    value: &ast::Value,
    map: F,
) -> Result<ManifestAstValue>
where
    F: FnOnce(&str) -> Result<ManifestAstValue>,
{
    if let ast::Value::String(value) = value {
        map(value)
    } else {
        Err(Error::UnexpectedAstContents {
            parsing,
            expected: vec![ManifestAstValueKind::String],
            found: value.value_kind().into(),
        })
    }
}
