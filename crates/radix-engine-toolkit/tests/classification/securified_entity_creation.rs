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
use radix_transactions::manifest::{compile_manifest, MockBlobProvider};

/// A dApp's manifest creating an account held by an access controller whose
/// three roles each require 2 of 2 keys.
const STOKENET_MANIFEST: &str = r#"
CREATE_ACCOUNT;
TAKE_ALL_FROM_WORKTOP
    Address("resource_tdx_2_1nfxxxxxxxxxxaccwnrxxxxxxxxx006664022062xxxxxxxxx4vczzk")
    Bucket("bucket1")
;
CREATE_ACCESS_CONTROLLER
    Bucket("bucket1")
    Tuple(
        Enum<2u8>(Enum<0u8>(Enum<2u8>(2u8, Array<Enum>(
            Enum<0u8>(NonFungibleGlobalId("resource_tdx_2_1nfxxxxxxxxxxed25sgxxxxxxxxx002236757237xxxxxxxxx3e2cpa:[36454148d521bae3a09eabb4de6b470f9924e898f8cfa4e3625fd5cb81]")),
            Enum<0u8>(NonFungibleGlobalId("resource_tdx_2_1nfxxxxxxxxxxed25sgxxxxxxxxx002236757237xxxxxxxxx3e2cpa:[7122c0a037fa42221ba40b7c624f4bd2afe9aadbab361c1d4df0f07c62]"))
        )))),
        Enum<2u8>(Enum<0u8>(Enum<2u8>(2u8, Array<Enum>(
            Enum<0u8>(NonFungibleGlobalId("resource_tdx_2_1nfxxxxxxxxxxed25sgxxxxxxxxx002236757237xxxxxxxxx3e2cpa:[36454148d521bae3a09eabb4de6b470f9924e898f8cfa4e3625fd5cb81]")),
            Enum<0u8>(NonFungibleGlobalId("resource_tdx_2_1nfxxxxxxxxxxed25sgxxxxxxxxx002236757237xxxxxxxxx3e2cpa:[7122c0a037fa42221ba40b7c624f4bd2afe9aadbab361c1d4df0f07c62]"))
        )))),
        Enum<2u8>(Enum<0u8>(Enum<2u8>(2u8, Array<Enum>(
            Enum<0u8>(NonFungibleGlobalId("resource_tdx_2_1nfxxxxxxxxxxed25sgxxxxxxxxx002236757237xxxxxxxxx3e2cpa:[36454148d521bae3a09eabb4de6b470f9924e898f8cfa4e3625fd5cb81]")),
            Enum<0u8>(NonFungibleGlobalId("resource_tdx_2_1nfxxxxxxxxxxed25sgxxxxxxxxx002236757237xxxxxxxxx3e2cpa:[7122c0a037fa42221ba40b7c624f4bd2afe9aadbab361c1d4df0f07c62]"))
        ))))
    )
    Enum<1u8>(20160u32)
    Enum<0u8>()
;
"#;

fn static_classes(
    manifest: &TransactionManifestV1,
) -> Vec<ManifestClassification> {
    let StaticAnalysis {
        manifest_classification,
        ..
    } = statically_analyze(manifest).unwrap();
    manifest_classification.into_iter().collect()
}

fn is_classified(manifest: &TransactionManifestV1) -> bool {
    static_classes(manifest)
        .contains(&ManifestClassification::SecurifiedEntityCreation)
}

fn two_of_two_keys() -> AccessRule {
    let key = |byte| {
        NonFungibleGlobalId::from_public_key(&PublicKey::Ed25519(
            Ed25519PublicKey([byte; Ed25519PublicKey::LENGTH]),
        ))
    };
    AccessRule::Protected(CompositeRequirement::BasicRequirement(
        BasicRequirement::CountOf(
            2,
            vec![
                ResourceOrNonFungible::NonFungible(key(1)),
                ResourceOrNonFungible::NonFungible(key(2)),
            ],
        ),
    ))
}

fn rule_set() -> RuleSet {
    RuleSet {
        primary_role: two_of_two_keys(),
        recovery_role: two_of_two_keys(),
        confirmation_role: two_of_two_keys(),
    }
}

/// Creates the access controller holding the owner badge in `bucket`.
fn create_access_controller(
    builder: ManifestBuilder,
    bucket: &str,
) -> ManifestBuilder {
    let bucket = builder.bucket(bucket);
    builder.call_function(
        ACCESS_CONTROLLER_PACKAGE,
        ACCESS_CONTROLLER_BLUEPRINT,
        ACCESS_CONTROLLER_CREATE_IDENT,
        AccessControllerCreateManifestInput {
            controlled_asset: bucket,
            rule_set: rule_set().into(),
            timed_recovery_delay_in_minutes: Some(20_160),
            address_reservation: None,
        },
    )
}

/// The manifest of `STOKENET_MANIFEST`, with `extra` added before the access
/// controller is created.
fn account_creation_with(
    extra: impl FnOnce(ManifestBuilder) -> ManifestBuilder,
) -> TransactionManifestV1 {
    let builder = ManifestBuilder::new()
        .new_account()
        .take_all_from_worktop(ACCOUNT_OWNER_BADGE, "owner_badge");
    create_access_controller(extra(builder), "owner_badge").build()
}

#[test]
fn the_dapps_manifest_classifies_as_securified_entity_creation() {
    let manifest = compile_manifest::<TransactionManifestV1>(
        STOKENET_MANIFEST,
        &NetworkDefinition::stokenet(),
        MockBlobProvider,
    )
    .unwrap();

    let classes = static_classes(&manifest);

    assert!(
        classes.contains(&ManifestClassification::SecurifiedEntityCreation),
        "{classes:?}"
    );
    // The account isn't securified: it's created so.
    assert!(!classes.contains(&ManifestClassification::EntitySecurify));
}

#[test]
fn creating_an_account_shows_it_and_its_rules() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let manifest = account_creation_with(|builder| builder);

    // Act
    let (
        StaticAnalysis {
            manifest_classification,
            ..
        },
        DynamicAnalysis {
            detailed_manifest_classification,
            entities_newly_created_summary,
            ..
        },
    ) = ledger.analyze(manifest);

    // Assert
    assert!(manifest_classification
        .contains(&ManifestClassification::SecurifiedEntityCreation));
    let Some(DetailedManifestClassification::SecurifiedEntityCreation(
        SecurifiedEntityCreationOutput { created_entities },
    )) = detailed_manifest_classification.last()
    else {
        panic!("Not a securified entity creation")
    };
    let [created] = created_entities.as_slice() else {
        panic!("{created_entities:?}")
    };
    assert_eq!(created.rule_set, rule_set());
    assert_eq!(created.timed_recovery_delay_in_minutes, Some(20_160));
    // The account created, as the engine named its owner badge after it.
    assert_eq!(
        created.entity_address.as_node_id().entity_type(),
        Some(EntityType::GlobalAccount)
    );
    assert!(entities_newly_created_summary
        .new_global_entities
        .contains(&created.entity_address));
}

#[test]
fn creating_an_identity_and_an_account_shows_both() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let builder = ManifestBuilder::new()
        .create_identity()
        .take_all_from_worktop(IDENTITY_OWNER_BADGE, "identity_badge");
    let builder = create_access_controller(builder, "identity_badge")
        .new_account()
        .take_all_from_worktop(ACCOUNT_OWNER_BADGE, "account_badge");
    let manifest = create_access_controller(builder, "account_badge").build();

    // Act
    let (
        _,
        DynamicAnalysis {
            detailed_manifest_classification,
            entities_newly_created_summary,
            ..
        },
    ) = ledger.analyze(manifest);

    // Assert
    let Some(DetailedManifestClassification::SecurifiedEntityCreation(
        SecurifiedEntityCreationOutput { created_entities },
    )) = detailed_manifest_classification.last()
    else {
        panic!("Not a securified entity creation")
    };
    let entity_types = created_entities
        .iter()
        .map(|created| created.entity_address.as_node_id().entity_type())
        .collect::<Vec<_>>();
    assert_eq!(
        entity_types,
        [
            Some(EntityType::GlobalIdentity),
            Some(EntityType::GlobalAccount)
        ]
    );
    for created in created_entities {
        assert!(entities_newly_created_summary
            .new_global_entities
            .contains(&created.entity_address));
    }
}

/// The wallet adds the fee payment when it submits the manifest.
#[test]
fn paying_the_fee_keeps_it_classified() {
    let fee_payer = TestAddressAllocator::new().new_account_address();
    let manifest = ManifestBuilder::new()
        .lock_fee(fee_payer, 10)
        .new_account()
        .take_all_from_worktop(ACCOUNT_OWNER_BADGE, "owner_badge");
    let manifest = create_access_controller(manifest, "owner_badge").build();

    assert!(is_classified(&manifest));
}

/// An account created that isn't put under an access controller, here
/// deposited into another account, isn't securified.
#[test]
fn an_entity_left_without_an_access_controller_does_not_classify() {
    let other_account = TestAddressAllocator::new().new_account_address();
    let manifest = account_creation_with(|builder| {
        builder
            .new_account()
            .try_deposit_entire_worktop_or_abort(other_account, None)
    });

    assert!(!is_classified(&manifest));
}

/// Neither an account securified nor one created with `create_advanced`,
/// whose owner role is whatever it was given rather than an owner badge.
#[test]
fn other_ways_of_making_an_entity_do_not_classify() {
    let account = TestAddressAllocator::new().new_account_address();
    let extras: [fn(ManifestBuilder, ComponentAddress) -> ManifestBuilder; 2] = [
        |builder, account| {
            let builder = builder
                .call_method(
                    account,
                    ACCOUNT_SECURIFY_IDENT,
                    AccountSecurifyManifestInput {},
                )
                .take_all_from_worktop(ACCOUNT_OWNER_BADGE, "securified");
            create_access_controller(builder, "securified")
        },
        |builder, _| {
            builder.call_function(
                ACCOUNT_PACKAGE,
                ACCOUNT_BLUEPRINT,
                ACCOUNT_CREATE_ADVANCED_IDENT,
                AccountCreateAdvancedManifestInput {
                    owner_role: OwnerRole::None.into(),
                    address_reservation: None,
                },
            )
        },
    ];
    for extra in extras {
        let manifest = account_creation_with(|builder| extra(builder, account));
        assert!(!is_classified(&manifest));
    }
}

/// Nothing the output doesn't show: setting metadata, moving XRD or topping
/// up an access controller.
#[test]
fn anything_else_does_not_classify() {
    let account = TestAddressAllocator::new().new_account_address();
    let access_controller =
        TestAddressAllocator::new().new_access_controller_address();
    let extras: [fn(
        ManifestBuilder,
        ComponentAddress,
        ComponentAddress,
    ) -> ManifestBuilder; 4] = [
        |builder, account, _| {
            builder.set_metadata(account, "owner_keys", "not important")
        },
        |builder, account, _| {
            builder
                .withdraw_from_account(account, XRD, 10)
                .deposit_entire_worktop(account)
        },
        |builder, account, access_controller| {
            builder
                .withdraw_from_account(account, XRD, 10)
                .take_all_from_worktop(XRD, "xrd")
                .with_bucket("xrd", |builder, bucket| {
                    builder.call_method(
                        access_controller,
                        ACCESS_CONTROLLER_CONTRIBUTE_RECOVERY_FEE_IDENT,
                        AccessControllerContributeRecoveryFeeManifestInput {
                            bucket,
                        },
                    )
                })
        },
        |builder, _, _| {
            builder
                .create_proof_from_bucket_of_all("owner_badge", "proof")
                .push_to_auth_zone("proof")
                .drop_auth_zone_proofs()
        },
    ];
    for extra in extras {
        let manifest = account_creation_with(|builder| {
            extra(builder, account, access_controller)
        });
        assert!(!is_classified(&manifest));
    }
}

/// One access controller holding the owner badges of two accounts created
/// would be shown as holding one of them at most. Creating a second access
/// controller without an owner badge doesn't make up for it.
#[test]
fn an_access_controller_holding_several_owner_badges_does_not_classify() {
    let builder = ManifestBuilder::new()
        .new_account()
        .new_account()
        .take_all_from_worktop(ACCOUNT_OWNER_BADGE, "both_badges");
    let builder = create_access_controller(builder, "both_badges")
        .take_all_from_worktop(ACCOUNT_OWNER_BADGE, "no_badge");
    let manifest = create_access_controller(builder, "no_badge").build();

    assert!(!is_classified(&manifest));
}
