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
use native_transaction::manifest::ast;
use native_transaction::manifest::decompiler::format_typed_value;
use native_transaction::manifest::decompiler::DecompilationContext;
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
                    .clone()
                    .unwrap_or_default()
                    .iter()
                    .map(|value| value.to_ast_value(bech32_coder))
                    .collect::<Result<Vec<_>, _>>()?,
            },
            Self::CallMethod {
                component_address,
                method_name,
                arguments,
            } => ast::Instruction::CallMethod {
                component_address: component_address.to_ast_value(bech32_coder)?,
                method_name: method_name.to_ast_value(bech32_coder)?,
                args: arguments
                    .clone()
                    .unwrap_or_default()
                    .iter()
                    .map(|value| value.to_ast_value(bech32_coder))
                    .collect::<Result<Vec<_>, _>>()?,
            },
            Self::TakeFromWorktop {
                resource_address,
                into_bucket,
            } => ast::Instruction::TakeFromWorktop {
                resource_address: resource_address.to_ast_value(bech32_coder)?,
                new_bucket: into_bucket.to_ast_value(bech32_coder)?,
            },
            Self::TakeFromWorktopByAmount {
                amount,
                resource_address,
                into_bucket,
            } => ast::Instruction::TakeFromWorktopByAmount {
                amount: amount.to_ast_value(bech32_coder)?,
                resource_address: resource_address.to_ast_value(bech32_coder)?,
                new_bucket: into_bucket.to_ast_value(bech32_coder)?,
            },
            Self::TakeFromWorktopByIds {
                ids,
                resource_address,
                into_bucket,
            } => ast::Instruction::TakeFromWorktopByIds {
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

            Self::AssertWorktopContains { resource_address } => {
                ast::Instruction::AssertWorktopContains {
                    resource_address: resource_address.to_ast_value(bech32_coder)?,
                }
            }
            Self::AssertWorktopContainsByAmount {
                amount,
                resource_address,
            } => ast::Instruction::AssertWorktopContainsByAmount {
                amount: amount.to_ast_value(bech32_coder)?,
                resource_address: resource_address.to_ast_value(bech32_coder)?,
            },
            Self::AssertWorktopContainsByIds {
                ids,
                resource_address,
            } => ast::Instruction::AssertWorktopContainsByIds {
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
            Self::CreateProofFromAuthZoneByAmount {
                amount,
                resource_address,
                into_proof,
            } => ast::Instruction::CreateProofFromAuthZoneByAmount {
                amount: amount.to_ast_value(bech32_coder)?,
                resource_address: resource_address.to_ast_value(bech32_coder)?,
                new_proof: into_proof.to_ast_value(bech32_coder)?,
            },
            Self::CreateProofFromAuthZoneByIds {
                ids,
                resource_address,
                into_proof,
            } => ast::Instruction::CreateProofFromAuthZoneByIds {
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
                code: code.to_ast_value(bech32_coder)?,
                schema: package_schema_bytes_to_native_ast(schema, bech32_coder)?,
                royalty_config: royalty_config.to_ast_value(bech32_coder)?,
                metadata: metadata.to_ast_value(bech32_coder)?,
            },
            Self::PublishPackageAdvanced {
                code,
                schema,
                royalty_config,
                metadata,
                access_rules,
            } => ast::Instruction::PublishPackageAdvanced {
                code: code.to_ast_value(bech32_coder)?,
                schema: package_schema_bytes_to_native_ast(schema, bech32_coder)?,
                royalty_config: royalty_config.to_ast_value(bech32_coder)?,
                metadata: metadata.to_ast_value(bech32_coder)?,
                access_rules: access_rules.to_ast_value(bech32_coder)?,
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
                entity_address: entity_address.to_ast_value(bech32_coder)?,
                key: key.to_ast_value(bech32_coder)?,
                value: value.to_ast_value(bech32_coder)?,
            },

            Self::RemoveMetadata {
                entity_address,
                key,
            } => ast::Instruction::RemoveMetadata {
                entity_address: entity_address.to_ast_value(bech32_coder)?,
                key: key.to_ast_value(bech32_coder)?,
            },

            Self::SetPackageRoyaltyConfig {
                package_address,
                royalty_config,
            } => ast::Instruction::SetPackageRoyaltyConfig {
                package_address: package_address.to_ast_value(bech32_coder)?,
                royalty_config: royalty_config.to_ast_value(bech32_coder)?,
            },

            Self::SetComponentRoyaltyConfig {
                component_address,
                royalty_config,
            } => ast::Instruction::SetComponentRoyaltyConfig {
                component_address: component_address.to_ast_value(bech32_coder)?,
                royalty_config: royalty_config.to_ast_value(bech32_coder)?,
            },

            Self::ClaimPackageRoyalty { package_address } => {
                ast::Instruction::ClaimPackageRoyalty {
                    package_address: package_address.to_ast_value(bech32_coder)?,
                }
            }

            Self::ClaimComponentRoyalty { component_address } => {
                ast::Instruction::ClaimComponentRoyalty {
                    component_address: component_address.to_ast_value(bech32_coder)?,
                }
            }

            Self::SetMethodAccessRule {
                entity_address,
                key,
                rule,
            } => ast::Instruction::SetMethodAccessRule {
                entity_address: entity_address.to_ast_value(bech32_coder)?,
                key: key.to_ast_value(bech32_coder)?,
                rule: rule.to_ast_value(bech32_coder)?,
            },

            Self::CreateFungibleResource {
                divisibility,
                metadata,
                access_rules,
            } => ast::Instruction::CreateFungibleResource {
                divisibility: divisibility.to_ast_value(bech32_coder)?,
                metadata: metadata.to_ast_value(bech32_coder)?,
                access_rules: access_rules.to_ast_value(bech32_coder)?,
            },
            Self::CreateFungibleResourceWithInitialSupply {
                divisibility,
                metadata,
                access_rules,
                initial_supply,
            } => ast::Instruction::CreateFungibleResourceWithInitialSupply {
                divisibility: divisibility.to_ast_value(bech32_coder)?,
                metadata: metadata.to_ast_value(bech32_coder)?,
                access_rules: access_rules.to_ast_value(bech32_coder)?,
                initial_supply: initial_supply.to_ast_value(bech32_coder)?,
            },
            Self::CreateNonFungibleResource {
                id_type,
                schema,
                metadata,
                access_rules,
            } => ast::Instruction::CreateNonFungibleResource {
                id_type: id_type.to_ast_value(bech32_coder)?,
                schema: schema.to_ast_value(bech32_coder)?,
                metadata: metadata.to_ast_value(bech32_coder)?,
                access_rules: access_rules.to_ast_value(bech32_coder)?,
            },
            Self::CreateNonFungibleResourceWithInitialSupply {
                id_type,
                schema,
                metadata,
                access_rules,
                initial_supply,
            } => ast::Instruction::CreateNonFungibleResourceWithInitialSupply {
                id_type: id_type.to_ast_value(bech32_coder)?,
                schema: schema.to_ast_value(bech32_coder)?,
                metadata: metadata.to_ast_value(bech32_coder)?,
                access_rules: access_rules.to_ast_value(bech32_coder)?,
                initial_supply: initial_supply.to_ast_value(bech32_coder)?,
            },
            Self::MintFungible {
                resource_address,
                amount,
            } => ast::Instruction::MintFungible {
                resource_address: resource_address.to_ast_value(bech32_coder)?,
                amount: amount.to_ast_value(bech32_coder)?,
            },
            Self::MintNonFungible {
                resource_address,
                entries,
            } => ast::Instruction::MintNonFungible {
                resource_address: resource_address.to_ast_value(bech32_coder)?,
                args: entries.to_ast_value(bech32_coder)?,
            },
            Self::MintUuidNonFungible {
                resource_address,
                entries,
            } => ast::Instruction::MintUuidNonFungible {
                resource_address: resource_address.to_ast_value(bech32_coder)?,
                args: entries.to_ast_value(bech32_coder)?,
            },
            Self::CreateAccessController {
                controlled_asset,
                rule_set,
                timed_recovery_delay_in_minutes,
            } => ast::Instruction::CreateAccessController {
                controlled_asset: controlled_asset.to_ast_value(bech32_coder)?,
                rule_set: rule_set.to_ast_value(bech32_coder)?,
                timed_recovery_delay_in_minutes: timed_recovery_delay_in_minutes
                    .to_ast_value(bech32_coder)?,
            },
            Self::CreateIdentity => ast::Instruction::CreateIdentity {},
            Self::CreateIdentityAdvanced { config } => ast::Instruction::CreateIdentityAdvanced {
                config: config.to_ast_value(bech32_coder)?,
            },
            Self::CreateValidator { key } => ast::Instruction::CreateValidator {
                key: key.to_ast_value(bech32_coder)?,
            },
            Self::CreateAccount {} => ast::Instruction::CreateAccount {},
            Self::CreateAccountAdvanced { config } => ast::Instruction::CreateAccountAdvanced {
                config: config.to_ast_value(bech32_coder)?,
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
                    let arguments = args
                        .iter()
                        .map(|v| ManifestAstValue::from_ast_value(v, bech32_coder))
                        .collect::<Result<Vec<_>, _>>()?;
                    match arguments.len() {
                        0 => None,
                        _ => Some(arguments),
                    }
                },
            },
            ast::Instruction::CallMethod {
                component_address,
                method_name,
                args,
            } => Self::CallMethod {
                component_address: ManifestAstValue::from_ast_value(
                    component_address,
                    bech32_coder,
                )?,
                method_name: ManifestAstValue::from_ast_value(method_name, bech32_coder)?,
                arguments: {
                    let arguments = args
                        .iter()
                        .map(|v| ManifestAstValue::from_ast_value(v, bech32_coder))
                        .collect::<Result<Vec<_>, _>>()?;
                    match arguments.len() {
                        0 => None,
                        _ => Some(arguments),
                    }
                },
            },

            ast::Instruction::TakeFromWorktop {
                resource_address,
                new_bucket,
            } => Self::TakeFromWorktop {
                resource_address: ManifestAstValue::from_ast_value(resource_address, bech32_coder)?,
                into_bucket: ManifestAstValue::from_ast_value(new_bucket, bech32_coder)?,
            },
            ast::Instruction::TakeFromWorktopByAmount {
                amount,
                resource_address,
                new_bucket,
            } => Self::TakeFromWorktopByAmount {
                amount: ManifestAstValue::from_ast_value(amount, bech32_coder)?,
                resource_address: ManifestAstValue::from_ast_value(resource_address, bech32_coder)?,
                into_bucket: ManifestAstValue::from_ast_value(new_bucket, bech32_coder)?,
            },
            ast::Instruction::TakeFromWorktopByIds {
                ids,
                resource_address,
                new_bucket,
            } => Self::TakeFromWorktopByIds {
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

            ast::Instruction::AssertWorktopContains { resource_address } => {
                Self::AssertWorktopContains {
                    resource_address: ManifestAstValue::from_ast_value(
                        resource_address,
                        bech32_coder,
                    )?,
                }
            }
            ast::Instruction::AssertWorktopContainsByAmount {
                amount,
                resource_address,
            } => Self::AssertWorktopContainsByAmount {
                amount: ManifestAstValue::from_ast_value(amount, bech32_coder)?,
                resource_address: ManifestAstValue::from_ast_value(resource_address, bech32_coder)?,
            },
            ast::Instruction::AssertWorktopContainsByIds {
                ids,
                resource_address,
            } => Self::AssertWorktopContainsByIds {
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
            ast::Instruction::CreateProofFromAuthZoneByAmount {
                amount,
                resource_address,
                new_proof,
            } => Self::CreateProofFromAuthZoneByAmount {
                amount: ManifestAstValue::from_ast_value(amount, bech32_coder)?,
                resource_address: ManifestAstValue::from_ast_value(resource_address, bech32_coder)?,
                into_proof: ManifestAstValue::from_ast_value(new_proof, bech32_coder)?,
            },
            ast::Instruction::CreateProofFromAuthZoneByIds {
                ids,
                resource_address,
                new_proof,
            } => Self::CreateProofFromAuthZoneByIds {
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
            ast::Instruction::PublishPackage {
                code,
                schema,
                royalty_config,
                metadata,
            } => Self::PublishPackage {
                code: ManifestAstValue::from_ast_value(code, bech32_coder)?,
                schema: package_schema_tuple_to_ret_ast(schema, bech32_coder)?,
                royalty_config: ManifestAstValue::from_ast_value(royalty_config, bech32_coder)?,
                metadata: ManifestAstValue::from_ast_value(metadata, bech32_coder)?,
            },
            ast::Instruction::PublishPackageAdvanced {
                code,
                schema,
                royalty_config,
                metadata,
                access_rules,
            } => Self::PublishPackageAdvanced {
                code: ManifestAstValue::from_ast_value(code, bech32_coder)?,
                schema: package_schema_tuple_to_ret_ast(schema, bech32_coder)?,
                royalty_config: ManifestAstValue::from_ast_value(royalty_config, bech32_coder)?,
                metadata: ManifestAstValue::from_ast_value(metadata, bech32_coder)?,
                access_rules: ManifestAstValue::from_ast_value(access_rules, bech32_coder)?,
            },
            ast::Instruction::RecallResource { vault_id, amount } => Self::RecallResource {
                vault_id: ManifestAstValue::from_ast_value(vault_id, bech32_coder)?,
                amount: ManifestAstValue::from_ast_value(amount, bech32_coder)?,
            },
            ast::Instruction::SetMetadata {
                entity_address,
                key,
                value,
            } => Self::SetMetadata {
                entity_address: ManifestAstValue::from_ast_value(entity_address, bech32_coder)?,
                key: ManifestAstValue::from_ast_value(key, bech32_coder)?,
                value: ManifestAstValue::from_ast_value(value, bech32_coder)?,
            },

            ast::Instruction::RemoveMetadata {
                entity_address,
                key,
            } => Self::RemoveMetadata {
                entity_address: ManifestAstValue::from_ast_value(entity_address, bech32_coder)?,
                key: ManifestAstValue::from_ast_value(key, bech32_coder)?,
            },

            ast::Instruction::SetPackageRoyaltyConfig {
                package_address,
                royalty_config,
            } => Self::SetPackageRoyaltyConfig {
                package_address: ManifestAstValue::from_ast_value(package_address, bech32_coder)?,
                royalty_config: ManifestAstValue::from_ast_value(royalty_config, bech32_coder)?,
            },

            ast::Instruction::SetComponentRoyaltyConfig {
                component_address,
                royalty_config,
            } => Self::SetComponentRoyaltyConfig {
                component_address: ManifestAstValue::from_ast_value(
                    component_address,
                    bech32_coder,
                )?,
                royalty_config: ManifestAstValue::from_ast_value(royalty_config, bech32_coder)?,
            },

            ast::Instruction::ClaimPackageRoyalty { package_address } => {
                Self::ClaimPackageRoyalty {
                    package_address: ManifestAstValue::from_ast_value(
                        package_address,
                        bech32_coder,
                    )?,
                }
            }

            ast::Instruction::ClaimComponentRoyalty { component_address } => {
                Self::ClaimComponentRoyalty {
                    component_address: ManifestAstValue::from_ast_value(
                        component_address,
                        bech32_coder,
                    )?,
                }
            }

            ast::Instruction::SetMethodAccessRule {
                entity_address,
                key,
                rule,
            } => Self::SetMethodAccessRule {
                entity_address: ManifestAstValue::from_ast_value(entity_address, bech32_coder)?,
                key: ManifestAstValue::from_ast_value(key, bech32_coder)?,
                rule: ManifestAstValue::from_ast_value(rule, bech32_coder)?,
            },

            ast::Instruction::CreateFungibleResource {
                divisibility,
                metadata,
                access_rules,
            } => Self::CreateFungibleResource {
                divisibility: ManifestAstValue::from_ast_value(divisibility, bech32_coder)?,
                metadata: ManifestAstValue::from_ast_value(metadata, bech32_coder)?,
                access_rules: ManifestAstValue::from_ast_value(access_rules, bech32_coder)?,
            },
            ast::Instruction::CreateFungibleResourceWithInitialSupply {
                divisibility,
                metadata,
                access_rules,
                initial_supply,
            } => Self::CreateFungibleResourceWithInitialSupply {
                divisibility: ManifestAstValue::from_ast_value(divisibility, bech32_coder)?,
                metadata: ManifestAstValue::from_ast_value(metadata, bech32_coder)?,
                access_rules: ManifestAstValue::from_ast_value(access_rules, bech32_coder)?,
                initial_supply: ManifestAstValue::from_ast_value(initial_supply, bech32_coder)?,
            },
            ast::Instruction::CreateNonFungibleResource {
                id_type,
                schema,
                metadata,
                access_rules,
            } => Self::CreateNonFungibleResource {
                id_type: ManifestAstValue::from_ast_value(id_type, bech32_coder)?,
                schema: ManifestAstValue::from_ast_value(schema, bech32_coder)?,
                metadata: ManifestAstValue::from_ast_value(metadata, bech32_coder)?,
                access_rules: ManifestAstValue::from_ast_value(access_rules, bech32_coder)?,
            },
            ast::Instruction::CreateNonFungibleResourceWithInitialSupply {
                id_type,
                schema,
                metadata,
                access_rules,
                initial_supply,
            } => Self::CreateNonFungibleResourceWithInitialSupply {
                id_type: ManifestAstValue::from_ast_value(id_type, bech32_coder)?,
                schema: ManifestAstValue::from_ast_value(schema, bech32_coder)?,
                metadata: ManifestAstValue::from_ast_value(metadata, bech32_coder)?,
                access_rules: ManifestAstValue::from_ast_value(access_rules, bech32_coder)?,
                initial_supply: ManifestAstValue::from_ast_value(initial_supply, bech32_coder)?,
            },

            ast::Instruction::MintFungible {
                resource_address,
                amount,
            } => Self::MintFungible {
                resource_address: ManifestAstValue::from_ast_value(resource_address, bech32_coder)?,
                amount: ManifestAstValue::from_ast_value(amount, bech32_coder)?,
            },
            ast::Instruction::MintNonFungible {
                resource_address,
                args,
            } => Self::MintNonFungible {
                resource_address: ManifestAstValue::from_ast_value(resource_address, bech32_coder)?,
                entries: ManifestAstValue::from_ast_value(args, bech32_coder)?,
            },
            ast::Instruction::MintUuidNonFungible {
                resource_address,
                args,
            } => Self::MintUuidNonFungible {
                resource_address: ManifestAstValue::from_ast_value(resource_address, bech32_coder)?,
                entries: ManifestAstValue::from_ast_value(args, bech32_coder)?,
            },

            ast::Instruction::CreateIdentity {} => Self::CreateIdentity,
            ast::Instruction::CreateIdentityAdvanced { config } => Self::CreateIdentityAdvanced {
                config: ManifestAstValue::from_ast_value(config, bech32_coder)?,
            },
            ast::Instruction::CreateAccessController {
                controlled_asset,
                rule_set,
                timed_recovery_delay_in_minutes,
            } => Self::CreateAccessController {
                controlled_asset: ManifestAstValue::from_ast_value(controlled_asset, bech32_coder)?,
                rule_set: ManifestAstValue::from_ast_value(rule_set, bech32_coder)?,
                timed_recovery_delay_in_minutes: ManifestAstValue::from_ast_value(
                    timed_recovery_delay_in_minutes,
                    bech32_coder,
                )?,
            },
            ast::Instruction::CreateValidator { key } => Self::CreateValidator {
                key: ManifestAstValue::from_ast_value(key, bech32_coder)?,
            },
            ast::Instruction::CreateAccount {} => Self::CreateAccount,
            ast::Instruction::CreateAccountAdvanced { config } => Self::CreateAccountAdvanced {
                config: ManifestAstValue::from_ast_value(config, bech32_coder)?,
            },
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
    let encoded_package_schema = if let ManifestAstValue::Bytes { value } = value {
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
    let package_schema_manifest_string = {
        let mut string = String::new();
        let mut context =
            DecompilationContext::new_with_optional_network(Some(bech32_coder.encoder()));
        format_typed_value(&mut string, &mut context, &package_schema)
            .expect("Impossible case! Valid SBOR can't fail here");
        string
    };

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
        value: encoded_package_schema,
    })
}
