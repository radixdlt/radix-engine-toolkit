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

    /* Faucet */

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

    /* Account */

    pub fn withdraw_from_account(
        self: Arc<Self>,
        account_address: Arc<Address>,
        resource_address: Arc<Address>,
        amount: Arc<Decimal>,
    ) -> Result<Arc<Self>> {
        builder_arc_map(self, |builder| {
            let account_address = NativeGlobalAddress::try_from(*account_address)?;
            let resource_address = NativeResourceAddress::try_from(*resource_address)?;
            let amount = amount.0;

            let instruction = NativeInstruction::CallMethod {
                address: NativeDynamicGlobalAddress::Static(account_address),
                method_name: NATIVE_ACCOUNT_WITHDRAW_IDENT.to_owned(),
                args: native_to_manifest_value_and_unwrap!(&NativeAccountWithdrawInput {
                    resource_address,
                    amount
                }),
            };
            builder.instructions.push(instruction);
            Ok(())
        })
    }

    pub fn withdraw_non_fungibles_from_account(
        self: Arc<Self>,
        account_address: Arc<Address>,
        resource_address: Arc<Address>,
        ids: Vec<NonFungibleLocalId>,
    ) -> Result<Arc<Self>> {
        builder_arc_map(self, |builder| {
            let account_address = NativeGlobalAddress::try_from(*account_address)?;
            let resource_address = NativeResourceAddress::try_from(*resource_address)?;
            let ids = ids
                .into_iter()
                .map(TryFrom::try_from)
                .collect::<Result<_>>()?;

            let instruction = NativeInstruction::CallMethod {
                address: NativeDynamicGlobalAddress::Static(account_address),
                method_name: NATIVE_ACCOUNT_WITHDRAW_NON_FUNGIBLES_IDENT.to_owned(),
                args: native_to_manifest_value_and_unwrap!(
                    &NativeAccountWithdrawNonFungiblesInput {
                        resource_address,
                        ids
                    }
                ),
            };
            builder.instructions.push(instruction);
            Ok(())
        })
    }

    pub fn create_account_advanced(self: Arc<Self>, owner_role: OwnerRole) -> Result<Arc<Self>> {
        builder_arc_map(self, |builder| {
            let owner_role = owner_role.to_native()?;

            let instruction = NativeInstruction::CallFunction {
                package_address: NativeDynamicPackageAddress::Static(NATIVE_ACCOUNT_PACKAGE),
                blueprint_name: NATIVE_ACCOUNT_BLUEPRINT.to_owned(),
                function_name: NATIVE_ACCOUNT_CREATE_ADVANCED_IDENT.to_owned(),
                args: native_to_manifest_value_and_unwrap!(
                    &NativeAccountCreateAdvancedManifestInput {
                        owner_role,
                        address_reservation: None
                    }
                ),
            };
            builder.instructions.push(instruction);
            Ok(())
        })
    }

    pub fn account_deposit(
        self: Arc<Self>,
        account_address: Arc<Address>,
        bucket: ManifestBuilderBucket,
    ) -> Result<Arc<Self>> {
        builder_arc_map(self, |builder| {
            let address = NativeGlobalAddress::try_from(*account_address)?;
            let bucket = builder.name_record.get_bucket(&bucket.name)?;

            let instruction = NativeInstruction::CallMethod {
                address: NativeDynamicGlobalAddress::Static(address),
                method_name: NATIVE_ACCOUNT_DEPOSIT_IDENT.to_owned(),
                args: manifest_args!(*bucket).into(),
            };
            builder.instructions.push(instruction);
            Ok(())
        })
    }

    pub fn account_try_deposit_or_abort(
        self: Arc<Self>,
        account_address: Arc<Address>,
        authorized_depositor_badge: Option<ResourceOrNonFungible>,
        bucket: ManifestBuilderBucket,
    ) -> Result<Arc<Self>> {
        builder_arc_map(self, |builder| {
            let address = NativeGlobalAddress::try_from(*account_address)?;
            let bucket = builder.name_record.get_bucket(&bucket.name)?;
            let authorized_depositor_badge = if let Some(badge) = authorized_depositor_badge {
                Some(badge.to_native()?)
            } else {
                None
            };

            let instruction = NativeInstruction::CallMethod {
                address: NativeDynamicGlobalAddress::Static(address),
                method_name: NATIVE_ACCOUNT_TRY_DEPOSIT_OR_ABORT_IDENT.to_owned(),
                args: manifest_args!(*bucket, authorized_depositor_badge).into(),
            };
            builder.instructions.push(instruction);
            Ok(())
        })
    }

    pub fn account_try_deposit_or_refund(
        self: Arc<Self>,
        account_address: Arc<Address>,
        authorized_depositor_badge: Option<ResourceOrNonFungible>,
        bucket: ManifestBuilderBucket,
    ) -> Result<Arc<Self>> {
        builder_arc_map(self, |builder| {
            let address = NativeGlobalAddress::try_from(*account_address)?;
            let bucket = builder.name_record.get_bucket(&bucket.name)?;
            let authorized_depositor_badge = if let Some(badge) = authorized_depositor_badge {
                Some(badge.to_native()?)
            } else {
                None
            };

            let instruction = NativeInstruction::CallMethod {
                address: NativeDynamicGlobalAddress::Static(address),
                method_name: NATIVE_ACCOUNT_TRY_DEPOSIT_OR_REFUND_IDENT.to_owned(),
                args: manifest_args!(*bucket, authorized_depositor_badge).into(),
            };
            builder.instructions.push(instruction);
            Ok(())
        })
    }

    pub fn account_deposit_batch(
        self: Arc<Self>,
        account_address: Arc<Address>,
    ) -> Result<Arc<Self>> {
        builder_arc_map(self, |builder| {
            let address = NativeGlobalAddress::try_from(*account_address)?;

            let instruction = NativeInstruction::CallMethod {
                address: NativeDynamicGlobalAddress::Static(address),
                method_name: NATIVE_ACCOUNT_DEPOSIT_BATCH_IDENT.to_owned(),
                args: manifest_args!(NativeManifestExpression::EntireWorktop,).into(),
            };
            builder.instructions.push(instruction);
            Ok(())
        })
    }

    pub fn account_try_deposit_batch_or_abort(
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

    pub fn account_try_deposit_batch_or_refund(
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

    /* Package */

    pub fn publish_package(
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

    /* Access Controller */

    pub fn access_controller_initiate_recovery(
        self: Arc<Self>,
        access_controller_address: Arc<Address>,
        proposer: Proposer,
        proposed_primary_role: Arc<AccessRule>,
        proposed_recovery_role: Arc<AccessRule>,
        proposed_confirmation_role: Arc<AccessRule>,
        proposed_timed_recovery_delay_in_minutes: Option<u32>,
    ) -> Result<Arc<Self>> {
        builder_arc_map(self, |builder| {
            let rule_set = NativeRuleSet {
                primary_role: proposed_primary_role.0.clone(),
                recovery_role: proposed_recovery_role.0.clone(),
                confirmation_role: proposed_confirmation_role.0.clone(),
            };

            let (method_name, args) = match proposer {
                Proposer::Primary => (
                    NATIVE_ACCESS_CONTROLLER_INITIATE_RECOVERY_AS_PRIMARY_IDENT,
                    native_to_manifest_value_and_unwrap!(
                        &NativeAccessControllerInitiateRecoveryAsPrimaryInput {
                            rule_set,
                            timed_recovery_delay_in_minutes:
                                proposed_timed_recovery_delay_in_minutes,
                        }
                    ),
                ),
                Proposer::Recovery => (
                    NATIVE_ACCESS_CONTROLLER_INITIATE_RECOVERY_AS_RECOVERY_IDENT,
                    native_to_manifest_value_and_unwrap!(
                        &NativeAccessControllerInitiateRecoveryAsRecoveryInput {
                            rule_set,
                            timed_recovery_delay_in_minutes:
                                proposed_timed_recovery_delay_in_minutes,
                        }
                    ),
                ),
            };

            let instruction = NativeInstruction::CallMethod {
                address: NativeDynamicGlobalAddress::Static(
                    (*access_controller_address).try_into()?,
                ),
                method_name: method_name.to_owned(),
                args,
            };
            builder.instructions.push(instruction);

            Ok(())
        })
    }

    pub fn access_controller_quick_confirm_recovery(
        self: Arc<Self>,
        access_controller_address: Arc<Address>,
        proposer: Proposer,
        proposed_primary_role: Arc<AccessRule>,
        proposed_recovery_role: Arc<AccessRule>,
        proposed_confirmation_role: Arc<AccessRule>,
        proposed_timed_recovery_delay_in_minutes: Option<u32>,
    ) -> Result<Arc<Self>> {
        builder_arc_map(self, |builder| {
            let rule_set = NativeRuleSet {
                primary_role: proposed_primary_role.0.clone(),
                recovery_role: proposed_recovery_role.0.clone(),
                confirmation_role: proposed_confirmation_role.0.clone(),
            };

            let (method_name, args) = match proposer {
                Proposer::Primary => (
                    NATIVE_ACCESS_CONTROLLER_QUICK_CONFIRM_PRIMARY_ROLE_RECOVERY_PROPOSAL_IDENT,
                    native_to_manifest_value_and_unwrap!(
                        &NativeAccessControllerQuickConfirmPrimaryRoleRecoveryProposalInput {
                            rule_set,
                            timed_recovery_delay_in_minutes:
                                proposed_timed_recovery_delay_in_minutes,
                        }
                    ),
                ),
                Proposer::Recovery => (
                    NATIVE_ACCESS_CONTROLLER_QUICK_CONFIRM_RECOVERY_ROLE_RECOVERY_PROPOSAL_IDENT,
                    native_to_manifest_value_and_unwrap!(
                        &NativeAccessControllerQuickConfirmRecoveryRoleRecoveryProposalInput {
                            rule_set,
                            timed_recovery_delay_in_minutes:
                                proposed_timed_recovery_delay_in_minutes,
                        }
                    ),
                ),
            };

            let instruction = NativeInstruction::CallMethod {
                address: NativeDynamicGlobalAddress::Static(
                    (*access_controller_address).try_into()?,
                ),
                method_name: method_name.to_owned(),
                args,
            };
            builder.instructions.push(instruction);

            Ok(())
        })
    }

    pub fn create_signature_based_access_controller(
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

    pub fn create_access_controller_with_securify_structure(
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

    /* Access Rule */

    pub fn set_role(
        self: Arc<Self>,
        address: Arc<Address>,
        module: ObjectModuleId,
        role_key: String,
        rule: Arc<AccessRule>,
    ) -> Result<Arc<Self>> {
        builder_arc_map(self, |builder| {
            let address = NativeGlobalAddress::try_from(*address)?;
            let module = NativeObjectModuleId::from(module);
            let rule = rule.0.clone();

            let instruction = NativeInstruction::CallRoleAssignmentMethod {
                address: NativeDynamicGlobalAddress::Static(address),
                method_name: NATIVE_ROLE_ASSIGNMENT_SET_IDENT.to_owned(),
                args: native_to_manifest_value_and_unwrap!(&NativeRoleAssignmentSetInput {
                    module,
                    role_key: NativeRoleKey { key: role_key },
                    rule
                }),
            };
            builder.instructions.push(instruction);

            Ok(())
        })
    }

    /* Metadata Module */

    pub fn set_metadata(
        self: Arc<Self>,
        address: Arc<Address>,
        key: String,
        value: MetadataValue,
    ) -> Result<Arc<Self>> {
        builder_arc_map(self, |builder| {
            let address = NativeGlobalAddress::try_from(*address)?;
            let value = value.to_native()?;

            let instruction = NativeInstruction::CallMetadataMethod {
                address: NativeDynamicGlobalAddress::Static(address),
                method_name: NATIVE_METADATA_SET_IDENT.to_string(),
                args: native_to_manifest_value_and_unwrap!(&NativeMetadataSetInput { key, value }),
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
