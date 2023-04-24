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

use native_transaction::manifest::generator::{generate_instruction, NameResolver};
use native_transaction::validation::ManifestValidator;
use scrypto::address::Bech32Decoder;
use scrypto::prelude::{
    ComponentAddress, Hash, IntegerNonFungibleLocalId, ManifestBlobRef, FAUCET_PACKAGE, RADIX_TOKEN,
};

use crate::model::engine_identifier::{BucketId, NetworkAwareNodeId, ProofId, TransientIdentifier};
use crate::model::value::ast::{EnumDiscriminator, ManifestAstValue, ManifestAstValueKind};
use crate::model::{address::Bech32Coder, instruction::Instruction, transaction::InstructionList};
use crate::utils::checked_copy_u8_slice;

fn example_component_address() -> ComponentAddress {
    let address = "component_sim1pyakxvzls3cwkfp25xz7dufp9jnw6wzxe3cxaku2ju7tlyuvusk6y9";
    let decoder = Bech32Decoder::for_simulator();
    ComponentAddress::try_from_bech32(&decoder, address).unwrap()
}

pub fn call_function1() -> Instruction {
    let instruction = Instruction::CallFunction {
        package_address: ManifestAstValue::Address {
            address: NetworkAwareNodeId(FAUCET_PACKAGE.as_node_id().0, 1),
        },
        blueprint_name: ManifestAstValue::String {
            value: "Faucet".into(),
        },
        function_name: ManifestAstValue::String {
            value: "new".to_owned(),
        },
        arguments: Some(vec![ManifestAstValue::Decimal {
            value: "1".parse().unwrap(),
        }]),
    };
    check_instruction(&instruction);
    instruction
}

pub fn call_function2() -> Instruction {
    let instruction = Instruction::CallFunction {
        package_address: ManifestAstValue::Address {
            address: NetworkAwareNodeId(FAUCET_PACKAGE.as_node_id().0, 1),
        },
        blueprint_name: ManifestAstValue::String {
            value: "Faucet".into(),
        },
        function_name: ManifestAstValue::String {
            value: "new".to_owned(),
        },
        arguments: Some(vec![ManifestAstValue::Decimal {
            value: "1".parse().unwrap(),
        }]),
    };
    check_instruction(&instruction);
    instruction
}

pub fn call_function3() -> Instruction {
    let instruction = Instruction::CallFunction {
        package_address: ManifestAstValue::Address {
            address: NetworkAwareNodeId(FAUCET_PACKAGE.as_node_id().0, 1),
        },
        blueprint_name: ManifestAstValue::String {
            value: "Faucet".into(),
        },
        function_name: ManifestAstValue::String {
            value: "new".to_owned(),
        },
        arguments: Some(vec![ManifestAstValue::Decimal {
            value: "1".parse().unwrap(),
        }]),
    };
    check_instruction(&instruction);
    instruction
}

pub fn call_function4() -> Instruction {
    let instruction = Instruction::CallFunction {
        package_address: ManifestAstValue::Address {
            address: NetworkAwareNodeId(FAUCET_PACKAGE.as_node_id().0, 1),
        },
        blueprint_name: ManifestAstValue::String {
            value: "Faucet".into(),
        },
        function_name: ManifestAstValue::String {
            value: "new".to_owned(),
        },
        arguments: Some(vec![ManifestAstValue::Decimal {
            value: "1".parse().unwrap(),
        }]),
    };
    check_instruction(&instruction);
    instruction
}

pub fn call_method1() -> Instruction {
    let instruction = Instruction::CallMethod {
        component_address: ManifestAstValue::Address {
            address: NetworkAwareNodeId(example_component_address().as_node_id().0, 1),
        },
        method_name: ManifestAstValue::String {
            value: "free".into(),
        },
        arguments: Some(vec![ManifestAstValue::Decimal {
            value: "1".parse().unwrap(),
        }]),
    };
    check_instruction(&instruction);
    instruction
}

pub fn call_method2() -> Instruction {
    let instruction = Instruction::CallMethod {
        component_address: ManifestAstValue::Address {
            address: NetworkAwareNodeId(example_component_address().as_node_id().0, 1),
        },
        method_name: ManifestAstValue::String {
            value: "free".into(),
        },
        arguments: Some(vec![ManifestAstValue::Decimal {
            value: "1".parse().unwrap(),
        }]),
    };
    check_instruction(&instruction);
    instruction
}

pub fn call_method3() -> Instruction {
    let instruction = Instruction::CallMethod {
        component_address: ManifestAstValue::Address {
            address: NetworkAwareNodeId(example_component_address().as_node_id().0, 1),
        },
        method_name: ManifestAstValue::String {
            value: "free".into(),
        },
        arguments: None,
    };
    check_instruction(&instruction);
    instruction
}

pub fn call_method4() -> Instruction {
    let instruction = Instruction::CallMethod {
        component_address: ManifestAstValue::Address {
            address: NetworkAwareNodeId(example_component_address().as_node_id().0, 1),
        },
        method_name: ManifestAstValue::String {
            value: "free".into(),
        },
        arguments: None,
    };
    check_instruction(&instruction);
    instruction
}

pub fn take_from_worktop1() -> Instruction {
    let instruction = Instruction::TakeFromWorktop {
        resource_address: ManifestAstValue::Address {
            address: NetworkAwareNodeId(RADIX_TOKEN.as_node_id().0, 1),
        },
        into_bucket: ManifestAstValue::Bucket {
            identifier: BucketId(TransientIdentifier::String {
                value: "ident".to_owned(),
            }),
        },
    };
    check_instruction(&instruction);
    instruction
}

pub fn take_from_worktop2() -> Instruction {
    let instruction = Instruction::TakeFromWorktop {
        resource_address: ManifestAstValue::Address {
            address: NetworkAwareNodeId(RADIX_TOKEN.as_node_id().0, 1),
        },
        into_bucket: ManifestAstValue::Bucket {
            identifier: BucketId(TransientIdentifier::String {
                value: "ident".to_owned(),
            }),
        },
    };
    check_instruction(&instruction);
    instruction
}

pub fn take_from_worktop_by_amount1() -> Instruction {
    let instruction = Instruction::TakeFromWorktopByAmount {
        resource_address: ManifestAstValue::Address {
            address: NetworkAwareNodeId(RADIX_TOKEN.as_node_id().0, 1),
        },
        amount: ManifestAstValue::Decimal {
            value: "1".parse().unwrap(),
        },
        into_bucket: ManifestAstValue::Bucket {
            identifier: BucketId(TransientIdentifier::String {
                value: "ident".into(),
            }),
        },
    };
    check_instruction(&instruction);
    instruction
}

pub fn take_from_worktop_by_amount2() -> Instruction {
    let instruction = Instruction::TakeFromWorktopByAmount {
        resource_address: ManifestAstValue::Address {
            address: NetworkAwareNodeId(RADIX_TOKEN.as_node_id().0, 1),
        },
        amount: ManifestAstValue::Decimal {
            value: "1".parse().unwrap(),
        },
        into_bucket: ManifestAstValue::Bucket {
            identifier: BucketId(TransientIdentifier::String {
                value: "ident".into(),
            }),
        },
    };
    check_instruction(&instruction);
    instruction
}

pub fn take_from_worktop_by_ids1() -> Instruction {
    let instruction = Instruction::TakeFromWorktopByIds {
        resource_address: ManifestAstValue::Address {
            address: NetworkAwareNodeId(RADIX_TOKEN.as_node_id().0, 1),
        },
        ids: vec![ManifestAstValue::NonFungibleLocalId {
            value: scrypto::prelude::NonFungibleLocalId::Integer(IntegerNonFungibleLocalId::new(1)),
        }],
        into_bucket: ManifestAstValue::Bucket {
            identifier: BucketId(TransientIdentifier::String {
                value: "ident".into(),
            }),
        },
    };
    check_instruction(&instruction);
    instruction
}

pub fn take_from_worktop_by_ids2() -> Instruction {
    let instruction = Instruction::TakeFromWorktopByIds {
        resource_address: ManifestAstValue::Address {
            address: NetworkAwareNodeId(RADIX_TOKEN.as_node_id().0, 1),
        },
        ids: vec![ManifestAstValue::NonFungibleLocalId {
            value: scrypto::prelude::NonFungibleLocalId::Integer(IntegerNonFungibleLocalId::new(1)),
        }],
        into_bucket: ManifestAstValue::Bucket {
            identifier: BucketId(TransientIdentifier::String {
                value: "ident".into(),
            }),
        },
    };
    check_instruction(&instruction);
    instruction
}

pub fn return_to_worktop() -> Instruction {
    let instruction = Instruction::ReturnToWorktop {
        bucket: ManifestAstValue::Bucket {
            identifier: BucketId(TransientIdentifier::String {
                value: "ident".into(),
            }),
        },
    };
    check_instruction(&instruction);
    instruction
}

pub fn assert_worktop_contains1() -> Instruction {
    let instruction = Instruction::AssertWorktopContains {
        resource_address: ManifestAstValue::Address {
            address: NetworkAwareNodeId(RADIX_TOKEN.as_node_id().0, 1),
        },
    };
    check_instruction(&instruction);
    instruction
}

pub fn assert_worktop_contains2() -> Instruction {
    let instruction = Instruction::AssertWorktopContains {
        resource_address: ManifestAstValue::Address {
            address: NetworkAwareNodeId(RADIX_TOKEN.as_node_id().0, 1),
        },
    };
    check_instruction(&instruction);
    instruction
}

pub fn assert_worktop_contains_by_amount1() -> Instruction {
    let instruction = Instruction::AssertWorktopContainsByAmount {
        resource_address: ManifestAstValue::Address {
            address: NetworkAwareNodeId(RADIX_TOKEN.as_node_id().0, 1),
        },
        amount: ManifestAstValue::Decimal {
            value: "1".parse().unwrap(),
        },
    };
    check_instruction(&instruction);
    instruction
}

pub fn assert_worktop_contains_by_amount2() -> Instruction {
    let instruction = Instruction::AssertWorktopContainsByAmount {
        resource_address: ManifestAstValue::Address {
            address: NetworkAwareNodeId(RADIX_TOKEN.as_node_id().0, 1),
        },
        amount: ManifestAstValue::Decimal {
            value: "1".parse().unwrap(),
        },
    };
    check_instruction(&instruction);
    instruction
}

pub fn assert_worktop_contains_by_ids1() -> Instruction {
    let instruction = Instruction::AssertWorktopContainsByIds {
        resource_address: ManifestAstValue::Address {
            address: NetworkAwareNodeId(RADIX_TOKEN.as_node_id().0, 1),
        },
        ids: vec![ManifestAstValue::NonFungibleLocalId {
            value: scrypto::prelude::NonFungibleLocalId::Integer(IntegerNonFungibleLocalId::new(1)),
        }],
    };
    check_instruction(&instruction);
    instruction
}

pub fn assert_worktop_contains_by_ids2() -> Instruction {
    let instruction = Instruction::AssertWorktopContainsByIds {
        resource_address: ManifestAstValue::Address {
            address: NetworkAwareNodeId(RADIX_TOKEN.as_node_id().0, 1),
        },
        ids: vec![ManifestAstValue::NonFungibleLocalId {
            value: scrypto::prelude::NonFungibleLocalId::Integer(IntegerNonFungibleLocalId::new(1)),
        }],
    };
    check_instruction(&instruction);
    instruction
}

pub fn pop_from_auth_zone() -> Instruction {
    let instruction = Instruction::PopFromAuthZone {
        into_proof: ManifestAstValue::Proof {
            identifier: ProofId(TransientIdentifier::String {
                value: "ident".into(),
            }),
        },
    };
    check_instruction(&instruction);
    instruction
}

pub fn push_to_auth_zone() -> Instruction {
    let instruction = Instruction::PushToAuthZone {
        proof: ManifestAstValue::Proof {
            identifier: ProofId(TransientIdentifier::String {
                value: "ident".into(),
            }),
        },
    };
    check_instruction(&instruction);
    instruction
}

pub fn clear_auth_zone() -> Instruction {
    let instruction = Instruction::ClearAuthZone;
    check_instruction(&instruction);
    instruction
}

pub fn create_proof_from_auth_zone1() -> Instruction {
    let instruction = Instruction::CreateProofFromAuthZone {
        resource_address: ManifestAstValue::Address {
            address: NetworkAwareNodeId(RADIX_TOKEN.as_node_id().0, 1),
        },
        into_proof: ManifestAstValue::Proof {
            identifier: ProofId(TransientIdentifier::String {
                value: "ident".to_owned(),
            }),
        },
    };
    check_instruction(&instruction);
    instruction
}

pub fn create_proof_from_auth_zone2() -> Instruction {
    let instruction = Instruction::CreateProofFromAuthZone {
        resource_address: ManifestAstValue::Address {
            address: NetworkAwareNodeId(RADIX_TOKEN.as_node_id().0, 1),
        },
        into_proof: ManifestAstValue::Proof {
            identifier: ProofId(TransientIdentifier::String {
                value: "ident".to_owned(),
            }),
        },
    };
    check_instruction(&instruction);
    instruction
}

pub fn create_proof_from_auth_zone_by_amount1() -> Instruction {
    let instruction = Instruction::CreateProofFromAuthZoneByAmount {
        resource_address: ManifestAstValue::Address {
            address: NetworkAwareNodeId(RADIX_TOKEN.as_node_id().0, 1),
        },
        amount: ManifestAstValue::Decimal {
            value: "1".parse().unwrap(),
        },
        into_proof: ManifestAstValue::Proof {
            identifier: ProofId(TransientIdentifier::String {
                value: "ident".into(),
            }),
        },
    };
    check_instruction(&instruction);
    instruction
}

pub fn create_proof_from_auth_zone_by_amount2() -> Instruction {
    let instruction = Instruction::CreateProofFromAuthZoneByAmount {
        resource_address: ManifestAstValue::Address {
            address: NetworkAwareNodeId(RADIX_TOKEN.as_node_id().0, 1),
        },
        amount: ManifestAstValue::Decimal {
            value: "1".parse().unwrap(),
        },
        into_proof: ManifestAstValue::Proof {
            identifier: ProofId(TransientIdentifier::String {
                value: "ident".into(),
            }),
        },
    };
    check_instruction(&instruction);
    instruction
}

pub fn create_proof_from_auth_zone_by_ids1() -> Instruction {
    let instruction = Instruction::CreateProofFromAuthZoneByIds {
        resource_address: ManifestAstValue::Address {
            address: NetworkAwareNodeId(RADIX_TOKEN.as_node_id().0, 1),
        },
        ids: vec![ManifestAstValue::NonFungibleLocalId {
            value: scrypto::prelude::NonFungibleLocalId::Integer(IntegerNonFungibleLocalId::new(1)),
        }],
        into_proof: ManifestAstValue::Proof {
            identifier: ProofId(TransientIdentifier::String {
                value: "ident".into(),
            }),
        },
    };
    check_instruction(&instruction);
    instruction
}

pub fn create_proof_from_auth_zone_by_ids2() -> Instruction {
    let instruction = Instruction::CreateProofFromAuthZoneByIds {
        resource_address: ManifestAstValue::Address {
            address: NetworkAwareNodeId(RADIX_TOKEN.as_node_id().0, 1),
        },
        ids: vec![ManifestAstValue::NonFungibleLocalId {
            value: scrypto::prelude::NonFungibleLocalId::Integer(IntegerNonFungibleLocalId::new(1)),
        }],
        into_proof: ManifestAstValue::Proof {
            identifier: ProofId(TransientIdentifier::String {
                value: "ident".into(),
            }),
        },
    };
    check_instruction(&instruction);
    instruction
}

pub fn create_proof_from_bucket() -> Instruction {
    let instruction = Instruction::CreateProofFromBucket {
        bucket: ManifestAstValue::Bucket {
            identifier: BucketId(TransientIdentifier::String {
                value: "bucket".into(),
            }),
        },
        into_proof: ManifestAstValue::Proof {
            identifier: ProofId(TransientIdentifier::String {
                value: "Proof".into(),
            }),
        },
    };
    check_instruction(&instruction);
    instruction
}

pub fn clone_proof() -> Instruction {
    let instruction = Instruction::CloneProof {
        proof: ManifestAstValue::Proof {
            identifier: ProofId(TransientIdentifier::String {
                value: "ident".into(),
            }),
        },
        into_proof: ManifestAstValue::Proof {
            identifier: ProofId(TransientIdentifier::String {
                value: "ident2".into(),
            }),
        },
    };
    check_instruction(&instruction);
    instruction
}

pub fn publish_package() -> Instruction {
    let instruction = Instruction::PublishPackage {
        code: ManifestAstValue::Blob {
            hash: ManifestBlobRef(
                checked_copy_u8_slice(
                    hex::decode("01ba4719c80b6fe911b091a7c05124b64eeece964e09c058ef8f9805daca546b")
                        .unwrap(),
                )
                .unwrap(),
            ),
        },
        schema: ManifestAstValue::Blob {
            hash: ManifestBlobRef(
                checked_copy_u8_slice(
                    hex::decode("01ba4719c80b6fe911b091a7c05124b64eeece964e09c058ef8f9805daca546b")
                        .unwrap(),
                )
                .unwrap(),
            ),
        },
        royalty_config: ManifestAstValue::Map {
            key_value_kind: ManifestAstValueKind::String,
            value_value_kind: ManifestAstValueKind::Tuple,
            entries: Vec::new(),
        },
        metadata: ManifestAstValue::Map {
            key_value_kind: ManifestAstValueKind::String,
            value_value_kind: ManifestAstValueKind::String,
            entries: Vec::new(),
        },
    };
    check_instruction(&instruction);
    instruction
}

pub fn publish_package_advanced() -> Instruction {
    let instruction = Instruction::PublishPackageAdvanced {
        code: ManifestAstValue::Blob {
            hash: ManifestBlobRef(
                checked_copy_u8_slice(
                    hex::decode("01ba4719c80b6fe911b091a7c05124b64eeece964e09c058ef8f9805daca546b")
                        .unwrap(),
                )
                .unwrap(),
            ),
        },
        schema: ManifestAstValue::Blob {
            hash: ManifestBlobRef(
                checked_copy_u8_slice(
                    hex::decode("01ba4719c80b6fe911b091a7c05124b64eeece964e09c058ef8f9805daca546b")
                        .unwrap(),
                )
                .unwrap(),
            ),
        },
        royalty_config: ManifestAstValue::Map {
            key_value_kind: ManifestAstValueKind::String,
            value_value_kind: ManifestAstValueKind::Tuple,
            entries: Vec::new(),
        },
        metadata: ManifestAstValue::Map {
            key_value_kind: ManifestAstValueKind::String,
            value_value_kind: ManifestAstValueKind::String,
            entries: Vec::new(),
        },
        access_rules: ManifestAstValue::Tuple {
            elements: vec![
                ManifestAstValue::Map {
                    key_value_kind: ManifestAstValueKind::Tuple,
                    value_value_kind: ManifestAstValueKind::Enum,
                    entries: vec![],
                },
                ManifestAstValue::Map {
                    key_value_kind: ManifestAstValueKind::Tuple,
                    value_value_kind: ManifestAstValueKind::Enum,
                    entries: vec![],
                },
                ManifestAstValue::Map {
                    key_value_kind: ManifestAstValueKind::String,
                    value_value_kind: ManifestAstValueKind::Enum,
                    entries: vec![],
                },
                ManifestAstValue::Enum {
                    variant: EnumDiscriminator::U8 { discriminator: 0 },
                    fields: Some(vec![ManifestAstValue::Enum {
                        variant: EnumDiscriminator::U8 { discriminator: 0 },
                        fields: None,
                    }]),
                },
                ManifestAstValue::Map {
                    key_value_kind: ManifestAstValueKind::Tuple,
                    value_value_kind: ManifestAstValueKind::Enum,
                    entries: vec![],
                },
                ManifestAstValue::Map {
                    key_value_kind: ManifestAstValueKind::String,
                    value_value_kind: ManifestAstValueKind::Enum,
                    entries: vec![],
                },
                ManifestAstValue::Enum {
                    variant: EnumDiscriminator::U8 { discriminator: 0 },
                    fields: Some(vec![ManifestAstValue::Enum {
                        variant: EnumDiscriminator::U8 { discriminator: 0 },
                        fields: None,
                    }]),
                },
            ],
        },
    };
    check_instruction(&instruction);
    instruction
}

pub fn burn_resource() -> Instruction {
    let instruction = Instruction::BurnResource {
        bucket: ManifestAstValue::Bucket {
            identifier: BucketId(TransientIdentifier::String {
                value: "ident".into(),
            }),
        },
    };
    check_instruction(&instruction);
    instruction
}

pub fn drop_all_proofs() -> Instruction {
    let instruction = Instruction::DropAllProofs;
    check_instruction(&instruction);
    instruction
}

pub fn drop_proof() -> Instruction {
    let instruction = Instruction::DropProof {
        proof: ManifestAstValue::Proof {
            identifier: ProofId(TransientIdentifier::String {
                value: "proof".into(),
            }),
        },
    };
    check_instruction(&instruction);
    instruction
}

pub fn recall_resource() -> Instruction {
    let instruction = Instruction::RecallResource {
        vault_id: ManifestAstValue::Address {
            address: "internal_vault_sim1pcqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz5f7ul"
                .parse()
                .unwrap(),
        },
        amount: ManifestAstValue::Decimal {
            value: "1".parse().unwrap(),
        },
    };
    check_instruction(&instruction);
    instruction
}

pub fn set_metadata() -> Instruction {
    let instruction = Instruction::SetMetadata {
        entity_address: ManifestAstValue::Address {
            address: NetworkAwareNodeId(example_component_address().as_node_id().0, 1),
        },
        key: ManifestAstValue::String {
            value: "name".into(),
        },
        value: ManifestAstValue::Enum {
            variant: EnumDiscriminator::U8 { discriminator: 0u8 },
            fields: Some(vec![ManifestAstValue::Enum {
                variant: EnumDiscriminator::U8 { discriminator: 0u8 },
                fields: Some(vec![ManifestAstValue::String {
                    value: "deadbeef".into(),
                }]),
            }]),
        },
    };
    check_instruction(&instruction);
    instruction
}

pub fn remove_metadata() -> Instruction {
    let instruction = Instruction::RemoveMetadata {
        entity_address: ManifestAstValue::Address {
            address: NetworkAwareNodeId(example_component_address().as_node_id().0, 1),
        },
        key: ManifestAstValue::String {
            value: "name".into(),
        },
    };
    check_instruction(&instruction);
    instruction
}

pub fn set_package_royalty_config() -> Instruction {
    let instruction = Instruction::SetPackageRoyaltyConfig {
        package_address: ManifestAstValue::Address {
            address: NetworkAwareNodeId(FAUCET_PACKAGE.as_node_id().0, 1),
        },
        royalty_config: ManifestAstValue::Map {
            key_value_kind: ManifestAstValueKind::String,
            value_value_kind: ManifestAstValueKind::Tuple,
            entries: Vec::new(),
        },
    };
    check_instruction(&instruction);
    instruction
}

pub fn set_component_royalty_config() -> Instruction {
    let instruction = Instruction::SetComponentRoyaltyConfig {
        component_address: ManifestAstValue::Address {
            address: NetworkAwareNodeId(example_component_address().as_node_id().0, 1),
        },
        royalty_config: ManifestAstValue::Tuple {
            elements: vec![
                ManifestAstValue::Map {
                    key_value_kind: ManifestAstValueKind::String,
                    value_value_kind: ManifestAstValueKind::U32,
                    entries: vec![],
                },
                ManifestAstValue::U32 { value: 1 },
            ],
        },
    };
    check_instruction(&instruction);
    instruction
}

pub fn claim_package_royalty() -> Instruction {
    let instruction = Instruction::ClaimPackageRoyalty {
        package_address: ManifestAstValue::Address {
            address: NetworkAwareNodeId(FAUCET_PACKAGE.as_node_id().0, 1),
        },
    };
    check_instruction(&instruction);
    instruction
}

pub fn claim_component_royalty() -> Instruction {
    let instruction = Instruction::ClaimComponentRoyalty {
        component_address: ManifestAstValue::Address {
            address: NetworkAwareNodeId(example_component_address().as_node_id().0, 1),
        },
    };
    check_instruction(&instruction);
    instruction
}

pub fn set_method_access_rule() -> Instruction {
    let instruction = Instruction::SetMethodAccessRule {
        entity_address: ManifestAstValue::Address {
            address: NetworkAwareNodeId(example_component_address().as_node_id().0, 1),
        },
        key: ManifestAstValue::Tuple {
            elements: vec![
                ManifestAstValue::Enum {
                    variant: EnumDiscriminator::U8 { discriminator: 0 },
                    fields: None,
                },
                ManifestAstValue::String {
                    value: "free".to_owned(),
                },
            ],
        },
        rule: ManifestAstValue::Enum {
            variant: EnumDiscriminator::U8 { discriminator: 0 },
            fields: None,
        },
    };
    check_instruction(&instruction);
    instruction
}

pub fn mint_fungible() -> Instruction {
    let instruction = Instruction::MintFungible {
        resource_address: ManifestAstValue::Address {
            address: NetworkAwareNodeId(RADIX_TOKEN.as_node_id().0, 1),
        },
        amount: ManifestAstValue::Decimal {
            value: "1".parse().unwrap(),
        },
    };
    check_instruction(&instruction);
    instruction
}

pub fn mint_non_fungible() -> Instruction {
    let instruction = Instruction::MintNonFungible {
        resource_address: ManifestAstValue::Address {
            address: NetworkAwareNodeId(RADIX_TOKEN.as_node_id().0, 1),
        },
        entries: ManifestAstValue::Map {
            key_value_kind: ManifestAstValueKind::NonFungibleLocalId,
            value_value_kind: ManifestAstValueKind::Tuple,
            entries: Vec::new(),
        },
    };
    check_instruction(&instruction);
    instruction
}

pub fn mint_uuid_non_fungible() -> Instruction {
    let instruction = Instruction::MintUuidNonFungible {
        resource_address: ManifestAstValue::Address {
            address: NetworkAwareNodeId(RADIX_TOKEN.as_node_id().0, 1),
        },
        entries: ManifestAstValue::Array {
            element_kind: ManifestAstValueKind::Tuple,
            elements: vec![
                ManifestAstValue::Tuple {
                    elements: vec![
                        ManifestAstValue::Tuple { elements: vec![] },
                        ManifestAstValue::Tuple { elements: vec![] },
                    ],
                },
                ManifestAstValue::Tuple {
                    elements: vec![
                        ManifestAstValue::Tuple { elements: vec![] },
                        ManifestAstValue::Tuple { elements: vec![] },
                    ],
                },
            ],
        },
    };
    check_instruction(&instruction);
    instruction
}

pub fn create_fungible_resource() -> Instruction {
    let instruction = Instruction::CreateFungibleResource {
        divisibility: ManifestAstValue::U8 { value: 18 },
        metadata: ManifestAstValue::Map {
            key_value_kind: ManifestAstValueKind::String,
            value_value_kind: ManifestAstValueKind::String,
            entries: Vec::new(),
        },
        access_rules: ManifestAstValue::Map {
            key_value_kind: ManifestAstValueKind::Enum,
            value_value_kind: ManifestAstValueKind::Tuple,
            entries: Vec::new(),
        },
    };
    check_instruction(&instruction);
    instruction
}

pub fn create_fungible_resource_with_initial_supply() -> Instruction {
    let instruction = Instruction::CreateFungibleResourceWithInitialSupply {
        divisibility: ManifestAstValue::U8 { value: 18 },
        metadata: ManifestAstValue::Map {
            key_value_kind: ManifestAstValueKind::String,
            value_value_kind: ManifestAstValueKind::String,
            entries: Vec::new(),
        },
        access_rules: ManifestAstValue::Map {
            key_value_kind: ManifestAstValueKind::Enum,
            value_value_kind: ManifestAstValueKind::Tuple,
            entries: Vec::new(),
        },
        initial_supply: ManifestAstValue::Decimal {
            value: "1".parse().unwrap(),
        },
    };
    check_instruction(&instruction);
    instruction
}

pub fn create_non_fungible_resource() -> Instruction {
    let instruction = Instruction::CreateNonFungibleResource {
        id_type: ManifestAstValue::Enum {
            variant: EnumDiscriminator::U8 { discriminator: 0 },
            fields: None,
        },
        schema: ManifestAstValue::Tuple {
            elements: vec![
                ManifestAstValue::Tuple {
                    elements: vec![
                        ManifestAstValue::Array {
                            element_kind: ManifestAstValueKind::Enum,
                            elements: vec![],
                        },
                        ManifestAstValue::Array {
                            element_kind: ManifestAstValueKind::Tuple,
                            elements: vec![],
                        },
                        ManifestAstValue::Array {
                            element_kind: ManifestAstValueKind::Enum,
                            elements: vec![],
                        },
                    ],
                },
                ManifestAstValue::Enum {
                    variant: EnumDiscriminator::U8 { discriminator: 0 },
                    fields: Some(vec![ManifestAstValue::U8 { value: 64 }]),
                },
                ManifestAstValue::Array {
                    element_kind: ManifestAstValueKind::String,
                    elements: vec![],
                },
            ],
        },
        metadata: ManifestAstValue::Map {
            key_value_kind: ManifestAstValueKind::String,
            value_value_kind: ManifestAstValueKind::String,
            entries: Vec::new(),
        },
        access_rules: ManifestAstValue::Map {
            key_value_kind: ManifestAstValueKind::Enum,
            value_value_kind: ManifestAstValueKind::Tuple,
            entries: Vec::new(),
        },
    };
    check_instruction(&instruction);
    instruction
}

// TODO: Figure out correct representation.
// pub fn create_non_fungible_resource_with_initial_supply() -> Instruction {
//     let instruction = Instruction::CreateNonFungibleResourceWithInitialSupply {
//         id_type: ManifestAstValue::Enum {
//             variant: EnumDiscriminator::U8 { discriminator: 0 },
//             fields: None,
//         },
//         metadata: ManifestAstValue::Map {
//             key_value_kind: ManifestAstValueKind::String,
//             value_value_kind: ManifestAstValueKind::String,
//             entries: Vec::new(),
//         },
//         access_rules: ManifestAstValue::Map {
//             key_value_kind: ManifestAstValueKind::Enum,
//             value_value_kind: ManifestAstValueKind::Tuple,
//             entries: Vec::new(),
//         },
//         initial_supply: ManifestAstValue::Array {
//             element_kind: ManifestAstValueKind::Array,
//             elements: vec![],
//         },
//     };
//     check_instruction(&instruction);
//     instruction
// }

pub fn create_access_controller() -> Instruction {
    let instruction = Instruction::CreateAccessController {
        controlled_asset: ManifestAstValue::Bucket {
            identifier: BucketId(TransientIdentifier::String {
                value: "ident".into(),
            }),
        },
        rule_set: ManifestAstValue::Tuple {
            elements: vec![
                ManifestAstValue::Enum {
                    variant: EnumDiscriminator::U8 { discriminator: 0 },
                    fields: None,
                },
                ManifestAstValue::Enum {
                    variant: EnumDiscriminator::U8 { discriminator: 0 },
                    fields: None,
                },
                ManifestAstValue::Enum {
                    variant: EnumDiscriminator::U8 { discriminator: 0 },
                    fields: None,
                },
            ],
        },
        timed_recovery_delay_in_minutes: ManifestAstValue::Some {
            value: Box::new(ManifestAstValue::U32 { value: 1 }),
        },
    };
    check_instruction(&instruction);
    instruction
}

pub fn create_identity() -> Instruction {
    let instruction = Instruction::CreateIdentity {};
    check_instruction(&instruction);
    instruction
}

pub fn create_identity_advanced() -> Instruction {
    let instruction = Instruction::CreateIdentityAdvanced {
        config: ManifestAstValue::Tuple {
            elements: vec![
                ManifestAstValue::Map {
                    key_value_kind: ManifestAstValueKind::Tuple,
                    value_value_kind: ManifestAstValueKind::Enum,
                    entries: vec![],
                },
                ManifestAstValue::Map {
                    key_value_kind: ManifestAstValueKind::Tuple,
                    value_value_kind: ManifestAstValueKind::Enum,
                    entries: vec![],
                },
                ManifestAstValue::Map {
                    key_value_kind: ManifestAstValueKind::String,
                    value_value_kind: ManifestAstValueKind::Enum,
                    entries: vec![],
                },
                ManifestAstValue::Enum {
                    variant: EnumDiscriminator::U8 { discriminator: 0 },
                    fields: Some(vec![ManifestAstValue::Enum {
                        variant: EnumDiscriminator::U8 { discriminator: 0 },
                        fields: None,
                    }]),
                },
                ManifestAstValue::Map {
                    key_value_kind: ManifestAstValueKind::Tuple,
                    value_value_kind: ManifestAstValueKind::Enum,
                    entries: vec![],
                },
                ManifestAstValue::Map {
                    key_value_kind: ManifestAstValueKind::String,
                    value_value_kind: ManifestAstValueKind::Enum,
                    entries: vec![],
                },
                ManifestAstValue::Enum {
                    variant: EnumDiscriminator::U8 { discriminator: 0 },
                    fields: Some(vec![ManifestAstValue::Enum {
                        variant: EnumDiscriminator::U8 { discriminator: 0 },
                        fields: None,
                    }]),
                },
            ],
        },
    };
    check_instruction(&instruction);
    instruction
}

pub fn create_account() -> Instruction {
    let instruction = Instruction::CreateAccount {};
    check_instruction(&instruction);
    instruction
}

pub fn create_account_advanced() -> Instruction {
    let instruction = Instruction::CreateAccountAdvanced {
        config: ManifestAstValue::Tuple {
            elements: vec![
                ManifestAstValue::Map {
                    key_value_kind: ManifestAstValueKind::Tuple,
                    value_value_kind: ManifestAstValueKind::Enum,
                    entries: vec![],
                },
                ManifestAstValue::Map {
                    key_value_kind: ManifestAstValueKind::Tuple,
                    value_value_kind: ManifestAstValueKind::Enum,
                    entries: vec![],
                },
                ManifestAstValue::Map {
                    key_value_kind: ManifestAstValueKind::String,
                    value_value_kind: ManifestAstValueKind::Enum,
                    entries: vec![],
                },
                ManifestAstValue::Enum {
                    variant: EnumDiscriminator::U8 { discriminator: 0 },
                    fields: Some(vec![ManifestAstValue::Enum {
                        variant: EnumDiscriminator::U8 { discriminator: 0 },
                        fields: None,
                    }]),
                },
                ManifestAstValue::Map {
                    key_value_kind: ManifestAstValueKind::Tuple,
                    value_value_kind: ManifestAstValueKind::Enum,
                    entries: vec![],
                },
                ManifestAstValue::Map {
                    key_value_kind: ManifestAstValueKind::String,
                    value_value_kind: ManifestAstValueKind::Enum,
                    entries: vec![],
                },
                ManifestAstValue::Enum {
                    variant: EnumDiscriminator::U8 { discriminator: 0 },
                    fields: Some(vec![ManifestAstValue::Enum {
                        variant: EnumDiscriminator::U8 { discriminator: 0 },
                        fields: None,
                    }]),
                },
            ],
        },
    };
    check_instruction(&instruction);
    instruction
}

pub fn create_validator() -> Instruction {
    let instruction = Instruction::CreateValidator {
        key: ManifestAstValue::Bytes {
            value: hex::decode(
                "0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
            )
            .unwrap(),
        },
    };
    check_instruction(&instruction);
    instruction
}

pub fn clear_signature_proofs() -> Instruction {
    let instruction = Instruction::ClearSignatureProofs;
    check_instruction(&instruction);
    instruction
}

fn check_instruction(instruction: &Instruction) {
    let bech32_coder = Bech32Coder::new(0x01);
    let mut blobs = vec![];
    let mut name_resolver = NameResolver::new();
    let mut id_validator = ManifestValidator::new();

    match instruction {
        Instruction::PushToAuthZone {
            proof:
                ManifestAstValue::Proof {
                    identifier: ProofId(TransientIdentifier::String { value }),
                },
        }
        | Instruction::CloneProof {
            proof:
                ManifestAstValue::Proof {
                    identifier: ProofId(TransientIdentifier::String { value }),
                },
            ..
        }
        | Instruction::DropProof {
            proof:
                ManifestAstValue::Proof {
                    identifier: ProofId(TransientIdentifier::String { value }),
                },
        } => {
            name_resolver
                .insert_proof(
                    value.to_string(),
                    id_validator
                        .new_proof(native_transaction::validation::ProofKind::AuthZoneProof)
                        .unwrap(),
                )
                .unwrap();
        }
        Instruction::ReturnToWorktop {
            bucket:
                ManifestAstValue::Bucket {
                    identifier: BucketId(TransientIdentifier::String { value }),
                },
        }
        | Instruction::CreateProofFromBucket {
            bucket:
                ManifestAstValue::Bucket {
                    identifier: BucketId(TransientIdentifier::String { value }),
                },
            ..
        }
        | Instruction::BurnResource {
            bucket:
                ManifestAstValue::Bucket {
                    identifier: BucketId(TransientIdentifier::String { value }),
                },
        }
        | Instruction::CreateAccessController {
            controlled_asset:
                ManifestAstValue::Bucket {
                    identifier: BucketId(TransientIdentifier::String { value }),
                },
            ..
        } => name_resolver
            .insert_bucket(value.to_string(), id_validator.new_bucket().unwrap())
            .unwrap(),
        Instruction::PublishPackage {
            code: ManifestAstValue::Blob { hash: code },
            schema: ManifestAstValue::Blob { hash: abi },
            ..
        } => {
            blobs.push(Hash(code.0));
            blobs.push(Hash(abi.0));
        }
        _ => {}
    }

    let instruction = InstructionList::Parsed(vec![instruction.clone()])
        .ast_instructions(&bech32_coder)
        .unwrap()[0]
        .clone();

    generate_instruction(
        &instruction,
        &mut id_validator,
        &mut name_resolver,
        bech32_coder.decoder(),
        &blobs.iter().map(|hash| (*hash, vec![])).collect(),
    )
    .unwrap_or_else(|error| panic!("Failed at: {:?}. Error: {:?}", instruction, error));
}
