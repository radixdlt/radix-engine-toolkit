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
use radix_engine_toolkit::transaction_types::*;
use scrypto_unit::*;
use transaction::prelude::*;

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
    assert!(manifest_summary.presented_proofs.is_empty());
    assert_eq!(
        manifest_summary.accounts_withdrawn_from,
        indexset![account1]
    );
    assert_eq!(
        manifest_summary.accounts_deposited_into,
        indexset![account2]
    );
    assert_eq!(
        manifest_summary.encountered_entities,
        indexset![
            GlobalAddress::from(account1),
            GlobalAddress::from(XRD),
            GlobalAddress::from(account2),
        ]
    );
    assert_eq!(
        manifest_summary.accounts_requiring_auth,
        indexset![account1]
    );
    assert_eq!(manifest_summary.identities_requiring_auth, indexset![]);
    assert_eq!(manifest_summary.reserved_instructions, indexset![]);
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
    assert!(execution_summary.presented_proofs.is_empty());
    assert_eq!(execution_summary.new_entities, NewEntities::default());
    assert_eq!(
        execution_summary.encountered_entities,
        indexset![
            GlobalAddress::from(account1),
            GlobalAddress::from(XRD),
            GlobalAddress::from(account2),
        ]
    );
    assert_eq!(
        execution_summary.accounts_requiring_auth,
        indexset![account1]
    );
    assert_eq!(execution_summary.identities_requiring_auth, indexset![]);
    assert_eq!(execution_summary.reserved_instructions, indexset![]);
    assert_eq!(execution_summary.detailed_classification.len(), 2);
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
    assert!(manifest_summary.presented_proofs.is_empty());
    assert_eq!(
        manifest_summary.accounts_withdrawn_from,
        indexset![account1]
    );
    assert_eq!(
        manifest_summary.accounts_deposited_into,
        indexset![account2]
    );
    assert_eq!(
        manifest_summary.encountered_entities,
        indexset![
            GlobalAddress::from(account1),
            GlobalAddress::from(XRD),
            GlobalAddress::from(account2),
        ]
    );
    assert_eq!(
        manifest_summary.accounts_requiring_auth,
        indexset![account1]
    );
    assert_eq!(manifest_summary.identities_requiring_auth, indexset![]);
    assert_eq!(manifest_summary.reserved_instructions, indexset![]);
    assert_eq!(
        manifest_summary.classification,
        indexset![ManifestClass::Transfer, ManifestClass::General]
    );

    assert_eq!(
        execution_summary.account_withdraws,
        indexmap! {
            account1 => vec![
                ResourceIndicator::Fungible(XRD, FungibleResourceIndicator::Guaranteed(dec!(10))),
                ResourceIndicator::Fungible(XRD, FungibleResourceIndicator::Guaranteed(dec!(10)))
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
    assert!(execution_summary.presented_proofs.is_empty());
    assert_eq!(execution_summary.new_entities, NewEntities::default());
    assert_eq!(
        execution_summary.encountered_entities,
        indexset![
            GlobalAddress::from(account1),
            GlobalAddress::from(XRD),
            GlobalAddress::from(account2),
        ]
    );
    assert_eq!(
        execution_summary.accounts_requiring_auth,
        indexset![account1]
    );
    assert_eq!(execution_summary.identities_requiring_auth, indexset![]);
    assert_eq!(execution_summary.reserved_instructions, indexset![]);
    assert_eq!(execution_summary.detailed_classification.len(), 2);
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
    assert!(manifest_summary.presented_proofs.is_empty());
    assert_eq!(
        manifest_summary.accounts_withdrawn_from,
        indexset![account1]
    );
    assert_eq!(
        manifest_summary.accounts_deposited_into,
        indexset![account2]
    );
    assert_eq!(
        manifest_summary.encountered_entities,
        indexset![
            GlobalAddress::from(account1),
            GlobalAddress::from(XRD),
            GlobalAddress::from(account2),
        ]
    );
    assert_eq!(
        manifest_summary.accounts_requiring_auth,
        indexset![account1]
    );
    assert_eq!(manifest_summary.identities_requiring_auth, indexset![]);
    assert_eq!(manifest_summary.reserved_instructions, indexset![]);
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
    assert!(execution_summary.presented_proofs.is_empty());
    assert_eq!(execution_summary.new_entities, NewEntities::default());
    assert_eq!(
        execution_summary.encountered_entities,
        indexset![
            GlobalAddress::from(account1),
            GlobalAddress::from(XRD),
            GlobalAddress::from(account2),
        ]
    );
    assert_eq!(
        execution_summary.accounts_requiring_auth,
        indexset![account1]
    );
    assert_eq!(execution_summary.identities_requiring_auth, indexset![]);
    assert_eq!(execution_summary.reserved_instructions, indexset![]);
    assert_eq!(execution_summary.detailed_classification.len(), 0);
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
    assert!(manifest_summary.presented_proofs.is_empty());
    assert_eq!(
        manifest_summary.accounts_withdrawn_from,
        indexset![account1]
    );
    assert_eq!(
        manifest_summary.accounts_deposited_into,
        indexset![account2]
    );
    assert_eq!(
        manifest_summary.encountered_entities,
        indexset![
            GlobalAddress::from(account1),
            GlobalAddress::from(XRD),
            GlobalAddress::from(account2),
        ]
    );
    assert_eq!(
        manifest_summary.accounts_requiring_auth,
        indexset![account1]
    );
    assert_eq!(manifest_summary.identities_requiring_auth, indexset![]);
    assert_eq!(
        manifest_summary.reserved_instructions,
        indexset![ReservedInstruction::AccountLockFee]
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
    assert!(execution_summary.presented_proofs.is_empty());
    assert_eq!(execution_summary.new_entities, NewEntities::default());
    assert_eq!(
        execution_summary.encountered_entities,
        indexset![
            GlobalAddress::from(account1),
            GlobalAddress::from(XRD),
            GlobalAddress::from(account2),
        ]
    );
    assert_eq!(
        execution_summary.accounts_requiring_auth,
        indexset![account1]
    );
    assert_eq!(execution_summary.identities_requiring_auth, indexset![]);
    assert_eq!(
        execution_summary.reserved_instructions,
        indexset![ReservedInstruction::AccountLockFee]
    );
    assert_eq!(execution_summary.detailed_classification.len(), 0);
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
    assert!(manifest_summary.presented_proofs.is_empty());
    assert_eq!(
        manifest_summary.accounts_withdrawn_from,
        indexset![account1]
    );
    assert_eq!(
        manifest_summary.accounts_deposited_into,
        indexset![account2]
    );
    assert_eq!(
        manifest_summary.encountered_entities,
        indexset![
            GlobalAddress::from(account1),
            GlobalAddress::from(XRD),
            GlobalAddress::from(account2),
        ]
    );
    assert_eq!(
        manifest_summary.accounts_requiring_auth,
        indexset![account1]
    );
    assert_eq!(manifest_summary.identities_requiring_auth, indexset![]);
    assert_eq!(
        manifest_summary.reserved_instructions,
        indexset![ReservedInstruction::AccountLockFee]
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
    assert!(execution_summary.presented_proofs.is_empty());
    assert_eq!(execution_summary.new_entities, NewEntities::default());
    assert_eq!(
        execution_summary.encountered_entities,
        indexset![
            GlobalAddress::from(account1),
            GlobalAddress::from(XRD),
            GlobalAddress::from(account2),
        ]
    );
    assert_eq!(
        execution_summary.accounts_requiring_auth,
        indexset![account1]
    );
    assert_eq!(execution_summary.identities_requiring_auth, indexset![]);
    assert_eq!(
        execution_summary.reserved_instructions,
        indexset![ReservedInstruction::AccountLockFee]
    );
    assert_eq!(execution_summary.detailed_classification.len(), 0);
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
    assert!(manifest_summary.presented_proofs.is_empty());
    assert_eq!(manifest_summary.accounts_withdrawn_from, indexset![]);
    assert_eq!(
        manifest_summary.accounts_deposited_into,
        indexset![account1]
    );
    assert_eq!(
        manifest_summary.encountered_entities,
        indexset![
            GlobalAddress::from(FAUCET),
            GlobalAddress::from(XRD),
            GlobalAddress::from(account1),
        ]
    );
    assert_eq!(manifest_summary.accounts_requiring_auth, indexset![]);
    assert_eq!(manifest_summary.identities_requiring_auth, indexset![]);
    assert_eq!(manifest_summary.reserved_instructions, indexset![]);
    assert_eq!(
        manifest_summary.classification,
        indexset![ManifestClass::General]
    );

    assert!(execution_summary.account_withdraws.is_empty());
    assert_eq!(
        execution_summary.account_deposits,
        indexmap! {
            account1 => vec![
                ResourceIndicator::Fungible(XRD, FungibleResourceIndicator::Guaranteed(dec!(10_000)))
            ]
        }
    );
    assert!(execution_summary.presented_proofs.is_empty());
    assert_eq!(execution_summary.new_entities, NewEntities::default());
    assert_eq!(
        execution_summary.encountered_entities,
        indexset![
            GlobalAddress::from(FAUCET),
            GlobalAddress::from(XRD),
            GlobalAddress::from(account1),
        ]
    );
    assert_eq!(execution_summary.accounts_requiring_auth, indexset![]);
    assert_eq!(execution_summary.identities_requiring_auth, indexset![]);
    assert_eq!(execution_summary.reserved_instructions, indexset![]);
    assert_eq!(execution_summary.detailed_classification.len(), 1);
    assert!(matches!(
        execution_summary.detailed_classification[0],
        DetailedManifestClass::General
    ));
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
