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
use radix_engine::types::FromPublicKey;
use radix_engine_common::prelude::to_manifest_value;

#[derive(Debug, Clone, Object, Default)]
pub struct ManifestBuilder {
    name_record: NameRecord,
    instructions: Vec<NativeInstruction>,
    blobs: Vec<Vec<u8>>,
}

#[uniffi::export]
impl ManifestBuilder {
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
            let resource_address = NativeResourceAddress::try_from(*resource_address)?;
            builder.name_record.new_bucket(&into_bucket.name)?;

            let instruction = NativeInstruction::TakeAllFromWorktop { resource_address };
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
            let resource_address = NativeResourceAddress::try_from(*resource_address)?;
            let amount = amount.0;
            builder.name_record.new_bucket(&into_bucket.name)?;

            let instruction = NativeInstruction::TakeFromWorktop {
                resource_address,
                amount,
            };
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
            let resource_address = NativeResourceAddress::try_from(*resource_address)?;
            let ids = ids
                .into_iter()
                .map(NativeNonFungibleLocalId::try_from)
                .collect::<Result<Vec<_>>>()?;
            builder.name_record.new_bucket(&into_bucket.name)?;

            let instruction = NativeInstruction::TakeNonFungiblesFromWorktop {
                resource_address,
                ids,
            };
            builder.instructions.push(instruction);
            Ok(())
        })
    }

    pub fn return_to_worktop(self: Arc<Self>, bucket: ManifestBuilderBucket) -> Result<Arc<Self>> {
        builder_arc_map(self, |builder| {
            let bucket = bucket.to_native(&builder.name_record)?;

            let instruction = NativeInstruction::ReturnToWorktop { bucket_id: bucket };
            builder.instructions.push(instruction);
            Ok(())
        })
    }

    pub fn assert_worktop_contains_any(
        self: Arc<Self>,
        resource_address: Arc<Address>,
    ) -> Result<Arc<Self>> {
        builder_arc_map(self, |builder| {
            let resource_address = NativeResourceAddress::try_from(*resource_address)?;

            let instruction = NativeInstruction::AssertWorktopContainsAny { resource_address };
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
            let resource_address = NativeResourceAddress::try_from(*resource_address)?;
            let amount = amount.0;

            let instruction = NativeInstruction::AssertWorktopContains {
                resource_address,
                amount,
            };
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
            let resource_address = NativeResourceAddress::try_from(*resource_address)?;
            let ids = ids
                .into_iter()
                .map(NativeNonFungibleLocalId::try_from)
                .collect::<Result<Vec<_>>>()?;

            let instruction = NativeInstruction::TakeNonFungiblesFromWorktop {
                resource_address,
                ids,
            };
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

            let instruction = NativeInstruction::PopFromAuthZone;
            builder.instructions.push(instruction);
            Ok(())
        })
    }

    pub fn push_to_auth_zone(self: Arc<Self>, proof: ManifestBuilderProof) -> Result<Arc<Self>> {
        builder_arc_map(self, |builder| {
            let proof = proof.to_native(&builder.name_record)?;

            let instruction = NativeInstruction::PushToAuthZone { proof_id: proof };
            builder.instructions.push(instruction);
            Ok(())
        })
    }

    pub fn drop_auth_zone_proofs(self: Arc<Self>) -> Result<Arc<Self>> {
        builder_arc_map(self, |builder| {
            let instruction = NativeInstruction::DropAuthZoneProofs;
            builder.instructions.push(instruction);
            Ok(())
        })
    }

    pub fn drop_auth_zone_signature_proofs(self: Arc<Self>) -> Result<Arc<Self>> {
        builder_arc_map(self, |builder| {
            let instruction = NativeInstruction::DropAuthZoneSignatureProofs;
            builder.instructions.push(instruction);
            Ok(())
        })
    }

    pub fn drop_all_proofs(self: Arc<Self>) -> Result<Arc<Self>> {
        builder_arc_map(self, |builder| {
            let instruction = NativeInstruction::DropAllProofs;
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
            let resource_address = NativeResourceAddress::try_from(*resource_address)?;
            builder.name_record.new_proof(&into_proof.name)?;

            let instruction = NativeInstruction::CreateProofFromAuthZoneOfAll { resource_address };
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
            let resource_address = NativeResourceAddress::try_from(*resource_address)?;
            let amount = amount.0;
            builder.name_record.new_proof(&into_proof.name)?;

            let instruction = NativeInstruction::CreateProofFromAuthZoneOfAmount {
                resource_address,
                amount,
            };
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
            let resource_address = NativeResourceAddress::try_from(*resource_address)?;
            let ids = ids
                .into_iter()
                .map(NativeNonFungibleLocalId::try_from)
                .collect::<Result<Vec<_>>>()?;
            builder.name_record.new_proof(&into_proof.name)?;

            let instruction = NativeInstruction::CreateProofFromAuthZoneOfNonFungibles {
                resource_address,
                ids,
            };
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

            let instruction = NativeInstruction::CreateProofFromBucketOfAll { bucket_id: bucket };
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

            let instruction = NativeInstruction::CreateProofFromBucketOfAmount {
                bucket_id: bucket,
                amount,
            };
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
                .map(NativeNonFungibleLocalId::try_from)
                .collect::<Result<Vec<_>>>()?;
            let bucket = bucket.to_native(&builder.name_record)?;
            builder.name_record.new_proof(&into_proof.name)?;

            let instruction = NativeInstruction::CreateProofFromBucketOfNonFungibles {
                bucket_id: bucket,
                ids,
            };
            builder.instructions.push(instruction);
            Ok(())
        })
    }

    pub fn burn_resource(self: Arc<Self>, bucket: ManifestBuilderBucket) -> Result<Arc<Self>> {
        builder_arc_map(self, |builder| {
            let bucket = bucket.to_native(&builder.name_record)?;

            let instruction = NativeInstruction::BurnResource { bucket_id: bucket };
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

            let instruction = NativeInstruction::CloneProof { proof_id: proof };
            builder.instructions.push(instruction);
            Ok(())
        })
    }

    pub fn drop_proof(self: Arc<Self>, proof: ManifestBuilderProof) -> Result<Arc<Self>> {
        builder_arc_map(self, |builder| {
            let proof = proof.to_native(&builder.name_record)?;

            let instruction = NativeInstruction::DropProof { proof_id: proof };
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
                NativeManifestAddress::Static(value) => value
                    .0
                    .try_into()
                    .map(NativeDynamicPackageAddress::Static)?,
                NativeManifestAddress::Named(value) => NativeDynamicPackageAddress::Named(value),
            };
            let args = NativeManifestValue::Tuple {
                fields: args
                    .into_iter()
                    .map(|x| x.to_native(&builder.name_record))
                    .collect::<Result<_>>()?,
            };

            let instruction = NativeInstruction::CallFunction {
                package_address: address,
                blueprint_name,
                function_name,
                args,
            };
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
                NativeManifestAddress::Static(value) => {
                    value.0.try_into().map(NativeDynamicGlobalAddress::Static)?
                }
                NativeManifestAddress::Named(value) => NativeDynamicGlobalAddress::Named(value),
            };
            let args = NativeManifestValue::Tuple {
                fields: args
                    .into_iter()
                    .map(|x| x.to_native(&builder.name_record))
                    .collect::<Result<_>>()?,
            };

            let instruction = NativeInstruction::CallMethod {
                address,
                method_name,
                args,
            };
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
                NativeManifestAddress::Static(value) => {
                    value.0.try_into().map(NativeDynamicGlobalAddress::Static)?
                }
                NativeManifestAddress::Named(value) => NativeDynamicGlobalAddress::Named(value),
            };
            let args = NativeManifestValue::Tuple {
                fields: args
                    .into_iter()
                    .map(|x| x.to_native(&builder.name_record))
                    .collect::<Result<_>>()?,
            };

            let instruction = NativeInstruction::CallRoyaltyMethod {
                address,
                method_name,
                args,
            };
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
                NativeManifestAddress::Static(value) => {
                    value.0.try_into().map(NativeDynamicGlobalAddress::Static)?
                }
                NativeManifestAddress::Named(value) => NativeDynamicGlobalAddress::Named(value),
            };
            let args = NativeManifestValue::Tuple {
                fields: args
                    .into_iter()
                    .map(|x| x.to_native(&builder.name_record))
                    .collect::<Result<_>>()?,
            };

            let instruction = NativeInstruction::CallMetadataMethod {
                address,
                method_name,
                args,
            };
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
                NativeManifestAddress::Static(value) => {
                    value.0.try_into().map(NativeDynamicGlobalAddress::Static)?
                }
                NativeManifestAddress::Named(value) => NativeDynamicGlobalAddress::Named(value),
            };
            let args = NativeManifestValue::Tuple {
                fields: args
                    .into_iter()
                    .map(|x| x.to_native(&builder.name_record))
                    .collect::<Result<_>>()?,
            };

            let instruction = NativeInstruction::CallRoleAssignmentMethod {
                address,
                method_name,
                args,
            };
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
            let address = NativeInternalAddress::try_from(*address)?;
            let args = NativeManifestValue::Tuple {
                fields: args
                    .into_iter()
                    .map(|x| x.to_native(&builder.name_record))
                    .collect::<Result<_>>()?,
            };

            let instruction = NativeInstruction::CallDirectVaultMethod {
                address,
                method_name,
                args,
            };
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
            let package_address = NativePackageAddress::try_from(*package_address)?;
            builder
                .name_record
                .new_address_reservation(&into_address_reservation.name)?;
            builder
                .name_record
                .new_named_address(&into_named_address.name)?;

            let instruction = NativeInstruction::AllocateGlobalAddress {
                package_address,
                blueprint_name,
            };
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
            let address = NativeGlobalAddress::try_from(*account_address)?;

            let instruction = NativeInstruction::CallMethod {
                address: NativeDynamicGlobalAddress::Static(address),
                method_name: NATIVE_ACCOUNT_DEPOSIT_BATCH_IDENT.to_owned(),
                args: manifest_args!(NativeManifestExpression::EntireWorktop).into(),
            };
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
            let address = NativeGlobalAddress::try_from(*account_address)?;
            let authorized_depositor_badge = if let Some(badge) = authorized_depositor_badge {
                Some(badge.to_native()?)
            } else {
                None
            };

            let instruction = NativeInstruction::CallMethod {
                address: NativeDynamicGlobalAddress::Static(address),
                method_name: NATIVE_ACCOUNT_TRY_DEPOSIT_BATCH_OR_REFUND_IDENT.to_owned(),
                args: manifest_args!(
                    NativeManifestExpression::EntireWorktop,
                    authorized_depositor_badge
                )
                .into(),
            };
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
            let address = NativeGlobalAddress::try_from(*account_address)?;
            let authorized_depositor_badge = if let Some(badge) = authorized_depositor_badge {
                Some(badge.to_native()?)
            } else {
                None
            };

            let instruction = NativeInstruction::CallMethod {
                address: NativeDynamicGlobalAddress::Static(address),
                method_name: NATIVE_ACCOUNT_TRY_DEPOSIT_BATCH_OR_ABORT_IDENT.to_owned(),
                args: manifest_args!(
                    NativeManifestExpression::EntireWorktop,
                    authorized_depositor_badge
                )
                .into(),
            };
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
            let code_blob = NativeManifestBlobRef(native_hash(&code).0);
            builder.blobs.push(code);

            let instruction = NativeInstruction::CallFunction {
                package_address: NativeDynamicPackageAddress::Static(NATIVE_PACKAGE_PACKAGE),
                blueprint_name: NATIVE_PACKAGE_BLUEPRINT.to_owned(),
                function_name: NATIVE_PACKAGE_PUBLISH_WASM_IDENT.to_owned(),
                args: native_to_manifest_value_and_unwrap!(
                    &NativePackagePublishWasmManifestInput {
                        code: code_blob,
                        definition: native_manifest_decode(&definition)?,
                        metadata: metadata.to_native()?,
                    }
                ),
            };
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
            let code_blob = NativeManifestBlobRef(native_hash(&code).0);
            builder.blobs.push(code);
            let address_reservation = match package_address {
                Some(reservation) => Some(
                    *builder
                        .name_record
                        .get_address_reservation(&reservation.name)?,
                ),
                None => None,
            };

            let instruction = NativeInstruction::CallFunction {
                package_address: NativeDynamicPackageAddress::Static(NATIVE_PACKAGE_PACKAGE),
                blueprint_name: NATIVE_PACKAGE_BLUEPRINT.to_owned(),
                function_name: NATIVE_PACKAGE_PUBLISH_WASM_ADVANCED_IDENT.to_owned(),
                args: native_to_manifest_value_and_unwrap!(
                    &NativePackagePublishWasmAdvancedManifestInput {
                        code: code_blob,
                        definition: native_manifest_decode(&definition)?,
                        metadata: metadata.to_native()?,
                        owner_role: owner_role.to_native()?,
                        package_address: address_reservation
                    }
                ),
            };
            builder.instructions.push(instruction);
            Ok(())
        })
    }

    pub fn faucet_free_xrd(self: Arc<Self>) -> Result<Arc<Self>> {
        builder_arc_map(self, |builder| {
            let instruction = NativeInstruction::CallMethod {
                address: NativeDynamicGlobalAddress::Static(NATIVE_FAUCET.into()),
                method_name: "free".to_owned(),
                args: manifest_args!().into(),
            };
            builder.instructions.push(instruction);
            Ok(())
        })
    }

    pub fn faucet_lock_fee(self: Arc<Self>) -> Result<Arc<Self>> {
        builder_arc_map(self, |builder| {
            let instruction = NativeInstruction::CallMethod {
                address: NativeDynamicGlobalAddress::Static(NATIVE_FAUCET.into()),
                method_name: "lock_fee".to_owned(),
                args: manifest_args!(native_dec!("100")).into(),
            };
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
            let bucket = builder.name_record.get_bucket(&controlled_asset.name)?;
            let address_reservation = match address_reservation {
                Some(reservation) => Some(
                    builder
                        .name_record
                        .get_address_reservation(&reservation.name)?,
                ),
                None => None,
            };

            let rule_set = NativeRuleSet {
                primary_role: native_rule!(native_require(
                    NativeNonFungibleGlobalId::from_public_key(&NativePublicKey::try_from(
                        primary_role
                    )?)
                )),
                recovery_role: native_rule!(native_require(
                    NativeNonFungibleGlobalId::from_public_key(&NativePublicKey::try_from(
                        recovery_role
                    )?)
                )),
                confirmation_role: native_rule!(native_require(
                    NativeNonFungibleGlobalId::from_public_key(&NativePublicKey::try_from(
                        confirmation_role
                    )?)
                )),
            };

            let instruction = NativeInstruction::CallFunction {
                package_address: NativeDynamicPackageAddress::Static(
                    NATIVE_ACCESS_CONTROLLER_PACKAGE,
                ),
                blueprint_name: NATIVE_ACCESS_CONTROLLER_BLUEPRINT.to_owned(),
                function_name: NATIVE_ACCESS_CONTROLLER_CREATE_IDENT.to_owned(),
                args: manifest_args!(
                    bucket,
                    rule_set,
                    timed_recovery_delay_in_minutes,
                    address_reservation
                )
                .into(),
            };
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
            let bucket = builder.name_record.get_bucket(&controlled_asset.name)?;
            let address_reservation = match address_reservation {
                Some(reservation) => Some(
                    builder
                        .name_record
                        .get_address_reservation(&reservation.name)?,
                ),
                None => None,
            };

            let rule_set = NativeRuleSet {
                primary_role: NativeAccessRule::try_from(primary_role)?,
                recovery_role: NativeAccessRule::try_from(recovery_role)?,
                confirmation_role: NativeAccessRule::try_from(confirmation_role)?,
            };

            let instruction = NativeInstruction::CallFunction {
                package_address: NativeDynamicPackageAddress::Static(
                    NATIVE_ACCESS_CONTROLLER_PACKAGE,
                ),
                blueprint_name: NATIVE_ACCESS_CONTROLLER_BLUEPRINT.to_owned(),
                function_name: NATIVE_ACCESS_CONTROLLER_CREATE_IDENT.to_owned(),
                args: manifest_args!(
                    bucket,
                    rule_set,
                    timed_recovery_delay_in_minutes,
                    address_reservation
                )
                .into(),
            };
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

            let (function_name, args) = if let Some(initial_supply) = initial_supply {
                (
                    NATIVE_FUNGIBLE_RESOURCE_MANAGER_CREATE_WITH_INITIAL_SUPPLY_IDENT,
                    native_to_manifest_value_and_unwrap!(
                        &NativeFungibleResourceManagerCreateWithInitialSupplyManifestInput {
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
                    NATIVE_FUNGIBLE_RESOURCE_MANAGER_CREATE_IDENT,
                    native_to_manifest_value_and_unwrap!(
                        &NativeFungibleResourceManagerCreateManifestInput {
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

            let instruction = NativeInstruction::CallFunction {
                package_address: NativeDynamicPackageAddress::Static(NATIVE_RESOURCE_PACKAGE),
                blueprint_name: NATIVE_FUNGIBLE_RESOURCE_MANAGER_BLUEPRINT.to_owned(),
                function_name: function_name.to_owned(),
                args,
            };
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
            let resource_address = NativeResourceAddress::try_from(*resource_address)?;
            let amount = amount.0;

            let instruction = NativeInstruction::CallMethod {
                address: NativeDynamicGlobalAddress::Static(resource_address.into()),
                method_name: NATIVE_FUNGIBLE_RESOURCE_MANAGER_MINT_IDENT.to_owned(),
                args: native_to_manifest_value_and_unwrap!(
                    &NativeFungibleResourceManagerMintInput { amount }
                ),
            };
            builder.instructions.push(instruction);
            Ok(())
        })
    }

    //=================
    // Builder Methods
    //=================

    pub fn build(self: Arc<Self>, network_id: u8) -> Arc<TransactionManifest> {
        Arc::new(TransactionManifest {
            instructions: Arc::new(Instructions(self.instructions.clone(), network_id)),
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

impl TryFrom<SecurityStructureRole> for NativeAccessRule {
    type Error = RadixEngineToolkitError;

    fn try_from(value: SecurityStructureRole) -> std::result::Result<Self, Self::Error> {
        let super_admin_factors = value
            .super_admin_factors
            .into_iter()
            .map(|pk| {
                NativePublicKey::try_from(pk)
                    .map(|pk| NativeNonFungibleGlobalId::from_public_key(&pk))
                    .map(NativeResourceOrNonFungible::NonFungible)
            })
            .collect::<Result<Vec<NativeResourceOrNonFungible>>>()?;
        let threshold_factors = value
            .threshold_factors
            .into_iter()
            .map(|pk| {
                NativePublicKey::try_from(pk)
                    .map(|pk| NativeNonFungibleGlobalId::from_public_key(&pk))
                    .map(NativeResourceOrNonFungible::NonFungible)
            })
            .collect::<Result<Vec<NativeResourceOrNonFungible>>>()?;

        Ok(NativeAccessRule::Protected(NativeAccessRuleNode::AnyOf(
            vec![
                NativeAccessRuleNode::ProofRule(NativeProofRule::CountOf(
                    value.threshold,
                    threshold_factors,
                )),
                NativeAccessRuleNode::ProofRule(NativeProofRule::AnyOf(super_admin_factors)),
            ],
        )))
    }
}

macro_rules! manifest_args {
    ($($args: expr),*$(,)?) => {{
        use ::sbor::Encoder;
        let mut buf = ::sbor::rust::vec::Vec::new();
        let mut encoder = radix_engine_common::data::manifest::ManifestEncoder::new(
            &mut buf,
            radix_engine_common::data::manifest::MANIFEST_SBOR_V1_MAX_DEPTH
        );
        encoder.write_payload_prefix(
            radix_engine_common::data::manifest::MANIFEST_SBOR_V1_PAYLOAD_PREFIX
        ).unwrap();
        encoder.write_value_kind(
            radix_engine_common::data::manifest::ManifestValueKind::Tuple
        ).unwrap();
        // Hack: stringify to skip ownership move semantics
        encoder.write_size(radix_engine_common::count!($(stringify!($args)),*)).unwrap();
        $(
            let arg = $args;
            encoder.encode(&arg).unwrap();
        )*
        let value = radix_engine_common::data::manifest::manifest_decode(&buf).unwrap();
        radix_engine_common::data::manifest::ManifestArgs::new_from_tuple_or_panic(value)
    }};
}
use manifest_args;

/// This macro defines a simple DSL for adding aliases to method and function calls to the manifest
/// builder without the need to manually author and maintain the relatively large amount of boiler-
/// plate code needed for it.
///
/// Both method and function aliases may be declared through this macro. The following is an example
/// usage of this macro:
///
/// ```rust,no_run
/// builder_alias! {
///     // The name of the method to add to the manifest builder.
///     builder_method: account_set_default_deposit_rule,
///     // The name of the method to call.
///     method_ident: NATIVE_ACCOUNT_SET_DEFAULT_DEPOSIT_RULE_IDENT,
///     // The instruction to use - this can either be a CallMethod, CallRoyaltyMethod,
///     // CallMetadataMethod or CallRoleAssignmentMethod.
///     instruction: CallMethod,
///     // This defines multiple things:
///     // 1. What arguments the builder method will have in its interface: their names and types
///     //    are defined.
///     // 2. The struct used for the input that will be encoded and converted into method args.
///     // 3. The exact conversions that need to be made between the types used in the manifest
///     //    builder interface to the types used in the input struct. This conversion happens
///     //    through the `FromWithNameRecordContext` trait.
///     args: NativeAccountSetDefaultDepositRuleInput {
///         default as default_deposit_rule: (AccountDefaultDepositRule => NativeDefaultDepositRule),
///     }
/// }
/// ```
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
        #[uniffi::export]
        impl ManifestBuilder {
            pub fn $builder_method(
                self: $crate::prelude::Arc<Self>,
                address: $crate::prelude::Arc<$crate::prelude::Address>,
                $(
                    $interface_arg_name: $interface_arg_type
                ),*
            ) -> $crate::prelude::Result<Arc<Self>> {
                $crate::builder::manifest_builder::utils::builder_arc_map(self, |builder| {
                    let instruction = $crate::prelude::NativeInstruction::$instruction {
                        address: $crate::prelude::NativeDynamicGlobalAddress::Static((*address).try_into()?),
                        method_name: $method_ident.to_owned(),
                        args: $crate::prelude::native_to_manifest_value_and_unwrap! {
                            &$input_type {
                                $(
                                    $input_arg_name: <
                                        $input_arg_type
                                        as $crate::builder::manifest_builder::traits::FromWithNameRecordContext<$interface_arg_type>
                                    >::from($interface_arg_name, &builder.name_record)?
                                ),*
                            }
                        }
                    };
                    builder.instructions.push(instruction);
                    Ok(())
                })
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
        impl ManifestBuilder {
            pub fn $builder_method(
                self: $crate::prelude::Arc<Self>,
                $(
                    $interface_arg_name: $interface_arg_type
                ),*
            ) -> $crate::prelude::Result<Arc<Self>> {
                $crate::builder::manifest_builder::utils::builder_arc_map(self, |builder| {
                    let instruction = $crate::prelude::NativeInstruction::CallFunction {
                        package_address: $crate::prelude::NativeDynamicPackageAddress::Static($package_address),
                        blueprint_name: $blueprint_ident.to_owned(),
                        function_name: $function_ident.to_owned(),
                        args: $crate::prelude::native_to_manifest_value_and_unwrap! {
                            &$input_type {
                                $(
                                    $input_arg_name: <
                                        $input_arg_type
                                        as $crate::builder::manifest_builder::traits::FromWithNameRecordContext<$interface_arg_type>
                                    >::from($interface_arg_name, &builder.name_record)?
                                ),*
                            }
                        }
                    };
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
        package_address: NATIVE_ACCOUNT_PACKAGE,
        blueprint_ident: NATIVE_ACCOUNT_BLUEPRINT,
        function_ident: NATIVE_ACCOUNT_CREATE_ADVANCED_IDENT,
        args: NativeAccountCreateAdvancedManifestInput {
            owner_role: (OwnerRole => NativeOwnerRole),
            address_reservation: (
                Option<ManifestBuilderAddressReservation> =>
             Option<NativeManifestAddressReservation>             )
        }
    },
    {
        builder_method: account_create,
        package_address: NATIVE_ACCOUNT_PACKAGE,
        blueprint_ident: NATIVE_ACCOUNT_BLUEPRINT,
        function_ident: NATIVE_ACCOUNT_CREATE_IDENT,
        args: NativeAccountCreateInput {}
    },
    {
        builder_method: account_securify,
        method_ident: NATIVE_ACCOUNT_SECURIFY_IDENT,
        instruction: CallMethod,
        args: NativeAccountSecurifyInput {}
    },
    {
        builder_method: account_lock_fee,
        method_ident: NATIVE_ACCOUNT_LOCK_FEE_IDENT,
        instruction: CallMethod,
        args: NativeAccountLockFeeInput {
            amount: (Arc<Decimal> => NativeDecimal)
        }
    },
    {
        builder_method: account_lock_contingent_fee,
        method_ident: NATIVE_ACCOUNT_LOCK_CONTINGENT_FEE_IDENT,
        instruction: CallMethod,
        args: NativeAccountLockContingentFeeInput {
            amount: (Arc<Decimal> => NativeDecimal)
        }
    },
    {
        builder_method: account_deposit,
        method_ident: NATIVE_ACCOUNT_DEPOSIT_IDENT,
        instruction: CallMethod,
        args: NativeAccountDepositManifestInput {
            bucket: (ManifestBuilderBucket => NativeManifestBucket)
        }
    },
    {
        builder_method: account_try_deposit_or_abort,
        method_ident: NATIVE_ACCOUNT_TRY_DEPOSIT_OR_ABORT_IDENT,
        instruction: CallMethod,
        args: NativeAccountTryDepositOrAbortManifestInput {
            bucket: (ManifestBuilderBucket => NativeManifestBucket),
            authorized_depositor_badge: (Option<ResourceOrNonFungible> => Option<NativeResourceOrNonFungible>),
        }
    },
    {
        builder_method: account_try_deposit_or_refund,
        method_ident: NATIVE_ACCOUNT_TRY_DEPOSIT_OR_REFUND_IDENT,
        instruction: CallMethod,
        args: NativeAccountTryDepositOrRefundManifestInput {
            bucket: (ManifestBuilderBucket => NativeManifestBucket),
            authorized_depositor_badge: (Option<ResourceOrNonFungible> => Option<NativeResourceOrNonFungible>),
        }
    },
    {
        builder_method: account_deposit_batch,
        method_ident: NATIVE_ACCOUNT_DEPOSIT_BATCH_IDENT,
        instruction: CallMethod,
        args: NativeAccountDepositBatchManifestInput {
            buckets: (Vec<ManifestBuilderBucket> => Vec<NativeManifestBucket>)
        }
    },
    {
        builder_method: account_try_deposit_batch_or_abort,
        method_ident: NATIVE_ACCOUNT_TRY_DEPOSIT_BATCH_OR_ABORT_IDENT,
        instruction: CallMethod,
        args: NativeAccountTryDepositBatchOrAbortManifestInput {
            buckets: (Vec<ManifestBuilderBucket> => Vec<NativeManifestBucket>),
            authorized_depositor_badge: (Option<ResourceOrNonFungible> => Option<NativeResourceOrNonFungible>),
        }
    },
    {
        builder_method: account_try_deposit_batch_or_refund,
        method_ident: NATIVE_ACCOUNT_TRY_DEPOSIT_BATCH_OR_REFUND_IDENT,
        instruction: CallMethod,
        args: NativeAccountTryDepositBatchOrRefundManifestInput {
            buckets: (Vec<ManifestBuilderBucket> => Vec<NativeManifestBucket>),
            authorized_depositor_badge: (Option<ResourceOrNonFungible> => Option<NativeResourceOrNonFungible>),
        }
    },
    {
        builder_method: account_withdraw,
        method_ident: NATIVE_ACCOUNT_WITHDRAW_IDENT,
        instruction: CallMethod,
        args: NativeAccountWithdrawInput {
            resource_address: (Arc<Address> => NativeResourceAddress),
            amount: (Arc<Decimal> => NativeDecimal),
        }
    },
    {
        builder_method: account_withdraw_non_fungibles,
        method_ident: NATIVE_ACCOUNT_WITHDRAW_NON_FUNGIBLES_IDENT,
        instruction: CallMethod,
        args: NativeAccountWithdrawNonFungiblesInput {
            resource_address: (Arc<Address> => NativeResourceAddress),
            ids: (Vec<NonFungibleLocalId> => IndexSet<NativeNonFungibleLocalId>),
        }
    },
    {
        builder_method: account_lock_fee_and_withdraw,
        method_ident: NATIVE_ACCOUNT_LOCK_FEE_AND_WITHDRAW_IDENT,
        instruction: CallMethod,
        args: NativeAccountLockFeeAndWithdrawInput {
            amount_to_lock: (Arc<Decimal> => NativeDecimal),
            resource_address: (Arc<Address> => NativeResourceAddress),
            amount: (Arc<Decimal> => NativeDecimal),
        }
    },
    {
        builder_method: account_lock_fee_and_withdraw_non_fungibles,
        method_ident: NATIVE_ACCOUNT_LOCK_FEE_AND_WITHDRAW_NON_FUNGIBLES_IDENT,
        instruction: CallMethod,
        args: NativeAccountLockFeeAndWithdrawNonFungiblesInput {
            amount_to_lock: (Arc<Decimal> => NativeDecimal),
            resource_address: (Arc<Address> => NativeResourceAddress),
            ids: (Vec<NonFungibleLocalId> => IndexSet<NativeNonFungibleLocalId>),
        }
    },
    {
        builder_method: account_create_proof_of_amount,
        method_ident: NATIVE_ACCOUNT_CREATE_PROOF_OF_AMOUNT_IDENT,
        instruction: CallMethod,
        args: NativeAccountCreateProofOfAmountInput {
            resource_address: (Arc<Address> => NativeResourceAddress),
            amount: (Arc<Decimal> => NativeDecimal),
        }
    },
    {
        builder_method: account_create_proof_of_non_fungibles,
        method_ident: NATIVE_ACCOUNT_CREATE_PROOF_OF_NON_FUNGIBLES_IDENT,
        instruction: CallMethod,
        args: NativeAccountCreateProofOfNonFungiblesInput {
            resource_address: (Arc<Address> => NativeResourceAddress),
            ids: (Vec<NonFungibleLocalId> => IndexSet<NativeNonFungibleLocalId>),
        }
    },
    {
        builder_method: account_set_default_deposit_rule,
        method_ident: NATIVE_ACCOUNT_SET_DEFAULT_DEPOSIT_RULE_IDENT,
        instruction: CallMethod,
        args: NativeAccountSetDefaultDepositRuleInput {
            default as default_deposit_rule: (AccountDefaultDepositRule => NativeDefaultDepositRule),
        }
    },
    {
        builder_method: account_set_resource_preference,
        method_ident: NATIVE_ACCOUNT_SET_RESOURCE_PREFERENCE_IDENT,
        instruction: CallMethod,
        args: NativeAccountSetResourcePreferenceInput {
            resource_address: (Arc<Address> => NativeResourceAddress),
            resource_preference: (ResourcePreference => NativeResourcePreference),
        }
    },
    {
        builder_method: account_remove_resource_preference,
        method_ident: NATIVE_ACCOUNT_REMOVE_RESOURCE_PREFERENCE_IDENT,
        instruction: CallMethod,
        args: NativeAccountRemoveResourcePreferenceInput {
            resource_address: (Arc<Address> => NativeResourceAddress),
        }
    },
    {
        builder_method: account_burn,
        method_ident: NATIVE_ACCOUNT_BURN_IDENT,
        instruction: CallMethod,
        args: NativeAccountBurnInput {
            resource_address: (Arc<Address> => NativeResourceAddress),
            amount: (Arc<Decimal> => NativeDecimal),
        }
    },
    {
        builder_method: account_burn_non_fungibles,
        method_ident: NATIVE_ACCOUNT_BURN_NON_FUNGIBLES_IDENT,
        instruction: CallMethod,
        args: NativeAccountBurnNonFungiblesInput {
            resource_address: (Arc<Address> => NativeResourceAddress),
            ids: (Vec<NonFungibleLocalId> => IndexSet<NativeNonFungibleLocalId>),
        }
    },
    {
        builder_method: account_add_authorized_depositor,
        method_ident: NATIVE_ACCOUNT_ADD_AUTHORIZED_DEPOSITOR,
        instruction: CallMethod,
        args: NativeAccountAddAuthorizedDepositorInput {
            badge: (ResourceOrNonFungible => NativeResourceOrNonFungible),
        }
    },
    {
        builder_method: account_remove_authorized_depositor,
        method_ident: NATIVE_ACCOUNT_REMOVE_AUTHORIZED_DEPOSITOR,
        instruction: CallMethod,
        args: NativeAccountRemoveAuthorizedDepositorInput {
            badge: (ResourceOrNonFungible => NativeResourceOrNonFungible),
        }
    },
    // ==========
    // Validator
    // ==========
    {
        builder_method: validator_register,
        method_ident: NATIVE_VALIDATOR_REGISTER_IDENT,
        instruction: CallMethod,
        args: NativeValidatorRegisterInput {}
    },
    {
        builder_method: validator_unregister,
        method_ident: NATIVE_VALIDATOR_UNREGISTER_IDENT,
        instruction: CallMethod,
        args: NativeValidatorUnregisterInput {}
    },
    {
        builder_method: validator_stake_as_owner,
        method_ident: NATIVE_VALIDATOR_STAKE_AS_OWNER_IDENT,
        instruction: CallMethod,
        args: NativeValidatorStakeAsOwnerManifestInput {
            stake: (ManifestBuilderBucket => NativeManifestBucket)
        }
    },
    {
        builder_method: validator_stake,
        method_ident: NATIVE_VALIDATOR_STAKE_IDENT,
        instruction: CallMethod,
        args: NativeValidatorStakeManifestInput {
            stake: (ManifestBuilderBucket => NativeManifestBucket)
        }
    },
    {
        builder_method: validator_unstake,
        method_ident: NATIVE_VALIDATOR_UNSTAKE_IDENT,
        instruction: CallMethod,
        args: NativeValidatorUnstakeManifestInput {
            stake_unit_bucket: (ManifestBuilderBucket => NativeManifestBucket)
        }
    },
    {
        builder_method: validator_claim_xrd,
        method_ident: NATIVE_VALIDATOR_CLAIM_XRD_IDENT,
        instruction: CallMethod,
        args: NativeValidatorClaimXrdManifestInput {
            bucket: (ManifestBuilderBucket => NativeManifestBucket)
        }
    },
    {
        builder_method: validator_update_key,
        method_ident: NATIVE_VALIDATOR_UPDATE_KEY_IDENT,
        instruction: CallMethod,
        args: NativeValidatorUpdateKeyInput {
            key: (PublicKey => NativeSecp256k1PublicKey)
        }
    },
    {
        builder_method: validator_update_fee,
        method_ident: NATIVE_VALIDATOR_UPDATE_FEE_IDENT,
        instruction: CallMethod,
        args: NativeValidatorUpdateFeeInput {
            new_fee_factor: (Arc<Decimal> => NativeDecimal)
        }
    },
    {
        builder_method: validator_update_accept_delegated_stake,
        method_ident: NATIVE_VALIDATOR_UPDATE_ACCEPT_DELEGATED_STAKE_IDENT,
        instruction: CallMethod,
        args: NativeValidatorUpdateAcceptDelegatedStakeInput {
            accept_delegated_stake: (bool => bool)
        }
    },
    {
        builder_method: validator_accepts_delegated_stake,
        method_ident: NATIVE_VALIDATOR_ACCEPTS_DELEGATED_STAKE_IDENT,
        instruction: CallMethod,
        args: NativeValidatorAcceptsDelegatedStakeInput {}
    },
    {
        builder_method: validator_total_stake_xrd_amount,
        method_ident: NATIVE_VALIDATOR_TOTAL_STAKE_XRD_AMOUNT_IDENT,
        instruction: CallMethod,
        args: NativeValidatorTotalStakeXrdAmountInput {}
    },
    {
        builder_method: validator_total_stake_unit_supply,
        method_ident: NATIVE_VALIDATOR_TOTAL_STAKE_UNIT_SUPPLY_IDENT,
        instruction: CallMethod,
        args: NativeValidatorTotalStakeUnitSupplyInput {}
    },
    {
        builder_method: validator_get_redemption_value,
        method_ident: NATIVE_VALIDATOR_GET_REDEMPTION_VALUE_IDENT,
        instruction: CallMethod,
        args: NativeValidatorGetRedemptionValueInput {
            amount_of_stake_units: (Arc<Decimal> => NativeDecimal)
        }
    },
    {
        builder_method: validator_signal_protocol_update_readiness,
        method_ident: NATIVE_VALIDATOR_SIGNAL_PROTOCOL_UPDATE_READINESS,
        instruction: CallMethod,
        args: NativeValidatorSignalProtocolUpdateReadinessInput {
            vote: (String => String)
        }
    },
    {
        builder_method: validator_get_protocol_update_readiness,
        method_ident: NATIVE_VALIDATOR_GET_PROTOCOL_UPDATE_READINESS_IDENT,
        instruction: CallMethod,
        args: NativeValidatorGetProtocolUpdateReadinessInput {}
    },
    {
        builder_method: validator_lock_owner_stake_units,
        method_ident: NATIVE_VALIDATOR_LOCK_OWNER_STAKE_UNITS_IDENT,
        instruction: CallMethod,
        args: NativeValidatorLockOwnerStakeUnitsManifestInput {
            stake_unit_bucket: (ManifestBuilderBucket => NativeManifestBucket)
        }
    },
    {
        builder_method: validator_start_unlock_owner_stake_units,
        method_ident: NATIVE_VALIDATOR_START_UNLOCK_OWNER_STAKE_UNITS_IDENT,
        instruction: CallMethod,
        args: NativeValidatorStartUnlockOwnerStakeUnitsInput {
            requested_stake_unit_amount: (Arc<Decimal> => NativeDecimal)
        }
    },
    {
        builder_method: validator_finish_unlock_owner_stake_units,
        method_ident: NATIVE_VALIDATOR_FINISH_UNLOCK_OWNER_STAKE_UNITS_IDENT,
        instruction: CallMethod,
        args: NativeValidatorFinishUnlockOwnerStakeUnitsInput {}
    },
    // ==================
    // Access Controller
    // ==================
    {
        builder_method: access_controller_create,
        method_ident: NATIVE_ACCESS_CONTROLLER_CREATE_IDENT,
        instruction: CallMethod,
        args: NativeAccessControllerCreateManifestInput {
            controlled_asset: (ManifestBuilderBucket => NativeManifestBucket),
            rule_set: (RuleSet => NativeRuleSet),
            timed_recovery_delay_in_minutes: (Option<u32> => Option<u32>),
            address_reservation: (Option<ManifestBuilderAddressReservation> => Option<NativeManifestAddressReservation>)
        }
    },
    {
        builder_method: access_controller_create_proof,
        method_ident: NATIVE_ACCESS_CONTROLLER_CREATE_PROOF_IDENT,
        instruction: CallMethod,
        args: NativeAccessControllerCreateProofInput {}
    },
    {
        builder_method: access_controller_initiate_recovery_as_primary,
        method_ident: NATIVE_ACCESS_CONTROLLER_INITIATE_RECOVERY_AS_PRIMARY_IDENT,
        instruction: CallMethod,
        args: NativeAccessControllerInitiateRecoveryAsPrimaryInput {
            rule_set: (RuleSet => NativeRuleSet),
            timed_recovery_delay_in_minutes: (Option<u32> => Option<u32>),
        }
    },
    {
        builder_method: access_controller_initiate_recovery_as_recovery,
        method_ident: NATIVE_ACCESS_CONTROLLER_INITIATE_RECOVERY_AS_RECOVERY_IDENT,
        instruction: CallMethod,
        args: NativeAccessControllerInitiateRecoveryAsRecoveryInput {
            rule_set: (RuleSet => NativeRuleSet),
            timed_recovery_delay_in_minutes: (Option<u32> => Option<u32>),
        }
    },
    {
        builder_method: access_controller_initiate_badge_withdraw_as_primary,
        method_ident: NATIVE_ACCESS_CONTROLLER_INITIATE_BADGE_WITHDRAW_ATTEMPT_AS_PRIMARY_IDENT,
        instruction: CallMethod,
        args: NativeAccessControllerInitiateBadgeWithdrawAttemptAsPrimaryInput {}
    },
    {
        builder_method: access_controller_initiate_badge_withdraw_as_recovery,
        method_ident: NATIVE_ACCESS_CONTROLLER_INITIATE_BADGE_WITHDRAW_ATTEMPT_AS_RECOVERY_IDENT,
        instruction: CallMethod,
        args: NativeAccessControllerInitiateBadgeWithdrawAttemptAsRecoveryInput {}
    },
    {
        builder_method: access_controller_quick_confirm_primary_role_recovery_proposal,
        method_ident: NATIVE_ACCESS_CONTROLLER_QUICK_CONFIRM_PRIMARY_ROLE_RECOVERY_PROPOSAL_IDENT,
        instruction: CallMethod,
        args: NativeAccessControllerQuickConfirmPrimaryRoleRecoveryProposalInput {
            rule_set: (RuleSet => NativeRuleSet),
            timed_recovery_delay_in_minutes: (Option<u32> => Option<u32>),
        }
    },
    {
        builder_method: access_controller_quick_confirm_recovery_role_recovery_proposal,
        method_ident: NATIVE_ACCESS_CONTROLLER_QUICK_CONFIRM_RECOVERY_ROLE_RECOVERY_PROPOSAL_IDENT,
        instruction: CallMethod,
        args: NativeAccessControllerQuickConfirmRecoveryRoleRecoveryProposalInput {
            rule_set: (RuleSet => NativeRuleSet),
            timed_recovery_delay_in_minutes: (Option<u32> => Option<u32>),
        }
    },
    {
        builder_method: access_controller_quick_confirm_primary_role_badge_withdraw_attempt,
        method_ident: NATIVE_ACCESS_CONTROLLER_QUICK_CONFIRM_PRIMARY_ROLE_BADGE_WITHDRAW_ATTEMPT_IDENT,
        instruction: CallMethod,
        args: NativeAccessControllerQuickConfirmPrimaryRoleBadgeWithdrawAttemptInput {}
    },
    {
        builder_method: access_controller_quick_confirm_recovery_role_badge_withdraw_attempt,
        method_ident: NATIVE_ACCESS_CONTROLLER_QUICK_CONFIRM_RECOVERY_ROLE_BADGE_WITHDRAW_ATTEMPT_IDENT,
        instruction: CallMethod,
        args: NativeAccessControllerQuickConfirmRecoveryRoleBadgeWithdrawAttemptInput {}
    },
    {
        builder_method: access_controller_timed_confirm_recovery,
        method_ident: NATIVE_ACCESS_CONTROLLER_TIMED_CONFIRM_RECOVERY_IDENT,
        instruction: CallMethod,
        args: NativeAccessControllerTimedConfirmRecoveryInput {
            rule_set: (RuleSet => NativeRuleSet),
            timed_recovery_delay_in_minutes: (Option<u32> => Option<u32>),
        }
    },
    {
        builder_method: access_controller_cancel_primary_role_recovery_proposal,
        method_ident: NATIVE_ACCESS_CONTROLLER_CANCEL_PRIMARY_ROLE_RECOVERY_PROPOSAL_IDENT,
        instruction: CallMethod,
        args: NativeAccessControllerCancelPrimaryRoleRecoveryProposalInput {}
    },
    {
        builder_method: access_controller_cancel_recovery_role_recovery_proposal,
        method_ident: NATIVE_ACCESS_CONTROLLER_CANCEL_RECOVERY_ROLE_RECOVERY_PROPOSAL_IDENT,
        instruction: CallMethod,
        args: NativeAccessControllerCancelRecoveryRoleRecoveryProposalInput {}
    },
    {
        builder_method: access_controller_cancel_primary_role_badge_withdraw_attempt,
        method_ident: NATIVE_ACCESS_CONTROLLER_CANCEL_PRIMARY_ROLE_BADGE_WITHDRAW_ATTEMPT_IDENT,
        instruction: CallMethod,
        args: NativeAccessControllerCancelPrimaryRoleBadgeWithdrawAttemptInput {}
    },
    {
        builder_method: access_controller_cancel_recovery_role_badge_withdraw_attempt,
        method_ident: NATIVE_ACCESS_CONTROLLER_CANCEL_RECOVERY_ROLE_BADGE_WITHDRAW_ATTEMPT_IDENT,
        instruction: CallMethod,
        args: NativeAccessControllerCancelRecoveryRoleBadgeWithdrawAttemptInput {}
    },
    {
        builder_method: access_controller_lock_primary_role,
        method_ident: NATIVE_ACCESS_CONTROLLER_LOCK_PRIMARY_ROLE_IDENT,
        instruction: CallMethod,
        args: NativeAccessControllerLockPrimaryRoleInput {}
    },
    {
        builder_method: access_controller_unlock_primary_role,
        method_ident: NATIVE_ACCESS_CONTROLLER_UNLOCK_PRIMARY_ROLE_IDENT,
        instruction: CallMethod,
        args: NativeAccessControllerUnlockPrimaryRoleInput {}
    },
    {
        builder_method: access_controller_stop_timed_recovery,
        method_ident: NATIVE_ACCESS_CONTROLLER_STOP_TIMED_RECOVERY_IDENT,
        instruction: CallMethod,
        args: NativeAccessControllerStopTimedRecoveryInput {
            rule_set: (RuleSet => NativeRuleSet),
            timed_recovery_delay_in_minutes: (Option<u32> => Option<u32>),
        }
    },
    {
        builder_method: access_controller_mint_recovery_badges,
        method_ident: NATIVE_ACCESS_CONTROLLER_MINT_RECOVERY_BADGES_IDENT,
        instruction: CallMethod,
        args: NativeAccessControllerMintRecoveryBadgesInput {
            non_fungible_local_ids: (Vec<NonFungibleLocalId> => IndexSet<NativeNonFungibleLocalId>),
        }
    },
    // =========
    // Identity
    // =========
    {
        builder_method: identity_create_advanced,
        method_ident: NATIVE_IDENTITY_CREATE_ADVANCED_IDENT,
        instruction: CallMethod,
        args: NativeIdentityCreateAdvancedInput {
            owner_role: (OwnerRole => NativeOwnerRole),
        }
    },
    {
        builder_method: identity_create,
        method_ident: NATIVE_IDENTITY_CREATE_IDENT,
        instruction: CallMethod,
        args: NativeIdentityCreateInput {}
    },
    {
        builder_method: identity_securify,
        method_ident: NATIVE_IDENTITY_SECURIFY_IDENT,
        instruction: CallMethod,
        args: NativeIdentitySecurifyToSingleBadgeInput {}
    },
    // ========
    // Package
    // ========
    {
        builder_method: package_claim_royalty,
        method_ident: NATIVE_PACKAGE_CLAIM_ROYALTIES_IDENT,
        instruction: CallMethod,
        args: NativePackageClaimRoyaltiesInput {}
    },
    // ==================
    // One Resource Pool
    // ==================
    {
        builder_method: one_resource_pool_instantiate,
        method_ident: NATIVE_ONE_RESOURCE_POOL_INSTANTIATE_IDENT,
        instruction: CallMethod,
        args: NativeOneResourcePoolInstantiateManifestInput {
            owner_role: (OwnerRole => NativeOwnerRole),
            pool_manager_rule: (Arc<AccessRule> => NativeAccessRule),
            resource_address: (Arc<Address> => NativeResourceAddress),
            address_reservation: (Option<ManifestBuilderAddressReservation> => Option<NativeManifestAddressReservation>)
        }
    },
    {
        builder_method: one_resource_pool_contribute,
        method_ident: NATIVE_ONE_RESOURCE_POOL_CONTRIBUTE_IDENT,
        instruction: CallMethod,
        args: NativeOneResourcePoolContributeManifestInput {
            bucket: (ManifestBuilderBucket => NativeManifestBucket)
        }
    },
    {
        builder_method: one_resource_pool_redeem,
        method_ident: NATIVE_ONE_RESOURCE_POOL_REDEEM_IDENT,
        instruction: CallMethod,
        args: NativeOneResourcePoolRedeemManifestInput {
            bucket: (ManifestBuilderBucket => NativeManifestBucket)
        }
    },
    {
        builder_method: one_resource_pool_protected_deposit,
        method_ident: NATIVE_ONE_RESOURCE_POOL_PROTECTED_DEPOSIT_IDENT,
        instruction: CallMethod,
        args: NativeOneResourcePoolProtectedDepositManifestInput {
            bucket: (ManifestBuilderBucket => NativeManifestBucket)
        }
    },
    {
        builder_method: one_resource_pool_protected_withdraw,
        method_ident: NATIVE_ONE_RESOURCE_POOL_PROTECTED_WITHDRAW_IDENT,
        instruction: CallMethod,
        args: NativeOneResourcePoolProtectedWithdrawManifestInput {
            amount: (Arc<Decimal> => NativeDecimal),
            withdraw_strategy: (WithdrawStrategy => NativeWithdrawStrategy)
        }
    },
    {
        builder_method: one_resource_pool_get_redemption_value,
        method_ident: NATIVE_ONE_RESOURCE_POOL_GET_REDEMPTION_VALUE_IDENT,
        instruction: CallMethod,
        args: NativeOneResourcePoolGetRedemptionValueManifestInput {
            amount_of_pool_units: (Arc<Decimal> => NativeDecimal),
        }
    },
    {
        builder_method: one_resource_pool_get_vault_amount,
        method_ident: NATIVE_ONE_RESOURCE_POOL_GET_VAULT_AMOUNT_IDENT,
        instruction: CallMethod,
        args: NativeOneResourcePoolGetVaultAmountManifestInput {}
    },
    // ==================
    // Two Resource Pool
    // ==================
    {
        builder_method: two_resource_pool_instantiate,
        method_ident: NATIVE_TWO_RESOURCE_POOL_INSTANTIATE_IDENT,
        instruction: CallMethod,
        args: NativeTwoResourcePoolInstantiateManifestInput {
            owner_role: (OwnerRole => NativeOwnerRole),
            pool_manager_rule: (Arc<AccessRule> => NativeAccessRule),
            resource_addresses: (Vec<Arc<Address>> => (NativeResourceAddress, NativeResourceAddress)),
            address_reservation: (Option<ManifestBuilderAddressReservation> => Option<NativeManifestAddressReservation>)
        }
    },
    {
        builder_method: two_resource_pool_contribute,
        method_ident: NATIVE_TWO_RESOURCE_POOL_CONTRIBUTE_IDENT,
        instruction: CallMethod,
        args: NativeTwoResourcePoolContributeManifestInput {
            buckets: (Vec<ManifestBuilderBucket> => (NativeManifestBucket, NativeManifestBucket))
        }
    },
    {
        builder_method: two_resource_pool_redeem,
        method_ident: NATIVE_TWO_RESOURCE_POOL_REDEEM_IDENT,
        instruction: CallMethod,
        args: NativeTwoResourcePoolRedeemManifestInput {
            bucket: (ManifestBuilderBucket => NativeManifestBucket)
        }
    },
    {
        builder_method: two_resource_pool_protected_deposit,
        method_ident: NATIVE_TWO_RESOURCE_POOL_PROTECTED_DEPOSIT_IDENT,
        instruction: CallMethod,
        args: NativeTwoResourcePoolProtectedDepositManifestInput {
            bucket: (ManifestBuilderBucket => NativeManifestBucket)
        }
    },
    {
        builder_method: two_resource_pool_protected_withdraw,
        method_ident: NATIVE_TWO_RESOURCE_POOL_PROTECTED_WITHDRAW_IDENT,
        instruction: CallMethod,
        args: NativeTwoResourcePoolProtectedWithdrawManifestInput {
            resource_address: (Arc<Address> => NativeResourceAddress),
            amount: (Arc<Decimal> => NativeDecimal),
            withdraw_strategy: (WithdrawStrategy => NativeWithdrawStrategy)
        }
    },
    {
        builder_method: two_resource_pool_get_redemption_value,
        method_ident: NATIVE_TWO_RESOURCE_POOL_GET_REDEMPTION_VALUE_IDENT,
        instruction: CallMethod,
        args: NativeTwoResourcePoolGetRedemptionValueManifestInput {
            amount_of_pool_units: (Arc<Decimal> => NativeDecimal),
        }
    },
    {
        builder_method: two_resource_pool_get_vault_amount,
        method_ident: NATIVE_TWO_RESOURCE_POOL_GET_VAULT_AMOUNTS_IDENT,
        instruction: CallMethod,
        args: NativeTwoResourcePoolGetVaultAmountsManifestInput {}
    },
    // ================
    // Metadata Module
    // ================
    {
        builder_method: metadata_set,
        method_ident: NATIVE_METADATA_SET_IDENT,
        instruction: CallMetadataMethod,
        args: NativeMetadataSetInput {
            key: (String => String),
            value: (MetadataValue => NativeMetadataValue)
        }
    },
    {
        builder_method: metadata_lock,
        method_ident: NATIVE_METADATA_LOCK_IDENT,
        instruction: CallMetadataMethod,
        args: NativeMetadataLockInput {
            key: (String => String),
        }
    },
    {
        builder_method: metadata_get,
        method_ident: NATIVE_METADATA_GET_IDENT,
        instruction: CallMetadataMethod,
        args: NativeMetadataGetInput {
            key: (String => String),
        }
    },
    {
        builder_method: metadata_remove,
        method_ident: NATIVE_METADATA_REMOVE_IDENT,
        instruction: CallMetadataMethod,
        args: NativeMetadataRemoveInput {
            key: (String => String),
        }
    },
    // =======================
    // Role Assignment Module
    // =======================
    {
        builder_method: role_assignment_get,
        method_ident: NATIVE_ROLE_ASSIGNMENT_GET_IDENT,
        instruction: CallRoleAssignmentMethod,
        args: NativeRoleAssignmentGetInput {
            module: (ModuleId => NativeObjectModuleId),
            role_key: (String => NativeRoleKey),
        }
    },
    {
        builder_method: role_assignment_set,
        method_ident: NATIVE_ROLE_ASSIGNMENT_SET_IDENT,
        instruction: CallRoleAssignmentMethod,
        args: NativeRoleAssignmentSetInput {
            module: (ModuleId => NativeObjectModuleId),
            role_key: (String => NativeRoleKey),
            rule: (Arc<AccessRule> => NativeAccessRule),
        }
    },
    {
        builder_method: role_assignment_set_owner,
        method_ident: NATIVE_ROLE_ASSIGNMENT_SET_OWNER_IDENT,
        instruction: CallRoleAssignmentMethod,
        args: NativeRoleAssignmentSetOwnerInput {
            rule: (Arc<AccessRule> => NativeAccessRule),
        }
    },
    {
        builder_method: role_assignment_lock_owner,
        method_ident: NATIVE_ROLE_ASSIGNMENT_LOCK_OWNER_IDENT,
        instruction: CallRoleAssignmentMethod,
        args: NativeRoleAssignmentLockOwnerInput {}
    },
    // ===============
    // Royalty Module
    // ===============
    {
        builder_method: royalty_set,
        method_ident: NATIVE_COMPONENT_ROYALTY_SET_ROYALTY_IDENT,
        instruction: CallRoyaltyMethod,
        args: NativeComponentRoyaltySetInput {
            method: (String => String),
            amount: (RoyaltyAmount => NativeRoyaltyAmount),
        }
    },
    {
        builder_method: royalty_lock,
        method_ident: NATIVE_COMPONENT_ROYALTY_LOCK_ROYALTY_IDENT,
        instruction: CallRoyaltyMethod,
        args: NativeComponentRoyaltyLockInput {
            method: (String => String),
        }
    },
    {
        builder_method: royalty_claim,
        method_ident: NATIVE_COMPONENT_ROYALTY_CLAIM_ROYALTIES_IDENT,
        instruction: CallRoyaltyMethod,
        args: NativeComponentClaimRoyaltiesInput {}
    },
}
