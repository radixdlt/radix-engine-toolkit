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

use super::Instruction;
use super::InstructionConversionError;
use super::PackageSchemaResolutionError;
use crate::model::address::Bech32Coder;
use crate::model::value::ast::ManifestAstValue;
use crate::model::value::ast::ManifestAstValueKind;
use crate::utils;
use itertools::Itertools;
use native_transaction::manifest::ast;
use native_transaction::manifest::generator::generate_value;
use native_transaction::manifest::generator::NameResolver;
use native_transaction::manifest::lexer;
use native_transaction::manifest::parser::Parser;
use scrypto::prelude::{manifest_decode, manifest_encode};
use scrypto::schema::PackageSchema;

impl Instruction {
    pub fn to_ast_instruction(
        &self,
        bech32_coder: &Bech32Coder,
    ) -> Result<ast::Instruction, InstructionConversionError> {
        let ast_instruction = match self {
            Self::CallFunction {
                package_address,
                blueprint_name,
                function_name,
                arguments,
            } => ast::Instruction::CallFunction {
                package_address: package_address.to_ast_value(bech32_coder)?,
                blueprint_name: blueprint_name.to_ast_value(bech32_coder)?,
                function_name: function_name.to_ast_value(bech32_coder)?,
                args: arguments
                    .iter()
                    .map(|value| value.to_ast_value(bech32_coder))
                    .collect::<Result<Vec<_>, _>>()?,
            },
            Self::CallMethod {
                component_address,
                method_name,
                arguments,
            } => ast::Instruction::CallMethod {
                address: component_address.to_ast_value(bech32_coder)?,
                method_name: method_name.to_ast_value(bech32_coder)?,
                args: arguments
                    .iter()
                    .map(|value| value.to_ast_value(bech32_coder))
                    .collect::<Result<Vec<_>, _>>()?,
            },
            Self::CallRoyaltyMethod {
                component_address,
                method_name,
                arguments,
            } => ast::Instruction::CallRoyaltyMethod {
                address: component_address.to_ast_value(bech32_coder)?,
                method_name: method_name.to_ast_value(bech32_coder)?,
                args: arguments
                    .iter()
                    .map(|value| value.to_ast_value(bech32_coder))
                    .collect::<Result<Vec<_>, _>>()?,
            },
            Self::CallMetadataMethod {
                component_address,
                method_name,
                arguments,
            } => ast::Instruction::CallMetadataMethod {
                address: component_address.to_ast_value(bech32_coder)?,
                method_name: method_name.to_ast_value(bech32_coder)?,
                args: arguments
                    .iter()
                    .map(|value| value.to_ast_value(bech32_coder))
                    .collect::<Result<Vec<_>, _>>()?,
            },
            Self::CallAccessRulesMethod {
                component_address,
                method_name,
                arguments,
            } => ast::Instruction::CallAccessRulesMethod {
                address: component_address.to_ast_value(bech32_coder)?,
                method_name: method_name.to_ast_value(bech32_coder)?,
                args: arguments
                    .iter()
                    .map(|value| value.to_ast_value(bech32_coder))
                    .collect::<Result<Vec<_>, _>>()?,
            },
            Self::TakeAllFromWorktop {
                resource_address,
                into_bucket,
            } => ast::Instruction::TakeAllFromWorktop {
                resource_address: resource_address.to_ast_value(bech32_coder)?,
                new_bucket: into_bucket.to_ast_value(bech32_coder)?,
            },
            Self::TakeFromWorktop {
                amount,
                resource_address,
                into_bucket,
            } => ast::Instruction::TakeFromWorktop {
                amount: amount.to_ast_value(bech32_coder)?,
                resource_address: resource_address.to_ast_value(bech32_coder)?,
                new_bucket: into_bucket.to_ast_value(bech32_coder)?,
            },
            Self::TakeNonFungiblesFromWorktop {
                ids,
                resource_address,
                into_bucket,
            } => ast::Instruction::TakeNonFungiblesFromWorktop {
                ids: ManifestAstValue::Array {
                    element_kind: ManifestAstValueKind::NonFungibleLocalId,
                    elements: ids.clone().into_iter().collect::<Vec<_>>(),
                }
                .to_ast_value(bech32_coder)?,
                resource_address: resource_address.to_ast_value(bech32_coder)?,
                new_bucket: into_bucket.to_ast_value(bech32_coder)?,
            },
            Self::ReturnToWorktop { bucket } => ast::Instruction::ReturnToWorktop {
                bucket: bucket.to_ast_value(bech32_coder)?,
            },

            Self::AssertWorktopContains {
                resource_address,
                amount,
            } => ast::Instruction::AssertWorktopContains {
                amount: amount.to_ast_value(bech32_coder)?,
                resource_address: resource_address.to_ast_value(bech32_coder)?,
            },
            Self::AssertWorktopContainsNonFungibles {
                ids,
                resource_address,
            } => ast::Instruction::AssertWorktopContainsNonFungibles {
                ids: ManifestAstValue::Array {
                    // TODO: This was `ManifestAstValueKind::Bucket` by mistake. What kind of test
                    // can we introduce to catch this?
                    element_kind: ManifestAstValueKind::NonFungibleLocalId,
                    elements: ids.clone().into_iter().collect::<Vec<_>>(),
                }
                .to_ast_value(bech32_coder)?,
                resource_address: resource_address.to_ast_value(bech32_coder)?,
            },

            Self::PopFromAuthZone { into_proof } => ast::Instruction::PopFromAuthZone {
                new_proof: into_proof.to_ast_value(bech32_coder)?,
            },
            Self::PushToAuthZone { proof } => ast::Instruction::PushToAuthZone {
                proof: proof.to_ast_value(bech32_coder)?,
            },
            Self::ClearAuthZone => ast::Instruction::ClearAuthZone,

            Self::CreateProofFromAuthZone {
                resource_address,
                into_proof,
            } => ast::Instruction::CreateProofFromAuthZone {
                resource_address: resource_address.to_ast_value(bech32_coder)?,
                new_proof: into_proof.to_ast_value(bech32_coder)?,
            },
            Self::CreateProofFromAuthZoneOfAll {
                resource_address,
                into_proof,
            } => ast::Instruction::CreateProofFromAuthZoneOfAll {
                resource_address: resource_address.to_ast_value(bech32_coder)?,
                new_proof: into_proof.to_ast_value(bech32_coder)?,
            },
            Self::CreateProofFromAuthZoneOfAmount {
                amount,
                resource_address,
                into_proof,
            } => ast::Instruction::CreateProofFromAuthZoneOfAmount {
                amount: amount.to_ast_value(bech32_coder)?,
                resource_address: resource_address.to_ast_value(bech32_coder)?,
                new_proof: into_proof.to_ast_value(bech32_coder)?,
            },
            Self::CreateProofFromAuthZoneOfNonFungibles {
                ids,
                resource_address,
                into_proof,
            } => ast::Instruction::CreateProofFromAuthZoneOfNonFungibles {
                ids: ManifestAstValue::Array {
                    element_kind: ManifestAstValueKind::NonFungibleLocalId,
                    elements: ids.clone().into_iter().collect::<Vec<_>>(),
                }
                .to_ast_value(bech32_coder)?,
                resource_address: resource_address.to_ast_value(bech32_coder)?,
                new_proof: into_proof.to_ast_value(bech32_coder)?,
            },
            Self::CreateProofFromBucket { bucket, into_proof } => {
                ast::Instruction::CreateProofFromBucket {
                    bucket: bucket.to_ast_value(bech32_coder)?,
                    new_proof: into_proof.to_ast_value(bech32_coder)?,
                }
            }
            Self::CreateProofFromBucketOfAll { bucket, into_proof } => {
                ast::Instruction::CreateProofFromBucketOfAll {
                    bucket: bucket.to_ast_value(bech32_coder)?,
                    new_proof: into_proof.to_ast_value(bech32_coder)?,
                }
            }
            Self::CreateProofFromBucketOfAmount {
                amount,
                bucket,
                into_proof,
            } => ast::Instruction::CreateProofFromBucketOfAmount {
                amount: amount.to_ast_value(bech32_coder)?,
                bucket: bucket.to_ast_value(bech32_coder)?,
                new_proof: into_proof.to_ast_value(bech32_coder)?,
            },
            Self::CreateProofFromBucketOfNonFungibles {
                ids,
                bucket,
                into_proof,
            } => ast::Instruction::CreateProofFromBucketOfNonFungibles {
                ids: ManifestAstValue::Array {
                    element_kind: ManifestAstValueKind::NonFungibleLocalId,
                    elements: ids.clone().into_iter().collect::<Vec<_>>(),
                }
                .to_ast_value(bech32_coder)?,
                bucket: bucket.to_ast_value(bech32_coder)?,
                new_proof: into_proof.to_ast_value(bech32_coder)?,
            },

            Self::CloneProof { proof, into_proof } => ast::Instruction::CloneProof {
                proof: proof.to_ast_value(bech32_coder)?,
                new_proof: into_proof.to_ast_value(bech32_coder)?,
            },

            Self::DropProof { proof } => ast::Instruction::DropProof {
                proof: proof.to_ast_value(bech32_coder)?,
            },
            Self::DropAllProofs => ast::Instruction::DropAllProofs,
            Self::ClearSignatureProofs => ast::Instruction::ClearSignatureProofs,
            Self::BurnResource { bucket } => ast::Instruction::BurnResource {
                bucket: bucket.to_ast_value(bech32_coder)?,
            },
            Self::PublishPackage {
                code,
                schema,
                royalty_config,
                metadata,
            } => ast::Instruction::PublishPackage {
                args: vec![
                    code.to_ast_value(bech32_coder)?,
                    package_schema_bytes_to_native_ast(schema, bech32_coder)?,
                    royalty_config.to_ast_value(bech32_coder)?,
                    metadata.to_ast_value(bech32_coder)?,
                ],
            },
            Self::PublishPackageAdvanced {
                code,
                schema,
                royalty_config,
                metadata,
                authority_rules: access_rules,
            } => ast::Instruction::PublishPackageAdvanced {
                args: vec![
                    code.to_ast_value(bech32_coder)?,
                    package_schema_bytes_to_native_ast(schema, bech32_coder)?,
                    royalty_config.to_ast_value(bech32_coder)?,
                    metadata.to_ast_value(bech32_coder)?,
                    access_rules.to_ast_value(bech32_coder)?,
                ],
            },

            Self::RecallResource { vault_id, amount } => ast::Instruction::RecallResource {
                vault_id: vault_id.to_ast_value(bech32_coder)?,
                amount: amount.to_ast_value(bech32_coder)?,
            },

            Self::SetMetadata {
                entity_address,
                key,
                value,
            } => ast::Instruction::SetMetadata {
                address: entity_address.to_ast_value(bech32_coder)?,
                args: vec![
                    key.to_ast_value(bech32_coder)?,
                    value.to_ast_value(bech32_coder)?,
                ],
            },

            Self::RemoveMetadata {
                entity_address,
                key,
            } => ast::Instruction::RemoveMetadata {
                address: entity_address.to_ast_value(bech32_coder)?,
                args: vec![key.to_ast_value(bech32_coder)?],
            },

            Self::SetPackageRoyaltyConfig {
                package_address,
                royalty_config,
            } => ast::Instruction::SetPackageRoyaltyConfig {
                address: package_address.to_ast_value(bech32_coder)?,
                args: vec![royalty_config.to_ast_value(bech32_coder)?],
            },

            Self::SetComponentRoyaltyConfig {
                component_address,
                royalty_config,
            } => ast::Instruction::SetComponentRoyaltyConfig {
                address: component_address.to_ast_value(bech32_coder)?,
                args: vec![royalty_config.to_ast_value(bech32_coder)?],
            },

            Self::ClaimPackageRoyalty { package_address } => {
                ast::Instruction::ClaimPackageRoyalty {
                    address: package_address.to_ast_value(bech32_coder)?,
                    args: vec![],
                }
            }

            Self::ClaimComponentRoyalty { component_address } => {
                ast::Instruction::ClaimComponentRoyalty {
                    address: component_address.to_ast_value(bech32_coder)?,
                    args: vec![],
                }
            }

            Self::SetAuthorityAccessRule {
                entity_address,
                object_key,
                authority_key,
                rule,
            } => ast::Instruction::SetAuthorityAccessRule {
                address: entity_address.to_ast_value(bech32_coder)?,
                args: vec![
                    object_key.to_ast_value(bech32_coder)?,
                    authority_key.to_ast_value(bech32_coder)?,
                    rule.to_ast_value(bech32_coder)?,
                ],
            },

            Self::SetAuthorityMutability {
                entity_address,
                object_key,
                authority_key,
                mutability,
            } => ast::Instruction::SetAuthorityMutability {
                address: entity_address.to_ast_value(bech32_coder)?,
                args: vec![
                    object_key.to_ast_value(bech32_coder)?,
                    authority_key.to_ast_value(bech32_coder)?,
                    mutability.to_ast_value(bech32_coder)?,
                ],
            },

            Self::CreateFungibleResource {
                divisibility,
                metadata,
                access_rules,
            } => ast::Instruction::CreateFungibleResource {
                args: vec![
                    divisibility.to_ast_value(bech32_coder)?,
                    metadata.to_ast_value(bech32_coder)?,
                    access_rules.to_ast_value(bech32_coder)?,
                ],
            },
            Self::CreateFungibleResourceWithInitialSupply {
                divisibility,
                metadata,
                access_rules,
                initial_supply,
            } => ast::Instruction::CreateFungibleResourceWithInitialSupply {
                args: vec![
                    divisibility.to_ast_value(bech32_coder)?,
                    metadata.to_ast_value(bech32_coder)?,
                    access_rules.to_ast_value(bech32_coder)?,
                    initial_supply.to_ast_value(bech32_coder)?,
                ],
            },
            Self::CreateNonFungibleResource {
                id_type,
                schema,
                metadata,
                access_rules,
            } => ast::Instruction::CreateNonFungibleResource {
                args: vec![
                    id_type.to_ast_value(bech32_coder)?,
                    schema.to_ast_value(bech32_coder)?,
                    metadata.to_ast_value(bech32_coder)?,
                    access_rules.to_ast_value(bech32_coder)?,
                ],
            },
            Self::CreateNonFungibleResourceWithInitialSupply {
                id_type,
                schema,
                metadata,
                access_rules,
                initial_supply,
            } => ast::Instruction::CreateNonFungibleResourceWithInitialSupply {
                args: vec![
                    id_type.to_ast_value(bech32_coder)?,
                    schema.to_ast_value(bech32_coder)?,
                    metadata.to_ast_value(bech32_coder)?,
                    access_rules.to_ast_value(bech32_coder)?,
                    initial_supply.to_ast_value(bech32_coder)?,
                ],
            },
            Self::MintFungible {
                resource_address,
                amount,
            } => ast::Instruction::MintFungible {
                address: resource_address.to_ast_value(bech32_coder)?,
                args: vec![amount.to_ast_value(bech32_coder)?],
            },
            Self::MintNonFungible {
                resource_address,
                entries,
            } => ast::Instruction::MintNonFungible {
                address: resource_address.to_ast_value(bech32_coder)?,
                args: vec![entries.to_ast_value(bech32_coder)?],
            },
            Self::MintUuidNonFungible {
                resource_address,
                entries,
            } => ast::Instruction::MintUuidNonFungible {
                address: resource_address.to_ast_value(bech32_coder)?,
                args: vec![entries.to_ast_value(bech32_coder)?],
            },
            Self::CreateAccessController {
                controlled_asset,
                rule_set,
                timed_recovery_delay_in_minutes,
            } => ast::Instruction::CreateAccessController {
                args: vec![
                    controlled_asset.to_ast_value(bech32_coder)?,
                    rule_set.to_ast_value(bech32_coder)?,
                    timed_recovery_delay_in_minutes.to_ast_value(bech32_coder)?,
                ],
            },
            Self::CreateIdentity => ast::Instruction::CreateIdentity { args: vec![] },
            Self::CreateIdentityAdvanced { config } => ast::Instruction::CreateIdentityAdvanced {
                args: vec![config.to_ast_value(bech32_coder)?],
            },
            Self::CreateValidator { key } => ast::Instruction::CreateValidator {
                args: vec![key.to_ast_value(bech32_coder)?],
            },
            Self::CreateAccount {} => ast::Instruction::CreateAccount { args: vec![] },
            Self::CreateAccountAdvanced { config } => ast::Instruction::CreateAccountAdvanced {
                args: vec![config.to_ast_value(bech32_coder)?],
            },
        };
        Ok(ast_instruction)
    }

    pub fn from_ast_instruction(
        ast_instruction: &ast::Instruction,
        bech32_coder: &Bech32Coder,
    ) -> Result<Self, InstructionConversionError> {
        let instruction = match ast_instruction {
            ast::Instruction::CallFunction {
                package_address,
                blueprint_name,
                function_name,
                args,
            } => Self::CallFunction {
                package_address: ManifestAstValue::from_ast_value(package_address, bech32_coder)?,
                blueprint_name: ManifestAstValue::from_ast_value(blueprint_name, bech32_coder)?,
                function_name: ManifestAstValue::from_ast_value(function_name, bech32_coder)?,
                arguments: {
                    args.iter()
                        .map(|v| ManifestAstValue::from_ast_value(v, bech32_coder))
                        .collect::<Result<Vec<_>, _>>()?
                },
            },
            ast::Instruction::CallMethod {
                address,
                method_name,
                args,
            } => Self::CallMethod {
                component_address: ManifestAstValue::from_ast_value(address, bech32_coder)?,
                method_name: ManifestAstValue::from_ast_value(method_name, bech32_coder)?,
                arguments: {
                    args.iter()
                        .map(|v| ManifestAstValue::from_ast_value(v, bech32_coder))
                        .collect::<Result<Vec<_>, _>>()?
                },
            },
            ast::Instruction::CallRoyaltyMethod {
                address,
                method_name,
                args,
            } => Self::CallRoyaltyMethod {
                component_address: ManifestAstValue::from_ast_value(address, bech32_coder)?,
                method_name: ManifestAstValue::from_ast_value(method_name, bech32_coder)?,
                arguments: {
                    args.iter()
                        .map(|v| ManifestAstValue::from_ast_value(v, bech32_coder))
                        .collect::<Result<Vec<_>, _>>()?
                },
            },
            ast::Instruction::CallMetadataMethod {
                address,
                method_name,
                args,
            } => Self::CallMetadataMethod {
                component_address: ManifestAstValue::from_ast_value(address, bech32_coder)?,
                method_name: ManifestAstValue::from_ast_value(method_name, bech32_coder)?,
                arguments: {
                    args.iter()
                        .map(|v| ManifestAstValue::from_ast_value(v, bech32_coder))
                        .collect::<Result<Vec<_>, _>>()?
                },
            },
            ast::Instruction::CallAccessRulesMethod {
                address,
                method_name,
                args,
            } => Self::CallAccessRulesMethod {
                component_address: ManifestAstValue::from_ast_value(address, bech32_coder)?,
                method_name: ManifestAstValue::from_ast_value(method_name, bech32_coder)?,
                arguments: {
                    args.iter()
                        .map(|v| ManifestAstValue::from_ast_value(v, bech32_coder))
                        .collect::<Result<Vec<_>, _>>()?
                },
            },

            ast::Instruction::TakeAllFromWorktop {
                resource_address,
                new_bucket,
            } => Self::TakeAllFromWorktop {
                resource_address: ManifestAstValue::from_ast_value(resource_address, bech32_coder)?,
                into_bucket: ManifestAstValue::from_ast_value(new_bucket, bech32_coder)?,
            },
            ast::Instruction::TakeFromWorktop {
                amount,
                resource_address,
                new_bucket,
            } => Self::TakeFromWorktop {
                amount: ManifestAstValue::from_ast_value(amount, bech32_coder)?,
                resource_address: ManifestAstValue::from_ast_value(resource_address, bech32_coder)?,
                into_bucket: ManifestAstValue::from_ast_value(new_bucket, bech32_coder)?,
            },
            ast::Instruction::TakeNonFungiblesFromWorktop {
                ids,
                resource_address,
                new_bucket,
            } => Self::TakeNonFungiblesFromWorktop {
                ids: if let ManifestAstValue::Array {
                    element_kind: _,
                    elements,
                } = ManifestAstValue::from_ast_value(ids, bech32_coder)?
                {
                    elements.into_iter().collect::<Vec<ManifestAstValue>>()
                } else {
                    panic!("Expected type Array!")
                },
                resource_address: ManifestAstValue::from_ast_value(resource_address, bech32_coder)?,
                into_bucket: ManifestAstValue::from_ast_value(new_bucket, bech32_coder)?,
            },
            ast::Instruction::ReturnToWorktop { bucket } => Self::ReturnToWorktop {
                bucket: ManifestAstValue::from_ast_value(bucket, bech32_coder)?,
            },

            ast::Instruction::AssertWorktopContains {
                resource_address,
                amount,
            } => Self::AssertWorktopContains {
                amount: ManifestAstValue::from_ast_value(amount, bech32_coder)?,
                resource_address: ManifestAstValue::from_ast_value(resource_address, bech32_coder)?,
            },
            ast::Instruction::AssertWorktopContainsNonFungibles {
                ids,
                resource_address,
            } => Self::AssertWorktopContainsNonFungibles {
                ids: if let ManifestAstValue::Array {
                    element_kind: _,
                    elements,
                } = ManifestAstValue::from_ast_value(ids, bech32_coder)?
                {
                    elements.into_iter().collect::<Vec<ManifestAstValue>>()
                } else {
                    panic!("Expected type Array!")
                },
                resource_address: ManifestAstValue::from_ast_value(resource_address, bech32_coder)?,
            },

            ast::Instruction::PopFromAuthZone { new_proof } => Self::PopFromAuthZone {
                into_proof: ManifestAstValue::from_ast_value(new_proof, bech32_coder)?,
            },
            ast::Instruction::PushToAuthZone { proof } => Self::PushToAuthZone {
                proof: ManifestAstValue::from_ast_value(proof, bech32_coder)?,
            },
            ast::Instruction::ClearAuthZone => Self::ClearAuthZone,

            ast::Instruction::CreateProofFromAuthZone {
                resource_address,
                new_proof,
            } => Self::CreateProofFromAuthZone {
                resource_address: ManifestAstValue::from_ast_value(resource_address, bech32_coder)?,
                into_proof: ManifestAstValue::from_ast_value(new_proof, bech32_coder)?,
            },
            ast::Instruction::CreateProofFromAuthZoneOfAll {
                resource_address,
                new_proof,
            } => Self::CreateProofFromAuthZoneOfAll {
                resource_address: ManifestAstValue::from_ast_value(resource_address, bech32_coder)?,
                into_proof: ManifestAstValue::from_ast_value(new_proof, bech32_coder)?,
            },
            ast::Instruction::CreateProofFromAuthZoneOfAmount {
                amount,
                resource_address,
                new_proof,
            } => Self::CreateProofFromAuthZoneOfAmount {
                amount: ManifestAstValue::from_ast_value(amount, bech32_coder)?,
                resource_address: ManifestAstValue::from_ast_value(resource_address, bech32_coder)?,
                into_proof: ManifestAstValue::from_ast_value(new_proof, bech32_coder)?,
            },
            ast::Instruction::CreateProofFromAuthZoneOfNonFungibles {
                ids,
                resource_address,
                new_proof,
            } => Self::CreateProofFromAuthZoneOfNonFungibles {
                ids: if let ManifestAstValue::Array {
                    element_kind: _,
                    elements,
                } = ManifestAstValue::from_ast_value(ids, bech32_coder)?
                {
                    elements.into_iter().collect::<Vec<ManifestAstValue>>()
                } else {
                    panic!("Expected type Array!")
                },
                resource_address: ManifestAstValue::from_ast_value(resource_address, bech32_coder)?,
                into_proof: ManifestAstValue::from_ast_value(new_proof, bech32_coder)?,
            },
            ast::Instruction::CreateProofFromBucket { bucket, new_proof } => {
                Self::CreateProofFromBucket {
                    bucket: ManifestAstValue::from_ast_value(bucket, bech32_coder)?,
                    into_proof: ManifestAstValue::from_ast_value(new_proof, bech32_coder)?,
                }
            }
            ast::Instruction::CreateProofFromBucketOfAll { bucket, new_proof } => {
                Self::CreateProofFromBucketOfAll {
                    bucket: ManifestAstValue::from_ast_value(bucket, bech32_coder)?,
                    into_proof: ManifestAstValue::from_ast_value(new_proof, bech32_coder)?,
                }
            }
            ast::Instruction::CreateProofFromBucketOfAmount {
                amount,
                bucket,
                new_proof,
            } => Self::CreateProofFromBucketOfAmount {
                amount: ManifestAstValue::from_ast_value(amount, bech32_coder)?,
                bucket: ManifestAstValue::from_ast_value(bucket, bech32_coder)?,
                into_proof: ManifestAstValue::from_ast_value(new_proof, bech32_coder)?,
            },
            ast::Instruction::CreateProofFromBucketOfNonFungibles {
                ids,
                bucket,
                new_proof,
            } => Self::CreateProofFromBucketOfNonFungibles {
                ids: if let ManifestAstValue::Array {
                    element_kind: _,
                    elements,
                } = ManifestAstValue::from_ast_value(ids, bech32_coder)?
                {
                    elements.into_iter().collect::<Vec<ManifestAstValue>>()
                } else {
                    panic!("Expected type Array!")
                },
                bucket: ManifestAstValue::from_ast_value(bucket, bech32_coder)?,
                into_proof: ManifestAstValue::from_ast_value(new_proof, bech32_coder)?,
            },

            ast::Instruction::CloneProof { proof, new_proof } => Self::CloneProof {
                proof: ManifestAstValue::from_ast_value(proof, bech32_coder)?,
                into_proof: ManifestAstValue::from_ast_value(new_proof, bech32_coder)?,
            },
            ast::Instruction::DropProof { proof } => Self::DropProof {
                proof: ManifestAstValue::from_ast_value(proof, bech32_coder)?,
            },
            ast::Instruction::DropAllProofs => Self::DropAllProofs,
            ast::Instruction::ClearSignatureProofs => Self::ClearSignatureProofs,
            ast::Instruction::BurnResource { bucket } => Self::BurnResource {
                bucket: ManifestAstValue::from_ast_value(bucket, bech32_coder)?,
            },
            ast::Instruction::PublishPackage { args } => {
                // let (code, schema, royalty_config, metadata) = ;
                let (code, schema, royalty_config, metadata) = unpack!(args);
                Self::PublishPackage {
                    code: ManifestAstValue::from_ast_value(code, bech32_coder)?,
                    schema: package_schema_tuple_to_ret_ast(schema, bech32_coder)?,
                    royalty_config: ManifestAstValue::from_ast_value(royalty_config, bech32_coder)?,
                    metadata: ManifestAstValue::from_ast_value(metadata, bech32_coder)?,
                }
            }
            ast::Instruction::PublishPackageAdvanced { args } => {
                let (code, schema, royalty_config, metadata, authority_rules) = unpack!(args);
                Self::PublishPackageAdvanced {
                    code: ManifestAstValue::from_ast_value(code, bech32_coder)?,
                    schema: package_schema_tuple_to_ret_ast(schema, bech32_coder)?,
                    royalty_config: ManifestAstValue::from_ast_value(royalty_config, bech32_coder)?,
                    metadata: ManifestAstValue::from_ast_value(metadata, bech32_coder)?,
                    authority_rules: ManifestAstValue::from_ast_value(
                        authority_rules,
                        bech32_coder,
                    )?,
                }
            }
            ast::Instruction::RecallResource { vault_id, amount } => Self::RecallResource {
                vault_id: ManifestAstValue::from_ast_value(vault_id, bech32_coder)?,
                amount: ManifestAstValue::from_ast_value(amount, bech32_coder)?,
            },
            ast::Instruction::SetMetadata { address, args } => {
                let (key, value) = unpack!(args);
                Self::SetMetadata {
                    entity_address: ManifestAstValue::from_ast_value(address, bech32_coder)?,
                    key: ManifestAstValue::from_ast_value(key, bech32_coder)?,
                    value: ManifestAstValue::from_ast_value(value, bech32_coder)?,
                }
            }

            ast::Instruction::RemoveMetadata { address, args } => {
                let (key,) = unpack!(args);
                Self::RemoveMetadata {
                    entity_address: ManifestAstValue::from_ast_value(address, bech32_coder)?,
                    key: ManifestAstValue::from_ast_value(key, bech32_coder)?,
                }
            }

            ast::Instruction::SetPackageRoyaltyConfig { address, args } => {
                let (royalty_config,) = unpack!(args);
                Self::SetPackageRoyaltyConfig {
                    package_address: ManifestAstValue::from_ast_value(address, bech32_coder)?,
                    royalty_config: ManifestAstValue::from_ast_value(royalty_config, bech32_coder)?,
                }
            }

            ast::Instruction::SetComponentRoyaltyConfig { address, args } => {
                let (royalty_config,) = unpack!(args);
                Self::SetComponentRoyaltyConfig {
                    component_address: ManifestAstValue::from_ast_value(address, bech32_coder)?,
                    royalty_config: ManifestAstValue::from_ast_value(royalty_config, bech32_coder)?,
                }
            }

            ast::Instruction::ClaimPackageRoyalty { address, .. } => Self::ClaimPackageRoyalty {
                package_address: ManifestAstValue::from_ast_value(address, bech32_coder)?,
            },

            ast::Instruction::ClaimComponentRoyalty { address, .. } => {
                Self::ClaimComponentRoyalty {
                    component_address: ManifestAstValue::from_ast_value(address, bech32_coder)?,
                }
            }

            ast::Instruction::SetAuthorityAccessRule { address, args } => {
                let (object_key, authority_key, rule) = unpack!(args);
                Self::SetAuthorityAccessRule {
                    entity_address: ManifestAstValue::from_ast_value(address, bech32_coder)?,
                    object_key: ManifestAstValue::from_ast_value(object_key, bech32_coder)?,
                    authority_key: ManifestAstValue::from_ast_value(authority_key, bech32_coder)?,
                    rule: ManifestAstValue::from_ast_value(rule, bech32_coder)?,
                }
            }
            ast::Instruction::SetAuthorityMutability { address, args } => {
                let (object_key, authority_key, mutability) = unpack!(args);
                Self::SetAuthorityMutability {
                    entity_address: ManifestAstValue::from_ast_value(address, bech32_coder)?,
                    object_key: ManifestAstValue::from_ast_value(object_key, bech32_coder)?,
                    authority_key: ManifestAstValue::from_ast_value(authority_key, bech32_coder)?,
                    mutability: ManifestAstValue::from_ast_value(mutability, bech32_coder)?,
                }
            }

            ast::Instruction::CreateFungibleResource { args } => {
                let (divisibility, metadata, access_rules) = unpack!(args);
                Self::CreateFungibleResource {
                    divisibility: ManifestAstValue::from_ast_value(divisibility, bech32_coder)?,
                    metadata: ManifestAstValue::from_ast_value(metadata, bech32_coder)?,
                    access_rules: ManifestAstValue::from_ast_value(access_rules, bech32_coder)?,
                }
            }
            ast::Instruction::CreateFungibleResourceWithInitialSupply { args } => {
                let (divisibility, metadata, access_rules, initial_supply) = unpack!(args);
                Self::CreateFungibleResourceWithInitialSupply {
                    divisibility: ManifestAstValue::from_ast_value(divisibility, bech32_coder)?,
                    metadata: ManifestAstValue::from_ast_value(metadata, bech32_coder)?,
                    access_rules: ManifestAstValue::from_ast_value(access_rules, bech32_coder)?,
                    initial_supply: ManifestAstValue::from_ast_value(initial_supply, bech32_coder)?,
                }
            }
            ast::Instruction::CreateNonFungibleResource { args } => {
                let (id_type, schema, metadata, access_rules) = unpack!(args);
                Self::CreateNonFungibleResource {
                    id_type: ManifestAstValue::from_ast_value(id_type, bech32_coder)?,
                    schema: ManifestAstValue::from_ast_value(schema, bech32_coder)?,
                    metadata: ManifestAstValue::from_ast_value(metadata, bech32_coder)?,
                    access_rules: ManifestAstValue::from_ast_value(access_rules, bech32_coder)?,
                }
            }
            ast::Instruction::CreateNonFungibleResourceWithInitialSupply { args } => {
                let (id_type, schema, metadata, access_rules, initial_supply) = unpack!(args);
                Self::CreateNonFungibleResourceWithInitialSupply {
                    id_type: ManifestAstValue::from_ast_value(id_type, bech32_coder)?,
                    schema: ManifestAstValue::from_ast_value(schema, bech32_coder)?,
                    metadata: ManifestAstValue::from_ast_value(metadata, bech32_coder)?,
                    access_rules: ManifestAstValue::from_ast_value(access_rules, bech32_coder)?,
                    initial_supply: ManifestAstValue::from_ast_value(initial_supply, bech32_coder)?,
                }
            }

            ast::Instruction::MintFungible { address, args } => {
                let (amount,) = unpack!(args);
                Self::MintFungible {
                    resource_address: ManifestAstValue::from_ast_value(address, bech32_coder)?,
                    amount: ManifestAstValue::from_ast_value(amount, bech32_coder)?,
                }
            }
            ast::Instruction::MintNonFungible { address, args } => {
                let (entries,) = unpack!(args);
                Self::MintNonFungible {
                    resource_address: ManifestAstValue::from_ast_value(address, bech32_coder)?,
                    entries: ManifestAstValue::from_ast_value(entries, bech32_coder)?,
                }
            }
            ast::Instruction::MintUuidNonFungible { address, args } => {
                let (entries,) = unpack!(args);
                Self::MintUuidNonFungible {
                    resource_address: ManifestAstValue::from_ast_value(address, bech32_coder)?,
                    entries: ManifestAstValue::from_ast_value(entries, bech32_coder)?,
                }
            }

            ast::Instruction::CreateIdentity { .. } => Self::CreateIdentity,
            ast::Instruction::CreateIdentityAdvanced { args } => {
                let (config,) = unpack!(args);
                Self::CreateIdentityAdvanced {
                    config: ManifestAstValue::from_ast_value(config, bech32_coder)?,
                }
            }
            ast::Instruction::CreateAccessController { args } => {
                let (controlled_asset, rule_set, timed_recovery_delay_in_minutes) = unpack!(args);
                Self::CreateAccessController {
                    controlled_asset: ManifestAstValue::from_ast_value(
                        controlled_asset,
                        bech32_coder,
                    )?,
                    rule_set: ManifestAstValue::from_ast_value(rule_set, bech32_coder)?,
                    timed_recovery_delay_in_minutes: ManifestAstValue::from_ast_value(
                        timed_recovery_delay_in_minutes,
                        bech32_coder,
                    )?,
                }
            }
            ast::Instruction::CreateValidator { args } => {
                let (key,) = unpack!(args);
                Self::CreateValidator {
                    key: ManifestAstValue::from_ast_value(key, bech32_coder)?,
                }
            }
            ast::Instruction::CreateAccount { .. } => Self::CreateAccount,
            ast::Instruction::CreateAccountAdvanced { args } => {
                let (config,) = unpack!(args);
                Self::CreateAccountAdvanced {
                    config: ManifestAstValue::from_ast_value(config, bech32_coder)?,
                }
            }
        };
        Ok(instruction)
    }
}

// Called using Instruction::to_ast_instruction
fn package_schema_bytes_to_native_ast(
    value: &ManifestAstValue,
    bech32_coder: &Bech32Coder,
) -> Result<native_transaction::manifest::ast::Value, PackageSchemaResolutionError> {
    // Extract the encoded package schema from the Bytes object
    let encoded_package_schema = if let ManifestAstValue::Bytes { hex: value } = value {
        Ok(value)
    } else {
        Err(PackageSchemaResolutionError::InvalidValueKind {
            expected: ManifestAstValueKind::Bytes,
            actual: value.kind(),
        })
    }?;

    // Decode the bytes as package schema
    let package_schema = manifest_decode::<PackageSchema>(encoded_package_schema)
        .map_err(|_| PackageSchemaResolutionError::FailedToSborDecode)?;

    // Convert the PackageSchema object to a manifest string
    let package_schema_manifest_string =
        utils::manifest_string_representation(&package_schema, bech32_coder);

    // Convert the package schema manifest string to the AST value model
    let ast = {
        let tokens = lexer::tokenize(&package_schema_manifest_string)
            .expect("Impossible case! String created by Scrypto's formatter");
        Parser::new(tokens)
            .parse_value()
            .expect("Impossible case! String created by Scrypto's formatter")
    };
    Ok(ast)
}

// Used during Instruction::from_ast_instruction
fn package_schema_tuple_to_ret_ast(
    value: &native_transaction::manifest::ast::Value,
    bech32_coder: &Bech32Coder,
) -> Result<ManifestAstValue, PackageSchemaResolutionError> {
    let package_schema = {
        let mut resolver = NameResolver::new();

        let manifest_value = generate_value(
            value,
            None,
            &mut resolver,
            bech32_coder.decoder(),
            &Default::default(),
        )
        .map_err(|_| PackageSchemaResolutionError::FailedToGenerateValue)?;

        let encoded = manifest_encode(&manifest_value)
            .map_err(|_| PackageSchemaResolutionError::FailedToSborEncode)?;
        manifest_decode::<PackageSchema>(&encoded)
            .map_err(|_| PackageSchemaResolutionError::FailedToSborDecode)?
    };

    let encoded_package_schema =
        manifest_encode(&package_schema).expect("Impossible case! Decoding succeeded");

    Ok(ManifestAstValue::Bytes {
        hex: encoded_package_schema,
    })
}

macro_rules! unpack {
    ($values: expr) => {
        $values.iter().collect_tuple().map_or(
            Err(InstructionConversionError::TupleConversionError {
                content: format!("{:?}", $values),
            }),
            Ok,
        )?
    };
}
use unpack;
