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

use native_json_library::prelude::*;
use scrypto::prelude::*;

use super::traits::HasExamples;

impl<'f> HasExamples<'f> for SerializableManifestValue {
    fn examples() -> Vec<Self> {
        vec![
            Self::Bool { value: true },
            Self::Bool { value: false },
            Self::I8 {
                value: i8::MIN.into(),
            },
            Self::I8 {
                value: i8::MAX.into(),
            },
            Self::I16 {
                value: i16::MIN.into(),
            },
            Self::I16 {
                value: i16::MAX.into(),
            },
            Self::I32 {
                value: i32::MIN.into(),
            },
            Self::I32 {
                value: i32::MAX.into(),
            },
            Self::I64 {
                value: i64::MIN.into(),
            },
            Self::I64 {
                value: i64::MAX.into(),
            },
            Self::I128 {
                value: i128::MIN.into(),
            },
            Self::I128 {
                value: i128::MAX.into(),
            },
            Self::U8 {
                value: u8::MIN.into(),
            },
            Self::U8 {
                value: u8::MAX.into(),
            },
            Self::U16 {
                value: u16::MIN.into(),
            },
            Self::U16 {
                value: u16::MAX.into(),
            },
            Self::U32 {
                value: u32::MIN.into(),
            },
            Self::U32 {
                value: u32::MAX.into(),
            },
            Self::U64 {
                value: u64::MIN.into(),
            },
            Self::U64 {
                value: u64::MAX.into(),
            },
            Self::U128 {
                value: u128::MIN.into(),
            },
            Self::U128 {
                value: u128::MAX.into(),
            },
            Self::String {
                value: "Hello World".to_owned(),
            },
            Self::Enum {
                discriminator: 0.into(),
                fields: vec![],
            },
            Self::Enum {
                discriminator: 0.into(),
                fields: vec![
                    Self::Bool { value: true },
                    Self::Bool { value: false },
                    Self::U8 { value: 0.into() },
                    Self::String {
                        value: "Hello World".to_owned(),
                    },
                ],
            },
            Self::Array {
                element_value_kind: SerializableManifestValueKind::Decimal,
                elements: vec![
                    Self::Decimal {
                        value: Decimal::MIN.into(),
                    },
                    Self::Decimal {
                        value: Decimal::MAX.into(),
                    },
                ],
            },
            Self::Tuple {
                fields: vec![
                    Self::Bool { value: true },
                    Self::Bool { value: false },
                    Self::U8 { value: 0.into() },
                    Self::String {
                        value: "Hello World".to_owned(),
                    },
                ],
            },
            Self::Map {
                key_value_kind: SerializableManifestValueKind::U8,
                value_value_kind: SerializableManifestValueKind::String,
                entries: vec![
                    (
                        Self::U8 { value: 0.into() },
                        Self::String { value: "A".into() },
                    ),
                    (
                        Self::U8 { value: 1.into() },
                        Self::String { value: "B".into() },
                    ),
                ],
            },
            Self::Address {
                value: SerializableManifestAddress::Static {
                    value: SerializableNodeId::from_global_address(RADIX_TOKEN, 0xf2),
                },
            },
            Self::Address {
                value: SerializableManifestAddress::Named {
                    value: SerializableNamedAddress::new(0),
                },
            },
            Self::Bucket {
                value: SerializableBucketId::new(0),
            },
            Self::Proof {
                value: SerializableProofId::new(0),
            },
            Self::AddressReservation {
                value: SerializableAddressReservation::new(0),
            },
            Self::Expression {
                value: SerializableExpression::EntireAuthZone,
            },
            Self::Expression {
                value: SerializableExpression::EntireWorktop,
            },
            Self::Blob {
                value: hash(b"Hello World").into(),
            },
            Self::Decimal {
                value: Decimal::MIN.into(),
            },
            Self::Decimal {
                value: Decimal::MAX.into(),
            },
            Self::PreciseDecimal {
                value: PreciseDecimal::MIN.into(),
            },
            Self::PreciseDecimal {
                value: PreciseDecimal::MAX.into(),
            },
            Self::NonFungibleLocalId {
                value: NonFungibleLocalId::string("Hello").unwrap().into(),
            },
            Self::NonFungibleLocalId {
                value: NonFungibleLocalId::integer(1).into(),
            },
            Self::NonFungibleLocalId {
                value: NonFungibleLocalId::bytes(vec![100]).unwrap().into(),
            },
            Self::NonFungibleLocalId {
                value: NonFungibleLocalId::from_str(
                    "{1111111111111111-1111111111111111-1111111111111111-1111111111111111}",
                )
                .unwrap()
                .into(),
            },
        ]
    }
}
