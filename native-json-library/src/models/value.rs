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

use radix_engine_common::prelude::converter::*;
use radix_engine_common::prelude::*;
use transaction::manifest::generator::*;
use transaction::prelude::*;

use super::manifest_runtime::*;
use crate::models::cryptographic::hash::*;
use crate::models::macros::*;
use crate::models::node_id::*;
use crate::models::non_fungible::*;
use crate::models::number::*;

define_enum_and_kind! {
    pub enum SerializableManifestValue {
        Bool {
            value: bool,
        },
        I8 {
            value: SerializableI8,
        },
        I16 {
            value: SerializableI16,
        },
        I32 {
            value: SerializableI32,
        },
        I64 {
            value: SerializableI64,
        },
        I128 {
            value: SerializableI128,
        },
        U8 {
            value: SerializableU8,
        },
        U16 {
            value: SerializableU16,
        },
        U32 {
            value: SerializableU32,
        },
        U64 {
            value: SerializableU64,
        },
        U128 {
            value: SerializableU128,
        },
        String {
            value: String,
        },
        Enum {
            discriminator: SerializableU8,
            fields: Vec<Self>,
        },
        Array {
            element_value_kind: SerializableManifestValueKind,
            elements: Vec<Self>,
        },
        Tuple {
            fields: Vec<Self>,
        },
        Map {
            key_value_kind: SerializableManifestValueKind,
            value_value_kind: SerializableManifestValueKind,
            entries: Vec<(Self, Self)>,
        },
        Address {
            value: SerializableManifestAddress,
        },
        Bucket {
            value: SerializableBucketId,
        },
        Proof {
            value: SerializableProofId,
        },
        Expression {
            value: SerializableExpression,
        },
        Blob {
            value: SerializableHash,
        },
        Decimal {
            value: SerializableDecimal,
        },
        PreciseDecimal {
            value: SerializablePreciseDecimal,
        },
        NonFungibleLocalId {
            value: SerializableNonFungibleLocalId,
        },
        AddressReservation {
            value: SerializableAddressReservation,
        },
    }
}

impl From<SerializableManifestValueKind> for ManifestValueKind {
    fn from(value: SerializableManifestValueKind) -> Self {
        match value {
            SerializableManifestValueKind::Bool => Self::Bool,
            SerializableManifestValueKind::I8 => Self::I8,
            SerializableManifestValueKind::I16 => Self::I16,
            SerializableManifestValueKind::I32 => Self::I32,
            SerializableManifestValueKind::I64 => Self::I64,
            SerializableManifestValueKind::I128 => Self::I128,
            SerializableManifestValueKind::U8 => Self::U8,
            SerializableManifestValueKind::U16 => Self::U16,
            SerializableManifestValueKind::U32 => Self::U32,
            SerializableManifestValueKind::U64 => Self::U64,
            SerializableManifestValueKind::U128 => Self::U128,
            SerializableManifestValueKind::String => Self::String,
            SerializableManifestValueKind::Enum => Self::Enum,
            SerializableManifestValueKind::Array => Self::Array,
            SerializableManifestValueKind::Tuple => Self::Tuple,
            SerializableManifestValueKind::Map => Self::Map,
            SerializableManifestValueKind::Address => {
                Self::Custom(ManifestCustomValueKind::Address)
            }
            SerializableManifestValueKind::Bucket => Self::Custom(ManifestCustomValueKind::Bucket),
            SerializableManifestValueKind::Proof => Self::Custom(ManifestCustomValueKind::Proof),
            SerializableManifestValueKind::Expression => {
                Self::Custom(ManifestCustomValueKind::Expression)
            }
            SerializableManifestValueKind::Blob => Self::Custom(ManifestCustomValueKind::Blob),
            SerializableManifestValueKind::Decimal => {
                Self::Custom(ManifestCustomValueKind::Decimal)
            }
            SerializableManifestValueKind::PreciseDecimal => {
                Self::Custom(ManifestCustomValueKind::PreciseDecimal)
            }
            SerializableManifestValueKind::NonFungibleLocalId => {
                Self::Custom(ManifestCustomValueKind::NonFungibleLocalId)
            }
            SerializableManifestValueKind::AddressReservation => {
                Self::Custom(ManifestCustomValueKind::AddressReservation)
            }
        }
    }
}

impl From<ManifestValueKind> for SerializableManifestValueKind {
    fn from(value: ManifestValueKind) -> Self {
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
            ManifestValueKind::Array => Self::Array,
            ManifestValueKind::Tuple => Self::Tuple,
            ManifestValueKind::Map => Self::Map,
            ManifestValueKind::Custom(custom) => match custom {
                ManifestCustomValueKind::Address => Self::Address,
                ManifestCustomValueKind::Bucket => Self::Bucket,
                ManifestCustomValueKind::Proof => Self::Proof,
                ManifestCustomValueKind::Expression => Self::Expression,
                ManifestCustomValueKind::Blob => Self::Blob,
                ManifestCustomValueKind::Decimal => Self::Decimal,
                ManifestCustomValueKind::PreciseDecimal => Self::PreciseDecimal,
                ManifestCustomValueKind::NonFungibleLocalId => Self::NonFungibleLocalId,
                ManifestCustomValueKind::AddressReservation => Self::AddressReservation,
            },
        }
    }
}

impl SerializableManifestValue {
    pub fn from_manifest_value(
        manifest_value: &ManifestValue,
        network_id: u8,
        bucket_name_mapping: &HashMap<ManifestBucket, String>,
        proof_name_mapping: &HashMap<ManifestProof, String>,
        address_reservation_name_mapping: &HashMap<ManifestAddressReservation, String>,
        named_address_name_mapping: &HashMap<u32, String>,
    ) -> Result<Self, ValueConversionError> {
        let value = match manifest_value {
            ManifestValue::Bool { value } => Self::Bool { value: *value },
            ManifestValue::I8 { value } => Self::I8 {
                value: into!(*value),
            },
            ManifestValue::I16 { value } => Self::I16 {
                value: into!(*value),
            },
            ManifestValue::I32 { value } => Self::I32 {
                value: into!(*value),
            },
            ManifestValue::I64 { value } => Self::I64 {
                value: into!(*value),
            },
            ManifestValue::I128 { value } => Self::I128 {
                value: into!(*value),
            },
            ManifestValue::U8 { value } => Self::U8 {
                value: into!(*value),
            },
            ManifestValue::U16 { value } => Self::U16 {
                value: into!(*value),
            },
            ManifestValue::U32 { value } => Self::U32 {
                value: into!(*value),
            },
            ManifestValue::U64 { value } => Self::U64 {
                value: into!(*value),
            },
            ManifestValue::U128 { value } => Self::U128 {
                value: into!(*value),
            },
            ManifestValue::String { value } => Self::String {
                value: value.to_owned(),
            },
            ManifestValue::Enum {
                discriminator,
                fields,
            } => Self::Enum {
                discriminator: into!(*discriminator),
                fields: fields
                    .iter()
                    .map(|value| {
                        Self::from_manifest_value(
                            value,
                            network_id,
                            bucket_name_mapping,
                            proof_name_mapping,
                            address_reservation_name_mapping,
                            named_address_name_mapping,
                        )
                    })
                    .collect::<Result<_, _>>()?,
            },
            ManifestValue::Array {
                element_value_kind,
                elements,
            } => Self::Array {
                element_value_kind: into!(*element_value_kind),
                elements: elements
                    .iter()
                    .map(|value| {
                        Self::from_manifest_value(
                            value,
                            network_id,
                            bucket_name_mapping,
                            proof_name_mapping,
                            address_reservation_name_mapping,
                            named_address_name_mapping,
                        )
                    })
                    .collect::<Result<_, _>>()?,
            },
            ManifestValue::Tuple { fields } => Self::Tuple {
                fields: fields
                    .iter()
                    .map(|value| {
                        Self::from_manifest_value(
                            value,
                            network_id,
                            bucket_name_mapping,
                            proof_name_mapping,
                            address_reservation_name_mapping,
                            named_address_name_mapping,
                        )
                    })
                    .collect::<Result<_, _>>()?,
            },
            ManifestValue::Map {
                key_value_kind,
                value_value_kind,
                entries,
            } => Self::Map {
                key_value_kind: into!(*key_value_kind),
                value_value_kind: into!(*value_value_kind),
                entries: entries
                    .iter()
                    .map(|(key, value)| {
                        Self::from_manifest_value(
                            key,
                            network_id,
                            bucket_name_mapping,
                            proof_name_mapping,
                            address_reservation_name_mapping,
                            named_address_name_mapping,
                        )
                        .and_then(|key| {
                            Self::from_manifest_value(
                                value,
                                network_id,
                                bucket_name_mapping,
                                proof_name_mapping,
                                address_reservation_name_mapping,
                                named_address_name_mapping,
                            )
                            .map(|value| (key, value))
                        })
                    })
                    .collect::<Result<_, _>>()?,
            },
            ManifestValue::Custom {
                value: ManifestCustomValue::Address(value),
            } => match value {
                ManifestAddress::Named(named) => named_address_name_mapping
                    .get(named)
                    .map(|named_address| SerializableNamedAddress(named_address.clone()))
                    .map(|value| Self::Address {
                        value: SerializableManifestAddress::Named { value },
                    })
                    .ok_or(ValueConversionError::NamedAddressHasNoAssociatedName(
                        *named,
                    ))?,
                ManifestAddress::Static(node_id) => Self::Address {
                    value: SerializableManifestAddress::Static {
                        value: SerializableNodeId(SerializableNodeIdInternal {
                            node_id: *node_id,
                            network_id,
                        }),
                    },
                },
            },
            ManifestValue::Custom {
                value: ManifestCustomValue::Bucket(value),
            } => bucket_name_mapping
                .get(value)
                .map(|bucket_name| SerializableBucketId(bucket_name.clone()))
                .map(|bucket_id| SerializableManifestValue::Bucket { value: bucket_id })
                .ok_or(ValueConversionError::BucketHasNoAssociatedName(value.0))?,
            ManifestValue::Custom {
                value: ManifestCustomValue::Proof(value),
            } => proof_name_mapping
                .get(value)
                .map(|proof_name| SerializableProofId(proof_name.clone()))
                .map(|proof_id| SerializableManifestValue::Proof { value: proof_id })
                .ok_or(ValueConversionError::ProofHasNoAssociatedName(value.0))?,
            ManifestValue::Custom {
                value: ManifestCustomValue::Expression(value),
            } => SerializableManifestValue::Expression {
                value: into!(*value),
            },
            ManifestValue::Custom {
                value: ManifestCustomValue::Blob(value),
            } => SerializableManifestValue::Blob {
                value: into!(value.0),
            },
            ManifestValue::Custom {
                value: ManifestCustomValue::Decimal(value),
            } => SerializableManifestValue::Decimal {
                value: into!(to_decimal(value)),
            },
            ManifestValue::Custom {
                value: ManifestCustomValue::PreciseDecimal(value),
            } => SerializableManifestValue::PreciseDecimal {
                value: into!(to_precise_decimal(value)),
            },
            ManifestValue::Custom {
                value: ManifestCustomValue::NonFungibleLocalId(value),
            } => SerializableManifestValue::NonFungibleLocalId {
                value: into!(to_non_fungible_local_id(value.clone())),
            },
            ManifestValue::Custom {
                value: ManifestCustomValue::AddressReservation(value),
            } => address_reservation_name_mapping
                .get(value)
                .map(|reservation_name| SerializableAddressReservation(reservation_name.clone()))
                .map(
                    |reservation| SerializableManifestValue::AddressReservation {
                        value: reservation,
                    },
                )
                .ok_or(ValueConversionError::AddressReservationHasNoAssociatedName(
                    value.0,
                ))?,
        };
        Ok(value)
    }

    pub fn to_manifest_value(
        &self,
        name_resolver: &mut NameResolver,
    ) -> Result<ManifestValue, ValueConversionError> {
        let value = match self {
            Self::Bool { value } => ManifestValue::Bool { value: *value },
            Self::I8 { value } => ManifestValue::I8 { value: value.0 },
            Self::I16 { value } => ManifestValue::I16 { value: value.0 },
            Self::I32 { value } => ManifestValue::I32 { value: value.0 },
            Self::I64 { value } => ManifestValue::I64 { value: value.0 },
            Self::I128 { value } => ManifestValue::I128 { value: value.0 },
            Self::U8 { value } => ManifestValue::U8 { value: value.0 },
            Self::U16 { value } => ManifestValue::U16 { value: value.0 },
            Self::U32 { value } => ManifestValue::U32 { value: value.0 },
            Self::U64 { value } => ManifestValue::U64 { value: value.0 },
            Self::U128 { value } => ManifestValue::U128 { value: value.0 },
            Self::String { value } => ManifestValue::String {
                value: value.to_owned(),
            },
            Self::Enum {
                discriminator,
                fields,
            } => ManifestValue::Enum {
                discriminator: discriminator.0,
                fields: fields
                    .iter()
                    .map(|value| value.to_manifest_value(name_resolver))
                    .collect::<Result<_, _>>()?,
            },
            Self::Array {
                element_value_kind,
                elements,
            } => ManifestValue::Array {
                element_value_kind: into!(*element_value_kind),
                elements: elements
                    .iter()
                    .map(|value| value.to_manifest_value(name_resolver))
                    .collect::<Result<_, _>>()?,
            },
            Self::Tuple { fields } => ManifestValue::Tuple {
                fields: fields
                    .iter()
                    .map(|value| value.to_manifest_value(name_resolver))
                    .collect::<Result<_, _>>()?,
            },
            Self::Map {
                key_value_kind,
                value_value_kind,
                entries,
            } => ManifestValue::Map {
                key_value_kind: into!(*key_value_kind),
                value_value_kind: into!(*value_value_kind),
                entries: entries
                    .iter()
                    .map(|(key, value)| {
                        key.to_manifest_value(name_resolver).and_then(|key| {
                            value
                                .to_manifest_value(name_resolver)
                                .map(|value| (key, value))
                        })
                    })
                    .collect::<Result<_, _>>()?,
            },
            Self::Bucket { value } => ManifestValue::Custom {
                value: name_resolver
                    .resolve_bucket(&value.0)
                    .map_err(|_| ValueConversionError::NamedBucketNotFound(value.0.clone()))
                    .map(ManifestCustomValue::Bucket)?,
            },
            Self::Proof { value } => ManifestValue::Custom {
                value: name_resolver
                    .resolve_proof(&value.0)
                    .map_err(|_| ValueConversionError::NamedProofNotFound(value.0.clone()))
                    .map(ManifestCustomValue::Proof)?,
            },
            Self::AddressReservation { value } => ManifestValue::Custom {
                value: name_resolver
                    .resolve_address_reservation(&value.0)
                    .map_err(|_| {
                        ValueConversionError::NamedAddressReservationNotFound(value.0.clone())
                    })
                    .map(ManifestCustomValue::AddressReservation)?,
            },
            Self::Expression { value } => ManifestValue::Custom {
                value: ManifestCustomValue::Expression(into!(*value)),
            },
            Self::Blob { value } => ManifestValue::Custom {
                value: ManifestCustomValue::Blob(ManifestBlobRef(into!(*value))),
            },
            Self::Decimal { value } => ManifestValue::Custom {
                value: ManifestCustomValue::Decimal(from_decimal(&value.0)),
            },
            Self::PreciseDecimal { value } => ManifestValue::Custom {
                value: ManifestCustomValue::PreciseDecimal(from_precise_decimal(&value.0)),
            },
            Self::NonFungibleLocalId { value } => ManifestValue::Custom {
                value: ManifestCustomValue::NonFungibleLocalId(from_non_fungible_local_id(
                    value.0.clone(),
                )),
            },
            Self::Address { value } => match value {
                SerializableManifestAddress::Static { value } => ManifestValue::Custom {
                    value: ManifestCustomValue::Address(ManifestAddress::Static(value.0.node_id)),
                },
                SerializableManifestAddress::Named { value } => ManifestValue::Custom {
                    value: ManifestCustomValue::Address(ManifestAddress::Named(
                        name_resolver.resolve_named_address(&value.0).map_err(|_| {
                            ValueConversionError::NamedAddressNotFound(value.0.clone())
                        })?,
                    )),
                },
            },
        };
        Ok(value)
    }
}

pub enum ValueConversionError {
    BucketHasNoAssociatedName(u32),
    ProofHasNoAssociatedName(u32),
    AddressReservationHasNoAssociatedName(u32),
    NamedAddressHasNoAssociatedName(u32),

    NamedBucketNotFound(String),
    NamedProofNotFound(String),
    NamedAddressReservationNotFound(String),
    NamedAddressNotFound(String),
}

macro_rules! into {
    ($expr: expr) => {
        Into::into($expr)
    };
}
use into;
