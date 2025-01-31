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
fn transfer_in_pool_contribution_transaction_disqualifies_classification() {
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
    assert!(!manifest_classification
        .contains(&ManifestClassification::PoolContribution));
    assert!(!detailed_manifest_classification.iter().any(
        |classification| matches!(
            classification,
            DetailedManifestClassification::PoolContribution(..)
        )
    ));
}
