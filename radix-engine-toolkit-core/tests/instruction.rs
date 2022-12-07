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

use radix_engine_toolkit_core::model::{
    BucketId, Identifier, Instruction, NetworkAwareComponentAddress, NetworkAwarePackageAddress,
    NetworkAwareResourceAddress, ScryptoReceiver, Value, ProofId, NonFungibleAddress, ValueKind,
};
use scrypto::prelude::*;

#[macro_use]
extern crate lazy_static;

#[test]
fn serialized_instructions_match_expected() {
    // Checking that the serialization of instructions matches
    for test_vector in JSON_CONVERSION_TEST_VECTORS.iter() {
        // Act
        let expected_serialized_instruction: serde_json::Value =
            serde_json::from_str(&test_vector.json_representation)
                .expect("Failed to deserialize trusted value");
        let serialized_instruction = serde_json::to_value(&test_vector.instruction)
            .expect("Failed to serialize trusted instruction");

        // Assert
        assert_eq!(expected_serialized_instruction, serialized_instruction);
    }
}

#[test]
fn deserialized_values_match_expected() {
    // Checking that the deserialization of values matches
    for test_vector in JSON_CONVERSION_TEST_VECTORS.iter() {
        // Act
        let expected_instruction = &test_vector.instruction;
        let deserialized_instruction = serde_json::from_str(&test_vector.json_representation)
            .expect("Deserialization failed!");

        // Assert
        assert_eq!(*expected_instruction, deserialized_instruction)
    }
}

struct InstructionSerializationTestVector {
    instruction: Instruction,
    json_representation: String,
}

impl InstructionSerializationTestVector {
    fn new<S: AsRef<str>>(instruction: Instruction, json_representation: S) -> Self {
        let json_representation: &str = json_representation.as_ref();
        let json_representation: String = json_representation.to_string();
        Self {
            instruction,
            json_representation,
        }
    }
}

lazy_static! {
    static ref JSON_CONVERSION_TEST_VECTORS: Vec<InstructionSerializationTestVector> = vec![
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
            Instruction::CallNativeFunction {
                blueprint_name: "HelloWorld".into(),
                function_name: "world_hello".into(),
                arguments: Some(vec![Value::Decimal {
                    value: "129333".parse().unwrap()
                }])
            },
            r#"{
                    "instruction": "CALL_NATIVE_FUNCTION",
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
            Instruction::CallNativeFunction {
                blueprint_name: "HelloWorld".into(),
                function_name: "world_hello".into(),
                arguments: None
            },
            r#"{
                    "instruction": "CALL_NATIVE_FUNCTION",
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
                component_address: ScryptoReceiver::ComponentAddress(
                    NetworkAwareComponentAddress {
                        network_id: 0xf2,
                        address: scrypto::prelude::ComponentAddress::Normal([0; 26]),
                    }
                ),
                method_name: "remove_user".into(),
                arguments: Some(vec![Value::NonFungibleId {
                    value: scrypto::prelude::NonFungibleId::U64(18)
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
                            "variant": "U64",
                            "value": "18"
                        }
                    ]
                }"#
        ),
        InstructionSerializationTestVector::new(
            Instruction::CallMethod {
                component_address: ScryptoReceiver::Component(
                    "000000000000000000000000000000000000000000000000000000000000000000000005"
                        .parse()
                        .unwrap()
                ),
                method_name: "remove_user".into(),
                arguments: Some(vec![Value::NonFungibleId {
                    value: scrypto::prelude::NonFungibleId::U64(18)
                }])
            },
            r#"{
                    "instruction": "CALL_METHOD",
                    "component_address": {
                        "type": "Component",
                        "identifier": "000000000000000000000000000000000000000000000000000000000000000000000005"
                    },
                    "method_name": {
                        "type": "String",
                        "value": "remove_user"
                    },
                    "arguments": [
                        {
                            "type": "NonFungibleId",
                            "variant": "U64",
                            "value": "18"
                        }
                    ]
                }"#
        ),
        InstructionSerializationTestVector::new(
            Instruction::CallNativeMethod {
                receiver: radix_engine_toolkit_core::model::RENode::Bucket(
                    radix_engine_toolkit_core::model::Identifier::U32(32)
                ),
                method_name: "inspect".into(),
                arguments: None
            },
            r#"{
                    "instruction": "CALL_NATIVE_METHOD",
                    "receiver": {
                        "type": "Bucket",
                        "identifier": 32
                    },
                    "method_name": {
                        "type": "String",
                        "value": "inspect"
                    }
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
                ids: HashSet::from([NonFungibleId::U32(18),]),
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
                        "variant": "U32",
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
                ids: HashSet::from([NonFungibleId::U32(18),]),
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
                        "variant": "U32",
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
                ids: HashSet::from([NonFungibleId::U32(18),]),
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
                        "variant": "U32",
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
                ids: HashSet::from([NonFungibleId::U32(18),]),
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
                        "variant": "U32",
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
                ids: HashSet::from([NonFungibleId::U32(18),]),
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
                        "variant": "U32",
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
                ids: HashSet::from([NonFungibleId::U32(18),]),
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
                        "variant": "U32",
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
                into_proof: ProofId(Identifier::U32(12))
            },
            r#"{
                "instruction": "CLONE_PROOF",
                "proof": {
                    "type": "Proof",
                    "identifier": 12
                },
                "into_proof": {
                    "type": "Proof",
                    "identifier": 12
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
                code: Blob("36dae540b7889956f1f1d8d46ba23e5e44bf5723aef2a8e6b698686c02583618".parse().unwrap()), 
                abi: Blob("15e8699a6d63a96f66f6feeb609549be2688b96b02119f260ae6dfd012d16a5d".parse().unwrap()), 
                owner_badge: NonFungibleAddress {
                    resource_address: NetworkAwareResourceAddress {
                            network_id: 0xf2,
                            address: scrypto::prelude::ResourceAddress::Normal([0; 26]),
                        },
                    non_fungible_id: NonFungibleId::U32(1144418947)
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
                        "variant": "U32",
                        "value": "1144418947"
                    }
                }
            }"#
        ),

        // TODO: Better test and more structured `CreateResource` is needed here. 
        InstructionSerializationTestVector::new(
            Instruction::CreateResource { 
                resource_type: Value::Enum { variant: "Fungible".into(), fields: Some(vec![Value::U8 { value: 18 }]) },
                metadata: Value::Array { element_type: ValueKind::Tuple, elements: vec![] },
                access_rules: Value::Array { element_type: ValueKind::Tuple, elements: vec![] },
                mint_params: Value::Option { value: Box::new(None) }
            },
            r#"{
                "instruction": "CREATE_RESOURCE",
                "resource_type": {
                    "type": "Enum",
                    "variant": "Fungible",
                    "fields": [
                        {
                            "type": "U8",
                            "value": "18"
                        }
                    ]
                },
                "metadata": {
                    "type": "Array",
                    "element_type": "Tuple",
                    "elements": []
                },
                "access_rules": {
                    "type": "Array",
                    "element_type": "Tuple",
                    "elements": []
                },
                "mint_params": {
                    "type": "Option",
                    "variant": "None"
                }
            }"#
        ),

        InstructionSerializationTestVector::new(
            Instruction::BurnBucket { 
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
        InstructionSerializationTestVector::new(
            Instruction::MintFungible {
                amount: dec!("123"),
                resource_address: NetworkAwareResourceAddress {
                    network_id: 0xf2,
                    address: ResourceAddress::Normal([0; 26]),
                },
            },
            r#"{
                "instruction": "MINT_FUNGIBLE",
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
    ];
}
