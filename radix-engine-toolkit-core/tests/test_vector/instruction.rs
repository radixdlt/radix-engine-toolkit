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

use radix_engine_toolkit_core::model::*;
use radix_transaction::manifest::lexer::tokenize;
use scrypto::prelude::*;

pub struct InstructionSerializationTestVector {
    pub instruction: Instruction,
    pub json_representation: String,
}

impl InstructionSerializationTestVector {
    pub fn new<S: AsRef<str>>(instruction: Instruction, json_representation: S) -> Self {
        let json_representation: &str = json_representation.as_ref();
        let json_representation: String = json_representation.to_string();
        Self {
            instruction,
            json_representation,
        }
    }
}

pub struct InstructionAstConversionsTestVector {
    pub instruction: Instruction,
    pub manifest_representation: String,
}

impl InstructionAstConversionsTestVector {
    pub fn new<S: AsRef<str>>(instruction: Instruction, manifest_representation: S) -> Self {
        let manifest_representation: &str = manifest_representation.as_ref();
        let manifest_representation: String = manifest_representation.into();
        Self {
            manifest_representation,
            instruction,
        }
    }

    pub fn manifest_representation_as_ast_instruction(
        &self,
    ) -> radix_transaction::manifest::ast::Instruction {
        radix_transaction::manifest::parser::Parser::new(
            tokenize(&self.manifest_representation).expect("Failed to tokenize trusted value"),
        )
        .parse_instruction()
        .expect("Failed to parse trusted value to ast value")
    }
}

lazy_static::lazy_static! {
    pub static ref INSTRUCTION_JSON_CONVERSION_TEST_VECTORS: Vec<InstructionSerializationTestVector> = vec![
        InstructionSerializationTestVector::new(
            Instruction::CallFunction {
                package_address: NetworkAwarePackageAddress {
                    network_id: 0xf2,
                    address: PackageAddress::Normal([0; 26]),
                },
                blueprint_name: "HelloWorld".into(),
                function_name: "world_hello".into(),
                arguments: Some(vec![Value::Decimal {
                    value: "129333".parse().unwrap()
                }])
            },
            r#"{
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
                }"#
        ),
        InstructionSerializationTestVector::new(
            Instruction::CallFunction {
                package_address: NetworkAwarePackageAddress {
                    network_id: 0xf2,
                    address: PackageAddress::Normal([0; 26]),
                },
                blueprint_name: "HelloWorld".into(),
                function_name: "world_hello".into(),
                arguments: None
            },
            r#"{
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
                    }
                }"#
        ),
        InstructionSerializationTestVector::new(
            Instruction::CallMethod {
                component_address: NetworkAwareComponentAddress {
                    network_id: 0xf2,
                    address: scrypto::prelude::ComponentAddress::Normal([0; 26]),
                },
                method_name: "remove_user".into(),
                arguments: Some(vec![Value::NonFungibleId {
                    value: scrypto::prelude::NonFungibleId::Number(18)
                }])
            },
            r#"{
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
                            "type": "NonFungibleId",
                            "variant": "Number",
                            "value": "18"
                        }
                    ]
                }"#
        ),
        InstructionSerializationTestVector::new(
            Instruction::TakeFromWorktop {
                resource_address: NetworkAwareResourceAddress {
                    network_id: 0xf2,
                    address: ResourceAddress::Normal([0; 26]),
                },
                into_bucket: BucketId(Identifier::String("bucket".into()))
            },
            r#"{
                "instruction": "TAKE_FROM_WORKTOP",
                "resource_address": {
                    "type": "ResourceAddress",
                    "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety"
                },
                "into_bucket": {
                    "type": "Bucket",
                    "identifier": "bucket"
                }
            }"#
        ),
        InstructionSerializationTestVector::new(
            Instruction::TakeFromWorktop {
                resource_address: NetworkAwareResourceAddress {
                    network_id: 0xf2,
                    address: ResourceAddress::Normal([0; 26]),
                },
                into_bucket: BucketId(Identifier::U32(29))
            },
            r#"{
                "instruction": "TAKE_FROM_WORKTOP",
                "resource_address": {
                    "type": "ResourceAddress",
                    "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety"
                },
                "into_bucket": {
                    "type": "Bucket",
                    "identifier": 29
                }
            }"#
        ),
        InstructionSerializationTestVector::new(
            Instruction::TakeFromWorktopByAmount {
                amount: dec!("123"),
                resource_address: NetworkAwareResourceAddress {
                    network_id: 0xf2,
                    address: ResourceAddress::Normal([0; 26]),
                },
                into_bucket: BucketId(Identifier::String("bucket".into()))
            },
            r#"{
                "instruction": "TAKE_FROM_WORKTOP_BY_AMOUNT",
                "amount": {
                    "type": "Decimal",
                    "value": "123"
                },
                "resource_address": {
                    "type": "ResourceAddress",
                    "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety"
                },
                "into_bucket": {
                    "type": "Bucket",
                    "identifier": "bucket"
                }
            }"#
        ),
        InstructionSerializationTestVector::new(
            Instruction::TakeFromWorktopByAmount {
                amount: dec!("123"),
                resource_address: NetworkAwareResourceAddress {
                    network_id: 0xf2,
                    address: ResourceAddress::Normal([0; 26]),
                },
                into_bucket: BucketId(Identifier::U32(29))
            },
            r#"{
                "instruction": "TAKE_FROM_WORKTOP_BY_AMOUNT",
                "amount": {
                    "type": "Decimal",
                    "value": "123"
                },
                "resource_address": {
                    "type": "ResourceAddress",
                    "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety"
                },
                "into_bucket": {
                    "type": "Bucket",
                    "identifier": 29
                }
            }"#
        ),
        InstructionSerializationTestVector::new(
            Instruction::TakeFromWorktopByIds {
                ids: HashSet::from([NonFungibleId::Number(18),]),
                resource_address: NetworkAwareResourceAddress {
                    network_id: 0xf2,
                    address: ResourceAddress::Normal([0; 26]),
                },
                into_bucket: BucketId(Identifier::String("bucket".into()))
            },
            r#"{
                "instruction": "TAKE_FROM_WORKTOP_BY_IDS",
                "ids": [
                    {
                        "type": "NonFungibleId",
                        "variant": "Number",
                        "value": "18"
                    }
                ],
                "resource_address": {
                    "type": "ResourceAddress",
                    "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety"
                },
                "into_bucket": {
                    "type": "Bucket",
                    "identifier": "bucket"
                }
            }"#
        ),
        InstructionSerializationTestVector::new(
            Instruction::TakeFromWorktopByIds {
                ids: HashSet::from([NonFungibleId::Number(18),]),
                resource_address: NetworkAwareResourceAddress {
                    network_id: 0xf2,
                    address: ResourceAddress::Normal([0; 26]),
                },
                into_bucket: BucketId(Identifier::U32(29))
            },
            r#"{
                "instruction": "TAKE_FROM_WORKTOP_BY_IDS",
                "ids": [
                    {
                        "type": "NonFungibleId",
                        "variant": "Number",
                        "value": "18"
                    }
                ],
                "resource_address": {
                    "type": "ResourceAddress",
                    "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety"
                },
                "into_bucket": {
                    "type": "Bucket",
                    "identifier": 29
                }
            }"#
        ),
        InstructionSerializationTestVector::new(
            Instruction::ReturnToWorktop {
                bucket: BucketId(Identifier::String("bucket".into()))
            },
            r#"{
                "instruction": "RETURN_TO_WORKTOP",
                "bucket": {
                    "type": "Bucket",
                    "identifier": "bucket"
                }
            }"#
        ),
        InstructionSerializationTestVector::new(
            Instruction::ReturnToWorktop {
                bucket: BucketId(Identifier::U32(12))
            },
            r#"{
                "instruction": "RETURN_TO_WORKTOP",
                "bucket": {
                    "type": "Bucket",
                    "identifier": 12
                }
            }"#
        ),
        InstructionSerializationTestVector::new(
            Instruction::AssertWorktopContains {
                resource_address: NetworkAwareResourceAddress {
                    network_id: 0xf2,
                    address: ResourceAddress::Normal([0; 26]),
                },
            },
            r#"{
                "instruction": "ASSERT_WORKTOP_CONTAINS",
                "resource_address": {
                    "type": "ResourceAddress",
                    "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety"
                }
            }"#
        ),
        InstructionSerializationTestVector::new(
            Instruction::AssertWorktopContains {
                resource_address: NetworkAwareResourceAddress {
                    network_id: 0xf2,
                    address: ResourceAddress::Normal([0; 26]),
                },
            },
            r#"{
                "instruction": "ASSERT_WORKTOP_CONTAINS",
                "resource_address": {
                    "type": "ResourceAddress",
                    "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety"
                }
            }"#
        ),
        InstructionSerializationTestVector::new(
            Instruction::AssertWorktopContainsByAmount {
                amount: dec!("123"),
                resource_address: NetworkAwareResourceAddress {
                    network_id: 0xf2,
                    address: ResourceAddress::Normal([0; 26]),
                },
            },
            r#"{
                "instruction": "ASSERT_WORKTOP_CONTAINS_BY_AMOUNT",
                "amount": {
                    "type": "Decimal",
                    "value": "123"
                },
                "resource_address": {
                    "type": "ResourceAddress",
                    "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety"
                }
            }"#
        ),
        InstructionSerializationTestVector::new(
            Instruction::AssertWorktopContainsByAmount {
                amount: dec!("123"),
                resource_address: NetworkAwareResourceAddress {
                    network_id: 0xf2,
                    address: ResourceAddress::Normal([0; 26]),
                },
            },
            r#"{
                "instruction": "ASSERT_WORKTOP_CONTAINS_BY_AMOUNT",
                "amount": {
                    "type": "Decimal",
                    "value": "123"
                },
                "resource_address": {
                    "type": "ResourceAddress",
                    "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety"
                }
            }"#
        ),
        InstructionSerializationTestVector::new(
            Instruction::AssertWorktopContainsByIds {
                ids: HashSet::from([NonFungibleId::Number(18),]),
                resource_address: NetworkAwareResourceAddress {
                    network_id: 0xf2,
                    address: ResourceAddress::Normal([0; 26]),
                },
            },
            r#"{
                "instruction": "ASSERT_WORKTOP_CONTAINS_BY_IDS",
                "ids": [
                    {
                        "type": "NonFungibleId",
                        "variant": "Number",
                        "value": "18"
                    }
                ],
                "resource_address": {
                    "type": "ResourceAddress",
                    "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety"
                }
            }"#
        ),
        InstructionSerializationTestVector::new(
            Instruction::AssertWorktopContainsByIds {
                ids: HashSet::from([NonFungibleId::Number(18),]),
                resource_address: NetworkAwareResourceAddress {
                    network_id: 0xf2,
                    address: ResourceAddress::Normal([0; 26]),
                },
            },
            r#"{
                "instruction": "ASSERT_WORKTOP_CONTAINS_BY_IDS",
                "ids": [
                    {
                        "type": "NonFungibleId",
                        "variant": "Number",
                        "value": "18"
                    }
                ],
                "resource_address": {
                    "type": "ResourceAddress",
                    "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety"
                }
            }"#
        ),
        InstructionSerializationTestVector::new(
            Instruction::PopFromAuthZone {
                into_proof: ProofId(Identifier::String("proof".into()))
            },
            r#"{
                "instruction": "POP_FROM_AUTH_ZONE",
                "into_proof": {
                    "type": "Proof",
                    "identifier": "proof"
                }
            }"#
        ),
        InstructionSerializationTestVector::new(
            Instruction::PopFromAuthZone {
                into_proof: ProofId(Identifier::U32(12))
            },
            r#"{
                "instruction": "POP_FROM_AUTH_ZONE",
                "into_proof": {
                    "type": "Proof",
                    "identifier": 12
                }
            }"#
        ),

        InstructionSerializationTestVector::new(
            Instruction::PushToAuthZone {
                proof: ProofId(Identifier::String("proof".into()))
            },
            r#"{
                "instruction": "PUSH_TO_AUTH_ZONE",
                "proof": {
                    "type": "Proof",
                    "identifier": "proof"
                }
            }"#
        ),
        InstructionSerializationTestVector::new(
            Instruction::PushToAuthZone {
                proof: ProofId(Identifier::U32(12))
            },
            r#"{
                "instruction": "PUSH_TO_AUTH_ZONE",
                "proof": {
                    "type": "Proof",
                    "identifier": 12
                }
            }"#
        ),

        InstructionSerializationTestVector::new(
            Instruction::ClearAuthZone{},
            r#"{
                "instruction": "CLEAR_AUTH_ZONE"
            }"#
        ),

        InstructionSerializationTestVector::new(
            Instruction::CreateProofFromAuthZone {
                resource_address: NetworkAwareResourceAddress {
                    network_id: 0xf2,
                    address: ResourceAddress::Normal([0; 26]),
                },
                into_proof: ProofId(Identifier::String("proof".into()))
            },
            r#"{
                "instruction": "CREATE_PROOF_FROM_AUTH_ZONE",
                "resource_address": {
                    "type": "ResourceAddress",
                    "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety"
                },
                "into_proof": {
                    "type": "Proof",
                    "identifier": "proof"
                }
            }"#
        ),
        InstructionSerializationTestVector::new(
            Instruction::CreateProofFromAuthZone {
                resource_address: NetworkAwareResourceAddress {
                    network_id: 0xf2,
                    address: ResourceAddress::Normal([0; 26]),
                },
                into_proof: ProofId(Identifier::U32(29))
            },
            r#"{
                "instruction": "CREATE_PROOF_FROM_AUTH_ZONE",
                "resource_address": {
                    "type": "ResourceAddress",
                    "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety"
                },
                "into_proof": {
                    "type": "Proof",
                    "identifier": 29
                }
            }"#
        ),
        InstructionSerializationTestVector::new(
            Instruction::CreateProofFromAuthZoneByAmount {
                amount: dec!("123"),
                resource_address: NetworkAwareResourceAddress {
                    network_id: 0xf2,
                    address: ResourceAddress::Normal([0; 26]),
                },
                into_proof: ProofId(Identifier::String("proof".into()))
            },
            r#"{
                "instruction": "CREATE_PROOF_FROM_AUTH_ZONE_BY_AMOUNT",
                "amount": {
                    "type": "Decimal",
                    "value": "123"
                },
                "resource_address": {
                    "type": "ResourceAddress",
                    "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety"
                },
                "into_proof": {
                    "type": "Proof",
                    "identifier": "proof"
                }
            }"#
        ),
        InstructionSerializationTestVector::new(
            Instruction::CreateProofFromAuthZoneByAmount {
                amount: dec!("123"),
                resource_address: NetworkAwareResourceAddress {
                    network_id: 0xf2,
                    address: ResourceAddress::Normal([0; 26]),
                },
                into_proof: ProofId(Identifier::U32(29))
            },
            r#"{
                "instruction": "CREATE_PROOF_FROM_AUTH_ZONE_BY_AMOUNT",
                "amount": {
                    "type": "Decimal",
                    "value": "123"
                },
                "resource_address": {
                    "type": "ResourceAddress",
                    "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety"
                },
                "into_proof": {
                    "type": "Proof",
                    "identifier": 29
                }
            }"#
        ),
        InstructionSerializationTestVector::new(
            Instruction::CreateProofFromAuthZoneByIds {
                ids: HashSet::from([NonFungibleId::Number(18),]),
                resource_address: NetworkAwareResourceAddress {
                    network_id: 0xf2,
                    address: ResourceAddress::Normal([0; 26]),
                },
                into_proof: ProofId(Identifier::String("proof".into()))
            },
            r#"{
                "instruction": "CREATE_PROOF_FROM_AUTH_ZONE_BY_IDS",
                "ids": [
                    {
                        "type": "NonFungibleId",
                        "variant": "Number",
                        "value": "18"
                    }
                ],
                "resource_address": {
                    "type": "ResourceAddress",
                    "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety"
                },
                "into_proof": {
                    "type": "Proof",
                    "identifier": "proof"
                }
            }"#
        ),
        InstructionSerializationTestVector::new(
            Instruction::CreateProofFromAuthZoneByIds {
                ids: HashSet::from([NonFungibleId::Number(18),]),
                resource_address: NetworkAwareResourceAddress {
                    network_id: 0xf2,
                    address: ResourceAddress::Normal([0; 26]),
                },
                into_proof: ProofId(Identifier::U32(29))
            },
            r#"{
                "instruction": "CREATE_PROOF_FROM_AUTH_ZONE_BY_IDS",
                "ids": [
                    {
                        "type": "NonFungibleId",
                        "variant": "Number",
                        "value": "18"
                    }
                ],
                "resource_address": {
                    "type": "ResourceAddress",
                    "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety"
                },
                "into_proof": {
                    "type": "Proof",
                    "identifier": 29
                }
            }"#
        ),

        InstructionSerializationTestVector::new(
            Instruction::CreateProofFromBucket {
                bucket: BucketId(Identifier::U32(12)),
                into_proof: ProofId(Identifier::U32(12))
            },
            r#"{
                "instruction": "CREATE_PROOF_FROM_BUCKET",
                "bucket": {
                    "type": "Bucket",
                    "identifier": 12
                },
                "into_proof": {
                    "type": "Proof",
                    "identifier": 12
                }
            }"#
        ),

        InstructionSerializationTestVector::new(
            Instruction::CloneProof {
                proof: ProofId(Identifier::U32(12)),
                into_proof: ProofId(Identifier::U32(13))
            },
            r#"{
                "instruction": "CLONE_PROOF",
                "proof": {
                    "type": "Proof",
                    "identifier": 12
                },
                "into_proof": {
                    "type": "Proof",
                    "identifier": 13
                }
            }"#
        ),
        InstructionSerializationTestVector::new(
            Instruction::DropProof {
                proof: ProofId(Identifier::U32(12)),
            },
            r#"{
                "instruction": "DROP_PROOF",
                "proof": {
                    "type": "Proof",
                    "identifier": 12
                }
            }"#
        ),
        InstructionSerializationTestVector::new(
            Instruction::DropAllProofs {},
            r#"{
                "instruction": "DROP_ALL_PROOFS"
            }"#
        ),
        InstructionSerializationTestVector::new(
            Instruction::PublishPackageWithOwner {
                code: ManifestBlobRef("36dae540b7889956f1f1d8d46ba23e5e44bf5723aef2a8e6b698686c02583618".parse().unwrap()),
                abi: ManifestBlobRef("15e8699a6d63a96f66f6feeb609549be2688b96b02119f260ae6dfd012d16a5d".parse().unwrap()),
                owner_badge: radix_engine_toolkit_core::model::NonFungibleAddress {
                    resource_address: NetworkAwareResourceAddress {
                            network_id: 0xf2,
                            address: scrypto::prelude::ResourceAddress::Normal([0; 26]),
                        },
                    non_fungible_id: NonFungibleId::Number(1144418947)
                }
            },
            r#"{
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
                    "type": "NonFungibleAddress",
                    "resource_address": {
                        "type": "ResourceAddress",
                        "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety"
                    },
                    "non_fungible_id": {
                        "type": "NonFungibleId",
                        "variant": "Number",
                        "value": "1144418947"
                    }
                }
            }"#
        ),

        InstructionSerializationTestVector::new(
            Instruction::BurnResource {
                bucket: BucketId(Identifier::String("bucket".into()))
            },
            r#"{
                "instruction": "BURN_BUCKET",
                "bucket": {
                    "type": "Bucket",
                    "identifier": "bucket"
                }
            }"#
        ),
    ];

    pub static ref INSTRUCTION_AST_CONVERSIONS_TEST_VECTORS: Vec<InstructionAstConversionsTestVector> = vec![
        InstructionAstConversionsTestVector::new(
            Instruction::CallFunction {
                package_address: NetworkAwarePackageAddress {
                    network_id: 0xf2,
                    address: PackageAddress::Normal([0; 26]),
                },
                blueprint_name: "HelloWorld".into(),
                function_name: "world_hello".into(),
                arguments: Some(vec![Value::Decimal {
                    value: "129333".parse().unwrap()
                }])
            },
            r#"CALL_FUNCTION PackageAddress("package_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqxrmwtq") "HelloWorld" "world_hello" Decimal("129333");"#
        ),
        InstructionAstConversionsTestVector::new(
            Instruction::CallFunction {
                package_address: NetworkAwarePackageAddress {
                    network_id: 0xf2,
                    address: PackageAddress::Normal([0; 26]),
                },
                blueprint_name: "HelloWorld".into(),
                function_name: "world_hello".into(),
                arguments: None
            },
            r#"CALL_FUNCTION PackageAddress("package_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqxrmwtq") "HelloWorld" "world_hello";"#
        ),
        InstructionAstConversionsTestVector::new(
            Instruction::CallMethod {
                component_address: NetworkAwareComponentAddress {
                    network_id: 0xf2,
                    address: scrypto::prelude::ComponentAddress::Normal([0; 26]),
                },
                method_name: "remove_user".into(),
                arguments: Some(vec![Value::NonFungibleId {
                    value: scrypto::prelude::NonFungibleId::Number(18)
                }])
            },
            r#"CALL_METHOD ComponentAddress("component_sim1qgqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq8ecz5v") "remove_user" NonFungibleId(18u64);"#
        ),
        InstructionAstConversionsTestVector::new(
            Instruction::TakeFromWorktop {
                resource_address: NetworkAwareResourceAddress {
                    network_id: 0xf2,
                    address: ResourceAddress::Normal([0; 26]),
                },
                into_bucket: BucketId(Identifier::String("bucket".into()))
            },
            r#"TAKE_FROM_WORKTOP ResourceAddress("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety") Bucket("bucket");"#
        ),
        InstructionAstConversionsTestVector::new(
            Instruction::TakeFromWorktop {
                resource_address: NetworkAwareResourceAddress {
                    network_id: 0xf2,
                    address: ResourceAddress::Normal([0; 26]),
                },
                into_bucket: BucketId(Identifier::U32(29))
            },
            r#"TAKE_FROM_WORKTOP ResourceAddress("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety") Bucket(29u32);"#
        ),
        InstructionAstConversionsTestVector::new(
            Instruction::TakeFromWorktopByAmount {
                amount: dec!("123"),
                resource_address: NetworkAwareResourceAddress {
                    network_id: 0xf2,
                    address: ResourceAddress::Normal([0; 26]),
                },
                into_bucket: BucketId(Identifier::String("bucket".into()))
            },
            r#"TAKE_FROM_WORKTOP_BY_AMOUNT Decimal("123") ResourceAddress("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety") Bucket("bucket");"#
        ),
        InstructionAstConversionsTestVector::new(
            Instruction::TakeFromWorktopByAmount {
                amount: dec!("123"),
                resource_address: NetworkAwareResourceAddress {
                    network_id: 0xf2,
                    address: ResourceAddress::Normal([0; 26]),
                },
                into_bucket: BucketId(Identifier::U32(29))
            },
            r#"TAKE_FROM_WORKTOP_BY_AMOUNT Decimal("123") ResourceAddress("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety") Bucket(29u32);"#
        ),
        InstructionAstConversionsTestVector::new(
            Instruction::TakeFromWorktopByIds {
                ids: HashSet::from([NonFungibleId::Number(18),]),
                resource_address: NetworkAwareResourceAddress {
                    network_id: 0xf2,
                    address: ResourceAddress::Normal([0; 26]),
                },
                into_bucket: BucketId(Identifier::String("bucket".into()))
            },
            r#"TAKE_FROM_WORKTOP_BY_IDS Array<NonFungibleId>(NonFungibleId(18u32)) ResourceAddress("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety") Bucket("bucket");"#
        ),
        InstructionAstConversionsTestVector::new(
            Instruction::TakeFromWorktopByIds {
                ids: HashSet::from([NonFungibleId::Number(18),]),
                resource_address: NetworkAwareResourceAddress {
                    network_id: 0xf2,
                    address: ResourceAddress::Normal([0; 26]),
                },
                into_bucket: BucketId(Identifier::U32(29))
            },
            r#"TAKE_FROM_WORKTOP_BY_IDS Array<NonFungibleId>(NonFungibleId(18u32)) ResourceAddress("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety") Bucket(29u32);"#
        ),
        InstructionAstConversionsTestVector::new(
            Instruction::ReturnToWorktop {
                bucket: BucketId(Identifier::String("bucket".into()))
            },
            r#"RETURN_TO_WORKTOP Bucket("bucket");"#
        ),
        InstructionAstConversionsTestVector::new(
            Instruction::ReturnToWorktop {
                bucket: BucketId(Identifier::U32(12))
            },
            r#"RETURN_TO_WORKTOP Bucket(12u32);"#
        ),
        InstructionAstConversionsTestVector::new(
            Instruction::AssertWorktopContains {
                resource_address: NetworkAwareResourceAddress {
                    network_id: 0xf2,
                    address: ResourceAddress::Normal([0; 26]),
                },
            },
            r#"ASSERT_WORKTOP_CONTAINS ResourceAddress("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety");"#
        ),
        InstructionAstConversionsTestVector::new(
            Instruction::AssertWorktopContains {
                resource_address: NetworkAwareResourceAddress {
                    network_id: 0xf2,
                    address: ResourceAddress::Normal([0; 26]),
                },
            },
            r#"ASSERT_WORKTOP_CONTAINS ResourceAddress("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety");"#
        ),
        InstructionAstConversionsTestVector::new(
            Instruction::AssertWorktopContainsByAmount {
                amount: dec!("123"),
                resource_address: NetworkAwareResourceAddress {
                    network_id: 0xf2,
                    address: ResourceAddress::Normal([0; 26]),
                },
            },
            r#"ASSERT_WORKTOP_CONTAINS_BY_AMOUNT Decimal("123") ResourceAddress("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety");"#
        ),
        InstructionAstConversionsTestVector::new(
            Instruction::AssertWorktopContainsByAmount {
                amount: dec!("123"),
                resource_address: NetworkAwareResourceAddress {
                    network_id: 0xf2,
                    address: ResourceAddress::Normal([0; 26]),
                },
            },
            r#"ASSERT_WORKTOP_CONTAINS_BY_AMOUNT Decimal("123") ResourceAddress("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety");"#
        ),
        InstructionAstConversionsTestVector::new(
            Instruction::AssertWorktopContainsByIds {
                ids: HashSet::from([NonFungibleId::Number(18),]),
                resource_address: NetworkAwareResourceAddress {
                    network_id: 0xf2,
                    address: ResourceAddress::Normal([0; 26]),
                },
            },
            r#"ASSERT_WORKTOP_CONTAINS_BY_IDS Array<NonFungibleId>(NonFungibleId(18u32)) ResourceAddress("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety");"#
        ),
        InstructionAstConversionsTestVector::new(
            Instruction::AssertWorktopContainsByIds {
                ids: HashSet::from([NonFungibleId::Number(18),]),
                resource_address: NetworkAwareResourceAddress {
                    network_id: 0xf2,
                    address: ResourceAddress::Normal([0; 26]),
                },
            },
            r#"ASSERT_WORKTOP_CONTAINS_BY_IDS Array<NonFungibleId>(NonFungibleId(18u32)) ResourceAddress("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety");"#
        ),
        InstructionAstConversionsTestVector::new(
            Instruction::PopFromAuthZone {
                into_proof: ProofId(Identifier::String("proof".into()))
            },
            r#"POP_FROM_AUTH_ZONE Proof("proof");"#
        ),
        InstructionAstConversionsTestVector::new(
            Instruction::PopFromAuthZone {
                into_proof: ProofId(Identifier::U32(12))
            },
            r#"POP_FROM_AUTH_ZONE Proof(12u32);"#
        ),

        InstructionAstConversionsTestVector::new(
            Instruction::PushToAuthZone {
                proof: ProofId(Identifier::String("proof".into()))
            },
            r#"PUSH_TO_AUTH_ZONE Proof("proof");"#
        ),
        InstructionAstConversionsTestVector::new(
            Instruction::PushToAuthZone {
                proof: ProofId(Identifier::U32(12))
            },
            r#"PUSH_TO_AUTH_ZONE Proof(12u32);"#
        ),

        InstructionAstConversionsTestVector::new(
            Instruction::ClearAuthZone{},
            r#"CLEAR_AUTH_ZONE;"#
        ),

        InstructionAstConversionsTestVector::new(
            Instruction::CreateProofFromAuthZone {
                resource_address: NetworkAwareResourceAddress {
                    network_id: 0xf2,
                    address: ResourceAddress::Normal([0; 26]),
                },
                into_proof: ProofId(Identifier::String("proof".into()))
            },
            r#"CREATE_PROOF_FROM_AUTH_ZONE ResourceAddress("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety") Proof("proof");"#
        ),
        InstructionAstConversionsTestVector::new(
            Instruction::CreateProofFromAuthZone {
                resource_address: NetworkAwareResourceAddress {
                    network_id: 0xf2,
                    address: ResourceAddress::Normal([0; 26]),
                },
                into_proof: ProofId(Identifier::U32(29))
            },
            r#"CREATE_PROOF_FROM_AUTH_ZONE ResourceAddress("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety") Proof(29u32);"#
        ),
        InstructionAstConversionsTestVector::new(
            Instruction::CreateProofFromAuthZoneByAmount {
                amount: dec!("123"),
                resource_address: NetworkAwareResourceAddress {
                    network_id: 0xf2,
                    address: ResourceAddress::Normal([0; 26]),
                },
                into_proof: ProofId(Identifier::String("proof".into()))
            },
            r#"CREATE_PROOF_FROM_AUTH_ZONE_BY_AMOUNT Decimal("123") ResourceAddress("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety") Proof("proof");"#
        ),
        InstructionAstConversionsTestVector::new(
            Instruction::CreateProofFromAuthZoneByAmount {
                amount: dec!("123"),
                resource_address: NetworkAwareResourceAddress {
                    network_id: 0xf2,
                    address: ResourceAddress::Normal([0; 26]),
                },
                into_proof: ProofId(Identifier::U32(29))
            },
            r#"CREATE_PROOF_FROM_AUTH_ZONE_BY_AMOUNT Decimal("123") ResourceAddress("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety") Proof(29u32);"#
        ),
        InstructionAstConversionsTestVector::new(
            Instruction::CreateProofFromAuthZoneByIds {
                ids: HashSet::from([NonFungibleId::Number(18),]),
                resource_address: NetworkAwareResourceAddress {
                    network_id: 0xf2,
                    address: ResourceAddress::Normal([0; 26]),
                },
                into_proof: ProofId(Identifier::String("proof".into()))
            },
            r#"CREATE_PROOF_FROM_AUTH_ZONE_BY_IDS Array<NonFungibleId>(NonFungibleId(18u32)) ResourceAddress("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety") Proof("proof");"#
        ),
        InstructionAstConversionsTestVector::new(
            Instruction::CreateProofFromAuthZoneByIds {
                ids: HashSet::from([NonFungibleId::Number(18),]),
                resource_address: NetworkAwareResourceAddress {
                    network_id: 0xf2,
                    address: ResourceAddress::Normal([0; 26]),
                },
                into_proof: ProofId(Identifier::U32(29))
            },
            r#"CREATE_PROOF_FROM_AUTH_ZONE_BY_IDS Array<NonFungibleId>(NonFungibleId(18u32)) ResourceAddress("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety") Proof(29u32);"#
        ),

        InstructionAstConversionsTestVector::new(
            Instruction::CreateProofFromBucket {
                bucket: BucketId(Identifier::U32(12)),
                into_proof: ProofId(Identifier::U32(12))
            },
            r#"CREATE_PROOF_FROM_BUCKET Bucket(12u32) Proof(12u32);"#
        ),

        InstructionAstConversionsTestVector::new(
            Instruction::CloneProof {
                proof: ProofId(Identifier::U32(12)),
                into_proof: ProofId(Identifier::U32(13))
            },
            r#"CLONE_PROOF Proof(12u32) Proof(13u32);"#
        ),
        InstructionAstConversionsTestVector::new(
            Instruction::DropProof {
                proof: ProofId(Identifier::U32(12)),
            },
            r#"DROP_PROOF Proof(12u32);"#
        ),
        InstructionAstConversionsTestVector::new(
            Instruction::DropAllProofs {},
            r#"DROP_ALL_PROOFS;"#
        ),
        InstructionAstConversionsTestVector::new(
            Instruction::PublishPackageWithOwner {
                code: ManifestBlobRef("36dae540b7889956f1f1d8d46ba23e5e44bf5723aef2a8e6b698686c02583618".parse().unwrap()),
                abi: ManifestBlobRef("15e8699a6d63a96f66f6feeb609549be2688b96b02119f260ae6dfd012d16a5d".parse().unwrap()),
                owner_badge: radix_engine_toolkit_core::model::NonFungibleAddress {
                    resource_address: NetworkAwareResourceAddress {
                            network_id: 0xf2,
                            address: scrypto::prelude::ResourceAddress::Normal([0; 26]),
                        },
                    non_fungible_id: NonFungibleId::Number(1144418947)
                }
            },
            r#"PUBLISH_PACKAGE_WITH_OWNER Blob("36dae540b7889956f1f1d8d46ba23e5e44bf5723aef2a8e6b698686c02583618") Blob("15e8699a6d63a96f66f6feeb609549be2688b96b02119f260ae6dfd012d16a5d") NonFungibleAddress("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety", 1144418947u32);"#
        ),

        InstructionAstConversionsTestVector::new(
            Instruction::BurnResource {
                bucket: BucketId(Identifier::String("bucket".into()))
            },
            r#"BURN_BUCKET Bucket("bucket");"#
        ),
    ];
}
