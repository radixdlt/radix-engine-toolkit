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

#[test]
fn transfer_of_fungibles_with_take_from_worktop_results_in_guaranteed_deposit()
{
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (_, _, account1) = ledger.new_account(true);
    let (_, _, account2) = ledger.new_account(true);

    let manifest = ManifestBuilder::new()
        .withdraw_from_account(account1, XRD, 10)
        .take_from_worktop(XRD, 10, "bucket")
        .try_deposit_or_abort(account2, None, "bucket")
        .build();

    // Act
    let (
        _,
        DynamicAnalysis {
            account_dynamic_resource_movements_summary:
                AccountDynamicResourceMovements {
                    account_withdraws,
                    account_deposits,
                },
            ..
        },
    ) = ledger.analyze(manifest);

    // Assert
    assert_eq!(account_withdraws.len(), 1);
    assert_eq!(account_deposits.len(), 1);
    assert_eq!(
        account_withdraws.first(),
        Some((
            &GlobalAddress::from(account1),
            &vec![InvocationIoItem::Fungible(
                XRD,
                EitherGuaranteedOrPredicted::Guaranteed(10.into())
            )]
        ))
    );
    assert_eq!(
        account_deposits.first(),
        Some((
            &GlobalAddress::from(account2),
            &vec![InvocationIoItem::Fungible(
                XRD,
                EitherGuaranteedOrPredicted::Guaranteed(10.into())
            )]
        ))
    );
}

#[test]
fn transfer_of_fungibles_with_take_all_from_worktop_results_in_guaranteed_deposit(
) {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (_, _, account1) = ledger.new_account(true);
    let (_, _, account2) = ledger.new_account(true);

    let manifest = ManifestBuilder::new()
        .withdraw_from_account(account1, XRD, 10)
        .take_all_from_worktop(XRD, "bucket")
        .try_deposit_or_abort(account2, None, "bucket")
        .build();

    // Act
    let (
        _,
        DynamicAnalysis {
            account_dynamic_resource_movements_summary:
                AccountDynamicResourceMovements {
                    account_withdraws,
                    account_deposits,
                },
            ..
        },
    ) = ledger.analyze(manifest);

    // Assert
    assert_eq!(account_withdraws.len(), 1);
    assert_eq!(account_deposits.len(), 1);
    assert_eq!(
        account_withdraws.first(),
        Some((
            &GlobalAddress::from(account1),
            &vec![InvocationIoItem::Fungible(
                XRD,
                EitherGuaranteedOrPredicted::Guaranteed(10.into())
            )]
        ))
    );
    assert_eq!(
        account_deposits.first(),
        Some((
            &GlobalAddress::from(account2),
            &vec![InvocationIoItem::Fungible(
                XRD,
                EitherGuaranteedOrPredicted::Guaranteed(10.into())
            )]
        ))
    );
}

#[test]
fn transfer_of_non_fungibles_by_amount_results_in_predicted_deposit() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (_, _, account1) = ledger.new_account(true);
    let (_, _, account2) = ledger.new_account(true);
    let resource_address = ledger.create_non_fungible_resource(account1);
    let ids = (1..=3)
        .map(NonFungibleLocalId::integer)
        .collect::<IndexSet<_>>();

    let manifest = ManifestBuilder::new()
        .withdraw_from_account(account1, resource_address, 3)
        .take_all_from_worktop(resource_address, "bucket")
        .try_deposit_or_abort(account2, None, "bucket")
        .build();

    // Act
    let (
        _,
        DynamicAnalysis {
            account_dynamic_resource_movements_summary:
                AccountDynamicResourceMovements {
                    account_withdraws,
                    account_deposits,
                },
            ..
        },
    ) = ledger.analyze(manifest);

    // Assert
    assert_eq!(account_withdraws.len(), 1);
    assert_eq!(account_deposits.len(), 1);
    assert_eq!(
        account_withdraws.first(),
        Some((
            &GlobalAddress::from(account1),
            &vec![InvocationIoItem::NonFungible(
                resource_address,
                EitherGuaranteedOrPredicted::Predicted(Tracked {
                    value: ids.clone(),
                    created_at: 0.into()
                })
            )]
        ))
    );
    assert_eq!(
        account_deposits.first(),
        Some((
            &GlobalAddress::from(account2),
            &vec![InvocationIoItem::NonFungible(
                resource_address,
                EitherGuaranteedOrPredicted::Predicted(Tracked {
                    value: ids,
                    created_at: 1.into()
                })
            )]
        ))
    );
}

#[test]
fn transfer_of_non_fungibles_by_amount_with_take_by_ids_results_in_guaranteed_deposit(
) {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (_, _, account1) = ledger.new_account(true);
    let (_, _, account2) = ledger.new_account(true);
    let resource_address = ledger.create_non_fungible_resource(account1);
    let ids = (1..=3)
        .map(NonFungibleLocalId::integer)
        .collect::<IndexSet<_>>();

    let manifest = ManifestBuilder::new()
        .withdraw_from_account(account1, resource_address, 3)
        .take_non_fungibles_from_worktop(
            resource_address,
            ids.clone(),
            "bucket",
        )
        .try_deposit_or_abort(account2, None, "bucket")
        .build();

    // Act
    let (
        _,
        DynamicAnalysis {
            account_dynamic_resource_movements_summary:
                AccountDynamicResourceMovements {
                    account_withdraws,
                    account_deposits,
                },
            ..
        },
    ) = ledger.analyze(manifest);

    // Assert
    assert_eq!(account_withdraws.len(), 1);
    assert_eq!(account_deposits.len(), 1);
    assert_eq!(
        account_withdraws.first(),
        Some((
            &GlobalAddress::from(account1),
            &vec![InvocationIoItem::NonFungible(
                resource_address,
                EitherGuaranteedOrPredicted::Predicted(Tracked {
                    value: ids.clone(),
                    created_at: 0.into()
                })
            )]
        ))
    );
    assert_eq!(
        account_deposits.first(),
        Some((
            &GlobalAddress::from(account2),
            &vec![InvocationIoItem::NonFungible(
                resource_address,
                EitherGuaranteedOrPredicted::Guaranteed(ids)
            )]
        ))
    );
}

#[test]
fn transfer_of_non_fungibles_by_amount_assertion_results_in_guaranteed_deposit()
{
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (_, _, account1) = ledger.new_account(true);
    let (_, _, account2) = ledger.new_account(true);
    let resource_address = ledger.create_non_fungible_resource(account1);
    let ids = (1..=3)
        .map(NonFungibleLocalId::integer)
        .collect::<IndexSet<_>>();

    let manifest = ManifestBuilder::new()
        .withdraw_from_account(account1, resource_address, 3)
        .assert_worktop_contains_non_fungibles(resource_address, ids.clone())
        .take_all_from_worktop(resource_address, "bucket")
        .try_deposit_or_abort(account2, None, "bucket")
        .build();

    // Act
    let (
        _,
        DynamicAnalysis {
            account_dynamic_resource_movements_summary:
                AccountDynamicResourceMovements {
                    account_withdraws,
                    account_deposits,
                },
            ..
        },
    ) = ledger.analyze(manifest);

    // Assert
    assert_eq!(account_withdraws.len(), 1);
    assert_eq!(account_deposits.len(), 1);
    assert_eq!(
        account_withdraws.first(),
        Some((
            &GlobalAddress::from(account1),
            &vec![InvocationIoItem::NonFungible(
                resource_address,
                EitherGuaranteedOrPredicted::Predicted(Tracked {
                    value: ids.clone(),
                    created_at: 0.into()
                })
            )]
        ))
    );
    assert_eq!(
        account_deposits.first(),
        Some((
            &GlobalAddress::from(account2),
            &vec![InvocationIoItem::NonFungible(
                resource_address,
                EitherGuaranteedOrPredicted::Guaranteed(ids)
            )]
        ))
    );
}

#[test]
fn getting_xrd_from_faucet_results_in_predicted_deposit() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (_, _, account) = ledger.new_account(true);

    let manifest = ManifestBuilder::new()
        .get_free_xrd_from_faucet()
        .take_all_from_worktop(XRD, "bucket")
        .try_deposit_or_abort(account, None, "bucket")
        .build();

    // Act
    let (
        _,
        DynamicAnalysis {
            account_dynamic_resource_movements_summary:
                AccountDynamicResourceMovements {
                    account_withdraws,
                    account_deposits,
                },
            ..
        },
    ) = ledger.analyze(manifest);

    // Assert
    assert_eq!(account_withdraws.len(), 0);
    assert_eq!(account_deposits.len(), 1);
    assert_eq!(
        account_deposits.first(),
        Some((
            &GlobalAddress::from(account),
            &vec![InvocationIoItem::Fungible(
                XRD,
                EitherGuaranteedOrPredicted::Predicted(Tracked {
                    value: 10_000.into(),
                    created_at: 1.into()
                })
            )]
        ))
    );
}

#[test]
fn all_account_withdraw_and_deposit_methods_are_picked_up() {
    use DepositMethod::*;
    use WithdrawMethod::*;

    let withdraw_methods = [
        Withdraw,
        WithdrawNonFungibles,
        LockFeeAndWithdraw,
        LockFeeAndWithdrawNonFungibles,
    ];

    let deposit_methods = [
        Deposit,
        DepositBatch,
        TryDepositOrAbort,
        TryDepositBatchOrAbort,
        TryDepositOrRefund,
        TryDepositBatchOrRefund,
    ];

    for (withdraw_method, deposit_method) in withdraw_methods
        .into_iter()
        .cartesian_product(deposit_methods)
    {
        test_transfer(withdraw_method, deposit_method);
    }
}

fn test_transfer(
    withdraw_method: WithdrawMethod,
    deposit_method: DepositMethod,
) {
    // Arrange
    println!("Working on {withdraw_method:#?} & {deposit_method:#?}");
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (_, _, source_account) = ledger.new_account(true);
    let (_, _, destination_account) = ledger.new_account(true);
    let ids = (1..=3)
        .map(NonFungibleLocalId::integer)
        .collect::<IndexSet<_>>();

    let resource_address = match withdraw_method {
        WithdrawMethod::Withdraw | WithdrawMethod::LockFeeAndWithdraw => XRD,
        WithdrawMethod::WithdrawNonFungibles
        | WithdrawMethod::LockFeeAndWithdrawNonFungibles => {
            ledger.create_non_fungible_resource(source_account)
        }
    };
    let is_fungible = resource_address.is_fungible();
    let is_or_refund_method = matches!(
        deposit_method,
        DepositMethod::TryDepositBatchOrRefund
            | DepositMethod::TryDepositOrRefund
    );

    let manifest = ManifestBuilder::new()
        .with_name_lookup(|mut builder, lookup| {
            builder = match withdraw_method {
                WithdrawMethod::Withdraw => builder.withdraw_from_account(
                    source_account,
                    resource_address,
                    3,
                ),
                WithdrawMethod::WithdrawNonFungibles => builder
                    .withdraw_non_fungibles_from_account(
                        source_account,
                        resource_address,
                        ids.clone(),
                    ),
                WithdrawMethod::LockFeeAndWithdraw => builder
                    .lock_fee_and_withdraw(
                        source_account,
                        1,
                        resource_address,
                        3,
                    ),
                WithdrawMethod::LockFeeAndWithdrawNonFungibles => builder
                    .lock_fee_and_withdraw_non_fungibles(
                        source_account,
                        1,
                        resource_address,
                        ids.clone(),
                    ),
            };

            builder = match is_fungible {
                true => {
                    builder.take_from_worktop(resource_address, 3, "bucket")
                }
                false => builder.take_non_fungibles_from_worktop(
                    resource_address,
                    ids.clone(),
                    "bucket",
                ),
            };

            let bucket = lookup.bucket("bucket");

            match deposit_method {
                DepositMethod::Deposit => {
                    builder.deposit(destination_account, bucket)
                }
                DepositMethod::DepositBatch => {
                    builder.deposit_batch(destination_account, [bucket])
                }
                DepositMethod::TryDepositOrAbort => builder
                    .try_deposit_batch_or_abort(
                        destination_account,
                        [bucket],
                        None,
                    ),
                DepositMethod::TryDepositBatchOrAbort => builder
                    .try_deposit_batch_or_abort(
                        destination_account,
                        [bucket],
                        None,
                    ),
                DepositMethod::TryDepositOrRefund => builder
                    .try_deposit_batch_or_refund(
                        destination_account,
                        [bucket],
                        None,
                    ),
                DepositMethod::TryDepositBatchOrRefund => builder
                    .try_deposit_batch_or_refund(
                        destination_account,
                        [bucket],
                        None,
                    ),
            }
        })
        .build();

    // Act
    let (
        _,
        DynamicAnalysis {
            account_dynamic_resource_movements_summary:
                AccountDynamicResourceMovements {
                    account_withdraws,
                    account_deposits,
                },
            ..
        },
    ) = ledger.analyze(manifest);

    // Assert
    assert_eq!(account_withdraws.len(), 1);
    if is_or_refund_method {
        assert_eq!(account_deposits.len(), 0);
    } else {
        assert_eq!(account_deposits.len(), 1);
    }
    if is_fungible {
        assert_eq!(
            account_withdraws.first(),
            Some((
                &GlobalAddress::from(source_account),
                &vec![InvocationIoItem::Fungible(
                    XRD,
                    EitherGuaranteedOrPredicted::Guaranteed(3.into())
                )]
            ))
        );
        if !is_or_refund_method {
            assert_eq!(
                account_deposits.first(),
                Some((
                    &GlobalAddress::from(destination_account),
                    &vec![InvocationIoItem::Fungible(
                        XRD,
                        EitherGuaranteedOrPredicted::Guaranteed(3.into())
                    )]
                ))
            );
        }
    } else {
        assert_eq!(
            account_withdraws.first(),
            Some((
                &GlobalAddress::from(source_account),
                &vec![InvocationIoItem::NonFungible(
                    resource_address,
                    EitherGuaranteedOrPredicted::Guaranteed(ids.clone())
                )]
            ))
        );
        if !is_or_refund_method {
            assert_eq!(
                account_deposits.first(),
                Some((
                    &GlobalAddress::from(destination_account),
                    &vec![InvocationIoItem::NonFungible(
                        resource_address,
                        EitherGuaranteedOrPredicted::Guaranteed(ids)
                    )]
                ))
            );
        }
    };
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WithdrawMethod {
    Withdraw,
    WithdrawNonFungibles,
    LockFeeAndWithdraw,
    LockFeeAndWithdrawNonFungibles,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DepositMethod {
    Deposit,
    DepositBatch,
    TryDepositOrAbort,
    TryDepositBatchOrAbort,
    TryDepositOrRefund,
    TryDepositBatchOrRefund,
}
