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

use crate::prelude::*;

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
use transaction::prelude::*;
use transaction::validation::*;

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
#[serde(tag = "kind")]
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
        fee_factor: SerializableManifestValue,
    },
}

impl SerializableInstruction {
    pub fn from_instruction(
        instruction: &InstructionV1,
        network_id: u8,
        id_allocator: &mut ManifestIdAllocator,
    ) -> Result<Self, InstructionConversionError> {
        let instruction = match instruction {
            InstructionV1::TakeFromWorktop {
                resource_address,
                amount,
            } => {
                let bucket = id_allocator.new_bucket_id();

                Self::TakeFromWorktop {
                    resource_address: SerializableManifestValue::from_typed(
                        resource_address,
                        network_id,
                    )?,
                    amount: SerializableManifestValue::from_typed(amount, network_id)?,
                    new_bucket: SerializableManifestValue::from_typed(&bucket, network_id)?,
                }
            }
            InstructionV1::TakeNonFungiblesFromWorktop {
                resource_address,
                ids,
            } => {
                let bucket = id_allocator.new_bucket_id();

                Self::TakeNonFungiblesFromWorktop {
                    ids: SerializableManifestValue::from_typed(ids, network_id)?,
                    resource_address: SerializableManifestValue::from_typed(
                        resource_address,
                        network_id,
                    )?,
                    new_bucket: SerializableManifestValue::from_typed(&bucket, network_id)?,
                }
            }
            InstructionV1::TakeAllFromWorktop { resource_address } => {
                let bucket = id_allocator.new_bucket_id();

                Self::TakeAllFromWorktop {
                    resource_address: SerializableManifestValue::from_typed(
                        resource_address,
                        network_id,
                    )?,
                    new_bucket: SerializableManifestValue::from_typed(&bucket, network_id)?,
                }
            }
            InstructionV1::ReturnToWorktop { bucket_id } => Self::ReturnToWorktop {
                bucket: SerializableManifestValue::from_typed(&bucket_id, network_id)?,
            },
            InstructionV1::AssertWorktopContains {
                resource_address,
                amount,
            } => Self::AssertWorktopContains {
                resource_address: SerializableManifestValue::from_typed(
                    resource_address,
                    network_id,
                )?,
                amount: SerializableManifestValue::from_typed(amount, network_id)?,
            },
            InstructionV1::AssertWorktopContainsNonFungibles {
                resource_address,
                ids,
            } => Self::AssertWorktopContainsNonFungibles {
                resource_address: SerializableManifestValue::from_typed(
                    resource_address,
                    network_id,
                )?,
                ids: SerializableManifestValue::from_typed(ids, network_id)?,
            },
            InstructionV1::PopFromAuthZone => {
                let proof = id_allocator.new_proof_id();

                Self::PopFromAuthZone {
                    new_proof: SerializableManifestValue::from_typed(&proof, network_id)?,
                }
            }
            InstructionV1::PushToAuthZone { proof_id } => Self::PushToAuthZone {
                proof: SerializableManifestValue::from_typed(&proof_id, network_id)?,
            },
            InstructionV1::ClearAuthZone => Self::ClearAuthZone,
            InstructionV1::CreateProofFromAuthZone { resource_address } => {
                let proof = id_allocator.new_proof_id();

                Self::CreateProofFromAuthZone {
                    resource_address: SerializableManifestValue::from_typed(
                        resource_address,
                        network_id,
                    )?,
                    new_proof: SerializableManifestValue::from_typed(&proof, network_id)?,
                }
            }
            InstructionV1::CreateProofFromAuthZoneOfAll { resource_address } => {
                let proof = id_allocator.new_proof_id();

                Self::CreateProofFromAuthZoneOfAll {
                    resource_address: SerializableManifestValue::from_typed(
                        resource_address,
                        network_id,
                    )?,
                    new_proof: SerializableManifestValue::from_typed(&proof, network_id)?,
                }
            }
            InstructionV1::CreateProofFromAuthZoneOfAmount {
                amount,
                resource_address,
            } => {
                let proof = id_allocator.new_proof_id();

                Self::CreateProofFromAuthZoneOfAmount {
                    resource_address: SerializableManifestValue::from_typed(
                        resource_address,
                        network_id,
                    )?,
                    amount: SerializableManifestValue::from_typed(amount, network_id)?,
                    new_proof: SerializableManifestValue::from_typed(&proof, network_id)?,
                }
            }
            InstructionV1::CreateProofFromAuthZoneOfNonFungibles {
                ids,
                resource_address,
            } => {
                let proof = id_allocator.new_proof_id();

                Self::CreateProofFromAuthZoneOfNonFungibles {
                    resource_address: SerializableManifestValue::from_typed(
                        resource_address,
                        network_id,
                    )?,
                    ids: SerializableManifestValue::from_typed(ids, network_id)?,
                    new_proof: SerializableManifestValue::from_typed(&proof, network_id)?,
                }
            }
            InstructionV1::ClearSignatureProofs => Self::ClearSignatureProofs,
            InstructionV1::CreateProofFromBucket { bucket_id } => {
                let proof = id_allocator.new_proof_id();

                Self::CreateProofFromBucket {
                    bucket: SerializableManifestValue::from_typed(bucket_id, network_id)?,
                    new_proof: SerializableManifestValue::from_typed(&proof, network_id)?,
                }
            }
            InstructionV1::CreateProofFromBucketOfAll { bucket_id } => {
                let proof = id_allocator.new_proof_id();

                Self::CreateProofFromBucketOfAll {
                    bucket: SerializableManifestValue::from_typed(bucket_id, network_id)?,
                    new_proof: SerializableManifestValue::from_typed(&proof, network_id)?,
                }
            }
            InstructionV1::CreateProofFromBucketOfAmount { amount, bucket_id } => {
                let proof = id_allocator.new_proof_id();

                Self::CreateProofFromBucketOfAmount {
                    bucket: SerializableManifestValue::from_typed(bucket_id, network_id)?,
                    amount: SerializableManifestValue::from_typed(amount, network_id)?,
                    new_proof: SerializableManifestValue::from_typed(&proof, network_id)?,
                }
            }
            InstructionV1::CreateProofFromBucketOfNonFungibles { ids, bucket_id } => {
                let proof = id_allocator.new_proof_id();

                Self::CreateProofFromBucketOfNonFungibles {
                    bucket: SerializableManifestValue::from_typed(bucket_id, network_id)?,
                    ids: SerializableManifestValue::from_typed(ids, network_id)?,
                    new_proof: SerializableManifestValue::from_typed(&proof, network_id)?,
                }
            }
            InstructionV1::BurnResource { bucket_id } => Self::BurnResource {
                bucket: SerializableManifestValue::from_typed(bucket_id, network_id)?,
            },
            InstructionV1::CloneProof { proof_id } => {
                let proof = id_allocator.new_proof_id();

                Self::CloneProof {
                    proof: SerializableManifestValue::from_typed(proof_id, network_id)?,
                    new_proof: SerializableManifestValue::from_typed(&proof, network_id)?,
                }
            }
            InstructionV1::DropProof { proof_id } => Self::DropProof {
                proof: SerializableManifestValue::from_typed(proof_id, network_id)?,
            },
            InstructionV1::DropAllProofs => Self::DropAllProofs,
            InstructionV1::CallFunction {
                package_address,
                blueprint_name,
                function_name,
                args,
            } => {
                let instruction = if let DynamicPackageAddress::Static(address) = package_address {
                    alias_call_function!(
                        address.as_node_id(),
                        blueprint_name,
                        function_name,
                        args,
                        network_id,
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
                    )
                } else {
                    None
                };

                if let Some(instruction) = instruction {
                    instruction
                } else {
                    Self::CallFunction {
                        package_address: SerializableManifestValue::from_typed(
                            package_address,
                            network_id,
                        )?,
                        blueprint_name: SerializableManifestValue::from_typed(
                            blueprint_name,
                            network_id,
                        )?,
                        function_name: SerializableManifestValue::from_typed(
                            function_name,
                            network_id,
                        )?,
                        args: SerializableManifestValue::from_typed(args, network_id)?,
                    }
                }
            }
            InstructionV1::CallMethod {
                address,
                method_name,
                args,
            } => {
                let instruction = if let DynamicGlobalAddress::Static(address) = address {
                    alias_call_method!(
                        address.as_node_id(),
                        ObjectModuleId::Main,
                        method_name,
                        args,
                        network_id,
                        [
                            ClaimPackageRoyaltyAlias,
                            MintFungibleAlias,
                            MintNonFungibleAlias,
                            MintUuidNonFungibleAlias,
                            CreateValidatorAlias,
                        ]
                    )
                } else {
                    None
                };

                if let Some(instruction) = instruction {
                    instruction
                } else {
                    Self::CallMethod {
                        address: SerializableManifestValue::from_typed(address, network_id)?,
                        method_name: SerializableManifestValue::from_typed(
                            method_name,
                            network_id,
                        )?,
                        args: SerializableManifestValue::from_typed(args, network_id)?,
                    }
                }
            }
            InstructionV1::CallRoyaltyMethod {
                address,
                method_name,
                args,
            } => {
                let instruction = if let DynamicGlobalAddress::Static(address) = address {
                    alias_call_method!(
                        address.as_node_id(),
                        ObjectModuleId::Royalty,
                        method_name,
                        args,
                        network_id,
                        [SetComponentRoyaltyConfigAlias, ClaimComponentRoyaltyAlias]
                    )
                } else {
                    None
                };

                if let Some(instruction) = instruction {
                    instruction
                } else {
                    Self::CallRoyaltyMethod {
                        address: SerializableManifestValue::from_typed(address, network_id)?,
                        method_name: SerializableManifestValue::from_typed(
                            method_name,
                            network_id,
                        )?,
                        args: SerializableManifestValue::from_typed(args, network_id)?,
                    }
                }
            }
            InstructionV1::CallMetadataMethod {
                address,
                method_name,
                args,
            } => {
                let instruction = if let DynamicGlobalAddress::Static(address) = address {
                    alias_call_method!(
                        address.as_node_id(),
                        ObjectModuleId::Metadata,
                        method_name,
                        args,
                        network_id,
                        [SetMetadataAlias, RemoveMetadataAlias]
                    )
                } else {
                    None
                };

                if let Some(instruction) = instruction {
                    instruction
                } else {
                    Self::CallMetadataMethod {
                        address: SerializableManifestValue::from_typed(address, network_id)?,
                        method_name: SerializableManifestValue::from_typed(
                            method_name,
                            network_id,
                        )?,
                        args: SerializableManifestValue::from_typed(args, network_id)?,
                    }
                }
            }
            InstructionV1::CallAccessRulesMethod {
                address,
                method_name,
                args,
            } => {
                let instruction = if let DynamicGlobalAddress::Static(address) = address {
                    alias_call_method!(
                        address.as_node_id(),
                        ObjectModuleId::Metadata,
                        method_name,
                        args,
                        network_id,
                        [UpdateRoleAlias]
                    )
                } else {
                    None
                };

                if let Some(instruction) = instruction {
                    instruction
                } else {
                    Self::CallAccessRulesMethod {
                        address: SerializableManifestValue::from_typed(address, network_id)?,
                        method_name: SerializableManifestValue::from_typed(
                            method_name,
                            network_id,
                        )?,
                        args: SerializableManifestValue::from_typed(args, network_id)?,
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
                    [FreezeVaultAlias, UnfreezeVaultAlias, RecallVaultAlias]
                )
                .unwrap()
            }
            InstructionV1::AllocateGlobalAddress {
                package_address,
                blueprint_name,
            } => {
                let address_reservation = id_allocator.new_address_reservation_id();
                let named_address = id_allocator.new_address_id();

                Self::AllocateGlobalAddress {
                    package_address: SerializableManifestValue::from_typed(
                        package_address,
                        network_id,
                    )?,
                    blueprint_name: SerializableManifestValue::from_typed(
                        blueprint_name,
                        network_id,
                    )?,
                    address_reservation: SerializableManifestValue::from_typed(
                        &address_reservation,
                        network_id,
                    )?,
                    named_address: SerializableManifestValue::from_typed(
                        &named_address,
                        network_id,
                    )?,
                }
            }
        };
        Ok(instruction)
    }

    pub fn to_instruction(&self) -> Result<InstructionV1, InstructionConversionError> {
        let instruction = match self {
            Self::TakeFromWorktop {
                resource_address,
                amount,
                ..
            } => InstructionV1::TakeFromWorktop {
                resource_address: resource_address.to_typed()?,
                amount: amount.to_typed()?,
            },
            Self::TakeNonFungiblesFromWorktop {
                ids,
                resource_address,
                ..
            } => InstructionV1::TakeNonFungiblesFromWorktop {
                resource_address: resource_address.to_typed()?,
                ids: ids.to_typed()?,
            },
            Self::TakeAllFromWorktop {
                resource_address, ..
            } => InstructionV1::TakeAllFromWorktop {
                resource_address: resource_address.to_typed()?,
            },
            Self::ReturnToWorktop { bucket } => InstructionV1::ReturnToWorktop {
                bucket_id: bucket.to_typed()?,
            },
            Self::AssertWorktopContains {
                resource_address,
                amount,
            } => InstructionV1::AssertWorktopContains {
                resource_address: resource_address.to_typed()?,
                amount: amount.to_typed()?,
            },
            Self::AssertWorktopContainsNonFungibles {
                resource_address,
                ids,
            } => InstructionV1::AssertWorktopContainsNonFungibles {
                resource_address: resource_address.to_typed()?,
                ids: ids.to_typed()?,
            },
            Self::PopFromAuthZone { .. } => InstructionV1::PopFromAuthZone {},
            Self::PushToAuthZone { proof } => InstructionV1::PushToAuthZone {
                proof_id: proof.to_typed()?,
            },
            Self::ClearAuthZone => InstructionV1::ClearAuthZone,
            Self::ClearSignatureProofs => InstructionV1::ClearSignatureProofs,
            Self::CreateProofFromAuthZone {
                resource_address, ..
            } => InstructionV1::CreateProofFromAuthZone {
                resource_address: resource_address.to_typed()?,
            },
            Self::CreateProofFromAuthZoneOfAll {
                resource_address, ..
            } => InstructionV1::CreateProofFromAuthZoneOfAll {
                resource_address: resource_address.to_typed()?,
            },
            Self::CreateProofFromAuthZoneOfAmount {
                resource_address,
                amount,
                ..
            } => InstructionV1::CreateProofFromAuthZoneOfAmount {
                resource_address: resource_address.to_typed()?,
                amount: amount.to_typed()?,
            },
            Self::CreateProofFromAuthZoneOfNonFungibles {
                resource_address,
                ids,
                ..
            } => InstructionV1::CreateProofFromAuthZoneOfNonFungibles {
                resource_address: resource_address.to_typed()?,
                ids: ids.to_typed()?,
            },
            Self::CreateProofFromBucket { bucket, .. } => InstructionV1::CreateProofFromBucket {
                bucket_id: bucket.to_typed()?,
            },
            Self::CreateProofFromBucketOfAll { bucket, .. } => {
                InstructionV1::CreateProofFromBucketOfAll {
                    bucket_id: bucket.to_typed()?,
                }
            }
            Self::CreateProofFromBucketOfAmount { bucket, amount, .. } => {
                InstructionV1::CreateProofFromBucketOfAmount {
                    bucket_id: bucket.to_typed()?,
                    amount: amount.to_typed()?,
                }
            }
            Self::CreateProofFromBucketOfNonFungibles { bucket, ids, .. } => {
                InstructionV1::CreateProofFromBucketOfNonFungibles {
                    bucket_id: bucket.to_typed()?,
                    ids: ids.to_typed()?,
                }
            }
            Self::BurnResource { bucket } => InstructionV1::BurnResource {
                bucket_id: bucket.to_typed()?,
            },
            Self::CloneProof { proof, .. } => InstructionV1::CloneProof {
                proof_id: proof.to_typed()?,
            },
            Self::DropProof { proof, .. } => InstructionV1::DropProof {
                proof_id: proof.to_typed()?,
            },
            Self::DropAllProofs {} => InstructionV1::DropAllProofs {},
            Self::CallFunction {
                package_address,
                blueprint_name,
                function_name,
                args,
            } => InstructionV1::CallFunction {
                package_address: package_address.to_typed()?,
                blueprint_name: blueprint_name.to_typed()?,
                function_name: function_name.to_typed()?,
                args: args.to_typed()?,
            },
            Self::CallMethod {
                address,
                method_name,
                args,
            } => InstructionV1::CallMethod {
                address: address.to_typed()?,
                method_name: method_name.to_typed()?,
                args: args.to_typed()?,
            },
            Self::CallRoyaltyMethod {
                address,
                method_name,
                args,
            } => InstructionV1::CallRoyaltyMethod {
                address: address.to_typed()?,
                method_name: method_name.to_typed()?,
                args: args.to_typed()?,
            },
            Self::CallMetadataMethod {
                address,
                method_name,
                args,
            } => InstructionV1::CallMetadataMethod {
                address: address.to_typed()?,
                method_name: method_name.to_typed()?,
                args: args.to_typed()?,
            },
            Self::CallAccessRulesMethod {
                address,
                method_name,
                args,
            } => InstructionV1::CallAccessRulesMethod {
                address: address.to_typed()?,
                method_name: method_name.to_typed()?,
                args: args.to_typed()?,
            },
            Self::AllocateGlobalAddress {
                package_address,
                blueprint_name,
                ..
            } => InstructionV1::AllocateGlobalAddress {
                package_address: package_address.to_typed()?,
                blueprint_name: blueprint_name.to_typed()?,
            },
            Self::RecallVault { vault_id, amount } => InstructionV1::CallDirectVaultMethod {
                address: vault_id.to_typed()?,
                method_name: VAULT_RECALL_IDENT.to_string(),
                args: manifest_args!(amount.to_manifest_value()?),
            },
            Self::FreezeVault { vault_id } => InstructionV1::CallDirectVaultMethod {
                address: vault_id.to_typed()?,
                method_name: VAULT_FREEZE_IDENT.to_string(),
                args: manifest_args!(),
            },
            Self::UnfreezeVault { vault_id } => InstructionV1::CallDirectVaultMethod {
                address: vault_id.to_typed()?,
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
                args: to_manifest_value(&PackagePublishWasmManifestIndexMapInput {
                    code: code.to_typed()?,
                    metadata: metadata.to_typed()?,
                    setup: resolve_encoded_type(setup)
                        .ok_or(InstructionConversionError::FailedToResolveSetup)?,
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
                args: to_manifest_value(&PackagePublishWasmAdvancedManifestIndexMapInput {
                    code: code.to_typed()?,
                    metadata: metadata.to_typed()?,
                    setup: resolve_encoded_type(setup)
                        .ok_or(InstructionConversionError::FailedToResolveSetup)?,
                    owner_rule: owner_rule.to_typed()?,
                    package_address: package_address.to_typed()?,
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
                args: to_manifest_value(&FungibleResourceManagerCreateIndexMapInput {
                    access_rules: access_rules.to_typed()?,
                    divisibility: divisibility.to_typed()?,
                    metadata: metadata.to_typed()?,
                    track_total_supply: track_total_supply.to_typed()?,
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
                args: to_manifest_value(
                    &FungibleResourceManagerCreateWithInitialSupplyIndexMapInput {
                        access_rules: access_rules.to_typed()?,
                        divisibility: divisibility.to_typed()?,
                        metadata: metadata.to_typed()?,
                        track_total_supply: track_total_supply.to_typed()?,
                        initial_supply: initial_supply.to_typed()?,
                    },
                ),
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
                args: to_manifest_value(&NonFungibleResourceManagerCreateIndexMapInput {
                    access_rules: access_rules.to_typed()?,
                    metadata: metadata.to_typed()?,
                    track_total_supply: track_total_supply.to_typed()?,
                    id_type: id_type.to_typed()?,
                    non_fungible_schema: non_fungible_schema.to_typed()?,
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
                    &NonFungibleResourceManagerCreateWithInitialSupplyManifestIndexMapInput {
                        access_rules: access_rules.to_typed()?,
                        metadata: metadata.to_typed()?,
                        track_total_supply: track_total_supply.to_typed()?,
                        id_type: id_type.to_typed()?,
                        non_fungible_schema: non_fungible_schema.to_typed()?,
                        entries: entries.to_typed()?,
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
                    controlled_asset: controlled_asset.to_typed()?,
                    rule_set: rule_set.to_typed()?,
                    timed_recovery_delay_in_minutes: timed_recovery_delay_in_minutes.to_typed()?,
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
                    owner_rule: owner_rule.to_typed()?,
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
                    owner_role: owner_role.to_typed()?,
                }),
            },
            Self::SetMetadata {
                address,
                key,
                value,
            } => InstructionV1::CallMetadataMethod {
                address: address.to_typed()?,
                method_name: METADATA_SET_IDENT.to_string(),
                args: to_manifest_value(&MetadataSetInput {
                    key: key.to_typed()?,
                    value: value.to_typed()?,
                }),
            },
            Self::RemoveMetadata { address, key } => InstructionV1::CallMetadataMethod {
                address: address.to_typed()?,
                method_name: METADATA_REMOVE_IDENT.to_string(),
                args: to_manifest_value(&MetadataRemoveInput {
                    key: key.to_typed()?,
                }),
            },
            Self::SetComponentRoyaltyConfig {
                address,
                method,
                amount,
            } => InstructionV1::CallRoyaltyMethod {
                address: address.to_typed()?,
                method_name: COMPONENT_ROYALTY_SET_ROYALTY_IDENT.to_string(),
                args: to_manifest_value(&ComponentSetRoyaltyInput {
                    method: method.to_typed()?,
                    amount: amount.to_typed()?,
                }),
            },
            Self::ClaimComponentRoyalty { address } => InstructionV1::CallRoyaltyMethod {
                address: address.to_typed()?,
                method_name: COMPONENT_ROYALTY_CLAIM_ROYALTIES_IDENT.to_string(),
                args: to_manifest_value(&ComponentClaimRoyaltiesInput {}),
            },
            Self::UpdateRole {
                address,
                role_key,
                rule,
                mutability,
            } => InstructionV1::CallAccessRulesMethod {
                address: address.to_typed()?,
                method_name: ACCESS_RULES_UPDATE_ROLE_IDENT.to_string(),
                args: to_manifest_value(&AccessRulesUpdateRoleInput {
                    role_key: role_key.to_typed()?,
                    rule: rule.to_typed()?,
                    mutability: mutability.to_typed()?,
                }),
            },
            Self::ClaimPackageRoyalty { address } => InstructionV1::CallMethod {
                address: address.to_typed()?,
                method_name: PACKAGE_CLAIM_ROYALTIES_IDENT.to_string(),
                args: to_manifest_value(&PackageClaimRoyaltiesInput {}),
            },
            Self::MintFungible { address, amount } => InstructionV1::CallMethod {
                address: address.to_typed()?,
                method_name: FUNGIBLE_RESOURCE_MANAGER_MINT_IDENT.to_string(),
                args: to_manifest_value(&FungibleResourceManagerMintInput {
                    amount: amount.to_typed()?,
                }),
            },
            Self::MintNonFungible { address, entries } => InstructionV1::CallMethod {
                address: address.to_typed()?,
                method_name: NON_FUNGIBLE_RESOURCE_MANAGER_MINT_IDENT.to_string(),
                args: to_manifest_value(&NonFungibleResourceManagerMintManifestIndexMapInput {
                    entries: entries.to_typed()?,
                }),
            },
            Self::MintUuidNonFungible { address, entries } => InstructionV1::CallMethod {
                address: address.to_typed()?,
                method_name: NON_FUNGIBLE_RESOURCE_MANAGER_MINT_UUID_IDENT.to_string(),
                args: to_manifest_value(&NonFungibleResourceManagerMintUuidManifestInput {
                    entries: entries.to_typed()?,
                }),
            },
            Self::CreateValidator { key, fee_factor } => InstructionV1::CallMethod {
                address: DynamicGlobalAddress::Static(CONSENSUS_MANAGER.into()),
                method_name: CONSENSUS_MANAGER_CREATE_VALIDATOR_IDENT.to_string(),
                args: to_manifest_value(&ConsensusManagerCreateValidatorInput {
                    key: key.to_typed()?,
                    fee_factor: fee_factor.to_typed()?,
                }),
            },
        };
        Ok(instruction)
    }
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
#[serde(tag = "kind", content = "error")]
pub enum InstructionConversionError {
    ValueConversionError(ValueConversionError),
    FailedToResolveSetup,
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
    ) -> Result<SerializableInstruction, ValueConversionError>;

    fn alias(
        node_id: &NodeId,
        object_module_id: ObjectModuleId,
        method_name: &str,
        args: &ManifestValue,
        network_id: u8,
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
                Self::handle_aliasing(node_id, &to_manifest_type(args).unwrap(), network_id).ok()
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
    ) -> Result<SerializableInstruction, ValueConversionError> {
        let vault_address = InternalAddress::new_or_panic(node_id.0);
        let instruction = SerializableInstruction::RecallVault {
            vault_id: SerializableManifestValue::from_typed(&vault_address, network_id)?,
            amount: SerializableManifestValue::from_typed(&amount, network_id)?,
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
    ) -> Result<SerializableInstruction, ValueConversionError> {
        let vault_address = InternalAddress::new_or_panic(node_id.0);
        let instruction = SerializableInstruction::FreezeVault {
            vault_id: SerializableManifestValue::from_typed(&vault_address, network_id)?,
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
    ) -> Result<SerializableInstruction, ValueConversionError> {
        let vault_address = InternalAddress::new_or_panic(node_id.0);
        let instruction = SerializableInstruction::UnfreezeVault {
            vault_id: SerializableManifestValue::from_typed(&vault_address, network_id)?,
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
    ) -> Result<SerializableInstruction, ValueConversionError> {
        let address = DynamicGlobalAddress::Static(GlobalAddress::new_or_panic(node_id.0));
        let instruction = SerializableInstruction::SetMetadata {
            address: SerializableManifestValue::from_typed(&address, network_id)?,
            key: SerializableManifestValue::from_typed(&key, network_id)?,
            value: SerializableManifestValue::from_typed(&value, network_id)?,
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
    ) -> Result<SerializableInstruction, ValueConversionError> {
        let address = DynamicGlobalAddress::Static(GlobalAddress::new_or_panic(node_id.0));
        let instruction = SerializableInstruction::RemoveMetadata {
            address: SerializableManifestValue::from_typed(&address, network_id)?,
            key: SerializableManifestValue::from_typed(&key, network_id)?,
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
    ) -> Result<SerializableInstruction, ValueConversionError> {
        let address = DynamicGlobalAddress::Static(GlobalAddress::new_or_panic(node_id.0));
        let instruction = SerializableInstruction::SetComponentRoyaltyConfig {
            address: SerializableManifestValue::from_typed(&address, network_id)?,
            method: SerializableManifestValue::from_typed(&method, network_id)?,
            amount: SerializableManifestValue::from_typed(&amount, network_id)?,
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
    ) -> Result<SerializableInstruction, ValueConversionError> {
        let address = DynamicGlobalAddress::Static(GlobalAddress::new_or_panic(node_id.0));
        let instruction = SerializableInstruction::ClaimComponentRoyalty {
            address: SerializableManifestValue::from_typed(&address, network_id)?,
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
    ) -> Result<SerializableInstruction, ValueConversionError> {
        let address = DynamicGlobalAddress::Static(GlobalAddress::new_or_panic(node_id.0));
        let instruction = SerializableInstruction::UpdateRole {
            address: SerializableManifestValue::from_typed(&address, network_id)?,
            role_key: SerializableManifestValue::from_typed(&role_key, network_id)?,
            rule: SerializableManifestValue::from_typed(&rule, network_id)?,
            mutability: SerializableManifestValue::from_typed(&mutability, network_id)?,
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
    ) -> Result<SerializableInstruction, ValueConversionError> {
        let address = DynamicGlobalAddress::Static(GlobalAddress::new_or_panic(node_id.0));
        let instruction = SerializableInstruction::ClaimPackageRoyalty {
            address: SerializableManifestValue::from_typed(&address, network_id)?,
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
    ) -> Result<SerializableInstruction, ValueConversionError> {
        let address = DynamicGlobalAddress::Static(GlobalAddress::new_or_panic(node_id.0));
        let instruction = SerializableInstruction::MintFungible {
            address: SerializableManifestValue::from_typed(&address, network_id)?,
            amount: SerializableManifestValue::from_typed(&amount, network_id)?,
        };
        Ok(instruction)
    }
}

struct MintNonFungibleAlias;
impl CallMethodAlias for MintNonFungibleAlias {
    type ScryptoInput = NonFungibleResourceManagerMintIndexMapInput;
    type ManifestInput = NonFungibleResourceManagerMintManifestIndexMapInput;
    const METHOD_NAME: &'static str = NON_FUNGIBLE_RESOURCE_MANAGER_MINT_IDENT;
    const MODULE: ObjectModuleId = ObjectModuleId::Main;

    fn is_valid_address(node_id: &NodeId) -> bool {
        node_id.is_global_non_fungible_resource_manager()
    }

    fn handle_aliasing(
        node_id: &NodeId,
        NonFungibleResourceManagerMintManifestIndexMapInput { entries }: &Self::ManifestInput,
        network_id: u8,
    ) -> Result<SerializableInstruction, ValueConversionError> {
        let address = DynamicGlobalAddress::Static(GlobalAddress::new_or_panic(node_id.0));
        let instruction = SerializableInstruction::MintNonFungible {
            address: SerializableManifestValue::from_typed(&address, network_id)?,
            entries: SerializableManifestValue::from_typed(&entries, network_id)?,
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
    ) -> Result<SerializableInstruction, ValueConversionError> {
        let address = DynamicGlobalAddress::Static(GlobalAddress::new_or_panic(node_id.0));
        let instruction = SerializableInstruction::MintUuidNonFungible {
            address: SerializableManifestValue::from_typed(&address, network_id)?,
            entries: SerializableManifestValue::from_typed(&entries, network_id)?,
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
        Self::ManifestInput { key, fee_factor }: &Self::ManifestInput,
        network_id: u8,
    ) -> Result<SerializableInstruction, ValueConversionError> {
        let instruction = SerializableInstruction::CreateValidator {
            key: SerializableManifestValue::from_typed(&key, network_id)?,
            fee_factor: SerializableManifestValue::from_typed(&fee_factor, network_id)?,
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
    ) -> Result<SerializableInstruction, ValueConversionError>;

    fn alias(
        node_id: &NodeId,
        blueprint_name: &str,
        function_name: &str,
        args: &ManifestValue,
        network_id: u8,
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
                Self::handle_aliasing(node_id, &to_manifest_type(args).unwrap(), network_id).ok()
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
    type ScryptoInput = PackagePublishWasmIndexMapInput;
    type ManifestInput = PackagePublishWasmIndexMapInput;
    const FUNCTION_NAME: &'static str = PACKAGE_PUBLISH_WASM_IDENT;
    const BLUEPRINT_NAME: &'static str = PACKAGE_BLUEPRINT;

    fn is_valid_address(node_id: &NodeId) -> bool {
        node_id == PACKAGE_PACKAGE.as_node_id()
    }

    fn handle_aliasing(
        _: &NodeId,
        PackagePublishWasmIndexMapInput {
            code,
            setup,
            metadata,
        }: &Self::ManifestInput,
        network_id: u8,
    ) -> Result<SerializableInstruction, ValueConversionError> {
        let instruction = SerializableInstruction::PublishPackage {
            code: SerializableManifestValue::from_typed(&code, network_id)?,
            setup: SerializableManifestValue::from_typed(
                &manifest_encode(&setup).unwrap(),
                network_id,
            )?,
            metadata: SerializableManifestValue::from_typed(&metadata, network_id)?,
        };
        Ok(instruction)
    }
}

struct PublishPackageAdvancedAlias;
impl CallFunctionAlias for PublishPackageAdvancedAlias {
    type ScryptoInput = PackagePublishWasmAdvancedIndexMapInput;
    type ManifestInput = PackagePublishWasmAdvancedManifestIndexMapInput;
    const FUNCTION_NAME: &'static str = PACKAGE_PUBLISH_WASM_ADVANCED_IDENT;
    const BLUEPRINT_NAME: &'static str = PACKAGE_BLUEPRINT;

    fn is_valid_address(node_id: &NodeId) -> bool {
        node_id == PACKAGE_PACKAGE.as_node_id()
    }

    fn handle_aliasing(
        _: &NodeId,
        PackagePublishWasmAdvancedManifestIndexMapInput {
            code,
            metadata,
            owner_rule,
            package_address,
            setup,
        }: &Self::ManifestInput,
        network_id: u8,
    ) -> Result<SerializableInstruction, ValueConversionError> {
        let instruction = SerializableInstruction::PublishPackageAdvanced {
            package_address: SerializableManifestValue::from_typed(&package_address, network_id)?,
            code: SerializableManifestValue::from_typed(&code, network_id)?,
            setup: SerializableManifestValue::from_typed(
                &manifest_encode(&setup).unwrap(),
                network_id,
            )?,
            metadata: SerializableManifestValue::from_typed(&metadata, network_id)?,
            owner_rule: SerializableManifestValue::from_typed(&owner_rule, network_id)?,
        };
        Ok(instruction)
    }
}

struct CreateFungibleResourceAlias;
impl CallFunctionAlias for CreateFungibleResourceAlias {
    type ScryptoInput = FungibleResourceManagerCreateIndexMapInput;
    type ManifestInput = FungibleResourceManagerCreateIndexMapInput;
    const FUNCTION_NAME: &'static str = FUNGIBLE_RESOURCE_MANAGER_CREATE_IDENT;
    const BLUEPRINT_NAME: &'static str = FUNGIBLE_RESOURCE_MANAGER_BLUEPRINT;

    fn is_valid_address(node_id: &NodeId) -> bool {
        node_id == RESOURCE_PACKAGE.as_node_id()
    }

    fn handle_aliasing(
        _: &NodeId,
        FungibleResourceManagerCreateIndexMapInput {
            access_rules,
            divisibility,
            metadata,
            track_total_supply,
        }: &Self::ManifestInput,
        network_id: u8,
    ) -> Result<SerializableInstruction, ValueConversionError> {
        let instruction = SerializableInstruction::CreateFungibleResource {
            track_total_supply: SerializableManifestValue::from_typed(
                &track_total_supply,
                network_id,
            )?,
            divisibility: SerializableManifestValue::from_typed(&divisibility, network_id)?,
            metadata: SerializableManifestValue::from_typed(&metadata, network_id)?,
            access_rules: SerializableManifestValue::from_typed(&access_rules, network_id)?,
        };
        Ok(instruction)
    }
}

struct CreateFungibleResourceWithInitialSupplyAlias;
impl CallFunctionAlias for CreateFungibleResourceWithInitialSupplyAlias {
    type ScryptoInput = FungibleResourceManagerCreateWithInitialSupplyIndexMapInput;
    type ManifestInput = FungibleResourceManagerCreateWithInitialSupplyIndexMapInput;
    const FUNCTION_NAME: &'static str = FUNGIBLE_RESOURCE_MANAGER_CREATE_WITH_INITIAL_SUPPLY_IDENT;
    const BLUEPRINT_NAME: &'static str = FUNGIBLE_RESOURCE_MANAGER_BLUEPRINT;

    fn is_valid_address(node_id: &NodeId) -> bool {
        node_id == RESOURCE_PACKAGE.as_node_id()
    }

    fn handle_aliasing(
        _: &NodeId,
        FungibleResourceManagerCreateWithInitialSupplyIndexMapInput {
            access_rules,
            divisibility,
            initial_supply,
            metadata,
            track_total_supply,
        }: &Self::ManifestInput,
        network_id: u8,
    ) -> Result<SerializableInstruction, ValueConversionError> {
        let instruction = SerializableInstruction::CreateFungibleResourceWithInitialSupply {
            track_total_supply: SerializableManifestValue::from_typed(
                &track_total_supply,
                network_id,
            )?,
            divisibility: SerializableManifestValue::from_typed(&divisibility, network_id)?,
            metadata: SerializableManifestValue::from_typed(&metadata, network_id)?,
            access_rules: SerializableManifestValue::from_typed(&access_rules, network_id)?,
            initial_supply: SerializableManifestValue::from_typed(&initial_supply, network_id)?,
        };
        Ok(instruction)
    }
}

struct CreateNonFungibleResourceAlias;
impl CallFunctionAlias for CreateNonFungibleResourceAlias {
    type ScryptoInput = NonFungibleResourceManagerCreateIndexMapInput;
    type ManifestInput = NonFungibleResourceManagerCreateIndexMapInput;
    const FUNCTION_NAME: &'static str = NON_FUNGIBLE_RESOURCE_MANAGER_CREATE_IDENT;
    const BLUEPRINT_NAME: &'static str = NON_FUNGIBLE_RESOURCE_MANAGER_BLUEPRINT;

    fn is_valid_address(node_id: &NodeId) -> bool {
        node_id == RESOURCE_PACKAGE.as_node_id()
    }

    fn handle_aliasing(
        _: &NodeId,
        NonFungibleResourceManagerCreateIndexMapInput {
            access_rules,
            metadata,
            track_total_supply,
            id_type,
            non_fungible_schema,
        }: &Self::ManifestInput,
        network_id: u8,
    ) -> Result<SerializableInstruction, ValueConversionError> {
        let instruction = SerializableInstruction::CreateNonFungibleResource {
            track_total_supply: SerializableManifestValue::from_typed(
                &track_total_supply,
                network_id,
            )?,
            id_type: SerializableManifestValue::from_typed(&id_type, network_id)?,
            metadata: SerializableManifestValue::from_typed(&metadata, network_id)?,
            access_rules: SerializableManifestValue::from_typed(&access_rules, network_id)?,
            non_fungible_schema: SerializableManifestValue::from_typed(
                &non_fungible_schema,
                network_id,
            )?,
        };
        Ok(instruction)
    }
}

struct CreateNonFungibleResourceWithInitialSupplyAlias;
impl CallFunctionAlias for CreateNonFungibleResourceWithInitialSupplyAlias {
    type ScryptoInput = NonFungibleResourceManagerCreateWithInitialSupplyIndexMapInput;
    type ManifestInput = NonFungibleResourceManagerCreateWithInitialSupplyManifestIndexMapInput;
    const FUNCTION_NAME: &'static str =
        NON_FUNGIBLE_RESOURCE_MANAGER_CREATE_WITH_INITIAL_SUPPLY_IDENT;
    const BLUEPRINT_NAME: &'static str = NON_FUNGIBLE_RESOURCE_MANAGER_BLUEPRINT;

    fn is_valid_address(node_id: &NodeId) -> bool {
        node_id == RESOURCE_PACKAGE.as_node_id()
    }

    fn handle_aliasing(
        _: &NodeId,
        NonFungibleResourceManagerCreateWithInitialSupplyManifestIndexMapInput {
            access_rules,
            metadata,
            track_total_supply,
            id_type,
            non_fungible_schema,
            entries,
        }: &Self::ManifestInput,
        network_id: u8,
    ) -> Result<SerializableInstruction, ValueConversionError> {
        let instruction = SerializableInstruction::CreateNonFungibleResourceWithInitialSupply {
            track_total_supply: SerializableManifestValue::from_typed(
                &track_total_supply,
                network_id,
            )?,
            id_type: SerializableManifestValue::from_typed(&id_type, network_id)?,
            metadata: SerializableManifestValue::from_typed(&metadata, network_id)?,
            access_rules: SerializableManifestValue::from_typed(&access_rules, network_id)?,
            non_fungible_schema: SerializableManifestValue::from_typed(
                &non_fungible_schema,
                network_id,
            )?,
            entries: SerializableManifestValue::from_typed(&entries, network_id)?,
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
    ) -> Result<SerializableInstruction, ValueConversionError> {
        let instruction = SerializableInstruction::CreateAccessController {
            controlled_asset: SerializableManifestValue::from_typed(&controlled_asset, network_id)?,
            rule_set: SerializableManifestValue::from_typed(&rule_set, network_id)?,
            timed_recovery_delay_in_minutes: SerializableManifestValue::from_typed(
                &timed_recovery_delay_in_minutes,
                network_id,
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
    ) -> Result<SerializableInstruction, ValueConversionError> {
        let instruction = SerializableInstruction::CreateIdentityAdvanced {
            owner_rule: SerializableManifestValue::from_typed(&owner_rule, network_id)?,
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
    ) -> Result<SerializableInstruction, ValueConversionError> {
        let instruction = SerializableInstruction::CreateAccountAdvanced {
            owner_role: SerializableManifestValue::from_typed(&owner_role, network_id)?,
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

pub fn to_serializable_instructions(
    instructions: &[InstructionV1],
    network_id: u8,
) -> Result<Vec<SerializableInstruction>, LocatedInstructionConversionError> {
    let mut id_allocator = ManifestIdAllocator::default();

    instructions
        .iter()
        .enumerate()
        .map(|(instruction_index, instruction)| {
            SerializableInstruction::from_instruction(instruction, network_id, &mut id_allocator)
                .map_err(|error| LocatedInstructionConversionError {
                    instruction_index,
                    error,
                })
        })
        .collect::<Result<_, _>>()
}

pub fn to_native_instructions(
    instructions: &[SerializableInstruction],
) -> Result<Vec<InstructionV1>, LocatedInstructionConversionError> {
    instructions
        .iter()
        .enumerate()
        .map(|(instruction_index, instruction)| {
            instruction
                .to_instruction()
                .map_err(|error| LocatedInstructionConversionError {
                    instruction_index,
                    error,
                })
        })
        .collect::<Result<_, _>>()
}

#[derive(Debug, Clone)]
pub struct LocatedInstructionConversionError {
    pub instruction_index: usize,
    pub error: InstructionConversionError,
}

fn resolve_encoded_type<T>(value: &SerializableManifestValue) -> Option<T>
where
    T: ManifestDecode,
{
    match value {
        SerializableManifestValue::Array {
            element_value_kind: SerializableManifestValueKind::U8,
            elements,
        } => elements
            .iter()
            .map(|element| match element {
                SerializableManifestValue::U8 { value } => Some(**value),
                _ => None,
            })
            .collect::<Option<Vec<u8>>>()
            .and_then(|bytes| manifest_decode(&bytes).ok()),
        _ => None,
    }
}
