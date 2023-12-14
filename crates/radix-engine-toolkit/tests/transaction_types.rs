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

use radix_engine::transaction::*;
use radix_engine::vm::*;
use radix_engine_interface::blueprints::pool::*;
use radix_engine_queries::typed_substate_layout::multi_resource_pool::*;
use radix_engine_queries::typed_substate_layout::one_resource_pool::*;
use radix_engine_queries::typed_substate_layout::two_resource_pool::*;
use radix_engine_toolkit::transaction_types::*;
use scrypto_unit::*;
use transaction::prelude::*;

macro_rules! assert_eq_three {
    (
        $item1: expr,
        $item2: expr,
        $item3: expr $(,)?
    ) => {
        assert_eq!($item1, $item2);
        assert_eq!($item2, $item3);
    };
    (
        $item1: expr,
        $item2: expr,
        $item3: expr,
        $($tokens: tt)?
    ) => {
        assert_eq!($item1, $item2, $($tokens)*);
        assert_eq!($item2, $item3, $($tokens)*);
    };
}

#[test]
fn empty_manifest_matches_none_of_the_transaction_types() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().without_trace().build();
    let manifest = ManifestBuilder::new().build();

    // Act
    let (manifest_summary, execution_summary) = test_runner.summarize(manifest);

    // Assert
    assert_eq!(manifest_summary.classification.len(), 0);
    assert_eq!(execution_summary.detailed_classification.len(), 0);
}

#[test]
fn simple_transfer_satisfies_the_transfer_and_general_transaction_types() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().without_trace().build();

    let (_, _, account1) = test_runner.new_account(false);
    let (_, _, account2) = test_runner.new_account(false);

    let manifest = ManifestBuilder::new()
        .withdraw_from_account(account1, XRD, 10)
        .take_from_worktop(XRD, 10, "xrd")
        .try_deposit_or_abort(account2, None, "xrd")
        .build();

    // Act
    let (manifest_summary, execution_summary) = test_runner.summarize(manifest);

    // Assert
    assert_eq_three!(
        manifest_summary.presented_proofs.len(),
        execution_summary.presented_proofs.len(),
        0
    );
    assert_eq_three!(
        manifest_summary.encountered_entities,
        execution_summary.encountered_entities,
        indexset![
            GlobalAddress::from(account1),
            GlobalAddress::from(XRD),
            GlobalAddress::from(account2),
        ]
    );
    assert_eq_three!(
        manifest_summary.accounts_requiring_auth,
        execution_summary.accounts_requiring_auth,
        indexset![account1]
    );
    assert_eq_three!(
        manifest_summary.identities_requiring_auth,
        execution_summary.identities_requiring_auth,
        indexset![]
    );
    assert_eq_three!(
        manifest_summary.reserved_instructions,
        execution_summary.reserved_instructions,
        indexset![]
    );
    assert_eq_three!(
        manifest_summary.classification.len(),
        execution_summary.detailed_classification.len(),
        2
    );

    assert_eq!(
        manifest_summary.accounts_withdrawn_from,
        indexset![account1]
    );
    assert_eq!(
        manifest_summary.accounts_deposited_into,
        indexset![account2]
    );
    assert_eq!(
        manifest_summary.classification,
        indexset![ManifestClass::Transfer, ManifestClass::General]
    );

    assert_eq!(
        execution_summary.account_withdraws,
        indexmap! {
            account1 => vec![
                ResourceIndicator::Fungible(XRD, FungibleResourceIndicator::Guaranteed(dec!(10)))
            ]
        }
    );
    assert_eq!(
        execution_summary.account_deposits,
        indexmap! {
            account2 => vec![
                ResourceIndicator::Fungible(XRD, FungibleResourceIndicator::Guaranteed(dec!(10)))
            ]
        }
    );
    assert_eq!(execution_summary.new_entities, NewEntities::default());
    assert!(matches!(
        execution_summary.detailed_classification[0],
        DetailedManifestClass::Transfer {
            is_one_to_one: true
        }
    ));
    assert!(matches!(
        execution_summary.detailed_classification[1],
        DetailedManifestClass::General
    ));
}

#[test]
fn non_simple_transfer_satisfies_the_transfer_and_general_transaction_types() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().without_trace().build();

    let (_, _, account1) = test_runner.new_account(false);
    let (_, _, account2) = test_runner.new_account(false);

    let manifest = ManifestBuilder::new()
        .withdraw_from_account(account1, XRD, 10)
        .take_from_worktop(XRD, 10, "xrd")
        .try_deposit_or_abort(account2, None, "xrd")
        .withdraw_from_account(account1, XRD, 10)
        .take_from_worktop(XRD, 10, "xrd1")
        .try_deposit_or_abort(account2, None, "xrd1")
        .build();

    // Act
    let (manifest_summary, execution_summary) = test_runner.summarize(manifest);

    // Assert
    assert_eq_three!(
        manifest_summary.presented_proofs.len(),
        execution_summary.presented_proofs.len(),
        0
    );
    assert_eq_three!(
        manifest_summary.encountered_entities,
        execution_summary.encountered_entities,
        indexset![
            GlobalAddress::from(account1),
            GlobalAddress::from(XRD),
            GlobalAddress::from(account2),
        ]
    );
    assert_eq_three!(
        manifest_summary.accounts_requiring_auth,
        execution_summary.accounts_requiring_auth,
        indexset![account1]
    );
    assert_eq_three!(
        manifest_summary.identities_requiring_auth,
        execution_summary.identities_requiring_auth,
        indexset![]
    );
    assert_eq_three!(
        manifest_summary.reserved_instructions,
        execution_summary.reserved_instructions,
        indexset![]
    );
    assert_eq_three!(
        manifest_summary.classification.len(),
        execution_summary.detailed_classification.len(),
        2
    );

    assert_eq!(
        manifest_summary.accounts_withdrawn_from,
        indexset![account1]
    );
    assert_eq!(
        manifest_summary.accounts_deposited_into,
        indexset![account2]
    );
    assert_eq!(
        manifest_summary.classification,
        indexset![ManifestClass::Transfer, ManifestClass::General]
    );

    assert_eq!(
        execution_summary.account_withdraws,
        indexmap! {
            account1 => vec![
                ResourceIndicator::Fungible(XRD, FungibleResourceIndicator::Guaranteed(dec!(10))),
                ResourceIndicator::Fungible(XRD, FungibleResourceIndicator::Guaranteed(dec!(10))),
            ]
        }
    );
    assert_eq!(
        execution_summary.account_deposits,
        indexmap! {
            account2 => vec![
                ResourceIndicator::Fungible(XRD, FungibleResourceIndicator::Guaranteed(dec!(10))),
                ResourceIndicator::Fungible(XRD, FungibleResourceIndicator::Guaranteed(dec!(10))),
            ]
        }
    );
    assert_eq!(execution_summary.new_entities, NewEntities::default());
    assert!(matches!(
        execution_summary.detailed_classification[0],
        DetailedManifestClass::Transfer {
            is_one_to_one: false
        }
    ));
    assert!(matches!(
        execution_summary.detailed_classification[1],
        DetailedManifestClass::General
    ));
}

#[test]
fn transfers_with_try_deposit_or_refund_are_invalid() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().without_trace().build();

    let (_, _, account1) = test_runner.new_account(false);
    let (_, _, account2) = test_runner.new_account(false);

    let manifest = ManifestBuilder::new()
        .withdraw_from_account(account1, XRD, 10)
        .take_from_worktop(XRD, 10, "xrd")
        .try_deposit_or_refund(account2, None, "xrd")
        .build();

    // Act
    let (manifest_summary, execution_summary) = test_runner.summarize(manifest);

    // Assert
    assert_eq_three!(
        manifest_summary.presented_proofs.len(),
        execution_summary.presented_proofs.len(),
        0
    );
    assert_eq_three!(
        manifest_summary.encountered_entities,
        execution_summary.encountered_entities,
        indexset![
            GlobalAddress::from(account1),
            GlobalAddress::from(XRD),
            GlobalAddress::from(account2),
        ]
    );
    assert_eq_three!(
        manifest_summary.accounts_requiring_auth,
        execution_summary.accounts_requiring_auth,
        indexset![account1]
    );
    assert_eq_three!(
        manifest_summary.identities_requiring_auth,
        execution_summary.identities_requiring_auth,
        indexset![]
    );
    assert_eq_three!(
        manifest_summary.reserved_instructions,
        execution_summary.reserved_instructions,
        indexset![]
    );
    assert_eq_three!(
        manifest_summary.classification.len(),
        execution_summary.detailed_classification.len(),
        0
    );

    assert_eq!(
        manifest_summary.accounts_withdrawn_from,
        indexset![account1]
    );
    assert_eq!(
        manifest_summary.accounts_deposited_into,
        indexset![account2]
    );
    assert_eq!(manifest_summary.classification, indexset![]);

    assert_eq!(
        execution_summary.account_withdraws,
        indexmap! {
            account1 => vec![
                ResourceIndicator::Fungible(XRD, FungibleResourceIndicator::Guaranteed(dec!(10)))
            ]
        }
    );
    assert_eq!(
        execution_summary.account_deposits,
        indexmap! {
            account2 => vec![
                ResourceIndicator::Fungible(XRD, FungibleResourceIndicator::Guaranteed(dec!(10)))
            ]
        }
    );
    assert_eq!(execution_summary.new_entities, NewEntities::default());
}

#[test]
fn lock_fee_is_recognized_as_a_reserved_instruction1() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().without_trace().build();

    let (_, _, account1) = test_runner.new_account(false);
    let (_, _, account2) = test_runner.new_account(false);

    let manifest = ManifestBuilder::new()
        .lock_fee(account1, dec!("10"))
        .withdraw_from_account(account1, XRD, 10)
        .take_from_worktop(XRD, 10, "xrd")
        .try_deposit_or_refund(account2, None, "xrd")
        .build();

    // Act
    let (manifest_summary, execution_summary) = test_runner.summarize(manifest);

    // Assert
    assert_eq_three!(
        manifest_summary.presented_proofs.len(),
        execution_summary.presented_proofs.len(),
        0
    );
    assert_eq_three!(
        manifest_summary.encountered_entities,
        execution_summary.encountered_entities,
        indexset![
            GlobalAddress::from(account1),
            GlobalAddress::from(XRD),
            GlobalAddress::from(account2),
        ]
    );
    assert_eq_three!(
        manifest_summary.accounts_requiring_auth,
        execution_summary.accounts_requiring_auth,
        indexset![account1]
    );
    assert_eq_three!(
        manifest_summary.identities_requiring_auth,
        execution_summary.identities_requiring_auth,
        indexset![]
    );
    assert_eq_three!(
        manifest_summary.reserved_instructions,
        execution_summary.reserved_instructions,
        indexset![ReservedInstruction::AccountLockFee]
    );
    assert_eq_three!(
        manifest_summary.classification.len(),
        execution_summary.detailed_classification.len(),
        0
    );

    assert_eq!(
        manifest_summary.accounts_withdrawn_from,
        indexset![account1]
    );
    assert_eq!(
        manifest_summary.accounts_deposited_into,
        indexset![account2]
    );
    assert_eq!(manifest_summary.classification, indexset![]);

    assert_eq!(
        execution_summary.account_withdraws,
        indexmap! {
            account1 => vec![
                ResourceIndicator::Fungible(XRD, FungibleResourceIndicator::Guaranteed(dec!(10)))
            ]
        }
    );
    assert_eq!(
        execution_summary.account_deposits,
        indexmap! {
            account2 => vec![
                ResourceIndicator::Fungible(XRD, FungibleResourceIndicator::Guaranteed(dec!(10)))
            ]
        }
    );
    assert_eq!(execution_summary.new_entities, NewEntities::default());
}

#[test]
fn lock_fee_is_recognized_as_a_reserved_instruction2() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().without_trace().build();

    let (_, _, account1) = test_runner.new_account(false);
    let (_, _, account2) = test_runner.new_account(false);

    let manifest = ManifestBuilder::new()
        .lock_fee_and_withdraw(account1, 10, XRD, 10)
        .take_from_worktop(XRD, 10, "xrd")
        .try_deposit_or_refund(account2, None, "xrd")
        .build();

    // Act
    let (manifest_summary, execution_summary) = test_runner.summarize(manifest);

    // Assert
    assert_eq_three!(
        manifest_summary.presented_proofs.len(),
        execution_summary.presented_proofs.len(),
        0
    );
    assert_eq_three!(
        manifest_summary.encountered_entities,
        execution_summary.encountered_entities,
        indexset![
            GlobalAddress::from(account1),
            GlobalAddress::from(XRD),
            GlobalAddress::from(account2),
        ]
    );
    assert_eq_three!(
        manifest_summary.accounts_requiring_auth,
        execution_summary.accounts_requiring_auth,
        indexset![account1]
    );
    assert_eq_three!(
        manifest_summary.identities_requiring_auth,
        execution_summary.identities_requiring_auth,
        indexset![]
    );
    assert_eq_three!(
        manifest_summary.reserved_instructions,
        execution_summary.reserved_instructions,
        indexset![ReservedInstruction::AccountLockFee]
    );
    assert_eq_three!(
        manifest_summary.classification.len(),
        execution_summary.detailed_classification.len(),
        0
    );

    assert_eq!(
        manifest_summary.accounts_withdrawn_from,
        indexset![account1]
    );
    assert_eq!(
        manifest_summary.accounts_deposited_into,
        indexset![account2]
    );
    assert_eq!(manifest_summary.classification, indexset![]);

    assert_eq!(
        execution_summary.account_withdraws,
        indexmap! {
            account1 => vec![
                ResourceIndicator::Fungible(XRD, FungibleResourceIndicator::Guaranteed(dec!(10)))
            ]
        }
    );
    assert_eq!(
        execution_summary.account_deposits,
        indexmap! {
            account2 => vec![
                ResourceIndicator::Fungible(XRD, FungibleResourceIndicator::Guaranteed(dec!(10)))
            ]
        }
    );
    assert_eq!(execution_summary.new_entities, NewEntities::default());
}

#[test]
fn faucet_fee_xrd_is_recognized_as_a_general_transaction() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().without_trace().build();

    let (_, _, account1) = test_runner.new_account(false);

    let manifest = ManifestBuilder::new()
        .get_free_xrd_from_faucet()
        .take_from_worktop(XRD, 10_000, "xrd")
        .try_deposit_or_abort(account1, None, "xrd")
        .build();

    // Act
    let (manifest_summary, execution_summary) = test_runner.summarize(manifest);

    // Assert
    assert_eq_three!(
        manifest_summary.presented_proofs.len(),
        execution_summary.presented_proofs.len(),
        0
    );
    assert_eq_three!(
        manifest_summary.encountered_entities,
        execution_summary.encountered_entities,
        indexset![
            GlobalAddress::from(FAUCET),
            GlobalAddress::from(XRD),
            GlobalAddress::from(account1),
        ]
    );
    assert_eq_three!(
        manifest_summary.accounts_requiring_auth,
        execution_summary.accounts_requiring_auth,
        indexset![]
    );
    assert_eq_three!(
        manifest_summary.identities_requiring_auth,
        execution_summary.identities_requiring_auth,
        indexset![]
    );
    assert_eq_three!(
        manifest_summary.reserved_instructions,
        execution_summary.reserved_instructions,
        indexset![]
    );
    assert_eq_three!(
        manifest_summary.classification.len(),
        execution_summary.detailed_classification.len(),
        1
    );

    assert_eq!(manifest_summary.accounts_withdrawn_from, indexset![]);
    assert_eq!(
        manifest_summary.accounts_deposited_into,
        indexset![account1]
    );
    assert_eq!(
        manifest_summary.classification,
        indexset![ManifestClass::General]
    );

    assert!(execution_summary.account_withdraws.is_empty());
    assert_eq!(
        execution_summary.account_deposits,
        indexmap! {
            account1 => vec![
                ResourceIndicator::Fungible(XRD, FungibleResourceIndicator::Guaranteed(dec!(10_000))),
            ]
        }
    );
    assert_eq!(execution_summary.new_entities, NewEntities::default());
    assert!(matches!(
        execution_summary.detailed_classification[0],
        DetailedManifestClass::General
    ));
}

#[test]
fn account_deposit_is_recognized_as_a_method_that_requires_auth() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().without_trace().build();

    let (_, _, account1) = test_runner.new_account(false);

    let manifest = ManifestBuilder::new()
        .get_free_xrd_from_faucet()
        .take_from_worktop(XRD, 10_000, "xrd")
        .deposit(account1, "xrd")
        .build();

    // Act
    let (manifest_summary, execution_summary) = test_runner.summarize(manifest);

    // Assert
    assert_eq_three!(
        manifest_summary.presented_proofs.len(),
        execution_summary.presented_proofs.len(),
        0
    );
    assert_eq_three!(
        manifest_summary.encountered_entities,
        execution_summary.encountered_entities,
        indexset![
            GlobalAddress::from(FAUCET),
            GlobalAddress::from(XRD),
            GlobalAddress::from(account1),
        ]
    );
    assert_eq_three!(
        manifest_summary.accounts_requiring_auth,
        execution_summary.accounts_requiring_auth,
        indexset![account1]
    );
    assert_eq_three!(
        manifest_summary.identities_requiring_auth,
        execution_summary.identities_requiring_auth,
        indexset![]
    );
    assert_eq_three!(
        manifest_summary.reserved_instructions,
        execution_summary.reserved_instructions,
        indexset![]
    );
    assert_eq_three!(
        manifest_summary.classification.len(),
        execution_summary.detailed_classification.len(),
        1
    );

    assert_eq!(manifest_summary.accounts_withdrawn_from, indexset![]);
    assert_eq!(
        manifest_summary.accounts_deposited_into,
        indexset![account1]
    );
    assert_eq!(
        manifest_summary.classification,
        indexset![ManifestClass::General]
    );

    assert!(execution_summary.account_withdraws.is_empty());
    assert_eq!(
        execution_summary.account_deposits,
        indexmap! {
            account1 => vec![
                ResourceIndicator::Fungible(XRD, FungibleResourceIndicator::Guaranteed(dec!(10_000))),
            ]
        }
    );
    assert_eq!(execution_summary.new_entities, NewEntities::default());
    assert!(matches!(
        execution_summary.detailed_classification[0],
        DetailedManifestClass::General
    ));
}

#[test]
fn account_deposit_batch_is_recognized_as_a_method_that_requires_auth() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().without_trace().build();

    let (_, _, account1) = test_runner.new_account(false);

    let manifest = ManifestBuilder::new()
        .get_free_xrd_from_faucet()
        .deposit_batch(account1)
        .build();

    // Act
    let (manifest_summary, execution_summary) = test_runner.summarize(manifest);

    // Assert
    assert_eq_three!(
        manifest_summary.presented_proofs.len(),
        execution_summary.presented_proofs.len(),
        0
    );
    assert_eq_three!(
        manifest_summary.encountered_entities,
        execution_summary.encountered_entities,
        indexset![GlobalAddress::from(FAUCET), GlobalAddress::from(account1),]
    );
    assert_eq_three!(
        manifest_summary.accounts_requiring_auth,
        execution_summary.accounts_requiring_auth,
        indexset![account1]
    );
    assert_eq_three!(
        manifest_summary.identities_requiring_auth,
        execution_summary.identities_requiring_auth,
        indexset![]
    );
    assert_eq_three!(
        manifest_summary.reserved_instructions,
        execution_summary.reserved_instructions,
        indexset![]
    );
    assert_eq_three!(
        manifest_summary.classification.len(),
        execution_summary.detailed_classification.len(),
        1
    );

    assert_eq!(manifest_summary.accounts_withdrawn_from, indexset![]);
    assert_eq!(
        manifest_summary.accounts_deposited_into,
        indexset![account1]
    );
    assert_eq!(
        manifest_summary.classification,
        indexset![ManifestClass::General]
    );

    assert!(execution_summary.account_withdraws.is_empty());
    assert_eq!(
        execution_summary.account_deposits,
        indexmap! {
            account1 => vec![
                ResourceIndicator::Fungible(XRD, FungibleResourceIndicator::Predicted(Predicted { value: dec!(10_000), instruction_index: 1 })),
            ]
        }
    );
    assert_eq!(execution_summary.new_entities, NewEntities::default());
    assert!(matches!(
        execution_summary.detailed_classification[0],
        DetailedManifestClass::General
    ));
}

#[test]
fn instruction_index_of_predicted_bucket_is_its_creation_instruction() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().without_trace().build();

    let (_, _, account1) = test_runner.new_account(false);

    let manifest = ManifestBuilder::new()
        .get_free_xrd_from_faucet()
        .take_all_from_worktop(XRD, "xrd")
        .deposit(account1, "xrd")
        .build();

    // Act
    let (manifest_summary, execution_summary) = test_runner.summarize(manifest);

    // Assert
    assert_eq_three!(
        manifest_summary.presented_proofs.len(),
        execution_summary.presented_proofs.len(),
        0
    );
    assert_eq_three!(
        manifest_summary.encountered_entities,
        execution_summary.encountered_entities,
        indexset![
            GlobalAddress::from(FAUCET),
            GlobalAddress::from(XRD),
            GlobalAddress::from(account1),
        ]
    );
    assert_eq_three!(
        manifest_summary.accounts_requiring_auth,
        execution_summary.accounts_requiring_auth,
        indexset![account1]
    );
    assert_eq_three!(
        manifest_summary.identities_requiring_auth,
        execution_summary.identities_requiring_auth,
        indexset![]
    );
    assert_eq_three!(
        manifest_summary.reserved_instructions,
        execution_summary.reserved_instructions,
        indexset![]
    );
    assert_eq_three!(
        manifest_summary.classification.len(),
        execution_summary.detailed_classification.len(),
        1
    );

    assert_eq!(manifest_summary.accounts_withdrawn_from, indexset![]);
    assert_eq!(
        manifest_summary.accounts_deposited_into,
        indexset![account1]
    );
    assert_eq!(
        manifest_summary.classification,
        indexset![ManifestClass::General]
    );

    assert!(execution_summary.account_withdraws.is_empty());
    assert_eq!(
        execution_summary.account_deposits,
        indexmap! {
            account1 => vec![
                ResourceIndicator::Fungible(XRD, FungibleResourceIndicator::Predicted(Predicted { value: dec!(10_000), instruction_index: 1 })),
            ]
        }
    );
    assert_eq!(execution_summary.new_entities, NewEntities::default());
    assert!(matches!(
        execution_summary.detailed_classification[0],
        DetailedManifestClass::General
    ));
}

#[test]
fn pool_contribution_transactions_are_recognized() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().without_trace().build();

    let (_, _, account) = test_runner.new_account(false);

    let (
        [resource1, resource2, resource3, resource4],
        [one_pool, two_pool, multi_pool],
        [one_pool_unit, two_pool_unit, multi_pool_unit],
    ) = create_pools(&mut test_runner, account);

    let manifest = ManifestBuilder::new()
        /* One Resource Pool */
        .withdraw_from_account(account, resource1, 100)
        .take_from_worktop(resource1, 100, "one_pool_bucket1")
        .with_bucket("one_pool_bucket1", |builder, bucket| {
            builder.call_method(
                one_pool,
                ONE_RESOURCE_POOL_CONTRIBUTE_IDENT,
                OneResourcePoolContributeManifestInput { bucket },
            )
        })
        .try_deposit_entire_worktop_or_abort(account, None)
        /* Two Resource Pool */
        .withdraw_from_account(account, resource1, 100)
        .withdraw_from_account(account, resource2, 100)
        .take_from_worktop(resource1, 100, "two_pool_bucket1")
        .take_from_worktop(resource2, 100, "two_pool_bucket2")
        .with_bucket("two_pool_bucket1", |builder, two_pool_bucket1| {
            builder.with_bucket(
                "two_pool_bucket2",
                |builder, two_pool_bucket2| {
                    builder.call_method(
                        two_pool,
                        TWO_RESOURCE_POOL_CONTRIBUTE_IDENT,
                        TwoResourcePoolContributeManifestInput {
                            buckets: (two_pool_bucket1, two_pool_bucket2),
                        },
                    )
                },
            )
        })
        .try_deposit_entire_worktop_or_abort(account, None)
        /* Multi Resource Pool */
        .withdraw_from_account(account, resource1, 100)
        .withdraw_from_account(account, resource2, 100)
        .withdraw_from_account(account, resource3, 100)
        .withdraw_from_account(account, resource4, 100)
        .call_method(
            multi_pool,
            MULTI_RESOURCE_POOL_CONTRIBUTE_IDENT,
            manifest_args!(ManifestExpression::EntireWorktop),
        )
        .try_deposit_entire_worktop_or_abort(account, None)
        .build();

    // Act
    let (manifest_summary, execution_summary) = test_runner.summarize(manifest);
    assert_eq_three!(
        manifest_summary.presented_proofs.len(),
        execution_summary.presented_proofs.len(),
        0
    );
    assert_eq_three!(
        manifest_summary.encountered_entities,
        execution_summary.encountered_entities,
        indexset![
            GlobalAddress::from(account),
            GlobalAddress::from(resource1),
            GlobalAddress::from(one_pool),
            GlobalAddress::from(resource2),
            GlobalAddress::from(two_pool),
            GlobalAddress::from(resource3),
            GlobalAddress::from(resource4),
            GlobalAddress::from(multi_pool),
        ]
    );
    assert_eq_three!(
        manifest_summary.accounts_requiring_auth,
        execution_summary.accounts_requiring_auth,
        indexset![account]
    );
    assert_eq_three!(
        manifest_summary.identities_requiring_auth,
        execution_summary.identities_requiring_auth,
        indexset![]
    );
    assert_eq_three!(
        manifest_summary.reserved_instructions,
        execution_summary.reserved_instructions,
        indexset![]
    );
    assert_eq_three!(
        manifest_summary.classification.len(),
        execution_summary.detailed_classification.len(),
        1
    );

    assert_eq!(manifest_summary.accounts_withdrawn_from, indexset![account]);
    assert_eq!(manifest_summary.accounts_deposited_into, indexset![account]);
    assert_eq!(
        manifest_summary.classification,
        indexset![ManifestClass::PoolContribution]
    );

    assert_eq!(
        execution_summary.account_withdraws,
        indexmap! {
            account => vec![
                /* One pool contribution */
                ResourceIndicator::Fungible(
                    resource1,
                    FungibleResourceIndicator::Guaranteed(dec!(100))
                ),
                /* Two pool contribution */
                ResourceIndicator::Fungible(
                    resource1,
                    FungibleResourceIndicator::Guaranteed(dec!(100))
                ),
                ResourceIndicator::Fungible(
                    resource2,
                    FungibleResourceIndicator::Guaranteed(dec!(100))
                ),
                /* Multi pool contribution */
                ResourceIndicator::Fungible(
                    resource1,
                    FungibleResourceIndicator::Guaranteed(dec!(100))
                ),
                ResourceIndicator::Fungible(
                    resource2,
                    FungibleResourceIndicator::Guaranteed(dec!(100))
                ),
                ResourceIndicator::Fungible(
                    resource3,
                    FungibleResourceIndicator::Guaranteed(dec!(100))
                ),
                ResourceIndicator::Fungible(
                    resource4,
                    FungibleResourceIndicator::Guaranteed(dec!(100))
                ),
            ]
        }
    );
    assert_eq!(
        execution_summary.account_deposits,
        indexmap! {
            account => vec![
                /* One Pool Units */
                ResourceIndicator::Fungible(
                    one_pool_unit,
                    FungibleResourceIndicator::Predicted(
                        Predicted {
                            value: dec!(100),
                            instruction_index: 3
                        }
                    )
                ),
                /* Two Pool Units */
                ResourceIndicator::Fungible(
                    two_pool_unit,
                    FungibleResourceIndicator::Predicted(
                        Predicted {
                            value: dec!(100),
                            instruction_index: 9
                        }
                    )
                ),
                /* Multi Pool Units */
                ResourceIndicator::Fungible(
                    multi_pool_unit,
                    FungibleResourceIndicator::Predicted(
                        Predicted {
                            value: dec!(10000),
                            instruction_index: 15
                        }
                    )
                ),
            ]
        }
    );
    assert_eq!(execution_summary.new_entities, NewEntities::default());

    let [DetailedManifestClass::PoolContribution {
        pool_addresses,
        pool_contributions,
    }] = execution_summary.detailed_classification.as_slice()
    else {
        panic!("Unexpected contents")
    };
    assert_eq!(
        pool_addresses.clone(),
        indexset![one_pool, two_pool, multi_pool,]
    );
    assert_eq!(
        pool_contributions.clone(),
        vec![
            TrackedPoolContribution {
                pool_address: one_pool,
                contributed_resources: indexmap! {
                    resource1 => dec!(100)
                },
                pool_units_resource_address: one_pool_unit,
                pool_units_amount: dec!(100)
            },
            TrackedPoolContribution {
                pool_address: two_pool,
                contributed_resources: indexmap! {
                    resource1 => dec!(100),
                    resource2 => dec!(100)
                },
                pool_units_resource_address: two_pool_unit,
                pool_units_amount: dec!(100)
            },
            TrackedPoolContribution {
                pool_address: multi_pool,
                contributed_resources: indexmap! {
                    resource1 => dec!(100),
                    resource2 => dec!(100),
                    resource3 => dec!(100),
                    resource4 => dec!(100)
                },
                pool_units_resource_address: multi_pool_unit,
                pool_units_amount: dec!(10000)
            },
        ]
    );
}

#[test]
fn multi_resource_pool_contribution_with_change_is_correctly_handled() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().without_trace().build();

    let (_, _, account) = test_runner.new_account(false);

    let (
        [resource1, resource2, resource3, resource4],
        [_, _, multi_pool],
        [_, _, multi_pool_unit],
    ) = create_pools(&mut test_runner, account);

    let manifest = ManifestBuilder::new()
        .withdraw_from_account(account, resource1, 100)
        .withdraw_from_account(account, resource2, 100)
        .withdraw_from_account(account, resource3, 100)
        .withdraw_from_account(account, resource4, 100)
        .call_method(
            multi_pool,
            MULTI_RESOURCE_POOL_CONTRIBUTE_IDENT,
            manifest_args!(ManifestExpression::EntireWorktop),
        )
        .try_deposit_entire_worktop_or_abort(account, None)
        .withdraw_from_account(account, resource1, 100)
        .withdraw_from_account(account, resource2, 100)
        .withdraw_from_account(account, resource3, 100)
        .withdraw_from_account(account, resource4, 50)
        .call_method(
            multi_pool,
            MULTI_RESOURCE_POOL_CONTRIBUTE_IDENT,
            manifest_args!(ManifestExpression::EntireWorktop),
        )
        .try_deposit_entire_worktop_or_abort(account, None)
        .build();

    // Act
    let (manifest_summary, execution_summary) = test_runner.summarize(manifest);
    assert_eq_three!(
        manifest_summary.presented_proofs.len(),
        execution_summary.presented_proofs.len(),
        0
    );
    assert_eq_three!(
        manifest_summary.encountered_entities,
        execution_summary.encountered_entities,
        indexset![
            GlobalAddress::from(account),
            GlobalAddress::from(resource1),
            GlobalAddress::from(resource2),
            GlobalAddress::from(resource3),
            GlobalAddress::from(resource4),
            GlobalAddress::from(multi_pool),
        ]
    );
    assert_eq_three!(
        manifest_summary.accounts_requiring_auth,
        execution_summary.accounts_requiring_auth,
        indexset![account]
    );
    assert_eq_three!(
        manifest_summary.identities_requiring_auth,
        execution_summary.identities_requiring_auth,
        indexset![]
    );
    assert_eq_three!(
        manifest_summary.reserved_instructions,
        execution_summary.reserved_instructions,
        indexset![]
    );
    assert_eq_three!(
        manifest_summary.classification.len(),
        execution_summary.detailed_classification.len(),
        1
    );

    assert_eq!(manifest_summary.accounts_withdrawn_from, indexset![account]);
    assert_eq!(manifest_summary.accounts_deposited_into, indexset![account]);
    assert_eq!(
        manifest_summary.classification,
        indexset![ManifestClass::PoolContribution]
    );

    assert_eq!(
        execution_summary.account_withdraws,
        indexmap! {
            account => vec![
                ResourceIndicator::Fungible(
                    resource1,
                    FungibleResourceIndicator::Guaranteed(dec!(100))
                ),
                ResourceIndicator::Fungible(
                    resource2,
                    FungibleResourceIndicator::Guaranteed(dec!(100))
                ),
                ResourceIndicator::Fungible(
                    resource3,
                    FungibleResourceIndicator::Guaranteed(dec!(100))
                ),
                ResourceIndicator::Fungible(
                    resource4,
                    FungibleResourceIndicator::Guaranteed(dec!(100))
                ),
                ResourceIndicator::Fungible(
                    resource1,
                    FungibleResourceIndicator::Guaranteed(dec!(100))
                ),
                ResourceIndicator::Fungible(
                    resource2,
                    FungibleResourceIndicator::Guaranteed(dec!(100))
                ),
                ResourceIndicator::Fungible(
                    resource3,
                    FungibleResourceIndicator::Guaranteed(dec!(100))
                ),
                ResourceIndicator::Fungible(
                    resource4,
                    FungibleResourceIndicator::Guaranteed(dec!(50))
                ),
            ]
        }
    );
    assert_eq!(
        execution_summary.account_deposits,
        indexmap! {
            account => vec![
                ResourceIndicator::Fungible(
                    multi_pool_unit,
                    FungibleResourceIndicator::Predicted(
                        Predicted {
                            value: dec!(10000),
                            instruction_index: 5
                        }
                    )
                ),
                ResourceIndicator::Fungible(
                    multi_pool_unit,
                    FungibleResourceIndicator::Predicted(
                        Predicted {
                            value: dec!(5000),
                            instruction_index: 11
                        }
                    )
                ),
                ResourceIndicator::Fungible(
                    resource1,
                    FungibleResourceIndicator::Predicted(
                        Predicted {
                            value: dec!(50),
                            instruction_index: 11
                        }
                    )
                ),
                ResourceIndicator::Fungible(
                    resource2,
                    FungibleResourceIndicator::Predicted(
                        Predicted {
                            value: dec!(50),
                            instruction_index: 11
                        }
                    )
                ),
                ResourceIndicator::Fungible(
                    resource3,
                    FungibleResourceIndicator::Predicted(
                        Predicted {
                            value: dec!(50),
                            instruction_index: 11
                        }
                    )
                ),
            ]
        }
    );
    assert_eq!(execution_summary.new_entities, NewEntities::default());

    let [DetailedManifestClass::PoolContribution {
        pool_addresses,
        pool_contributions,
    }] = execution_summary.detailed_classification.as_slice()
    else {
        panic!("Unexpected contents")
    };
    assert_eq!(pool_addresses.clone(), indexset![multi_pool,]);
    assert_eq!(
        pool_contributions.clone(),
        vec![
            TrackedPoolContribution {
                pool_address: multi_pool,
                contributed_resources: indexmap! {
                    resource1 => dec!(100),
                    resource2 => dec!(100),
                    resource3 => dec!(100),
                    resource4 => dec!(100)
                },
                pool_units_resource_address: multi_pool_unit,
                pool_units_amount: dec!(10000)
            },
            TrackedPoolContribution {
                pool_address: multi_pool,
                contributed_resources: indexmap! {
                    resource1 => dec!(50),
                    resource2 => dec!(50),
                    resource3 => dec!(50),
                    resource4 => dec!(50)
                },
                pool_units_resource_address: multi_pool_unit,
                pool_units_amount: dec!(5000)
            }
        ]
    );
}

#[test]
fn pool_redemption_transactions_are_recognized() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().without_trace().build();

    let (pk, _, account) = test_runner.new_account(false);

    let (
        [resource1, resource2, resource3, resource4],
        [one_pool, two_pool, multi_pool],
        [one_pool_unit, two_pool_unit, multi_pool_unit],
    ) = create_pools(&mut test_runner, account);

    let manifest = ManifestBuilder::new()
        /* One Resource Pool */
        .withdraw_from_account(account, resource1, 100)
        .take_from_worktop(resource1, 100, "one_pool_bucket1")
        .with_bucket("one_pool_bucket1", |builder, bucket| {
            builder.call_method(
                one_pool,
                ONE_RESOURCE_POOL_CONTRIBUTE_IDENT,
                OneResourcePoolContributeManifestInput { bucket },
            )
        })
        .try_deposit_entire_worktop_or_abort(account, None)
        /* Two Resource Pool */
        .withdraw_from_account(account, resource1, 100)
        .withdraw_from_account(account, resource2, 100)
        .take_from_worktop(resource1, 100, "two_pool_bucket1")
        .take_from_worktop(resource2, 100, "two_pool_bucket2")
        .with_bucket("two_pool_bucket1", |builder, two_pool_bucket1| {
            builder.with_bucket(
                "two_pool_bucket2",
                |builder, two_pool_bucket2| {
                    builder.call_method(
                        two_pool,
                        TWO_RESOURCE_POOL_CONTRIBUTE_IDENT,
                        TwoResourcePoolContributeManifestInput {
                            buckets: (two_pool_bucket1, two_pool_bucket2),
                        },
                    )
                },
            )
        })
        .try_deposit_entire_worktop_or_abort(account, None)
        /* Multi Resource Pool */
        .withdraw_from_account(account, resource1, 100)
        .withdraw_from_account(account, resource2, 100)
        .withdraw_from_account(account, resource3, 100)
        .withdraw_from_account(account, resource4, 100)
        .call_method(
            multi_pool,
            MULTI_RESOURCE_POOL_CONTRIBUTE_IDENT,
            manifest_args!(ManifestExpression::EntireWorktop),
        )
        .try_deposit_entire_worktop_or_abort(account, None)
        .build();
    test_runner
        .execute_manifest_ignoring_fee(
            manifest,
            vec![NonFungibleGlobalId::from_public_key(&pk)],
        )
        .expect_commit_success();

    let manifest = ManifestBuilder::new()
        /* One Pool */
        .withdraw_from_account(account, one_pool_unit, 100)
        .take_all_from_worktop(one_pool_unit, "one_pool_unit")
        .with_bucket("one_pool_unit", |builder, bucket| {
            builder.call_method(
                one_pool,
                ONE_RESOURCE_POOL_REDEEM_IDENT,
                OneResourcePoolRedeemManifestInput { bucket },
            )
        })
        /* Two Pool */
        .withdraw_from_account(account, two_pool_unit, 100)
        .take_all_from_worktop(two_pool_unit, "two_pool_unit")
        .with_bucket("two_pool_unit", |builder, bucket| {
            builder.call_method(
                two_pool,
                TWO_RESOURCE_POOL_REDEEM_IDENT,
                TwoResourcePoolRedeemManifestInput { bucket },
            )
        })
        /* Multi Pool */
        .withdraw_from_account(account, multi_pool_unit, 10000)
        .take_all_from_worktop(multi_pool_unit, "multi_pool_unit")
        .with_bucket("multi_pool_unit", |builder, bucket| {
            builder.call_method(
                multi_pool,
                MULTI_RESOURCE_POOL_REDEEM_IDENT,
                MultiResourcePoolRedeemManifestInput { bucket },
            )
        })
        /* Deposit everything! */
        .try_deposit_entire_worktop_or_abort(account, None)
        .build();

    // Act
    let (manifest_summary, execution_summary) = test_runner.summarize(manifest);
    assert_eq_three!(
        manifest_summary.presented_proofs.len(),
        execution_summary.presented_proofs.len(),
        0
    );
    assert_eq_three!(
        manifest_summary.encountered_entities,
        execution_summary.encountered_entities,
        indexset![
            GlobalAddress::from(account),
            GlobalAddress::from(one_pool_unit),
            GlobalAddress::from(one_pool),
            GlobalAddress::from(two_pool_unit),
            GlobalAddress::from(two_pool),
            GlobalAddress::from(multi_pool_unit),
            GlobalAddress::from(multi_pool),
        ]
    );
    assert_eq_three!(
        manifest_summary.accounts_requiring_auth,
        execution_summary.accounts_requiring_auth,
        indexset![account]
    );
    assert_eq_three!(
        manifest_summary.identities_requiring_auth,
        execution_summary.identities_requiring_auth,
        indexset![]
    );
    assert_eq_three!(
        manifest_summary.reserved_instructions,
        execution_summary.reserved_instructions,
        indexset![]
    );
    assert_eq_three!(
        manifest_summary.classification.len(),
        execution_summary.detailed_classification.len(),
        1
    );

    assert_eq!(manifest_summary.accounts_withdrawn_from, indexset![account]);
    assert_eq!(manifest_summary.accounts_deposited_into, indexset![account]);
    assert_eq!(
        manifest_summary.classification,
        indexset![ManifestClass::PoolRedemption]
    );

    assert_eq!(
        execution_summary.account_withdraws,
        indexmap! {
            account => vec![
                ResourceIndicator::Fungible(
                    one_pool_unit,
                    FungibleResourceIndicator::Guaranteed(dec!(100))
                ),
                ResourceIndicator::Fungible(
                    two_pool_unit,
                    FungibleResourceIndicator::Guaranteed(dec!(100))
                ),
                ResourceIndicator::Fungible(
                    multi_pool_unit,
                    FungibleResourceIndicator::Guaranteed(dec!(10000))
                ),
            ]
        }
    );
    assert_eq!(
        execution_summary.account_deposits,
        indexmap! {
            account => vec![
                /* One pool contribution */
                ResourceIndicator::Fungible(
                    resource1,
                    FungibleResourceIndicator::Predicted(
                        Predicted {
                            value: dec!(300),
                            instruction_index: 9
                        }
                    )
                ),
                ResourceIndicator::Fungible(
                    resource2,
                    FungibleResourceIndicator::Predicted(
                        Predicted {
                            value: dec!(200),
                            instruction_index: 9
                        }
                    )
                ),
                ResourceIndicator::Fungible(
                    resource3,
                    FungibleResourceIndicator::Predicted(
                        Predicted {
                            value: dec!(100),
                            instruction_index: 9
                        }
                    )
                ),
                ResourceIndicator::Fungible(
                    resource4,
                    FungibleResourceIndicator::Predicted(
                        Predicted {
                            value: dec!(100),
                            instruction_index: 9
                        }
                    )
                ),
            ]
        }
    );
    assert_eq!(execution_summary.new_entities, NewEntities::default());

    let [DetailedManifestClass::PoolRedemption {
        pool_addresses,
        pool_redemptions,
    }] = execution_summary.detailed_classification.as_slice()
    else {
        panic!("Unexpected contents")
    };
    assert_eq!(
        pool_addresses.clone(),
        indexset![one_pool, two_pool, multi_pool,]
    );
    assert_eq!(
        pool_redemptions.clone(),
        vec![
            TrackedPoolRedemption {
                pool_address: one_pool,
                redeemed_resources: indexmap! {
                    resource1 => dec!(100)
                },
                pool_units_resource_address: one_pool_unit,
                pool_units_amount: dec!(100)
            },
            TrackedPoolRedemption {
                pool_address: two_pool,
                redeemed_resources: indexmap! {
                    resource1 => dec!(100),
                    resource2 => dec!(100)
                },
                pool_units_resource_address: two_pool_unit,
                pool_units_amount: dec!(100)
            },
            TrackedPoolRedemption {
                pool_address: multi_pool,
                redeemed_resources: indexmap! {
                    resource1 => dec!(100),
                    resource2 => dec!(100),
                    resource3 => dec!(100),
                    resource4 => dec!(100)
                },
                pool_units_resource_address: multi_pool_unit,
                pool_units_amount: dec!(10000)
            },
        ]
    );
}

fn create_pools(
    test_runner: &mut DefaultTestRunner,
    account: ComponentAddress,
) -> (
    [ResourceAddress; 4],
    [ComponentAddress; 3],
    [ResourceAddress; 3],
) {
    let [resource1, resource2, resource3, resource4] = [0u8; 4].map(|_| {
        test_runner.create_fungible_resource(dec!(100_000_000_000), 18, account)
    });

    let receipt = test_runner.execute_manifest(
        ManifestBuilder::new()
            .lock_fee_from_faucet()
            .call_function(
                POOL_PACKAGE,
                ONE_RESOURCE_POOL_BLUEPRINT_IDENT,
                ONE_RESOURCE_POOL_INSTANTIATE_IDENT,
                OneResourcePoolInstantiateManifestInput {
                    address_reservation: None,
                    owner_role: OwnerRole::None,
                    pool_manager_rule: rule!(allow_all),
                    resource_address: resource1,
                },
            )
            .call_function(
                POOL_PACKAGE,
                TWO_RESOURCE_POOL_BLUEPRINT_IDENT,
                TWO_RESOURCE_POOL_INSTANTIATE_IDENT,
                TwoResourcePoolInstantiateManifestInput {
                    address_reservation: None,
                    owner_role: OwnerRole::None,
                    pool_manager_rule: rule!(allow_all),
                    resource_addresses: (resource1, resource2),
                },
            )
            .call_function(
                POOL_PACKAGE,
                MULTI_RESOURCE_POOL_BLUEPRINT_IDENT,
                MULTI_RESOURCE_POOL_INSTANTIATE_IDENT,
                MultiResourcePoolInstantiateManifestInput {
                    address_reservation: None,
                    owner_role: OwnerRole::None,
                    pool_manager_rule: rule!(allow_all),
                    resource_addresses: indexset![
                        resource1, resource2, resource3, resource4
                    ],
                },
            )
            .build(),
        vec![],
    );
    let commit_result = receipt.expect_commit_success();

    let one_resource_pool = commit_result
        .new_component_addresses()
        .get_index(0)
        .unwrap();
    let two_resource_pool = commit_result
        .new_component_addresses()
        .get_index(1)
        .unwrap();
    let multi_resource_pool = commit_result
        .new_component_addresses()
        .get_index(2)
        .unwrap();

    let one_resource_pool_pool_unit =
        commit_result.new_resource_addresses().get_index(0).unwrap();
    let two_resource_pool_pool_unit =
        commit_result.new_resource_addresses().get_index(1).unwrap();
    let multi_resource_pool_pool_unit =
        commit_result.new_resource_addresses().get_index(2).unwrap();

    (
        [resource1, resource2, resource3, resource4],
        [*one_resource_pool, *two_resource_pool, *multi_resource_pool],
        [
            *one_resource_pool_pool_unit,
            *two_resource_pool_pool_unit,
            *multi_resource_pool_pool_unit,
        ],
    )
}

#[extend::ext]
impl<E, D> TestRunner<E, D>
where
    E: NativeVmExtension,
    D: TestDatabase,
{
    fn preview(
        &mut self,
        manifest: TransactionManifestV1,
    ) -> TransactionReceiptV1 {
        self.preview_manifest(
            manifest,
            vec![],
            0,
            PreviewFlags {
                use_free_credit: true,
                assume_all_signature_proofs: true,
                skip_epoch_check: true,
            },
        )
    }

    fn summarize(
        &mut self,
        manifest: TransactionManifestV1,
    ) -> (ManifestSummary, ExecutionSummary) {
        let receipt = TestRunnerEDExt::preview(self, manifest.clone());

        let manifest_summary =
            radix_engine_toolkit::transaction_types::summary(&manifest);
        let execution_summary =
            radix_engine_toolkit::transaction_types::execution_summary(
                &manifest, &receipt,
            )
            .unwrap();

        (manifest_summary, execution_summary)
    }
}
