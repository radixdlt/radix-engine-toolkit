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

#[test]
fn multiple_pool_redemption_classifies_as_pool_redemption_transaction() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (pk, _, account) = ledger.new_account(true);

    let CreatedPoolEntities {
        resource_address1,
        resource_address2,
        one_resource_pool:
            PoolInformation {
                component_address: one_resource_pool,
                pool_unit_resource_address: one_resource_pool_unit,
            },
        two_resource_pool:
            PoolInformation {
                component_address: two_resource_pool,
                pool_unit_resource_address: two_resource_pool_unit,
            },
        multi_resource_pool:
            PoolInformation {
                component_address: multi_resource_pool,
                pool_unit_resource_address: multi_resource_pool_unit,
            },
    } = ledger.create_pool_entities(account);

    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .with_name_lookup(|builder, lookup| {
            builder
                .withdraw_from_account(account, resource_address1, 3)
                .withdraw_from_account(account, resource_address2, 2)
                .take_from_worktop(
                    resource_address1,
                    1,
                    "resource_address1_bucket1",
                )
                .take_from_worktop(
                    resource_address1,
                    1,
                    "resource_address1_bucket2",
                )
                .take_from_worktop(
                    resource_address1,
                    1,
                    "resource_address1_bucket3",
                )
                .take_from_worktop(
                    resource_address2,
                    1,
                    "resource_address2_bucket1",
                )
                .take_from_worktop(
                    resource_address2,
                    1,
                    "resource_address2_bucket2",
                )
                .call_method(
                    one_resource_pool,
                    ONE_RESOURCE_POOL_CONTRIBUTE_IDENT,
                    OneResourcePoolContributeManifestInput {
                        bucket: lookup.bucket("resource_address1_bucket1"),
                    },
                )
                .call_method(
                    two_resource_pool,
                    TWO_RESOURCE_POOL_CONTRIBUTE_IDENT,
                    TwoResourcePoolContributeManifestInput {
                        buckets: (
                            lookup.bucket("resource_address1_bucket2"),
                            lookup.bucket("resource_address2_bucket1"),
                        ),
                    },
                )
                .call_method(
                    multi_resource_pool,
                    MULTI_RESOURCE_POOL_CONTRIBUTE_IDENT,
                    MultiResourcePoolContributeManifestInput {
                        buckets: ManifestBucketBatch::ManifestBuckets(vec![
                            lookup.bucket("resource_address1_bucket3"),
                            lookup.bucket("resource_address2_bucket2"),
                        ]),
                    },
                )
                .try_deposit_entire_worktop_or_abort(account, None)
        })
        .build();
    ledger
        .execute_manifest(
            manifest,
            vec![NonFungibleGlobalId::from_public_key(&pk)],
        )
        .expect_commit_success();

    let manifest = ManifestBuilder::new()
        .with_name_lookup(|builder, lookup| {
            builder
                .withdraw_from_account(account, one_resource_pool_unit, 1)
                .withdraw_from_account(account, two_resource_pool_unit, 1)
                .withdraw_from_account(account, multi_resource_pool_unit, 1)
                .take_all_from_worktop(
                    one_resource_pool_unit,
                    "one_resource_pool_unit",
                )
                .take_all_from_worktop(
                    two_resource_pool_unit,
                    "two_resource_pool_unit",
                )
                .take_all_from_worktop(
                    multi_resource_pool_unit,
                    "multi_resource_pool_unit",
                )
                .call_method(
                    one_resource_pool,
                    ONE_RESOURCE_POOL_REDEEM_IDENT,
                    OneResourcePoolRedeemManifestInput {
                        bucket: lookup.bucket("one_resource_pool_unit"),
                    },
                )
                .call_method(
                    two_resource_pool,
                    TWO_RESOURCE_POOL_REDEEM_IDENT,
                    TwoResourcePoolRedeemManifestInput {
                        bucket: lookup.bucket("two_resource_pool_unit"),
                    },
                )
                .call_method(
                    multi_resource_pool,
                    MULTI_RESOURCE_POOL_REDEEM_IDENT,
                    MultiResourcePoolRedeemManifestInput {
                        bucket: lookup.bucket("multi_resource_pool_unit"),
                    },
                )
                .try_deposit_entire_worktop_or_abort(account, None)
        })
        .build();

    // Act
    let (
        StaticAnalysis {
            manifest_classification,
            ..
        },
        DynamicAnalysis {
            detailed_manifest_classification,
            ..
        },
    ) = ledger.analyze(manifest);

    // Assert
    assert!(manifest_classification
        .contains(&ManifestClassification::PoolRedemption));
    let Some(DetailedManifestClassification::PoolRedemption(
        PoolRedemptionOutput {
            redemption_operations,
        },
    )) = detailed_manifest_classification.last()
    else {
        panic!("Not a pool redemption transaction")
    };
    assert_eq!(redemption_operations.len(), 3);

    let one_resource_pool_redemption = redemption_operations.first().unwrap();
    let two_resource_pool_redemption = redemption_operations.get(1).unwrap();
    let multi_resource_pool_redemption = redemption_operations.get(2).unwrap();

    assert_eq!(one_resource_pool_redemption.pool_address, one_resource_pool);
    assert_eq!(
        one_resource_pool_redemption.redeemed_resources,
        indexmap! {
            resource_address1 => dec!(1)
        }
    );
    assert_eq!(
        one_resource_pool_redemption.pool_units_resource_address,
        one_resource_pool_unit
    );
    assert_eq!(one_resource_pool_redemption.pool_units_amount, dec!(1));

    assert_eq!(two_resource_pool_redemption.pool_address, two_resource_pool);
    assert_eq!(
        two_resource_pool_redemption.redeemed_resources,
        indexmap! {
            resource_address1 => dec!(1),
            resource_address2 => dec!(1),
        }
    );
    assert_eq!(
        two_resource_pool_redemption.pool_units_resource_address,
        two_resource_pool_unit
    );
    assert_eq!(two_resource_pool_redemption.pool_units_amount, dec!(1));

    assert_eq!(
        multi_resource_pool_redemption.pool_address,
        multi_resource_pool
    );
    assert_eq!(
        multi_resource_pool_redemption.redeemed_resources,
        indexmap! {
            resource_address1 => dec!(1),
            resource_address2 => dec!(1),
        }
    );
    assert_eq!(
        multi_resource_pool_redemption.pool_units_resource_address,
        multi_resource_pool_unit
    );
    assert_eq!(multi_resource_pool_redemption.pool_units_amount, dec!(1));
}

#[test]
fn transfer_in_pool_redemption_transaction_qualifies_for_classification_but_not_detailed_classification(
) {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (pk, _, account) = ledger.new_account(true);

    let CreatedPoolEntities {
        resource_address1,
        resource_address2,
        one_resource_pool:
            PoolInformation {
                component_address: one_resource_pool,
                pool_unit_resource_address: one_resource_pool_unit,
            },
        two_resource_pool:
            PoolInformation {
                component_address: two_resource_pool,
                pool_unit_resource_address: two_resource_pool_unit,
            },
        multi_resource_pool:
            PoolInformation {
                component_address: multi_resource_pool,
                pool_unit_resource_address: multi_resource_pool_unit,
            },
    } = ledger.create_pool_entities(account);

    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .with_name_lookup(|builder, lookup| {
            builder
                .withdraw_from_account(account, resource_address1, 3)
                .withdraw_from_account(account, resource_address2, 2)
                .take_from_worktop(
                    resource_address1,
                    1,
                    "resource_address1_bucket1",
                )
                .take_from_worktop(
                    resource_address1,
                    1,
                    "resource_address1_bucket2",
                )
                .take_from_worktop(
                    resource_address1,
                    1,
                    "resource_address1_bucket3",
                )
                .take_from_worktop(
                    resource_address2,
                    1,
                    "resource_address2_bucket1",
                )
                .take_from_worktop(
                    resource_address2,
                    1,
                    "resource_address2_bucket2",
                )
                .call_method(
                    one_resource_pool,
                    ONE_RESOURCE_POOL_CONTRIBUTE_IDENT,
                    OneResourcePoolContributeManifestInput {
                        bucket: lookup.bucket("resource_address1_bucket1"),
                    },
                )
                .call_method(
                    two_resource_pool,
                    TWO_RESOURCE_POOL_CONTRIBUTE_IDENT,
                    TwoResourcePoolContributeManifestInput {
                        buckets: (
                            lookup.bucket("resource_address1_bucket2"),
                            lookup.bucket("resource_address2_bucket1"),
                        ),
                    },
                )
                .call_method(
                    multi_resource_pool,
                    MULTI_RESOURCE_POOL_CONTRIBUTE_IDENT,
                    MultiResourcePoolContributeManifestInput {
                        buckets: ManifestBucketBatch::ManifestBuckets(vec![
                            lookup.bucket("resource_address1_bucket3"),
                            lookup.bucket("resource_address2_bucket2"),
                        ]),
                    },
                )
                .try_deposit_entire_worktop_or_abort(account, None)
        })
        .build();
    ledger
        .execute_manifest(
            manifest,
            vec![NonFungibleGlobalId::from_public_key(&pk)],
        )
        .expect_commit_success();

    let manifest = ManifestBuilder::new()
        .with_name_lookup(|builder, lookup| {
            builder
                .withdraw_from_account(account, XRD, 1)
                .withdraw_from_account(account, one_resource_pool_unit, 1)
                .withdraw_from_account(account, two_resource_pool_unit, 1)
                .withdraw_from_account(account, multi_resource_pool_unit, 1)
                .take_all_from_worktop(
                    one_resource_pool_unit,
                    "one_resource_pool_unit",
                )
                .take_all_from_worktop(
                    two_resource_pool_unit,
                    "two_resource_pool_unit",
                )
                .take_all_from_worktop(
                    multi_resource_pool_unit,
                    "multi_resource_pool_unit",
                )
                .call_method(
                    one_resource_pool,
                    ONE_RESOURCE_POOL_REDEEM_IDENT,
                    OneResourcePoolRedeemManifestInput {
                        bucket: lookup.bucket("one_resource_pool_unit"),
                    },
                )
                .call_method(
                    two_resource_pool,
                    TWO_RESOURCE_POOL_REDEEM_IDENT,
                    TwoResourcePoolRedeemManifestInput {
                        bucket: lookup.bucket("two_resource_pool_unit"),
                    },
                )
                .call_method(
                    multi_resource_pool,
                    MULTI_RESOURCE_POOL_REDEEM_IDENT,
                    MultiResourcePoolRedeemManifestInput {
                        bucket: lookup.bucket("multi_resource_pool_unit"),
                    },
                )
                .try_deposit_entire_worktop_or_abort(account, None)
        })
        .build();

    // Act
    let (
        StaticAnalysis {
            manifest_classification,
            ..
        },
        DynamicAnalysis {
            detailed_manifest_classification,
            ..
        },
    ) = ledger.analyze(manifest);

    // Assert
    assert!(manifest_classification
        .contains(&ManifestClassification::PoolRedemption));
    assert!(!detailed_manifest_classification.iter().any(
        |classification| matches!(
            classification,
            DetailedManifestClassification::PoolRedemption(..)
        )
    ));
}
