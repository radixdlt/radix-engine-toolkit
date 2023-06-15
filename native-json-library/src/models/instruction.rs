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

use super::value::*;
use radix_engine::blueprints::package::*;
use radix_engine::types::*;
use radix_engine_common::prelude::*;
use radix_engine_toolkit::utils::*;
use schemars::*;
use scrypto::api::node_modules::auth::*;
use scrypto::api::node_modules::metadata::*;
use scrypto::api::node_modules::royalty::*;
use scrypto::api::ObjectModuleId;
use scrypto::blueprints::access_controller::*;
use scrypto::blueprints::account::*;
use scrypto::blueprints::consensus_manager::*;
use scrypto::blueprints::identity::*;
use serde::*;
use transaction::manifest::generator::NameResolver;
use transaction::prelude::*;
use transaction::validation::*;

#[derive(Serialize, Deserialize, JsonSchema)]
pub enum SerializableInstruction {
    TakeFromWorktop {
        resource_address: SerializableManifestValue,
        amount: SerializableManifestValue,
        new_bucket: SerializableManifestValue,
    },

    TakeNonFungiblesFromWorktop {
        ids: SerializableManifestValue,
        resource_address: SerializableManifestValue,
        new_bucket: SerializableManifestValue,
    },

    TakeAllFromWorktop {
        resource_address: SerializableManifestValue,
        new_bucket: SerializableManifestValue,
    },

    ReturnToWorktop {
        bucket: SerializableManifestValue,
    },

    AssertWorktopContains {
        resource_address: SerializableManifestValue,
        amount: SerializableManifestValue,
    },

    AssertWorktopContainsNonFungibles {
        resource_address: SerializableManifestValue,
        ids: SerializableManifestValue,
    },

    PopFromAuthZone {
        new_proof: SerializableManifestValue,
    },

    PushToAuthZone {
        proof: SerializableManifestValue,
    },

    ClearAuthZone,

    CreateProofFromAuthZone {
        resource_address: SerializableManifestValue,
        new_proof: SerializableManifestValue,
    },

    CreateProofFromAuthZoneOfAmount {
        resource_address: SerializableManifestValue,
        amount: SerializableManifestValue,
        new_proof: SerializableManifestValue,
    },

    CreateProofFromAuthZoneOfNonFungibles {
        resource_address: SerializableManifestValue,
        ids: SerializableManifestValue,
        new_proof: SerializableManifestValue,
    },

    CreateProofFromAuthZoneOfAll {
        resource_address: SerializableManifestValue,
        new_proof: SerializableManifestValue,
    },

    ClearSignatureProofs,

    CreateProofFromBucket {
        bucket: SerializableManifestValue,
        new_proof: SerializableManifestValue,
    },

    CreateProofFromBucketOfAmount {
        bucket: SerializableManifestValue,
        amount: SerializableManifestValue,
        new_proof: SerializableManifestValue,
    },

    CreateProofFromBucketOfNonFungibles {
        bucket: SerializableManifestValue,
        ids: SerializableManifestValue,
        new_proof: SerializableManifestValue,
    },

    CreateProofFromBucketOfAll {
        bucket: SerializableManifestValue,
        new_proof: SerializableManifestValue,
    },

    BurnResource {
        bucket: SerializableManifestValue,
    },

    CloneProof {
        proof: SerializableManifestValue,
        new_proof: SerializableManifestValue,
    },

    DropProof {
        proof: SerializableManifestValue,
    },

    CallFunction {
        package_address: SerializableManifestValue,
        blueprint_name: SerializableManifestValue,
        function_name: SerializableManifestValue,
        args: SerializableManifestValue,
    },

    CallMethod {
        address: SerializableManifestValue,
        method_name: SerializableManifestValue,
        args: SerializableManifestValue,
    },

    CallRoyaltyMethod {
        address: SerializableManifestValue,
        method_name: SerializableManifestValue,
        args: SerializableManifestValue,
    },

    CallMetadataMethod {
        address: SerializableManifestValue,
        method_name: SerializableManifestValue,
        args: SerializableManifestValue,
    },

    CallAccessRulesMethod {
        address: SerializableManifestValue,
        method_name: SerializableManifestValue,
        args: SerializableManifestValue,
    },

    DropAllProofs,

    AllocateGlobalAddress {
        package_address: SerializableManifestValue,
        blueprint_name: SerializableManifestValue,
        address_reservation: SerializableManifestValue,
        named_address: SerializableManifestValue,
    },

    RecallVault {
        vault_id: SerializableManifestValue,
        amount: SerializableManifestValue,
    },
    FreezeVault {
        vault_id: SerializableManifestValue,
    },
    UnfreezeVault {
        vault_id: SerializableManifestValue,
    },

    PublishPackage {
        code: SerializableManifestValue,
        setup: SerializableManifestValue,
        metadata: SerializableManifestValue,
    },
    PublishPackageAdvanced {
        package_address: SerializableManifestValue,
        code: SerializableManifestValue,
        setup: SerializableManifestValue,
        metadata: SerializableManifestValue,
        owner_rule: SerializableManifestValue,
    },
    CreateFungibleResource {
        track_total_supply: SerializableManifestValue,
        divisibility: SerializableManifestValue,
        metadata: SerializableManifestValue,
        access_rules: SerializableManifestValue,
    },
    CreateFungibleResourceWithInitialSupply {
        track_total_supply: SerializableManifestValue,
        divisibility: SerializableManifestValue,
        metadata: SerializableManifestValue,
        access_rules: SerializableManifestValue,
        initial_supply: SerializableManifestValue,
    },
    CreateNonFungibleResource {
        id_type: SerializableManifestValue,
        track_total_supply: SerializableManifestValue,
        non_fungible_schema: SerializableManifestValue,
        metadata: SerializableManifestValue,
        access_rules: SerializableManifestValue,
    },
    CreateNonFungibleResourceWithInitialSupply {
        id_type: SerializableManifestValue,
        track_total_supply: SerializableManifestValue,
        non_fungible_schema: SerializableManifestValue,
        metadata: SerializableManifestValue,
        access_rules: SerializableManifestValue,
        entries: SerializableManifestValue,
    },
    CreateAccessController {
        controlled_asset: SerializableManifestValue,
        rule_set: SerializableManifestValue,
        timed_recovery_delay_in_minutes: SerializableManifestValue,
    },
    CreateIdentity {},
    CreateIdentityAdvanced {
        owner_rule: SerializableManifestValue,
    },
    CreateAccount {},
    CreateAccountAdvanced {
        owner_role: SerializableManifestValue,
    },

    SetMetadata {
        address: SerializableManifestValue,
        key: SerializableManifestValue,
        value: SerializableManifestValue,
    },
    RemoveMetadata {
        address: SerializableManifestValue,
        key: SerializableManifestValue,
    },
    SetComponentRoyaltyConfig {
        address: SerializableManifestValue,
        method: SerializableManifestValue,
        amount: SerializableManifestValue,
    },
    ClaimComponentRoyalty {
        address: SerializableManifestValue,
    },
    UpdateRole {
        address: SerializableManifestValue,
        role_key: SerializableManifestValue,
        rule: SerializableManifestValue,
        mutability: SerializableManifestValue,
    },

    SetPackageRoyaltyConfig {
        address: SerializableManifestValue,
        blueprint: SerializableManifestValue,
        fn_name: SerializableManifestValue,
        royalty: SerializableManifestValue,
    },
    ClaimPackageRoyalty {
        address: SerializableManifestValue,
    },
    MintFungible {
        address: SerializableManifestValue,
        amount: SerializableManifestValue,
    },
    MintNonFungible {
        address: SerializableManifestValue,
        entries: SerializableManifestValue,
    },
    MintUuidNonFungible {
        address: SerializableManifestValue,
        entries: SerializableManifestValue,
    },
    CreateValidator {
        key: SerializableManifestValue,
    },
}

impl SerializableInstruction {
    pub fn from_instruction(
        instruction: &InstructionV1,
        network_id: u8,
        id_allocator: &mut ManifestIdAllocator,
        name_dictionary: &mut NameDictionary,
    ) -> Result<Self, InstructionConversionError> {
        let instruction = match instruction {
            InstructionV1::TakeFromWorktop {
                resource_address,
                amount,
            } => {
                let bucket = id_allocator.new_bucket_id();
                let bucket_name = format!("bucket{}", bucket.0 + 1);
                name_dictionary.add_bucket(&bucket, &bucket_name);

                Self::TakeFromWorktop {
                    resource_address: SerializableManifestValue::from_typed(
                        resource_address,
                        network_id,
                        name_dictionary,
                    )?,
                    amount: SerializableManifestValue::from_typed(
                        amount,
                        network_id,
                        name_dictionary,
                    )?,
                    new_bucket: SerializableManifestValue::from_typed(
                        &bucket,
                        network_id,
                        name_dictionary,
                    )?,
                }
            }
            InstructionV1::TakeNonFungiblesFromWorktop {
                resource_address,
                ids,
            } => {
                let bucket = id_allocator.new_bucket_id();
                let bucket_name = format!("bucket{}", bucket.0 + 1);
                name_dictionary.add_bucket(&bucket, &bucket_name);

                Self::TakeNonFungiblesFromWorktop {
                    ids: SerializableManifestValue::from_typed(ids, network_id, name_dictionary)?,
                    resource_address: SerializableManifestValue::from_typed(
                        resource_address,
                        network_id,
                        name_dictionary,
                    )?,
                    new_bucket: SerializableManifestValue::from_typed(
                        &bucket,
                        network_id,
                        name_dictionary,
                    )?,
                }
            }
            InstructionV1::TakeAllFromWorktop { resource_address } => {
                let bucket = id_allocator.new_bucket_id();
                let bucket_name = format!("bucket{}", bucket.0 + 1);
                name_dictionary.add_bucket(&bucket, &bucket_name);

                Self::TakeAllFromWorktop {
                    resource_address: SerializableManifestValue::from_typed(
                        resource_address,
                        network_id,
                        name_dictionary,
                    )?,
                    new_bucket: SerializableManifestValue::from_typed(
                        &bucket,
                        network_id,
                        name_dictionary,
                    )?,
                }
            }
            InstructionV1::ReturnToWorktop { bucket_id } => Self::ReturnToWorktop {
                bucket: SerializableManifestValue::from_typed(
                    &bucket_id,
                    network_id,
                    name_dictionary,
                )?,
            },
            InstructionV1::AssertWorktopContains {
                resource_address,
                amount,
            } => Self::AssertWorktopContains {
                resource_address: SerializableManifestValue::from_typed(
                    resource_address,
                    network_id,
                    name_dictionary,
                )?,
                amount: SerializableManifestValue::from_typed(amount, network_id, name_dictionary)?,
            },
            InstructionV1::AssertWorktopContainsNonFungibles {
                resource_address,
                ids,
            } => Self::AssertWorktopContainsNonFungibles {
                resource_address: SerializableManifestValue::from_typed(
                    resource_address,
                    network_id,
                    name_dictionary,
                )?,
                ids: SerializableManifestValue::from_typed(ids, network_id, name_dictionary)?,
            },
            InstructionV1::PopFromAuthZone => {
                let proof = id_allocator.new_proof_id();
                let proof_name = format!("proof{}", proof.0 + 1);
                name_dictionary.add_proof(&proof, &proof_name);

                Self::PopFromAuthZone {
                    new_proof: SerializableManifestValue::from_typed(
                        &proof,
                        network_id,
                        name_dictionary,
                    )?,
                }
            }
            InstructionV1::PushToAuthZone { proof_id } => Self::PushToAuthZone {
                proof: SerializableManifestValue::from_typed(
                    &proof_id,
                    network_id,
                    name_dictionary,
                )?,
            },
            InstructionV1::ClearAuthZone => Self::ClearAuthZone,
            InstructionV1::CreateProofFromAuthZone { resource_address } => {
                let proof = id_allocator.new_proof_id();
                let proof_name = format!("proof{}", proof.0 + 1);
                name_dictionary.add_proof(&proof, &proof_name);

                Self::CreateProofFromAuthZone {
                    resource_address: SerializableManifestValue::from_typed(
                        resource_address,
                        network_id,
                        name_dictionary,
                    )?,
                    new_proof: SerializableManifestValue::from_typed(
                        &proof,
                        network_id,
                        name_dictionary,
                    )?,
                }
            }
            InstructionV1::CreateProofFromAuthZoneOfAll { resource_address } => {
                let proof = id_allocator.new_proof_id();
                let proof_name = format!("proof{}", proof.0 + 1);
                name_dictionary.add_proof(&proof, &proof_name);

                Self::CreateProofFromAuthZoneOfAll {
                    resource_address: SerializableManifestValue::from_typed(
                        resource_address,
                        network_id,
                        name_dictionary,
                    )?,
                    new_proof: SerializableManifestValue::from_typed(
                        &proof,
                        network_id,
                        name_dictionary,
                    )?,
                }
            }
            InstructionV1::CreateProofFromAuthZoneOfAmount {
                amount,
                resource_address,
            } => {
                let proof = id_allocator.new_proof_id();
                let proof_name = format!("proof{}", proof.0 + 1);
                name_dictionary.add_proof(&proof, &proof_name);

                Self::CreateProofFromAuthZoneOfAmount {
                    resource_address: SerializableManifestValue::from_typed(
                        resource_address,
                        network_id,
                        name_dictionary,
                    )?,
                    amount: SerializableManifestValue::from_typed(
                        amount,
                        network_id,
                        name_dictionary,
                    )?,
                    new_proof: SerializableManifestValue::from_typed(
                        &proof,
                        network_id,
                        name_dictionary,
                    )?,
                }
            }
            InstructionV1::CreateProofFromAuthZoneOfNonFungibles {
                ids,
                resource_address,
            } => {
                let proof = id_allocator.new_proof_id();
                let proof_name = format!("proof{}", proof.0 + 1);
                name_dictionary.add_proof(&proof, &proof_name);

                Self::CreateProofFromAuthZoneOfNonFungibles {
                    resource_address: SerializableManifestValue::from_typed(
                        resource_address,
                        network_id,
                        name_dictionary,
                    )?,
                    ids: SerializableManifestValue::from_typed(ids, network_id, name_dictionary)?,
                    new_proof: SerializableManifestValue::from_typed(
                        &proof,
                        network_id,
                        name_dictionary,
                    )?,
                }
            }
            InstructionV1::ClearSignatureProofs => Self::ClearSignatureProofs,
            InstructionV1::CreateProofFromBucket { bucket_id } => {
                let proof = id_allocator.new_proof_id();
                let proof_name = format!("proof{}", proof.0 + 1);
                name_dictionary.add_proof(&proof, &proof_name);

                Self::CreateProofFromBucket {
                    bucket: SerializableManifestValue::from_typed(
                        bucket_id,
                        network_id,
                        name_dictionary,
                    )?,
                    new_proof: SerializableManifestValue::from_typed(
                        &proof,
                        network_id,
                        name_dictionary,
                    )?,
                }
            }
            InstructionV1::CreateProofFromBucketOfAll { bucket_id } => {
                let proof = id_allocator.new_proof_id();
                let proof_name = format!("proof{}", proof.0 + 1);
                name_dictionary.add_proof(&proof, &proof_name);

                Self::CreateProofFromBucketOfAll {
                    bucket: SerializableManifestValue::from_typed(
                        bucket_id,
                        network_id,
                        name_dictionary,
                    )?,
                    new_proof: SerializableManifestValue::from_typed(
                        &proof,
                        network_id,
                        name_dictionary,
                    )?,
                }
            }
            InstructionV1::CreateProofFromBucketOfAmount { amount, bucket_id } => {
                let proof = id_allocator.new_proof_id();
                let proof_name = format!("proof{}", proof.0 + 1);
                name_dictionary.add_proof(&proof, &proof_name);

                Self::CreateProofFromBucketOfAmount {
                    bucket: SerializableManifestValue::from_typed(
                        bucket_id,
                        network_id,
                        name_dictionary,
                    )?,
                    amount: SerializableManifestValue::from_typed(
                        amount,
                        network_id,
                        name_dictionary,
                    )?,
                    new_proof: SerializableManifestValue::from_typed(
                        &proof,
                        network_id,
                        name_dictionary,
                    )?,
                }
            }
            InstructionV1::CreateProofFromBucketOfNonFungibles { ids, bucket_id } => {
                let proof = id_allocator.new_proof_id();
                let proof_name = format!("proof{}", proof.0 + 1);
                name_dictionary.add_proof(&proof, &proof_name);

                Self::CreateProofFromBucketOfNonFungibles {
                    bucket: SerializableManifestValue::from_typed(
                        bucket_id,
                        network_id,
                        name_dictionary,
                    )?,
                    ids: SerializableManifestValue::from_typed(ids, network_id, name_dictionary)?,
                    new_proof: SerializableManifestValue::from_typed(
                        &proof,
                        network_id,
                        name_dictionary,
                    )?,
                }
            }
            InstructionV1::BurnResource { bucket_id } => Self::BurnResource {
                bucket: SerializableManifestValue::from_typed(
                    bucket_id,
                    network_id,
                    name_dictionary,
                )?,
            },
            InstructionV1::CloneProof { proof_id } => {
                let proof = id_allocator.new_proof_id();
                let proof_name = format!("proof{}", proof.0 + 1);
                name_dictionary.add_proof(&proof, &proof_name);

                Self::CloneProof {
                    proof: SerializableManifestValue::from_typed(
                        proof_id,
                        network_id,
                        name_dictionary,
                    )?,
                    new_proof: SerializableManifestValue::from_typed(
                        &proof,
                        network_id,
                        name_dictionary,
                    )?,
                }
            }
            InstructionV1::DropProof { proof_id } => Self::DropProof {
                proof: SerializableManifestValue::from_typed(
                    proof_id,
                    network_id,
                    name_dictionary,
                )?,
            },
            InstructionV1::DropAllProofs => Self::DropAllProofs,
            InstructionV1::CallFunction {
                package_address,
                blueprint_name,
                function_name,
                args,
            } => {
                if let DynamicPackageAddress::Static(address) = package_address {
                    if let Some(instruction) = alias_call_function!(
                        address.as_node_id(),
                        blueprint_name,
                        function_name,
                        args,
                        network_id,
                        name_dictionary,
                        [
                            PublishPackageAlias,
                            PublishPackageAdvancedAlias,
                            CreateFungibleResourceAlias,
                            CreateFungibleResourceWithInitialSupplyAlias,
                            CreateNonFungibleResourceAlias,
                            CreateNonFungibleResourceWithInitialSupplyAlias,
                            CreateAccessControllerAlias,
                            CreateIdentityAlias,
                            CreateIdentityAdvancedAlias,
                            CreateAccountAlias,
                            CreateAccountAdvancedAlias,
                        ]
                    ) {
                        instruction
                    } else {
                        Self::CallFunction {
                            package_address: SerializableManifestValue::from_typed(
                                package_address,
                                network_id,
                                name_dictionary,
                            )?,
                            blueprint_name: SerializableManifestValue::from_typed(
                                blueprint_name,
                                network_id,
                                name_dictionary,
                            )?,
                            function_name: SerializableManifestValue::from_typed(
                                function_name,
                                network_id,
                                name_dictionary,
                            )?,
                            args: SerializableManifestValue::from_typed(
                                args,
                                network_id,
                                name_dictionary,
                            )?,
                        }
                    }
                } else {
                    Self::CallFunction {
                        package_address: SerializableManifestValue::from_typed(
                            package_address,
                            network_id,
                            name_dictionary,
                        )?,
                        blueprint_name: SerializableManifestValue::from_typed(
                            blueprint_name,
                            network_id,
                            name_dictionary,
                        )?,
                        function_name: SerializableManifestValue::from_typed(
                            function_name,
                            network_id,
                            name_dictionary,
                        )?,
                        args: SerializableManifestValue::from_typed(
                            args,
                            network_id,
                            name_dictionary,
                        )?,
                    }
                }
            }
            InstructionV1::CallMethod {
                address,
                method_name,
                args,
            } => {
                if let DynamicGlobalAddress::Static(address) = address {
                    if let Some(instruction) = alias_call_method!(
                        address.as_node_id(),
                        ObjectModuleId::Main,
                        method_name,
                        args,
                        network_id,
                        name_dictionary,
                        [
                            SetPackageRoyaltyConfigAlias,
                            ClaimPackageRoyaltyAlias,
                            MintFungibleAlias,
                            MintNonFungibleAlias,
                            MintUuidNonFungibleAlias,
                            CreateValidatorAlias,
                        ]
                    ) {
                        instruction
                    } else {
                        Self::CallMethod {
                            address: SerializableManifestValue::from_typed(
                                address,
                                network_id,
                                name_dictionary,
                            )?,
                            method_name: SerializableManifestValue::from_typed(
                                method_name,
                                network_id,
                                name_dictionary,
                            )?,
                            args: SerializableManifestValue::from_typed(
                                args,
                                network_id,
                                name_dictionary,
                            )?,
                        }
                    }
                } else {
                    Self::CallMethod {
                        address: SerializableManifestValue::from_typed(
                            address,
                            network_id,
                            name_dictionary,
                        )?,
                        method_name: SerializableManifestValue::from_typed(
                            method_name,
                            network_id,
                            name_dictionary,
                        )?,
                        args: SerializableManifestValue::from_typed(
                            args,
                            network_id,
                            name_dictionary,
                        )?,
                    }
                }
            }
            InstructionV1::CallRoyaltyMethod {
                address,
                method_name,
                args,
            } => {
                if let DynamicGlobalAddress::Static(address) = address {
                    if let Some(instruction) = alias_call_method!(
                        address.as_node_id(),
                        ObjectModuleId::Royalty,
                        method_name,
                        args,
                        network_id,
                        name_dictionary,
                        [SetComponentRoyaltyConfigAlias, ClaimComponentRoyaltyAlias]
                    ) {
                        instruction
                    } else {
                        Self::CallRoyaltyMethod {
                            address: SerializableManifestValue::from_typed(
                                address,
                                network_id,
                                name_dictionary,
                            )?,
                            method_name: SerializableManifestValue::from_typed(
                                method_name,
                                network_id,
                                name_dictionary,
                            )?,
                            args: SerializableManifestValue::from_typed(
                                args,
                                network_id,
                                name_dictionary,
                            )?,
                        }
                    }
                } else {
                    Self::CallRoyaltyMethod {
                        address: SerializableManifestValue::from_typed(
                            address,
                            network_id,
                            name_dictionary,
                        )?,
                        method_name: SerializableManifestValue::from_typed(
                            method_name,
                            network_id,
                            name_dictionary,
                        )?,
                        args: SerializableManifestValue::from_typed(
                            args,
                            network_id,
                            name_dictionary,
                        )?,
                    }
                }
            }
            InstructionV1::CallMetadataMethod {
                address,
                method_name,
                args,
            } => {
                if let DynamicGlobalAddress::Static(address) = address {
                    if let Some(instruction) = alias_call_method!(
                        address.as_node_id(),
                        ObjectModuleId::Metadata,
                        method_name,
                        args,
                        network_id,
                        name_dictionary,
                        [SetMetadataAlias, RemoveMetadataAlias]
                    ) {
                        instruction
                    } else {
                        Self::CallMetadataMethod {
                            address: SerializableManifestValue::from_typed(
                                address,
                                network_id,
                                name_dictionary,
                            )?,
                            method_name: SerializableManifestValue::from_typed(
                                method_name,
                                network_id,
                                name_dictionary,
                            )?,
                            args: SerializableManifestValue::from_typed(
                                args,
                                network_id,
                                name_dictionary,
                            )?,
                        }
                    }
                } else {
                    Self::CallMetadataMethod {
                        address: SerializableManifestValue::from_typed(
                            address,
                            network_id,
                            name_dictionary,
                        )?,
                        method_name: SerializableManifestValue::from_typed(
                            method_name,
                            network_id,
                            name_dictionary,
                        )?,
                        args: SerializableManifestValue::from_typed(
                            args,
                            network_id,
                            name_dictionary,
                        )?,
                    }
                }
            }
            InstructionV1::CallAccessRulesMethod {
                address,
                method_name,
                args,
            } => {
                if let DynamicGlobalAddress::Static(address) = address {
                    if let Some(instruction) = alias_call_method!(
                        address.as_node_id(),
                        ObjectModuleId::AccessRules,
                        method_name,
                        args,
                        network_id,
                        name_dictionary,
                        [UpdateRoleAlias]
                    ) {
                        instruction
                    } else {
                        Self::CallAccessRulesMethod {
                            address: SerializableManifestValue::from_typed(
                                address,
                                network_id,
                                name_dictionary,
                            )?,
                            method_name: SerializableManifestValue::from_typed(
                                method_name,
                                network_id,
                                name_dictionary,
                            )?,
                            args: SerializableManifestValue::from_typed(
                                args,
                                network_id,
                                name_dictionary,
                            )?,
                        }
                    }
                } else {
                    Self::CallAccessRulesMethod {
                        address: SerializableManifestValue::from_typed(
                            address,
                            network_id,
                            name_dictionary,
                        )?,
                        method_name: SerializableManifestValue::from_typed(
                            method_name,
                            network_id,
                            name_dictionary,
                        )?,
                        args: SerializableManifestValue::from_typed(
                            args,
                            network_id,
                            name_dictionary,
                        )?,
                    }
                }
            }
            InstructionV1::CallDirectVaultMethod {
                address,
                method_name,
                args,
            } => {
                // TODO: seems to be special cased in Scrypto, why?
                alias_call_method!(
                    address.as_node_id(),
                    ObjectModuleId::Main,
                    method_name,
                    args,
                    network_id,
                    name_dictionary,
                    [FreezeVaultAlias, UnfreezeVaultAlias, RecallVaultAlias]
                )
                .unwrap()
            }
            InstructionV1::AllocateGlobalAddress {
                package_address,
                blueprint_name,
            } => {
                let address_reservation = id_allocator.new_address_reservation_id();
                let address_reservation_name =
                    format!("address_reservation{}", address_reservation.0 + 1);
                name_dictionary
                    .add_address_reservation(&address_reservation, &address_reservation_name);

                let named_address = id_allocator.new_address_id();
                let named_address_name = format!("named_address{}", named_address + 1);
                name_dictionary.add_named_address(named_address, &named_address_name);

                Self::AllocateGlobalAddress {
                    package_address: SerializableManifestValue::from_typed(
                        package_address,
                        network_id,
                        name_dictionary,
                    )?,
                    blueprint_name: SerializableManifestValue::from_typed(
                        blueprint_name,
                        network_id,
                        name_dictionary,
                    )?,
                    address_reservation: SerializableManifestValue::from_typed(
                        &address_reservation,
                        network_id,
                        name_dictionary,
                    )?,
                    named_address: SerializableManifestValue::from_typed(
                        &named_address,
                        network_id,
                        name_dictionary,
                    )?,
                }
            }
        };
        Ok(instruction)
    }

    pub fn to_instruction(
        &self,
        name_resolver: &mut NameResolver,
    ) -> Result<InstructionV1, InstructionConversionError> {
        let instruction = match self {
            Self::TakeFromWorktop {
                resource_address,
                amount,
                ..
            } => InstructionV1::TakeFromWorktop {
                resource_address: resource_address.to_typed(name_resolver)?,
                amount: amount.to_typed(name_resolver)?,
            },
            Self::TakeNonFungiblesFromWorktop {
                ids,
                resource_address,
                ..
            } => InstructionV1::TakeNonFungiblesFromWorktop {
                resource_address: resource_address.to_typed(name_resolver)?,
                ids: ids.to_typed(name_resolver)?,
            },
            Self::TakeAllFromWorktop {
                resource_address, ..
            } => InstructionV1::TakeAllFromWorktop {
                resource_address: resource_address.to_typed(name_resolver)?,
            },
            Self::ReturnToWorktop { bucket } => InstructionV1::ReturnToWorktop {
                bucket_id: bucket.to_typed(name_resolver)?,
            },
            Self::AssertWorktopContains {
                resource_address,
                amount,
            } => InstructionV1::AssertWorktopContains {
                resource_address: resource_address.to_typed(name_resolver)?,
                amount: amount.to_typed(name_resolver)?,
            },
            Self::AssertWorktopContainsNonFungibles {
                resource_address,
                ids,
            } => InstructionV1::AssertWorktopContainsNonFungibles {
                resource_address: resource_address.to_typed(name_resolver)?,
                ids: ids.to_typed(name_resolver)?,
            },
            Self::PopFromAuthZone { .. } => InstructionV1::PopFromAuthZone {},
            Self::PushToAuthZone { proof } => InstructionV1::PushToAuthZone {
                proof_id: proof.to_typed(name_resolver)?,
            },
            Self::ClearAuthZone => InstructionV1::ClearAuthZone,
            Self::ClearSignatureProofs => InstructionV1::ClearSignatureProofs,
            Self::CreateProofFromAuthZone {
                resource_address, ..
            } => InstructionV1::CreateProofFromAuthZone {
                resource_address: resource_address.to_typed(name_resolver)?,
            },
            Self::CreateProofFromAuthZoneOfAll {
                resource_address, ..
            } => InstructionV1::CreateProofFromAuthZoneOfAll {
                resource_address: resource_address.to_typed(name_resolver)?,
            },
            Self::CreateProofFromAuthZoneOfAmount {
                resource_address,
                amount,
                ..
            } => InstructionV1::CreateProofFromAuthZoneOfAmount {
                resource_address: resource_address.to_typed(name_resolver)?,
                amount: amount.to_typed(name_resolver)?,
            },
            Self::CreateProofFromAuthZoneOfNonFungibles {
                resource_address,
                ids,
                ..
            } => InstructionV1::CreateProofFromAuthZoneOfNonFungibles {
                resource_address: resource_address.to_typed(name_resolver)?,
                ids: ids.to_typed(name_resolver)?,
            },
            Self::CreateProofFromBucket { bucket, .. } => InstructionV1::CreateProofFromBucket {
                bucket_id: bucket.to_typed(name_resolver)?,
            },
            Self::CreateProofFromBucketOfAll { bucket, .. } => {
                InstructionV1::CreateProofFromBucketOfAll {
                    bucket_id: bucket.to_typed(name_resolver)?,
                }
            }
            Self::CreateProofFromBucketOfAmount { bucket, amount, .. } => {
                InstructionV1::CreateProofFromBucketOfAmount {
                    bucket_id: bucket.to_typed(name_resolver)?,
                    amount: amount.to_typed(name_resolver)?,
                }
            }
            Self::CreateProofFromBucketOfNonFungibles { bucket, ids, .. } => {
                InstructionV1::CreateProofFromBucketOfNonFungibles {
                    bucket_id: bucket.to_typed(name_resolver)?,
                    ids: ids.to_typed(name_resolver)?,
                }
            }
            Self::BurnResource { bucket } => InstructionV1::BurnResource {
                bucket_id: bucket.to_typed(name_resolver)?,
            },
            Self::CloneProof { proof, .. } => InstructionV1::CloneProof {
                proof_id: proof.to_typed(name_resolver)?,
            },
            Self::DropProof { proof, .. } => InstructionV1::DropProof {
                proof_id: proof.to_typed(name_resolver)?,
            },
            Self::DropAllProofs {} => InstructionV1::DropAllProofs {},
            Self::CallFunction {
                package_address,
                blueprint_name,
                function_name,
                args,
            } => InstructionV1::CallFunction {
                package_address: package_address.to_typed(name_resolver)?,
                blueprint_name: blueprint_name.to_typed(name_resolver)?,
                function_name: function_name.to_typed(name_resolver)?,
                args: args.to_typed(name_resolver)?,
            },
            Self::CallMethod {
                address,
                method_name,
                args,
            } => InstructionV1::CallMethod {
                address: address.to_typed(name_resolver)?,
                method_name: method_name.to_typed(name_resolver)?,
                args: args.to_typed(name_resolver)?,
            },
            Self::CallRoyaltyMethod {
                address,
                method_name,
                args,
            } => InstructionV1::CallRoyaltyMethod {
                address: address.to_typed(name_resolver)?,
                method_name: method_name.to_typed(name_resolver)?,
                args: args.to_typed(name_resolver)?,
            },
            Self::CallMetadataMethod {
                address,
                method_name,
                args,
            } => InstructionV1::CallMetadataMethod {
                address: address.to_typed(name_resolver)?,
                method_name: method_name.to_typed(name_resolver)?,
                args: args.to_typed(name_resolver)?,
            },
            Self::CallAccessRulesMethod {
                address,
                method_name,
                args,
            } => InstructionV1::CallAccessRulesMethod {
                address: address.to_typed(name_resolver)?,
                method_name: method_name.to_typed(name_resolver)?,
                args: args.to_typed(name_resolver)?,
            },
            Self::AllocateGlobalAddress {
                package_address,
                blueprint_name,
                ..
            } => InstructionV1::AllocateGlobalAddress {
                package_address: package_address.to_typed(name_resolver)?,
                blueprint_name: blueprint_name.to_typed(name_resolver)?,
            },
            Self::RecallVault { vault_id, amount } => InstructionV1::CallDirectVaultMethod {
                address: vault_id.to_typed(name_resolver)?,
                method_name: VAULT_RECALL_IDENT.to_string(),
                args: manifest_args!(amount.to_manifest_value(name_resolver)?),
            },
            Self::FreezeVault { vault_id } => InstructionV1::CallDirectVaultMethod {
                address: vault_id.to_typed(name_resolver)?,
                method_name: VAULT_FREEZE_IDENT.to_string(),
                args: manifest_args!(),
            },
            Self::UnfreezeVault { vault_id } => InstructionV1::CallDirectVaultMethod {
                address: vault_id.to_typed(name_resolver)?,
                method_name: VAULT_UNFREEZE_IDENT.to_string(),
                args: manifest_args!(),
            },
            Self::PublishPackage {
                code,
                setup,
                metadata,
            } => InstructionV1::CallFunction {
                package_address: DynamicPackageAddress::Static(PACKAGE_PACKAGE),
                blueprint_name: PACKAGE_BLUEPRINT.to_string(),
                function_name: PACKAGE_PUBLISH_WASM_IDENT.to_string(),
                args: to_manifest_value(&PackagePublishWasmManifestInput {
                    code: code.to_typed(name_resolver)?,
                    metadata: metadata.to_typed(name_resolver)?,
                    setup: setup.to_typed(name_resolver)?,
                }),
            },
            Self::PublishPackageAdvanced {
                code,
                setup,
                metadata,
                owner_rule,
                package_address,
            } => InstructionV1::CallFunction {
                package_address: DynamicPackageAddress::Static(PACKAGE_PACKAGE),
                blueprint_name: PACKAGE_BLUEPRINT.to_string(),
                function_name: PACKAGE_PUBLISH_WASM_ADVANCED_IDENT.to_string(),
                args: to_manifest_value(&PackagePublishWasmAdvancedManifestInput {
                    code: code.to_typed(name_resolver)?,
                    metadata: metadata.to_typed(name_resolver)?,
                    setup: setup.to_typed(name_resolver)?,
                    owner_rule: owner_rule.to_typed(name_resolver)?,
                    package_address: package_address.to_typed(name_resolver)?,
                }),
            },
            Self::CreateFungibleResource {
                access_rules,
                divisibility,
                metadata,
                track_total_supply,
            } => InstructionV1::CallFunction {
                package_address: DynamicPackageAddress::Static(RESOURCE_PACKAGE),
                blueprint_name: FUNGIBLE_RESOURCE_MANAGER_BLUEPRINT.to_string(),
                function_name: FUNGIBLE_RESOURCE_MANAGER_CREATE_IDENT.to_string(),
                args: to_manifest_value(&FungibleResourceManagerCreateInput {
                    access_rules: access_rules.to_typed(name_resolver)?,
                    divisibility: divisibility.to_typed(name_resolver)?,
                    metadata: metadata.to_typed(name_resolver)?,
                    track_total_supply: track_total_supply.to_typed(name_resolver)?,
                }),
            },
            Self::CreateFungibleResourceWithInitialSupply {
                access_rules,
                divisibility,
                metadata,
                track_total_supply,
                initial_supply,
            } => InstructionV1::CallFunction {
                package_address: DynamicPackageAddress::Static(RESOURCE_PACKAGE),
                blueprint_name: FUNGIBLE_RESOURCE_MANAGER_BLUEPRINT.to_string(),
                function_name: FUNGIBLE_RESOURCE_MANAGER_CREATE_WITH_INITIAL_SUPPLY_IDENT
                    .to_string(),
                args: to_manifest_value(&FungibleResourceManagerCreateWithInitialSupplyInput {
                    access_rules: access_rules.to_typed(name_resolver)?,
                    divisibility: divisibility.to_typed(name_resolver)?,
                    metadata: metadata.to_typed(name_resolver)?,
                    track_total_supply: track_total_supply.to_typed(name_resolver)?,
                    initial_supply: initial_supply.to_typed(name_resolver)?,
                }),
            },
            Self::CreateNonFungibleResource {
                access_rules,
                id_type,
                metadata,
                non_fungible_schema,
                track_total_supply,
            } => InstructionV1::CallFunction {
                package_address: DynamicPackageAddress::Static(RESOURCE_PACKAGE),
                blueprint_name: NON_FUNGIBLE_RESOURCE_MANAGER_BLUEPRINT.to_string(),
                function_name: NON_FUNGIBLE_RESOURCE_MANAGER_CREATE_IDENT.to_string(),
                args: to_manifest_value(&NonFungibleResourceManagerCreateInput {
                    access_rules: access_rules.to_typed(name_resolver)?,
                    metadata: metadata.to_typed(name_resolver)?,
                    track_total_supply: track_total_supply.to_typed(name_resolver)?,
                    id_type: id_type.to_typed(name_resolver)?,
                    non_fungible_schema: non_fungible_schema.to_typed(name_resolver)?,
                }),
            },
            Self::CreateNonFungibleResourceWithInitialSupply {
                access_rules,
                id_type,
                metadata,
                non_fungible_schema,
                track_total_supply,
                entries,
            } => InstructionV1::CallFunction {
                package_address: DynamicPackageAddress::Static(RESOURCE_PACKAGE),
                blueprint_name: NON_FUNGIBLE_RESOURCE_MANAGER_BLUEPRINT.to_string(),
                function_name: NON_FUNGIBLE_RESOURCE_MANAGER_CREATE_WITH_INITIAL_SUPPLY_IDENT
                    .to_string(),
                args: to_manifest_value(
                    &NonFungibleResourceManagerCreateWithInitialSupplyManifestInput {
                        access_rules: access_rules.to_typed(name_resolver)?,
                        metadata: metadata.to_typed(name_resolver)?,
                        track_total_supply: track_total_supply.to_typed(name_resolver)?,
                        id_type: id_type.to_typed(name_resolver)?,
                        non_fungible_schema: non_fungible_schema.to_typed(name_resolver)?,
                        entries: entries.to_typed(name_resolver)?,
                    },
                ),
            },
            Self::CreateAccessController {
                controlled_asset,
                rule_set,
                timed_recovery_delay_in_minutes,
            } => InstructionV1::CallFunction {
                package_address: DynamicPackageAddress::Static(ACCESS_CONTROLLER_PACKAGE),
                blueprint_name: ACCESS_CONTROLLER_BLUEPRINT.to_string(),
                function_name: ACCESS_CONTROLLER_CREATE_GLOBAL_IDENT.to_string(),
                args: to_manifest_value(&AccessControllerCreateGlobalManifestInput {
                    controlled_asset: controlled_asset.to_typed(name_resolver)?,
                    rule_set: rule_set.to_typed(name_resolver)?,
                    timed_recovery_delay_in_minutes: timed_recovery_delay_in_minutes
                        .to_typed(name_resolver)?,
                }),
            },
            Self::CreateIdentity {} => InstructionV1::CallFunction {
                package_address: DynamicPackageAddress::Static(IDENTITY_PACKAGE),
                blueprint_name: IDENTITY_BLUEPRINT.to_string(),
                function_name: IDENTITY_CREATE_IDENT.to_string(),
                args: to_manifest_value(&IdentityCreateInput {}),
            },
            Self::CreateIdentityAdvanced { owner_rule } => InstructionV1::CallFunction {
                package_address: DynamicPackageAddress::Static(IDENTITY_PACKAGE),
                blueprint_name: IDENTITY_BLUEPRINT.to_string(),
                function_name: IDENTITY_CREATE_ADVANCED_IDENT.to_string(),
                args: to_manifest_value(&IdentityCreateAdvancedInput {
                    owner_rule: owner_rule.to_typed(name_resolver)?,
                }),
            },
            Self::CreateAccount {} => InstructionV1::CallFunction {
                package_address: DynamicPackageAddress::Static(ACCOUNT_PACKAGE),
                blueprint_name: ACCOUNT_BLUEPRINT.to_string(),
                function_name: ACCOUNT_CREATE_IDENT.to_string(),
                args: to_manifest_value(&AccountCreateInput {}),
            },
            Self::CreateAccountAdvanced { owner_role } => InstructionV1::CallFunction {
                package_address: DynamicPackageAddress::Static(ACCOUNT_PACKAGE),
                blueprint_name: ACCOUNT_BLUEPRINT.to_string(),
                function_name: ACCOUNT_CREATE_ADVANCED_IDENT.to_string(),
                args: to_manifest_value(&AccountCreateAdvancedInput {
                    owner_role: owner_role.to_typed(name_resolver)?,
                }),
            },
            Self::SetMetadata {
                address,
                key,
                value,
            } => InstructionV1::CallMetadataMethod {
                address: address.to_typed(name_resolver)?,
                method_name: METADATA_SET_IDENT.to_string(),
                args: to_manifest_value(&MetadataSetInput {
                    key: key.to_typed(name_resolver)?,
                    value: value.to_typed(name_resolver)?,
                }),
            },
            Self::RemoveMetadata { address, key } => InstructionV1::CallMetadataMethod {
                address: address.to_typed(name_resolver)?,
                method_name: METADATA_REMOVE_IDENT.to_string(),
                args: to_manifest_value(&MetadataRemoveInput {
                    key: key.to_typed(name_resolver)?,
                }),
            },
            Self::SetComponentRoyaltyConfig {
                address,
                method,
                amount,
            } => InstructionV1::CallRoyaltyMethod {
                address: address.to_typed(name_resolver)?,
                method_name: COMPONENT_ROYALTY_SET_ROYALTY_IDENT.to_string(),
                args: to_manifest_value(&ComponentSetRoyaltyInput {
                    method: method.to_typed(name_resolver)?,
                    amount: amount.to_typed(name_resolver)?,
                }),
            },
            Self::ClaimComponentRoyalty { address } => InstructionV1::CallRoyaltyMethod {
                address: address.to_typed(name_resolver)?,
                method_name: COMPONENT_ROYALTY_CLAIM_ROYALTIES_IDENT.to_string(),
                args: to_manifest_value(&ComponentClaimRoyaltiesInput {}),
            },
            Self::UpdateRole {
                address,
                role_key,
                rule,
                mutability,
            } => InstructionV1::CallAccessRulesMethod {
                address: address.to_typed(name_resolver)?,
                method_name: ACCESS_RULES_UPDATE_ROLE_IDENT.to_string(),
                args: to_manifest_value(&AccessRulesUpdateRoleInput {
                    role_key: role_key.to_typed(name_resolver)?,
                    rule: rule.to_typed(name_resolver)?,
                    mutability: mutability.to_typed(name_resolver)?,
                }),
            },
            Self::SetPackageRoyaltyConfig {
                address,
                blueprint,
                fn_name,
                royalty,
            } => InstructionV1::CallMethod {
                address: address.to_typed(name_resolver)?,
                method_name: PACKAGE_SET_ROYALTY_IDENT.to_string(),
                args: to_manifest_value(&PackageSetRoyaltyInput {
                    blueprint: blueprint.to_typed(name_resolver)?,
                    fn_name: fn_name.to_typed(name_resolver)?,
                    royalty: royalty.to_typed(name_resolver)?,
                }),
            },
            Self::ClaimPackageRoyalty { address } => InstructionV1::CallMethod {
                address: address.to_typed(name_resolver)?,
                method_name: PACKAGE_CLAIM_ROYALTIES_IDENT.to_string(),
                args: to_manifest_value(&PackageClaimRoyaltiesInput {}),
            },
            Self::MintFungible { address, amount } => InstructionV1::CallMethod {
                address: address.to_typed(name_resolver)?,
                method_name: FUNGIBLE_RESOURCE_MANAGER_MINT_IDENT.to_string(),
                args: to_manifest_value(&FungibleResourceManagerMintInput {
                    amount: amount.to_typed(name_resolver)?,
                }),
            },
            Self::MintNonFungible { address, entries } => InstructionV1::CallMethod {
                address: address.to_typed(name_resolver)?,
                method_name: NON_FUNGIBLE_RESOURCE_MANAGER_MINT_IDENT.to_string(),
                args: to_manifest_value(&NonFungibleResourceManagerMintManifestInput {
                    entries: entries.to_typed(name_resolver)?,
                }),
            },
            Self::MintUuidNonFungible { address, entries } => InstructionV1::CallMethod {
                address: address.to_typed(name_resolver)?,
                method_name: NON_FUNGIBLE_RESOURCE_MANAGER_MINT_UUID_IDENT.to_string(),
                args: to_manifest_value(&NonFungibleResourceManagerMintUuidManifestInput {
                    entries: entries.to_typed(name_resolver)?,
                }),
            },
            Self::CreateValidator { key } => InstructionV1::CallMethod {
                address: DynamicGlobalAddress::Static(CONSENSUS_MANAGER.into()),
                method_name: CONSENSUS_MANAGER_CREATE_VALIDATOR_IDENT.to_string(),
                args: to_manifest_value(&ConsensusManagerCreateValidatorInput {
                    key: key.to_typed(name_resolver)?,
                }),
            },
        };
        Ok(instruction)
    }
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", content = "error")]
pub enum InstructionConversionError {
    ValueConversionError(ValueConversionError),
}

impl From<ValueConversionError> for InstructionConversionError {
    fn from(value: ValueConversionError) -> Self {
        Self::ValueConversionError(value)
    }
}

trait CallMethodAlias {
    type ScryptoInput: ScryptoDescribe;
    type ManifestInput: ManifestDecode;
    const METHOD_NAME: &'static str;
    const MODULE: ObjectModuleId;

    fn is_valid_address(node_id: &NodeId) -> bool;

    fn handle_aliasing(
        node_id: &NodeId,
        args: &Self::ManifestInput,
        network_id: u8,
        name_dictionary: &NameDictionary,
    ) -> Result<SerializableInstruction, ValueConversionError>;

    fn alias(
        node_id: &NodeId,
        object_module_id: ObjectModuleId,
        method_name: &str,
        args: &ManifestValue,
        network_id: u8,
        name_dictionary: &NameDictionary,
    ) -> Option<SerializableInstruction> {
        if Self::is_valid_address(node_id)
            && object_module_id == Self::MODULE
            && method_name == Self::METHOD_NAME
        {
            let encoded_value = manifest_encode(&args).unwrap();
            let (local_type_index, schema) =
                generate_full_schema_from_single_type::<Self::ScryptoInput, ScryptoCustomSchema>();
            if validate_payload_against_schema::<ManifestCustomExtension, _>(
                &encoded_value,
                &schema,
                local_type_index,
                &(),
            )
            .is_ok()
            {
                Self::handle_aliasing(
                    node_id,
                    &to_manifest_type(args).unwrap(),
                    network_id,
                    name_dictionary,
                )
                .ok()
            } else {
                None
            }
        } else {
            None
        }
    }
}

struct RecallVaultAlias;
impl CallMethodAlias for RecallVaultAlias {
    type ScryptoInput = VaultRecallInput;
    type ManifestInput = VaultRecallInput;
    const METHOD_NAME: &'static str = VAULT_RECALL_IDENT;
    const MODULE: ObjectModuleId = ObjectModuleId::Main;

    fn is_valid_address(node_id: &NodeId) -> bool {
        node_id.is_internal_vault()
    }

    fn handle_aliasing(
        node_id: &NodeId,
        VaultRecallInput { amount }: &VaultRecallInput,
        network_id: u8,
        name_dictionary: &NameDictionary,
    ) -> Result<SerializableInstruction, ValueConversionError> {
        let vault_address = InternalAddress::new_or_panic(node_id.0);
        let instruction = SerializableInstruction::RecallVault {
            vault_id: SerializableManifestValue::from_typed(
                &vault_address,
                network_id,
                name_dictionary,
            )?,
            amount: SerializableManifestValue::from_typed(&amount, network_id, name_dictionary)?,
        };
        Ok(instruction)
    }
}

struct FreezeVaultAlias;
impl CallMethodAlias for FreezeVaultAlias {
    type ScryptoInput = VaultFreezeInput;
    type ManifestInput = VaultFreezeInput;
    const METHOD_NAME: &'static str = VAULT_FREEZE_IDENT;
    const MODULE: ObjectModuleId = ObjectModuleId::Main;

    fn is_valid_address(node_id: &NodeId) -> bool {
        node_id.is_internal_vault()
    }

    fn handle_aliasing(
        node_id: &NodeId,
        VaultFreezeInput {}: &Self::ManifestInput,
        network_id: u8,
        name_dictionary: &NameDictionary,
    ) -> Result<SerializableInstruction, ValueConversionError> {
        let vault_address = InternalAddress::new_or_panic(node_id.0);
        let instruction = SerializableInstruction::FreezeVault {
            vault_id: SerializableManifestValue::from_typed(
                &vault_address,
                network_id,
                name_dictionary,
            )?,
        };
        Ok(instruction)
    }
}

struct UnfreezeVaultAlias;
impl CallMethodAlias for UnfreezeVaultAlias {
    type ScryptoInput = VaultUnfreezeInput;
    type ManifestInput = VaultUnfreezeInput;
    const METHOD_NAME: &'static str = VAULT_UNFREEZE_IDENT;
    const MODULE: ObjectModuleId = ObjectModuleId::Main;

    fn is_valid_address(node_id: &NodeId) -> bool {
        node_id.is_internal_vault()
    }

    fn handle_aliasing(
        node_id: &NodeId,
        VaultUnfreezeInput {}: &Self::ManifestInput,
        network_id: u8,
        name_dictionary: &NameDictionary,
    ) -> Result<SerializableInstruction, ValueConversionError> {
        let vault_address = InternalAddress::new_or_panic(node_id.0);
        let instruction = SerializableInstruction::UnfreezeVault {
            vault_id: SerializableManifestValue::from_typed(
                &vault_address,
                network_id,
                name_dictionary,
            )?,
        };
        Ok(instruction)
    }
}

struct SetMetadataAlias;
impl CallMethodAlias for SetMetadataAlias {
    type ScryptoInput = MetadataSetInput;
    type ManifestInput = MetadataSetInput;
    const METHOD_NAME: &'static str = METADATA_SET_IDENT;
    const MODULE: ObjectModuleId = ObjectModuleId::Metadata;

    fn is_valid_address(node_id: &NodeId) -> bool {
        node_id.is_global()
    }

    fn handle_aliasing(
        node_id: &NodeId,
        MetadataSetInput { key, value }: &Self::ManifestInput,
        network_id: u8,
        name_dictionary: &NameDictionary,
    ) -> Result<SerializableInstruction, ValueConversionError> {
        let address = GlobalAddress::new_or_panic(node_id.0);
        let instruction = SerializableInstruction::SetMetadata {
            address: SerializableManifestValue::from_typed(&address, network_id, name_dictionary)?,
            key: SerializableManifestValue::from_typed(&key, network_id, name_dictionary)?,
            value: SerializableManifestValue::from_typed(&value, network_id, name_dictionary)?,
        };
        Ok(instruction)
    }
}

struct RemoveMetadataAlias;
impl CallMethodAlias for RemoveMetadataAlias {
    type ScryptoInput = MetadataRemoveInput;
    type ManifestInput = MetadataRemoveInput;
    const METHOD_NAME: &'static str = METADATA_REMOVE_IDENT;
    const MODULE: ObjectModuleId = ObjectModuleId::Metadata;

    fn is_valid_address(node_id: &NodeId) -> bool {
        node_id.is_global()
    }

    fn handle_aliasing(
        node_id: &NodeId,
        MetadataRemoveInput { key }: &Self::ManifestInput,
        network_id: u8,
        name_dictionary: &NameDictionary,
    ) -> Result<SerializableInstruction, ValueConversionError> {
        let address = GlobalAddress::new_or_panic(node_id.0);
        let instruction = SerializableInstruction::RemoveMetadata {
            address: SerializableManifestValue::from_typed(&address, network_id, name_dictionary)?,
            key: SerializableManifestValue::from_typed(&key, network_id, name_dictionary)?,
        };
        Ok(instruction)
    }
}

struct SetComponentRoyaltyConfigAlias;
impl CallMethodAlias for SetComponentRoyaltyConfigAlias {
    type ScryptoInput = ComponentSetRoyaltyInput;
    type ManifestInput = ComponentSetRoyaltyInput;
    const METHOD_NAME: &'static str = COMPONENT_ROYALTY_SET_ROYALTY_IDENT;
    const MODULE: ObjectModuleId = ObjectModuleId::Royalty;

    fn is_valid_address(node_id: &NodeId) -> bool {
        node_id.is_global()
    }

    fn handle_aliasing(
        node_id: &NodeId,
        ComponentSetRoyaltyInput { amount, method }: &Self::ManifestInput,
        network_id: u8,
        name_dictionary: &NameDictionary,
    ) -> Result<SerializableInstruction, ValueConversionError> {
        let address = GlobalAddress::new_or_panic(node_id.0);
        let instruction = SerializableInstruction::SetComponentRoyaltyConfig {
            address: SerializableManifestValue::from_typed(&address, network_id, name_dictionary)?,
            method: SerializableManifestValue::from_typed(&method, network_id, name_dictionary)?,
            amount: SerializableManifestValue::from_typed(&amount, network_id, name_dictionary)?,
        };
        Ok(instruction)
    }
}

struct ClaimComponentRoyaltyAlias;
impl CallMethodAlias for ClaimComponentRoyaltyAlias {
    type ScryptoInput = ComponentClaimRoyaltiesInput;
    type ManifestInput = ComponentClaimRoyaltiesInput;
    const METHOD_NAME: &'static str = COMPONENT_ROYALTY_CLAIM_ROYALTIES_IDENT;
    const MODULE: ObjectModuleId = ObjectModuleId::Royalty;

    fn is_valid_address(node_id: &NodeId) -> bool {
        node_id.is_global()
    }

    fn handle_aliasing(
        node_id: &NodeId,
        ComponentClaimRoyaltiesInput {}: &Self::ManifestInput,
        network_id: u8,
        name_dictionary: &NameDictionary,
    ) -> Result<SerializableInstruction, ValueConversionError> {
        let address = GlobalAddress::new_or_panic(node_id.0);
        let instruction = SerializableInstruction::ClaimComponentRoyalty {
            address: SerializableManifestValue::from_typed(&address, network_id, name_dictionary)?,
        };
        Ok(instruction)
    }
}

struct UpdateRoleAlias;
impl CallMethodAlias for UpdateRoleAlias {
    type ScryptoInput = AccessRulesUpdateRoleInput;
    type ManifestInput = AccessRulesUpdateRoleInput;
    const METHOD_NAME: &'static str = ACCESS_RULES_UPDATE_ROLE_IDENT;
    const MODULE: ObjectModuleId = ObjectModuleId::AccessRules;

    fn is_valid_address(node_id: &NodeId) -> bool {
        node_id.is_global()
    }

    fn handle_aliasing(
        node_id: &NodeId,
        AccessRulesUpdateRoleInput {
            rule,
            role_key,
            mutability,
        }: &Self::ManifestInput,
        network_id: u8,
        name_dictionary: &NameDictionary,
    ) -> Result<SerializableInstruction, ValueConversionError> {
        let address = GlobalAddress::new_or_panic(node_id.0);
        let instruction = SerializableInstruction::UpdateRole {
            address: SerializableManifestValue::from_typed(&address, network_id, name_dictionary)?,
            role_key: SerializableManifestValue::from_typed(
                &role_key,
                network_id,
                name_dictionary,
            )?,
            rule: SerializableManifestValue::from_typed(&rule, network_id, name_dictionary)?,
            mutability: SerializableManifestValue::from_typed(
                &mutability,
                network_id,
                name_dictionary,
            )?,
        };
        Ok(instruction)
    }
}

struct SetPackageRoyaltyConfigAlias;
impl CallMethodAlias for SetPackageRoyaltyConfigAlias {
    type ScryptoInput = PackageSetRoyaltyInput;
    type ManifestInput = PackageSetRoyaltyInput;
    const METHOD_NAME: &'static str = PACKAGE_SET_ROYALTY_IDENT;
    const MODULE: ObjectModuleId = ObjectModuleId::Main;

    fn is_valid_address(node_id: &NodeId) -> bool {
        node_id.is_global_package()
    }

    fn handle_aliasing(
        node_id: &NodeId,
        PackageSetRoyaltyInput {
            blueprint,
            fn_name,
            royalty,
        }: &Self::ManifestInput,
        network_id: u8,
        name_dictionary: &NameDictionary,
    ) -> Result<SerializableInstruction, ValueConversionError> {
        let address = GlobalAddress::new_or_panic(node_id.0);
        let instruction = SerializableInstruction::SetPackageRoyaltyConfig {
            address: SerializableManifestValue::from_typed(&address, network_id, name_dictionary)?,
            blueprint: SerializableManifestValue::from_typed(
                &blueprint,
                network_id,
                name_dictionary,
            )?,
            fn_name: SerializableManifestValue::from_typed(&fn_name, network_id, name_dictionary)?,
            royalty: SerializableManifestValue::from_typed(&royalty, network_id, name_dictionary)?,
        };
        Ok(instruction)
    }
}

struct ClaimPackageRoyaltyAlias;
impl CallMethodAlias for ClaimPackageRoyaltyAlias {
    type ScryptoInput = PackageClaimRoyaltiesInput;
    type ManifestInput = PackageClaimRoyaltiesInput;
    const METHOD_NAME: &'static str = PACKAGE_CLAIM_ROYALTIES_IDENT;
    const MODULE: ObjectModuleId = ObjectModuleId::Main;

    fn is_valid_address(node_id: &NodeId) -> bool {
        node_id.is_global_package()
    }

    fn handle_aliasing(
        node_id: &NodeId,
        PackageClaimRoyaltiesInput {}: &Self::ManifestInput,
        network_id: u8,
        name_dictionary: &NameDictionary,
    ) -> Result<SerializableInstruction, ValueConversionError> {
        let address = GlobalAddress::new_or_panic(node_id.0);
        let instruction = SerializableInstruction::ClaimPackageRoyalty {
            address: SerializableManifestValue::from_typed(&address, network_id, name_dictionary)?,
        };
        Ok(instruction)
    }
}

struct MintFungibleAlias;
impl CallMethodAlias for MintFungibleAlias {
    type ScryptoInput = FungibleResourceManagerMintInput;
    type ManifestInput = FungibleResourceManagerMintInput;
    const METHOD_NAME: &'static str = FUNGIBLE_RESOURCE_MANAGER_MINT_IDENT;
    const MODULE: ObjectModuleId = ObjectModuleId::Main;

    fn is_valid_address(node_id: &NodeId) -> bool {
        node_id.is_global_fungible_resource_manager()
    }

    fn handle_aliasing(
        node_id: &NodeId,
        FungibleResourceManagerMintInput { amount }: &Self::ManifestInput,
        network_id: u8,
        name_dictionary: &NameDictionary,
    ) -> Result<SerializableInstruction, ValueConversionError> {
        let address = GlobalAddress::new_or_panic(node_id.0);
        let instruction = SerializableInstruction::MintFungible {
            address: SerializableManifestValue::from_typed(&address, network_id, name_dictionary)?,
            amount: SerializableManifestValue::from_typed(&amount, network_id, name_dictionary)?,
        };
        Ok(instruction)
    }
}

struct MintNonFungibleAlias;
impl CallMethodAlias for MintNonFungibleAlias {
    type ScryptoInput = NonFungibleResourceManagerMintInput;
    type ManifestInput = NonFungibleResourceManagerMintManifestInput;
    const METHOD_NAME: &'static str = NON_FUNGIBLE_RESOURCE_MANAGER_MINT_IDENT;
    const MODULE: ObjectModuleId = ObjectModuleId::Main;

    fn is_valid_address(node_id: &NodeId) -> bool {
        node_id.is_global_non_fungible_resource_manager()
    }

    fn handle_aliasing(
        node_id: &NodeId,
        NonFungibleResourceManagerMintManifestInput { entries }: &Self::ManifestInput,
        network_id: u8,
        name_dictionary: &NameDictionary,
    ) -> Result<SerializableInstruction, ValueConversionError> {
        let address = GlobalAddress::new_or_panic(node_id.0);
        let instruction = SerializableInstruction::MintNonFungible {
            address: SerializableManifestValue::from_typed(&address, network_id, name_dictionary)?,
            entries: SerializableManifestValue::from_typed(&entries, network_id, name_dictionary)?,
        };
        Ok(instruction)
    }
}

struct MintUuidNonFungibleAlias;
impl CallMethodAlias for MintUuidNonFungibleAlias {
    type ScryptoInput = NonFungibleResourceManagerMintUuidInput;
    type ManifestInput = NonFungibleResourceManagerMintUuidManifestInput;
    const METHOD_NAME: &'static str = NON_FUNGIBLE_RESOURCE_MANAGER_MINT_UUID_IDENT;
    const MODULE: ObjectModuleId = ObjectModuleId::Main;

    fn is_valid_address(node_id: &NodeId) -> bool {
        node_id.is_global_non_fungible_resource_manager()
    }

    fn handle_aliasing(
        node_id: &NodeId,
        Self::ManifestInput { entries }: &Self::ManifestInput,
        network_id: u8,
        name_dictionary: &NameDictionary,
    ) -> Result<SerializableInstruction, ValueConversionError> {
        let address = GlobalAddress::new_or_panic(node_id.0);
        let instruction = SerializableInstruction::MintUuidNonFungible {
            address: SerializableManifestValue::from_typed(&address, network_id, name_dictionary)?,
            entries: SerializableManifestValue::from_typed(&entries, network_id, name_dictionary)?,
        };
        Ok(instruction)
    }
}

struct CreateValidatorAlias;
impl CallMethodAlias for CreateValidatorAlias {
    type ScryptoInput = ConsensusManagerCreateValidatorInput;
    type ManifestInput = ConsensusManagerCreateValidatorInput;
    const METHOD_NAME: &'static str = CONSENSUS_MANAGER_CREATE_VALIDATOR_IDENT;
    const MODULE: ObjectModuleId = ObjectModuleId::Main;

    fn is_valid_address(node_id: &NodeId) -> bool {
        node_id.is_global_consensus_manager()
    }

    fn handle_aliasing(
        _: &NodeId,
        Self::ManifestInput { key }: &Self::ManifestInput,
        network_id: u8,
        name_dictionary: &NameDictionary,
    ) -> Result<SerializableInstruction, ValueConversionError> {
        let instruction = SerializableInstruction::CreateValidator {
            key: SerializableManifestValue::from_typed(&key, network_id, name_dictionary)?,
        };
        Ok(instruction)
    }
}

trait CallFunctionAlias {
    type ScryptoInput: ScryptoDescribe;
    type ManifestInput: ManifestDecode;
    const FUNCTION_NAME: &'static str;
    const BLUEPRINT_NAME: &'static str;

    fn is_valid_address(node_id: &NodeId) -> bool;

    fn handle_aliasing(
        node_id: &NodeId,
        args: &Self::ManifestInput,
        network_id: u8,
        name_dictionary: &NameDictionary,
    ) -> Result<SerializableInstruction, ValueConversionError>;

    fn alias(
        node_id: &NodeId,
        blueprint_name: &str,
        function_name: &str,
        args: &ManifestValue,
        network_id: u8,
        name_dictionary: &NameDictionary,
    ) -> Option<SerializableInstruction> {
        if Self::is_valid_address(node_id)
            && blueprint_name == Self::BLUEPRINT_NAME
            && function_name == Self::FUNCTION_NAME
        {
            let encoded_value = manifest_encode(&args).unwrap();
            let (local_type_index, schema) =
                generate_full_schema_from_single_type::<Self::ScryptoInput, ScryptoCustomSchema>();
            if validate_payload_against_schema::<ManifestCustomExtension, _>(
                &encoded_value,
                &schema,
                local_type_index,
                &(),
            )
            .is_ok()
            {
                Self::handle_aliasing(
                    node_id,
                    &to_manifest_type(args).unwrap(),
                    network_id,
                    name_dictionary,
                )
                .ok()
            } else {
                None
            }
        } else {
            None
        }
    }
}

struct PublishPackageAlias;
impl CallFunctionAlias for PublishPackageAlias {
    type ScryptoInput = PackagePublishWasmInput;
    type ManifestInput = PackagePublishWasmInput;
    const FUNCTION_NAME: &'static str = PACKAGE_PUBLISH_WASM_IDENT;
    const BLUEPRINT_NAME: &'static str = PACKAGE_BLUEPRINT;

    fn is_valid_address(node_id: &NodeId) -> bool {
        node_id == PACKAGE_PACKAGE.as_node_id()
    }

    fn handle_aliasing(
        _: &NodeId,
        PackagePublishWasmInput {
            code,
            setup,
            metadata,
        }: &Self::ManifestInput,
        network_id: u8,
        name_dictionary: &NameDictionary,
    ) -> Result<SerializableInstruction, ValueConversionError> {
        let instruction = SerializableInstruction::PublishPackage {
            code: SerializableManifestValue::from_typed(&code, network_id, name_dictionary)?,
            setup: SerializableManifestValue::from_typed(
                &manifest_encode(&setup),
                network_id,
                name_dictionary,
            )?,
            metadata: SerializableManifestValue::from_typed(
                &metadata,
                network_id,
                name_dictionary,
            )?,
        };
        Ok(instruction)
    }
}

struct PublishPackageAdvancedAlias;
impl CallFunctionAlias for PublishPackageAdvancedAlias {
    type ScryptoInput = PackagePublishWasmAdvancedInput;
    type ManifestInput = PackagePublishWasmAdvancedManifestInput;
    const FUNCTION_NAME: &'static str = PACKAGE_PUBLISH_WASM_ADVANCED_IDENT;
    const BLUEPRINT_NAME: &'static str = PACKAGE_BLUEPRINT;

    fn is_valid_address(node_id: &NodeId) -> bool {
        node_id == PACKAGE_PACKAGE.as_node_id()
    }

    fn handle_aliasing(
        _: &NodeId,
        PackagePublishWasmAdvancedManifestInput {
            code,
            metadata,
            owner_rule,
            package_address,
            setup,
        }: &Self::ManifestInput,
        network_id: u8,
        name_dictionary: &NameDictionary,
    ) -> Result<SerializableInstruction, ValueConversionError> {
        let instruction = SerializableInstruction::PublishPackageAdvanced {
            package_address: SerializableManifestValue::from_typed(
                &package_address,
                network_id,
                name_dictionary,
            )?,
            code: SerializableManifestValue::from_typed(&code, network_id, name_dictionary)?,
            setup: SerializableManifestValue::from_typed(
                &manifest_encode(&setup),
                network_id,
                name_dictionary,
            )?,
            metadata: SerializableManifestValue::from_typed(
                &metadata,
                network_id,
                name_dictionary,
            )?,
            owner_rule: SerializableManifestValue::from_typed(
                &owner_rule,
                network_id,
                name_dictionary,
            )?,
        };
        Ok(instruction)
    }
}

struct CreateFungibleResourceAlias;
impl CallFunctionAlias for CreateFungibleResourceAlias {
    type ScryptoInput = FungibleResourceManagerCreateInput;
    type ManifestInput = FungibleResourceManagerCreateInput;
    const FUNCTION_NAME: &'static str = FUNGIBLE_RESOURCE_MANAGER_CREATE_IDENT;
    const BLUEPRINT_NAME: &'static str = FUNGIBLE_RESOURCE_MANAGER_BLUEPRINT;

    fn is_valid_address(node_id: &NodeId) -> bool {
        node_id == RESOURCE_PACKAGE.as_node_id()
    }

    fn handle_aliasing(
        _: &NodeId,
        FungibleResourceManagerCreateInput {
            access_rules,
            divisibility,
            metadata,
            track_total_supply,
        }: &Self::ManifestInput,
        network_id: u8,
        name_dictionary: &NameDictionary,
    ) -> Result<SerializableInstruction, ValueConversionError> {
        let instruction = SerializableInstruction::CreateFungibleResource {
            track_total_supply: SerializableManifestValue::from_typed(
                &track_total_supply,
                network_id,
                name_dictionary,
            )?,
            divisibility: SerializableManifestValue::from_typed(
                &divisibility,
                network_id,
                name_dictionary,
            )?,
            metadata: SerializableManifestValue::from_typed(
                &metadata,
                network_id,
                name_dictionary,
            )?,
            access_rules: SerializableManifestValue::from_typed(
                &access_rules,
                network_id,
                name_dictionary,
            )?,
        };
        Ok(instruction)
    }
}

struct CreateFungibleResourceWithInitialSupplyAlias;
impl CallFunctionAlias for CreateFungibleResourceWithInitialSupplyAlias {
    type ScryptoInput = FungibleResourceManagerCreateWithInitialSupplyInput;
    type ManifestInput = FungibleResourceManagerCreateWithInitialSupplyInput;
    const FUNCTION_NAME: &'static str = FUNGIBLE_RESOURCE_MANAGER_CREATE_WITH_INITIAL_SUPPLY_IDENT;
    const BLUEPRINT_NAME: &'static str = FUNGIBLE_RESOURCE_MANAGER_BLUEPRINT;

    fn is_valid_address(node_id: &NodeId) -> bool {
        node_id == RESOURCE_PACKAGE.as_node_id()
    }

    fn handle_aliasing(
        _: &NodeId,
        FungibleResourceManagerCreateWithInitialSupplyInput {
            access_rules,
            divisibility,
            initial_supply,
            metadata,
            track_total_supply,
        }: &Self::ManifestInput,
        network_id: u8,
        name_dictionary: &NameDictionary,
    ) -> Result<SerializableInstruction, ValueConversionError> {
        let instruction = SerializableInstruction::CreateFungibleResourceWithInitialSupply {
            track_total_supply: SerializableManifestValue::from_typed(
                &track_total_supply,
                network_id,
                name_dictionary,
            )?,
            divisibility: SerializableManifestValue::from_typed(
                &divisibility,
                network_id,
                name_dictionary,
            )?,
            metadata: SerializableManifestValue::from_typed(
                &metadata,
                network_id,
                name_dictionary,
            )?,
            access_rules: SerializableManifestValue::from_typed(
                &access_rules,
                network_id,
                name_dictionary,
            )?,
            initial_supply: SerializableManifestValue::from_typed(
                &initial_supply,
                network_id,
                name_dictionary,
            )?,
        };
        Ok(instruction)
    }
}

struct CreateNonFungibleResourceAlias;
impl CallFunctionAlias for CreateNonFungibleResourceAlias {
    type ScryptoInput = NonFungibleResourceManagerCreateInput;
    type ManifestInput = NonFungibleResourceManagerCreateInput;
    const FUNCTION_NAME: &'static str = NON_FUNGIBLE_RESOURCE_MANAGER_CREATE_IDENT;
    const BLUEPRINT_NAME: &'static str = NON_FUNGIBLE_RESOURCE_MANAGER_BLUEPRINT;

    fn is_valid_address(node_id: &NodeId) -> bool {
        node_id == RESOURCE_PACKAGE.as_node_id()
    }

    fn handle_aliasing(
        _: &NodeId,
        NonFungibleResourceManagerCreateInput {
            access_rules,
            metadata,
            track_total_supply,
            id_type,
            non_fungible_schema,
        }: &Self::ManifestInput,
        network_id: u8,
        name_dictionary: &NameDictionary,
    ) -> Result<SerializableInstruction, ValueConversionError> {
        let instruction = SerializableInstruction::CreateNonFungibleResource {
            track_total_supply: SerializableManifestValue::from_typed(
                &track_total_supply,
                network_id,
                name_dictionary,
            )?,
            id_type: SerializableManifestValue::from_typed(&id_type, network_id, name_dictionary)?,
            metadata: SerializableManifestValue::from_typed(
                &metadata,
                network_id,
                name_dictionary,
            )?,
            access_rules: SerializableManifestValue::from_typed(
                &access_rules,
                network_id,
                name_dictionary,
            )?,
            non_fungible_schema: SerializableManifestValue::from_typed(
                &non_fungible_schema,
                network_id,
                name_dictionary,
            )?,
        };
        Ok(instruction)
    }
}

struct CreateNonFungibleResourceWithInitialSupplyAlias;
impl CallFunctionAlias for CreateNonFungibleResourceWithInitialSupplyAlias {
    type ScryptoInput = NonFungibleResourceManagerCreateWithInitialSupplyInput;
    type ManifestInput = NonFungibleResourceManagerCreateWithInitialSupplyManifestInput;
    const FUNCTION_NAME: &'static str =
        NON_FUNGIBLE_RESOURCE_MANAGER_CREATE_WITH_INITIAL_SUPPLY_IDENT;
    const BLUEPRINT_NAME: &'static str = NON_FUNGIBLE_RESOURCE_MANAGER_BLUEPRINT;

    fn is_valid_address(node_id: &NodeId) -> bool {
        node_id == RESOURCE_PACKAGE.as_node_id()
    }

    fn handle_aliasing(
        _: &NodeId,
        NonFungibleResourceManagerCreateWithInitialSupplyManifestInput {
            access_rules,
            metadata,
            track_total_supply,
            id_type,
            non_fungible_schema,
            entries,
        }: &Self::ManifestInput,
        network_id: u8,
        name_dictionary: &NameDictionary,
    ) -> Result<SerializableInstruction, ValueConversionError> {
        let instruction = SerializableInstruction::CreateNonFungibleResourceWithInitialSupply {
            track_total_supply: SerializableManifestValue::from_typed(
                &track_total_supply,
                network_id,
                name_dictionary,
            )?,
            id_type: SerializableManifestValue::from_typed(&id_type, network_id, name_dictionary)?,
            metadata: SerializableManifestValue::from_typed(
                &metadata,
                network_id,
                name_dictionary,
            )?,
            access_rules: SerializableManifestValue::from_typed(
                &access_rules,
                network_id,
                name_dictionary,
            )?,
            non_fungible_schema: SerializableManifestValue::from_typed(
                &non_fungible_schema,
                network_id,
                name_dictionary,
            )?,
            entries: SerializableManifestValue::from_typed(&entries, network_id, name_dictionary)?,
        };
        Ok(instruction)
    }
}

struct CreateAccessControllerAlias;
impl CallFunctionAlias for CreateAccessControllerAlias {
    type ScryptoInput = AccessControllerCreateGlobalInput;
    type ManifestInput = AccessControllerCreateGlobalManifestInput;
    const FUNCTION_NAME: &'static str = ACCESS_CONTROLLER_CREATE_GLOBAL_IDENT;
    const BLUEPRINT_NAME: &'static str = ACCESS_CONTROLLER_BLUEPRINT;

    fn is_valid_address(node_id: &NodeId) -> bool {
        node_id == ACCESS_CONTROLLER_PACKAGE.as_node_id()
    }

    fn handle_aliasing(
        _: &NodeId,
        AccessControllerCreateGlobalManifestInput {
            controlled_asset,
            rule_set,
            timed_recovery_delay_in_minutes,
        }: &Self::ManifestInput,
        network_id: u8,
        name_dictionary: &NameDictionary,
    ) -> Result<SerializableInstruction, ValueConversionError> {
        let instruction = SerializableInstruction::CreateAccessController {
            controlled_asset: SerializableManifestValue::from_typed(
                &controlled_asset,
                network_id,
                name_dictionary,
            )?,
            rule_set: SerializableManifestValue::from_typed(
                &rule_set,
                network_id,
                name_dictionary,
            )?,
            timed_recovery_delay_in_minutes: SerializableManifestValue::from_typed(
                &timed_recovery_delay_in_minutes,
                network_id,
                name_dictionary,
            )?,
        };
        Ok(instruction)
    }
}

struct CreateIdentityAlias;
impl CallFunctionAlias for CreateIdentityAlias {
    type ScryptoInput = IdentityCreateInput;
    type ManifestInput = IdentityCreateInput;
    const FUNCTION_NAME: &'static str = IDENTITY_CREATE_IDENT;
    const BLUEPRINT_NAME: &'static str = IDENTITY_BLUEPRINT;

    fn is_valid_address(node_id: &NodeId) -> bool {
        node_id == IDENTITY_PACKAGE.as_node_id()
    }

    fn handle_aliasing(
        _: &NodeId,
        IdentityCreateInput {}: &Self::ManifestInput,
        _: u8,
        _: &NameDictionary,
    ) -> Result<SerializableInstruction, ValueConversionError> {
        let instruction = SerializableInstruction::CreateIdentity {};
        Ok(instruction)
    }
}

struct CreateIdentityAdvancedAlias;
impl CallFunctionAlias for CreateIdentityAdvancedAlias {
    type ScryptoInput = IdentityCreateAdvancedInput;
    type ManifestInput = IdentityCreateAdvancedInput;
    const FUNCTION_NAME: &'static str = IDENTITY_CREATE_ADVANCED_IDENT;
    const BLUEPRINT_NAME: &'static str = IDENTITY_BLUEPRINT;

    fn is_valid_address(node_id: &NodeId) -> bool {
        node_id == IDENTITY_PACKAGE.as_node_id()
    }

    fn handle_aliasing(
        _: &NodeId,
        IdentityCreateAdvancedInput { owner_rule }: &Self::ManifestInput,
        network_id: u8,
        name_dictionary: &NameDictionary,
    ) -> Result<SerializableInstruction, ValueConversionError> {
        let instruction = SerializableInstruction::CreateIdentityAdvanced {
            owner_rule: SerializableManifestValue::from_typed(
                &owner_rule,
                network_id,
                name_dictionary,
            )?,
        };
        Ok(instruction)
    }
}

struct CreateAccountAlias;
impl CallFunctionAlias for CreateAccountAlias {
    type ScryptoInput = AccountCreateInput;
    type ManifestInput = AccountCreateInput;
    const FUNCTION_NAME: &'static str = ACCOUNT_CREATE_IDENT;
    const BLUEPRINT_NAME: &'static str = ACCOUNT_BLUEPRINT;

    fn is_valid_address(node_id: &NodeId) -> bool {
        node_id == ACCOUNT_PACKAGE.as_node_id()
    }

    fn handle_aliasing(
        _: &NodeId,
        AccountCreateInput {}: &Self::ManifestInput,
        _: u8,
        _: &NameDictionary,
    ) -> Result<SerializableInstruction, ValueConversionError> {
        let instruction = SerializableInstruction::CreateAccount {};
        Ok(instruction)
    }
}

struct CreateAccountAdvancedAlias;
impl CallFunctionAlias for CreateAccountAdvancedAlias {
    type ScryptoInput = AccountCreateAdvancedInput;
    type ManifestInput = AccountCreateAdvancedInput;
    const FUNCTION_NAME: &'static str = ACCOUNT_CREATE_ADVANCED_IDENT;
    const BLUEPRINT_NAME: &'static str = ACCOUNT_BLUEPRINT;

    fn is_valid_address(node_id: &NodeId) -> bool {
        node_id == ACCOUNT_PACKAGE.as_node_id()
    }

    fn handle_aliasing(
        _: &NodeId,
        AccountCreateAdvancedInput { owner_role }: &Self::ManifestInput,
        network_id: u8,
        name_dictionary: &NameDictionary,
    ) -> Result<SerializableInstruction, ValueConversionError> {
        let instruction = SerializableInstruction::CreateAccountAdvanced {
            owner_role: SerializableManifestValue::from_typed(
                &owner_role,
                network_id,
                name_dictionary,
            )?,
        };
        Ok(instruction)
    }
}

macro_rules! alias_call_method {
    (
        $node_id: expr,
        $object_module_id: expr,
        $method_name: expr,
        $args: expr,
        $network_id: expr,
        $name_dictionary: expr,
        [
            $($aliasing_handler: ty),* $(,)?
        ]
    ) => {
        [
            $(
                <$aliasing_handler>::alias(
                    $node_id,
                    $object_module_id,
                    $method_name,
                    $args,
                    $network_id,
                    $name_dictionary,
                )
            ),*
        ].into_iter().find_map(|instruction| instruction)
    };
}

macro_rules! alias_call_function {
    (
        $node_id: expr,
        $blueprint_name: expr,
        $function_name: expr,
        $args: expr,
        $network_id: expr,
        $name_dictionary: expr,
        [
            $($aliasing_handler: ty),* $(,)?
        ]
    ) => {
        [
            $(
                <$aliasing_handler>::alias(
                    $node_id,
                    $blueprint_name,
                    $function_name,
                    $args,
                    $network_id,
                    $name_dictionary,
                )
            ),*
        ].into_iter().find_map(|instruction| instruction)
    };
}

use alias_call_function;
use alias_call_method;

// TODO: Temporary, add to Scrypto and then remove from here.
#[derive(Debug, Eq, PartialEq, ManifestSbor)]
pub struct AccessControllerCreateGlobalManifestInput {
    pub controlled_asset: ManifestBucket,
    pub rule_set: RuleSet,
    pub timed_recovery_delay_in_minutes: Option<u32>,
}
