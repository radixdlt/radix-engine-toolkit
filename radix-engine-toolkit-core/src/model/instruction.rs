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

use radix_transaction::manifest::ast::{Instruction as AstInstruction, Value as AstValue};
use scrypto::prelude::{
    Decimal, EcdsaSecp256k1PublicKey, ManifestBlobRef, NonFungibleId, SystemAddress,
};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::collections::HashSet;

use crate::error::Error;
use crate::model::value::*;
use crate::model::{
    Bech32Coder, BucketId, EntityAddress, NetworkAwareComponentAddress, NetworkAwarePackageAddress,
    NetworkAwareResourceAddress, NonFungibleAddress, ProofId, ValueSerializationProxy,
};
use crate::traits::ValidateWithContext;

use super::NetworkAwareSystemAddress;

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "instruction", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Instruction {
    CallFunction {
        #[serde_as(as = "ValueSerializationProxy")]
        package_address: NetworkAwarePackageAddress,

        #[serde_as(as = "ValueSerializationProxy")]
        blueprint_name: String,

        #[serde_as(as = "ValueSerializationProxy")]
        function_name: String,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        arguments: Option<Vec<Value>>,
    },

    CallMethod {
        #[serde_as(as = "ValueSerializationProxy")]
        component_address: NetworkAwareComponentAddress,

        #[serde_as(as = "ValueSerializationProxy")]
        method_name: String,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        arguments: Option<Vec<Value>>,
    },

    TakeFromWorktop {
        #[serde_as(as = "ValueSerializationProxy")]
        resource_address: NetworkAwareResourceAddress,

        #[serde_as(as = "ValueSerializationProxy")]
        into_bucket: BucketId,
    },
    TakeFromWorktopByAmount {
        #[serde_as(as = "ValueSerializationProxy")]
        amount: Decimal,

        #[serde_as(as = "ValueSerializationProxy")]
        resource_address: NetworkAwareResourceAddress,

        #[serde_as(as = "ValueSerializationProxy")]
        into_bucket: BucketId,
    },
    TakeFromWorktopByIds {
        #[serde_as(as = "HashSet<ValueSerializationProxy>")]
        ids: HashSet<NonFungibleId>,

        #[serde_as(as = "ValueSerializationProxy")]
        resource_address: NetworkAwareResourceAddress,

        #[serde_as(as = "ValueSerializationProxy")]
        into_bucket: BucketId,
    },
    ReturnToWorktop {
        #[serde_as(as = "ValueSerializationProxy")]
        bucket: BucketId,
    },

    AssertWorktopContains {
        #[serde_as(as = "ValueSerializationProxy")]
        resource_address: NetworkAwareResourceAddress,
    },
    AssertWorktopContainsByAmount {
        #[serde_as(as = "ValueSerializationProxy")]
        amount: Decimal,

        #[serde_as(as = "ValueSerializationProxy")]
        resource_address: NetworkAwareResourceAddress,
    },
    AssertWorktopContainsByIds {
        #[serde_as(as = "HashSet<ValueSerializationProxy>")]
        ids: HashSet<NonFungibleId>,

        #[serde_as(as = "ValueSerializationProxy")]
        resource_address: NetworkAwareResourceAddress,
    },

    PopFromAuthZone {
        #[serde_as(as = "ValueSerializationProxy")]
        into_proof: ProofId,
    },
    PushToAuthZone {
        #[serde_as(as = "ValueSerializationProxy")]
        proof: ProofId,
    },
    ClearAuthZone,

    CreateProofFromAuthZone {
        #[serde_as(as = "ValueSerializationProxy")]
        resource_address: NetworkAwareResourceAddress,

        #[serde_as(as = "ValueSerializationProxy")]
        into_proof: ProofId,
    },
    CreateProofFromAuthZoneByAmount {
        #[serde_as(as = "ValueSerializationProxy")]
        amount: Decimal,

        #[serde_as(as = "ValueSerializationProxy")]
        resource_address: NetworkAwareResourceAddress,

        #[serde_as(as = "ValueSerializationProxy")]
        into_proof: ProofId,
    },
    CreateProofFromAuthZoneByIds {
        #[serde_as(as = "HashSet<ValueSerializationProxy>")]
        ids: HashSet<NonFungibleId>,

        #[serde_as(as = "ValueSerializationProxy")]
        resource_address: NetworkAwareResourceAddress,

        #[serde_as(as = "ValueSerializationProxy")]
        into_proof: ProofId,
    },

    CreateProofFromBucket {
        #[serde_as(as = "ValueSerializationProxy")]
        bucket: BucketId,

        #[serde_as(as = "ValueSerializationProxy")]
        into_proof: ProofId,
    },

    CloneProof {
        #[serde_as(as = "ValueSerializationProxy")]
        proof: ProofId,

        #[serde_as(as = "ValueSerializationProxy")]
        into_proof: ProofId,
    },
    DropProof {
        #[serde_as(as = "ValueSerializationProxy")]
        proof: ProofId,
    },
    DropAllProofs,

    PublishPackage {
        #[serde_as(as = "ValueSerializationProxy")]
        code: ManifestBlobRef,

        #[serde_as(as = "ValueSerializationProxy")]
        abi: ManifestBlobRef,

        // TODO: Switch to a more structured format
        royalty_config: Value,
        metadata: Value,
        access_rules: Value,
    },
    PublishPackageWithOwner {
        #[serde_as(as = "ValueSerializationProxy")]
        code: ManifestBlobRef,

        #[serde_as(as = "ValueSerializationProxy")]
        abi: ManifestBlobRef,

        #[serde_as(as = "ValueSerializationProxy")]
        owner_badge: NonFungibleAddress,
    },

    BurnResource {
        #[serde_as(as = "ValueSerializationProxy")]
        bucket: BucketId,
    },
    // TODO: Switch to a more structured format
    RecallResource {
        vault_id: Value,
        amount: Value,
    },
    SetMetadata {
        entity_address: Value,
        key: Value,
        value: Value,
    },
    SetPackageRoyaltyConfig {
        package_address: Value,
        royalty_config: Value,
    },

    SetComponentRoyaltyConfig {
        component_address: Value,
        royalty_config: Value,
    },

    // TODO: Dedicated bucket for this?
    ClaimPackageRoyalty {
        package_address: Value,
    },

    // TODO: Dedicated bucket for this?
    ClaimComponentRoyalty {
        component_address: Value,
    },

    SetMethodAccessRule {
        entity_address: Value,
        index: Value,
        key: Value,
        rule: Value,
    },

    // TODO: Figure out a better structured format for this
    CreateFungibleResource {
        divisibility: Value,
        metadata: Value,
        access_rules: Value,
        initial_supply: Value,
    },

    // TODO: Figure out a better structured format for this
    CreateFungibleResourceWithOwner {
        divisibility: Value,
        metadata: Value,
        owner_badge: Value,
        initial_supply: Value,
    },

    // TODO: Figure out a better structured format for this
    CreateNonFungibleResource {
        id_type: Value,
        metadata: Value,
        access_rules: Value,
        initial_supply: Value,
    },

    // TODO: Figure out a better structured format for this
    CreateNonFungibleResourceWithOwner {
        id_type: Value,
        metadata: Value,
        owner_badge: Value,
        initial_supply: Value,
    },

    // TODO: Figure out a better structured format for this
    MintFungible {
        resource_address: Value,
        amount: Value,
    },
    // TODO: Figure out a better structured format for this
    MintNonFungible {
        resource_address: Value,
        entries: Value,
    },

    RegisterValidator {
        #[serde_as(as = "ValueSerializationProxy")]
        validator: EcdsaSecp256k1PublicKey,
    },

    UnregisterValidator {
        #[serde_as(as = "ValueSerializationProxy")]
        validator: EcdsaSecp256k1PublicKey,
    },
}

impl Instruction {
    // ============
    // Conversions
    // ============
    pub fn to_ast_instruction(&self, bech32_coder: &Bech32Coder) -> Result<AstInstruction, Error> {
        let instruction = self.clone();
        let ast_instruction = match instruction {
            Self::CallFunction {
                package_address,
                blueprint_name,
                function_name,
                arguments,
            } => AstInstruction::CallFunction {
                package_address: Value::from(package_address).to_ast_value(bech32_coder)?,
                blueprint_name: Value::from(blueprint_name).to_ast_value(bech32_coder)?,
                function_name: Value::from(function_name).to_ast_value(bech32_coder)?,
                args: arguments
                    .unwrap_or_default()
                    .iter()
                    .map(|v| v.to_ast_value(bech32_coder))
                    .collect::<Result<Vec<AstValue>, _>>()?,
            },
            Self::CallMethod {
                component_address,
                method_name,
                arguments,
            } => AstInstruction::CallMethod {
                component_address: Value::from(component_address).to_ast_value(bech32_coder)?,
                method_name: Value::from(method_name).to_ast_value(bech32_coder)?,
                args: arguments
                    .unwrap_or_default()
                    .iter()
                    .map(|v| v.to_ast_value(bech32_coder))
                    .collect::<Result<Vec<AstValue>, _>>()?,
            },
            Self::TakeFromWorktop {
                resource_address,
                into_bucket,
            } => AstInstruction::TakeFromWorktop {
                resource_address: Value::from(resource_address).to_ast_value(bech32_coder)?,
                new_bucket: Value::from(into_bucket).to_ast_value(bech32_coder)?,
            },
            Self::TakeFromWorktopByAmount {
                amount,
                resource_address,
                into_bucket,
            } => AstInstruction::TakeFromWorktopByAmount {
                amount: Value::from(amount).to_ast_value(bech32_coder)?,
                resource_address: Value::from(resource_address).to_ast_value(bech32_coder)?,
                new_bucket: Value::from(into_bucket).to_ast_value(bech32_coder)?,
            },
            Self::TakeFromWorktopByIds {
                ids,
                resource_address,
                into_bucket,
            } => AstInstruction::TakeFromWorktopByIds {
                ids: Value::Array {
                    element_type: crate::model::value::ValueKind::Bucket,
                    elements: ids.into_iter().map(Value::from).collect(),
                }
                .to_ast_value(bech32_coder)?,
                resource_address: Value::from(resource_address).to_ast_value(bech32_coder)?,
                new_bucket: Value::from(into_bucket).to_ast_value(bech32_coder)?,
            },
            Self::ReturnToWorktop { bucket } => AstInstruction::ReturnToWorktop {
                bucket: Value::from(bucket).to_ast_value(bech32_coder)?,
            },

            Self::AssertWorktopContains { resource_address } => {
                AstInstruction::AssertWorktopContains {
                    resource_address: Value::from(resource_address).to_ast_value(bech32_coder)?,
                }
            }
            Self::AssertWorktopContainsByAmount {
                amount,
                resource_address,
            } => AstInstruction::AssertWorktopContainsByAmount {
                amount: Value::from(amount).to_ast_value(bech32_coder)?,
                resource_address: Value::from(resource_address).to_ast_value(bech32_coder)?,
            },
            Self::AssertWorktopContainsByIds {
                ids,
                resource_address,
            } => AstInstruction::AssertWorktopContainsByIds {
                ids: Value::Array {
                    element_type: crate::model::value::ValueKind::Bucket,
                    elements: ids.into_iter().map(Value::from).collect(),
                }
                .to_ast_value(bech32_coder)?,
                resource_address: Value::from(resource_address).to_ast_value(bech32_coder)?,
            },

            Self::PopFromAuthZone { into_proof } => AstInstruction::PopFromAuthZone {
                new_proof: Value::from(into_proof).to_ast_value(bech32_coder)?,
            },
            Self::PushToAuthZone { proof } => AstInstruction::PushToAuthZone {
                proof: Value::from(proof).to_ast_value(bech32_coder)?,
            },
            Self::ClearAuthZone => AstInstruction::ClearAuthZone,

            Self::CreateProofFromAuthZone {
                resource_address,
                into_proof,
            } => AstInstruction::CreateProofFromAuthZone {
                resource_address: Value::from(resource_address).to_ast_value(bech32_coder)?,
                new_proof: Value::from(into_proof).to_ast_value(bech32_coder)?,
            },
            Self::CreateProofFromAuthZoneByAmount {
                amount,
                resource_address,
                into_proof,
            } => AstInstruction::CreateProofFromAuthZoneByAmount {
                amount: Value::from(amount).to_ast_value(bech32_coder)?,
                resource_address: Value::from(resource_address).to_ast_value(bech32_coder)?,
                new_proof: Value::from(into_proof).to_ast_value(bech32_coder)?,
            },
            Self::CreateProofFromAuthZoneByIds {
                ids,
                resource_address,
                into_proof,
            } => AstInstruction::CreateProofFromAuthZoneByIds {
                ids: Value::Array {
                    element_type: crate::model::value::ValueKind::Bucket,
                    elements: ids.into_iter().map(Value::from).collect(),
                }
                .to_ast_value(bech32_coder)?,
                resource_address: Value::from(resource_address).to_ast_value(bech32_coder)?,
                new_proof: Value::from(into_proof).to_ast_value(bech32_coder)?,
            },
            Self::CreateProofFromBucket { bucket, into_proof } => {
                AstInstruction::CreateProofFromBucket {
                    bucket: Value::from(bucket).to_ast_value(bech32_coder)?,
                    new_proof: Value::from(into_proof).to_ast_value(bech32_coder)?,
                }
            }

            Self::CloneProof { proof, into_proof } => AstInstruction::CloneProof {
                proof: Value::from(proof).to_ast_value(bech32_coder)?,
                new_proof: Value::from(into_proof).to_ast_value(bech32_coder)?,
            },

            Self::DropProof { proof } => AstInstruction::DropProof {
                proof: Value::from(proof).to_ast_value(bech32_coder)?,
            },
            Self::DropAllProofs => AstInstruction::DropAllProofs,
            Self::PublishPackageWithOwner {
                code,
                abi,
                owner_badge,
            } => AstInstruction::PublishPackageWithOwner {
                owner_badge: Value::from(owner_badge).to_ast_value(bech32_coder)?,
                code: Value::from(code).to_ast_value(bech32_coder)?,
                abi: Value::from(abi).to_ast_value(bech32_coder)?,
            },
            Self::BurnResource { bucket } => AstInstruction::BurnResource {
                bucket: Value::from(bucket).to_ast_value(bech32_coder)?,
            },
            Self::PublishPackage {
                code,
                abi,
                royalty_config,
                metadata,
                access_rules,
            } => AstInstruction::PublishPackage {
                code: Value::from(code).to_ast_value(bech32_coder)?,
                abi: Value::from(abi).to_ast_value(bech32_coder)?,
                royalty_config: royalty_config.to_ast_value(bech32_coder)?,
                metadata: metadata.to_ast_value(bech32_coder)?,
                access_rules: access_rules.to_ast_value(bech32_coder)?,
            },

            Self::RecallResource { vault_id, amount } => AstInstruction::RecallResource {
                vault_id: vault_id.to_ast_value(bech32_coder)?,
                amount: amount.to_ast_value(bech32_coder)?,
            },

            Self::SetMetadata {
                entity_address,
                key,
                value,
            } => AstInstruction::SetMetadata {
                entity_address: entity_address.to_ast_value(bech32_coder)?,
                key: key.to_ast_value(bech32_coder)?,
                value: value.to_ast_value(bech32_coder)?,
            },

            Self::SetPackageRoyaltyConfig {
                package_address,
                royalty_config,
            } => AstInstruction::SetPackageRoyaltyConfig {
                package_address: package_address.to_ast_value(bech32_coder)?,
                royalty_config: royalty_config.to_ast_value(bech32_coder)?,
            },

            Self::SetComponentRoyaltyConfig {
                component_address,
                royalty_config,
            } => AstInstruction::SetComponentRoyaltyConfig {
                component_address: component_address.to_ast_value(bech32_coder)?,
                royalty_config: royalty_config.to_ast_value(bech32_coder)?,
            },

            // TODO: Dedicated bucket for this?
            Self::ClaimPackageRoyalty { package_address } => AstInstruction::ClaimPackageRoyalty {
                package_address: package_address.to_ast_value(bech32_coder)?,
            },

            // TODO: Dedicated bucket for this?
            Self::ClaimComponentRoyalty { component_address } => {
                AstInstruction::ClaimComponentRoyalty {
                    component_address: component_address.to_ast_value(bech32_coder)?,
                }
            }

            Self::SetMethodAccessRule {
                entity_address,
                index,
                key,
                rule,
            } => AstInstruction::SetMethodAccessRule {
                entity_address: entity_address.to_ast_value(bech32_coder)?,
                index: index.to_ast_value(bech32_coder)?,
                key: key.to_ast_value(bech32_coder)?,
                rule: rule.to_ast_value(bech32_coder)?,
            },

            Self::CreateFungibleResource {
                divisibility,
                metadata,
                access_rules,
                initial_supply,
            } => AstInstruction::CreateFungibleResource {
                divisibility: divisibility.to_ast_value(bech32_coder)?,
                metadata: metadata.to_ast_value(bech32_coder)?,
                access_rules: access_rules.to_ast_value(bech32_coder)?,
                initial_supply: initial_supply.to_ast_value(bech32_coder)?,
            },
            Self::CreateFungibleResourceWithOwner {
                divisibility,
                metadata,
                owner_badge,
                initial_supply,
            } => AstInstruction::CreateFungibleResourceWithOwner {
                divisibility: divisibility.to_ast_value(bech32_coder)?,
                metadata: metadata.to_ast_value(bech32_coder)?,
                owner_badge: owner_badge.to_ast_value(bech32_coder)?,
                initial_supply: initial_supply.to_ast_value(bech32_coder)?,
            },
            Self::CreateNonFungibleResource {
                id_type,
                metadata,
                access_rules,
                initial_supply,
            } => AstInstruction::CreateNonFungibleResource {
                id_type: id_type.to_ast_value(bech32_coder)?,
                metadata: metadata.to_ast_value(bech32_coder)?,
                access_rules: access_rules.to_ast_value(bech32_coder)?,
                initial_supply: initial_supply.to_ast_value(bech32_coder)?,
            },
            Self::CreateNonFungibleResourceWithOwner {
                id_type,
                metadata,
                owner_badge,
                initial_supply,
            } => AstInstruction::CreateNonFungibleResourceWithOwner {
                id_type: id_type.to_ast_value(bech32_coder)?,
                metadata: metadata.to_ast_value(bech32_coder)?,
                owner_badge: owner_badge.to_ast_value(bech32_coder)?,
                initial_supply: initial_supply.to_ast_value(bech32_coder)?,
            },
            Self::MintFungible {
                resource_address,
                amount,
            } => AstInstruction::MintFungible {
                resource_address: resource_address.to_ast_value(bech32_coder)?,
                amount: amount.to_ast_value(bech32_coder)?,
            },
            Self::MintNonFungible {
                resource_address,
                entries,
            } => AstInstruction::MintNonFungible {
                resource_address: resource_address.to_ast_value(bech32_coder)?,
                entries: entries.to_ast_value(bech32_coder)?,
            },
            Self::RegisterValidator { validator } => AstInstruction::RegisterValidator {
                validator: Value::from(validator).to_ast_value(bech32_coder)?,
            },
            Self::UnregisterValidator { validator } => AstInstruction::RegisterValidator {
                validator: Value::from(validator).to_ast_value(bech32_coder)?,
            },
        };
        Ok(ast_instruction)
    }

    pub fn from_ast_instruction(
        ast_instruction: &AstInstruction,
        bech32_coder: &Bech32Coder,
    ) -> Result<Self, Error> {
        let instruction = match ast_instruction {
            AstInstruction::CallFunction {
                package_address,
                blueprint_name,
                function_name,
                args,
            } => Self::CallFunction {
                package_address: Value::from_ast_value(package_address, bech32_coder)?
                    .try_into()?,
                blueprint_name: Value::from_ast_value(blueprint_name, bech32_coder)?.try_into()?,
                function_name: Value::from_ast_value(function_name, bech32_coder)?.try_into()?,
                arguments: {
                    let arguments = args
                        .iter()
                        .map(|v| Value::from_ast_value(v, bech32_coder))
                        .collect::<Result<Vec<Value>, _>>()?;
                    match arguments.len() {
                        0 => None,
                        _ => Some(arguments),
                    }
                },
            },
            AstInstruction::CallMethod {
                component_address,
                method_name,
                args,
            } => Self::CallMethod {
                component_address: Value::from_ast_value(component_address, bech32_coder)?
                    .try_into()?,
                method_name: Value::from_ast_value(method_name, bech32_coder)?.try_into()?,
                arguments: {
                    let arguments = args
                        .iter()
                        .map(|v| Value::from_ast_value(v, bech32_coder))
                        .collect::<Result<Vec<Value>, _>>()?;
                    match arguments.len() {
                        0 => None,
                        _ => Some(arguments),
                    }
                },
            },

            AstInstruction::TakeFromWorktop {
                resource_address,
                new_bucket,
            } => Self::TakeFromWorktop {
                resource_address: Value::from_ast_value(resource_address, bech32_coder)?
                    .try_into()?,
                into_bucket: Value::from_ast_value(new_bucket, bech32_coder)?.try_into()?,
            },
            AstInstruction::TakeFromWorktopByAmount {
                amount,
                resource_address,
                new_bucket,
            } => Self::TakeFromWorktopByAmount {
                amount: Value::from_ast_value(amount, bech32_coder)?.try_into()?,
                resource_address: Value::from_ast_value(resource_address, bech32_coder)?
                    .try_into()?,
                into_bucket: Value::from_ast_value(new_bucket, bech32_coder)?.try_into()?,
            },
            AstInstruction::TakeFromWorktopByIds {
                ids,
                resource_address,
                new_bucket,
            } => Self::TakeFromWorktopByIds {
                ids: if let Value::Array {
                    element_type: _,
                    elements,
                } = Value::from_ast_value(ids, bech32_coder)?
                {
                    elements
                        .into_iter()
                        .map(NonFungibleId::try_from)
                        .collect::<Result<HashSet<NonFungibleId>, _>>()?
                } else {
                    panic!("Expected type Array!")
                },
                resource_address: Value::from_ast_value(resource_address, bech32_coder)?
                    .try_into()?,
                into_bucket: Value::from_ast_value(new_bucket, bech32_coder)?.try_into()?,
            },
            AstInstruction::ReturnToWorktop { bucket } => Self::ReturnToWorktop {
                bucket: Value::from_ast_value(bucket, bech32_coder)?.try_into()?,
            },

            AstInstruction::AssertWorktopContains { resource_address } => {
                Self::AssertWorktopContains {
                    resource_address: Value::from_ast_value(resource_address, bech32_coder)?
                        .try_into()?,
                }
            }
            AstInstruction::AssertWorktopContainsByAmount {
                amount,
                resource_address,
            } => Self::AssertWorktopContainsByAmount {
                amount: Value::from_ast_value(amount, bech32_coder)?.try_into()?,
                resource_address: Value::from_ast_value(resource_address, bech32_coder)?
                    .try_into()?,
            },
            AstInstruction::AssertWorktopContainsByIds {
                ids,
                resource_address,
            } => Self::AssertWorktopContainsByIds {
                ids: if let Value::Array {
                    element_type: _,
                    elements,
                } = Value::from_ast_value(ids, bech32_coder)?
                {
                    elements
                        .into_iter()
                        .map(NonFungibleId::try_from)
                        .collect::<Result<HashSet<NonFungibleId>, _>>()?
                } else {
                    panic!("Expected type Array!")
                },
                resource_address: Value::from_ast_value(resource_address, bech32_coder)?
                    .try_into()?,
            },

            AstInstruction::PopFromAuthZone { new_proof } => Self::PopFromAuthZone {
                into_proof: Value::from_ast_value(new_proof, bech32_coder)?.try_into()?,
            },
            AstInstruction::PushToAuthZone { proof } => Self::PushToAuthZone {
                proof: Value::from_ast_value(proof, bech32_coder)?.try_into()?,
            },
            AstInstruction::ClearAuthZone => Self::ClearAuthZone,

            AstInstruction::CreateProofFromAuthZone {
                resource_address,
                new_proof,
            } => Self::CreateProofFromAuthZone {
                resource_address: Value::from_ast_value(resource_address, bech32_coder)?
                    .try_into()?,
                into_proof: Value::from_ast_value(new_proof, bech32_coder)?.try_into()?,
            },
            AstInstruction::CreateProofFromAuthZoneByAmount {
                amount,
                resource_address,
                new_proof,
            } => Self::CreateProofFromAuthZoneByAmount {
                amount: Value::from_ast_value(amount, bech32_coder)?.try_into()?,
                resource_address: Value::from_ast_value(resource_address, bech32_coder)?
                    .try_into()?,
                into_proof: Value::from_ast_value(new_proof, bech32_coder)?.try_into()?,
            },
            AstInstruction::CreateProofFromAuthZoneByIds {
                ids,
                resource_address,
                new_proof,
            } => Self::CreateProofFromAuthZoneByIds {
                ids: if let Value::Array {
                    element_type: _,
                    elements,
                } = Value::from_ast_value(ids, bech32_coder)?
                {
                    elements
                        .into_iter()
                        .map(NonFungibleId::try_from)
                        .collect::<Result<HashSet<NonFungibleId>, _>>()?
                } else {
                    panic!("Expected type Array!")
                },
                resource_address: Value::from_ast_value(resource_address, bech32_coder)?
                    .try_into()?,
                into_proof: Value::from_ast_value(new_proof, bech32_coder)?.try_into()?,
            },
            AstInstruction::CreateProofFromBucket { bucket, new_proof } => {
                Self::CreateProofFromBucket {
                    bucket: Value::from_ast_value(bucket, bech32_coder)?.try_into()?,
                    into_proof: Value::from_ast_value(new_proof, bech32_coder)?.try_into()?,
                }
            }

            AstInstruction::CloneProof { proof, new_proof } => Self::CloneProof {
                proof: Value::from_ast_value(proof, bech32_coder)?.try_into()?,
                into_proof: Value::from_ast_value(new_proof, bech32_coder)?.try_into()?,
            },
            AstInstruction::DropProof { proof } => Self::DropProof {
                proof: Value::from_ast_value(proof, bech32_coder)?.try_into()?,
            },
            AstInstruction::DropAllProofs => Self::DropAllProofs,
            AstInstruction::PublishPackageWithOwner {
                code,
                abi,
                owner_badge,
            } => Self::PublishPackageWithOwner {
                owner_badge: Value::from_ast_value(owner_badge, bech32_coder)?.try_into()?,
                code: Value::from_ast_value(code, bech32_coder)?.try_into()?,
                abi: Value::from_ast_value(abi, bech32_coder)?.try_into()?,
            },
            AstInstruction::BurnResource { bucket } => Self::BurnResource {
                bucket: Value::from_ast_value(bucket, bech32_coder)?.try_into()?,
            },
            AstInstruction::PublishPackage {
                code,
                abi,
                royalty_config,
                metadata,
                access_rules,
            } => Self::PublishPackage {
                code: Value::from_ast_value(code, bech32_coder)?.try_into()?,
                abi: Value::from_ast_value(abi, bech32_coder)?.try_into()?,
                royalty_config: Value::from_ast_value(royalty_config, bech32_coder)?,
                metadata: Value::from_ast_value(metadata, bech32_coder)?,
                access_rules: Value::from_ast_value(access_rules, bech32_coder)?,
            },
            AstInstruction::RecallResource { vault_id, amount } => Self::RecallResource {
                vault_id: Value::from_ast_value(vault_id, bech32_coder)?,
                amount: Value::from_ast_value(amount, bech32_coder)?,
            },
            AstInstruction::SetMetadata {
                entity_address,
                key,
                value,
            } => Self::SetMetadata {
                entity_address: Value::from_ast_value(entity_address, bech32_coder)?,
                key: Value::from_ast_value(key, bech32_coder)?,
                value: Value::from_ast_value(value, bech32_coder)?,
            },

            AstInstruction::SetPackageRoyaltyConfig {
                package_address,
                royalty_config,
            } => Self::SetPackageRoyaltyConfig {
                package_address: Value::from_ast_value(package_address, bech32_coder)?,
                royalty_config: Value::from_ast_value(royalty_config, bech32_coder)?,
            },

            AstInstruction::SetComponentRoyaltyConfig {
                component_address,
                royalty_config,
            } => Self::SetComponentRoyaltyConfig {
                component_address: Value::from_ast_value(component_address, bech32_coder)?,
                royalty_config: Value::from_ast_value(royalty_config, bech32_coder)?,
            },

            AstInstruction::ClaimPackageRoyalty { package_address } => Self::ClaimPackageRoyalty {
                package_address: Value::from_ast_value(package_address, bech32_coder)?,
            },

            AstInstruction::ClaimComponentRoyalty { component_address } => {
                Self::ClaimComponentRoyalty {
                    component_address: Value::from_ast_value(component_address, bech32_coder)?,
                }
            }

            AstInstruction::SetMethodAccessRule {
                entity_address,
                index,
                key,
                rule,
            } => Self::SetMethodAccessRule {
                entity_address: Value::from_ast_value(entity_address, bech32_coder)?,
                index: Value::from_ast_value(index, bech32_coder)?,
                key: Value::from_ast_value(key, bech32_coder)?,
                rule: Value::from_ast_value(rule, bech32_coder)?,
            },

            AstInstruction::CreateFungibleResource {
                divisibility,
                metadata,
                access_rules,
                initial_supply,
            } => Self::CreateFungibleResource {
                divisibility: Value::from_ast_value(divisibility, bech32_coder)?,
                metadata: Value::from_ast_value(metadata, bech32_coder)?,
                access_rules: Value::from_ast_value(access_rules, bech32_coder)?,
                initial_supply: Value::from_ast_value(initial_supply, bech32_coder)?,
            },
            AstInstruction::CreateFungibleResourceWithOwner {
                divisibility,
                metadata,
                owner_badge,
                initial_supply,
            } => Self::CreateFungibleResourceWithOwner {
                divisibility: Value::from_ast_value(divisibility, bech32_coder)?,
                metadata: Value::from_ast_value(metadata, bech32_coder)?,
                owner_badge: Value::from_ast_value(owner_badge, bech32_coder)?,
                initial_supply: Value::from_ast_value(initial_supply, bech32_coder)?,
            },
            AstInstruction::CreateNonFungibleResource {
                id_type,
                metadata,
                access_rules,
                initial_supply,
            } => Self::CreateNonFungibleResource {
                id_type: Value::from_ast_value(id_type, bech32_coder)?,
                metadata: Value::from_ast_value(metadata, bech32_coder)?,
                access_rules: Value::from_ast_value(access_rules, bech32_coder)?,
                initial_supply: Value::from_ast_value(initial_supply, bech32_coder)?,
            },
            AstInstruction::CreateNonFungibleResourceWithOwner {
                id_type,
                metadata,
                owner_badge,
                initial_supply,
            } => Self::CreateNonFungibleResourceWithOwner {
                id_type: Value::from_ast_value(id_type, bech32_coder)?,
                metadata: Value::from_ast_value(metadata, bech32_coder)?,
                owner_badge: Value::from_ast_value(owner_badge, bech32_coder)?,
                initial_supply: Value::from_ast_value(initial_supply, bech32_coder)?,
            },

            AstInstruction::MintFungible {
                resource_address,
                amount,
            } => Self::MintFungible {
                resource_address: Value::from_ast_value(resource_address, bech32_coder)?,
                amount: Value::from_ast_value(amount, bech32_coder)?,
            },
            AstInstruction::MintNonFungible {
                resource_address,
                entries,
            } => Self::MintNonFungible {
                resource_address: Value::from_ast_value(resource_address, bech32_coder)?,
                entries: Value::from_ast_value(entries, bech32_coder)?,
            },
            AstInstruction::RegisterValidator { validator } => Self::RegisterValidator {
                validator: Value::from_ast_value(validator, bech32_coder)?.try_into()?,
            },
            AstInstruction::UnregisterValidator { validator } => Self::UnregisterValidator {
                validator: Value::from_ast_value(validator, bech32_coder)?.try_into()?,
            },
        };
        Ok(instruction)
    }
}

// ===========
// Validation
// ===========

impl ValidateWithContext<u8> for Instruction {
    fn validate(&self, network_id: u8) -> Result<(), Error> {
        let instruction = self.clone();
        match instruction {
            Self::CallFunction {
                package_address,
                arguments,
                ..
            } => {
                EntityAddress::from(package_address).validate(network_id)?;
                arguments
                    .unwrap_or_default()
                    .iter()
                    .map(|arg| arg.validate((network_id, None)))
                    .collect::<Result<Vec<()>, Error>>()?;
                Ok(())
            }

            Self::CallMethod {
                component_address,
                arguments,
                ..
            } => {
                EntityAddress::from(component_address).validate(network_id)?;
                arguments
                    .unwrap_or_default()
                    .iter()
                    .map(|arg| arg.validate((network_id, None)))
                    .collect::<Result<Vec<()>, Error>>()?;
                Ok(())
            }

            Self::TakeFromWorktop {
                resource_address, ..
            } => {
                EntityAddress::from(resource_address).validate(network_id)?;
                Ok(())
            }
            Self::TakeFromWorktopByAmount {
                resource_address, ..
            } => {
                EntityAddress::from(resource_address).validate(network_id)?;
                Ok(())
            }
            Self::TakeFromWorktopByIds {
                resource_address, ..
            } => {
                EntityAddress::from(resource_address).validate(network_id)?;
                Ok(())
            }
            Self::ReturnToWorktop { bucket: _ } => Ok(()),

            Self::AssertWorktopContains { resource_address } => {
                EntityAddress::from(resource_address).validate(network_id)?;
                Ok(())
            }
            Self::AssertWorktopContainsByAmount {
                resource_address, ..
            } => {
                EntityAddress::from(resource_address).validate(network_id)?;
                Ok(())
            }
            Self::AssertWorktopContainsByIds {
                resource_address, ..
            } => {
                EntityAddress::from(resource_address).validate(network_id)?;
                Ok(())
            }

            Self::PopFromAuthZone { .. } => Ok(()),
            Self::PushToAuthZone { .. } => Ok(()),
            Self::ClearAuthZone => Ok(()),

            Self::CreateProofFromAuthZone {
                resource_address, ..
            } => {
                EntityAddress::from(resource_address).validate(network_id)?;
                Ok(())
            }
            Self::CreateProofFromAuthZoneByAmount {
                resource_address, ..
            } => {
                EntityAddress::from(resource_address).validate(network_id)?;
                Ok(())
            }
            Self::CreateProofFromAuthZoneByIds {
                resource_address, ..
            } => {
                EntityAddress::from(resource_address).validate(network_id)?;
                Ok(())
            }

            Self::CreateProofFromBucket { .. } => Ok(()),

            Self::CloneProof { proof: _, .. } => Ok(()),
            Self::DropProof { proof: _ } => Ok(()),
            Self::DropAllProofs => Ok(()),

            Self::PublishPackageWithOwner { .. } => Ok(()),

            Self::BurnResource { bucket: _ } => Ok(()),
            // TODO: Add validation for these instructions
            _ => Ok(()),
        }
    }
}
