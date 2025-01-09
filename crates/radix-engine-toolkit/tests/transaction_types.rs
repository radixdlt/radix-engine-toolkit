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

use radix_engine::blueprints::pool::v1::constants::*;
use radix_engine::system::system_modules::execution_trace::ResourceSpecifier;
use radix_engine_interface::blueprints::account::*;
use radix_engine_interface::blueprints::consensus_manager::*;
use radix_engine_interface::blueprints::pool::*;
use radix_engine_toolkit::transaction_types::*;
use radix_transactions::prelude::*;
use scrypto_test::prelude::*;

mod test_runner_extension;
use test_runner_extension::LedgerSimulatorEDExt;

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
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let manifest = ManifestBuilder::new().build();

    // Act
    let (static_analysis, dynamic_analysis) = ledger.summarize(manifest);

    // Assert
    assert_eq!(static_analysis.classification.len(), 0);
    assert_eq!(dynamic_analysis.detailed_classification.len(), 0);
}

#[test]
fn lock_fee_still_keeps_the_transfer_classification_but_adds_a_reserved_instruction()
 {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();

    let (_, _, account1) = ledger.new_account(false);
    let (_, _, account2) = ledger.new_account(false);

    let manifest = ManifestBuilder::new()
        .lock_fee(account1, 10)
        .withdraw_from_account(account1, XRD, 10)
        .take_from_worktop(XRD, 10, "xrd")
        .try_deposit_or_abort(account2, None, "xrd")
        .build();

    // Act
    let (static_analysis, dynamic_analysis) = ledger.summarize(manifest);

    // Assert
    assert_eq_three!(
        static_analysis.presented_proofs.len(),
        dynamic_analysis.presented_proofs.len(),
        0
    );
    assert_eq_three!(
        static_analysis.encountered_entities,
        dynamic_analysis.encountered_entities,
        indexset![
            GlobalAddress::from(account1),
            GlobalAddress::from(XRD),
            GlobalAddress::from(account2),
        ]
    );
    assert_eq_three!(
        static_analysis.accounts_requiring_auth,
        dynamic_analysis.accounts_requiring_auth,
        indexset![account1]
    );
    assert_eq_three!(
        static_analysis.identities_requiring_auth,
        dynamic_analysis.identities_requiring_auth,
        indexset![]
    );
    assert_eq_three!(
        static_analysis.reserved_instructions,
        dynamic_analysis.reserved_instructions,
        indexset![ReservedInstruction::AccountLockFee]
    );
    assert_eq_three!(
        static_analysis.classification.len(),
        dynamic_analysis.detailed_classification.len(),
        2
    );

    assert_eq!(static_analysis.accounts_withdrawn_from, indexset![account1]);
    assert_eq!(static_analysis.accounts_deposited_into, indexset![account2]);
    assert_eq!(
        static_analysis.classification,
        indexset![ManifestClass::Transfer, ManifestClass::General]
    );

    assert_eq!(
        dynamic_analysis.account_withdraws,
        indexmap! {
            account1 => vec![
                ResourceIndicator::Fungible(XRD, FungibleResourceIndicator::Guaranteed(dec!(10)))
            ]
        }
    );
    assert_eq!(
        dynamic_analysis.account_deposits,
        indexmap! {
            account2 => vec![
                ResourceIndicator::Fungible(XRD, FungibleResourceIndicator::Guaranteed(dec!(10)))
            ]
        }
    );
    assert_eq!(dynamic_analysis.new_entities, NewEntities::default());
    assert!(matches!(
        &dynamic_analysis.detailed_classification[0],
        DetailedManifestClass::Transfer {
            is_one_to_one: true
        }
    ));
    assert!(matches!(
        dynamic_analysis.detailed_classification[1],
        DetailedManifestClass::General
    ));

    assert_eq!(dynamic_analysis.newly_created_non_fungibles.len(), 0);
}

#[test]
fn simple_transfer_satisfies_the_transfer_and_general_transaction_types() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();

    let (_, _, account1) = ledger.new_account(false);
    let (_, _, account2) = ledger.new_account(false);

    let manifest = ManifestBuilder::new()
        .withdraw_from_account(account1, XRD, 10)
        .take_from_worktop(XRD, 10, "xrd")
        .try_deposit_or_abort(account2, None, "xrd")
        .build();

    // Act
    let (static_analysis, dynamic_analysis) = ledger.summarize(manifest);

    // Assert
    assert_eq_three!(
        static_analysis.presented_proofs.len(),
        dynamic_analysis.presented_proofs.len(),
        0
    );
    assert_eq_three!(
        static_analysis.encountered_entities,
        dynamic_analysis.encountered_entities,
        indexset![
            GlobalAddress::from(account1),
            GlobalAddress::from(XRD),
            GlobalAddress::from(account2),
        ]
    );
    assert_eq_three!(
        static_analysis.accounts_requiring_auth,
        dynamic_analysis.accounts_requiring_auth,
        indexset![account1]
    );
    assert_eq_three!(
        static_analysis.identities_requiring_auth,
        dynamic_analysis.identities_requiring_auth,
        indexset![]
    );
    assert_eq_three!(
        static_analysis.reserved_instructions,
        dynamic_analysis.reserved_instructions,
        indexset![]
    );
    assert_eq_three!(
        static_analysis.classification.len(),
        dynamic_analysis.detailed_classification.len(),
        2
    );

    assert_eq!(static_analysis.accounts_withdrawn_from, indexset![account1]);
    assert_eq!(static_analysis.accounts_deposited_into, indexset![account2]);
    assert_eq!(
        static_analysis.classification,
        indexset![ManifestClass::Transfer, ManifestClass::General]
    );

    assert_eq!(
        dynamic_analysis.account_withdraws,
        indexmap! {
            account1 => vec![
                ResourceIndicator::Fungible(XRD, FungibleResourceIndicator::Guaranteed(dec!(10)))
            ]
        }
    );
    assert_eq!(
        dynamic_analysis.account_deposits,
        indexmap! {
            account2 => vec![
                ResourceIndicator::Fungible(XRD, FungibleResourceIndicator::Guaranteed(dec!(10)))
            ]
        }
    );
    assert_eq!(dynamic_analysis.new_entities, NewEntities::default());
    assert!(matches!(
        dynamic_analysis.detailed_classification[0],
        DetailedManifestClass::Transfer {
            is_one_to_one: true
        }
    ));
    assert!(matches!(
        dynamic_analysis.detailed_classification[1],
        DetailedManifestClass::General
    ));

    assert_eq!(dynamic_analysis.newly_created_non_fungibles.len(), 0);
}

#[test]
fn non_simple_transfer_satisfies_the_transfer_and_general_transaction_types() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();

    let (_, _, account1) = ledger.new_account(false);
    let (_, _, account2) = ledger.new_account(false);

    let manifest = ManifestBuilder::new()
        .withdraw_from_account(account1, XRD, 10)
        .take_from_worktop(XRD, 10, "xrd")
        .try_deposit_or_abort(account2, None, "xrd")
        .withdraw_from_account(account1, XRD, 10)
        .take_from_worktop(XRD, 10, "xrd1")
        .try_deposit_or_abort(account2, None, "xrd1")
        .build();

    // Act
    let (static_analysis, dynamic_analysis) = ledger.summarize(manifest);

    // Assert
    assert_eq_three!(
        static_analysis.presented_proofs.len(),
        dynamic_analysis.presented_proofs.len(),
        0
    );
    assert_eq_three!(
        static_analysis.encountered_entities,
        dynamic_analysis.encountered_entities,
        indexset![
            GlobalAddress::from(account1),
            GlobalAddress::from(XRD),
            GlobalAddress::from(account2),
        ]
    );
    assert_eq_three!(
        static_analysis.accounts_requiring_auth,
        dynamic_analysis.accounts_requiring_auth,
        indexset![account1]
    );
    assert_eq_three!(
        static_analysis.identities_requiring_auth,
        dynamic_analysis.identities_requiring_auth,
        indexset![]
    );
    assert_eq_three!(
        static_analysis.reserved_instructions,
        dynamic_analysis.reserved_instructions,
        indexset![]
    );
    assert_eq_three!(
        static_analysis.classification.len(),
        dynamic_analysis.detailed_classification.len(),
        2
    );

    assert_eq!(static_analysis.accounts_withdrawn_from, indexset![account1]);
    assert_eq!(static_analysis.accounts_deposited_into, indexset![account2]);
    assert_eq!(
        static_analysis.classification,
        indexset![ManifestClass::Transfer, ManifestClass::General]
    );

    assert_eq!(
        dynamic_analysis.account_withdraws,
        indexmap! {
            account1 => vec![
                ResourceIndicator::Fungible(XRD, FungibleResourceIndicator::Guaranteed(dec!(10))),
                ResourceIndicator::Fungible(XRD, FungibleResourceIndicator::Guaranteed(dec!(10))),
            ]
        }
    );
    assert_eq!(
        dynamic_analysis.account_deposits,
        indexmap! {
            account2 => vec![
                ResourceIndicator::Fungible(XRD, FungibleResourceIndicator::Guaranteed(dec!(10))),
                ResourceIndicator::Fungible(XRD, FungibleResourceIndicator::Guaranteed(dec!(10))),
            ]
        }
    );
    assert_eq!(dynamic_analysis.new_entities, NewEntities::default());
    assert!(matches!(
        dynamic_analysis.detailed_classification[0],
        DetailedManifestClass::Transfer {
            is_one_to_one: false
        }
    ));
    assert!(matches!(
        dynamic_analysis.detailed_classification[1],
        DetailedManifestClass::General
    ));

    assert_eq!(dynamic_analysis.newly_created_non_fungibles.len(), 0);
}

#[test]
fn transfers_with_try_deposit_or_refund_are_invalid() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();

    let (_, _, account1) = ledger.new_account(false);
    let (_, _, account2) = ledger.new_account(false);

    let manifest = ManifestBuilder::new()
        .withdraw_from_account(account1, XRD, 10)
        .take_from_worktop(XRD, 10, "xrd")
        .try_deposit_or_refund(account2, None, "xrd")
        .build();

    // Act
    let (static_analysis, dynamic_analysis) = ledger.summarize(manifest);

    // Assert
    assert_eq_three!(
        static_analysis.presented_proofs.len(),
        dynamic_analysis.presented_proofs.len(),
        0
    );
    assert_eq_three!(
        static_analysis.encountered_entities,
        dynamic_analysis.encountered_entities,
        indexset![
            GlobalAddress::from(account1),
            GlobalAddress::from(XRD),
            GlobalAddress::from(account2),
        ]
    );
    assert_eq_three!(
        static_analysis.accounts_requiring_auth,
        dynamic_analysis.accounts_requiring_auth,
        indexset![account1]
    );
    assert_eq_three!(
        static_analysis.identities_requiring_auth,
        dynamic_analysis.identities_requiring_auth,
        indexset![]
    );
    assert_eq_three!(
        static_analysis.reserved_instructions,
        dynamic_analysis.reserved_instructions,
        indexset![]
    );
    assert_eq_three!(
        static_analysis.classification.len(),
        dynamic_analysis.detailed_classification.len(),
        0
    );

    assert_eq!(static_analysis.accounts_withdrawn_from, indexset![account1]);
    assert_eq!(static_analysis.accounts_deposited_into, indexset![account2]);
    assert_eq!(static_analysis.classification, indexset![]);

    assert_eq!(
        dynamic_analysis.account_withdraws,
        indexmap! {
            account1 => vec![
                ResourceIndicator::Fungible(XRD, FungibleResourceIndicator::Guaranteed(dec!(10)))
            ]
        }
    );
    assert_eq!(
        dynamic_analysis.account_deposits,
        indexmap! {
            account2 => vec![
                ResourceIndicator::Fungible(XRD, FungibleResourceIndicator::Guaranteed(dec!(10)))
            ]
        }
    );
    assert_eq!(dynamic_analysis.new_entities, NewEntities::default());

    assert_eq!(dynamic_analysis.newly_created_non_fungibles.len(), 0);
}

#[test]
fn lock_fee_is_recognized_as_a_reserved_instruction1() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();

    let (_, _, account1) = ledger.new_account(false);
    let (_, _, account2) = ledger.new_account(false);

    let manifest = ManifestBuilder::new()
        .lock_fee(account1, dec!("10"))
        .withdraw_from_account(account1, XRD, 10)
        .take_from_worktop(XRD, 10, "xrd")
        .try_deposit_or_refund(account2, None, "xrd")
        .build();

    // Act
    let (static_analysis, dynamic_analysis) = ledger.summarize(manifest);

    // Assert
    assert_eq_three!(
        static_analysis.presented_proofs.len(),
        dynamic_analysis.presented_proofs.len(),
        0
    );
    assert_eq_three!(
        static_analysis.encountered_entities,
        dynamic_analysis.encountered_entities,
        indexset![
            GlobalAddress::from(account1),
            GlobalAddress::from(XRD),
            GlobalAddress::from(account2),
        ]
    );
    assert_eq_three!(
        static_analysis.accounts_requiring_auth,
        dynamic_analysis.accounts_requiring_auth,
        indexset![account1]
    );
    assert_eq_three!(
        static_analysis.identities_requiring_auth,
        dynamic_analysis.identities_requiring_auth,
        indexset![]
    );
    assert_eq_three!(
        static_analysis.reserved_instructions,
        dynamic_analysis.reserved_instructions,
        indexset![ReservedInstruction::AccountLockFee]
    );
    assert_eq_three!(
        static_analysis.classification.len(),
        dynamic_analysis.detailed_classification.len(),
        0
    );

    assert_eq!(static_analysis.accounts_withdrawn_from, indexset![account1]);
    assert_eq!(static_analysis.accounts_deposited_into, indexset![account2]);
    assert_eq!(static_analysis.classification, indexset![]);

    assert_eq!(
        dynamic_analysis.account_withdraws,
        indexmap! {
            account1 => vec![
                ResourceIndicator::Fungible(XRD, FungibleResourceIndicator::Guaranteed(dec!(10)))
            ]
        }
    );
    assert_eq!(
        dynamic_analysis.account_deposits,
        indexmap! {
            account2 => vec![
                ResourceIndicator::Fungible(XRD, FungibleResourceIndicator::Guaranteed(dec!(10)))
            ]
        }
    );
    assert_eq!(dynamic_analysis.new_entities, NewEntities::default());

    assert_eq!(dynamic_analysis.newly_created_non_fungibles.len(), 0);
}

#[test]
fn lock_fee_is_recognized_as_a_reserved_instruction2() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();

    let (_, _, account1) = ledger.new_account(false);
    let (_, _, account2) = ledger.new_account(false);

    let manifest = ManifestBuilder::new()
        .lock_fee_and_withdraw(account1, 10, XRD, 10)
        .take_from_worktop(XRD, 10, "xrd")
        .try_deposit_or_refund(account2, None, "xrd")
        .build();

    // Act
    let (static_analysis, dynamic_analysis) = ledger.summarize(manifest);

    // Assert
    assert_eq_three!(
        static_analysis.presented_proofs.len(),
        dynamic_analysis.presented_proofs.len(),
        0
    );
    assert_eq_three!(
        static_analysis.encountered_entities,
        dynamic_analysis.encountered_entities,
        indexset![
            GlobalAddress::from(account1),
            GlobalAddress::from(XRD),
            GlobalAddress::from(account2),
        ]
    );
    assert_eq_three!(
        static_analysis.accounts_requiring_auth,
        dynamic_analysis.accounts_requiring_auth,
        indexset![account1]
    );
    assert_eq_three!(
        static_analysis.identities_requiring_auth,
        dynamic_analysis.identities_requiring_auth,
        indexset![]
    );
    assert_eq_three!(
        static_analysis.reserved_instructions,
        dynamic_analysis.reserved_instructions,
        indexset![ReservedInstruction::AccountLockFee]
    );
    assert_eq_three!(
        static_analysis.classification.len(),
        dynamic_analysis.detailed_classification.len(),
        0
    );

    assert_eq!(static_analysis.accounts_withdrawn_from, indexset![account1]);
    assert_eq!(static_analysis.accounts_deposited_into, indexset![account2]);
    assert_eq!(static_analysis.classification, indexset![]);

    assert_eq!(
        dynamic_analysis.account_withdraws,
        indexmap! {
            account1 => vec![
                ResourceIndicator::Fungible(XRD, FungibleResourceIndicator::Guaranteed(dec!(10)))
            ]
        }
    );
    assert_eq!(
        dynamic_analysis.account_deposits,
        indexmap! {
            account2 => vec![
                ResourceIndicator::Fungible(XRD, FungibleResourceIndicator::Guaranteed(dec!(10)))
            ]
        }
    );
    assert_eq!(dynamic_analysis.new_entities, NewEntities::default());

    assert_eq!(dynamic_analysis.newly_created_non_fungibles.len(), 0);
}

#[test]
fn faucet_fee_xrd_is_recognized_as_a_general_transaction() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();

    let (_, _, account1) = ledger.new_account(false);

    let manifest = ManifestBuilder::new()
        .get_free_xrd_from_faucet()
        .take_from_worktop(XRD, 10_000, "xrd")
        .try_deposit_or_abort(account1, None, "xrd")
        .build();

    // Act
    let (static_analysis, dynamic_analysis) = ledger.summarize(manifest);

    // Assert
    assert_eq_three!(
        static_analysis.presented_proofs.len(),
        dynamic_analysis.presented_proofs.len(),
        0
    );
    assert_eq_three!(
        static_analysis.encountered_entities,
        dynamic_analysis.encountered_entities,
        indexset![
            GlobalAddress::from(FAUCET),
            GlobalAddress::from(XRD),
            GlobalAddress::from(account1),
        ]
    );
    assert_eq_three!(
        static_analysis.accounts_requiring_auth,
        dynamic_analysis.accounts_requiring_auth,
        indexset![]
    );
    assert_eq_three!(
        static_analysis.identities_requiring_auth,
        dynamic_analysis.identities_requiring_auth,
        indexset![]
    );
    assert_eq_three!(
        static_analysis.reserved_instructions,
        dynamic_analysis.reserved_instructions,
        indexset![]
    );
    assert_eq_three!(
        static_analysis.classification.len(),
        dynamic_analysis.detailed_classification.len(),
        1
    );

    assert_eq!(static_analysis.accounts_withdrawn_from, indexset![]);
    assert_eq!(static_analysis.accounts_deposited_into, indexset![account1]);
    assert_eq!(
        static_analysis.classification,
        indexset![ManifestClass::General]
    );

    assert!(dynamic_analysis.account_withdraws.is_empty());
    assert_eq!(
        dynamic_analysis.account_deposits,
        indexmap! {
            account1 => vec![
                ResourceIndicator::Fungible(XRD, FungibleResourceIndicator::Guaranteed(dec!(10_000))),
            ]
        }
    );
    assert_eq!(dynamic_analysis.new_entities, NewEntities::default());
    assert!(matches!(
        dynamic_analysis.detailed_classification[0],
        DetailedManifestClass::General
    ));

    assert_eq!(dynamic_analysis.newly_created_non_fungibles.len(), 0);
}

#[test]
fn account_deposit_is_recognized_as_a_method_that_requires_auth() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();

    let (_, _, account1) = ledger.new_account(false);

    let manifest = ManifestBuilder::new()
        .get_free_xrd_from_faucet()
        .take_from_worktop(XRD, 10_000, "xrd")
        .deposit(account1, "xrd")
        .build();

    // Act
    let (static_analysis, dynamic_analysis) = ledger.summarize(manifest);

    // Assert
    assert_eq_three!(
        static_analysis.presented_proofs.len(),
        dynamic_analysis.presented_proofs.len(),
        0
    );
    assert_eq_three!(
        static_analysis.encountered_entities,
        dynamic_analysis.encountered_entities,
        indexset![
            GlobalAddress::from(FAUCET),
            GlobalAddress::from(XRD),
            GlobalAddress::from(account1),
        ]
    );
    assert_eq_three!(
        static_analysis.accounts_requiring_auth,
        dynamic_analysis.accounts_requiring_auth,
        indexset![account1]
    );
    assert_eq_three!(
        static_analysis.identities_requiring_auth,
        dynamic_analysis.identities_requiring_auth,
        indexset![]
    );
    assert_eq_three!(
        static_analysis.reserved_instructions,
        dynamic_analysis.reserved_instructions,
        indexset![]
    );
    assert_eq_three!(
        static_analysis.classification.len(),
        dynamic_analysis.detailed_classification.len(),
        1
    );

    assert_eq!(static_analysis.accounts_withdrawn_from, indexset![]);
    assert_eq!(static_analysis.accounts_deposited_into, indexset![account1]);
    assert_eq!(
        static_analysis.classification,
        indexset![ManifestClass::General]
    );

    assert!(dynamic_analysis.account_withdraws.is_empty());
    assert_eq!(
        dynamic_analysis.account_deposits,
        indexmap! {
            account1 => vec![
                ResourceIndicator::Fungible(XRD, FungibleResourceIndicator::Guaranteed(dec!(10_000))),
            ]
        }
    );
    assert_eq!(dynamic_analysis.new_entities, NewEntities::default());
    assert!(matches!(
        dynamic_analysis.detailed_classification[0],
        DetailedManifestClass::General
    ));

    assert_eq!(dynamic_analysis.newly_created_non_fungibles.len(), 0);
}

#[test]
fn account_deposit_batch_is_recognized_as_a_method_that_requires_auth() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();

    let (_, _, account1) = ledger.new_account(false);

    let manifest = ManifestBuilder::new()
        .get_free_xrd_from_faucet()
        .deposit_entire_worktop(account1)
        .build();

    // Act
    let (static_analysis, dynamic_analysis) = ledger.summarize(manifest);

    // Assert
    assert_eq_three!(
        static_analysis.presented_proofs.len(),
        dynamic_analysis.presented_proofs.len(),
        0
    );
    assert_eq_three!(
        static_analysis.encountered_entities,
        dynamic_analysis.encountered_entities,
        indexset![GlobalAddress::from(FAUCET), GlobalAddress::from(account1),]
    );
    assert_eq_three!(
        static_analysis.accounts_requiring_auth,
        dynamic_analysis.accounts_requiring_auth,
        indexset![account1]
    );
    assert_eq_three!(
        static_analysis.identities_requiring_auth,
        dynamic_analysis.identities_requiring_auth,
        indexset![]
    );
    assert_eq_three!(
        static_analysis.reserved_instructions,
        dynamic_analysis.reserved_instructions,
        indexset![]
    );
    assert_eq_three!(
        static_analysis.classification.len(),
        dynamic_analysis.detailed_classification.len(),
        1
    );

    assert_eq!(static_analysis.accounts_withdrawn_from, indexset![]);
    assert_eq!(static_analysis.accounts_deposited_into, indexset![account1]);
    assert_eq!(
        static_analysis.classification,
        indexset![ManifestClass::General]
    );

    assert!(dynamic_analysis.account_withdraws.is_empty());
    assert_eq!(
        dynamic_analysis.account_deposits,
        indexmap! {
            account1 => vec![
                ResourceIndicator::Fungible(XRD, FungibleResourceIndicator::Predicted(Predicted { value: dec!(10_000), instruction_index: 1 })),
            ]
        }
    );
    assert_eq!(dynamic_analysis.new_entities, NewEntities::default());
    assert!(matches!(
        dynamic_analysis.detailed_classification[0],
        DetailedManifestClass::General
    ));

    assert_eq!(dynamic_analysis.newly_created_non_fungibles.len(), 0);
}

#[test]
fn instruction_index_of_predicted_bucket_is_its_creation_instruction() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();

    let (_, _, account1) = ledger.new_account(false);

    let manifest = ManifestBuilder::new()
        .get_free_xrd_from_faucet()
        .take_all_from_worktop(XRD, "xrd")
        .deposit(account1, "xrd")
        .build();

    // Act
    let (static_analysis, dynamic_analysis) = ledger.summarize(manifest);

    // Assert
    assert_eq_three!(
        static_analysis.presented_proofs.len(),
        dynamic_analysis.presented_proofs.len(),
        0
    );
    assert_eq_three!(
        static_analysis.encountered_entities,
        dynamic_analysis.encountered_entities,
        indexset![
            GlobalAddress::from(FAUCET),
            GlobalAddress::from(XRD),
            GlobalAddress::from(account1),
        ]
    );
    assert_eq_three!(
        static_analysis.accounts_requiring_auth,
        dynamic_analysis.accounts_requiring_auth,
        indexset![account1]
    );
    assert_eq_three!(
        static_analysis.identities_requiring_auth,
        dynamic_analysis.identities_requiring_auth,
        indexset![]
    );
    assert_eq_three!(
        static_analysis.reserved_instructions,
        dynamic_analysis.reserved_instructions,
        indexset![]
    );
    assert_eq_three!(
        static_analysis.classification.len(),
        dynamic_analysis.detailed_classification.len(),
        1
    );

    assert_eq!(static_analysis.accounts_withdrawn_from, indexset![]);
    assert_eq!(static_analysis.accounts_deposited_into, indexset![account1]);
    assert_eq!(
        static_analysis.classification,
        indexset![ManifestClass::General]
    );

    assert!(dynamic_analysis.account_withdraws.is_empty());
    assert_eq!(
        dynamic_analysis.account_deposits,
        indexmap! {
            account1 => vec![
                ResourceIndicator::Fungible(XRD, FungibleResourceIndicator::Predicted(Predicted { value: dec!(10_000), instruction_index: 1 })),
            ]
        }
    );
    assert_eq!(dynamic_analysis.new_entities, NewEntities::default());
    assert!(matches!(
        dynamic_analysis.detailed_classification[0],
        DetailedManifestClass::General
    ));

    assert_eq!(dynamic_analysis.newly_created_non_fungibles.len(), 0);
}

#[test]
fn pool_contribution_transactions_are_recognized() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();

    let (_, _, account) = ledger.new_account(false);

    let (
        [resource1, resource2, resource3, resource4],
        [one_pool, two_pool, multi_pool],
        [one_pool_unit, two_pool_unit, multi_pool_unit],
    ) = create_pools(&mut ledger, account);

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
    let (static_analysis, dynamic_analysis) = ledger.summarize(manifest);
    assert_eq_three!(
        static_analysis.presented_proofs.len(),
        dynamic_analysis.presented_proofs.len(),
        0
    );
    assert_eq_three!(
        static_analysis.encountered_entities,
        dynamic_analysis.encountered_entities,
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
        static_analysis.accounts_requiring_auth,
        dynamic_analysis.accounts_requiring_auth,
        indexset![account]
    );
    assert_eq_three!(
        static_analysis.identities_requiring_auth,
        dynamic_analysis.identities_requiring_auth,
        indexset![]
    );
    assert_eq_three!(
        static_analysis.reserved_instructions,
        dynamic_analysis.reserved_instructions,
        indexset![]
    );
    assert_eq_three!(
        static_analysis.classification.len(),
        dynamic_analysis.detailed_classification.len(),
        1
    );

    assert_eq!(static_analysis.accounts_withdrawn_from, indexset![account]);
    assert_eq!(static_analysis.accounts_deposited_into, indexset![account]);
    assert_eq!(
        static_analysis.classification,
        indexset![ManifestClass::PoolContribution]
    );

    assert_eq!(
        dynamic_analysis.account_withdraws,
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
        dynamic_analysis.account_deposits,
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
                            value: dec!(100),
                            instruction_index: 15
                        }
                    )
                ),
            ]
        }
    );
    assert_eq!(dynamic_analysis.new_entities, NewEntities::default());

    let [
        DetailedManifestClass::PoolContribution {
            pool_addresses,
            pool_contributions,
        },
    ] = dynamic_analysis.detailed_classification.as_slice()
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
                pool_units_amount: dec!(100)
            },
        ]
    );

    assert_eq!(dynamic_analysis.newly_created_non_fungibles.len(), 0);
}

#[test]
fn multi_resource_pool_contribution_with_change_is_correctly_handled() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();

    let (_, _, account) = ledger.new_account(false);

    let (
        [resource1, resource2, resource3, resource4],
        [_, _, multi_pool],
        [_, _, multi_pool_unit],
    ) = create_pools(&mut ledger, account);

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
    let (static_analysis, dynamic_analysis) = ledger.summarize(manifest);
    assert_eq_three!(
        static_analysis.presented_proofs.len(),
        dynamic_analysis.presented_proofs.len(),
        0
    );
    assert_eq_three!(
        static_analysis.encountered_entities,
        dynamic_analysis.encountered_entities,
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
        static_analysis.accounts_requiring_auth,
        dynamic_analysis.accounts_requiring_auth,
        indexset![account]
    );
    assert_eq_three!(
        static_analysis.identities_requiring_auth,
        dynamic_analysis.identities_requiring_auth,
        indexset![]
    );
    assert_eq_three!(
        static_analysis.reserved_instructions,
        dynamic_analysis.reserved_instructions,
        indexset![]
    );
    assert_eq_three!(
        static_analysis.classification.len(),
        dynamic_analysis.detailed_classification.len(),
        1
    );

    assert_eq!(static_analysis.accounts_withdrawn_from, indexset![account]);
    assert_eq!(static_analysis.accounts_deposited_into, indexset![account]);
    assert_eq!(
        static_analysis.classification,
        indexset![ManifestClass::PoolContribution]
    );

    assert_eq!(
        dynamic_analysis.account_withdraws,
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
        dynamic_analysis.account_deposits,
        indexmap! {
            account => vec![
                ResourceIndicator::Fungible(
                    multi_pool_unit,
                    FungibleResourceIndicator::Predicted(
                        Predicted {
                            value: dec!(100),
                            instruction_index: 5
                        }
                    )
                ),
                ResourceIndicator::Fungible(
                    multi_pool_unit,
                    FungibleResourceIndicator::Predicted(
                        Predicted {
                            value: dec!(50),
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
    assert_eq!(dynamic_analysis.new_entities, NewEntities::default());

    let [
        DetailedManifestClass::PoolContribution {
            pool_addresses,
            pool_contributions,
        },
    ] = dynamic_analysis.detailed_classification.as_slice()
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
                pool_units_amount: dec!(100)
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
                pool_units_amount: dec!(50)
            }
        ]
    );

    assert_eq!(dynamic_analysis.newly_created_non_fungibles.len(), 0);
}

#[test]
fn pool_redemption_transactions_are_recognized() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();

    let (pk, _, account) = ledger.new_account(false);

    let (
        [resource1, resource2, resource3, resource4],
        [one_pool, two_pool, multi_pool],
        [one_pool_unit, two_pool_unit, multi_pool_unit],
    ) = create_pools(&mut ledger, account);

    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
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
    ledger
        .execute_manifest(
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
        .withdraw_from_account(account, multi_pool_unit, 100)
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
    let (static_analysis, dynamic_analysis) = ledger.summarize(manifest);
    assert_eq_three!(
        static_analysis.presented_proofs.len(),
        dynamic_analysis.presented_proofs.len(),
        0
    );
    assert_eq_three!(
        static_analysis.encountered_entities,
        dynamic_analysis.encountered_entities,
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
        static_analysis.accounts_requiring_auth,
        dynamic_analysis.accounts_requiring_auth,
        indexset![account]
    );
    assert_eq_three!(
        static_analysis.identities_requiring_auth,
        dynamic_analysis.identities_requiring_auth,
        indexset![]
    );
    assert_eq_three!(
        static_analysis.reserved_instructions,
        dynamic_analysis.reserved_instructions,
        indexset![]
    );
    assert_eq_three!(
        static_analysis.classification.len(),
        dynamic_analysis.detailed_classification.len(),
        1
    );

    assert_eq!(static_analysis.accounts_withdrawn_from, indexset![account]);
    assert_eq!(static_analysis.accounts_deposited_into, indexset![account]);
    assert_eq!(
        static_analysis.classification,
        indexset![ManifestClass::PoolRedemption]
    );

    assert_eq!(
        dynamic_analysis.account_withdraws,
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
                    FungibleResourceIndicator::Guaranteed(dec!(100))
                ),
            ]
        }
    );
    assert_eq!(
        dynamic_analysis.account_deposits,
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
    assert_eq!(dynamic_analysis.new_entities, NewEntities::default());

    let [
        DetailedManifestClass::PoolRedemption {
            pool_addresses,
            pool_redemptions,
        },
    ] = dynamic_analysis.detailed_classification.as_slice()
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
                pool_units_amount: dec!(100)
            },
        ]
    );

    assert_eq!(dynamic_analysis.newly_created_non_fungibles.len(), 0);
}

#[test]
fn validator_stake_transactions_are_recognized() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();

    let (pk, _, account) = ledger.new_account(false);
    let (validator1, stake_unit1, _) = ledger.new_validator(pk, account);
    let (validator2, stake_unit2, _) = ledger.new_validator(pk, account);

    let manifest = ManifestBuilder::new()
        .withdraw_from_account(account, XRD, 200)
        .take_from_worktop(XRD, 100, "xrd1")
        .take_from_worktop(XRD, 100, "xrd2")
        .stake_validator(validator1, "xrd1")
        .stake_validator(validator2, "xrd2")
        .try_deposit_entire_worktop_or_abort(account, None)
        .build();

    // Act
    let (static_analysis, dynamic_analysis) = ledger.summarize(manifest);

    // Assert
    assert_eq_three!(
        static_analysis.presented_proofs.len(),
        dynamic_analysis.presented_proofs.len(),
        0
    );
    assert_eq_three!(
        static_analysis.encountered_entities,
        dynamic_analysis.encountered_entities,
        indexset![
            GlobalAddress::from(account),
            GlobalAddress::from(XRD),
            GlobalAddress::from(validator1),
            GlobalAddress::from(validator2),
        ]
    );
    assert_eq_three!(
        static_analysis.accounts_requiring_auth,
        dynamic_analysis.accounts_requiring_auth,
        indexset![account]
    );
    assert_eq_three!(
        static_analysis.identities_requiring_auth,
        dynamic_analysis.identities_requiring_auth,
        indexset![]
    );
    assert_eq_three!(
        static_analysis.reserved_instructions,
        dynamic_analysis.reserved_instructions,
        indexset![]
    );
    assert_eq_three!(
        static_analysis.classification.len(),
        dynamic_analysis.detailed_classification.len(),
        1
    );

    assert_eq!(static_analysis.accounts_withdrawn_from, indexset![account]);
    assert_eq!(static_analysis.accounts_deposited_into, indexset![account]);
    assert_eq!(
        static_analysis.classification,
        indexset![ManifestClass::ValidatorStake]
    );

    assert_eq!(
        dynamic_analysis.account_withdraws,
        indexmap! {
            account => vec![
                ResourceIndicator::Fungible(
                    XRD,
                    FungibleResourceIndicator::Guaranteed(dec!(200))
                )
            ]
        }
    );
    assert_eq!(
        dynamic_analysis.account_deposits,
        indexmap! {
            account => vec![
                ResourceIndicator::Fungible(
                    stake_unit1,
                    FungibleResourceIndicator::Predicted(
                        Predicted { value: dec!(100), instruction_index: 5 }
                    )
                ),
                ResourceIndicator::Fungible(
                    stake_unit2,
                    FungibleResourceIndicator::Predicted(
                        Predicted { value: dec!(100), instruction_index: 5 }
                    )
                ),
            ]
        }
    );
    assert_eq!(dynamic_analysis.new_entities, NewEntities::default());
    assert_eq!(
        dynamic_analysis.detailed_classification[0],
        DetailedManifestClass::ValidatorStake {
            validator_addresses: indexset![validator1, validator2],
            validator_stakes: vec![
                TrackedValidatorStake {
                    validator_address: validator1,
                    xrd_amount: dec!(100),
                    liquid_stake_unit_address: stake_unit1,
                    liquid_stake_unit_amount: dec!(100)
                },
                TrackedValidatorStake {
                    validator_address: validator2,
                    xrd_amount: dec!(100),
                    liquid_stake_unit_address: stake_unit2,
                    liquid_stake_unit_amount: dec!(100)
                }
            ]
        }
    );

    assert_eq!(dynamic_analysis.newly_created_non_fungibles.len(), 0);
}

#[test]
fn validator_unstake_transactions_are_recognized() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();

    let (pk, _, account) = ledger.new_account(false);
    let (validator1, stake_unit1, claim_nft1) =
        ledger.new_validator(pk, account);
    let (validator2, stake_unit2, claim_nft2) =
        ledger.new_validator(pk, account);

    ledger
        .execute_manifest(
            ManifestBuilder::new()
                .lock_fee_from_faucet()
                .withdraw_from_account(account, XRD, 200)
                .take_from_worktop(XRD, 100, "xrd1")
                .take_from_worktop(XRD, 100, "xrd2")
                .stake_validator(validator1, "xrd1")
                .stake_validator(validator2, "xrd2")
                .try_deposit_entire_worktop_or_abort(account, None)
                .build(),
            vec![NonFungibleGlobalId::from_public_key(&pk)],
        )
        .expect_commit_success();

    let manifest = ManifestBuilder::new()
        .withdraw_from_account(account, stake_unit1, 100)
        .withdraw_from_account(account, stake_unit2, 100)
        .take_from_worktop(stake_unit1, 100, "stake_unit1")
        .take_from_worktop(stake_unit2, 100, "stake_unit2")
        .unstake_validator(validator1, "stake_unit1")
        .unstake_validator(validator2, "stake_unit2")
        .try_deposit_entire_worktop_or_abort(account, None)
        .build();

    // Act
    let (static_analysis, dynamic_analysis) = ledger.summarize(manifest);

    // Assert
    assert_eq_three!(
        static_analysis.presented_proofs.len(),
        dynamic_analysis.presented_proofs.len(),
        0
    );
    assert_eq_three!(
        static_analysis.encountered_entities,
        dynamic_analysis.encountered_entities,
        indexset![
            GlobalAddress::from(account),
            GlobalAddress::from(stake_unit1),
            GlobalAddress::from(stake_unit2),
            GlobalAddress::from(validator1),
            GlobalAddress::from(validator2),
        ]
    );
    assert_eq_three!(
        static_analysis.accounts_requiring_auth,
        dynamic_analysis.accounts_requiring_auth,
        indexset![account]
    );
    assert_eq_three!(
        static_analysis.identities_requiring_auth,
        dynamic_analysis.identities_requiring_auth,
        indexset![]
    );
    assert_eq_three!(
        static_analysis.reserved_instructions,
        dynamic_analysis.reserved_instructions,
        indexset![]
    );
    assert_eq_three!(
        static_analysis.classification.len(),
        dynamic_analysis.detailed_classification.len(),
        1
    );

    assert_eq!(static_analysis.accounts_withdrawn_from, indexset![account]);
    assert_eq!(static_analysis.accounts_deposited_into, indexset![account]);
    assert_eq!(
        static_analysis.classification,
        indexset![ManifestClass::ValidatorUnstake]
    );

    let nf_id_local_1 = NonFungibleLocalId::from_str(
        "{9da60161aa56f3dc-b05ee091e6e496eb-926b11ceb384a4cb-16af5319924a3426}",
    )
    .unwrap();
    let nf_id_local_2 = NonFungibleLocalId::from_str(
        "{3f227ceec72040aa-843bb0cffe837873-44bc4e240172759b-482113584acda37c}",
    )
    .unwrap();

    assert_eq!(
        dynamic_analysis.account_withdraws,
        indexmap! {
            account => vec![
                ResourceIndicator::Fungible(
                    stake_unit1,
                    FungibleResourceIndicator::Guaranteed(dec!(100))
                ),
                ResourceIndicator::Fungible(
                    stake_unit2,
                    FungibleResourceIndicator::Guaranteed(dec!(100))
                ),
            ]
        }
    );
    assert_eq!(
        dynamic_analysis.account_deposits,
        indexmap! {
            account => vec![
                ResourceIndicator::NonFungible(
                    claim_nft1,
                    NonFungibleResourceIndicator::ByAll {
                        predicted_amount: Predicted {
                            value: dec!(1),
                            instruction_index: 6
                        },
                        predicted_ids: Predicted {
                            value: indexset![
                                nf_id_local_1.clone()
                            ],
                            instruction_index: 6
                        },
                    }
                ),
                ResourceIndicator::NonFungible(
                    claim_nft2,
                    NonFungibleResourceIndicator::ByAll {
                        predicted_amount: Predicted {
                            value: dec!(1),
                            instruction_index: 6
                        },
                        predicted_ids: Predicted {
                            value: indexset![
                                nf_id_local_2.clone()
                            ],
                            instruction_index: 6
                        },
                    }
                ),
            ]
        }
    );
    assert_eq!(dynamic_analysis.new_entities, NewEntities::default());
    assert_eq!(
        dynamic_analysis.detailed_classification[0],
        DetailedManifestClass::ValidatorUnstake {
            validator_addresses: indexset![validator1, validator2],
            validator_unstakes: vec![
                TrackedValidatorUnstake {
                    validator_address: validator1,
                    liquid_stake_unit_address: stake_unit1,
                    liquid_stake_unit_amount: dec!(100),
                    claim_nft_address: claim_nft1,
                    claim_nft_ids: indexset![nf_id_local_1.clone()]
                },
                TrackedValidatorUnstake {
                    validator_address: validator2,
                    liquid_stake_unit_address: stake_unit2,
                    liquid_stake_unit_amount: dec!(100),
                    claim_nft_address: claim_nft2,
                    claim_nft_ids: indexset![nf_id_local_2.clone()]
                },
            ],
            claims_non_fungible_data: indexmap! {
                NonFungibleGlobalId::new(
                    claim_nft1,
                    NonFungibleLocalId::from_str("{9da60161aa56f3dc-b05ee091e6e496eb-926b11ceb384a4cb-16af5319924a3426}").unwrap()
                ) => UnstakeData {
                    name: "Stake Claim".to_owned(),
                    claim_epoch: Epoch::of(23),
                    claim_amount: dec!(100)
                },
                NonFungibleGlobalId::new(
                    claim_nft2,
                    NonFungibleLocalId::from_str("{3f227ceec72040aa-843bb0cffe837873-44bc4e240172759b-482113584acda37c}").unwrap()
                ) => UnstakeData {
                    name: "Stake Claim".to_owned(),
                    claim_epoch: Epoch::of(23),
                    claim_amount: dec!(100)
                }
            }
        }
    );

    assert_eq!(dynamic_analysis.newly_created_non_fungibles.len(), 2);
    assert!(
        dynamic_analysis
            .newly_created_non_fungibles
            .contains(&NonFungibleGlobalId::new(claim_nft1, nf_id_local_1))
    );
    assert!(
        dynamic_analysis
            .newly_created_non_fungibles
            .contains(&NonFungibleGlobalId::new(claim_nft2, nf_id_local_2))
    );
}

#[test]
fn validator_claim_transactions_are_recognized() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();

    let (pk, _, account) = ledger.new_account(false);
    let (validator1, stake_unit1, claim_nft1) =
        ledger.new_validator(pk, account);
    let (validator2, stake_unit2, claim_nft2) =
        ledger.new_validator(pk, account);

    ledger
        .execute_manifest(
            ManifestBuilder::new()
                .lock_fee_from_faucet()
                .withdraw_from_account(account, XRD, 200)
                .take_from_worktop(XRD, 100, "xrd1")
                .take_from_worktop(XRD, 100, "xrd2")
                .stake_validator(validator1, "xrd1")
                .stake_validator(validator2, "xrd2")
                .try_deposit_entire_worktop_or_abort(account, None)
                .build(),
            vec![NonFungibleGlobalId::from_public_key(&pk)],
        )
        .expect_commit_success();
    ledger
        .execute_manifest(
            ManifestBuilder::new()
                .lock_fee_from_faucet()
                .withdraw_from_account(account, stake_unit1, 100)
                .withdraw_from_account(account, stake_unit2, 100)
                .take_from_worktop(stake_unit1, 100, "stake_unit1")
                .take_from_worktop(stake_unit2, 100, "stake_unit2")
                .unstake_validator(validator1, "stake_unit1")
                .unstake_validator(validator2, "stake_unit2")
                .try_deposit_entire_worktop_or_abort(account, None)
                .build(),
            vec![NonFungibleGlobalId::from_public_key(&pk)],
        )
        .expect_commit_success();

    ledger.advance_epoch(100);

    let manifest = ManifestBuilder::new()
        .withdraw_from_account(account, claim_nft1, 1)
        .withdraw_from_account(account, claim_nft2, 1)
        .take_from_worktop(claim_nft1, 1, "claim_nft1")
        .take_from_worktop(claim_nft2, 1, "claim_nft2")
        .with_bucket("claim_nft1", |builder, bucket| {
            builder.call_method(
                validator1,
                VALIDATOR_CLAIM_XRD_IDENT,
                ValidatorClaimXrdManifestInput { bucket },
            )
        })
        .with_bucket("claim_nft2", |builder, bucket| {
            builder.call_method(
                validator2,
                VALIDATOR_CLAIM_XRD_IDENT,
                ValidatorClaimXrdManifestInput { bucket },
            )
        })
        .try_deposit_entire_worktop_or_abort(account, None)
        .build();

    // Act
    let (static_analysis, dynamic_analysis) = ledger.summarize(manifest);

    // Assert
    assert_eq_three!(
        static_analysis.presented_proofs.len(),
        dynamic_analysis.presented_proofs.len(),
        0
    );
    assert_eq_three!(
        static_analysis.encountered_entities,
        dynamic_analysis.encountered_entities,
        indexset![
            GlobalAddress::from(account),
            GlobalAddress::from(claim_nft1),
            GlobalAddress::from(claim_nft2),
            GlobalAddress::from(validator1),
            GlobalAddress::from(validator2),
        ]
    );
    assert_eq_three!(
        static_analysis.accounts_requiring_auth,
        dynamic_analysis.accounts_requiring_auth,
        indexset![account]
    );
    assert_eq_three!(
        static_analysis.identities_requiring_auth,
        dynamic_analysis.identities_requiring_auth,
        indexset![]
    );
    assert_eq_three!(
        static_analysis.reserved_instructions,
        dynamic_analysis.reserved_instructions,
        indexset![]
    );
    assert_eq_three!(
        static_analysis.classification.len(),
        dynamic_analysis.detailed_classification.len(),
        1
    );

    assert_eq!(static_analysis.accounts_withdrawn_from, indexset![account]);
    assert_eq!(static_analysis.accounts_deposited_into, indexset![account]);
    assert_eq!(
        static_analysis.classification,
        indexset![ManifestClass::ValidatorClaim]
    );

    let nf_id_local_1 = NonFungibleLocalId::from_str(
        "{88187e7fec84a59c-9713f20d4bdd245a-90c9c04347db595f-07a038d384ce12a4}",
    )
    .unwrap();
    let nf_id_local_2 = NonFungibleLocalId::from_str(
        "{03c4420890c75309-ba0fc0af2b23105d-70c7912c61912798-ff789c9f0d3a0ac5}",
    )
    .unwrap();

    assert_eq!(
        dynamic_analysis.account_withdraws,
        indexmap! {
            account => vec![
                ResourceIndicator::NonFungible(
                    claim_nft1,
                    NonFungibleResourceIndicator::ByAmount {
                        amount: dec!(1),
                        predicted_ids: Predicted {
                            value: indexset![
                                nf_id_local_1.clone()
                            ],
                            instruction_index: 0
                        },
                    }
                ),
                ResourceIndicator::NonFungible(
                    claim_nft2,
                    NonFungibleResourceIndicator::ByAmount {
                        amount: dec!(1),
                        predicted_ids: Predicted {
                            value: indexset![
                                nf_id_local_2.clone()
                            ],
                            instruction_index: 1
                        },
                    }
                ),
            ]
        }
    );
    assert_eq!(
        dynamic_analysis.account_deposits,
        indexmap! {
            account => vec![
                ResourceIndicator::Fungible(
                    XRD,
                    FungibleResourceIndicator::Predicted(
                        Predicted {
                            value: dec!(200),
                            instruction_index: 6
                        }
                    )
                ),
            ]
        }
    );
    assert_eq!(dynamic_analysis.new_entities, NewEntities::default());
    assert_eq!(
        dynamic_analysis.detailed_classification[0],
        DetailedManifestClass::ValidatorClaim {
            validator_addresses: indexset![validator1, validator2],
            validator_claims: vec![
                TrackedValidatorClaim {
                    validator_address: validator1,
                    claim_nft_address: claim_nft1,
                    claim_nft_ids: indexset![nf_id_local_1.clone()],
                    xrd_amount: dec!(100)
                },
                TrackedValidatorClaim {
                    validator_address: validator2,
                    claim_nft_address: claim_nft2,
                    claim_nft_ids: indexset![nf_id_local_2.clone()],
                    xrd_amount: dec!(100)
                }
            ]
        }
    );

    assert_eq!(dynamic_analysis.newly_created_non_fungibles.len(), 0);
}

#[test]
fn account_deposit_settings_changes_are_recognized() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (_, _, account) = ledger.new_account(false);

    let manifest = ManifestBuilder::new()
        .call_method(
            account,
            ACCOUNT_SET_DEFAULT_DEPOSIT_RULE_IDENT,
            AccountSetDefaultDepositRuleInput {
                default: DefaultDepositRule::Accept,
            },
        )
        .call_method(
            account,
            ACCOUNT_SET_DEFAULT_DEPOSIT_RULE_IDENT,
            AccountSetDefaultDepositRuleInput {
                default: DefaultDepositRule::Reject,
            },
        )
        .call_method(
            account,
            ACCOUNT_ADD_AUTHORIZED_DEPOSITOR_IDENT,
            AccountAddAuthorizedDepositorInput {
                badge: ResourceOrNonFungible::Resource(XRD),
            },
        )
        .call_method(
            account,
            ACCOUNT_REMOVE_AUTHORIZED_DEPOSITOR_IDENT,
            AccountRemoveAuthorizedDepositorInput {
                badge: ResourceOrNonFungible::Resource(ACCOUNT_OWNER_BADGE),
            },
        )
        .call_method(
            account,
            ACCOUNT_SET_RESOURCE_PREFERENCE_IDENT,
            AccountSetResourcePreferenceInput {
                resource_address: ACCOUNT_OWNER_BADGE,
                resource_preference: ResourcePreference::Allowed,
            },
        )
        .call_method(
            account,
            ACCOUNT_SET_RESOURCE_PREFERENCE_IDENT,
            AccountSetResourcePreferenceInput {
                resource_address: ACCOUNT_OWNER_BADGE,
                resource_preference: ResourcePreference::Disallowed,
            },
        )
        .call_method(
            account,
            ACCOUNT_SET_RESOURCE_PREFERENCE_IDENT,
            AccountSetResourcePreferenceInput {
                resource_address: VALIDATOR_OWNER_BADGE,
                resource_preference: ResourcePreference::Allowed,
            },
        )
        .call_method(
            account,
            ACCOUNT_REMOVE_RESOURCE_PREFERENCE_IDENT,
            AccountRemoveResourcePreferenceInput {
                resource_address: IDENTITY_OWNER_BADGE,
            },
        )
        .build();
    let (static_analysis, dynamic_analysis) = ledger.summarize(manifest);

    assert_eq_three!(
        static_analysis.presented_proofs.len(),
        dynamic_analysis.presented_proofs.len(),
        0
    );
    assert_eq_three!(
        static_analysis.encountered_entities,
        dynamic_analysis.encountered_entities,
        indexset![
            GlobalAddress::from(account),
            GlobalAddress::from(XRD),
            GlobalAddress::from(ACCOUNT_OWNER_BADGE),
            GlobalAddress::from(VALIDATOR_OWNER_BADGE),
            GlobalAddress::from(IDENTITY_OWNER_BADGE),
        ]
    );
    assert_eq_three!(
        static_analysis.accounts_requiring_auth,
        dynamic_analysis.accounts_requiring_auth,
        indexset![account]
    );
    assert_eq_three!(
        static_analysis.identities_requiring_auth,
        dynamic_analysis.identities_requiring_auth,
        indexset![]
    );
    assert_eq_three!(
        static_analysis.reserved_instructions,
        dynamic_analysis.reserved_instructions,
        indexset![]
    );
    assert_eq_three!(
        static_analysis.classification.len(),
        dynamic_analysis.detailed_classification.len(),
        1
    );

    assert_eq!(static_analysis.accounts_withdrawn_from, indexset![]);
    assert_eq!(static_analysis.accounts_deposited_into, indexset![]);
    assert_eq!(
        static_analysis.classification,
        indexset![ManifestClass::AccountDepositSettingsUpdate]
    );
    assert!(dynamic_analysis.account_withdraws.is_empty());
    assert!(dynamic_analysis.account_deposits.is_empty());
    assert_eq!(dynamic_analysis.new_entities, NewEntities::default());
    assert_eq!(
        dynamic_analysis.detailed_classification[0],
        DetailedManifestClass::AccountDepositSettingsUpdate {
            resource_preferences_updates: indexmap! {
                account => indexmap! {
                    ACCOUNT_OWNER_BADGE => Update::Set(ResourcePreference::Disallowed),
                    VALIDATOR_OWNER_BADGE => Update::Set(ResourcePreference::Allowed),
                    IDENTITY_OWNER_BADGE => Update::Remove,
                }
            },
            deposit_mode_updates: indexmap! {
                account => DefaultDepositRule::Reject
            },
            authorized_depositors_updates: indexmap! {
                account => indexmap! {
                    ResourceOrNonFungible::Resource(XRD) => Operation::Added,
                    ResourceOrNonFungible::Resource(ACCOUNT_OWNER_BADGE) => Operation::Removed
                }
            },
        }
    );
}

#[test]
fn presented_proofs_fungible() {
    use radix_engine::system::system_modules::execution_trace::ResourceSpecifier;

    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (_, _, account_1) = ledger.new_allocated_account();
    let (_, _, account_2) = ledger.new_allocated_account();
    let address_1 = ledger.create_fungible_resource(dec!(100), 0, account_1);
    let address_2 = ledger.create_fungible_resource(dec!(100), 0, account_1);
    let address_3 = ledger.create_fungible_resource(dec!(100), 0, account_2);

    //Act
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .create_proof_from_account_of_amount(account_1, address_1, 60)
        .create_proof_from_account_of_amount(account_2, address_3, 30)
        .create_proof_from_account_of_amount(account_1, address_2, 100)
        .create_proof_from_account_of_amount(account_1, address_1, 80)
        .create_proof_from_account_of_amount(account_2, address_3, 5)
        .build();
    let (static_analysis, _) = ledger.summarize(manifest);

    // Assert
    assert_eq!(static_analysis.presented_proofs.len(), 2);
    let account_1_proofs =
        static_analysis.presented_proofs.get(&account_1).unwrap();
    assert_eq!(account_1_proofs.len(), 3);
    assert_eq!(
        account_1_proofs[0],
        ResourceSpecifier::Amount(address_1, dec!(60))
    );
    assert_eq!(
        account_1_proofs[1],
        ResourceSpecifier::Amount(address_2, dec!(100))
    );
    assert_eq!(
        account_1_proofs[2],
        ResourceSpecifier::Amount(address_1, dec!(80))
    );
    let account_2_proofs =
        static_analysis.presented_proofs.get(&account_2).unwrap();
    assert_eq!(account_2_proofs.len(), 2);
    assert_eq!(
        account_2_proofs[0],
        ResourceSpecifier::Amount(address_3, dec!(30))
    );
    assert_eq!(
        account_2_proofs[1],
        ResourceSpecifier::Amount(address_3, dec!(5))
    );
}

#[test]
fn presented_proofs_non_fungible() {
    use radix_engine::system::system_modules::execution_trace::ResourceSpecifier;

    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (_, _, account_1) = ledger.new_allocated_account();
    let (_, _, account_2) = ledger.new_allocated_account();
    let address_1 = ledger.create_non_fungible_resource(account_1);
    let address_2 = ledger.create_non_fungible_resource(account_1);
    let address_3 = ledger.create_non_fungible_resource(account_2);

    //Act
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .create_proof_from_account_of_non_fungibles(
            account_1,
            address_1,
            [NonFungibleLocalId::integer(1)],
        )
        .create_proof_from_account_of_non_fungibles(
            account_1,
            address_2,
            [NonFungibleLocalId::integer(3)],
        )
        .create_proof_from_account_of_non_fungibles(
            account_2,
            address_3,
            [
                NonFungibleLocalId::integer(2),
                NonFungibleLocalId::integer(3),
            ],
        )
        .create_proof_from_account_of_non_fungibles(
            account_1,
            address_1,
            [
                NonFungibleLocalId::integer(1),
                NonFungibleLocalId::integer(2),
            ],
        )
        .create_proof_from_account_of_non_fungibles(
            account_2,
            address_3,
            [NonFungibleLocalId::integer(2)],
        )
        .build();
    let (static_analysis, _) = ledger.summarize(manifest);

    // Assert
    assert_eq!(static_analysis.presented_proofs.len(), 2);
    let account_1_proofs =
        static_analysis.presented_proofs.get(&account_1).unwrap();
    assert_eq!(account_1_proofs.len(), 3);
    assert_eq!(
        account_1_proofs[0],
        ResourceSpecifier::Ids(
            address_1,
            [NonFungibleLocalId::integer(1)].into()
        )
    );
    assert_eq!(
        account_1_proofs[1],
        ResourceSpecifier::Ids(
            address_2,
            [NonFungibleLocalId::integer(3)].into()
        )
    );
    assert_eq!(
        account_1_proofs[2],
        ResourceSpecifier::Ids(
            address_1,
            [
                NonFungibleLocalId::integer(1),
                NonFungibleLocalId::integer(2)
            ]
            .into()
        )
    );
    let account_2_proofs =
        static_analysis.presented_proofs.get(&account_2).unwrap();
    assert_eq!(account_2_proofs.len(), 2);
    assert_eq!(
        account_2_proofs[0],
        ResourceSpecifier::Ids(
            address_3,
            [
                NonFungibleLocalId::integer(2),
                NonFungibleLocalId::integer(3)
            ]
            .into()
        )
    );
    assert_eq!(
        account_2_proofs[1],
        ResourceSpecifier::Ids(
            address_3,
            [NonFungibleLocalId::integer(2)].into()
        )
    );
}

fn create_pools(
    ledger: &mut DefaultLedgerSimulator,
    account: ComponentAddress,
) -> (
    [ResourceAddress; 4],
    [ComponentAddress; 3],
    [ResourceAddress; 3],
) {
    let [resource1, resource2, resource3, resource4] = [0u8; 4].map(|_| {
        ledger.create_fungible_resource(dec!(100_000_000_000), 18, account)
    });

    let receipt = ledger.execute_manifest(
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
                    resource_address: resource1.into(),
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
                    resource_addresses: (resource1.into(), resource2.into()),
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
                        resource1.into(),
                        resource2.into(),
                        resource3.into(),
                        resource4.into()
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

#[test]
fn account_locker_is_recognized_as_general_transaction() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (public_key, _, account) = ledger.new_account(false);

    let [owner_badge, storer_badge, recoverer_badge] =
        std::array::from_fn(|_| {
            ledger.create_fungible_resource(dec!(1), 0, account)
        });

    let account_locker = ledger
        .execute_manifest(
            ManifestBuilder::new()
                .lock_fee_from_faucet()
                .call_function(
                    LOCKER_PACKAGE,
                    ACCOUNT_LOCKER_BLUEPRINT,
                    ACCOUNT_LOCKER_INSTANTIATE_IDENT,
                    AccountLockerInstantiateManifestInput {
                        owner_role: OwnerRole::Fixed(rule!(require(
                            owner_badge
                        ))),
                        storer_role: rule!(require(storer_badge)),
                        storer_updater_role: rule!(require(storer_badge)),
                        recoverer_role: rule!(require(recoverer_badge)),
                        recoverer_updater_role: rule!(require(recoverer_badge)),
                        address_reservation: None,
                    },
                )
                .build(),
            vec![NonFungibleGlobalId::from_public_key(&public_key)],
        )
        .expect_commit_success()
        .new_component_addresses()
        .first()
        .copied()
        .unwrap();

    // Act
    let manifest = ManifestBuilder::new()
        .get_free_xrd_from_faucet()
        .create_proof_from_account_of_amount(account, storer_badge, dec!(1))
        .take_from_worktop(XRD, dec!(1), "bucket")
        .with_bucket("bucket", |builder, bucket| {
            builder.call_method(
                account_locker,
                ACCOUNT_LOCKER_STORE_IDENT,
                AccountLockerStoreManifestInput {
                    bucket,
                    claimant: account.into(),
                    try_direct_send: false,
                },
            )
        })
        .deposit_entire_worktop(account)
        .build();
    let (static_analysis, dynamic_analysis) = ledger.summarize(manifest);

    // Assert
    assert_eq_three!(
        static_analysis.classification.len(),
        dynamic_analysis.detailed_classification.len(),
        1
    );
    assert_eq!(
        static_analysis.classification,
        indexset![ManifestClass::General]
    );
    assert_eq_three!(
        static_analysis.presented_proofs.len(),
        dynamic_analysis.presented_proofs.len(),
        1
    );
    let account_proofs =
        static_analysis.presented_proofs.get(&account).unwrap();
    assert_eq!(account_proofs.len(), 1);
    assert_eq!(
        account_proofs[0],
        ResourceSpecifier::Amount(storer_badge, dec!(1))
    );

    assert_eq_three!(
        static_analysis.encountered_entities,
        dynamic_analysis.encountered_entities,
        indexset![
            GlobalAddress::from(FAUCET),
            GlobalAddress::from(account),
            GlobalAddress::from(storer_badge),
            GlobalAddress::from(XRD),
            GlobalAddress::from(account_locker),
        ]
    );

    assert_eq_three!(
        static_analysis.accounts_requiring_auth,
        dynamic_analysis.accounts_requiring_auth,
        indexset![account]
    );
    assert_eq_three!(
        static_analysis.identities_requiring_auth,
        dynamic_analysis.identities_requiring_auth,
        indexset![]
    );

    assert!(dynamic_analysis.account_withdraws.is_empty());
    assert_eq!(static_analysis.accounts_withdrawn_from, indexset![]);

    assert_eq!(static_analysis.accounts_deposited_into.len(), 1);
    assert_eq!(static_analysis.accounts_deposited_into, indexset![account]);

    assert_eq!(dynamic_analysis.new_entities, NewEntities::default());
}

#[test]
fn lock_fee_manifest_has_no_classification_except_general() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (_, _, account) = ledger.new_account(false);

    // Act
    let manifest = ManifestBuilder::new().lock_fee(account, 10).build();
    let (static_analysis, dynamic_analysis) = ledger.summarize(manifest);

    println!("{:#?}", static_analysis.classification);

    // Assert
    assert_eq_three!(
        static_analysis.classification.len(),
        dynamic_analysis.detailed_classification.len(),
        1
    );
    assert_eq!(
        static_analysis.classification,
        indexset![ManifestClass::General]
    );
}
