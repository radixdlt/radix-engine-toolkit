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

//! This module contains tests for invariants that the toolkit relies on to
//! verify them. If one of the tests in this module fail then it means that an
//! invariant that we rely on has been broken and that other things deep in the
//! toolkit logic could also break.

// TODO: Add a system of invariant names that we use. Have a test here for the
// invariant and cross-reference it with the name when the invariant is used in
// the code. This is to make it easier for us to keep track of these invariants
// and quickly search and verify them.

use radix_engine::system::system_modules::execution_trace::ResourceSpecifier;
use radix_transactions::manifest::static_resource_movements::*;
use radix_transactions::manifest::*;
use scrypto_test::prelude::*;

/// This is a test to ensure that the take and return to worktop operations will
/// produce an event in the worktop changes even if the amount is zero and that
/// this event doesn't get silently swallowed.
#[test]
fn take_and_return_to_worktop_of_zero_have_worktop_change_events() {
    // Arrange
    let mut ledger = LedgerSimulatorBuilder::new().build();
    let (_, _, account1) = ledger.new_account(false);
    let (_, _, account2) = ledger.new_account(false);

    // Act
    let manifest = ManifestBuilder::new()
        .lock_fee_and_withdraw(account1, 10, XRD, 0)
        .take_from_worktop(XRD, 0, "bucket")
        .return_to_worktop("bucket")
        .try_deposit_entire_worktop_or_abort(account2, None)
        .build();
    let receipt = ledger.preview_manifest(
        manifest,
        Default::default(),
        Default::default(),
        PreviewFlags {
            assume_all_signature_proofs: true,
            ..Default::default()
        },
    );

    // Assert
    let worktop_changes = receipt
        .expect_commit_success()
        .execution_trace
        .as_ref()
        .unwrap()
        .worktop_changes();
    assert_eq!(
        worktop_changes.get(&0),
        Some(&vec![WorktopChange::Put(ResourceSpecifier::Amount(
            XRD,
            dec!(0)
        ))])
    );
    assert_eq!(
        worktop_changes.get(&1),
        Some(&vec![WorktopChange::Take(ResourceSpecifier::Amount(
            XRD,
            dec!(0)
        ))])
    );
    assert_eq!(
        worktop_changes.get(&2),
        Some(&vec![WorktopChange::Put(ResourceSpecifier::Amount(
            XRD,
            dec!(0)
        ))])
    );
    assert_eq!(
        worktop_changes.get(&3),
        Some(&vec![WorktopChange::Take(ResourceSpecifier::Amount(
            XRD,
            dec!(0)
        ))])
    );
}

/// This test ensures that when we perform a take and return to worktop of a
/// zero amount that we get the appropriate set of worktop states that we expect
/// to see.
#[test]
fn take_and_return_to_worktop_of_zero_have_invocation_information_in_static_analyzer(
) {
    // Arrange
    let mut ledger = LedgerSimulatorBuilder::new().build();
    let (_, _, account1) = ledger.new_account(false);
    let (_, _, account2) = ledger.new_account(false);

    // Act
    let manifest = ManifestBuilder::new()
        .lock_fee_and_withdraw(account1, 10, XRD, 0)
        .take_from_worktop(XRD, 0, "bucket")
        .return_to_worktop("bucket")
        .try_deposit_entire_worktop_or_abort(account2, None)
        .build();
    let invocation_static_information = {
        let interpreter = StaticManifestInterpreter::new(
            ValidationRuleset::babylon_equivalent(),
            &manifest,
        );
        let mut visitor = StaticResourceMovementsVisitor::new(false);
        interpreter
            .validate_and_apply_visitor(&mut visitor)
            .unwrap();
        visitor.output().invocation_static_information
    };

    // Assert
    assert!(invocation_static_information.get(&0).is_some_and(
        |invocation_information| {
            invocation_information
                .output
                .specified_resources()
                .get(&XRD)
                .is_some_and(|xrd| xrd.is_zero())
        },
    ));
    assert!(invocation_static_information.get(&1).is_none());
    assert!(invocation_static_information.get(&2).is_none());
    assert!(invocation_static_information.get(&3).is_some_and(
        |invocation_information| {
            invocation_information
                .input
                .specified_resources()
                .get(&XRD)
                .is_some_and(|xrd| xrd.is_zero())
        },
    ));
}
