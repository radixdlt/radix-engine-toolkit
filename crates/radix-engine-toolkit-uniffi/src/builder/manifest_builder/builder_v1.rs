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

#![allow(clippy::too_many_arguments)]

use crate::prelude::*;
use radix_common::prelude::{to_manifest_value, FromPublicKey};

#[derive(Debug, Clone, Object, Default)]
pub struct ManifestV1Builder {
    name_record: NameRecord,
    instructions: Vec<engine::InstructionV1>,
    blobs: Vec<Vec<u8>>,
}

#[uniffi::export]
impl ManifestV1Builder {
    #[uniffi::constructor]
    pub fn new() -> Arc<Self> {
        Arc::new(Default::default())
    }

    //===================
    // Base Instructions
    //===================

    pub fn take_all_from_worktop(
        self: Arc<Self>,
        resource_address: Arc<Address>,
        into_bucket: ManifestBuilderBucket,
    ) -> Result<Arc<Self>> {
        builder_arc_map(self, |builder| {
            let resource_address =
                engine::ResourceAddress::try_from(*resource_address)?;
            builder.name_record.new_bucket(&into_bucket.name)?;

            let instruction = engine::InstructionV1::TakeAllFromWorktop(
                engine::TakeAllFromWorktop { resource_address },
            );
            builder.instructions.push(instruction);
            Ok(())
        })
    }

    pub fn take_from_worktop(
        self: Arc<Self>,
        resource_address: Arc<Address>,
        amount: Arc<Decimal>,
        into_bucket: ManifestBuilderBucket,
    ) -> Result<Arc<Self>> {
        builder_arc_map(self, |builder| {
            let resource_address =
                engine::ResourceAddress::try_from(*resource_address)?;
            let amount = amount.0;
            builder.name_record.new_bucket(&into_bucket.name)?;

            let instruction = engine::InstructionV1::TakeFromWorktop(
                engine::TakeFromWorktop {
                    resource_address,
                    amount,
                },
            );
            builder.instructions.push(instruction);
            Ok(())
        })
    }

    pub fn take_non_fungibles_from_worktop(
        self: Arc<Self>,
        resource_address: Arc<Address>,
        ids: Vec<NonFungibleLocalId>,
        into_bucket: ManifestBuilderBucket,
    ) -> Result<Arc<Self>> {
        builder_arc_map(self, |builder| {
            let resource_address =
                engine::ResourceAddress::try_from(*resource_address)?;
            let ids = ids
                .into_iter()
                .map(engine::NonFungibleLocalId::try_from)
                .collect::<Result<Vec<_>>>()?;
            builder.name_record.new_bucket(&into_bucket.name)?;

            let instruction =
                engine::InstructionV1::TakeNonFungiblesFromWorktop(
                    engine::TakeNonFungiblesFromWorktop {
                        resource_address,
                        ids,
                    },
                );
            builder.instructions.push(instruction);
            Ok(())
        })
    }

    pub fn return_to_worktop(
        self: Arc<Self>,
        bucket: ManifestBuilderBucket,
    ) -> Result<Arc<Self>> {
        builder_arc_map(self, |builder| {
            let bucket = bucket.to_native(&builder.name_record)?;

            let instruction = engine::InstructionV1::ReturnToWorktop(
                engine::ReturnToWorktop { bucket_id: bucket },
            );
            builder.instructions.push(instruction);
            Ok(())
        })
    }

    pub fn assert_worktop_contains_any(
        self: Arc<Self>,
        resource_address: Arc<Address>,
    ) -> Result<Arc<Self>> {
        builder_arc_map(self, |builder| {
            let resource_address =
                engine::ResourceAddress::try_from(*resource_address)?;

            let instruction = engine::InstructionV1::AssertWorktopContainsAny(
                engine::AssertWorktopContainsAny { resource_address },
            );
            builder.instructions.push(instruction);
            Ok(())
        })
    }

    pub fn assert_worktop_contains(
        self: Arc<Self>,
        resource_address: Arc<Address>,
        amount: Arc<Decimal>,
    ) -> Result<Arc<Self>> {
        builder_arc_map(self, |builder| {
            let resource_address =
                engine::ResourceAddress::try_from(*resource_address)?;
            let amount = amount.0;

            let instruction = engine::InstructionV1::AssertWorktopContains(
                engine::AssertWorktopContains {
                    resource_address,
                    amount,
                },
            );
            builder.instructions.push(instruction);
            Ok(())
        })
    }

    pub fn assert_worktop_contains_non_fungibles(
        self: Arc<Self>,
        resource_address: Arc<Address>,
        ids: Vec<NonFungibleLocalId>,
    ) -> Result<Arc<Self>> {
        builder_arc_map(self, |builder| {
            let resource_address =
                engine::ResourceAddress::try_from(*resource_address)?;
            let ids = ids
                .into_iter()
                .map(engine::NonFungibleLocalId::try_from)
                .collect::<Result<Vec<_>>>()?;

            let instruction =
                engine::InstructionV1::TakeNonFungiblesFromWorktop(
                    engine::TakeNonFungiblesFromWorktop {
                        resource_address,
                        ids,
                    },
                );
            builder.instructions.push(instruction);
            Ok(())
        })
    }

    pub fn pop_from_auth_zone(
        self: Arc<Self>,
        into_proof: ManifestBuilderProof,
    ) -> Result<Arc<Self>> {
        builder_arc_map(self, |builder| {
            builder.name_record.new_proof(&into_proof.name)?;

            let instruction =
                engine::InstructionV1::PopFromAuthZone(engine::PopFromAuthZone);
            builder.instructions.push(instruction);
            Ok(())
        })
    }

    pub fn push_to_auth_zone(
        self: Arc<Self>,
        proof: ManifestBuilderProof,
    ) -> Result<Arc<Self>> {
        builder_arc_map(self, |builder| {
            let proof = proof.to_native(&builder.name_record)?;

            let instruction =
                engine::InstructionV1::PushToAuthZone(engine::PushToAuthZone {
                    proof_id: proof,
                });
            builder.instructions.push(instruction);
            Ok(())
        })
    }

    pub fn drop_auth_zone_proofs(self: Arc<Self>) -> Result<Arc<Self>> {
        builder_arc_map(self, |builder| {
            let instruction = engine::InstructionV1::DropAuthZoneProofs(
                engine::DropAuthZoneProofs,
            );
            builder.instructions.push(instruction);
            Ok(())
        })
    }

    pub fn drop_auth_zone_signature_proofs(
        self: Arc<Self>,
    ) -> Result<Arc<Self>> {
        builder_arc_map(self, |builder| {
            let instruction =
                engine::InstructionV1::DropAuthZoneSignatureProofs(
                    engine::DropAuthZoneSignatureProofs,
                );
            builder.instructions.push(instruction);
            Ok(())
        })
    }

    pub fn drop_all_proofs(self: Arc<Self>) -> Result<Arc<Self>> {
        builder_arc_map(self, |builder| {
            let instruction =
                engine::InstructionV1::DropAllProofs(engine::DropAllProofs);
            builder.instructions.push(instruction);
            Ok(())
        })
    }

    pub fn create_proof_from_auth_zone_of_all(
        self: Arc<Self>,
        resource_address: Arc<Address>,
        into_proof: ManifestBuilderProof,
    ) -> Result<Arc<Self>> {
        builder_arc_map(self, |builder| {
            let resource_address =
                engine::ResourceAddress::try_from(*resource_address)?;
            builder.name_record.new_proof(&into_proof.name)?;

            let instruction =
                engine::InstructionV1::CreateProofFromAuthZoneOfAll(
                    engine::CreateProofFromAuthZoneOfAll { resource_address },
                );
            builder.instructions.push(instruction);
            Ok(())
        })
    }

    pub fn create_proof_from_auth_zone_of_amount(
        self: Arc<Self>,
        resource_address: Arc<Address>,
        amount: Arc<Decimal>,
        into_proof: ManifestBuilderProof,
    ) -> Result<Arc<Self>> {
        builder_arc_map(self, |builder| {
            let resource_address =
                engine::ResourceAddress::try_from(*resource_address)?;
            let amount = amount.0;
            builder.name_record.new_proof(&into_proof.name)?;

            let instruction =
                engine::InstructionV1::CreateProofFromAuthZoneOfAmount(
                    engine::CreateProofFromAuthZoneOfAmount {
                        resource_address,
                        amount,
                    },
                );
            builder.instructions.push(instruction);
            Ok(())
        })
    }

    pub fn create_proof_from_auth_zone_of_non_fungibles(
        self: Arc<Self>,
        resource_address: Arc<Address>,
        ids: Vec<NonFungibleLocalId>,
        into_proof: ManifestBuilderProof,
    ) -> Result<Arc<Self>> {
        builder_arc_map(self, |builder| {
            let resource_address =
                engine::ResourceAddress::try_from(*resource_address)?;
            let ids = ids
                .into_iter()
                .map(engine::NonFungibleLocalId::try_from)
                .collect::<Result<Vec<_>>>()?;
            builder.name_record.new_proof(&into_proof.name)?;

            let instruction =
                engine::InstructionV1::CreateProofFromAuthZoneOfNonFungibles(
                    engine::CreateProofFromAuthZoneOfNonFungibles {
                        resource_address,
                        ids,
                    },
                );
            builder.instructions.push(instruction);
            Ok(())
        })
    }

    pub fn create_proof_from_bucket_of_all(
        self: Arc<Self>,
        bucket: ManifestBuilderBucket,
        into_proof: ManifestBuilderProof,
    ) -> Result<Arc<Self>> {
        builder_arc_map(self, |builder| {
            let bucket = bucket.to_native(&builder.name_record)?;
            builder.name_record.new_proof(&into_proof.name)?;

            let instruction = engine::InstructionV1::CreateProofFromBucketOfAll(
                engine::CreateProofFromBucketOfAll { bucket_id: bucket },
            );
            builder.instructions.push(instruction);
            Ok(())
        })
    }

    pub fn create_proof_from_bucket_of_amount(
        self: Arc<Self>,
        amount: Arc<Decimal>,
        bucket: ManifestBuilderBucket,
        into_proof: ManifestBuilderProof,
    ) -> Result<Arc<Self>> {
        builder_arc_map(self, |builder| {
            let amount = amount.0;
            let bucket = bucket.to_native(&builder.name_record)?;
            builder.name_record.new_proof(&into_proof.name)?;

            let instruction =
                engine::InstructionV1::CreateProofFromBucketOfAmount(
                    engine::CreateProofFromBucketOfAmount {
                        bucket_id: bucket,
                        amount,
                    },
                );
            builder.instructions.push(instruction);
            Ok(())
        })
    }

    pub fn create_proof_from_bucket_of_non_fungibles(
        self: Arc<Self>,
        ids: Vec<NonFungibleLocalId>,
        bucket: ManifestBuilderBucket,
        into_proof: ManifestBuilderProof,
    ) -> Result<Arc<Self>> {
        builder_arc_map(self, |builder| {
            let ids = ids
                .into_iter()
                .map(engine::NonFungibleLocalId::try_from)
                .collect::<Result<Vec<_>>>()?;
            let bucket = bucket.to_native(&builder.name_record)?;
            builder.name_record.new_proof(&into_proof.name)?;

            let instruction =
                engine::InstructionV1::CreateProofFromBucketOfNonFungibles(
                    engine::CreateProofFromBucketOfNonFungibles {
                        bucket_id: bucket,
                        ids,
                    },
                );
            builder.instructions.push(instruction);
            Ok(())
        })
    }

    pub fn burn_resource(
        self: Arc<Self>,
        bucket: ManifestBuilderBucket,
    ) -> Result<Arc<Self>> {
        builder_arc_map(self, |builder| {
            let bucket = bucket.to_native(&builder.name_record)?;

            let instruction =
                engine::InstructionV1::BurnResource(engine::BurnResource {
                    bucket_id: bucket,
                });
            builder.instructions.push(instruction);
            Ok(())
        })
    }

    pub fn clone_proof(
        self: Arc<Self>,
        proof: ManifestBuilderProof,
        into_proof: ManifestBuilderProof,
    ) -> Result<Arc<Self>> {
        builder_arc_map(self, |builder| {
            builder.name_record.new_proof(&into_proof.name)?;
            let proof = proof.to_native(&builder.name_record)?;

            let instruction =
                engine::InstructionV1::CloneProof(engine::CloneProof {
                    proof_id: proof,
                });
            builder.instructions.push(instruction);
            Ok(())
        })
    }

    pub fn drop_proof(
        self: Arc<Self>,
        proof: ManifestBuilderProof,
    ) -> Result<Arc<Self>> {
        builder_arc_map(self, |builder| {
            let proof = proof.to_native(&builder.name_record)?;

            let instruction =
                engine::InstructionV1::DropProof(engine::DropProof {
                    proof_id: proof,
                });
            builder.instructions.push(instruction);
            Ok(())
        })
    }

    pub fn call_function(
        self: Arc<Self>,
        address: ManifestBuilderAddress,
        blueprint_name: String,
        function_name: String,
        args: Vec<ManifestBuilderValue>,
    ) -> Result<Arc<Self>> {
        builder_arc_map(self, |builder| {
            let address = match address.to_native(&builder.name_record)? {
                engine::ManifestAddress::Static(value) => value
                    .0
                    .try_into()
                    .map(engine::DynamicPackageAddress::Static)?,
                engine::ManifestAddress::Named(value) => {
                    engine::DynamicPackageAddress::Named(value)
                }
            };
            let args = engine::ManifestValue::Tuple {
                fields: args
                    .into_iter()
                    .map(|x| x.to_native(&builder.name_record))
                    .collect::<Result<_>>()?,
            };

            let instruction =
                engine::InstructionV1::CallFunction(engine::CallFunction {
                    package_address: address,
                    blueprint_name,
                    function_name,
                    args,
                });
            builder.instructions.push(instruction);
            Ok(())
        })
    }

    pub fn call_method(
        self: Arc<Self>,
        address: ManifestBuilderAddress,
        method_name: String,
        args: Vec<ManifestBuilderValue>,
    ) -> Result<Arc<Self>> {
        builder_arc_map(self, |builder| {
            let address = match address.to_native(&builder.name_record)? {
                engine::ManifestAddress::Static(value) => value
                    .0
                    .try_into()
                    .map(engine::DynamicGlobalAddress::Static)?,
                engine::ManifestAddress::Named(value) => {
                    engine::DynamicGlobalAddress::Named(value)
                }
            };
            let args = engine::ManifestValue::Tuple {
                fields: args
                    .into_iter()
                    .map(|x| x.to_native(&builder.name_record))
                    .collect::<Result<_>>()?,
            };

            let instruction =
                engine::InstructionV1::CallMethod(engine::CallMethod {
                    address,
                    method_name,
                    args,
                });
            builder.instructions.push(instruction);
            Ok(())
        })
    }

    pub fn call_royalty_method(
        self: Arc<Self>,
        address: ManifestBuilderAddress,
        method_name: String,
        args: Vec<ManifestBuilderValue>,
    ) -> Result<Arc<Self>> {
        builder_arc_map(self, |builder| {
            let address = match address.to_native(&builder.name_record)? {
                engine::ManifestAddress::Static(value) => value
                    .0
                    .try_into()
                    .map(engine::DynamicGlobalAddress::Static)?,
                engine::ManifestAddress::Named(value) => {
                    engine::DynamicGlobalAddress::Named(value)
                }
            };
            let args = engine::ManifestValue::Tuple {
                fields: args
                    .into_iter()
                    .map(|x| x.to_native(&builder.name_record))
                    .collect::<Result<_>>()?,
            };

            let instruction = engine::InstructionV1::CallRoyaltyMethod(
                engine::CallRoyaltyMethod {
                    address,
                    method_name,
                    args,
                },
            );
            builder.instructions.push(instruction);
            Ok(())
        })
    }

    pub fn call_metadata_method(
        self: Arc<Self>,
        address: ManifestBuilderAddress,
        method_name: String,
        args: Vec<ManifestBuilderValue>,
    ) -> Result<Arc<Self>> {
        builder_arc_map(self, |builder| {
            let address = match address.to_native(&builder.name_record)? {
                engine::ManifestAddress::Static(value) => value
                    .0
                    .try_into()
                    .map(engine::DynamicGlobalAddress::Static)?,
                engine::ManifestAddress::Named(value) => {
                    engine::DynamicGlobalAddress::Named(value)
                }
            };
            let args = engine::ManifestValue::Tuple {
                fields: args
                    .into_iter()
                    .map(|x| x.to_native(&builder.name_record))
                    .collect::<Result<_>>()?,
            };

            let instruction = engine::InstructionV1::CallMetadataMethod(
                engine::CallMetadataMethod {
                    address,
                    method_name,
                    args,
                },
            );
            builder.instructions.push(instruction);
            Ok(())
        })
    }

    pub fn call_access_rules_method(
        self: Arc<Self>,
        address: ManifestBuilderAddress,
        method_name: String,
        args: Vec<ManifestBuilderValue>,
    ) -> Result<Arc<Self>> {
        builder_arc_map(self, |builder| {
            let address = match address.to_native(&builder.name_record)? {
                engine::ManifestAddress::Static(value) => value
                    .0
                    .try_into()
                    .map(engine::DynamicGlobalAddress::Static)?,
                engine::ManifestAddress::Named(value) => {
                    engine::DynamicGlobalAddress::Named(value)
                }
            };
            let args = engine::ManifestValue::Tuple {
                fields: args
                    .into_iter()
                    .map(|x| x.to_native(&builder.name_record))
                    .collect::<Result<_>>()?,
            };

            let instruction = engine::InstructionV1::CallRoleAssignmentMethod(
                engine::CallRoleAssignmentMethod {
                    address,
                    method_name,
                    args,
                },
            );
            builder.instructions.push(instruction);
            Ok(())
        })
    }

    pub fn call_direct_vault_method(
        self: Arc<Self>,
        address: Arc<Address>,
        method_name: String,
        args: Vec<ManifestBuilderValue>,
    ) -> Result<Arc<Self>> {
        builder_arc_map(self, |builder| {
            let address = engine::InternalAddress::try_from(*address)?;
            let args = engine::ManifestValue::Tuple {
                fields: args
                    .into_iter()
                    .map(|x| x.to_native(&builder.name_record))
                    .collect::<Result<_>>()?,
            };

            let instruction = engine::InstructionV1::CallDirectVaultMethod(
                engine::CallDirectVaultMethod {
                    address,
                    method_name,
                    args,
                },
            );
            builder.instructions.push(instruction);
            Ok(())
        })
    }

    pub fn allocate_global_address(
        self: Arc<Self>,
        package_address: Arc<Address>,
        blueprint_name: String,
        into_address_reservation: ManifestBuilderAddressReservation,
        into_named_address: ManifestBuilderNamedAddress,
    ) -> Result<Arc<Self>> {
        builder_arc_map(self, |builder| {
            let package_address =
                engine::PackageAddress::try_from(*package_address)?;
            builder
                .name_record
                .new_address_reservation(&into_address_reservation.name)?;
            builder
                .name_record
                .new_named_address(&into_named_address.name)?;

            let instruction = engine::InstructionV1::AllocateGlobalAddress(
                engine::AllocateGlobalAddress {
                    package_address,
                    blueprint_name,
                },
            );
            builder.instructions.push(instruction);
            Ok(())
        })
    }

    //=====================
    // Instruction Aliases
    //=====================

    fn account_deposit_entire_worktop(
        self: Arc<Self>,
        account_address: Arc<Address>,
    ) -> Result<Arc<Self>> {
        builder_arc_map(self, |builder| {
            let address = engine::GlobalAddress::try_from(*account_address)?;

            let instruction =
                engine::InstructionV1::CallMethod(engine::CallMethod {
                    address: engine::DynamicGlobalAddress::Static(address),
                    method_name: engine::ACCOUNT_DEPOSIT_BATCH_IDENT.to_owned(),
                    args: manifest_args!(
                        engine::ManifestExpression::EntireWorktop
                    )
                    .into(),
                });
            builder.instructions.push(instruction);
            Ok(())
        })
    }

    fn account_try_deposit_entire_worktop_or_refund(
        self: Arc<Self>,
        account_address: Arc<Address>,
        authorized_depositor_badge: Option<ResourceOrNonFungible>,
    ) -> Result<Arc<Self>> {
        builder_arc_map(self, |builder| {
            let address = engine::GlobalAddress::try_from(*account_address)?;
            let authorized_depositor_badge =
                if let Some(badge) = authorized_depositor_badge {
                    Some(badge.to_native()?)
                } else {
                    None
                };

            let instruction =
                engine::InstructionV1::CallMethod(engine::CallMethod {
                    address: engine::DynamicGlobalAddress::Static(address),
                    method_name:
                        engine::ACCOUNT_TRY_DEPOSIT_BATCH_OR_REFUND_IDENT
                            .to_owned(),
                    args: manifest_args!(
                        engine::ManifestExpression::EntireWorktop,
                        authorized_depositor_badge
                    )
                    .into(),
                });
            builder.instructions.push(instruction);
            Ok(())
        })
    }

    fn account_try_deposit_entire_worktop_or_abort(
        self: Arc<Self>,
        account_address: Arc<Address>,
        authorized_depositor_badge: Option<ResourceOrNonFungible>,
    ) -> Result<Arc<Self>> {
        builder_arc_map(self, |builder| {
            let address = engine::GlobalAddress::try_from(*account_address)?;
            let authorized_depositor_badge =
                if let Some(badge) = authorized_depositor_badge {
                    Some(badge.to_native()?)
                } else {
                    None
                };

            let instruction =
                engine::InstructionV1::CallMethod(engine::CallMethod {
                    address: engine::DynamicGlobalAddress::Static(address),
                    method_name:
                        engine::ACCOUNT_TRY_DEPOSIT_BATCH_OR_ABORT_IDENT
                            .to_owned(),
                    args: manifest_args!(
                        engine::ManifestExpression::EntireWorktop,
                        authorized_depositor_badge
                    )
                    .into(),
                });
            builder.instructions.push(instruction);
            Ok(())
        })
    }

    pub fn package_publish(
        self: Arc<Self>,
        code: Vec<u8>,
        definition: Vec<u8>,
        metadata: MetadataInit,
    ) -> Result<Arc<Self>> {
        builder_arc_map(self, |builder| {
            let code_blob = engine::ManifestBlobRef(engine::hash(&code).0);
            builder.blobs.push(code);

            let instruction =
                engine::InstructionV1::CallFunction(engine::CallFunction {
                    package_address: engine::DynamicPackageAddress::Static(
                        engine::PACKAGE_PACKAGE,
                    ),
                    blueprint_name: engine::PACKAGE_BLUEPRINT.to_owned(),
                    function_name: engine::PACKAGE_PUBLISH_WASM_IDENT
                        .to_owned(),
                    args: engine::to_manifest_value_and_unwrap!(
                        &engine::PackagePublishWasmManifestInput {
                            code: code_blob,
                            definition: engine::manifest_decode(&definition)?,
                            metadata: metadata.to_native()?,
                        }
                    ),
                });
            builder.instructions.push(instruction);
            Ok(())
        })
    }

    pub fn package_publish_advanced(
        self: Arc<Self>,
        owner_role: OwnerRole,
        code: Vec<u8>,
        definition: Vec<u8>,
        metadata: MetadataInit,
        package_address: Option<ManifestBuilderAddressReservation>,
    ) -> Result<Arc<Self>> {
        builder_arc_map(self, |builder| {
            let code_blob = engine::ManifestBlobRef(engine::hash(&code).0);
            builder.blobs.push(code);
            let address_reservation = match package_address {
                Some(reservation) => Some(
                    *builder
                        .name_record
                        .get_address_reservation(&reservation.name)?,
                ),
                None => None,
            };

            let instruction =
                engine::InstructionV1::CallFunction(engine::CallFunction {
                    package_address: engine::DynamicPackageAddress::Static(
                        engine::PACKAGE_PACKAGE,
                    ),
                    blueprint_name: engine::PACKAGE_BLUEPRINT.to_owned(),
                    function_name: engine::PACKAGE_PUBLISH_WASM_ADVANCED_IDENT
                        .to_owned(),
                    args: engine::to_manifest_value_and_unwrap!(
                        &engine::PackagePublishWasmAdvancedManifestInput {
                            code: code_blob,
                            definition: engine::manifest_decode(&definition)?,
                            metadata: metadata.to_native()?,
                            owner_role: owner_role.to_native()?,
                            package_address: address_reservation
                        }
                    ),
                });
            builder.instructions.push(instruction);
            Ok(())
        })
    }

    pub fn faucet_free_xrd(self: Arc<Self>) -> Result<Arc<Self>> {
        builder_arc_map(self, |builder| {
            let instruction =
                engine::InstructionV1::CallMethod(engine::CallMethod {
                    address: engine::DynamicGlobalAddress::Static(
                        engine::FAUCET.into(),
                    ),
                    method_name: "free".to_owned(),
                    args: manifest_args!().into(),
                });
            builder.instructions.push(instruction);
            Ok(())
        })
    }

    pub fn faucet_lock_fee(self: Arc<Self>) -> Result<Arc<Self>> {
        builder_arc_map(self, |builder| {
            let instruction =
                engine::InstructionV1::CallMethod(engine::CallMethod {
                    address: engine::DynamicGlobalAddress::Static(
                        engine::FAUCET.into(),
                    ),
                    method_name: "lock_fee".to_owned(),
                    args: manifest_args!(engine::dec!("100")).into(),
                });
            builder.instructions.push(instruction);
            Ok(())
        })
    }

    pub fn access_controller_new_from_public_keys(
        self: Arc<Self>,
        controlled_asset: ManifestBuilderBucket,
        primary_role: PublicKey,
        recovery_role: PublicKey,
        confirmation_role: PublicKey,
        timed_recovery_delay_in_minutes: Option<u32>,
        address_reservation: Option<ManifestBuilderAddressReservation>,
    ) -> Result<Arc<Self>> {
        builder_arc_map(self, |builder| {
            let bucket =
                builder.name_record.get_bucket(&controlled_asset.name)?;
            let address_reservation = match address_reservation {
                Some(reservation) => Some(
                    builder
                        .name_record
                        .get_address_reservation(&reservation.name)?,
                ),
                None => None,
            };

            let rule_set = engine::RuleSet {
                primary_role: engine::rule!(require(
                    engine::NonFungibleGlobalId::from_public_key(
                        &engine::PublicKey::try_from(primary_role)?
                    )
                )),
                recovery_role: engine::rule!(require(
                    engine::NonFungibleGlobalId::from_public_key(
                        &engine::PublicKey::try_from(recovery_role)?
                    )
                )),
                confirmation_role: engine::rule!(require(
                    engine::NonFungibleGlobalId::from_public_key(
                        &engine::PublicKey::try_from(confirmation_role)?
                    )
                )),
            };

            let instruction =
                engine::InstructionV1::CallFunction(engine::CallFunction {
                    package_address: engine::DynamicPackageAddress::Static(
                        engine::ACCESS_CONTROLLER_PACKAGE,
                    ),
                    blueprint_name: engine::ACCESS_CONTROLLER_BLUEPRINT
                        .to_owned(),
                    function_name: engine::ACCESS_CONTROLLER_CREATE_IDENT
                        .to_owned(),
                    args: manifest_args!(
                        bucket,
                        rule_set,
                        timed_recovery_delay_in_minutes,
                        address_reservation
                    )
                    .into(),
                });
            builder.instructions.push(instruction);
            Ok(())
        })
    }

    pub fn access_controller_create_with_security_structure(
        self: Arc<Self>,
        controlled_asset: ManifestBuilderBucket,
        primary_role: SecurityStructureRole,
        recovery_role: SecurityStructureRole,
        confirmation_role: SecurityStructureRole,
        timed_recovery_delay_in_minutes: Option<u32>,
        address_reservation: Option<ManifestBuilderAddressReservation>,
    ) -> Result<Arc<Self>> {
        builder_arc_map(self, |builder| {
            let bucket =
                builder.name_record.get_bucket(&controlled_asset.name)?;
            let address_reservation = match address_reservation {
                Some(reservation) => Some(
                    builder
                        .name_record
                        .get_address_reservation(&reservation.name)?,
                ),
                None => None,
            };

            let rule_set = engine::RuleSet {
                primary_role: engine::AccessRule::try_from(primary_role)?,
                recovery_role: engine::AccessRule::try_from(recovery_role)?,
                confirmation_role: engine::AccessRule::try_from(
                    confirmation_role,
                )?,
            };

            let instruction =
                engine::InstructionV1::CallFunction(engine::CallFunction {
                    package_address: engine::DynamicPackageAddress::Static(
                        engine::ACCESS_CONTROLLER_PACKAGE,
                    ),
                    blueprint_name: engine::ACCESS_CONTROLLER_BLUEPRINT
                        .to_owned(),
                    function_name: engine::ACCESS_CONTROLLER_CREATE_IDENT
                        .to_owned(),
                    args: manifest_args!(
                        bucket,
                        rule_set,
                        timed_recovery_delay_in_minutes,
                        address_reservation
                    )
                    .into(),
                });
            builder.instructions.push(instruction);
            Ok(())
        })
    }

    /* Resources */

    pub fn create_fungible_resource_manager(
        self: Arc<Self>,
        owner_role: OwnerRole,
        track_total_supply: bool,
        divisibility: u8,
        initial_supply: Option<Arc<Decimal>>,
        resource_roles: FungibleResourceRoles,
        metadata: MetadataModuleConfig,
        address_reservation: Option<ManifestBuilderAddressReservation>,
    ) -> Result<Arc<Self>> {
        builder_arc_map(self, |builder| {
            let owner_role = owner_role.to_native()?;
            let resource_roles = resource_roles.to_native()?;
            let metadata = metadata.to_native()?;
            let address_reservation = if let Some(value) = address_reservation {
                Some(value.to_native(&builder.name_record)?)
            } else {
                None
            };

            let (function_name, args) = if let Some(initial_supply) =
                initial_supply
            {
                (
                    engine::FUNGIBLE_RESOURCE_MANAGER_CREATE_WITH_INITIAL_SUPPLY_IDENT,
                    engine::to_manifest_value_and_unwrap!(
                        &engine::FungibleResourceManagerCreateWithInitialSupplyManifestInput {
                            owner_role,
                            track_total_supply,
                            divisibility,
                            initial_supply: initial_supply.0,
                            resource_roles,
                            metadata,
                            address_reservation
                        }
                    ),
                )
            } else {
                (
                    engine::FUNGIBLE_RESOURCE_MANAGER_CREATE_IDENT,
                    engine::to_manifest_value_and_unwrap!(
                        &engine::FungibleResourceManagerCreateManifestInput {
                            owner_role,
                            track_total_supply,
                            divisibility,
                            resource_roles,
                            metadata,
                            address_reservation
                        }
                    ),
                )
            };

            let instruction =
                engine::InstructionV1::CallFunction(engine::CallFunction {
                    package_address: engine::DynamicPackageAddress::Static(
                        engine::RESOURCE_PACKAGE,
                    ),
                    blueprint_name: engine::FUNGIBLE_RESOURCE_MANAGER_BLUEPRINT
                        .to_owned(),
                    function_name: function_name.to_owned(),
                    args,
                });
            builder.instructions.push(instruction);
            Ok(())
        })
    }

    pub fn mint_fungible(
        self: Arc<Self>,
        resource_address: Arc<Address>,
        amount: Arc<Decimal>,
    ) -> Result<Arc<Self>> {
        builder_arc_map(self, |builder| {
            let resource_address =
                engine::ResourceAddress::try_from(*resource_address)?;
            let amount = amount.0;

            let instruction =
                engine::InstructionV1::CallMethod(engine::CallMethod {
                    address: engine::DynamicGlobalAddress::Static(
                        resource_address.into(),
                    ),
                    method_name: engine::FUNGIBLE_RESOURCE_MANAGER_MINT_IDENT
                        .to_owned(),
                    args: engine::to_manifest_value_and_unwrap!(
                        &engine::FungibleResourceManagerMintManifestInput {
                            amount
                        }
                    ),
                });
            builder.instructions.push(instruction);
            Ok(())
        })
    }

    //=================
    // Builder Methods
    //=================

    pub fn build(
        self: Arc<Self>,
        network_id: u8,
    ) -> Arc<TransactionManifestV1> {
        Arc::new(TransactionManifestV1 {
            instructions: Arc::new(InstructionsV1(
                self.instructions.clone(),
                network_id,
            )),
            blobs: self.blobs.clone(),
        })
    }
}

#[derive(Debug, Clone, Record)]
pub struct SecurityStructureRole {
    pub super_admin_factors: Vec<PublicKey>,
    pub threshold_factors: Vec<PublicKey>,
    pub threshold: u8,
}

impl TryFrom<SecurityStructureRole> for engine::AccessRule {
    type Error = RadixEngineToolkitError;

    fn try_from(
        value: SecurityStructureRole,
    ) -> std::result::Result<Self, Self::Error> {
        let super_admin_factors = value
            .super_admin_factors
            .into_iter()
            .map(|pk| {
                engine::PublicKey::try_from(pk)
                    .map(|pk| engine::NonFungibleGlobalId::from_public_key(&pk))
                    .map(engine::ResourceOrNonFungible::NonFungible)
            })
            .collect::<Result<Vec<engine::ResourceOrNonFungible>>>()?;
        let threshold_factors = value
            .threshold_factors
            .into_iter()
            .map(|pk| {
                engine::PublicKey::try_from(pk)
                    .map(|pk| engine::NonFungibleGlobalId::from_public_key(&pk))
                    .map(engine::ResourceOrNonFungible::NonFungible)
            })
            .collect::<Result<Vec<engine::ResourceOrNonFungible>>>()?;

        Ok(engine::AccessRule::Protected(
            engine::CompositeRequirement::AnyOf(vec![
                engine::CompositeRequirement::BasicRequirement(
                    engine::BasicRequirement::CountOf(
                        value.threshold,
                        threshold_factors,
                    ),
                ),
                engine::CompositeRequirement::BasicRequirement(
                    engine::BasicRequirement::AnyOf(super_admin_factors),
                ),
            ]),
        ))
    }
}

macro_rules! manifest_args {
    ($($args: expr),*$(,)?) => {{
        use ::sbor::Encoder;
        let mut buf = ::sbor::rust::vec::Vec::new();
        let mut encoder = radix_common::data::manifest::ManifestEncoder::new(
            &mut buf,
            radix_common::data::manifest::MANIFEST_SBOR_V1_MAX_DEPTH
        );
        encoder.write_payload_prefix(
            radix_common::data::manifest::MANIFEST_SBOR_V1_PAYLOAD_PREFIX
        ).unwrap();
        encoder.write_value_kind(
            radix_common::data::manifest::ManifestValueKind::Tuple
        ).unwrap();
        // Hack: stringify to skip ownership move semantics
        encoder.write_size(radix_common::count!($(stringify!($args)),*)).unwrap();
        $(
            let arg = $args;
            encoder.encode(&arg).unwrap();
        )*
        let value = radix_common::data::manifest::manifest_decode(&buf).unwrap();
        radix_common::data::manifest::ManifestArgs::new_from_tuple_or_panic(value)
    }};
}
use manifest_args;

/// This macro defines a simple DSL for adding aliases to method and function
/// calls to the manifest builder without the need to manually author and
/// maintain the relatively large amount of boiler- plate code needed for it.
macro_rules! builder_alias {
    (
        $(
            { $($tokens: tt)* }
        ),* $(,)?
    ) => {
        $(builder_alias!( $($tokens)* );)*
    };
    (
        builder_method: $builder_method: ident,
        method_ident: $method_ident: expr,
        instruction: $instruction: ident,
        args: $input_type: ident {
            $(
                $arg_name: ident: ( $interface_arg_type: ty => $underlying_arg_type: ty )
            ),* $(,)?
        } $(,)?
    ) => {
        builder_alias_internal! {
            builder_method: $builder_method,
            method_ident: $method_ident,
            instruction: $instruction,
            input_type: $input_type,
            args: [
                $(
                    {
                        interface_arg_name: $arg_name,
                        interface_arg_type: $interface_arg_type,
                        input_arg_name: $arg_name,
                        input_arg_type: $underlying_arg_type,
                    }
                ),*
            ]
        }
    };
    (
        builder_method: $builder_method: ident,
        method_ident: $method_ident: expr,
        instruction: $instruction: ident,
        args: $input_type: ident {
            $(
                $arg_name: ident as $arg_name_alias: ident : ( $interface_arg_type: ty => $underlying_arg_type: ty )
            ),* $(,)?
        } $(,)?
    ) => {
        builder_alias_internal! {
            builder_method: $builder_method,
            method_ident: $method_ident,
            instruction: $instruction,
            input_type: $input_type,
            args: [
                $(
                    {
                        interface_arg_name: $arg_name_alias,
                        interface_arg_type: $interface_arg_type,
                        input_arg_name: $arg_name,
                        input_arg_type: $underlying_arg_type,
                    }
                ),*
            ]
        }
    };
    (
        builder_method: $builder_method: ident,
        package_address: $package_address: expr,
        blueprint_ident: $blueprint_ident: expr,
        function_ident: $function_ident: expr,
        args: $input_ident: ident {
            $(
                $arg_name: ident: ( $interface_arg_type: ty => $underlying_arg_type: ty )
            ),* $(,)?
        } $(,)?
    ) => {
        builder_alias_internal! {
            builder_method: $builder_method,
            package_address: $package_address,
            blueprint_ident: $blueprint_ident,
            function_ident: $function_ident,
            input_type: $input_ident,
            args: [
                $(
                    {
                        interface_arg_name: $arg_name,
                        interface_arg_type: $interface_arg_type,
                        input_arg_name: $arg_name,
                        input_arg_type: $underlying_arg_type,
                    }
                ),*
            ]
        }
    };
    (
        builder_method: $builder_method: ident,
        package_address: $package_address: expr,
        blueprint_ident: $blueprint_ident: expr,
        function_ident: $function_ident: expr,
        args: $input_ident: ident {
            $(
                $arg_name: ident as $arg_name_alias: ident: ( $interface_arg_type: ty => $underlying_arg_type: ty )
            ),* $(,)?
        } $(,)?
    ) => {
        builder_alias_internal! {
            builder_method: $builder_method,
            package_address: $package_address,
            blueprint_ident: $blueprint_ident,
            function_ident: $function_ident,
            input_type: $input_ident,
            args: [
                $(
                    {
                        interface_arg_name: $arg_name_alias,
                        interface_arg_type: $interface_arg_type,
                        input_arg_name: $arg_name,
                        input_arg_type: $underlying_arg_type,
                    }
                ),*
            ]
        }
    };
}

macro_rules! builder_alias_internal {
    (
        builder_method: $builder_method: ident,
        method_ident: $method_ident: expr,
        instruction: $instruction: ident,
        input_type: $input_type: ident,
        args: [
            $(
                {
                    /* Interface */
                    interface_arg_name: $interface_arg_name: ident,
                    interface_arg_type: $interface_arg_type: ty,
                    /* Input */
                    input_arg_name: $input_arg_name: ident,
                    input_arg_type: $input_arg_type: ty $(,)?
                }
            ),* $(,)?
        ] $(,)?
    ) => {
        paste::paste! {
            #[uniffi::export]
            impl ManifestV1Builder {
                pub fn $builder_method(
                    self: $crate::prelude::Arc<Self>,
                    address: $crate::prelude::Arc<$crate::prelude::Address>,
                    $(
                        $interface_arg_name: $interface_arg_type
                    ),*
                ) -> $crate::prelude::Result<Arc<Self>> {
                    $crate::builder::manifest_builder::utils::builder_arc_map(self, |builder| {
                        let instruction = $crate::prelude::engine::InstructionV1::$instruction( engine::[< $instruction >]{
                            address: $crate::prelude::engine::DynamicGlobalAddress::Static((*address).try_into()?),
                            method_name: $method_ident.to_owned(),
                            args: $crate::prelude::engine::to_manifest_value_and_unwrap! {
                                &engine::$input_type {
                                    $(
                                        $input_arg_name: <
                                            $input_arg_type
                                            as $crate::builder::manifest_builder::traits::FromWithNameRecordContext<$interface_arg_type>
                                        >::from($interface_arg_name, &builder.name_record)?
                                    ),*
                                }
                            }
                        });
                        builder.instructions.push(instruction);
                        Ok(())
                    })
                }
            }
        }
    };
    (
        builder_method: $builder_method: ident,
        package_address: $package_address: expr,
        blueprint_ident: $blueprint_ident: expr,
        function_ident: $function_ident: expr,
        input_type: $input_type: ident,
        args: [
            $(
                {
                    /* Interface */
                    interface_arg_name: $interface_arg_name: ident,
                    interface_arg_type: $interface_arg_type: ty,
                    /* Input */
                    input_arg_name: $input_arg_name: ident,
                    input_arg_type: $input_arg_type: ty $(,)?
                }
            ),* $(,)?
        ] $(,)?
    ) => {
        #[uniffi::export]
        impl ManifestV1Builder {
            pub fn $builder_method(
                self: $crate::prelude::Arc<Self>,
                $(
                    $interface_arg_name: $interface_arg_type
                ),*
            ) -> $crate::prelude::Result<Arc<Self>> {
                $crate::builder::manifest_builder::utils::builder_arc_map(self, |builder| {
                    let instruction = $crate::prelude::engine::InstructionV1::CallFunction(engine::CallFunction {
                        package_address: $crate::prelude::engine::DynamicPackageAddress::Static($package_address),
                        blueprint_name: $blueprint_ident.to_owned(),
                        function_name: $function_ident.to_owned(),
                        args: $crate::prelude::engine::to_manifest_value_and_unwrap! {
                            &engine::$input_type {
                                $(
                                    $input_arg_name: <
                                        $input_arg_type
                                        as $crate::builder::manifest_builder::traits::FromWithNameRecordContext<$interface_arg_type>
                                    >::from($interface_arg_name, &builder.name_record)?
                                ),*
                            }
                        }
                    });
                    builder.instructions.push(instruction);
                    Ok(())
                })
            }
        }
    }
}

builder_alias! {
    // ========
    // Account
    // ========
    {
        builder_method: account_create_advanced,
        package_address: engine::ACCOUNT_PACKAGE,
        blueprint_ident: engine::ACCOUNT_BLUEPRINT,
        function_ident: engine::ACCOUNT_CREATE_ADVANCED_IDENT,
        args: AccountCreateAdvancedManifestInput {
            owner_role: (OwnerRole => engine::OwnerRole),
            address_reservation: (
                Option<ManifestBuilderAddressReservation>
                    => Option<engine::ManifestAddressReservation>
            )
        }
    },
    {
        builder_method: account_create,
        package_address: engine::ACCOUNT_PACKAGE,
        blueprint_ident: engine::ACCOUNT_BLUEPRINT,
        function_ident: engine::ACCOUNT_CREATE_IDENT,
        args: AccountCreateManifestInput {}
    },
    {
        builder_method: account_securify,
        method_ident: engine::ACCOUNT_SECURIFY_IDENT,
        instruction: CallMethod,
        args: AccountSecurifyManifestInput {}
    },
    {
        builder_method: account_lock_fee,
        method_ident: engine::ACCOUNT_LOCK_FEE_IDENT,
        instruction: CallMethod,
        args: AccountLockFeeManifestInput {
            amount: (Arc<Decimal> => engine::Decimal)
        }
    },
    {
        builder_method: account_lock_contingent_fee,
        method_ident: engine::ACCOUNT_LOCK_CONTINGENT_FEE_IDENT,
        instruction: CallMethod,
        args: AccountLockContingentFeeManifestInput {
            amount: (Arc<Decimal> => engine::Decimal)
        }
    },
    {
        builder_method: account_deposit,
        method_ident: engine::ACCOUNT_DEPOSIT_IDENT,
        instruction: CallMethod,
        args: AccountDepositManifestInput {
            bucket: (ManifestBuilderBucket => engine::ManifestBucket)
        }
    },
    {
        builder_method: account_try_deposit_or_abort,
        method_ident: engine::ACCOUNT_TRY_DEPOSIT_OR_ABORT_IDENT,
        instruction: CallMethod,
        args: AccountTryDepositOrAbortManifestInput {
            bucket: (ManifestBuilderBucket => engine::ManifestBucket),
            authorized_depositor_badge: (Option<ResourceOrNonFungible> => Option<engine::ManifestResourceOrNonFungible>),
        }
    },
    {
        builder_method: account_try_deposit_or_refund,
        method_ident: engine::ACCOUNT_TRY_DEPOSIT_OR_REFUND_IDENT,
        instruction: CallMethod,
        args: AccountTryDepositOrRefundManifestInput {
            bucket: (ManifestBuilderBucket => engine::ManifestBucket),
            authorized_depositor_badge: (Option<ResourceOrNonFungible> => Option<engine::ManifestResourceOrNonFungible>),
        }
    },
    {
        builder_method: account_deposit_batch,
        method_ident: engine::ACCOUNT_DEPOSIT_BATCH_IDENT,
        instruction: CallMethod,
        args: AccountDepositBatchManifestInput {
            buckets: (Vec<ManifestBuilderBucket> => engine::ManifestBucketBatch)
        }
    },
    {
        builder_method: account_try_deposit_batch_or_abort,
        method_ident: engine::ACCOUNT_TRY_DEPOSIT_BATCH_OR_ABORT_IDENT,
        instruction: CallMethod,
        args: AccountTryDepositBatchOrAbortManifestInput {
            buckets: (Vec<ManifestBuilderBucket> => engine::ManifestBucketBatch),
            authorized_depositor_badge: (Option<ResourceOrNonFungible> => Option<engine::ManifestResourceOrNonFungible>),
        }
    },
    {
        builder_method: account_try_deposit_batch_or_refund,
        method_ident: engine::ACCOUNT_TRY_DEPOSIT_BATCH_OR_REFUND_IDENT,
        instruction: CallMethod,
        args: AccountTryDepositBatchOrRefundManifestInput {
            buckets: (Vec<ManifestBuilderBucket> => engine::ManifestBucketBatch),
            authorized_depositor_badge: (Option<ResourceOrNonFungible> => Option<engine::ManifestResourceOrNonFungible>),
        }
    },
    {
        builder_method: account_withdraw,
        method_ident: engine::ACCOUNT_WITHDRAW_IDENT,
        instruction: CallMethod,
        args: AccountWithdrawManifestInput {
            resource_address: (Arc<Address> => engine::ManifestResourceAddress),
            amount: (Arc<Decimal> => engine::Decimal),
        }
    },
    {
        builder_method: account_withdraw_non_fungibles,
        method_ident: engine::ACCOUNT_WITHDRAW_NON_FUNGIBLES_IDENT,
        instruction: CallMethod,
        args: AccountWithdrawNonFungiblesManifestInput {
            resource_address: (Arc<Address> => engine::ManifestResourceAddress),
            ids: (Vec<NonFungibleLocalId> => engine::IndexSet<engine::NonFungibleLocalId>),
        }
    },
    {
        builder_method: account_lock_fee_and_withdraw,
        method_ident: engine::ACCOUNT_LOCK_FEE_AND_WITHDRAW_IDENT,
        instruction: CallMethod,
        args: AccountLockFeeAndWithdrawManifestInput {
            amount_to_lock: (Arc<Decimal> => engine::Decimal),
            resource_address: (Arc<Address> => engine::ManifestResourceAddress),
            amount: (Arc<Decimal> => engine::Decimal),
        }
    },
    {
        builder_method: account_lock_fee_and_withdraw_non_fungibles,
        method_ident: engine::ACCOUNT_LOCK_FEE_AND_WITHDRAW_NON_FUNGIBLES_IDENT,
        instruction: CallMethod,
        args: AccountLockFeeAndWithdrawNonFungiblesManifestInput {
            amount_to_lock: (Arc<Decimal> => engine::Decimal),
            resource_address: (Arc<Address> => engine::ManifestResourceAddress),
            ids: (Vec<NonFungibleLocalId> => engine::IndexSet<engine::NonFungibleLocalId>),
        }
    },
    {
        builder_method: account_create_proof_of_amount,
        method_ident: engine::ACCOUNT_CREATE_PROOF_OF_AMOUNT_IDENT,
        instruction: CallMethod,
        args: AccountCreateProofOfAmountManifestInput {
            resource_address: (Arc<Address> => engine::ManifestResourceAddress),
            amount: (Arc<Decimal> => engine::Decimal),
        }
    },
    {
        builder_method: account_create_proof_of_non_fungibles,
        method_ident: engine::ACCOUNT_CREATE_PROOF_OF_NON_FUNGIBLES_IDENT,
        instruction: CallMethod,
        args: AccountCreateProofOfNonFungiblesManifestInput {
            resource_address: (Arc<Address> => engine::ManifestResourceAddress),
            ids: (Vec<NonFungibleLocalId> => engine::IndexSet<engine::NonFungibleLocalId>),
        }
    },
    {
        builder_method: account_set_default_deposit_rule,
        method_ident: engine::ACCOUNT_SET_DEFAULT_DEPOSIT_RULE_IDENT,
        instruction: CallMethod,
        args: AccountSetDefaultDepositRuleManifestInput {
            default as default_deposit_rule: (AccountDefaultDepositRule => engine::DefaultDepositRule),
        }
    },
    {
        builder_method: account_set_resource_preference,
        method_ident: engine::ACCOUNT_SET_RESOURCE_PREFERENCE_IDENT,
        instruction: CallMethod,
        args: AccountSetResourcePreferenceManifestInput {
            resource_address: (Arc<Address> => engine::ManifestResourceAddress),
            resource_preference: (ResourcePreference => engine::ResourcePreference),
        }
    },
    {
        builder_method: account_remove_resource_preference,
        method_ident: engine::ACCOUNT_REMOVE_RESOURCE_PREFERENCE_IDENT,
        instruction: CallMethod,
        args: AccountRemoveResourcePreferenceManifestInput {
            resource_address: (Arc<Address> => engine::ManifestResourceAddress),
        }
    },
    {
        builder_method: account_burn,
        method_ident: engine::ACCOUNT_BURN_IDENT,
        instruction: CallMethod,
        args: AccountBurnManifestInput {
            resource_address: (Arc<Address> => engine::ManifestResourceAddress),
            amount: (Arc<Decimal> => engine::Decimal),
        }
    },
    {
        builder_method: account_burn_non_fungibles,
        method_ident: engine::ACCOUNT_BURN_NON_FUNGIBLES_IDENT,
        instruction: CallMethod,
        args: AccountBurnNonFungiblesManifestInput {
            resource_address: (Arc<Address> => engine::ManifestResourceAddress),
            ids: (Vec<NonFungibleLocalId> => engine::IndexSet<engine::NonFungibleLocalId>),
        }
    },
    {
        builder_method: account_add_authorized_depositor,
        method_ident: engine::ACCOUNT_ADD_AUTHORIZED_DEPOSITOR_IDENT,
        instruction: CallMethod,
        args: AccountAddAuthorizedDepositorManifestInput {
            badge: (ResourceOrNonFungible => engine::ManifestResourceOrNonFungible),
        }
    },
    {
        builder_method: account_remove_authorized_depositor,
        method_ident: engine::ACCOUNT_REMOVE_AUTHORIZED_DEPOSITOR_IDENT,
        instruction: CallMethod,
        args: AccountRemoveAuthorizedDepositorManifestInput {
            badge: (ResourceOrNonFungible => engine::ManifestResourceOrNonFungible),
        }
    },
    // ==========
    // Validator
    // ==========
    {
        builder_method: validator_register,
        method_ident: engine::VALIDATOR_REGISTER_IDENT,
        instruction: CallMethod,
        args: ValidatorRegisterManifestInput {}
    },
    {
        builder_method: validator_unregister,
        method_ident: engine::VALIDATOR_UNREGISTER_IDENT,
        instruction: CallMethod,
        args: ValidatorUnregisterManifestInput {}
    },
    {
        builder_method: validator_stake_as_owner,
        method_ident: engine::VALIDATOR_STAKE_AS_OWNER_IDENT,
        instruction: CallMethod,
        args: ValidatorStakeAsOwnerManifestInput {
            stake: (ManifestBuilderBucket => engine::ManifestBucket)
        }
    },
    {
        builder_method: validator_stake,
        method_ident: engine::VALIDATOR_STAKE_IDENT,
        instruction: CallMethod,
        args: ValidatorStakeManifestInput {
            stake: (ManifestBuilderBucket => engine::ManifestBucket)
        }
    },
    {
        builder_method: validator_unstake,
        method_ident: engine::VALIDATOR_UNSTAKE_IDENT,
        instruction: CallMethod,
        args: ValidatorUnstakeManifestInput {
            stake_unit_bucket: (ManifestBuilderBucket => engine::ManifestBucket)
        }
    },
    {
        builder_method: validator_claim_xrd,
        method_ident: engine::VALIDATOR_CLAIM_XRD_IDENT,
        instruction: CallMethod,
        args: ValidatorClaimXrdManifestInput {
            bucket: (ManifestBuilderBucket => engine::ManifestBucket)
        }
    },
    {
        builder_method: validator_update_key,
        method_ident: engine::VALIDATOR_UPDATE_KEY_IDENT,
        instruction: CallMethod,
        args: ValidatorUpdateKeyManifestInput {
            key: (PublicKey => engine::Secp256k1PublicKey)
        }
    },
    {
        builder_method: validator_update_fee,
        method_ident: engine::VALIDATOR_UPDATE_FEE_IDENT,
        instruction: CallMethod,
        args: ValidatorUpdateFeeManifestInput {
            new_fee_factor: (Arc<Decimal> => engine::Decimal)
        }
    },
    {
        builder_method: validator_update_accept_delegated_stake,
        method_ident: engine::VALIDATOR_UPDATE_ACCEPT_DELEGATED_STAKE_IDENT,
        instruction: CallMethod,
        args: ValidatorUpdateAcceptDelegatedStakeManifestInput {
            accept_delegated_stake: (bool => bool)
        }
    },
    {
        builder_method: validator_accepts_delegated_stake,
        method_ident: engine::VALIDATOR_ACCEPTS_DELEGATED_STAKE_IDENT,
        instruction: CallMethod,
        args: ValidatorAcceptsDelegatedStakeManifestInput {}
    },
    {
        builder_method: validator_total_stake_xrd_amount,
        method_ident: engine::VALIDATOR_TOTAL_STAKE_XRD_AMOUNT_IDENT,
        instruction: CallMethod,
        args: ValidatorTotalStakeXrdAmountManifestInput {}
    },
    {
        builder_method: validator_total_stake_unit_supply,
        method_ident: engine::VALIDATOR_TOTAL_STAKE_UNIT_SUPPLY_IDENT,
        instruction: CallMethod,
        args: ValidatorTotalStakeUnitSupplyManifestInput {}
    },
    {
        builder_method: validator_get_redemption_value,
        method_ident: engine::VALIDATOR_GET_REDEMPTION_VALUE_IDENT,
        instruction: CallMethod,
        args: ValidatorGetRedemptionValueManifestInput {
            amount_of_stake_units: (Arc<Decimal> => engine::Decimal)
        }
    },
    {
        builder_method: validator_signal_protocol_update_readiness,
        method_ident: engine::VALIDATOR_SIGNAL_PROTOCOL_UPDATE_READINESS_IDENT,
        instruction: CallMethod,
        args: ValidatorSignalProtocolUpdateReadinessManifestInput {
            vote: (String => String)
        }
    },
    {
        builder_method: validator_get_protocol_update_readiness,
        method_ident: engine::VALIDATOR_GET_PROTOCOL_UPDATE_READINESS_IDENT,
        instruction: CallMethod,
        args: ValidatorGetProtocolUpdateReadinessManifestInput {}
    },
    {
        builder_method: validator_lock_owner_stake_units,
        method_ident: engine::VALIDATOR_LOCK_OWNER_STAKE_UNITS_IDENT,
        instruction: CallMethod,
        args: ValidatorLockOwnerStakeUnitsManifestInput {
            stake_unit_bucket: (ManifestBuilderBucket => engine::ManifestBucket)
        }
    },
    {
        builder_method: validator_start_unlock_owner_stake_units,
        method_ident: engine::VALIDATOR_START_UNLOCK_OWNER_STAKE_UNITS_IDENT,
        instruction: CallMethod,
        args: ValidatorStartUnlockOwnerStakeUnitsManifestInput {
            requested_stake_unit_amount: (Arc<Decimal> => engine::Decimal)
        }
    },
    {
        builder_method: validator_finish_unlock_owner_stake_units,
        method_ident: engine::VALIDATOR_FINISH_UNLOCK_OWNER_STAKE_UNITS_IDENT,
        instruction: CallMethod,
        args: ValidatorFinishUnlockOwnerStakeUnitsManifestInput {}
    },
    // ==================
    // Access Controller
    // ==================
    {
        builder_method: access_controller_create,
        package_address: engine::ACCESS_CONTROLLER_PACKAGE,
        blueprint_ident: engine::ACCESS_CONTROLLER_BLUEPRINT,
        function_ident: engine::ACCESS_CONTROLLER_CREATE_IDENT,
        args: AccessControllerCreateManifestInput {
            controlled_asset: (ManifestBuilderBucket => engine::ManifestBucket),
            rule_set: (RuleSet => engine::RuleSet),
            timed_recovery_delay_in_minutes: (Option<u32> => Option<u32>),
            address_reservation: (Option<ManifestBuilderAddressReservation> => Option<engine::ManifestAddressReservation>)
        }
    },
    {
        builder_method: access_controller_create_proof,
        method_ident: engine::ACCESS_CONTROLLER_CREATE_PROOF_IDENT,
        instruction: CallMethod,
        args: AccessControllerCreateProofManifestInput {}
    },
    {
        builder_method: access_controller_initiate_recovery_as_primary,
        method_ident: engine::ACCESS_CONTROLLER_INITIATE_RECOVERY_AS_PRIMARY_IDENT,
        instruction: CallMethod,
        args: AccessControllerInitiateRecoveryAsPrimaryManifestInput {
            rule_set: (RuleSet => engine::RuleSet),
            timed_recovery_delay_in_minutes: (Option<u32> => Option<u32>),
        }
    },
    {
        builder_method: access_controller_initiate_recovery_as_recovery,
        method_ident: engine::ACCESS_CONTROLLER_INITIATE_RECOVERY_AS_RECOVERY_IDENT,
        instruction: CallMethod,
        args: AccessControllerInitiateRecoveryAsRecoveryManifestInput {
            rule_set: (RuleSet => engine::RuleSet),
            timed_recovery_delay_in_minutes: (Option<u32> => Option<u32>),
        }
    },
    {
        builder_method: access_controller_initiate_badge_withdraw_as_primary,
        method_ident: engine::ACCESS_CONTROLLER_INITIATE_BADGE_WITHDRAW_ATTEMPT_AS_PRIMARY_IDENT,
        instruction: CallMethod,
        args: AccessControllerInitiateBadgeWithdrawAttemptAsPrimaryManifestInput {}
    },
    {
        builder_method: access_controller_initiate_badge_withdraw_as_recovery,
        method_ident: engine::ACCESS_CONTROLLER_INITIATE_BADGE_WITHDRAW_ATTEMPT_AS_RECOVERY_IDENT,
        instruction: CallMethod,
        args: AccessControllerInitiateBadgeWithdrawAttemptAsRecoveryManifestInput {}
    },
    {
        builder_method: access_controller_quick_confirm_primary_role_recovery_proposal,
        method_ident: engine::ACCESS_CONTROLLER_QUICK_CONFIRM_PRIMARY_ROLE_RECOVERY_PROPOSAL_IDENT,
        instruction: CallMethod,
        args: AccessControllerQuickConfirmPrimaryRoleRecoveryProposalManifestInput {
            rule_set: (RuleSet => engine::RuleSet),
            timed_recovery_delay_in_minutes: (Option<u32> => Option<u32>),
        }
    },
    {
        builder_method: access_controller_quick_confirm_recovery_role_recovery_proposal,
        method_ident: engine::ACCESS_CONTROLLER_QUICK_CONFIRM_RECOVERY_ROLE_RECOVERY_PROPOSAL_IDENT,
        instruction: CallMethod,
        args: AccessControllerQuickConfirmRecoveryRoleRecoveryProposalManifestInput {
            rule_set: (RuleSet => engine::RuleSet),
            timed_recovery_delay_in_minutes: (Option<u32> => Option<u32>),
        }
    },
    {
        builder_method: access_controller_quick_confirm_primary_role_badge_withdraw_attempt,
        method_ident: engine::ACCESS_CONTROLLER_QUICK_CONFIRM_PRIMARY_ROLE_BADGE_WITHDRAW_ATTEMPT_IDENT,
        instruction: CallMethod,
        args: AccessControllerQuickConfirmPrimaryRoleBadgeWithdrawAttemptManifestInput {}
    },
    {
        builder_method: access_controller_quick_confirm_recovery_role_badge_withdraw_attempt,
        method_ident: engine::ACCESS_CONTROLLER_QUICK_CONFIRM_RECOVERY_ROLE_BADGE_WITHDRAW_ATTEMPT_IDENT,
        instruction: CallMethod,
        args: AccessControllerQuickConfirmRecoveryRoleBadgeWithdrawAttemptManifestInput {}
    },
    {
        builder_method: access_controller_timed_confirm_recovery,
        method_ident: engine::ACCESS_CONTROLLER_TIMED_CONFIRM_RECOVERY_IDENT,
        instruction: CallMethod,
        args: AccessControllerTimedConfirmRecoveryManifestInput {
            rule_set: (RuleSet => engine::RuleSet),
            timed_recovery_delay_in_minutes: (Option<u32> => Option<u32>),
        }
    },
    {
        builder_method: access_controller_cancel_primary_role_recovery_proposal,
        method_ident: engine::ACCESS_CONTROLLER_CANCEL_PRIMARY_ROLE_RECOVERY_PROPOSAL_IDENT,
        instruction: CallMethod,
        args: AccessControllerCancelPrimaryRoleRecoveryProposalManifestInput {}
    },
    {
        builder_method: access_controller_cancel_recovery_role_recovery_proposal,
        method_ident: engine::ACCESS_CONTROLLER_CANCEL_RECOVERY_ROLE_RECOVERY_PROPOSAL_IDENT,
        instruction: CallMethod,
        args: AccessControllerCancelRecoveryRoleRecoveryProposalManifestInput {}
    },
    {
        builder_method: access_controller_cancel_primary_role_badge_withdraw_attempt,
        method_ident: engine::ACCESS_CONTROLLER_CANCEL_PRIMARY_ROLE_BADGE_WITHDRAW_ATTEMPT_IDENT,
        instruction: CallMethod,
        args: AccessControllerCancelPrimaryRoleBadgeWithdrawAttemptManifestInput {}
    },
    {
        builder_method: access_controller_cancel_recovery_role_badge_withdraw_attempt,
        method_ident: engine::ACCESS_CONTROLLER_CANCEL_RECOVERY_ROLE_BADGE_WITHDRAW_ATTEMPT_IDENT,
        instruction: CallMethod,
        args: AccessControllerCancelRecoveryRoleBadgeWithdrawAttemptManifestInput {}
    },
    {
        builder_method: access_controller_lock_primary_role,
        method_ident: engine::ACCESS_CONTROLLER_LOCK_PRIMARY_ROLE_IDENT,
        instruction: CallMethod,
        args: AccessControllerLockPrimaryRoleManifestInput {}
    },
    {
        builder_method: access_controller_unlock_primary_role,
        method_ident: engine::ACCESS_CONTROLLER_UNLOCK_PRIMARY_ROLE_IDENT,
        instruction: CallMethod,
        args: AccessControllerUnlockPrimaryRoleManifestInput {}
    },
    {
        builder_method: access_controller_stop_timed_recovery,
        method_ident: engine::ACCESS_CONTROLLER_STOP_TIMED_RECOVERY_IDENT,
        instruction: CallMethod,
        args: AccessControllerStopTimedRecoveryManifestInput {
            rule_set: (RuleSet => engine::RuleSet),
            timed_recovery_delay_in_minutes: (Option<u32> => Option<u32>),
        }
    },
    {
        builder_method: access_controller_mint_recovery_badges,
        method_ident: engine::ACCESS_CONTROLLER_MINT_RECOVERY_BADGES_IDENT,
        instruction: CallMethod,
        args: AccessControllerMintRecoveryBadgesManifestInput {
            non_fungible_local_ids: (Vec<NonFungibleLocalId> => engine::IndexSet<engine::NonFungibleLocalId>),
        }
    },
    // =========
    // Identity
    // =========
    {
        builder_method: identity_create_advanced,
        package_address: engine::IDENTITY_PACKAGE,
        blueprint_ident: engine::IDENTITY_BLUEPRINT,
        function_ident: engine::IDENTITY_CREATE_ADVANCED_IDENT,
        args: IdentityCreateAdvancedManifestInput {
            owner_role: (OwnerRole => engine::OwnerRole),
        }
    },
    {
        builder_method: identity_create,
        package_address: engine::IDENTITY_PACKAGE,
        blueprint_ident: engine::IDENTITY_BLUEPRINT,
        function_ident: engine::IDENTITY_CREATE_IDENT,
        args: IdentityCreateManifestInput {}
    },
    {
        builder_method: identity_securify,
        method_ident: engine::IDENTITY_SECURIFY_IDENT,
        instruction: CallMethod,
        args: IdentitySecurifyToSingleBadgeManifestInput {}
    },
    // ========
    // Package
    // ========
    {
        builder_method: package_claim_royalty,
        method_ident: engine::PACKAGE_CLAIM_ROYALTIES_IDENT,
        instruction: CallMethod,
        args: PackageClaimRoyaltiesManifestInput {}
    },
    // ==================
    // One Resource Pool
    // ==================
    {
        builder_method: one_resource_pool_instantiate,
        package_address: engine::POOL_PACKAGE,
        blueprint_ident: "OneResourcePool",
        function_ident: engine::ONE_RESOURCE_POOL_INSTANTIATE_IDENT,
        args: OneResourcePoolInstantiateManifestInput {
            owner_role: (OwnerRole => engine::OwnerRole),
            pool_manager_rule: (Arc<AccessRule> => engine::AccessRule),
            resource_address: (Arc<Address> => engine::ManifestResourceAddress),
            address_reservation: (Option<ManifestBuilderAddressReservation> => Option<engine::ManifestAddressReservation>)
        }
    },
    {
        builder_method: one_resource_pool_contribute,
        method_ident: engine::ONE_RESOURCE_POOL_CONTRIBUTE_IDENT,
        instruction: CallMethod,
        args: OneResourcePoolContributeManifestInput {
            bucket: (ManifestBuilderBucket => engine::ManifestBucket)
        }
    },
    {
        builder_method: one_resource_pool_redeem,
        method_ident: engine::ONE_RESOURCE_POOL_REDEEM_IDENT,
        instruction: CallMethod,
        args: OneResourcePoolRedeemManifestInput {
            bucket: (ManifestBuilderBucket => engine::ManifestBucket)
        }
    },
    {
        builder_method: one_resource_pool_protected_deposit,
        method_ident: engine::ONE_RESOURCE_POOL_PROTECTED_DEPOSIT_IDENT,
        instruction: CallMethod,
        args: OneResourcePoolProtectedDepositManifestInput {
            bucket: (ManifestBuilderBucket => engine::ManifestBucket)
        }
    },
    {
        builder_method: one_resource_pool_protected_withdraw,
        method_ident: engine::ONE_RESOURCE_POOL_PROTECTED_WITHDRAW_IDENT,
        instruction: CallMethod,
        args: OneResourcePoolProtectedWithdrawManifestInput {
            amount: (Arc<Decimal> => engine::Decimal),
            withdraw_strategy: (WithdrawStrategy => engine::WithdrawStrategy)
        }
    },
    {
        builder_method: one_resource_pool_get_redemption_value,
        method_ident: engine::ONE_RESOURCE_POOL_GET_REDEMPTION_VALUE_IDENT,
        instruction: CallMethod,
        args: OneResourcePoolGetRedemptionValueManifestInput {
            amount_of_pool_units: (Arc<Decimal> => engine::Decimal),
        }
    },
    {
        builder_method: one_resource_pool_get_vault_amount,
        method_ident: engine::ONE_RESOURCE_POOL_GET_VAULT_AMOUNT_IDENT,
        instruction: CallMethod,
        args: OneResourcePoolGetVaultAmountManifestInput {}
    },
    // ==================
    // Two Resource Pool
    // ==================
    {
        builder_method: two_resource_pool_instantiate,
        package_address: engine::POOL_PACKAGE,
        blueprint_ident: "TwoResourcePool",
        function_ident: engine::TWO_RESOURCE_POOL_INSTANTIATE_IDENT,
        args: TwoResourcePoolInstantiateManifestInput {
            owner_role: (OwnerRole => engine::OwnerRole),
            pool_manager_rule: (Arc<AccessRule> => engine::AccessRule),
            resource_addresses: (Vec<Arc<Address>> => (engine::ManifestResourceAddress, engine::ManifestResourceAddress)),
            address_reservation: (Option<ManifestBuilderAddressReservation> => Option<engine::ManifestAddressReservation>)
        }
    },
    {
        builder_method: two_resource_pool_contribute,
        method_ident: engine::TWO_RESOURCE_POOL_CONTRIBUTE_IDENT,
        instruction: CallMethod,
        args: TwoResourcePoolContributeManifestInput {
            buckets: (Vec<ManifestBuilderBucket> => (engine::ManifestBucket, engine::ManifestBucket))
        }
    },
    {
        builder_method: two_resource_pool_redeem,
        method_ident: engine::TWO_RESOURCE_POOL_REDEEM_IDENT,
        instruction: CallMethod,
        args: TwoResourcePoolRedeemManifestInput {
            bucket: (ManifestBuilderBucket => engine::ManifestBucket)
        }
    },
    {
        builder_method: two_resource_pool_protected_deposit,
        method_ident: engine::TWO_RESOURCE_POOL_PROTECTED_DEPOSIT_IDENT,
        instruction: CallMethod,
        args: TwoResourcePoolProtectedDepositManifestInput {
            bucket: (ManifestBuilderBucket => engine::ManifestBucket)
        }
    },
    {
        builder_method: two_resource_pool_protected_withdraw,
        method_ident: engine::TWO_RESOURCE_POOL_PROTECTED_WITHDRAW_IDENT,
        instruction: CallMethod,
        args: TwoResourcePoolProtectedWithdrawManifestInput {
            resource_address: (Arc<Address> => engine::ManifestResourceAddress),
            amount: (Arc<Decimal> => engine::Decimal),
            withdraw_strategy: (WithdrawStrategy => engine::WithdrawStrategy)
        }
    },
    {
        builder_method: two_resource_pool_get_redemption_value,
        method_ident: engine::TWO_RESOURCE_POOL_GET_REDEMPTION_VALUE_IDENT,
        instruction: CallMethod,
        args: TwoResourcePoolGetRedemptionValueManifestInput {
            amount_of_pool_units: (Arc<Decimal> => engine::Decimal),
        }
    },
    {
        builder_method: two_resource_pool_get_vault_amount,
        method_ident: engine::TWO_RESOURCE_POOL_GET_VAULT_AMOUNTS_IDENT,
        instruction: CallMethod,
        args: TwoResourcePoolGetVaultAmountsManifestInput {}
    },
    // ====================
    // Multi Resource Pool
    // ====================
    {
        builder_method: multi_resource_pool_instantiate,
        package_address: engine::POOL_PACKAGE,
        blueprint_ident: "MultiResourcePool",
        function_ident: engine::MULTI_RESOURCE_POOL_INSTANTIATE_IDENT,
        args: MultiResourcePoolInstantiateManifestInput {
            owner_role: (OwnerRole => engine::OwnerRole),
            pool_manager_rule: (Arc<AccessRule> => engine::AccessRule),
            resource_addresses: (Vec<Arc<Address>> => engine::IndexSet<engine::ManifestResourceAddress>),
            address_reservation: (Option<ManifestBuilderAddressReservation> => Option<engine::ManifestAddressReservation>)
        }
    },
    {
        builder_method: multi_resource_pool_contribute,
        method_ident: engine::MULTI_RESOURCE_POOL_CONTRIBUTE_IDENT,
        instruction: CallMethod,
        args: MultiResourcePoolContributeManifestInput {
            buckets: (Vec<ManifestBuilderBucket> => engine::ManifestBucketBatch)
        }
    },
    {
        builder_method: multi_resource_pool_redeem,
        method_ident: engine::MULTI_RESOURCE_POOL_REDEEM_IDENT,
        instruction: CallMethod,
        args: MultiResourcePoolRedeemManifestInput {
            bucket: (ManifestBuilderBucket => engine::ManifestBucket)
        }
    },
    {
        builder_method: multi_resource_pool_protected_deposit,
        method_ident: engine::MULTI_RESOURCE_POOL_PROTECTED_DEPOSIT_IDENT,
        instruction: CallMethod,
        args: MultiResourcePoolProtectedDepositManifestInput {
            bucket: (ManifestBuilderBucket => engine::ManifestBucket)
        }
    },
    {
        builder_method: multi_resource_pool_protected_withdraw,
        method_ident: engine::MULTI_RESOURCE_POOL_PROTECTED_WITHDRAW_IDENT,
        instruction: CallMethod,
        args: MultiResourcePoolProtectedWithdrawManifestInput {
            resource_address: (Arc<Address> => engine::ManifestResourceAddress),
            amount: (Arc<Decimal> => engine::Decimal),
            withdraw_strategy: (WithdrawStrategy => engine::WithdrawStrategy)
        }
    },
    {
        builder_method: multi_resource_pool_get_redemption_value,
        method_ident: engine::MULTI_RESOURCE_POOL_GET_REDEMPTION_VALUE_IDENT,
        instruction: CallMethod,
        args: MultiResourcePoolGetRedemptionValueManifestInput {
            amount_of_pool_units: (Arc<Decimal> => engine::Decimal),
        }
    },
    {
        builder_method: multi_resource_pool_get_vault_amount,
        method_ident: engine::MULTI_RESOURCE_POOL_GET_VAULT_AMOUNTS_IDENT,
        instruction: CallMethod,
        args: MultiResourcePoolGetVaultAmountsManifestInput {}
    },
    // ================
    // Metadata Module
    // ================
    {
        builder_method: metadata_set,
        method_ident: engine::METADATA_SET_IDENT,
        instruction: CallMetadataMethod,
        args: MetadataSetManifestInput {
            key: (String => String),
            value: (MetadataValue => engine::MetadataValue)
        }
    },
    {
        builder_method: metadata_lock,
        method_ident: engine::METADATA_LOCK_IDENT,
        instruction: CallMetadataMethod,
        args: MetadataLockManifestInput {
            key: (String => String),
        }
    },
    {
        builder_method: metadata_get,
        method_ident: engine::METADATA_GET_IDENT,
        instruction: CallMetadataMethod,
        args: MetadataGetManifestInput {
            key: (String => String),
        }
    },
    {
        builder_method: metadata_remove,
        method_ident: engine::METADATA_REMOVE_IDENT,
        instruction: CallMetadataMethod,
        args: MetadataRemoveManifestInput {
            key: (String => String),
        }
    },
    // =======================
    // Role Assignment Module
    // =======================
    {
        builder_method: role_assignment_get,
        method_ident: engine::ROLE_ASSIGNMENT_GET_IDENT,
        instruction: CallRoleAssignmentMethod,
        args: RoleAssignmentGetManifestInput {
            module: (ModuleId => engine::ObjectModuleId),
            role_key: (String => engine::RoleKey),
        }
    },
    {
        builder_method: role_assignment_set,
        method_ident: engine::ROLE_ASSIGNMENT_SET_IDENT,
        instruction: CallRoleAssignmentMethod,
        args: RoleAssignmentSetManifestInput {
            module: (ModuleId => engine::ObjectModuleId),
            role_key: (String => engine::RoleKey),
            rule: (Arc<AccessRule> => engine::AccessRule),
        }
    },
    {
        builder_method: role_assignment_set_owner,
        method_ident: engine::ROLE_ASSIGNMENT_SET_OWNER_IDENT,
        instruction: CallRoleAssignmentMethod,
        args: RoleAssignmentSetOwnerManifestInput {
            rule: (Arc<AccessRule> => engine::AccessRule),
        }
    },
    {
        builder_method: role_assignment_lock_owner,
        method_ident: engine::ROLE_ASSIGNMENT_LOCK_OWNER_IDENT,
        instruction: CallRoleAssignmentMethod,
        args: RoleAssignmentLockOwnerManifestInput {}
    },
    // ===============
    // Royalty Module
    // ===============
    {
        builder_method: royalty_set,
        method_ident: engine::COMPONENT_ROYALTY_SET_ROYALTY_IDENT,
        instruction: CallRoyaltyMethod,
        args: ComponentRoyaltySetManifestInput {
            method: (String => String),
            amount: (RoyaltyAmount => engine::RoyaltyAmount),
        }
    },
    {
        builder_method: royalty_lock,
        method_ident: engine::COMPONENT_ROYALTY_LOCK_ROYALTY_IDENT,
        instruction: CallRoyaltyMethod,
        args: ComponentRoyaltyLockManifestInput {
            method: (String => String),
        }
    },
    {
        builder_method: royalty_claim,
        method_ident: engine::COMPONENT_ROYALTY_CLAIM_ROYALTIES_IDENT,
        instruction: CallRoyaltyMethod,
        args: ComponentClaimRoyaltiesManifestInput {}
    },
    // ===============
    // Account Locker
    // ===============
    {
        builder_method: account_locker_instantiate,
        package_address: engine::LOCKER_PACKAGE,
        blueprint_ident: engine::ACCOUNT_LOCKER_BLUEPRINT,
        function_ident: engine::ACCOUNT_LOCKER_INSTANTIATE_IDENT,
        args: AccountLockerInstantiateManifestInput {
            owner_role: (OwnerRole => engine::OwnerRole),
            storer_role: (Arc<AccessRule> => engine::AccessRule),
            storer_updater_role: (Arc<AccessRule> => engine::AccessRule),
            recoverer_role: (Arc<AccessRule> => engine::AccessRule),
            recoverer_updater_role: (Arc<AccessRule> => engine::AccessRule),
            address_reservation: (Option<ManifestBuilderAddressReservation> => Option<engine::ManifestAddressReservation>)
        }
    },
    {
        builder_method: account_locker_instantiate_simple,
        package_address: engine::LOCKER_PACKAGE,
        blueprint_ident: engine::ACCOUNT_LOCKER_BLUEPRINT,
        function_ident: engine::ACCOUNT_LOCKER_INSTANTIATE_SIMPLE_IDENT,
        args: AccountLockerInstantiateSimpleManifestInput {
            allow_recover: (bool => bool),
        }
    },
    {
        builder_method: account_locker_store,
        method_ident: engine::ACCOUNT_LOCKER_STORE_IDENT,
        instruction: CallMethod,
        args: AccountLockerStoreManifestInput {
            claimant: (Arc<Address> => engine::GenericGlobal<engine::ManifestComponentAddress, engine::AccountMarker>),
            bucket: (ManifestBuilderBucket => engine::ManifestBucket),
            try_direct_send: (bool => bool),
        }
    },
    {
        builder_method: account_locker_airdrop,
        method_ident: engine::ACCOUNT_LOCKER_AIRDROP_IDENT,
        instruction: CallMethod,
        args: AccountLockerAirdropManifestInput {
            claimants: (
                HashMap<String, ResourceSpecifier> =>
                IndexMap<engine::GenericGlobal<engine::ManifestComponentAddress, engine::AccountMarker>, radix_engine_interface::blueprints::locker::ResourceSpecifier>
            ),
            bucket: (ManifestBuilderBucket => engine::ManifestBucket),
            try_direct_send: (bool => bool),
        }
    },
    {
        builder_method: account_locker_recover,
        method_ident: engine::ACCOUNT_LOCKER_RECOVER_IDENT,
        instruction: CallMethod,
        args: AccountLockerRecoverManifestInput {
            claimant: (Arc<Address> => engine::GenericGlobal<engine::ManifestComponentAddress, engine::AccountMarker>),
            resource_address: (Arc<Address> => engine::ManifestResourceAddress),
            amount: (Arc<Decimal> => engine::Decimal),
        }
    },
    {
        builder_method: account_locker_recover_non_fungibles,
        method_ident: engine::ACCOUNT_LOCKER_RECOVER_NON_FUNGIBLES_IDENT,
        instruction: CallMethod,
        args: AccountLockerRecoverNonFungiblesManifestInput {
            claimant: (Arc<Address> => engine::GenericGlobal<engine::ManifestComponentAddress, engine::AccountMarker>),
            resource_address: (Arc<Address> => engine::ManifestResourceAddress),
            ids: (Vec<NonFungibleLocalId> => engine::IndexSet<engine::NonFungibleLocalId>),
        }
    },
    {
        builder_method: account_locker_claim,
        method_ident: engine::ACCOUNT_LOCKER_CLAIM_IDENT,
        instruction: CallMethod,
        args: AccountLockerClaimManifestInput {
            claimant: (Arc<Address> => engine::GenericGlobal<engine::ManifestComponentAddress, engine::AccountMarker>),
            resource_address: (Arc<Address> => engine::ManifestResourceAddress),
            amount: (Arc<Decimal> => engine::Decimal),
        }
    },
    {
        builder_method: account_locker_claim_non_fungibles,
        method_ident: engine::ACCOUNT_LOCKER_CLAIM_NON_FUNGIBLES_IDENT,
        instruction: CallMethod,
        args: AccountLockerClaimNonFungiblesManifestInput {
            claimant: (Arc<Address> => engine::GenericGlobal<engine::ManifestComponentAddress, engine::AccountMarker>),
            resource_address: (Arc<Address> => engine::ManifestResourceAddress),
            ids: (Vec<NonFungibleLocalId> => engine::IndexSet<engine::NonFungibleLocalId>),
        }
    },
    {
        builder_method: account_locker_get_amount,
        method_ident: engine::ACCOUNT_LOCKER_GET_AMOUNT_IDENT,
        instruction: CallMethod,
        args: AccountLockerGetAmountManifestInput {
            claimant: (Arc<Address> => engine::GenericGlobal<engine::ManifestComponentAddress, engine::AccountMarker>),
            resource_address: (Arc<Address> => engine::ManifestResourceAddress),
        }
    },
    {
        builder_method: account_locker_get_non_fungible_local_ids,
        method_ident: engine::ACCOUNT_LOCKER_GET_NON_FUNGIBLE_LOCAL_IDS_IDENT,
        instruction: CallMethod,
        args: AccountLockerGetNonFungibleLocalIdsManifestInput {
            claimant: (Arc<Address> => engine::GenericGlobal<engine::ManifestComponentAddress, engine::AccountMarker>),
            resource_address: (Arc<Address> => engine::ManifestResourceAddress),
            limit: (u32 => u32)
        }
    },
}
