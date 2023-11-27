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

use radix_engine_interface::blueprints::account::{
    AccountAddAuthorizedDepositorInput, ACCOUNT_ADD_AUTHORIZED_DEPOSITOR,
};
use radix_engine_toolkit::instruction_visitor::{
    core::traverser::traverse,
    visitors::transaction_type::general_transaction_visitor::GeneralTransactionTypeVisitor,
};
use scrypto::prelude::*;
use scrypto_unit::*;
use transaction::prelude::*;

#[test]
fn account_create_proof_method_is_allowed_in_general_transaction() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().without_trace().build();
    let (pk, _, account) = test_runner.new_account(true);

    let manifest = ManifestBuilder::new()
        .create_proof_from_account_of_amount(account, XRD, 1)
        .build();
    let receipt = test_runner.preview_manifest(
        manifest.clone(),
        vec![pk.into()],
        Default::default(),
        PreviewFlags {
            use_free_credit: true,
            assume_all_signature_proofs: true,
            skip_epoch_check: true,
        },
    );

    // Act
    let mut visitor =
        GeneralTransactionTypeVisitor::new(
            receipt
                .expect_commit_success()
                .execution_trace
                .as_ref()
                .unwrap(),
        );
    traverse(&manifest.instructions, &mut [&mut visitor]).unwrap();

    // Assert
    assert!(visitor.output().is_some())
}

#[test]
fn account_burn_resources_method_is_disallowed_in_general_transaction() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().without_trace().build();
    let (pk, _, account) = test_runner.new_account(true);
    let resource_address = test_runner
        .create_freely_mintable_and_burnable_fungible_resource(
            OwnerRole::None,
            Some(1.into()),
            18,
            account,
        );

    let manifest = ManifestBuilder::new()
        .lock_fee(account, 10)
        .burn_in_account(account, resource_address, 1)
        .build();
    let receipt = test_runner.preview_manifest(
        manifest.clone(),
        vec![pk.into()],
        Default::default(),
        Default::default(),
    );

    // Act
    let mut visitor =
        GeneralTransactionTypeVisitor::new(
            receipt
                .expect_commit_success()
                .execution_trace
                .as_ref()
                .unwrap(),
        );
    traverse(&manifest.instructions, &mut [&mut visitor]).unwrap();

    // Assert
    assert!(visitor.output().is_none())
}

#[test]
fn account_add_authorized_depositor_method_is_disallowed_in_general_transaction(
) {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().without_trace().build();
    let (pk, _, account) = test_runner.new_account(true);

    let manifest =
        ManifestBuilder::new()
            .lock_fee(account, 10)
            .call_method(
                account,
                ACCOUNT_ADD_AUTHORIZED_DEPOSITOR,
                AccountAddAuthorizedDepositorInput {
                    badge: ResourceOrNonFungible::Resource(XRD),
                },
            )
            .build();
    let receipt = test_runner.preview_manifest(
        manifest.clone(),
        vec![pk.into()],
        Default::default(),
        Default::default(),
    );

    // Act
    let mut visitor =
        GeneralTransactionTypeVisitor::new(
            receipt
                .expect_commit_success()
                .execution_trace
                .as_ref()
                .unwrap(),
        );
    traverse(&manifest.instructions, &mut [&mut visitor]).unwrap();

    // Assert
    assert!(visitor.output().is_none())
}
