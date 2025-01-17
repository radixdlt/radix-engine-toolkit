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
fn multiple_pool_contributions_classifies_as_pool_contribution_transaction() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (_, _, account) = ledger.new_account(true);

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
        .contains(&ManifestClassification::PoolContribution));
    let Some(DetailedManifestClassification::PoolContribution(
        PoolContributionOutput {
            contribution_operations,
        },
    )) = detailed_manifest_classification.last()
    else {
        panic!("Not a pool contribution transaction")
    };
    assert_eq!(contribution_operations.len(), 3);

    let one_resource_pool_contribution =
        contribution_operations.first().unwrap();
    let two_resource_pool_contribution =
        contribution_operations.get(1).unwrap();
    let multi_resource_pool_contribution =
        contribution_operations.get(2).unwrap();

    assert_eq!(
        one_resource_pool_contribution.pool_address,
        one_resource_pool
    );
    assert_eq!(
        one_resource_pool_contribution.contributed_resources,
        indexmap! {
            resource_address1 => dec!(1)
        }
    );
    assert_eq!(
        one_resource_pool_contribution.pool_units_resource_address,
        one_resource_pool_unit
    );
    assert_eq!(one_resource_pool_contribution.pool_units_amount, dec!(1));

    assert_eq!(
        two_resource_pool_contribution.pool_address,
        two_resource_pool
    );
    assert_eq!(
        two_resource_pool_contribution.contributed_resources,
        indexmap! {
            resource_address1 => dec!(1),
            resource_address2 => dec!(1),
        }
    );
    assert_eq!(
        two_resource_pool_contribution.pool_units_resource_address,
        two_resource_pool_unit
    );
    assert_eq!(two_resource_pool_contribution.pool_units_amount, dec!(1));

    assert_eq!(
        multi_resource_pool_contribution.pool_address,
        multi_resource_pool
    );
    assert_eq!(
        multi_resource_pool_contribution.contributed_resources,
        indexmap! {
            resource_address1 => dec!(1),
            resource_address2 => dec!(1),
        }
    );
    assert_eq!(
        multi_resource_pool_contribution.pool_units_resource_address,
        multi_resource_pool_unit
    );
    assert_eq!(multi_resource_pool_contribution.pool_units_amount, dec!(1));
}

#[test]
fn transfer_in_pool_contribution_transaction_qualifies_for_classification_but_not_detailed_classification(
) {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (_, _, account) = ledger.new_account(true);

    let CreatedPoolEntities {
        resource_address1,
        resource_address2,
        one_resource_pool:
            PoolInformation {
                component_address: one_resource_pool,
                pool_unit_resource_address: _,
            },
        two_resource_pool:
            PoolInformation {
                component_address: two_resource_pool,
                pool_unit_resource_address: _,
            },
        multi_resource_pool:
            PoolInformation {
                component_address: multi_resource_pool,
                pool_unit_resource_address: _,
            },
    } = ledger.create_pool_entities(account);

    let manifest = ManifestBuilder::new()
        .with_name_lookup(|builder, lookup| {
            builder
                .withdraw_from_account(account, resource_address1, 3)
                .withdraw_from_account(account, resource_address2, 3)
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
        .contains(&ManifestClassification::PoolContribution));
    assert!(!detailed_manifest_classification.iter().any(
        |classification| matches!(
            classification,
            DetailedManifestClassification::PoolContribution(..)
        )
    ));
}
