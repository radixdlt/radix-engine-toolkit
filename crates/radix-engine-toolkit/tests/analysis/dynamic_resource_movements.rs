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

/// This is a test that the instruction index that we have in the [`Tracked`]
/// resources is the instruction index of their creation and not that of their
/// consumption. This is to allow the wallet to be able to create assertions
/// correctly.
#[test]
fn tracked_invocation_io_item_has_instruction_index_of_its_creation() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();

    let (_, _, account) = ledger.new_account(false);
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .get_free_xrd_from_faucet()
        .take_all_from_worktop(XRD, "xrd")
        .try_deposit_or_abort(account, None, "xrd")
        .build();

    // Act
    let (_, dynamic_analysis) = ledger.analyze(manifest);

    // Assert
    let account_deposits = dynamic_analysis
        .account_dynamic_resource_movements_summary
        .account_deposits
        .get(&GlobalAddress::from(account))
        .unwrap();
    assert_eq!(account_deposits.len(), 1);
    assert!(matches!(
        account_deposits.first(),
        Some(InvocationIoItem::Fungible(
            XRD,
            EitherGuaranteedOrPredicted::Predicted(Tracked {
                value,
                created_at
            })
        )) if *value == dec!(10_000) && *created_at.value() == 2
    ));
}
