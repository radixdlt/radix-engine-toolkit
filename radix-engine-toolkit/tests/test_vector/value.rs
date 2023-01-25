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

use native_transaction::manifest::generator::{generate_value, NameResolver};
use native_transaction::manifest::lexer::tokenize;
use radix_engine_toolkit::{address::*, BucketId, ProofId, TransientIdentifier};
use radix_engine_toolkit::{Value, ValueKind};
use scrypto::prelude::{Hash, ScryptoValue};
use scrypto::runtime::ManifestBlobRef;
extern crate lazy_static;

pub struct ValueRepresentationTestVector {
    pub value: Value,
    pub json_representation: String,
    pub manifest_representation: String,
}

impl ValueRepresentationTestVector {
    pub fn new<S: AsRef<str>, T: AsRef<str>>(
        value: Value,
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

    pub fn manifest_representation_as_scrypto_value(
        &self,
        bech32_coder: &Bech32Coder,
    ) -> ScryptoValue {
        let mut blobs = BTreeMap::<Hash, Vec<u8>>::new();
        if let Value::Blob { ref hash } = self.value {
            blobs.insert(hash.0, Vec::new());
        };

        let ast_value = self.manifest_representation_as_ast_value();
        generate_value(
            &ast_value,
            None,
            &mut NameResolver::new(),
            &bech32_coder.decoder(),
            &blobs,
        )
        .expect("Failed to generate scrypto value from ast_value")
    }
}

lazy_static::lazy_static! {
    pub static ref VALUE_CONVERSION_TEST_VECTORS: Vec<ValueRepresentationTestVector> = vec![
        // ================
        // Primitive Types
        // ================

        // Unit and Boolean
        ValueRepresentationTestVector::new(
            Value::Bool { value: true },
            r#"{"type": "Bool", "value": true}"#,
            r#"true"#,
        ),
        ValueRepresentationTestVector::new(
            Value::Bool { value: false },
            r#"{"type": "Bool", "value": false}"#,
            r#"false"#,
        ),
        // Unsigned Integers
        ValueRepresentationTestVector::new(
            Value::U8 { value: 19 },
            r#"{"type": "U8", "value": "19"}"#,
            "19u8"
        ),
        ValueRepresentationTestVector::new(
            Value::U16 { value: 19 },
            r#"{"type": "U16", "value": "19"}"#,
            "19u16"
        ),
        ValueRepresentationTestVector::new(
            Value::U32 { value: 19 },
            r#"{"type": "U32", "value": "19"}"#,
            "19u32"
        ),
        ValueRepresentationTestVector::new(
            Value::U64 { value: 19 },
            r#"{"type": "U64", "value": "19"}"#,
            "19u64"
        ),
        ValueRepresentationTestVector::new(
            Value::U128 { value: 19 },
            r#"{"type": "U128", "value": "19"}"#,
            "19u128"
        ),
        // Signed Integers
        ValueRepresentationTestVector::new(
            Value::I8 { value: 19 },
            r#"{"type": "I8", "value": "19"}"#,
            "19i8"
        ),
        ValueRepresentationTestVector::new(
            Value::I16 { value: 19 },
            r#"{"type": "I16", "value": "19"}"#,
            "19i16"
        ),
        ValueRepresentationTestVector::new(
            Value::I32 { value: 19 },
            r#"{"type": "I32", "value": "19"}"#,
            "19i32"
        ),
        ValueRepresentationTestVector::new(
            Value::I64 { value: 19 },
            r#"{"type": "I64", "value": "19"}"#,
            "19i64"
        ),
        ValueRepresentationTestVector::new(
            Value::I128 { value: 19 },
            r#"{"type": "I128", "value": "19"}"#,
            "19i128"
        ),
        // String
        ValueRepresentationTestVector::new(
            Value::String {
                value: "P2P Cash System".into(),
            },
            r#"{"type": "String", "value": "P2P Cash System"}"#,
            r#""P2P Cash System""#,
        ),
        // Enums and Enum Aliases (Option & Result)
        ValueRepresentationTestVector::new(
            Value::Enum {
                variant: radix_engine_toolkit::EnumDiscriminator::U8{discriminator: 1},
                fields: Some(vec![Value::String {
                    value: "Component".into(),
                }]),
            },
            r#"{"type": "Enum", "variant": {"type": "U8", "discriminator": "1"}, "fields": [{"type": "String", "value": "Component"}]}"#,
            r#"Enum("Option::Some", "Component")"#,
        ),
        ValueRepresentationTestVector::new(
            Value::Some {
                value: Box::new(Value::String {
                    value: "Component".into(),
                }),
            },
            r#"{"type": "Some", "value": {"type": "String", "value": "Component"}}"#,
            r#"Some("Component")"#,
        ),
        ValueRepresentationTestVector::new(
            Value::None,
            r#"{"type": "None"}"#,
            r#"None"#,
        ),
        ValueRepresentationTestVector::new(
            Value::Ok {
                value: Box::new(Value::String {
                    value: "Component".into(),
                }),
            },
            r#"{"type": "Ok", "value": {"type": "String", "value": "Component"}}"#,
            r#"Ok("Component")"#,
        ),
        ValueRepresentationTestVector::new(
            Value::Err {
                value: Box::new(Value::String {
                    value: "Component".into(),
                }),
            },
            r#"{"type": "Err", "value": {"type": "String", "value": "Component"}}"#,
            r#"Err("Component")"#,
        ),
        // =================
        // Collection Types
        // =================
        ValueRepresentationTestVector::new(
            Value::Array {
                element_kind: ValueKind::String,
                elements: vec![Value::String {
                    value: "World, Hello!".into(),
                }],
            },
            r#"{"type": "Array", "element_kind": "String", "elements": [{"type": "String", "value": "World, Hello!"}]}"#,
            r#"Array<String>("World, Hello!")"#,
        ),
        ValueRepresentationTestVector::new(
            Value::Map {
                key_value_kind: ValueKind::String,
                value_value_kind: ValueKind::U16,
                entries: vec![
                    (
                        Value::String {
                            value: "Hello, World!".into()
                        },
                        Value::U16 { value: 919 }
                    ),
                    (
                        Value::String {
                            value: "World, Hello!".into()
                        },
                        Value::U16 { value: 111 }
                    )
                ]
            },
            r#"{"type": "Map", "key_value_kind": "String", "value_value_kind": "U16", "entries": [[{"type":"String","value":"Hello, World!"},{"type":"U16","value":"919"}],[{"type":"String","value":"World, Hello!"},{"type":"U16","value":"111"}]]}"#,
            r#"Map<String, U16>("Hello, World!", 919u16, "World, Hello!", 111u16)"#,
        ),
        ValueRepresentationTestVector::new(
            Value::Tuple {
                elements: vec![Value::I64 { value: 19 }, Value::I8 { value: 19 }],
            },
            r#"{"type": "Tuple", "elements": [{"type": "I64", "value": "19"}, {"type": "I8", "value": "19"}]}"#,
            "Tuple(19i64, 19i8)"
        ),
        // ============================
        // Decimal And Precise Decimal
        // ============================
        ValueRepresentationTestVector::new(
            Value::Decimal {
                value: "1923319912.102221313".parse().unwrap(),
            },
            r#"{"type": "Decimal", "value": "1923319912.102221313"}"#,
            r#"Decimal("1923319912.102221313")"#,
        ),
        ValueRepresentationTestVector::new(
            Value::PreciseDecimal {
                value: "1923319912.102221313".parse().unwrap(),
            },
            r#"{"type": "PreciseDecimal", "value": "1923319912.102221313"}"#,
            r#"PreciseDecimal("1923319912.102221313")"#,
        ),
        // ==============
        // Address Types
        // ==============
        ValueRepresentationTestVector::new(
            Value::ComponentAddress {
                address: NetworkAwareComponentAddress {
                    network_id: 0xf2,
                    address: scrypto::prelude::ComponentAddress::Account([0; 26]),
                },
            },
            r#"{"type": "ComponentAddress", "address": "account_sim1qvqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqg5cu7q"}"#,
            r#"ComponentAddress("account_sim1qvqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqg5cu7q")"#,
        ),
        ValueRepresentationTestVector::new(
            Value::ResourceAddress {
                address: NetworkAwareResourceAddress {
                    network_id: 0xf2,
                    address: scrypto::prelude::ResourceAddress::Normal([0; 26]),
                },
            },
            r#"{"type": "ResourceAddress", "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety"}"#,
            r#"ResourceAddress("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety")"#,
        ),
        ValueRepresentationTestVector::new(
            Value::PackageAddress {
                address: NetworkAwarePackageAddress {
                    network_id: 0xf2,
                    address: scrypto::prelude::PackageAddress::Normal([0; 26]),
                },
            },
            r#"{"type": "PackageAddress", "address": "package_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqxrmwtq"}"#,
            r#"PackageAddress("package_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqxrmwtq")"#,
        ),
        ValueRepresentationTestVector::new(
            Value::SystemAddress {
                address: NetworkAwareSystemAddress {
                    network_id: 0xf2,
                    address: scrypto::prelude::SystemAddress::EpochManager([0; 26]),
                },
            },
            r#"{"type": "SystemAddress", "address": "system_sim1qsqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqsglvqq"}"#,
            r#"SystemAddress("system_sim1qsqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqsglvqq")"#,
        ),
        // ==============
        // Cryptographic
        // ==============
        ValueRepresentationTestVector::new(
            Value::Hash { value: "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824".parse().unwrap() },
            r#"{"type": "Hash", "value": "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"}"#,
            r#"Hash("2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824")"#,
        ),
        ValueRepresentationTestVector::new(
            Value::EcdsaSecp256k1PublicKey { public_key: "0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798".parse().unwrap() },
            r#"{"type": "EcdsaSecp256k1PublicKey", "public_key": "0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798"}"#,
            r#"EcdsaSecp256k1PublicKey("0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798")"#,
        ),
        ValueRepresentationTestVector::new(
            Value::EddsaEd25519PublicKey { public_key: "4cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29".parse().unwrap() },
            r#"{"type": "EddsaEd25519PublicKey", "public_key": "4cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29"}"#,
            r#"EddsaEd25519PublicKey("4cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29")"#,
        ),
        ValueRepresentationTestVector::new(
            Value::EcdsaSecp256k1Signature { signature: "0079224ea514206706298d8d620f660828f7987068d6d02757e6f3cbbf4a51ab133395db69db1bc9b2726dd99e34efc252d8258dcb003ebaba42be349f50f7765e".parse().unwrap() },
            r#"{"type": "EcdsaSecp256k1Signature", "signature": "0079224ea514206706298d8d620f660828f7987068d6d02757e6f3cbbf4a51ab133395db69db1bc9b2726dd99e34efc252d8258dcb003ebaba42be349f50f7765e"}"#,
            r#"EcdsaSecp256k1Signature("0079224ea514206706298d8d620f660828f7987068d6d02757e6f3cbbf4a51ab133395db69db1bc9b2726dd99e34efc252d8258dcb003ebaba42be349f50f7765e")"#,
        ),
        ValueRepresentationTestVector::new(
            Value::EddsaEd25519Signature { signature: "ce993adc51111309a041faa65cbcf1154d21ed0ecdc2d54070bc90b9deb744aa8605b3f686fa178fba21070b4a4678e54eee3486a881e0e328251cd37966de09".parse().unwrap() },
            r#"{"type": "EddsaEd25519Signature", "signature": "ce993adc51111309a041faa65cbcf1154d21ed0ecdc2d54070bc90b9deb744aa8605b3f686fa178fba21070b4a4678e54eee3486a881e0e328251cd37966de09"}"#,
            r#"EddsaEd25519Signature("ce993adc51111309a041faa65cbcf1154d21ed0ecdc2d54070bc90b9deb744aa8605b3f686fa178fba21070b4a4678e54eee3486a881e0e328251cd37966de09")"#,
        ),

        // ===================
        // Buckets and Proofs
        // ===================
        ValueRepresentationTestVector::new(
            Value::Bucket { identifier: BucketId(TransientIdentifier::String{ identifier: "xrd_bucket".into()}) },
            r#"{"type": "Bucket", "variant": "String", "identifier": "xrd_bucket"}"#,
            r#"Bucket("xrd_bucket")"#
        ),
        ValueRepresentationTestVector::new(
            Value::Bucket { identifier: BucketId(TransientIdentifier::U32{ identifier: 28}) },
            r#"{"type": "Bucket", "variant": "U32", "identifier": "28"}"#,
            r#"Bucket(28u32)"#
        ),
        ValueRepresentationTestVector::new(
            Value::Proof { identifier: ProofId(TransientIdentifier::String{ identifier: "xrd_proof".into()}) },
            r#"{"type": "Proof", "variant": "String", "identifier": "xrd_proof"}"#,
            r#"Proof("xrd_proof")"#
        ),
        ValueRepresentationTestVector::new(
            Value::Proof { identifier: ProofId(TransientIdentifier::U32{ identifier: 28}) },
            r#"{"type": "Proof", "variant": "U32", "identifier": "28"}"#,
            r#"Proof(28u32)"#
        ),

        // ==========================
        // Non Fungible Id & Address
        // ==========================

        ValueRepresentationTestVector::new(
            Value::NonFungibleId { value: scrypto::prelude::NonFungibleId::Number(114441894733333) },
            r#"{"type": "NonFungibleId", "variant": "Number", "value": "114441894733333"}"#,
            r#"NonFungibleId(114441894733333u64)"#,
        ),
        ValueRepresentationTestVector::new(
            Value::NonFungibleId { value: scrypto::prelude::NonFungibleId::UUID(11444189334733333) },
            r#"{"type": "NonFungibleId", "variant": "UUID", "value": "11444189334733333"}"#,
            r#"NonFungibleId(11444189334733333u128)"#,
        ),
        ValueRepresentationTestVector::new(
            Value::NonFungibleId { value: scrypto::prelude::NonFungibleId::String("hello_world".into()) },
            r#"{"type": "NonFungibleId", "variant": "String", "value": "hello_world"}"#,
            r#"NonFungibleId("hello_world")"#,
        ),
        ValueRepresentationTestVector::new(
            Value::NonFungibleId { value: scrypto::prelude::NonFungibleId::Bytes(vec![0x10, 0xa2, 0x31, 0x01]) },
            r#"{"type": "NonFungibleId", "variant": "Bytes", "value": "10a23101"}"#,
            r#"NonFungibleId(Bytes("10a23101"))"#,
        ),

        ValueRepresentationTestVector::new(
            Value::NonFungibleAddress {
                address: radix_engine_toolkit::model::NonFungibleAddress {
                    resource_address: NetworkAwareResourceAddress {
                            network_id: 0xf2,
                            address: scrypto::prelude::ResourceAddress::Normal([0; 26]),
                        },
                    non_fungible_id: scrypto::prelude::NonFungibleId::Number(114441894733333)
                }
            },
            r#"{"type": "NonFungibleAddress", "resource_address": {"type": "ResourceAddress", "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety"}, "non_fungible_id": {"type": "NonFungibleId", "variant": "Number", "value": "114441894733333"}}"#,
            r#"NonFungibleAddress("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety", 114441894733333u64)"#,
        ),
        ValueRepresentationTestVector::new(
            Value::NonFungibleAddress {
                address: radix_engine_toolkit::model::NonFungibleAddress {
                    resource_address: NetworkAwareResourceAddress {
                            network_id: 0xf2,
                            address: scrypto::prelude::ResourceAddress::Normal([0; 26]),
                        },
                    non_fungible_id: scrypto::prelude::NonFungibleId::UUID(11444189334733333)
                }
            },
            r#"{"type": "NonFungibleAddress", "resource_address": {"type": "ResourceAddress", "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety"}, "non_fungible_id": {"type": "NonFungibleId", "variant": "UUID", "value": "11444189334733333"}}"#,
            r#"NonFungibleAddress("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety", 11444189334733333u128)"#,
        ),
        ValueRepresentationTestVector::new(
            Value::NonFungibleAddress {
                address: radix_engine_toolkit::model::NonFungibleAddress {
                    resource_address: NetworkAwareResourceAddress {
                            network_id: 0xf2,
                            address: scrypto::prelude::ResourceAddress::Normal([0; 26]),
                        },
                    non_fungible_id: scrypto::prelude::NonFungibleId::String("hello_world".into())
                }
            },
            r#"{"type": "NonFungibleAddress", "resource_address": {"type": "ResourceAddress", "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety"}, "non_fungible_id": {"type": "NonFungibleId", "variant": "String", "value": "hello_world"}}"#,
            r#"NonFungibleAddress("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety", "hello_world")"#,
        ),
        ValueRepresentationTestVector::new(
            Value::NonFungibleAddress {
                address: radix_engine_toolkit::model::NonFungibleAddress {
                    resource_address: NetworkAwareResourceAddress {
                            network_id: 0xf2,
                            address: scrypto::prelude::ResourceAddress::Normal([0; 26]),
                        },
                    non_fungible_id: scrypto::prelude::NonFungibleId::Bytes(vec![0x10, 0xa2, 0x31, 0x01])
                }
            },
            r#"{"type": "NonFungibleAddress", "resource_address": {"type": "ResourceAddress", "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety"}, "non_fungible_id": {"type": "NonFungibleId", "variant": "Bytes", "value": "10a23101"}}"#,
            r#"NonFungibleAddress("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety", Bytes("10a23101"))"#,
        ),

        // =================
        // Other Misc Types
        // =================
        ValueRepresentationTestVector::new(
            Value::Blob { hash: ManifestBlobRef("2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824".parse().unwrap()) },
            r#"{"type": "Blob", "hash": "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"}"#,
            r#"Blob("2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824")"#
        ),
        ValueRepresentationTestVector::new(
            Value::Expression { value: scrypto::runtime::ManifestExpression::EntireAuthZone },
            r#"{"type": "Expression", "value": "ENTIRE_AUTH_ZONE"}"#,
            r#"Expression("ENTIRE_AUTH_ZONE")"#
        ),
        ValueRepresentationTestVector::new(
            Value::Expression { value: scrypto::runtime::ManifestExpression::EntireWorktop },
            r#"{"type": "Expression", "value": "ENTIRE_WORKTOP"}"#,
            r#"Expression("ENTIRE_WORKTOP")"#
        ),
        ValueRepresentationTestVector::new(
            Value::Bytes { value: vec![0x12, 0x19, 0x12, 0x20, 0x8] },
            r#"{"type": "Bytes", "value": "1219122008"}"#,
            r#"Bytes("1219122008")"#
        ),
    ];
}
