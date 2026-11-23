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

//! Access controller calls whose effect can't be shown to the user aren't of
//! any access controller class.
//!
//! Addresses only the manifest knows come from allocating them, which no
//! access controller class permits either, so these tests also guard against
//! permitting it later.

use super::access_controller_utils::*;
use crate::prelude::*;
use radix_transactions::manifest::{compile_manifest, MockBlobProvider};

fn classes_of(manifest: &str) -> Vec<ManifestClassification> {
    let manifest = compile_manifest::<TransactionManifestV1>(
        manifest,
        &NetworkDefinition::simulator(),
        MockBlobProvider,
    )
    .unwrap();
    access_controller_classes(&manifest)
}

fn encode(address: impl Into<GlobalAddress>) -> String {
    AddressBech32Encoder::for_simulator()
        .encode(address.into().as_bytes())
        .unwrap()
}

/// The rules can't be shown: the resource they require only gets an address
/// in the manifest.
#[test]
fn proposing_rules_requiring_a_resource_of_the_manifest_is_not_classified() {
    let access_controller =
        encode(TestAddressAllocator::new().new_access_controller_address());
    let package = encode(RESOURCE_PACKAGE);
    let rule = r#"Enum<AccessRule::Protected>(
        Enum<CompositeRequirement::BasicRequirement>(
            Enum<BasicRequirement::Require>(
                Enum<ResourceOrNonFungible::Resource>(NamedAddress("resource"))
            )
        )
    )"#;
    let rule_set = format!(
        "Tuple({rule}, Enum<AccessRule::AllowAll>(), Enum<AccessRule::AllowAll>())"
    );
    let manifest = format!(
        r#"
        ALLOCATE_GLOBAL_ADDRESS
            Address("{package}")
            "NonFungibleResourceManager"
            AddressReservation("reservation")
            NamedAddress("resource");
        CALL_METHOD
            Address("{access_controller}")
            "initiate_recovery_as_recovery"
            {rule_set}
            None;
        CALL_METHOD
            Address("{access_controller}")
            "quick_confirm_recovery_role_recovery_proposal"
            {rule_set}
            None;
        "#
    );
    assert_eq!(classes_of(&manifest), vec![]);
}

/// Every call is made on an access controller only the manifest knows.
#[test]
fn calling_an_access_controller_of_the_manifest_is_not_classified() {
    let calls: [fn(ManifestBuilder, ManifestNamedAddress) -> ManifestBuilder;
        3] = [
        |builder, access_controller| {
            builder.call_method(
                access_controller,
                ACCESS_CONTROLLER_LOCK_PRIMARY_ROLE_IDENT,
                (),
            )
        },
        |builder, access_controller| {
            builder.call_method(
                access_controller,
                ACCESS_CONTROLLER_CANCEL_PRIMARY_ROLE_RECOVERY_PROPOSAL_IDENT,
                (),
            )
        },
        |builder, access_controller| {
            builder.call_method(
                access_controller,
                ACCESS_CONTROLLER_INITIATE_RECOVERY_AS_RECOVERY_IDENT,
                AccessControllerInitiateRecoveryAsRecoveryManifestInput {
                    rule_set: allow_all().into(),
                    timed_recovery_delay_in_minutes: None,
                },
            )
        },
    ];
    for call in calls {
        let manifest = ManifestBuilder::new()
            .allocate_global_address(
                ACCESS_CONTROLLER_PACKAGE,
                ACCESS_CONTROLLER_BLUEPRINT,
                "reservation",
                "access_controller",
            )
            .then(|builder| {
                let access_controller =
                    builder.named_address("access_controller");
                call(builder, access_controller)
            })
            // The reservation is never used: the access controller isn't
            // created.
            .build_no_validate();
        assert_access_controller_class(manifest, None);
    }
}

#[test]
fn contributing_other_resources_than_xrd_is_not_classified() {
    let mut allocator = TestAddressAllocator::new();
    let account = allocator.new_account_address();
    let access_controller = allocator.new_access_controller_address();
    let resource = ResourceAddress::new_or_panic(
        allocator
            .new_node_id(EntityType::GlobalFungibleResourceManager)
            .0,
    );
    let manifest = ManifestBuilder::new()
        .lock_primary_role(access_controller)
        .withdraw_from_account(account, resource, 100)
        .take_from_worktop(resource, 100, "bucket")
        .then(|builder| {
            let bucket = builder.bucket("bucket");
            builder.call_method(
                access_controller,
                ACCESS_CONTROLLER_CONTRIBUTE_RECOVERY_FEE_IDENT,
                AccessControllerContributeRecoveryFeeManifestInput { bucket },
            )
        })
        .build();
    assert_access_controller_class(manifest, None);
}

#[test]
fn locking_the_fee_of_an_access_controller_is_permitted() {
    let mut allocator = TestAddressAllocator::new();
    let access_controller = allocator.new_access_controller_address();
    let manifest = ManifestBuilder::new()
        .call_method(
            access_controller,
            ACCESS_CONTROLLER_LOCK_RECOVERY_FEE_IDENT,
            AccessControllerLockRecoveryFeeManifestInput {
                amount: Decimal::one(),
            },
        )
        .call_method(
            access_controller,
            ACCESS_CONTROLLER_CREATE_PROOF_IDENT,
            (),
        )
        .lock_primary_role(access_controller)
        .build();
    assert_access_controller_class(
        manifest,
        Some(ManifestClassification::AccessControllerPrimaryRoleLock),
    );
}

#[test]
fn other_access_controller_methods_are_not_classified() {
    let access_controller =
        TestAddressAllocator::new().new_access_controller_address();
    for (method, arguments) in [
        (
            ACCESS_CONTROLLER_INITIATE_BADGE_WITHDRAW_ATTEMPT_AS_PRIMARY_IDENT,
            manifest_args!(),
        ),
        (
            ACCESS_CONTROLLER_INITIATE_BADGE_WITHDRAW_ATTEMPT_AS_RECOVERY_IDENT,
            manifest_args!(),
        ),
        (
            ACCESS_CONTROLLER_CANCEL_PRIMARY_ROLE_BADGE_WITHDRAW_ATTEMPT_IDENT,
            manifest_args!(),
        ),
        (
            ACCESS_CONTROLLER_CANCEL_RECOVERY_ROLE_BADGE_WITHDRAW_ATTEMPT_IDENT,
            manifest_args!(),
        ),
        // These two hand the controlled badge out of the access controller.
        (
            ACCESS_CONTROLLER_QUICK_CONFIRM_PRIMARY_ROLE_BADGE_WITHDRAW_ATTEMPT_IDENT,
            manifest_args!(),
        ),
        (
            ACCESS_CONTROLLER_QUICK_CONFIRM_RECOVERY_ROLE_BADGE_WITHDRAW_ATTEMPT_IDENT,
            manifest_args!(),
        ),
        (
            ACCESS_CONTROLLER_MINT_RECOVERY_BADGES_IDENT,
            manifest_args!(IndexSet::<NonFungibleLocalId>::new()),
        ),
    ] {
        let manifest = ManifestBuilder::new()
            .lock_primary_role(access_controller)
            .call_method_raw(access_controller, method, arguments.into())
            .build();
        assert_access_controller_class(manifest, None);
    }
}
