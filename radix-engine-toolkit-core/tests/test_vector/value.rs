// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at

//   http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

#![allow(dead_code)]
#![macro_use]
extern crate lazy_static;

use radix_engine_toolkit_core::error::Error;
use radix_engine_toolkit_core::model::*;
use radix_transaction::manifest::ast::Type as AstValueKind;
use radix_transaction::manifest::generator::{generate_value, NameResolver};
use radix_transaction::manifest::lexer::tokenize;
use sbor::rust::collections::IndexMap;
use scrypto::prelude::*;

pub struct ValueJsonRepresentationTestVector {
    pub value: Value,
    pub json_representation: String,
}

impl ValueJsonRepresentationTestVector {
    pub fn new<S: AsRef<str>>(value: Value, json_representation: S) -> Self {
        let json_representation: &str = json_representation.as_ref();
        let json_representation: String = json_representation.into();
        Self {
            value,
            json_representation,
        }
    }
}

pub struct ValueAstConversionsTestVector {
    pub value: Value,
    pub manifest_representation: String,
}

impl ValueAstConversionsTestVector {
    pub fn new<S: AsRef<str>>(value: Value, manifest_representation: S) -> Self {
        let manifest_representation: &str = manifest_representation.as_ref();
        let manifest_representation: String = manifest_representation.into();
        Self {
            manifest_representation,
            value,
        }
    }

    pub fn manifest_representation_as_ast_value(&self) -> radix_transaction::manifest::ast::Value {
        radix_transaction::manifest::parser::Parser::new(
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
            &bech32_coder.decoder,
            &blobs,
        )
        .expect("Failed to generate scrypto value from ast_value")
    }
}

pub struct ValueValidationTestVector {
    pub value: Value,
    pub validation_result: Result<(), Error>,
}

impl ValueValidationTestVector {
    pub fn new(value: Value, validation_result: Result<(), Error>) -> Self {
        Self {
            value,
            validation_result,
        }
    }
}

pub struct ValueKindTestVector {
    pub value: Value,
    pub value_kind: ValueKind,
    pub ast_value_kind: AstValueKind,
}

impl ValueKindTestVector {
    pub fn new(value: Value, value_kind: ValueKind, ast_value_kind: AstValueKind) -> Self {
        Self {
            value,
            value_kind,
            ast_value_kind,
        }
    }
}

lazy_static::lazy_static! {
    pub static ref VALUE_JSON_CONVERSION_TEST_VECTORS: Vec<ValueJsonRepresentationTestVector> = vec![
        // ================
        // Primitive Types
        // ================

        // Unit and Boolean
        ValueJsonRepresentationTestVector::new(Value::Unit, r#"{"type": "Unit"}"#),
        ValueJsonRepresentationTestVector::new(
            Value::Bool { value: true },
            r#"{"type": "Bool", "value": true}"#,
        ),
        ValueJsonRepresentationTestVector::new(
            Value::Bool { value: false },
            r#"{"type": "Bool", "value": false}"#,
        ),
        // Unsigned Integers
        ValueJsonRepresentationTestVector::new(
            Value::U8 { value: 19 },
            r#"{"type": "U8", "value": "19"}"#,
        ),
        ValueJsonRepresentationTestVector::new(
            Value::U16 { value: 19 },
            r#"{"type": "U16", "value": "19"}"#,
        ),
        ValueJsonRepresentationTestVector::new(
            Value::U32 { value: 19 },
            r#"{"type": "U32", "value": "19"}"#,
        ),
        ValueJsonRepresentationTestVector::new(
            Value::U64 { value: 19 },
            r#"{"type": "U64", "value": "19"}"#,
        ),
        ValueJsonRepresentationTestVector::new(
            Value::U128 { value: 19 },
            r#"{"type": "U128", "value": "19"}"#,
        ),
        // Signed Integers
        ValueJsonRepresentationTestVector::new(
            Value::I8 { value: 19 },
            r#"{"type": "I8", "value": "19"}"#,
        ),
        ValueJsonRepresentationTestVector::new(
            Value::I16 { value: 19 },
            r#"{"type": "I16", "value": "19"}"#,
        ),
        ValueJsonRepresentationTestVector::new(
            Value::I32 { value: 19 },
            r#"{"type": "I32", "value": "19"}"#,
        ),
        ValueJsonRepresentationTestVector::new(
            Value::I64 { value: 19 },
            r#"{"type": "I64", "value": "19"}"#,
        ),
        ValueJsonRepresentationTestVector::new(
            Value::I128 { value: 19 },
            r#"{"type": "I128", "value": "19"}"#,
        ),
        // String
        ValueJsonRepresentationTestVector::new(
            Value::String {
                value: "P2P Cash System".into(),
            },
            r#"{"type": "String", "value": "P2P Cash System"}"#,
        ),
        // Enums and Enum Aliases (Option & Result)
        ValueJsonRepresentationTestVector::new(
            Value::Enum {
                variant: "Create".into(),
                fields: Some(vec![Value::String {
                    value: "Component".into(),
                }]),
            },
            r#"{"type": "Enum", "variant": "Create", "fields": [{"type": "String", "value": "Component"}]}"#,
        ),
        ValueJsonRepresentationTestVector::new(
            Value::Option {
                value: Box::new(Some(Value::String {
                    value: "Component".into(),
                })),
            },
            r#"{"type": "Option", "variant": "Some", "field": {"type": "String", "value": "Component"}}"#,
        ),
        ValueJsonRepresentationTestVector::new(
            Value::Option {
                value: Box::new(None),
            },
            r#"{"type": "Option", "variant": "None"}"#,
        ),
        ValueJsonRepresentationTestVector::new(
            Value::Result {
                value: Box::new(Ok(Value::String {
                    value: "Component".into(),
                })),
            },
            r#"{"type": "Result", "variant": "Ok", "field": {"type": "String", "value": "Component"}}"#,
        ),
        ValueJsonRepresentationTestVector::new(
            Value::Result {
                value: Box::new(Err(Value::String {
                    value: "Component".into(),
                })),
            },
            r#"{"type": "Result", "variant": "Err", "field": {"type": "String", "value": "Component"}}"#,
        ),
        // =================
        // Collection Types
        // =================
        ValueJsonRepresentationTestVector::new(
            Value::Array {
                element_type: ValueKind::String,
                elements: vec![Value::String {
                    value: "World, Hello!".into(),
                }],
            },
            r#"{"type": "Array", "element_type": "String", "elements": [{"type": "String", "value": "World, Hello!"}]}"#,
        ),
        ValueJsonRepresentationTestVector::new(
            Value::Tuple {
                elements: vec![Value::I64 { value: 19 }, Value::I8 { value: 19 }],
            },
            r#"{"type": "Tuple", "elements": [{"type": "I64", "value": "19"}, {"type": "I8", "value": "19"}]}"#,
        ),
        // ============================
        // Decimal And Precise Decimal
        // ============================
        ValueJsonRepresentationTestVector::new(
            Value::Decimal {
                value: "1923319912.102221313".parse().unwrap(),
            },
            r#"{"type": "Decimal", "value": "1923319912.102221313"}"#,
        ),
        ValueJsonRepresentationTestVector::new(
            Value::PreciseDecimal {
                value: "1923319912.102221313".parse().unwrap(),
            },
            r#"{"type": "PreciseDecimal", "value": "1923319912.102221313"}"#,
        ),
        // ==============
        // Address Types
        // ==============
        ValueJsonRepresentationTestVector::new(
            Value::ComponentAddress {
                address: NetworkAwareComponentAddress {
                    network_id: 0xf2,
                    address: scrypto::prelude::ComponentAddress::Normal([0; 26]),
                },
            },
            r#"{"type": "ComponentAddress", "address": "component_sim1qgqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq8ecz5v"}"#,
        ),
        ValueJsonRepresentationTestVector::new(
            Value::ComponentAddress {
                address: NetworkAwareComponentAddress {
                    network_id: 0xf2,
                    address: scrypto::prelude::ComponentAddress::Account([0; 26]),
                },
            },
            r#"{"type": "ComponentAddress", "address": "account_sim1qvqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqg5cu7q"}"#,
        ),
        ValueJsonRepresentationTestVector::new(
            Value::ResourceAddress {
                address: NetworkAwareResourceAddress {
                    network_id: 0xf2,
                    address: scrypto::prelude::ResourceAddress::Normal([0; 26]),
                },
            },
            r#"{"type": "ResourceAddress", "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety"}"#,
        ),
        ValueJsonRepresentationTestVector::new(
            Value::PackageAddress {
                address: NetworkAwarePackageAddress {
                    network_id: 0xf2,
                    address: scrypto::prelude::PackageAddress::Normal([0; 26]),
                },
            },
            r#"{"type": "PackageAddress", "address": "package_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqxrmwtq"}"#,
        ),
        ValueJsonRepresentationTestVector::new(
            Value::SystemAddress {
                address: NetworkAwareSystemAddress {
                    network_id: 0xf2,
                    address: scrypto::prelude::SystemAddress::EpochManager([0; 26]),
                },
            },
            r#"{"type": "SystemAddress", "address": "system_sim1qsqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqsglvqq"}"#,
        ),
        ValueJsonRepresentationTestVector::new(
            Value::SystemAddress {
                address: NetworkAwareSystemAddress {
                    network_id: 0xf2,
                    address: scrypto::prelude::SystemAddress::Clock([0; 26]),
                },
            },
            r#"{"type": "SystemAddress", "address": "system_sim1q5qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz4jjwj"}"#,
        ),
        // ==============
        // Cryptographic
        // ==============
        ValueJsonRepresentationTestVector::new(
            Value::Hash { value: "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824".parse().unwrap() },
            r#"{"type": "Hash", "value": "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"}"#,
        ),
        ValueJsonRepresentationTestVector::new(
            Value::EcdsaSecp256k1PublicKey { public_key: "0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798".parse().unwrap() },
            r#"{"type": "EcdsaSecp256k1PublicKey", "public_key": "0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798"}"#,
        ),
        ValueJsonRepresentationTestVector::new(
            Value::EddsaEd25519PublicKey { public_key: "4cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29".parse().unwrap() },
            r#"{"type": "EddsaEd25519PublicKey", "public_key": "4cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29"}"#,
        ),
        ValueJsonRepresentationTestVector::new(
            Value::EcdsaSecp256k1Signature { signature: "0079224ea514206706298d8d620f660828f7987068d6d02757e6f3cbbf4a51ab133395db69db1bc9b2726dd99e34efc252d8258dcb003ebaba42be349f50f7765e".parse().unwrap() },
            r#"{"type": "EcdsaSecp256k1Signature", "signature": "0079224ea514206706298d8d620f660828f7987068d6d02757e6f3cbbf4a51ab133395db69db1bc9b2726dd99e34efc252d8258dcb003ebaba42be349f50f7765e"}"#,
        ),
        ValueJsonRepresentationTestVector::new(
            Value::EddsaEd25519Signature { signature: "ce993adc51111309a041faa65cbcf1154d21ed0ecdc2d54070bc90b9deb744aa8605b3f686fa178fba21070b4a4678e54eee3486a881e0e328251cd37966de09".parse().unwrap() },
            r#"{"type": "EddsaEd25519Signature", "signature": "ce993adc51111309a041faa65cbcf1154d21ed0ecdc2d54070bc90b9deb744aa8605b3f686fa178fba21070b4a4678e54eee3486a881e0e328251cd37966de09"}"#,
        ),

        // ===================
        // Buckets and Proofs
        // ===================
        ValueJsonRepresentationTestVector::new(
            Value::Bucket { identifier: BucketId(Identifier::String("xrd_bucket".into())) },
            r#"{"type": "Bucket", "identifier": "xrd_bucket"}"#
        ),
        ValueJsonRepresentationTestVector::new(
            Value::Bucket { identifier: BucketId(Identifier::U32(28)) },
            r#"{"type": "Bucket", "identifier": 28}"#
        ),
        ValueJsonRepresentationTestVector::new(
            Value::Proof { identifier: ProofId(Identifier::String("xrd_proof".into())) },
            r#"{"type": "Proof", "identifier": "xrd_proof"}"#
        ),
        ValueJsonRepresentationTestVector::new(
            Value::Proof { identifier: ProofId(Identifier::U32(28)) },
            r#"{"type": "Proof", "identifier": 28}"#
        ),

        // ==========================
        // Non Fungible Id & Address
        // ==========================

        ValueJsonRepresentationTestVector::new(
            Value::NonFungibleId { value: NonFungibleId::U32(1144418947) },
            r#"{"type": "NonFungibleId", "variant": "U32", "value": "1144418947"}"#,
        ),
        ValueJsonRepresentationTestVector::new(
            Value::NonFungibleId { value: NonFungibleId::U64(114441894733333) },
            r#"{"type": "NonFungibleId", "variant": "U64", "value": "114441894733333"}"#,
        ),
        ValueJsonRepresentationTestVector::new(
            Value::NonFungibleId { value: NonFungibleId::UUID(11444189334733333) },
            r#"{"type": "NonFungibleId", "variant": "UUID", "value": "11444189334733333"}"#,
        ),
        ValueJsonRepresentationTestVector::new(
            Value::NonFungibleId { value: NonFungibleId::String("hello_world".into()) },
            r#"{"type": "NonFungibleId", "variant": "String", "value": "hello_world"}"#,
        ),
        ValueJsonRepresentationTestVector::new(
            Value::NonFungibleId { value: NonFungibleId::Bytes(vec![0x10, 0xa2, 0x31, 0x01]) },
            r#"{"type": "NonFungibleId", "variant": "Bytes", "value": "10a23101"}"#,
        ),

        ValueJsonRepresentationTestVector::new(
            Value::NonFungibleAddress {
                address: radix_engine_toolkit_core::model::NonFungibleAddress {
                    resource_address: NetworkAwareResourceAddress {
                            network_id: 0xf2,
                            address: scrypto::prelude::ResourceAddress::Normal([0; 26]),
                        },
                    non_fungible_id: NonFungibleId::U32(1144418947)
                }
            },
            r#"{"type": "NonFungibleAddress", "resource_address": {"type": "ResourceAddress", "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety"}, "non_fungible_id": {"type": "NonFungibleId", "variant": "U32", "value": "1144418947"}}"#,
        ),
        ValueJsonRepresentationTestVector::new(
            Value::NonFungibleAddress {
                address: radix_engine_toolkit_core::model::NonFungibleAddress {
                    resource_address: NetworkAwareResourceAddress {
                            network_id: 0xf2,
                            address: scrypto::prelude::ResourceAddress::Normal([0; 26]),
                        },
                    non_fungible_id: NonFungibleId::U64(114441894733333)
                }
            },
            r#"{"type": "NonFungibleAddress", "resource_address": {"type": "ResourceAddress", "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety"}, "non_fungible_id": {"type": "NonFungibleId", "variant": "U64", "value": "114441894733333"}}"#,
        ),
        ValueJsonRepresentationTestVector::new(
            Value::NonFungibleAddress {
                address: radix_engine_toolkit_core::model::NonFungibleAddress {
                    resource_address: NetworkAwareResourceAddress {
                            network_id: 0xf2,
                            address: scrypto::prelude::ResourceAddress::Normal([0; 26]),
                        },
                    non_fungible_id: NonFungibleId::UUID(11444189334733333)
                }
            },
            r#"{"type": "NonFungibleAddress", "resource_address": {"type": "ResourceAddress", "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety"}, "non_fungible_id": {"type": "NonFungibleId", "variant": "UUID", "value": "11444189334733333"}}"#,
        ),
        ValueJsonRepresentationTestVector::new(
            Value::NonFungibleAddress {
                address: radix_engine_toolkit_core::model::NonFungibleAddress {
                    resource_address: NetworkAwareResourceAddress {
                            network_id: 0xf2,
                            address: scrypto::prelude::ResourceAddress::Normal([0; 26]),
                        },
                    non_fungible_id: NonFungibleId::String("hello_world".into())
                }
            },
            r#"{"type": "NonFungibleAddress", "resource_address": {"type": "ResourceAddress", "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety"}, "non_fungible_id": {"type": "NonFungibleId", "variant": "String", "value": "hello_world"}}"#,
        ),
        ValueJsonRepresentationTestVector::new(
            Value::NonFungibleAddress {
                address: radix_engine_toolkit_core::model::NonFungibleAddress {
                    resource_address: NetworkAwareResourceAddress {
                            network_id: 0xf2,
                            address: scrypto::prelude::ResourceAddress::Normal([0; 26]),
                        },
                    non_fungible_id: NonFungibleId::Bytes(vec![0x10, 0xa2, 0x31, 0x01])
                }
            },
            r#"{"type": "NonFungibleAddress", "resource_address": {"type": "ResourceAddress", "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety"}, "non_fungible_id": {"type": "NonFungibleId", "variant": "Bytes", "value": "10a23101"}}"#,
        ),

        // =================
        // Other Misc Types
        // =================
        ValueJsonRepresentationTestVector::new(
            Value::Blob { hash: Blob("2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824".parse().unwrap()) },
            r#"{"type": "Blob", "hash": "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"}"#
        ),
        ValueJsonRepresentationTestVector::new(
            Value::Expression { value: ManifestExpression::EntireAuthZone },
            r#"{"type": "Expression", "value": "ENTIRE_AUTH_ZONE"}"#
        ),
        ValueJsonRepresentationTestVector::new(
            Value::Expression { value: ManifestExpression::EntireWorktop },
            r#"{"type": "Expression", "value": "ENTIRE_WORKTOP"}"#
        ),
        ValueJsonRepresentationTestVector::new(
            Value::Bytes { value: vec![0x12, 0x19, 0x12, 0x20, 0x8] },
            r#"{"type": "Bytes", "value": "1219122008"}"#
        ),
    ];

    pub static ref VALUE_AST_CONVERSIONS_TEST_VECTORS: Vec<ValueAstConversionsTestVector> = vec![
        // ================
        // Primitive Types
        // ================

        // Unit and Boolean
        ValueAstConversionsTestVector::new(
            Value::Unit,
            r#"()"#
        ),
        ValueAstConversionsTestVector::new(
            Value::Bool { value: true },
            r#"true"#,
        ),
        ValueAstConversionsTestVector::new(
            Value::Bool { value: false },
            r#"false"#,
        ),
        // Unsigned Integers
        ValueAstConversionsTestVector::new(
            Value::U8 { value: 19 },
            "19u8"
        ),
        ValueAstConversionsTestVector::new(
            Value::U16 { value: 19 },
            "19u16"
        ),
        ValueAstConversionsTestVector::new(
            Value::U32 { value: 19 },
            "19u32"
        ),
        ValueAstConversionsTestVector::new(
            Value::U64 { value: 19 },
            "19u64"
        ),
        ValueAstConversionsTestVector::new(
            Value::U128 { value: 19 },
            "19u128"
        ),
        // Signed Integers
        ValueAstConversionsTestVector::new(
            Value::I8 { value: 19 },
            "19i8"
        ),
        ValueAstConversionsTestVector::new(
            Value::I16 { value: 19 },
            "19i16"
        ),
        ValueAstConversionsTestVector::new(
            Value::I32 { value: 19 },
            "19i32"
        ),
        ValueAstConversionsTestVector::new(
            Value::I64 { value: 19 },
            "19i64"
        ),
        ValueAstConversionsTestVector::new(
            Value::I128 { value: 19 },
            "19i128"
        ),
        // String
        ValueAstConversionsTestVector::new(
            Value::String {
                value: "P2P Cash System".into(),
            },
            r#""P2P Cash System""#,
        ),
        // Enums and Enum Aliases (Option & Result)
        ValueAstConversionsTestVector::new(
            Value::Enum {
                variant: "Create".into(),
                fields: Some(vec![Value::String {
                    value: "Component".into(),
                }]),
            },
            r#"Enum("Create", "Component")"#,
        ),
        ValueAstConversionsTestVector::new(
            Value::Option {
                value: Box::new(Some(Value::String {
                    value: "Component".into(),
                })),
            },
            r#"Some("Component")"#,
        ),
        ValueAstConversionsTestVector::new(
            Value::Option {
                value: Box::new(None),
            },
            r#"None"#,
        ),
        ValueAstConversionsTestVector::new(
            Value::Result {
                value: Box::new(Ok(Value::String {
                    value: "Component".into(),
                })),
            },
            r#"Ok("Component")"#,
        ),
        ValueAstConversionsTestVector::new(
            Value::Result {
                value: Box::new(Err(Value::String {
                    value: "Component".into(),
                })),
            },
            r#"Err("Component")"#,
        ),
        // =================
        // Collection Types
        // =================
        ValueAstConversionsTestVector::new(
            Value::Array {
                element_type: ValueKind::String,
                elements: vec![Value::String {
                    value: "World, Hello!".into(),
                }],
            },
            r#"Array<String>("World, Hello!")"#,
        ),
        ValueAstConversionsTestVector::new(
            Value::Tuple {
                elements: vec![Value::I64 { value: 19 }, Value::I8 { value: 19 }],
            },
            "Tuple(19i64, 19i8)"
        ),
        // ============================
        // Decimal And Precise Decimal
        // ============================
        ValueAstConversionsTestVector::new(
            Value::Decimal {
                value: "1923319912.102221313".parse().unwrap(),
            },
            r#"Decimal("1923319912.102221313")"#,
        ),
        ValueAstConversionsTestVector::new(
            Value::PreciseDecimal {
                value: "1923319912.102221313".parse().unwrap(),
            },
            r#"PreciseDecimal("1923319912.102221313")"#,
        ),
        // ==============
        // Address Types
        // ==============
        ValueAstConversionsTestVector::new(
            Value::ComponentAddress {
                address: NetworkAwareComponentAddress {
                    network_id: 0xf2,
                    address: scrypto::prelude::ComponentAddress::Normal([0; 26]),
                },
            },
            r#"ComponentAddress("component_sim1qgqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq8ecz5v")"#,
        ),
        ValueAstConversionsTestVector::new(
            Value::ResourceAddress {
                address: NetworkAwareResourceAddress {
                    network_id: 0xf2,
                    address: scrypto::prelude::ResourceAddress::Normal([0; 26]),
                },
            },
            r#"ResourceAddress("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety")"#,
        ),
        ValueAstConversionsTestVector::new(
            Value::PackageAddress {
                address: NetworkAwarePackageAddress {
                    network_id: 0xf2,
                    address: scrypto::prelude::PackageAddress::Normal([0; 26]),
                },
            },
            r#"PackageAddress("package_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqxrmwtq")"#,
        ),
        ValueAstConversionsTestVector::new(
            Value::SystemAddress {
                address: NetworkAwareSystemAddress {
                    network_id: 0xf2,
                    address: scrypto::prelude::SystemAddress::EpochManager([0; 26]),
                },
            },
            r#"SystemAddress("system_sim1qsqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqsglvqq")"#,
        ),
        // ==============
        // Cryptographic
        // ==============
        ValueAstConversionsTestVector::new(
            Value::Hash { value: "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824".parse().unwrap() },
            r#"Hash("2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824")"#,
        ),
        ValueAstConversionsTestVector::new(
            Value::EcdsaSecp256k1PublicKey { public_key: "0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798".parse().unwrap() },
            r#"EcdsaSecp256k1PublicKey("0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798")"#,
        ),
        ValueAstConversionsTestVector::new(
            Value::EddsaEd25519PublicKey { public_key: "4cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29".parse().unwrap() },
            r#"EddsaEd25519PublicKey("4cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29")"#,
        ),
        ValueAstConversionsTestVector::new(
            Value::EcdsaSecp256k1Signature { signature: "0079224ea514206706298d8d620f660828f7987068d6d02757e6f3cbbf4a51ab133395db69db1bc9b2726dd99e34efc252d8258dcb003ebaba42be349f50f7765e".parse().unwrap() },
            r#"EcdsaSecp256k1Signature("0079224ea514206706298d8d620f660828f7987068d6d02757e6f3cbbf4a51ab133395db69db1bc9b2726dd99e34efc252d8258dcb003ebaba42be349f50f7765e")"#,
        ),
        ValueAstConversionsTestVector::new(
            Value::EddsaEd25519Signature { signature: "ce993adc51111309a041faa65cbcf1154d21ed0ecdc2d54070bc90b9deb744aa8605b3f686fa178fba21070b4a4678e54eee3486a881e0e328251cd37966de09".parse().unwrap() },
            r#"EddsaEd25519Signature("ce993adc51111309a041faa65cbcf1154d21ed0ecdc2d54070bc90b9deb744aa8605b3f686fa178fba21070b4a4678e54eee3486a881e0e328251cd37966de09")"#,
        ),

        // ===================
        // Buckets and Proofs
        // ===================
        ValueAstConversionsTestVector::new(
            Value::Bucket { identifier: BucketId(Identifier::String("xrd_bucket".into())) },
            r#"Bucket("xrd_bucket")"#
        ),
        ValueAstConversionsTestVector::new(
            Value::Bucket { identifier: BucketId(Identifier::U32(28)) },
            r#"Bucket(28u32)"#
        ),
        ValueAstConversionsTestVector::new(
            Value::Proof { identifier: ProofId(Identifier::String("xrd_proof".into())) },
            r#"Proof("xrd_proof")"#
        ),
        ValueAstConversionsTestVector::new(
            Value::Proof { identifier: ProofId(Identifier::U32(28)) },
            r#"Proof(28u32)"#
        ),

        // ==========================
        // Non Fungible Id & Address
        // ==========================
        ValueAstConversionsTestVector::new(
            Value::NonFungibleId { value: NonFungibleId::U32(1144418947) },
            r#"NonFungibleId(1144418947u32)"#,
        ),
        ValueAstConversionsTestVector::new(
            Value::NonFungibleId { value: NonFungibleId::U64(114441894733333) },
            r#"NonFungibleId(114441894733333u64)"#,
        ),
        ValueAstConversionsTestVector::new(
            Value::NonFungibleId { value: NonFungibleId::UUID(11444189334733333) },
            r#"NonFungibleId(11444189334733333u128)"#,
        ),
        ValueAstConversionsTestVector::new(
            Value::NonFungibleId { value: NonFungibleId::String("hello_world".into()) },
            r#"NonFungibleId("hello_world")"#,
        ),
        ValueAstConversionsTestVector::new(
            Value::NonFungibleId { value: NonFungibleId::Bytes(vec![0x10, 0xa2, 0x31, 0x01]) },
            r#"NonFungibleId(Bytes("10a23101"))"#,
        ),

        ValueAstConversionsTestVector::new(
            Value::NonFungibleAddress {
                address: radix_engine_toolkit_core::model::NonFungibleAddress {
                    resource_address: NetworkAwareResourceAddress {
                            network_id: 0xf2,
                            address: scrypto::prelude::ResourceAddress::Normal([0; 26]),
                        },
                    non_fungible_id: NonFungibleId::U32(1144418947)
                }
            },
            r#"NonFungibleAddress("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety", 1144418947u32)"#,
        ),
        ValueAstConversionsTestVector::new(
            Value::NonFungibleAddress {
                address: radix_engine_toolkit_core::model::NonFungibleAddress {
                    resource_address: NetworkAwareResourceAddress {
                            network_id: 0xf2,
                            address: scrypto::prelude::ResourceAddress::Normal([0; 26]),
                        },
                    non_fungible_id: NonFungibleId::U64(114441894733333)
                }
            },
            r#"NonFungibleAddress("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety", 114441894733333u64)"#,
        ),
        ValueAstConversionsTestVector::new(
            Value::NonFungibleAddress {
                address: radix_engine_toolkit_core::model::NonFungibleAddress {
                    resource_address: NetworkAwareResourceAddress {
                            network_id: 0xf2,
                            address: scrypto::prelude::ResourceAddress::Normal([0; 26]),
                        },
                    non_fungible_id: NonFungibleId::UUID(11444189334733333)
                }
            },
            r#"NonFungibleAddress("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety", 11444189334733333u128)"#,
        ),
        ValueAstConversionsTestVector::new(
            Value::NonFungibleAddress {
                address: radix_engine_toolkit_core::model::NonFungibleAddress {
                    resource_address: NetworkAwareResourceAddress {
                            network_id: 0xf2,
                            address: scrypto::prelude::ResourceAddress::Normal([0; 26]),
                        },
                    non_fungible_id: NonFungibleId::String("hello_world".into())
                }
            },
            r#"NonFungibleAddress("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety", "hello_world")"#,
        ),
        ValueAstConversionsTestVector::new(
            Value::NonFungibleAddress {
                address: radix_engine_toolkit_core::model::NonFungibleAddress {
                    resource_address: NetworkAwareResourceAddress {
                            network_id: 0xf2,
                            address: scrypto::prelude::ResourceAddress::Normal([0; 26]),
                        },
                    non_fungible_id: NonFungibleId::Bytes(vec![0x10, 0xa2, 0x31, 0x01])
                }
            },
            r#"NonFungibleAddress("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety", Bytes("10a23101"))"#,
        ),

        // =================
        // Other Misc Types
        // =================
        ValueAstConversionsTestVector::new(
            Value::Blob { hash: Blob("2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824".parse().unwrap()) },
            r#"Blob("2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824")"#
        ),
        ValueAstConversionsTestVector::new(
            Value::Expression { value: ManifestExpression::EntireAuthZone },
            r#"Expression("ENTIRE_AUTH_ZONE")"#
        ),
        ValueAstConversionsTestVector::new(
            Value::Expression { value: ManifestExpression::EntireWorktop },
            r#"Expression("ENTIRE_WORKTOP")"#
        ),
        ValueAstConversionsTestVector::new(
            Value::Bytes { value: vec![0x12, 0x19, 0x12, 0x20, 0x8] },
            r#"Bytes("1219122008")"#
        ),
    ];

    pub static ref VALUE_VALIDATION_TEST_VECTORS: Vec<ValueValidationTestVector> = vec![
        // ====================================
        // Address Network Mismatch Validation
        // ====================================
        ValueValidationTestVector::new(
            Value::ResourceAddress {
                address: NetworkAwareResourceAddress {
                    network_id: 0x10,
                    address: scrypto::prelude::ResourceAddress::Normal([0; 26])
                }
            },
            Err(Error::NetworkMismatchError { expected: 0xf2, found: 0x10 })
        ),
        ValueValidationTestVector::new(
            Value::ComponentAddress {
                address: NetworkAwareComponentAddress {
                    network_id: 0x10,
                    address: scrypto::prelude::ComponentAddress::Normal([0; 26])
                }
            },
            Err(Error::NetworkMismatchError { expected: 0xf2, found: 0x10 })
        ),
        ValueValidationTestVector::new(
            Value::SystemAddress {
                address: NetworkAwareSystemAddress {
                    network_id: 0x10,
                    address: scrypto::prelude::SystemAddress::EpochManager([0; 26])
                }
            },
            Err(Error::NetworkMismatchError { expected: 0xf2, found: 0x10 })
        ),
        ValueValidationTestVector::new(
            Value::PackageAddress {
                address: NetworkAwarePackageAddress {
                    network_id: 0x10,
                    address: scrypto::prelude::PackageAddress::Normal([0; 26])
                }
            },
            Err(Error::NetworkMismatchError { expected: 0xf2, found: 0x10 })
        ),

        ValueValidationTestVector::new(
            Value::Array { element_type: ValueKind::Array, elements: vec![
                Value::Array { element_type: ValueKind::Tuple, elements: vec![
                    Value::Tuple { elements: vec![
                        Value::Array { element_type: ValueKind::Tuple, elements: vec![
                            Value::Tuple { elements: vec![
                                Value::PackageAddress {
                                    address: NetworkAwarePackageAddress {
                                        network_id: 0x10,
                                        address: scrypto::prelude::PackageAddress::Normal([0; 26])
                                    }
                                }
                            ] }
                        ] }
                    ] }
                ] }
            ] },
            Err(Error::NetworkMismatchError { expected: 0xf2, found: 0x10 })
        ),
        // ============================
        // Collection Types Validation
        // ============================
        ValueValidationTestVector::new(
            Value::Array {
                element_type: ValueKind::Decimal,
                elements: vec![
                    Value::Decimal { value: dec!("20") },
                    Value::Decimal { value: dec!("100") },
                    Value::Decimal {
                        value: dec!("192.31"),
                    },
                ],
            },
            Ok(())
        ),
        ValueValidationTestVector::new(
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
            Err(Error::InvalidType {
                expected_types: vec![ValueKind::Decimal],
                actual_type: ValueKind::PreciseDecimal
            })
        ),
        ValueValidationTestVector::new(
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
            },
            Err(Error::InvalidType {
                expected_types: vec![ValueKind::Decimal],
                actual_type: ValueKind::PreciseDecimal
            })
        ),
    ];

    pub static ref VALUE_KIND_TEST_VECTORS: Vec<ValueKindTestVector> = vec![
        // ================
        // Primitive Types
        // ================

        // Unit and Boolean
        ValueKindTestVector::new(
            Value::Unit,
            ValueKind::Unit,
            AstValueKind::Unit
        ),
        ValueKindTestVector::new(
            Value::Bool { value: true },
            ValueKind::Bool,
            AstValueKind::Bool,
        ),
        ValueKindTestVector::new(
            Value::Bool { value: false },
            ValueKind::Bool,
            AstValueKind::Bool,
        ),
        // Unsigned Integers
        ValueKindTestVector::new(
            Value::U8 { value: 19 },
            ValueKind::U8,
            AstValueKind::U8,
        ),
        ValueKindTestVector::new(
            Value::U16 { value: 19 },
            ValueKind::U16,
            AstValueKind::U16,
        ),
        ValueKindTestVector::new(
            Value::U32 { value: 19 },
            ValueKind::U32,
            AstValueKind::U32,
        ),
        ValueKindTestVector::new(
            Value::U64 { value: 19 },
            ValueKind::U64,
            AstValueKind::U64,
        ),
        ValueKindTestVector::new(
            Value::U128 { value: 19 },
            ValueKind::U128,
            AstValueKind::U128,
        ),
        // Signed Integers
        ValueKindTestVector::new(
            Value::I8 { value: 19 },
            ValueKind::I8,
            AstValueKind::I8,
        ),
        ValueKindTestVector::new(
            Value::I16 { value: 19 },
            ValueKind::I16,
            AstValueKind::I16,
        ),
        ValueKindTestVector::new(
            Value::I32 { value: 19 },
            ValueKind::I32,
            AstValueKind::I32,
        ),
        ValueKindTestVector::new(
            Value::I64 { value: 19 },
            ValueKind::I64,
            AstValueKind::I64,
        ),
        ValueKindTestVector::new(
            Value::I128 { value: 19 },
            ValueKind::I128,
            AstValueKind::I128,
        ),
        // String
        ValueKindTestVector::new(
            Value::String {
                value: "P2P Cash System".into(),
            },
            ValueKind::String,
            AstValueKind::String,
        ),
        // Enums and Enum Aliases (Option & Result)
        ValueKindTestVector::new(
            Value::Enum {
                variant: "Create".into(),
                fields: Some(vec![Value::String {
                    value: "Component".into(),
                }]),
            },
            ValueKind::Enum,
            AstValueKind::Enum,
        ),
        ValueKindTestVector::new(
            Value::Option {
                value: Box::new(Some(Value::String {
                    value: "Component".into(),
                })),
            },
            ValueKind::Option,
            AstValueKind::Enum,
        ),
        ValueKindTestVector::new(
            Value::Option {
                value: Box::new(None),
            },
            ValueKind::Option,
            AstValueKind::Enum,
        ),
        ValueKindTestVector::new(
            Value::Result {
                value: Box::new(Ok(Value::String {
                    value: "Component".into(),
                })),
            },
            ValueKind::Result,
            AstValueKind::Enum,
        ),
        ValueKindTestVector::new(
            Value::Result {
                value: Box::new(Err(Value::String {
                    value: "Component".into(),
                })),
            },
            ValueKind::Result,
            AstValueKind::Enum,
        ),
        // =================
        // Collection Types
        // =================
        ValueKindTestVector::new(
            Value::Array {
                element_type: ValueKind::String,
                elements: vec![Value::String {
                    value: "World, Hello!".into(),
                }],
            },
            ValueKind::Array,
            AstValueKind::Array,
        ),
        ValueKindTestVector::new(
            Value::Tuple {
                elements: vec![Value::I64 { value: 19 }, Value::I8 { value: 19 }],
            },
            ValueKind::Tuple,
            AstValueKind::Tuple,
        ),
        // ============================
        // Decimal And Precise Decimal
        // ============================
        ValueKindTestVector::new(
            Value::Decimal {
                value: "1923319912.102221313".parse().unwrap(),
            },
            ValueKind::Decimal,
            AstValueKind::Decimal,
        ),
        ValueKindTestVector::new(
            Value::PreciseDecimal {
                value: "1923319912.102221313".parse().unwrap(),
            },
            ValueKind::PreciseDecimal,
            AstValueKind::PreciseDecimal,
        ),
        // ==============
        // Address Types
        // ==============
        ValueKindTestVector::new(
            Value::ComponentAddress {
                address: NetworkAwareComponentAddress {
                    network_id: 0xf2,
                    address: scrypto::prelude::ComponentAddress::Normal([0; 26]),
                },
            },
            ValueKind::ComponentAddress,
            AstValueKind::ComponentAddress,
        ),
        ValueKindTestVector::new(
            Value::ComponentAddress {
                address: NetworkAwareComponentAddress {
                    network_id: 0xf2,
                    address: scrypto::prelude::ComponentAddress::Account([0; 26]),
                },
            },
            ValueKind::ComponentAddress,
            AstValueKind::ComponentAddress,
        ),
        ValueKindTestVector::new(
            Value::ResourceAddress {
                address: NetworkAwareResourceAddress {
                    network_id: 0xf2,
                    address: scrypto::prelude::ResourceAddress::Normal([0; 26]),
                },
            },
            ValueKind::ResourceAddress,
            AstValueKind::ResourceAddress,
        ),
        ValueKindTestVector::new(
            Value::PackageAddress {
                address: NetworkAwarePackageAddress {
                    network_id: 0xf2,
                    address: scrypto::prelude::PackageAddress::Normal([0; 26]),
                },
            },
            ValueKind::PackageAddress,
            AstValueKind::PackageAddress,
        ),
        ValueKindTestVector::new(
            Value::SystemAddress {
                address: NetworkAwareSystemAddress {
                    network_id: 0xf2,
                    address: scrypto::prelude::SystemAddress::EpochManager([0; 26]),
                },
            },
            ValueKind::SystemAddress,
            AstValueKind::SystemAddress,
        ),
        ValueKindTestVector::new(
            Value::SystemAddress {
                address: NetworkAwareSystemAddress {
                    network_id: 0xf2,
                    address: scrypto::prelude::SystemAddress::Clock([0; 26]),
                },
            },
            ValueKind::SystemAddress,
            AstValueKind::SystemAddress,
        ),
        // ==============
        // Cryptographic
        // ==============
        ValueKindTestVector::new(
            Value::Hash { value: "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824".parse().unwrap() },
            ValueKind::Hash,
            AstValueKind::Hash,
        ),
        ValueKindTestVector::new(
            Value::EcdsaSecp256k1PublicKey { public_key: "0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798".parse().unwrap() },
            ValueKind::EcdsaSecp256k1PublicKey,
            AstValueKind::EcdsaSecp256k1PublicKey,
        ),
        ValueKindTestVector::new(
            Value::EddsaEd25519PublicKey { public_key: "4cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29".parse().unwrap() },
            ValueKind::EddsaEd25519PublicKey,
            AstValueKind::EddsaEd25519PublicKey,
        ),
        ValueKindTestVector::new(
            Value::EcdsaSecp256k1Signature { signature: "0079224ea514206706298d8d620f660828f7987068d6d02757e6f3cbbf4a51ab133395db69db1bc9b2726dd99e34efc252d8258dcb003ebaba42be349f50f7765e".parse().unwrap() },
            ValueKind::EcdsaSecp256k1Signature,
            AstValueKind::EcdsaSecp256k1Signature,
        ),
        ValueKindTestVector::new(
            Value::EddsaEd25519Signature { signature: "ce993adc51111309a041faa65cbcf1154d21ed0ecdc2d54070bc90b9deb744aa8605b3f686fa178fba21070b4a4678e54eee3486a881e0e328251cd37966de09".parse().unwrap() },
            ValueKind::EddsaEd25519Signature,
            AstValueKind::EddsaEd25519Signature,
        ),

        // ===================
        // Buckets and Proofs
        // ===================
        ValueKindTestVector::new(
            Value::Bucket { identifier: BucketId(Identifier::String("xrd_bucket".into())) },
            ValueKind::Bucket,
            AstValueKind::Bucket
        ),
        ValueKindTestVector::new(
            Value::Bucket { identifier: BucketId(Identifier::U32(28)) },
            ValueKind::Bucket,
            AstValueKind::Bucket
        ),
        ValueKindTestVector::new(
            Value::Proof { identifier: ProofId(Identifier::String("xrd_proof".into())) },
            ValueKind::Proof,
            AstValueKind::Proof
        ),
        ValueKindTestVector::new(
            Value::Proof { identifier: ProofId(Identifier::U32(28)) },
            ValueKind::Proof,
            AstValueKind::Proof
        ),

        // ==========================
        // Non Fungible Id & Address
        // ==========================

        ValueKindTestVector::new(
            Value::NonFungibleId { value: NonFungibleId::U32(1144418947) },
            ValueKind::NonFungibleId,
            AstValueKind::NonFungibleId,
        ),
        ValueKindTestVector::new(
            Value::NonFungibleId { value: NonFungibleId::U64(114441894733333) },
            ValueKind::NonFungibleId,
            AstValueKind::NonFungibleId,
        ),
        ValueKindTestVector::new(
            Value::NonFungibleId { value: NonFungibleId::UUID(11444189334733333) },
            ValueKind::NonFungibleId,
            AstValueKind::NonFungibleId,
        ),
        ValueKindTestVector::new(
            Value::NonFungibleId { value: NonFungibleId::String("hello_world".into()) },
            ValueKind::NonFungibleId,
            AstValueKind::NonFungibleId,
        ),
        ValueKindTestVector::new(
            Value::NonFungibleId { value: NonFungibleId::Bytes(vec![0x10, 0xa2, 0x31, 0x01]) },
            ValueKind::NonFungibleId,
            AstValueKind::NonFungibleId,
        ),

        ValueKindTestVector::new(
            Value::NonFungibleAddress {
                address: radix_engine_toolkit_core::model::NonFungibleAddress {
                    resource_address: NetworkAwareResourceAddress {
                            network_id: 0xf2,
                            address: scrypto::prelude::ResourceAddress::Normal([0; 26]),
                        },
                    non_fungible_id: NonFungibleId::U32(1144418947)
                }
            },
            ValueKind::NonFungibleAddress,
            AstValueKind::NonFungibleAddress,
        ),
        ValueKindTestVector::new(
            Value::NonFungibleAddress {
                address: radix_engine_toolkit_core::model::NonFungibleAddress {
                    resource_address: NetworkAwareResourceAddress {
                            network_id: 0xf2,
                            address: scrypto::prelude::ResourceAddress::Normal([0; 26]),
                        },
                    non_fungible_id: NonFungibleId::U64(114441894733333)
                }
            },
            ValueKind::NonFungibleAddress,
            AstValueKind::NonFungibleAddress,
        ),
        ValueKindTestVector::new(
            Value::NonFungibleAddress {
                address: radix_engine_toolkit_core::model::NonFungibleAddress {
                    resource_address: NetworkAwareResourceAddress {
                            network_id: 0xf2,
                            address: scrypto::prelude::ResourceAddress::Normal([0; 26]),
                        },
                    non_fungible_id: NonFungibleId::UUID(11444189334733333)
                }
            },
            ValueKind::NonFungibleAddress,
            AstValueKind::NonFungibleAddress,
        ),
        ValueKindTestVector::new(
            Value::NonFungibleAddress {
                address: radix_engine_toolkit_core::model::NonFungibleAddress {
                    resource_address: NetworkAwareResourceAddress {
                            network_id: 0xf2,
                            address: scrypto::prelude::ResourceAddress::Normal([0; 26]),
                        },
                    non_fungible_id: NonFungibleId::String("hello_world".into())
                }
            },
            ValueKind::NonFungibleAddress,
            AstValueKind::NonFungibleAddress,
        ),
        ValueKindTestVector::new(
            Value::NonFungibleAddress {
                address: radix_engine_toolkit_core::model::NonFungibleAddress {
                    resource_address: NetworkAwareResourceAddress {
                            network_id: 0xf2,
                            address: scrypto::prelude::ResourceAddress::Normal([0; 26]),
                        },
                    non_fungible_id: NonFungibleId::Bytes(vec![0x10, 0xa2, 0x31, 0x01])
                }
            },
            ValueKind::NonFungibleAddress,
            AstValueKind::NonFungibleAddress,
        ),

        // =================
        // Other Misc Types
        // =================
        ValueKindTestVector::new(
            Value::Blob { hash: Blob("2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824".parse().unwrap()) },
            ValueKind::Blob,
            AstValueKind::Blob
        ),
        ValueKindTestVector::new(
            Value::Expression { value: ManifestExpression::EntireAuthZone },
            ValueKind::Expression,
            AstValueKind::Expression
        ),
        ValueKindTestVector::new(
            Value::Expression { value: ManifestExpression::EntireWorktop },
            ValueKind::Expression,
            AstValueKind::Expression
        ),
        ValueKindTestVector::new(
            Value::Bytes { value: vec![0x12, 0x19, 0x12, 0x20, 0x8] },
            ValueKind::Bytes,
            AstValueKind::Bytes
        ),
    ];
}
