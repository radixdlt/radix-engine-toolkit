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

use native_transaction::manifest::lexer::tokenize;
use radix_engine_toolkit::{model::*, EnumDiscriminator};
use scrypto::prelude::{NonFungibleLocalId, *};

pub struct InstructionRepresentationTestVector {
    pub instruction: Instruction,
    pub json_representation: String,
    pub manifest_representation: String,
}

impl InstructionRepresentationTestVector {
    pub fn new<S: AsRef<str>, T: AsRef<str>>(
        instruction: Instruction,
        json_representation: S,
        manifest_representation: T,
    ) -> Self {
        Self {
            instruction,
            json_representation: json_representation.as_ref().into(),
            manifest_representation: manifest_representation.as_ref().into(),
        }
    }

    pub fn manifest_representation_as_ast_instruction(
        &self,
    ) -> native_transaction::manifest::ast::Instruction {
        native_transaction::manifest::parser::Parser::new(
            tokenize(&self.manifest_representation).expect("Failed to tokenize trusted value"),
        )
        .parse_instruction()
        .expect("Failed to parse trusted value to ast value")
    }
}

lazy_static::lazy_static! {
    pub static ref INSTRUCTION_CONVERSION_TEST_VECTORS: Vec<InstructionRepresentationTestVector> = vec![
        InstructionRepresentationTestVector::new(
            Instruction::CallFunction {
                package_address: Value::PackageAddress {
                    address: NetworkAwarePackageAddress {
                        network_id: 0xf2,
                        address: PackageAddress::Normal([0; 26]),
                    },
                },
                blueprint_name: Value::String {
                    value: "HelloWorld".into(),
                },
                function_name: Value::String {
                    value: "world_hello".into(),
                },
                arguments: Some(vec![Value::Decimal {
                    value: "129333".parse().unwrap(),
                }]),
            },
            r#"
            {
                "instruction": "CALL_FUNCTION",
                "package_address": {
                    "type": "PackageAddress",
                    "address": "package_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqxrmwtq"
                },
                "blueprint_name": {
                    "type": "String",
                    "value": "HelloWorld"
                },
                "function_name": {
                    "type": "String",
                    "value": "world_hello"
                },
                "arguments": [
                    {
                        "type": "Decimal",
                        "value": "129333"
                    }
                ]
            }
            "#,
            r#"CALL_FUNCTION PackageAddress("package_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqxrmwtq") "HelloWorld" "world_hello" Decimal("129333");"#,
        ),
        InstructionRepresentationTestVector::new(
            Instruction::CallMethod {
                component_address: Value::ComponentAddress {
                    address: NetworkAwareComponentAddress {
                        network_id: 0xf2,
                        address: scrypto::prelude::ComponentAddress::Normal([0; 26]),
                    },
                },
                method_name: Value::String {
                    value: "remove_user".into(),
                },
                arguments: Some(vec![Value::Decimal {
                    value: "12".parse().unwrap(),
                }]),
            },
            r#"
            {
                "instruction": "CALL_METHOD",
                "component_address": {
                    "type": "ComponentAddress",
                    "address": "component_sim1qgqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq8ecz5v"
                },
                "method_name": {
                    "type": "String",
                    "value": "remove_user"
                },
                "arguments": [
                    {
                        "type": "Decimal",
                        "value": "12"
                    }
                ]
            }
            "#,
            r#"
            CALL_METHOD 
                ComponentAddress("component_sim1qgqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq8ecz5v") 
                "remove_user" 
                Decimal("12");
            "#,
        ),
        InstructionRepresentationTestVector::new(
            Instruction::TakeFromWorktop {
                resource_address: Value::ResourceAddress {
                    address: NetworkAwareResourceAddress {
                        network_id: 0xf2,
                        address: RADIX_TOKEN,
                    },
                },
                into_bucket: Value::Bucket {
                    identifier: BucketId(TransientIdentifier::U32 { value: 1 }),
                },
            },
            r#"
            {
                "instruction": "TAKE_FROM_WORKTOP",
                "resource_address": {
                    "type": "ResourceAddress",
                    "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety"
                },
                "into_bucket": {
                    "type": "Bucket",
                    "identifier": {
                        "type": "U32",
                        "value": "1"
                    }
                }
            }
            "#,
            r#"
            TAKE_FROM_WORKTOP
                ResourceAddress("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety")
                Bucket(1u32);
            "#,
        ),
        InstructionRepresentationTestVector::new(
            Instruction::TakeFromWorktopByAmount {
                resource_address: Value::ResourceAddress {
                    address: NetworkAwareResourceAddress {
                        network_id: 0xf2,
                        address: RADIX_TOKEN,
                    },
                },
                amount: Value::Decimal {
                    value: "1".parse().unwrap(),
                },
                into_bucket: Value::Bucket {
                    identifier: BucketId(TransientIdentifier::U32 { value: 1 }),
                },
            },
            r#"
            {
                "instruction": "TAKE_FROM_WORKTOP_BY_AMOUNT",
                "resource_address": {
                    "type": "ResourceAddress", 
                    "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety"
                },
                "amount": {
                    "type": "Decimal",
                    "value": "1"
                },
                "into_bucket": {
                    "type": "Bucket",
                    "identifier": {
                        "type": "U32",
                        "value": "1"
                    }
                }
            }
            "#,
            r#"
            TAKE_FROM_WORKTOP_BY_AMOUNT
                Decimal("1")
                ResourceAddress("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety")
                Bucket(1u32);
            "#,
        ),
        InstructionRepresentationTestVector::new(
            Instruction::TakeFromWorktopByIds {
                resource_address: Value::ResourceAddress {
                    address: NetworkAwareResourceAddress {
                        network_id: 0xf2,
                        address: RADIX_TOKEN,
                    },
                },
                ids: vec![Value::NonFungibleLocalId {
                    value: scrypto::prelude::NonFungibleLocalId::Integer(1),
                }],
                into_bucket: Value::Bucket {
                    identifier: BucketId(TransientIdentifier::U32 { value: 1 }),
                },
            },
            r#"
            {
                "instruction": "TAKE_FROM_WORKTOP_BY_IDS",
                "resource_address": {
                    "type": "ResourceAddress", 
                    "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety"
                },
                "ids": [
                    {
                        "type": "NonFungibleLocalId",
                        "value": {
                            "type": "Integer",
                            "value": "1"
                        }
                    }
                ],
                "into_bucket": {
                    "type": "Bucket",
                    "identifier": {
                        "type": "U32",
                        "value": "1"
                    }
                }
            }
            "#,
            r##"
            TAKE_FROM_WORKTOP_BY_IDS
                Array<NonFungibleLocalId>(NonFungibleLocalId("#1#"))
                ResourceAddress("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety")
                Bucket(1u32);
            "##,
        ),
        InstructionRepresentationTestVector::new(
            Instruction::ReturnToWorktop {
                bucket: Value::Bucket {
                    identifier: BucketId(TransientIdentifier::U32 { value: 1 }),
                },
            },
            r#"
            {
                "instruction": "RETURN_TO_WORKTOP",
                "bucket": {
                    "type": "Bucket",
                    "identifier": {
                        "type": "U32",
                        "value": "1"
                    }
                }
            }
            "#,
            r##"
            RETURN_TO_WORKTOP
                Bucket(1u32);
            "##,
        ),
        InstructionRepresentationTestVector::new(
            Instruction::AssertWorktopContains {
                resource_address: Value::ResourceAddress {
                    address: NetworkAwareResourceAddress {
                        network_id: 0xf2,
                        address: RADIX_TOKEN,
                    },
                },
            },
            r#"
            {
                "instruction": "ASSERT_WORKTOP_CONTAINS",
                "resource_address": {
                    "type": "ResourceAddress",
                    "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety"
                }
            }
            "#,
            r#"
            ASSERT_WORKTOP_CONTAINS
                ResourceAddress("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety");
            "#,
        ),
        InstructionRepresentationTestVector::new(
            Instruction::AssertWorktopContainsByAmount {
                resource_address: Value::ResourceAddress {
                    address: NetworkAwareResourceAddress {
                        network_id: 0xf2,
                        address: RADIX_TOKEN,
                    },
                },
                amount: Value::Decimal {
                    value: "1".parse().unwrap(),
                },
            },
            r#"
            {
                "instruction": "ASSERT_WORKTOP_CONTAINS_BY_AMOUNT",
                "resource_address": {
                    "type": "ResourceAddress", 
                    "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety"
                },
                "amount": {
                    "type": "Decimal",
                    "value": "1"
                }
            }
            "#,
            r#"
            ASSERT_WORKTOP_CONTAINS_BY_AMOUNT
                Decimal("1")
                ResourceAddress("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety");
            "#,
        ),
        InstructionRepresentationTestVector::new(
            Instruction::AssertWorktopContainsByIds {
                resource_address: Value::ResourceAddress {
                    address: NetworkAwareResourceAddress {
                        network_id: 0xf2,
                        address: RADIX_TOKEN,
                    },
                },
                ids: vec![Value::NonFungibleLocalId {
                    value: scrypto::prelude::NonFungibleLocalId::Integer(1),
                }],
            },
            r#"
            {
                "instruction": "ASSERT_WORKTOP_CONTAINS_BY_IDS",
                "resource_address": {
                    "type": "ResourceAddress", 
                    "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety"
                },
                "ids": [
                    {
                        "type": "NonFungibleLocalId",
                        "value": {
                            "type": "Integer",
                            "value": "1"
                        }
                    }
                ]
            }
            "#,
            r##"
            ASSERT_WORKTOP_CONTAINS_BY_IDS
                Array<NonFungibleLocalId>(NonFungibleLocalId("#1#"))
                ResourceAddress("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety");
            "##,
        ),
        InstructionRepresentationTestVector::new(
            Instruction::PopFromAuthZone {
                into_proof: Value::Proof {
                    identifier: ProofId(TransientIdentifier::U32 { value: 1 }),
                },
            },
            r#"
            {
                "instruction": "POP_FROM_AUTH_ZONE",
                "into_proof": {
                    "type": "Proof",
                    "identifier": {
                        "type": "U32",
                        "value": "1"
                    }
                }
            }
            "#,
            r##"
            POP_FROM_AUTH_ZONE 
                Proof(1u32);
            "##,
        ),
        InstructionRepresentationTestVector::new(
            Instruction::PushToAuthZone {
                proof: Value::Proof {
                    identifier: ProofId(TransientIdentifier::U32 { value: 1 }),
                },
            },
            r#"
            {
                "instruction": "PUSH_TO_AUTH_ZONE",
                "proof": {
                    "type": "Proof",
                    "identifier": {
                        "type": "U32",
                        "value": "1"
                    }
                }
            }
            "#,
            r##"
            PUSH_TO_AUTH_ZONE 
                Proof(1u32);
            "##,
        ),
        InstructionRepresentationTestVector::new(
            Instruction::ClearAuthZone,
            r#"
            {
                "instruction": "CLEAR_AUTH_ZONE"
            }
            "#,
            r##"
            CLEAR_AUTH_ZONE;
            "##,
        ),
        InstructionRepresentationTestVector::new(
            Instruction::CreateProofFromAuthZone {
                resource_address: Value::ResourceAddress {
                    address: NetworkAwareResourceAddress {
                        network_id: 0xf2,
                        address: RADIX_TOKEN,
                    },
                },
                into_proof: Value::Proof {
                    identifier: ProofId(TransientIdentifier::U32 { value: 1 }),
                },
            },
            r#"
            {
                "instruction": "CREATE_PROOF_FROM_AUTH_ZONE",
                "resource_address": {
                    "type": "ResourceAddress",
                    "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety"
                },
                "into_proof": {
                    "type": "Proof",
                    "identifier": {
                        "type": "U32",
                        "value": "1"
                    }
                }
            }
            "#,
            r#"
            CREATE_PROOF_FROM_AUTH_ZONE
                ResourceAddress("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety")
                Proof(1u32);
            "#,
        ),
        InstructionRepresentationTestVector::new(
            Instruction::CreateProofFromAuthZoneByAmount {
                resource_address: Value::ResourceAddress {
                    address: NetworkAwareResourceAddress {
                        network_id: 0xf2,
                        address: RADIX_TOKEN,
                    },
                },
                amount: Value::Decimal {
                    value: "1".parse().unwrap(),
                },
                into_proof: Value::Proof {
                    identifier: ProofId(TransientIdentifier::U32 { value: 1 }),
                },
            },
            r#"
            {
                "instruction": "CREATE_PROOF_FROM_AUTH_ZONE_BY_AMOUNT",
                "resource_address": {
                    "type": "ResourceAddress", 
                    "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety"
                },
                "amount": {
                    "type": "Decimal",
                    "value": "1"
                },
                "into_proof": {
                    "type": "Proof",
                    "identifier": {
                        "type": "U32",
                        "value": "1"
                    }
                }
            }
            "#,
            r#"
            CREATE_PROOF_FROM_AUTH_ZONE_BY_AMOUNT
                Decimal("1")
                ResourceAddress("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety")
                Proof(1u32);
            "#,
        ),
        InstructionRepresentationTestVector::new(
            Instruction::CreateProofFromAuthZoneByIds {
                resource_address: Value::ResourceAddress {
                    address: NetworkAwareResourceAddress {
                        network_id: 0xf2,
                        address: RADIX_TOKEN,
                    },
                },
                ids: vec![Value::NonFungibleLocalId {
                    value: scrypto::prelude::NonFungibleLocalId::Integer(1),
                }],
                into_proof: Value::Proof {
                    identifier: ProofId(TransientIdentifier::U32 { value: 1 }),
                },
            },
            r#"
            {
                "instruction": "CREATE_PROOF_FROM_AUTH_ZONE_BY_IDS",
                "resource_address": {
                    "type": "ResourceAddress", 
                    "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety"
                },
                "ids": [
                    {
                        "type": "NonFungibleLocalId",
                        "value": {
                            "type": "Integer",
                            "value": "1"
                        }
                    }
                ],
                "into_proof": {
                    "type": "Proof",
                    "identifier": {
                        "type": "U32",
                        "value": "1"
                    }
                }
            }
            "#,
            r##"
            CREATE_PROOF_FROM_AUTH_ZONE_BY_IDS
                Array<NonFungibleLocalId>(NonFungibleLocalId("#1#"))
                ResourceAddress("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety")
                Proof(1u32);
            "##,
        ),
        InstructionRepresentationTestVector::new(
            Instruction::CloneProof {
                proof: Value::Proof {
                    identifier: ProofId(TransientIdentifier::U32 { value: 1 }),
                },
                into_proof: Value::Proof {
                    identifier: ProofId(TransientIdentifier::U32 { value: 2 }),
                },
            },
            r#"
            {
                "instruction": "CLONE_PROOF",
                "proof": {
                    "type": "Proof",
                    "identifier": {
                        "type": "U32",
                        "value": "1"
                    }
                },
                "into_proof": {
                    "type": "Proof",
                    "identifier": {
                        "type": "U32",
                        "value": "2"
                    }
                }
            }
            "#,
            r##"
            CLONE_PROOF
                Proof(1u32)
                Proof(2u32);
            "##,
        ),
        InstructionRepresentationTestVector::new(
            Instruction::DropProof {
                proof: Value::Proof {
                    identifier: ProofId(TransientIdentifier::U32 { value: 1 }),
                },
            },
            r#"
            {
                "instruction": "DROP_PROOF",
                "proof": {
                    "type": "Proof",
                    "identifier": {
                        "type": "U32",
                        "value": "1"
                    }
                }
            }
            "#,
            r##"
            DROP_PROOF
                Proof(1u32);
            "##,
        ),
        InstructionRepresentationTestVector::new(
            Instruction::DropAllProofs,
            r#"
            {
                "instruction": "DROP_ALL_PROOFS"
            }
            "#,
            r##"
            DROP_ALL_PROOFS;
            "##,
        ),
        InstructionRepresentationTestVector::new(
            Instruction::PublishPackage {
                code: Value::Blob {
                    hash: Hash::from_str(
                        "36dae540b7889956f1f1d8d46ba23e5e44bf5723aef2a8e6b698686c02583618",
                    )
                    .map(ManifestBlobRef)
                    .unwrap(),
                },
                abi: Value::Blob {
                    hash: Hash::from_str(
                        "15e8699a6d63a96f66f6feeb609549be2688b96b02119f260ae6dfd012d16a5d",
                    )
                    .map(ManifestBlobRef)
                    .unwrap(),
                },
                royalty_config: Value::Map {
                    key_value_kind: ValueKind::String,
                    value_value_kind: ValueKind::Tuple,
                    entries: Vec::new(),
                },
                metadata: Value::Map {
                    key_value_kind: ValueKind::String,
                    value_value_kind: ValueKind::String,
                    entries: Vec::new(),
                },
                access_rules: Value::Enum {
                    variant: EnumDiscriminator::U8 { discriminator: 0 },
                    fields: None,
                },
            },
            r#"
            {
                "instruction": "PUBLISH_PACKAGE",
                "code": {
                    "type": "Blob",
                    "hash": "36dae540b7889956f1f1d8d46ba23e5e44bf5723aef2a8e6b698686c02583618"
                },
                "abi": {
                    "type": "Blob",
                    "hash": "15e8699a6d63a96f66f6feeb609549be2688b96b02119f260ae6dfd012d16a5d"
                },
                "royalty_config": {
                    "type": "Map",
                    "key_value_kind": "String",
                    "value_value_kind": "Tuple",
                    "entries": []
                },
                "metadata": {
                    "type": "Map",
                    "key_value_kind": "String",
                    "value_value_kind": "String",
                    "entries": []
                },
                "access_rules": {
                    "type": "Enum",
                    "variant": {
                        "type": "U8",
                        "discriminator": "0"
                    }
                }
            }
            "#,
            r##"
            PUBLISH_PACKAGE
                Blob("36dae540b7889956f1f1d8d46ba23e5e44bf5723aef2a8e6b698686c02583618")
                Blob("15e8699a6d63a96f66f6feeb609549be2688b96b02119f260ae6dfd012d16a5d")
                Map<String, Tuple>()
                Map<String, String>()
                Enum(0u8);
            "##,
        ),
        InstructionRepresentationTestVector::new(
            Instruction::PublishPackageWithOwner {
                code: Value::Blob {
                    hash: Hash::from_str(
                        "36dae540b7889956f1f1d8d46ba23e5e44bf5723aef2a8e6b698686c02583618",
                    )
                    .map(ManifestBlobRef)
                    .unwrap(),
                },
                abi: Value::Blob {
                    hash: Hash::from_str(
                        "15e8699a6d63a96f66f6feeb609549be2688b96b02119f260ae6dfd012d16a5d",
                    )
                    .map(ManifestBlobRef)
                    .unwrap(),
                },
                owner_badge: Value::NonFungibleGlobalId {
                    address: radix_engine_toolkit::NonFungibleGlobalId {
                        resource_address: NetworkAwareResourceAddress {
                            network_id: 0xf2,
                            address: RADIX_TOKEN,
                        },
                        non_fungible_local_id: NonFungibleLocalId::Integer(1),
                    },
                },
            },
            r#"
            {
                "instruction": "PUBLISH_PACKAGE_WITH_OWNER",
                "code": {
                    "type": "Blob",
                    "hash": "36dae540b7889956f1f1d8d46ba23e5e44bf5723aef2a8e6b698686c02583618"
                },
                "abi": {
                    "type": "Blob",
                    "hash": "15e8699a6d63a96f66f6feeb609549be2688b96b02119f260ae6dfd012d16a5d"
                },
                "owner_badge": {
                    "type": "NonFungibleGlobalId",
                    "resource_address": {
                        "type": "ResourceAddress",
                        "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety"
                    },
                    "non_fungible_local_id": {
                        "type": "NonFungibleLocalId",
                        "value": {
                            "type": "Integer",
                            "value": "1"
                        }
                    }
                }
            }
            "#,
            r##"
            PUBLISH_PACKAGE_WITH_OWNER
                Blob("36dae540b7889956f1f1d8d46ba23e5e44bf5723aef2a8e6b698686c02583618")
                Blob("15e8699a6d63a96f66f6feeb609549be2688b96b02119f260ae6dfd012d16a5d")
                NonFungibleGlobalId("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety:#1#");
            "##,
        ),
        InstructionRepresentationTestVector::new(
            Instruction::BurnResource {
                bucket: Value::Bucket {
                    identifier: BucketId(TransientIdentifier::U32 { value: 1 }),
                },
            },
            r#"
            {
                "instruction": "BURN_RESOURCE",
                "bucket": {
                    "type": "Bucket",
                    "identifier": {
                        "type": "U32",
                        "value": "1"
                    }
                }
            }
            "#,
            r##"
            BURN_RESOURCE
                Bucket(1u32);
            "##,
        ),
        InstructionRepresentationTestVector::new(
            Instruction::RecallResource {
                vault_id: Value::Bytes {
                    value: hex::decode(
                        "776e134adba9d55474c4fe9b04a5f39dc8164b9a9c22dae66a34e1417162c327912cc492",
                    )
                    .unwrap(),
                },
                amount: Value::Decimal {
                    value: "1".parse().unwrap(),
                },
            },
            r#"
            {
                "instruction": "RECALL_RESOURCE",
                "vault_id": {
                    "type": "Bytes",
                    "value": "776e134adba9d55474c4fe9b04a5f39dc8164b9a9c22dae66a34e1417162c327912cc492"
                },
                "amount": {
                    "type": "Decimal",
                    "value": "1"
                }
            }
            "#,
            r##"
            RECALL_RESOURCE
                Bytes("776e134adba9d55474c4fe9b04a5f39dc8164b9a9c22dae66a34e1417162c327912cc492")
                Decimal("1");
            "##,
        ),
        InstructionRepresentationTestVector::new(
            Instruction::SetMetadata {
                entity_address: Value::ComponentAddress {
                    address: NetworkAwareComponentAddress {
                        network_id: 0xf2,
                        address: FAUCET_COMPONENT,
                    },
                },
                key: Value::String {
                    value: "name".into(),
                },
                value: Value::String {
                    value: "deadbeef".into(),
                },
            },
            r#"
            {
                "instruction": "SET_METADATA",
                "entity_address": {
                    "type": "ComponentAddress",
                    "address": "component_sim1qgehpqdhhr62xh76wh6gppnyn88a0uau68epljprvj3sxknsqr"
                },
                "key": {
                    "type": "String",
                    "value": "name"
                },
                "value": {
                    "type": "String",
                    "value": "deadbeef"
                }
            }
            "#,
            r##"
            SET_METADATA
                ComponentAddress("component_sim1qgehpqdhhr62xh76wh6gppnyn88a0uau68epljprvj3sxknsqr")
                "name"
                "deadbeef";
            "##,
        ),
        InstructionRepresentationTestVector::new(
            Instruction::SetMetadata {
                entity_address: Value::PackageAddress {
                    address: NetworkAwarePackageAddress {
                        network_id: 0xf2,
                        address: FAUCET_PACKAGE,
                    },
                },
                key: Value::String {
                    value: "name".into(),
                },
                value: Value::String {
                    value: "deadbeef".into(),
                },
            },
            r#"
            {
                "instruction": "SET_METADATA",
                "entity_address": {
                    "type": "PackageAddress",
                    "address": "package_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqxrmwtq"
                },
                "key": {
                    "type": "String",
                    "value": "name"
                },
                "value": {
                    "type": "String",
                    "value": "deadbeef"
                }
            }
            "#,
            r##"
            SET_METADATA
                PackageAddress("package_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqxrmwtq")
                "name"
                "deadbeef";
            "##,
        ),
        InstructionRepresentationTestVector::new(
            Instruction::SetMetadata {
                entity_address: Value::ResourceAddress {
                    address: NetworkAwareResourceAddress {
                        network_id: 0xf2,
                        address: RADIX_TOKEN,
                    },
                },
                key: Value::String {
                    value: "name".into(),
                },
                value: Value::String {
                    value: "deadbeef".into(),
                },
            },
            r#"
            {
                "instruction": "SET_METADATA",
                "entity_address": {
                    "type": "ResourceAddress",
                    "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety"
                },
                "key": {
                    "type": "String",
                    "value": "name"
                },
                "value": {
                    "type": "String",
                    "value": "deadbeef"
                }
            }
            "#,
            r##"
            SET_METADATA
                ResourceAddress("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety")
                "name"
                "deadbeef";
            "##,
        ),
        InstructionRepresentationTestVector::new(
            Instruction::SetPackageRoyaltyConfig {
                package_address: Value::PackageAddress {
                    address: NetworkAwarePackageAddress {
                        network_id: 0xf2,
                        address: FAUCET_PACKAGE,
                    },
                },
                royalty_config: Value::Map {
                    key_value_kind: ValueKind::String,
                    value_value_kind: ValueKind::Tuple,
                    entries: Vec::new(),
                },
            },
            r#"
            {
                "instruction": "SET_PACKAGE_ROYALTY_CONFIG",
                "package_address": {
                    "type": "PackageAddress",
                    "address": "package_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqxrmwtq"
                },
                "royalty_config": {
                    "type": "Map",
                    "key_value_kind": "String",
                    "value_value_kind": "Tuple",
                    "entries": []
                }
            }
            "#,
            r##"
            SET_PACKAGE_ROYALTY_CONFIG
                PackageAddress("package_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqxrmwtq")
                Map<String, Tuple>();
            "##,
        ),
        InstructionRepresentationTestVector::new(
            Instruction::SetComponentRoyaltyConfig {
                component_address: Value::ComponentAddress {
                    address: NetworkAwareComponentAddress {
                        network_id: 0xf2,
                        address: FAUCET_COMPONENT,
                    },
                },
                royalty_config: Value::Map {
                    key_value_kind: ValueKind::String,
                    value_value_kind: ValueKind::Tuple,
                    entries: Vec::new(),
                },
            },
            r#"
            {
                "instruction": "SET_COMPONENT_ROYALTY_CONFIG",
                "component_address": {
                    "type": "ComponentAddress",
                    "address": "component_sim1qgehpqdhhr62xh76wh6gppnyn88a0uau68epljprvj3sxknsqr"
                },
                "royalty_config": {
                    "type": "Map",
                    "key_value_kind": "String",
                    "value_value_kind": "Tuple",
                    "entries": []
                }
            }
            "#,
            r##"
            SET_COMPONENT_ROYALTY_CONFIG
                ComponentAddress("component_sim1qgehpqdhhr62xh76wh6gppnyn88a0uau68epljprvj3sxknsqr")
                Map<String, Tuple>();
            "##,
        ),
        InstructionRepresentationTestVector::new(
            Instruction::ClaimPackageRoyalty {
                package_address: Value::PackageAddress {
                    address: NetworkAwarePackageAddress {
                        network_id: 0xf2,
                        address: FAUCET_PACKAGE,
                    },
                },
            },
            r#"
            {
                "instruction": "CLAIM_PACKAGE_ROYALTY",
                "package_address": {
                    "type": "PackageAddress",
                    "address": "package_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqxrmwtq"
                }
            }
            "#,
            r##"
            CLAIM_PACKAGE_ROYALTY
                PackageAddress("package_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqxrmwtq");
            "##,
        ),
        InstructionRepresentationTestVector::new(
            Instruction::ClaimComponentRoyalty {
                component_address: Value::ComponentAddress {
                    address: NetworkAwareComponentAddress {
                        network_id: 0xf2,
                        address: FAUCET_COMPONENT,
                    },
                },
            },
            r#"
            {
                "instruction": "CLAIM_COMPONENT_ROYALTY",
                "component_address": {
                    "type": "ComponentAddress",
                    "address": "component_sim1qgehpqdhhr62xh76wh6gppnyn88a0uau68epljprvj3sxknsqr"
                }
            }
            "#,
            r##"
            CLAIM_COMPONENT_ROYALTY
                ComponentAddress("component_sim1qgehpqdhhr62xh76wh6gppnyn88a0uau68epljprvj3sxknsqr");
            "##,
        ),
        InstructionRepresentationTestVector::new(
            Instruction::SetMethodAccessRule {
                entity_address: Value::ComponentAddress {
                    address: NetworkAwareComponentAddress {
                        network_id: 0xf2,
                        address: FAUCET_COMPONENT,
                    },
                },
                index: Value::U8 { value: 0 },
                key: Value::String {
                    value: "get_token".into(),
                },
                rule: Value::Enum {
                    variant: EnumDiscriminator::U8 { discriminator: 0 },
                    fields: None,
                },
            },
            r#"
            {
                "instruction": "SET_METHOD_ACCESS_RULE",
                "entity_address": {
                    "type": "ComponentAddress",
                    "address": "component_sim1qgehpqdhhr62xh76wh6gppnyn88a0uau68epljprvj3sxknsqr"
                },
                "index": {
                    "type": "U8",
                    "value": "0"
                },
                "key": {
                    "type": "String",
                    "value": "get_token"
                },
                "rule": {
                    "type": "Enum",
                    "variant": {
                        "type": "U8",
                        "discriminator": "0"
                    }
                }
            }
            "#,
            r##"
            SET_METHOD_ACCESS_RULE
                ComponentAddress("component_sim1qgehpqdhhr62xh76wh6gppnyn88a0uau68epljprvj3sxknsqr")
                0u8
                "get_token"
                Enum(0u8);
            "##,
        ),
        InstructionRepresentationTestVector::new(
            Instruction::MintFungible {
                resource_address: Value::ResourceAddress { address: NetworkAwareResourceAddress {
                    network_id: 0xf2,
                    address: RADIX_TOKEN,
                } },
                amount: Value::Decimal { value: "1".parse().unwrap() }
            },
            r#"
            {
                "instruction": "MINT_FUNGIBLE",
                "resource_address": {
                    "type": "ResourceAddress",
                    "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety"
                },
                "amount": {
                    "type": "Decimal",
                    "value": "1"
                }
            }
            "#,
            r##"
            MINT_FUNGIBLE
                ResourceAddress("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety")
                Decimal("1");
            "##,
        ),
        InstructionRepresentationTestVector::new(
            Instruction::MintNonFungible {
                resource_address: Value::ResourceAddress { address: NetworkAwareResourceAddress {
                    network_id: 0xf2,
                    address: RADIX_TOKEN,
                } },
                entries: Value::Map {
                    key_value_kind: ValueKind::NonFungibleLocalId,
                    value_value_kind: ValueKind::Tuple,
                    entries: Vec::new()
                }
            },
            r#"
            {
                "instruction": "MINT_NON_FUNGIBLE",
                "resource_address": {
                    "type": "ResourceAddress",
                    "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety"
                },
                "entries": {
                    "type": "Map",
                    "key_value_kind": "NonFungibleLocalId",
                    "value_value_kind": "Tuple",
                    "entries": []
                }
            }
            "#,
            r##"
            MINT_NON_FUNGIBLE
                ResourceAddress("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety")
                Map<NonFungibleLocalId, Tuple>();
            "##,
        ),
        InstructionRepresentationTestVector::new(
            Instruction::CreateFungibleResource {
                divisibility: Value::U8 { value: 18 },
                metadata: Value::Map {
                    key_value_kind: ValueKind::String,
                    value_value_kind: ValueKind::String,
                    entries: Vec::new()
                },
                access_rules: Value::Map {
                    key_value_kind: ValueKind::String,
                    value_value_kind: ValueKind::Enum,
                    entries: Vec::new()
                },
                initial_supply: Value::None
            },
            r#"
            {
                "instruction": "CREATE_FUNGIBLE_RESOURCE",
                "divisibility": {
                    "type": "U8",
                    "value": "18"
                },
                "metadata": {
                    "type": "Map",
                    "key_value_kind": "String",
                    "value_value_kind": "String",
                    "entries": []
                },
                "access_rules": {
                    "type": "Map",
                    "key_value_kind": "String",
                    "value_value_kind": "Enum",
                    "entries": []
                },
                "initial_supply": {
                    "type": "None"
                }
            }
            "#,
            r##"
            CREATE_FUNGIBLE_RESOURCE
                18u8
                Map<String, String>()
                Map<String, Enum>()
                None;
            "##,
        ),
        InstructionRepresentationTestVector::new(
            Instruction::CreateFungibleResourceWithOwner {
                divisibility: Value::U8 { value: 18 },
                metadata: Value::Map {
                    key_value_kind: ValueKind::String,
                    value_value_kind: ValueKind::String,
                    entries: Vec::new()
                },
                owner_badge: Value::NonFungibleGlobalId {
                    address: radix_engine_toolkit::NonFungibleGlobalId {
                        resource_address: NetworkAwareResourceAddress {
                            network_id: 0xf2,
                            address: RADIX_TOKEN,
                        },
                        non_fungible_local_id: NonFungibleLocalId::Integer(1),
                    },
                },
                initial_supply: Value::None
            },
            r#"
            {
                "instruction": "CREATE_FUNGIBLE_RESOURCE_WITH_OWNER",
                "divisibility": {
                    "type": "U8",
                    "value": "18"
                },
                "metadata": {
                    "type": "Map",
                    "key_value_kind": "String",
                    "value_value_kind": "String",
                    "entries": []
                },
                "owner_badge": {
                    "type": "NonFungibleGlobalId",
                    "resource_address": {
                        "type": "ResourceAddress",
                        "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety"
                    },
                    "non_fungible_local_id": {
                        "type": "NonFungibleLocalId",
                        "value": {
                            "type": "Integer",
                            "value": "1"
                        }
                    }
                },
                "initial_supply": {
                    "type": "None"
                }
            }
            "#,
            r##"
            CREATE_FUNGIBLE_RESOURCE_WITH_OWNER
                18u8
                Map<String, String>()
                NonFungibleGlobalId("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety:#1#")
                None;
            "##,
        ),
        InstructionRepresentationTestVector::new(
            Instruction::CreateNonFungibleResource {
                id_type: Value::Enum { variant: EnumDiscriminator::U8 { discriminator: 0 }, fields: None },
                metadata: Value::Map {
                    key_value_kind: ValueKind::String,
                    value_value_kind: ValueKind::String,
                    entries: Vec::new()
                },
                access_rules: Value::Map {
                    key_value_kind: ValueKind::String,
                    value_value_kind: ValueKind::Enum,
                    entries: Vec::new()
                },
                initial_supply: Value::None
            },
            r#"
            {
                "instruction": "CREATE_NON_FUNGIBLE_RESOURCE",
                "id_type": {
                    "type": "Enum",
                    "variant": {
                        "type": "U8",
                        "discriminator": "0"
                    }
                },
                "metadata": {
                    "type": "Map",
                    "key_value_kind": "String",
                    "value_value_kind": "String",
                    "entries": []
                },
                "access_rules": {
                    "type": "Map",
                    "key_value_kind": "String",
                    "value_value_kind": "Enum",
                    "entries": []
                },
                "initial_supply": {
                    "type": "None"
                }
            }
            "#,
            r##"
            CREATE_NON_FUNGIBLE_RESOURCE
                Enum(0u8)
                Map<String, String>()
                Map<String, Enum>()
                None;
            "##,
        ),
        InstructionRepresentationTestVector::new(
            Instruction::CreateNonFungibleResourceWithOwner {
                id_type: Value::Enum { variant: EnumDiscriminator::U8 { discriminator: 0 }, fields: None },
                metadata: Value::Map {
                    key_value_kind: ValueKind::String,
                    value_value_kind: ValueKind::String,
                    entries: Vec::new()
                },
                owner_badge: Value::NonFungibleGlobalId {
                    address: radix_engine_toolkit::NonFungibleGlobalId {
                        resource_address: NetworkAwareResourceAddress {
                            network_id: 0xf2,
                            address: RADIX_TOKEN,
                        },
                        non_fungible_local_id: NonFungibleLocalId::Integer(1),
                    },
                },
                initial_supply: Value::None
            },
            r#"
            {
                "instruction": "CREATE_NON_FUNGIBLE_RESOURCE_WITH_OWNER",
                "id_type": {
                    "type": "Enum",
                    "variant": {
                        "type": "U8",
                        "discriminator": "0"
                    }
                },
                "metadata": {
                    "type": "Map",
                    "key_value_kind": "String",
                    "value_value_kind": "String",
                    "entries": []
                },
                "owner_badge": {
                    "type": "NonFungibleGlobalId",
                    "resource_address": {
                        "type": "ResourceAddress",
                        "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety"
                    },
                    "non_fungible_local_id": {
                        "type": "NonFungibleLocalId",
                        "value": {
                            "type": "Integer",
                            "value": "1"
                        }
                    }
                },
                "initial_supply": {
                    "type": "None"
                }
            }
            "#,
            r##"
            CREATE_NON_FUNGIBLE_RESOURCE_WITH_OWNER
                Enum(0u8)
                Map<String, String>()
                NonFungibleGlobalId("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety:#1#")
                None;
            "##,
        ),
        InstructionRepresentationTestVector::new(
            Instruction::CreateAccessController {
                controlled_asset: Value::Bucket { identifier: BucketId(TransientIdentifier::U32 { value: 1 }) },
                primary_role: Value::Enum { variant: EnumDiscriminator::U8 { discriminator: 0 }, fields: None },
                recovery_role: Value::Enum { variant: EnumDiscriminator::U8 { discriminator: 0 }, fields: None },
                confirmation_role: Value::Enum { variant: EnumDiscriminator::U8 { discriminator: 0 }, fields: None },
                timed_recovery_delay_in_minutes: Value::Some { value: Box::new(Value::U8 { value: 1 }) }
            },
            r#"
            {
                "instruction": "CREATE_ACCESS_CONTROLLER",
                "controlled_asset": {
                    "type": "Bucket",
                    "identifier": {
                        "type": "U32",
                        "value": "1"
                    }
                },
                "primary_role": {
                    "type": "Enum",
                    "variant": {
                        "type": "U8",
                        "discriminator": "0"
                    }
                },
                "recovery_role": {
                    "type": "Enum",
                    "variant": {
                        "type": "U8",
                        "discriminator": "0"
                    }
                },
                "confirmation_role": {
                    "type": "Enum",
                    "variant": {
                        "type": "U8",
                        "discriminator": "0"
                    }
                },
                "timed_recovery_delay_in_minutes": {
                    "type": "Some",
                    "value": {
                        "type": "U8",
                        "value": "1"
                    }
                }
            }
            "#,
            r##"
            CREATE_ACCESS_CONTROLLER
                Bucket(1u32)
                Enum(0u8)
                Enum(0u8)
                Enum(0u8)
                Some(1u8);
            "##,
        ),
        InstructionRepresentationTestVector::new(
            Instruction::CreateIdentity {
                access_rule: Value::Enum { variant: EnumDiscriminator::U8 { discriminator: 0 }, fields: None }
            },
            r#"
            {
                "instruction": "CREATE_IDENTITY",
                "access_rule": {
                    "type": "Enum",
                    "variant": {
                        "type": "U8",
                        "discriminator": "0"
                    }
                }
            }
            "#,
            r##"
            CREATE_IDENTITY
                Enum(0u8);
            "##,
        ),
        InstructionRepresentationTestVector::new(
            Instruction::AssertAccessRule {
                access_rule: Value::Enum { variant: EnumDiscriminator::U8 { discriminator: 0 }, fields: None }
            },
            r#"
            {
                "instruction": "ASSERT_ACCESS_RULE",
                "access_rule": {
                    "type": "Enum",
                    "variant": {
                        "type": "U8",
                        "discriminator": "0"
                    }
                }
            }
            "#,
            r##"
            ASSERT_ACCESS_RULE
                Enum(0u8);
            "##,
        ),
        InstructionRepresentationTestVector::new(
            Instruction::CreateValidator {
                key: Value::EcdsaSecp256k1PublicKey {
                    public_key: "0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798".parse().unwrap()
                },
            },
            r#"
            {
                "instruction": "CREATE_VALIDATOR",
                "key": {
                    "type": "EcdsaSecp256k1PublicKey",
                    "public_key": "0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798"
                }
            }
            "#,
            r##"
            CREATE_VALIDATOR
                EcdsaSecp256k1PublicKey("0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798");
            "##,
        ),
    ];
}
