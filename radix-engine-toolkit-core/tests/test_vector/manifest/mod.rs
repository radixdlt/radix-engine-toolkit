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

// All of the manifests in this module are obtained from the original Scrypto repo. The hopes of the
// test vectors here is to make sure that Scrypto can understand can also be understood by the Radix
// Engine Toolkit. Source of manifests:
// https://github.com/radixdlt/radixdlt-scrypto/tree/release/betanet-v1/transaction/examples

use std::collections::HashSet;

use radix_engine_toolkit_core::model::{
    Identifier, Instruction::*, ManifestInstructions, NonFungibleAddress, TransactionManifest, 
    Value, ValueKind,
};
use scrypto::prelude::{Expression, NonFungibleId};

pub struct TransactionManifestTestVector {
    pub original_manifest: String,
    pub manifest: TransactionManifest,
    pub expected_json_representation: TransactionManifest,
}

impl TransactionManifestTestVector {
    pub fn new<S: AsRef<str>>(
        manifest: S,
        blobs: &[&[u8]],
        expected_json_representation: TransactionManifest,
    ) -> Self {
        Self {
            original_manifest: manifest.as_ref().to_string(),
            manifest: TransactionManifest {
                instructions: radix_engine_toolkit_core::model::ManifestInstructions::String(
                    manifest.as_ref().to_string(),
                ),
                blobs: blobs.iter().map(|x| (*x).to_owned()).collect(),
            },
            expected_json_representation,
        }
    }
}

macro_rules! parse {
    ($expression: expr) => {
        $expression.parse().unwrap()
    };
}

lazy_static::lazy_static! {
    pub static ref TRANSACTION_MANIFEST_TEST_VECTORS: Vec<TransactionManifestTestVector> = vec![
        TransactionManifestTestVector::new(
            include_str!("./complex.rtm"),
            &[include_bytes!("./complex.code"), include_bytes!("./complex.abi")],
            TransactionManifest {
                instructions: ManifestInstructions::JSON(vec![
                    // Withdraw XRD from account
                    CallMethod {
                        component_address: parse!("account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064"),
                        method_name: parse!("withdraw_by_amount"),
                        arguments: Some(vec![
                            Value::Decimal { value: parse!("5.0") },
                            Value::ResourceAddress { address: parse!("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag") },
                        ])
                    },

                    // Buy GUM with XRD
                    TakeFromWorktopByAmount {
                        amount: parse!("2.0"),
                        resource_address: parse!("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"),
                        into_bucket: Identifier::String(parse!("xrd")).into()
                    },
                    CallMethod {
                        component_address: parse!("component_sim1q2f9vmyrmeladvz0ejfttcztqv3genlsgpu9vue83mcs835hum"),
                        method_name: parse!("buy_gumball"),
                        arguments: Some(vec![
                            Value::Bucket { identifier: Identifier::String(parse!("xrd")).into() }
                        ])
                    },
                    AssertWorktopContainsByAmount {
                        amount: parse!("3.0"),
                        resource_address: parse!("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag")
                    },
                    AssertWorktopContains {
                        resource_address: parse!("resource_sim1qzhdk7tq68u8msj38r6v6yqa5myc64ejx3ud20zlh9gseqtux6")
                    },

                    // Create a proof from bucket, clone it and drop both
                    TakeFromWorktop {
                        resource_address: parse!("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"),
                        into_bucket: Identifier::String(parse!("some_xrd")).into()
                    },
                    CreateProofFromBucket {
                        bucket: Identifier::String(parse!("some_xrd")).into(),
                        into_proof: Identifier::String(parse!("proof1")).into()
                    },
                    CloneProof {
                        proof: Identifier::String(parse!("proof1")).into(),
                        into_proof: Identifier::String(parse!("proof2")).into()
                    },
                    DropProof {
                        proof: Identifier::String(parse!("proof1")).into(),
                    },
                    DropProof {
                        proof: Identifier::String(parse!("proof2")).into(),
                    },

                    // Create a proof from account and drop it
                    CallMethod {
                        component_address: parse!("account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064"),
                        method_name: parse!("create_proof_by_amount"),
                        arguments: Some(vec![
                            Value::Decimal { value: parse!("5.0") },
                            Value::ResourceAddress { address: parse!("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag") },
                        ])
                    },
                    PopFromAuthZone {
                        into_proof: Identifier::String(parse!("proof3")).into()
                    },
                    DropProof {
                        proof: Identifier::String(parse!("proof3")).into(),
                    },

                    // Return a bucket to worktop
                    ReturnToWorktop {
                        bucket: Identifier::String(parse!("some_xrd")).into()
                    },
                    TakeFromWorktopByIds {
                        ids: HashSet::from([
                            NonFungibleId::Bytes(hex::decode("031b84c5567b126440995d3ed5aaba0565d71e1834604819ff9c17f5e9d5dd078f").unwrap())
                        ]),
                        resource_address: parse!("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"),
                        into_bucket: Identifier::String(parse!("nfts")).into()
                    },

                    // Create a new fungible resource
                    CreateResource {
                        resource_type: Value::Enum {
                            variant: parse!("Fungible"),
                            fields: Some(vec![Value::U8 { value: 0 }])
                        },
                        metadata: Value::Array {
                            element_type: ValueKind::Tuple,
                            elements: vec![]
                        },
                        access_rules: Value::Array {
                            element_type: ValueKind::Tuple,
                            elements: vec![]
                        },
                        mint_params: Value::Option {
                            value: Box::new(Some(Value::Enum {
                                variant: parse!("Fungible"),
                                fields: Some(vec![Value::Decimal { value: parse!("1.0") }])
                            }))
                        }
                    },

                    // Cancel all buckets and move resources to account
                    CallMethod {
                        component_address: parse!("account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064"),
                        method_name: parse!("deposit_batch"),
                        arguments: Some(vec![Value::Expression { value: Expression::entire_worktop() }])
                    },

                    // Drop all proofs
                    DropAllProofs,

                    // Complicated method that takes all of the number types
                    CallMethod {
                        component_address: parse!("component_sim1q2f9vmyrmeladvz0ejfttcztqv3genlsgpu9vue83mcs835hum"),
                        method_name: parse!("complicated_method"),
                        arguments: Some(vec![
                            Value::Decimal { value: parse!("1") },
                            Value::PreciseDecimal { value: parse!("2") }
                        ])
                    },

                    // Publish package
                    PublishPackageWithOwner {
                        code: parse!("36dae540b7889956f1f1d8d46ba23e5e44bf5723aef2a8e6b698686c02583618"),
                        abi: parse!("15e8699a6d63a96f66f6feeb609549be2688b96b02119f260ae6dfd012d16a5d"),
                        owner_badge: NonFungibleAddress {
                            resource_address: parse!("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"),
                            non_fungible_id: NonFungibleId::Bytes(hex::decode("031b84c5567b126440995d3ed5aaba0565d71e1834604819ff9c17f5e9d5dd078f").unwrap())
                        }
                    }

                ]),
                blobs: vec![
                    include_bytes!("./complex.code").to_vec(),
                    include_bytes!("./complex.abi").to_vec()
                ]
            }
        ),
        TransactionManifestTestVector::new(
            include_str!("./call_method.rtm"),
            &[],
            TransactionManifest {
                instructions: ManifestInstructions::JSON(vec![
                    // Invoke scrypto method (both global and local)
                    CallMethod {
                        component_address: parse!("component_sim1qgvyxt5rrjhwctw7krgmgkrhv82zuamcqkq75tkkrwgs00m736"),
                        method_name: parse!("free_xrd"),
                        arguments: None
                    },

                    // Invoke native method (ref only)
                    TakeFromWorktop {
                        resource_address: parse!("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"),
                        into_bucket: Identifier::String(parse!("xrd")).into()
                    },
                    CreateProofFromAuthZone {
                        resource_address: parse!("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"),
                        into_proof: Identifier::String(parse!("proof")).into()
                    },
                ]),
                blobs: Vec::new()
            }
        ),
        TransactionManifestTestVector::new(
            include_str!("./call_function.rtm"),
            &[],
            TransactionManifest {
                instructions: ManifestInstructions::JSON(vec![
                    // Invoke scrypto function
                    CallFunction {
                        package_address: parse!("package_sim1qy4hrp8a9apxldp5cazvxgwdj80cxad4u8cpkaqqnhlsa3lfpe"),
                        blueprint_name: parse!("Blueprint"),
                        function_name: parse!("function"),
                        arguments: None,
                    },
                ]),
                blobs: vec![]
            }
        ),
        TransactionManifestTestVector::new(
            include_str!("./any_value.rtm"),
            &[include_bytes!("./any_value.blob")],
            TransactionManifest {
                instructions: ManifestInstructions::JSON(vec![
                    TakeFromWorktop {
                        resource_address: parse!("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"),
                        into_bucket: Identifier::String(parse!("temp1")).into()
                    },

                    CreateProofFromAuthZone {
                        resource_address: parse!("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"),
                        into_proof: Identifier::String(parse!("temp2")).into()
                    },

                    CallMethod {
                        component_address: parse!("component_sim1q2f9vmyrmeladvz0ejfttcztqv3genlsgpu9vue83mcs835hum"),
                        method_name: parse!("with_aliases"),
                        arguments: Some(vec![
                            // Test aliases and their non-aliased versions
                            Value::Option { value: Box::new(None) },
                            Value::Option { value: Box::new(None) },

                            Value::Option { value: Box::new(Some(Value::String { value: parse!("hello") })) },
                            Value::Option { value: Box::new(Some(Value::String { value: parse!("hello") })) },

                            Value::Result { value: Box::new(Ok(Value::String { value: parse!("test") })) },
                            Value::Result { value: Box::new(Ok(Value::String { value: parse!("test") })) },

                            Value::Result { value: Box::new(Err(Value::String { value: parse!("test123") })) },
                            Value::Result { value: Box::new(Err(Value::String { value: parse!("test123") })) },

                            Value::Bytes { value: vec![5u8, 10u8, 255u8] },
                            Value::Array { element_type: ValueKind::U8, elements: [5u8, 10u8, 255u8].iter().map(|x| Value::U8 { value: *x }).collect() },
                        ])
                    },

                    CallMethod {
                        component_address: parse!("component_sim1q2f9vmyrmeladvz0ejfttcztqv3genlsgpu9vue83mcs835hum"),
                        method_name: parse!("with_all_types"),
                        arguments: Some(vec![
                            // Global address types
                            Value::PackageAddress { address: parse!("package_sim1qyqzcexvnyg60z7lnlwauh66nhzg3m8tch2j8wc0e70qkydk8r") },
                            Value::ComponentAddress { address: parse!("account_sim1q0u9gxewjxj8nhxuaschth2mgencma2hpkgwz30s9wlslthace") },
                            Value::ResourceAddress { address: parse!("resource_sim1qq8cays25704xdyap2vhgmshkkfyr023uxdtk59ddd4qs8cr5v") },
                            Value::SystemAddress { address: parse!("system_sim1qne8qu4seyvzfgd94p3z8rjcdl3v0nfhv84judpum2lq7x4635") },

                            // RE nodes types
                            Value::Component { identifier: parse!("000000000000000000000000000000000000000000000000000000000000000005000000") },
                            Value::KeyValueStore { identifier: parse!("000000000000000000000000000000000000000000000000000000000000000005000000") },
                            Value::Bucket { identifier: Identifier::String(parse!("temp1")).into() },
                            Value::Proof { identifier: Identifier::String(parse!("temp2")).into() },
                            Value::Vault { identifier: parse!("000000000000000000000000000000000000000000000000000000000000000005000000") },

                            // Other interpreted types
                            Value::Expression { value: Expression::new("ALL_WORKTOP_RESOURCES") },
                            Value::Blob { hash: parse!("36dae540b7889956f1f1d8d46ba23e5e44bf5723aef2a8e6b698686c02583618") },
                            Value::NonFungibleAddress { address: NonFungibleAddress {
                                resource_address: parse!("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"),
                                non_fungible_id: NonFungibleId::String(parse!("value"))
                            }},
                            Value::NonFungibleAddress { address: NonFungibleAddress {
                                resource_address: parse!("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"),
                                non_fungible_id: NonFungibleId::U32(123)
                            }},
                            Value::NonFungibleAddress { address: NonFungibleAddress {
                                resource_address: parse!("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"),
                                non_fungible_id: NonFungibleId::U64(456)
                            }},
                            Value::NonFungibleAddress { address: NonFungibleAddress {
                                resource_address: parse!("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"),
                                non_fungible_id: NonFungibleId::Bytes(hex::decode("031b84c5567b126440995d3ed5aaba0565d71e1834604819ff9c17f5e9d5dd078f").unwrap())
                            }},
                            Value::NonFungibleAddress { address: NonFungibleAddress {
                                resource_address: parse!("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"),
                                non_fungible_id: NonFungibleId::UUID(1234567890)
                            }},

                            // Uninterpreted
                            Value::Hash {
                                value: parse!("2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824")
                            },
                            Value::EcdsaSecp256k1PublicKey {
                                public_key: parse!("0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798")
                            },
                            Value::EcdsaSecp256k1Signature {
                                signature: parse!("0079224ea514206706298d8d620f660828f7987068d6d02757e6f3cbbf4a51ab133395db69db1bc9b2726dd99e34efc252d8258dcb003ebaba42be349f50f7765e")
                            },
                            Value::EddsaEd25519PublicKey {
                                public_key: parse!("4cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29")
                            },
                            Value::EddsaEd25519Signature {
                                signature: parse!("ce993adc51111309a041faa65cbcf1154d21ed0ecdc2d54070bc90b9deb744aa8605b3f686fa178fba21070b4a4678e54eee3486a881e0e328251cd37966de09")
                            },
                            Value::Decimal { value: parse!("1.2") },
                            Value::PreciseDecimal { value: parse!("1.2") },
                            Value::NonFungibleId { value: NonFungibleId::Bytes(hex::decode("031b84c5567b126440995d3ed5aaba0565d71e1834604819ff9c17f5e9d5dd078f").unwrap()) },
                            Value::NonFungibleId { value: NonFungibleId::U32(12) },
                            Value::NonFungibleId { value: NonFungibleId::U64(12345u64) },
                            Value::NonFungibleId { value: NonFungibleId::UUID(1234567890) },
                            Value::NonFungibleId { value: NonFungibleId::String(parse!("SomeId"))  },
                        ])
                    }
                ]),
                blobs: vec![
                    include_bytes!("./any_value.blob").to_vec()
                ]
            }
        ),
        TransactionManifestTestVector::new(
            include_str!("./non_fungible_ids_canonical.rtm"),
            &[],
            TransactionManifest {
                instructions: ManifestInstructions::JSON(vec![
                    TakeFromWorktopByIds {
                        ids: HashSet::from([NonFungibleId::U32(12)]),
                        resource_address: parse!("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"),
                        into_bucket: Identifier::String(parse!("bucket1")).into()
                    },
                    TakeFromWorktopByIds {
                        ids: HashSet::from([NonFungibleId::U64(19)]),
                        resource_address: parse!("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"),
                        into_bucket: Identifier::String(parse!("bucket2")).into()
                    },
                    TakeFromWorktopByIds {
                        ids: HashSet::from([NonFungibleId::String(parse!("HelloWorld"))]),
                        resource_address: parse!("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"),
                        into_bucket: Identifier::String(parse!("bucket3")).into()
                    },
                    TakeFromWorktopByIds {
                        ids: HashSet::from([NonFungibleId::Bytes(hex::decode("121922ff03").unwrap())]),
                        resource_address: parse!("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"),
                        into_bucket: Identifier::String(parse!("bucket4")).into()
                    },
                    TakeFromWorktopByIds {
                        ids: HashSet::from([NonFungibleId::UUID(1922931322)]),
                        resource_address: parse!("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"),
                        into_bucket: Identifier::String(parse!("bucket5")).into()
                    },
                ]),
                blobs: vec![]
            }
        ),
    ];
}
