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

#![allow(dead_code)]
#![macro_use]

use std::collections::BTreeMap;

use native_transaction::data::{ManifestBlobRef, ManifestExpression};
use native_transaction::manifest::generator::{generate_value, NameResolver};
use native_transaction::manifest::lexer::tokenize;
use native_transaction_data::ManifestValue;
use radix_engine_toolkit::model::address::*;
use radix_engine_toolkit::model::engine_identifier::{BucketId, ProofId, TransientIdentifier};
use radix_engine_toolkit::model::value::ast::{
    EnumDiscriminator, ManifestAstValue, ManifestAstValueKind,
};
use scrypto::prelude::{
    BytesNonFungibleLocalId, Hash, IntegerNonFungibleLocalId, StringNonFungibleLocalId,
    UUIDNonFungibleLocalId,
};
use scrypto_utils::copy_u8_array;
extern crate lazy_static;

pub struct ManifestAstValueRepresentationTestVector {
    pub value: ManifestAstValue,
    pub json_representation: String,
    pub manifest_representation: String,
}

impl ManifestAstValueRepresentationTestVector {
    pub fn new<S: AsRef<str>, T: AsRef<str>>(
        value: ManifestAstValue,
        json_representation: S,
        manifest_representation: T,
    ) -> Self {
        Self {
            value,
            json_representation: json_representation.as_ref().into(),
            manifest_representation: manifest_representation.as_ref().into(),
        }
    }

    pub fn manifest_representation_as_ast_value(&self) -> native_transaction::manifest::ast::Value {
        native_transaction::manifest::parser::Parser::new(
            tokenize(&self.manifest_representation).expect("Failed to tokenize trusted value"),
        )
        .parse_value()
        .expect("Failed to parse trusted value to ast value")
    }

    pub fn manifest_representation_as_manifest_value(
        &self,
        bech32_coder: &Bech32Coder,
    ) -> ManifestValue {
        let mut blobs = BTreeMap::<Hash, Vec<u8>>::new();
        if let ManifestAstValue::Blob { ref hash } = self.value {
            blobs.insert(Hash(hash.0), Vec::new());
        };

        let ast_value = self.manifest_representation_as_ast_value();
        generate_value(
            &ast_value,
            None,
            &mut NameResolver::new(),
            bech32_coder.decoder(),
            &blobs,
        )
        .expect("Failed to generate scrypto value from ast_value")
    }
}

lazy_static::lazy_static! {
    pub static ref VALUE_CONVERSION_TEST_VECTORS: Vec<ManifestAstValueRepresentationTestVector> = vec![
        // ================
        // Primitive Types
        // ================

        // Unit and Boolean
        ManifestAstValueRepresentationTestVector::new(
            ManifestAstValue::Bool { value: true },
            r#"{"type": "Bool", "value": true}"#,
            r#"true"#,
        ),
        ManifestAstValueRepresentationTestVector::new(
            ManifestAstValue::Bool { value: false },
            r#"{"type": "Bool", "value": false}"#,
            r#"false"#,
        ),
        // Unsigned Integers
        ManifestAstValueRepresentationTestVector::new(
            ManifestAstValue::U8 { value: 19 },
            r#"{"type": "U8", "value": "19"}"#,
            "19u8"
        ),
        ManifestAstValueRepresentationTestVector::new(
            ManifestAstValue::U16 { value: 19 },
            r#"{"type": "U16", "value": "19"}"#,
            "19u16"
        ),
        ManifestAstValueRepresentationTestVector::new(
            ManifestAstValue::U32 { value: 19 },
            r#"{"type": "U32", "value": "19"}"#,
            "19u32"
        ),
        ManifestAstValueRepresentationTestVector::new(
            ManifestAstValue::U64 { value: 19 },
            r#"{"type": "U64", "value": "19"}"#,
            "19u64"
        ),
        ManifestAstValueRepresentationTestVector::new(
            ManifestAstValue::U128 { value: 19 },
            r#"{"type": "U128", "value": "19"}"#,
            "19u128"
        ),
        // Signed Integers
        ManifestAstValueRepresentationTestVector::new(
            ManifestAstValue::I8 { value: 19 },
            r#"{"type": "I8", "value": "19"}"#,
            "19i8"
        ),
        ManifestAstValueRepresentationTestVector::new(
            ManifestAstValue::I16 { value: 19 },
            r#"{"type": "I16", "value": "19"}"#,
            "19i16"
        ),
        ManifestAstValueRepresentationTestVector::new(
            ManifestAstValue::I32 { value: 19 },
            r#"{"type": "I32", "value": "19"}"#,
            "19i32"
        ),
        ManifestAstValueRepresentationTestVector::new(
            ManifestAstValue::I64 { value: 19 },
            r#"{"type": "I64", "value": "19"}"#,
            "19i64"
        ),
        ManifestAstValueRepresentationTestVector::new(
            ManifestAstValue::I128 { value: 19 },
            r#"{"type": "I128", "value": "19"}"#,
            "19i128"
        ),
        // String
        ManifestAstValueRepresentationTestVector::new(
            ManifestAstValue::String {
                value: "P2P Cash System".into(),
            },
            r#"{"type": "String", "value": "P2P Cash System"}"#,
            r#""P2P Cash System""#,
        ),
        // Enums and Enum Aliases (Option & Result)
        ManifestAstValueRepresentationTestVector::new(
            ManifestAstValue::Enum {
                variant: EnumDiscriminator::U8{discriminator: 1},
                fields: Some(vec![ManifestAstValue::String {
                    value: "Component".into(),
                }]),
            },
            r#"{"type": "Enum", "variant": {"type": "U8", "discriminator": "1"}, "fields": [{"type": "String", "value": "Component"}]}"#,
            r#"Enum("Option::Some", "Component")"#,
        ),
        ManifestAstValueRepresentationTestVector::new(
            ManifestAstValue::Some {
                value: Box::new(ManifestAstValue::String {
                    value: "Component".into(),
                }),
            },
            r#"{"type": "Some", "value": {"type": "String", "value": "Component"}}"#,
            r#"Some("Component")"#,
        ),
        ManifestAstValueRepresentationTestVector::new(
            ManifestAstValue::None,
            r#"{"type": "None"}"#,
            r#"None"#,
        ),
        ManifestAstValueRepresentationTestVector::new(
            ManifestAstValue::Ok {
                value: Box::new(ManifestAstValue::String {
                    value: "Component".into(),
                }),
            },
            r#"{"type": "Ok", "value": {"type": "String", "value": "Component"}}"#,
            r#"Ok("Component")"#,
        ),
        ManifestAstValueRepresentationTestVector::new(
            ManifestAstValue::Err {
                value: Box::new(ManifestAstValue::String {
                    value: "Component".into(),
                }),
            },
            r#"{"type": "Err", "value": {"type": "String", "value": "Component"}}"#,
            r#"Err("Component")"#,
        ),
        // =================
        // Collection Types
        // =================
        ManifestAstValueRepresentationTestVector::new(
            ManifestAstValue::Array {
                element_kind: ManifestAstValueKind::String,
                elements: vec![ManifestAstValue::String {
                    value: "World, Hello!".into(),
                }],
            },
            r#"{"type": "Array", "element_kind": "String", "elements": [{"type": "String", "value": "World, Hello!"}]}"#,
            r#"Array<String>("World, Hello!")"#,
        ),
        ManifestAstValueRepresentationTestVector::new(
            ManifestAstValue::Map {
                key_value_kind: ManifestAstValueKind::String,
                value_value_kind: ManifestAstValueKind::U16,
                entries: vec![
                    (
                        ManifestAstValue::String {
                            value: "Hello, World!".into()
                        },
                        ManifestAstValue::U16 { value: 919 }
                    ),
                    (
                        ManifestAstValue::String {
                            value: "World, Hello!".into()
                        },
                        ManifestAstValue::U16 { value: 111 }
                    )
                ]
            },
            r#"{"type": "Map", "key_value_kind": "String", "value_value_kind": "U16", "entries": [[{"type":"String","value":"Hello, World!"},{"type":"U16","value":"919"}],[{"type":"String","value":"World, Hello!"},{"type":"U16","value":"111"}]]}"#,
            r#"Map<String, U16>("Hello, World!", 919u16, "World, Hello!", 111u16)"#,
        ),
        ManifestAstValueRepresentationTestVector::new(
            ManifestAstValue::Tuple {
                elements: vec![ManifestAstValue::I64 { value: 19 }, ManifestAstValue::I8 { value: 19 }],
            },
            r#"{"type": "Tuple", "elements": [{"type": "I64", "value": "19"}, {"type": "I8", "value": "19"}]}"#,
            "Tuple(19i64, 19i8)"
        ),
        // ============================
        // Decimal And Precise Decimal
        // ============================
        ManifestAstValueRepresentationTestVector::new(
            ManifestAstValue::Decimal {
                value: "1923319912.102221313".parse().unwrap(),
            },
            r#"{"type": "Decimal", "value": "1923319912.102221313"}"#,
            r#"Decimal("1923319912.102221313")"#,
        ),
        ManifestAstValueRepresentationTestVector::new(
            ManifestAstValue::PreciseDecimal {
                value: "1923319912.102221313".parse().unwrap(),
            },
            r#"{"type": "PreciseDecimal", "value": "1923319912.102221313"}"#,
            r#"PreciseDecimal("1923319912.102221313")"#,
        ),
        // ==============
        // Address Types
        // ==============
        ManifestAstValueRepresentationTestVector::new(
            ManifestAstValue::ComponentAddress {
                address: NetworkAwareComponentAddress {
                    network_id: 0xf2,
                    address: scrypto::prelude::ComponentAddress::Account([0; 26]),
                },
            },
            r#"{"type": "ComponentAddress", "address": "account_sim1qvqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqg5cu7q"}"#,
            r#"ComponentAddress("account_sim1qvqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqg5cu7q")"#,
        ),
        ManifestAstValueRepresentationTestVector::new(
            ManifestAstValue::ResourceAddress {
                address: NetworkAwareResourceAddress {
                    network_id: 0xf2,
                    address: scrypto::prelude::ResourceAddress::Normal([0; 26]),
                },
            },
            r#"{"type": "ResourceAddress", "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety"}"#,
            r#"ResourceAddress("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety")"#,
        ),
        ManifestAstValueRepresentationTestVector::new(
            ManifestAstValue::PackageAddress {
                address: NetworkAwarePackageAddress {
                    network_id: 0xf2,
                    address: scrypto::prelude::PackageAddress::Normal([0; 26]),
                },
            },
            r#"{"type": "PackageAddress", "address": "package_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqxrmwtq"}"#,
            r#"PackageAddress("package_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqxrmwtq")"#,
        ),

        // ===================
        // Buckets and Proofs
        // ===================
        ManifestAstValueRepresentationTestVector::new(
            ManifestAstValue::Bucket { identifier: BucketId(TransientIdentifier::String{ value: "xrd_bucket".into()}) },
            r#"{"type": "Bucket", "identifier": {"type": "String", "value": "xrd_bucket"}}"#,
            r#"Bucket("xrd_bucket")"#
        ),
        ManifestAstValueRepresentationTestVector::new(
            ManifestAstValue::Bucket { identifier: BucketId(TransientIdentifier::U32{ value: 28}) },
            r#"{"type": "Bucket", "identifier": {"type": "U32", "value": "28"}}"#,
            r#"Bucket(28u32)"#
        ),
        ManifestAstValueRepresentationTestVector::new(
            ManifestAstValue::Proof { identifier: ProofId(TransientIdentifier::String{ value: "xrd_proof".into()}) },
            r#"{"type": "Proof", "identifier": {"type": "String", "value": "xrd_proof"}}"#,
            r#"Proof("xrd_proof")"#
        ),
        ManifestAstValueRepresentationTestVector::new(
            ManifestAstValue::Proof { identifier: ProofId(TransientIdentifier::U32{ value: 28}) },
            r#"{"type": "Proof", "identifier": {"type": "U32", "value": "28"}}"#,
            r#"Proof(28u32)"#
        ),

        // ==========================
        // Non Fungible Id & Address
        // ==========================

        ManifestAstValueRepresentationTestVector::new(
            ManifestAstValue::NonFungibleLocalId { value: scrypto::prelude::NonFungibleLocalId::Integer(IntegerNonFungibleLocalId::new(114441894733333)) },
            r#"{"type": "NonFungibleLocalId", "value": {"type": "Integer", "value": "114441894733333"}}"#,
            "NonFungibleLocalId(\"#114441894733333#\")"
        ),
        ManifestAstValueRepresentationTestVector::new(
            ManifestAstValue::NonFungibleLocalId { value: scrypto::prelude::NonFungibleLocalId::UUID(UUIDNonFungibleLocalId::new(238510006928098330588051703199685491739).unwrap()) },
            r#"{"type": "NonFungibleLocalId", "value": {"type": "UUID", "value": "238510006928098330588051703199685491739"}}"#,
            r#"NonFungibleLocalId("{b36f5b3f-835b-406c-980f-7788d8f13c1b}")"#,
        ),
        ManifestAstValueRepresentationTestVector::new(
            ManifestAstValue::NonFungibleLocalId { value: scrypto::prelude::NonFungibleLocalId::String(StringNonFungibleLocalId::new("hello_world".into()).unwrap()) },
            r#"{"type": "NonFungibleLocalId", "value": {"type": "String", "value": "hello_world"}}"#,
            r#"NonFungibleLocalId("<hello_world>")"#,
        ),
        ManifestAstValueRepresentationTestVector::new(
            ManifestAstValue::NonFungibleLocalId { value: scrypto::prelude::NonFungibleLocalId::Bytes(BytesNonFungibleLocalId::new(vec![0x10, 0xa2, 0x31, 0x01]).unwrap()) },
            r#"{"type": "NonFungibleLocalId", "value": {"type": "Bytes", "value": "10a23101"}}"#,
            r#"NonFungibleLocalId("[10a23101]")"#,
        ),

        ManifestAstValueRepresentationTestVector::new(
            ManifestAstValue::NonFungibleGlobalId {
                address: NonFungibleGlobalId {
                    resource_address: NetworkAwareResourceAddress {
                            network_id: 0xf2,
                            address: scrypto::prelude::ResourceAddress::Normal([0; 26]),
                        },
                    non_fungible_local_id: scrypto::prelude::NonFungibleLocalId::Integer(IntegerNonFungibleLocalId::new(114441894733333))
                }
            },
            r#"{"type": "NonFungibleGlobalId", "resource_address": {"type": "ResourceAddress", "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety"}, "non_fungible_local_id": {"type": "NonFungibleLocalId", "value": {"type": "Integer", "value": "114441894733333"}}}"#,
            r#"NonFungibleGlobalId("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety:#114441894733333#")"#,
        ),
        ManifestAstValueRepresentationTestVector::new(
            ManifestAstValue::NonFungibleGlobalId {
                address: NonFungibleGlobalId {
                    resource_address: NetworkAwareResourceAddress {
                            network_id: 0xf2,
                            address: scrypto::prelude::ResourceAddress::Normal([0; 26]),
                        },
                    non_fungible_local_id: scrypto::prelude::NonFungibleLocalId::UUID(UUIDNonFungibleLocalId::new(238510006928098330588051703199685491739).unwrap())
                }
            },
            r#"{"type": "NonFungibleGlobalId", "resource_address": {"type": "ResourceAddress", "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety"}, "non_fungible_local_id": {"type": "NonFungibleLocalId", "value": {"type": "UUID", "value": "238510006928098330588051703199685491739"}}}"#,
            r#"NonFungibleGlobalId("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety:{b36f5b3f-835b-406c-980f-7788d8f13c1b}")"#,
        ),
        ManifestAstValueRepresentationTestVector::new(
            ManifestAstValue::NonFungibleGlobalId {
                address: NonFungibleGlobalId {
                    resource_address: NetworkAwareResourceAddress {
                            network_id: 0xf2,
                            address: scrypto::prelude::ResourceAddress::Normal([0; 26]),
                        },
                    non_fungible_local_id: scrypto::prelude::NonFungibleLocalId::String(StringNonFungibleLocalId::new("hello_world".into()).unwrap())
                }
            },
            r#"{"type": "NonFungibleGlobalId", "resource_address": {"type": "ResourceAddress", "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety"}, "non_fungible_local_id": {"type": "NonFungibleLocalId", "value": {"type": "String", "value": "hello_world"}}}"#,
            r#"NonFungibleGlobalId("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety:<hello_world>")"#,
        ),
        ManifestAstValueRepresentationTestVector::new(
            ManifestAstValue::NonFungibleGlobalId {
                address: NonFungibleGlobalId {
                    resource_address: NetworkAwareResourceAddress {
                            network_id: 0xf2,
                            address: scrypto::prelude::ResourceAddress::Normal([0; 26]),
                        },
                    non_fungible_local_id: scrypto::prelude::NonFungibleLocalId::Bytes(BytesNonFungibleLocalId::new(vec![0x10, 0xa2, 0x31, 0x01]).unwrap())
                }
            },
            r#"{"type": "NonFungibleGlobalId", "resource_address": {"type": "ResourceAddress", "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety"}, "non_fungible_local_id": {"type": "NonFungibleLocalId", "value": {"type": "Bytes", "value": "10a23101"}}}"#,
            r#"NonFungibleGlobalId("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety:[10a23101]"))"#,
        ),

        // =================
        // Other Misc Types
        // =================
        ManifestAstValueRepresentationTestVector::new(
            ManifestAstValue::Blob { hash: ManifestBlobRef(copy_u8_array(&hex::decode("2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824").unwrap())) },
            r#"{"type": "Blob", "hash": "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"}"#,
            r#"Blob("2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824")"#
        ),
        ManifestAstValueRepresentationTestVector::new(
            ManifestAstValue::Expression { value: ManifestExpression::EntireAuthZone },
            r#"{"type": "Expression", "value": "ENTIRE_AUTH_ZONE"}"#,
            r#"Expression("ENTIRE_AUTH_ZONE")"#
        ),
        ManifestAstValueRepresentationTestVector::new(
            ManifestAstValue::Expression { value: ManifestExpression::EntireWorktop },
            r#"{"type": "Expression", "value": "ENTIRE_WORKTOP"}"#,
            r#"Expression("ENTIRE_WORKTOP")"#
        ),
        ManifestAstValueRepresentationTestVector::new(
            ManifestAstValue::Bytes { value: vec![0x12, 0x19, 0x12, 0x20, 0x8] },
            r#"{"type": "Bytes", "value": "1219122008"}"#,
            r#"Bytes("1219122008")"#
        ),
    ];
}
