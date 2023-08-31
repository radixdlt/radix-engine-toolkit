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

use radix_engine_interface::blueprints::access_controller::*;
use radix_engine_interface::blueprints::account::*;
use radix_engine_toolkit_core::functions::manifest::*;
use scrypto::prelude::*;
use transaction::prelude::*;

mod test_data;

#[test]
fn manifest_hash_can_be_obtained() {
    // Arrange
    let manifest = test_data::manifest();

    // Act
    let manifest_hash = radix_engine_toolkit_core::functions::manifest::hash(&manifest);

    // Assert
    assert!(manifest_hash.is_ok())
}

#[test]
fn manifest_can_be_compiled() {
    // Arrange
    let manifest = test_data::manifest();

    // Act
    let compiled = radix_engine_toolkit_core::functions::manifest::compile(&manifest);

    // Assert
    assert!(compiled.is_ok())
}

#[test]
fn manifest_can_be_compiled_and_later_decompiled() {
    // Arrange
    let manifest = test_data::manifest();
    let compiled = radix_engine_toolkit_core::functions::manifest::compile(&manifest).unwrap();

    // Act
    let decompiled = radix_engine_toolkit_core::functions::manifest::decompile(compiled);

    // Assert
    assert!(decompiled.is_ok());
    assert_eq!(decompiled, Ok(manifest))
}

#[test]
fn manifest_can_be_statically_validated() {
    // Arrange
    let manifest = test_data::manifest();

    // Act
    let validation_result =
        radix_engine_toolkit_core::functions::manifest::statically_validate(&manifest);

    // Assert
    assert!(validation_result.is_ok())
}

#[test]
fn manifest_modification_assertions_are_added_at_expected_indices() {
    // Arrange
    let manifest = TransactionManifestV1 {
        instructions: vec![
            InstructionV1::CallMethod {
                address: DynamicGlobalAddress::Static(FAUCET.into()),
                method_name: "free".to_owned(),
                args: to_manifest_value_and_unwrap!(&()),
            },
            InstructionV1::CallMethod {
                address: DynamicGlobalAddress::Static(account(1).into()),
                method_name: "deposit_batch".to_owned(),
                args: manifest_args!(ManifestExpression::EntireWorktop).into(),
            },
            InstructionV1::CallMethod {
                address: DynamicGlobalAddress::Static(FAUCET.into()),
                method_name: "free".to_owned(),
                args: to_manifest_value_and_unwrap!(&()),
            },
            InstructionV1::CallMethod {
                address: DynamicGlobalAddress::Static(account(2).into()),
                method_name: "deposit_batch".to_owned(),
                args: manifest_args!(ManifestExpression::EntireWorktop).into(),
            },
            InstructionV1::CallMethod {
                address: DynamicGlobalAddress::Static(FAUCET.into()),
                method_name: "free".to_owned(),
                args: to_manifest_value_and_unwrap!(&()),
            },
            InstructionV1::CallMethod {
                address: DynamicGlobalAddress::Static(account(3).into()),
                method_name: "deposit_batch".to_owned(),
                args: manifest_args!(ManifestExpression::EntireWorktop).into(),
            },
        ],
        blobs: Default::default(),
    };
    let modifications = TransactionManifestModifications {
        add_access_controller_proofs: Default::default(),
        add_lock_fee: Default::default(),
        add_assertions: vec![
            (
                1,
                Assertion::Amount {
                    resource_address: XRD,
                    amount: dec!("100"),
                },
            ),
            (
                3,
                Assertion::Amount {
                    resource_address: XRD,
                    amount: dec!("200"),
                },
            ),
            (
                5,
                Assertion::Amount {
                    resource_address: XRD,
                    amount: dec!("300"),
                },
            ),
        ],
    };

    // Act
    let modified_manifest =
        radix_engine_toolkit_core::functions::manifest::modify(&manifest, modifications).unwrap();

    // Assert
    let expected_instructions = vec![
        InstructionV1::CallMethod {
            address: DynamicGlobalAddress::Static(FAUCET.into()),
            method_name: "free".to_owned(),
            args: to_manifest_value_and_unwrap!(&()),
        },
        InstructionV1::AssertWorktopContains {
            resource_address: XRD,
            amount: dec!("100"),
        },
        InstructionV1::CallMethod {
            address: DynamicGlobalAddress::Static(account(1).into()),
            method_name: "deposit_batch".to_owned(),
            args: manifest_args!(ManifestExpression::EntireWorktop).into(),
        },
        InstructionV1::CallMethod {
            address: DynamicGlobalAddress::Static(FAUCET.into()),
            method_name: "free".to_owned(),
            args: to_manifest_value_and_unwrap!(&()),
        },
        InstructionV1::AssertWorktopContains {
            resource_address: XRD,
            amount: dec!("200"),
        },
        InstructionV1::CallMethod {
            address: DynamicGlobalAddress::Static(account(2).into()),
            method_name: "deposit_batch".to_owned(),
            args: manifest_args!(ManifestExpression::EntireWorktop).into(),
        },
        InstructionV1::CallMethod {
            address: DynamicGlobalAddress::Static(FAUCET.into()),
            method_name: "free".to_owned(),
            args: to_manifest_value_and_unwrap!(&()),
        },
        InstructionV1::AssertWorktopContains {
            resource_address: XRD,
            amount: dec!("300"),
        },
        InstructionV1::CallMethod {
            address: DynamicGlobalAddress::Static(account(3).into()),
            method_name: "deposit_batch".to_owned(),
            args: manifest_args!(ManifestExpression::EntireWorktop).into(),
        },
    ];
    assert_eq!(modified_manifest.instructions, expected_instructions);
}

#[test]
fn manifest_modification_assertions_are_added_at_expected_indices_even_when_assertions_are_unsorted(
) {
    // Arrange
    let manifest = TransactionManifestV1 {
        instructions: vec![
            InstructionV1::CallMethod {
                address: DynamicGlobalAddress::Static(FAUCET.into()),
                method_name: "free".to_owned(),
                args: to_manifest_value_and_unwrap!(&()),
            },
            InstructionV1::CallMethod {
                address: DynamicGlobalAddress::Static(account(1).into()),
                method_name: "deposit_batch".to_owned(),
                args: manifest_args!(ManifestExpression::EntireWorktop).into(),
            },
            InstructionV1::CallMethod {
                address: DynamicGlobalAddress::Static(FAUCET.into()),
                method_name: "free".to_owned(),
                args: to_manifest_value_and_unwrap!(&()),
            },
            InstructionV1::CallMethod {
                address: DynamicGlobalAddress::Static(account(2).into()),
                method_name: "deposit_batch".to_owned(),
                args: manifest_args!(ManifestExpression::EntireWorktop).into(),
            },
            InstructionV1::CallMethod {
                address: DynamicGlobalAddress::Static(FAUCET.into()),
                method_name: "free".to_owned(),
                args: to_manifest_value_and_unwrap!(&()),
            },
            InstructionV1::CallMethod {
                address: DynamicGlobalAddress::Static(account(3).into()),
                method_name: "deposit_batch".to_owned(),
                args: manifest_args!(ManifestExpression::EntireWorktop).into(),
            },
        ],
        blobs: Default::default(),
    };
    let modifications = TransactionManifestModifications {
        add_access_controller_proofs: Default::default(),
        add_lock_fee: Default::default(),
        add_assertions: vec![
            (
                3,
                Assertion::Amount {
                    resource_address: XRD,
                    amount: dec!("200"),
                },
            ),
            (
                1,
                Assertion::Amount {
                    resource_address: XRD,
                    amount: dec!("100"),
                },
            ),
            (
                5,
                Assertion::Amount {
                    resource_address: XRD,
                    amount: dec!("300"),
                },
            ),
        ],
    };

    // Act
    let modified_manifest =
        radix_engine_toolkit_core::functions::manifest::modify(&manifest, modifications).unwrap();

    // Assert
    let expected_instructions = vec![
        InstructionV1::CallMethod {
            address: DynamicGlobalAddress::Static(FAUCET.into()),
            method_name: "free".to_owned(),
            args: to_manifest_value_and_unwrap!(&()),
        },
        InstructionV1::AssertWorktopContains {
            resource_address: XRD,
            amount: dec!("100"),
        },
        InstructionV1::CallMethod {
            address: DynamicGlobalAddress::Static(account(1).into()),
            method_name: "deposit_batch".to_owned(),
            args: manifest_args!(ManifestExpression::EntireWorktop).into(),
        },
        InstructionV1::CallMethod {
            address: DynamicGlobalAddress::Static(FAUCET.into()),
            method_name: "free".to_owned(),
            args: to_manifest_value_and_unwrap!(&()),
        },
        InstructionV1::AssertWorktopContains {
            resource_address: XRD,
            amount: dec!("200"),
        },
        InstructionV1::CallMethod {
            address: DynamicGlobalAddress::Static(account(2).into()),
            method_name: "deposit_batch".to_owned(),
            args: manifest_args!(ManifestExpression::EntireWorktop).into(),
        },
        InstructionV1::CallMethod {
            address: DynamicGlobalAddress::Static(FAUCET.into()),
            method_name: "free".to_owned(),
            args: to_manifest_value_and_unwrap!(&()),
        },
        InstructionV1::AssertWorktopContains {
            resource_address: XRD,
            amount: dec!("300"),
        },
        InstructionV1::CallMethod {
            address: DynamicGlobalAddress::Static(account(3).into()),
            method_name: "deposit_batch".to_owned(),
            args: manifest_args!(ManifestExpression::EntireWorktop).into(),
        },
    ];
    assert_eq!(modified_manifest.instructions, expected_instructions);
}

#[test]
fn assertion_out_of_bounds_results_in_an_error_not_a_panic() {
    // Arrange
    let manifest = TransactionManifestV1 {
        instructions: vec![],
        blobs: Default::default(),
    };

    let modifications = TransactionManifestModifications {
        add_access_controller_proofs: Default::default(),
        add_lock_fee: Default::default(),
        add_assertions: vec![
            (
                1,
                Assertion::Amount {
                    resource_address: XRD,
                    amount: dec!("300"),
                },
            ),
            (
                2,
                Assertion::Amount {
                    resource_address: XRD,
                    amount: dec!("300"),
                },
            ),
        ],
    };

    // Act
    let result = radix_engine_toolkit_core::functions::manifest::modify(&manifest, modifications);

    // Assert
    assert_eq!(
        result,
        Err(ManifestModificationError::AssertionIndexOutOfBounds {
            assertion_index: 2,
            instructions_length: 0
        })
    )
}

#[test]
fn adding_lock_fee_with_no_existing_call_to_withdraw_inserts_a_new_instruction() {
    let manifest = TransactionManifestV1 {
        instructions: vec![
            InstructionV1::CallMethod {
                address: DynamicGlobalAddress::Static(FAUCET.into()),
                method_name: "free".to_owned(),
                args: to_manifest_value_and_unwrap!(&()),
            },
            InstructionV1::CallMethod {
                address: DynamicGlobalAddress::Static(account(1).into()),
                method_name: "deposit_batch".to_owned(),
                args: manifest_args!(ManifestExpression::EntireWorktop).into(),
            },
            InstructionV1::CallMethod {
                address: DynamicGlobalAddress::Static(FAUCET.into()),
                method_name: "free".to_owned(),
                args: to_manifest_value_and_unwrap!(&()),
            },
            InstructionV1::CallMethod {
                address: DynamicGlobalAddress::Static(account(2).into()),
                method_name: "deposit_batch".to_owned(),
                args: manifest_args!(ManifestExpression::EntireWorktop).into(),
            },
            InstructionV1::CallMethod {
                address: DynamicGlobalAddress::Static(FAUCET.into()),
                method_name: "free".to_owned(),
                args: to_manifest_value_and_unwrap!(&()),
            },
            InstructionV1::CallMethod {
                address: DynamicGlobalAddress::Static(account(3).into()),
                method_name: "deposit_batch".to_owned(),
                args: manifest_args!(ManifestExpression::EntireWorktop).into(),
            },
        ],
        blobs: Default::default(),
    };
    let modifications = TransactionManifestModifications {
        add_access_controller_proofs: Default::default(),
        add_lock_fee: Some((account(1), dec!("10"))),
        add_assertions: Default::default(),
    };

    // Act
    let modified_manifest =
        radix_engine_toolkit_core::functions::manifest::modify(&manifest, modifications).unwrap();

    // Assert
    let expected_instructions = vec![
        InstructionV1::CallMethod {
            address: DynamicGlobalAddress::Static(account(1).into()),
            method_name: ACCOUNT_LOCK_FEE_IDENT.to_owned(),
            args: to_manifest_value_and_unwrap!(&AccountLockFeeInput { amount: dec!("10") }),
        },
        InstructionV1::CallMethod {
            address: DynamicGlobalAddress::Static(FAUCET.into()),
            method_name: "free".to_owned(),
            args: to_manifest_value_and_unwrap!(&()),
        },
        InstructionV1::CallMethod {
            address: DynamicGlobalAddress::Static(account(1).into()),
            method_name: "deposit_batch".to_owned(),
            args: manifest_args!(ManifestExpression::EntireWorktop).into(),
        },
        InstructionV1::CallMethod {
            address: DynamicGlobalAddress::Static(FAUCET.into()),
            method_name: "free".to_owned(),
            args: to_manifest_value_and_unwrap!(&()),
        },
        InstructionV1::CallMethod {
            address: DynamicGlobalAddress::Static(account(2).into()),
            method_name: "deposit_batch".to_owned(),
            args: manifest_args!(ManifestExpression::EntireWorktop).into(),
        },
        InstructionV1::CallMethod {
            address: DynamicGlobalAddress::Static(FAUCET.into()),
            method_name: "free".to_owned(),
            args: to_manifest_value_and_unwrap!(&()),
        },
        InstructionV1::CallMethod {
            address: DynamicGlobalAddress::Static(account(3).into()),
            method_name: "deposit_batch".to_owned(),
            args: manifest_args!(ManifestExpression::EntireWorktop).into(),
        },
    ];
    assert_eq!(modified_manifest.instructions, expected_instructions);
}

#[test]
fn adding_lock_fee_with_no_existing_call_to_withdraw_inserts_a_new_instruction_even_when_manifest_is_empty(
) {
    let manifest = TransactionManifestV1 {
        instructions: vec![],
        blobs: Default::default(),
    };
    let modifications = TransactionManifestModifications {
        add_access_controller_proofs: Default::default(),
        add_lock_fee: Some((account(1), dec!("10"))),
        add_assertions: Default::default(),
    };

    // Act
    let modified_manifest =
        radix_engine_toolkit_core::functions::manifest::modify(&manifest, modifications).unwrap();

    // Assert
    let expected_instructions = vec![InstructionV1::CallMethod {
        address: DynamicGlobalAddress::Static(account(1).into()),
        method_name: ACCOUNT_LOCK_FEE_IDENT.to_owned(),
        args: to_manifest_value_and_unwrap!(&AccountLockFeeInput { amount: dec!("10") }),
    }];
    assert_eq!(modified_manifest.instructions, expected_instructions);
}

#[test]
fn adding_lock_fee_with_an_existing_withdraw_converts_it_to_lock_fee_and_withdraw() {
    let manifest = TransactionManifestV1 {
        instructions: vec![
            InstructionV1::CallMethod {
                address: DynamicGlobalAddress::Static(account(1).into()),
                method_name: ACCOUNT_WITHDRAW_IDENT.to_owned(),
                args: to_manifest_value_and_unwrap!(&AccountWithdrawInput {
                    resource_address: XRD,
                    amount: dec!("999")
                }),
            },
            InstructionV1::CallMethod {
                address: DynamicGlobalAddress::Static(FAUCET.into()),
                method_name: "free".to_owned(),
                args: to_manifest_value_and_unwrap!(&()),
            },
            InstructionV1::CallMethod {
                address: DynamicGlobalAddress::Static(account(1).into()),
                method_name: "deposit_batch".to_owned(),
                args: manifest_args!(ManifestExpression::EntireWorktop).into(),
            },
            InstructionV1::CallMethod {
                address: DynamicGlobalAddress::Static(FAUCET.into()),
                method_name: "free".to_owned(),
                args: to_manifest_value_and_unwrap!(&()),
            },
            InstructionV1::CallMethod {
                address: DynamicGlobalAddress::Static(account(2).into()),
                method_name: "deposit_batch".to_owned(),
                args: manifest_args!(ManifestExpression::EntireWorktop).into(),
            },
            InstructionV1::CallMethod {
                address: DynamicGlobalAddress::Static(FAUCET.into()),
                method_name: "free".to_owned(),
                args: to_manifest_value_and_unwrap!(&()),
            },
            InstructionV1::CallMethod {
                address: DynamicGlobalAddress::Static(account(3).into()),
                method_name: "deposit_batch".to_owned(),
                args: manifest_args!(ManifestExpression::EntireWorktop).into(),
            },
        ],
        blobs: Default::default(),
    };
    let modifications = TransactionManifestModifications {
        add_access_controller_proofs: Default::default(),
        add_lock_fee: Some((account(1), dec!("10"))),
        add_assertions: Default::default(),
    };

    // Act
    let modified_manifest =
        radix_engine_toolkit_core::functions::manifest::modify(&manifest, modifications).unwrap();

    // Assert
    let expected_instructions = vec![
        InstructionV1::CallMethod {
            address: DynamicGlobalAddress::Static(account(1).into()),
            method_name: ACCOUNT_LOCK_FEE_AND_WITHDRAW_IDENT.to_owned(),
            args: to_manifest_value_and_unwrap!(&AccountLockFeeAndWithdrawInput {
                amount_to_lock: dec!("10"),
                resource_address: XRD,
                amount: dec!("999")
            }),
        },
        InstructionV1::CallMethod {
            address: DynamicGlobalAddress::Static(FAUCET.into()),
            method_name: "free".to_owned(),
            args: to_manifest_value_and_unwrap!(&()),
        },
        InstructionV1::CallMethod {
            address: DynamicGlobalAddress::Static(account(1).into()),
            method_name: "deposit_batch".to_owned(),
            args: manifest_args!(ManifestExpression::EntireWorktop).into(),
        },
        InstructionV1::CallMethod {
            address: DynamicGlobalAddress::Static(FAUCET.into()),
            method_name: "free".to_owned(),
            args: to_manifest_value_and_unwrap!(&()),
        },
        InstructionV1::CallMethod {
            address: DynamicGlobalAddress::Static(account(2).into()),
            method_name: "deposit_batch".to_owned(),
            args: manifest_args!(ManifestExpression::EntireWorktop).into(),
        },
        InstructionV1::CallMethod {
            address: DynamicGlobalAddress::Static(FAUCET.into()),
            method_name: "free".to_owned(),
            args: to_manifest_value_and_unwrap!(&()),
        },
        InstructionV1::CallMethod {
            address: DynamicGlobalAddress::Static(account(3).into()),
            method_name: "deposit_batch".to_owned(),
            args: manifest_args!(ManifestExpression::EntireWorktop).into(),
        },
    ];
    assert_eq!(modified_manifest.instructions, expected_instructions);
}

#[test]
fn adding_lock_fee_with_an_existing_withdraw_non_fungibles_converts_it_to_lock_fee_and_withdraw_non_fungibles(
) {
    let manifest = TransactionManifestV1 {
        instructions: vec![
            InstructionV1::CallMethod {
                address: DynamicGlobalAddress::Static(account(1).into()),
                method_name: ACCOUNT_WITHDRAW_NON_FUNGIBLES_IDENT.to_owned(),
                args: to_manifest_value_and_unwrap!(&AccountWithdrawNonFungiblesInput {
                    resource_address: XRD,
                    ids: [
                        NonFungibleLocalId::integer(1),
                        NonFungibleLocalId::integer(2)
                    ]
                    .into()
                }),
            },
            InstructionV1::CallMethod {
                address: DynamicGlobalAddress::Static(FAUCET.into()),
                method_name: "free".to_owned(),
                args: to_manifest_value_and_unwrap!(&()),
            },
            InstructionV1::CallMethod {
                address: DynamicGlobalAddress::Static(account(1).into()),
                method_name: "deposit_batch".to_owned(),
                args: manifest_args!(ManifestExpression::EntireWorktop).into(),
            },
            InstructionV1::CallMethod {
                address: DynamicGlobalAddress::Static(FAUCET.into()),
                method_name: "free".to_owned(),
                args: to_manifest_value_and_unwrap!(&()),
            },
            InstructionV1::CallMethod {
                address: DynamicGlobalAddress::Static(account(2).into()),
                method_name: "deposit_batch".to_owned(),
                args: manifest_args!(ManifestExpression::EntireWorktop).into(),
            },
            InstructionV1::CallMethod {
                address: DynamicGlobalAddress::Static(FAUCET.into()),
                method_name: "free".to_owned(),
                args: to_manifest_value_and_unwrap!(&()),
            },
            InstructionV1::CallMethod {
                address: DynamicGlobalAddress::Static(account(3).into()),
                method_name: "deposit_batch".to_owned(),
                args: manifest_args!(ManifestExpression::EntireWorktop).into(),
            },
        ],
        blobs: Default::default(),
    };
    let modifications = TransactionManifestModifications {
        add_access_controller_proofs: Default::default(),
        add_lock_fee: Some((account(1), dec!("10"))),
        add_assertions: Default::default(),
    };

    // Act
    let modified_manifest =
        radix_engine_toolkit_core::functions::manifest::modify(&manifest, modifications).unwrap();

    // Assert
    let expected_instructions = vec![
        InstructionV1::CallMethod {
            address: DynamicGlobalAddress::Static(account(1).into()),
            method_name: ACCOUNT_LOCK_FEE_AND_WITHDRAW_NON_FUNGIBLES_IDENT.to_owned(),
            args: to_manifest_value_and_unwrap!(&AccountLockFeeAndWithdrawNonFungiblesInput {
                amount_to_lock: dec!("10"),
                resource_address: XRD,
                ids: [
                    NonFungibleLocalId::integer(1),
                    NonFungibleLocalId::integer(2)
                ]
                .into()
            }),
        },
        InstructionV1::CallMethod {
            address: DynamicGlobalAddress::Static(FAUCET.into()),
            method_name: "free".to_owned(),
            args: to_manifest_value_and_unwrap!(&()),
        },
        InstructionV1::CallMethod {
            address: DynamicGlobalAddress::Static(account(1).into()),
            method_name: "deposit_batch".to_owned(),
            args: manifest_args!(ManifestExpression::EntireWorktop).into(),
        },
        InstructionV1::CallMethod {
            address: DynamicGlobalAddress::Static(FAUCET.into()),
            method_name: "free".to_owned(),
            args: to_manifest_value_and_unwrap!(&()),
        },
        InstructionV1::CallMethod {
            address: DynamicGlobalAddress::Static(account(2).into()),
            method_name: "deposit_batch".to_owned(),
            args: manifest_args!(ManifestExpression::EntireWorktop).into(),
        },
        InstructionV1::CallMethod {
            address: DynamicGlobalAddress::Static(FAUCET.into()),
            method_name: "free".to_owned(),
            args: to_manifest_value_and_unwrap!(&()),
        },
        InstructionV1::CallMethod {
            address: DynamicGlobalAddress::Static(account(3).into()),
            method_name: "deposit_batch".to_owned(),
            args: manifest_args!(ManifestExpression::EntireWorktop).into(),
        },
    ];
    assert_eq!(modified_manifest.instructions, expected_instructions);
}

#[test]
fn adding_lock_fee_with_an_existing_withdraw_from_a_different_account_adds_a_new_instruction() {
    let manifest = TransactionManifestV1 {
        instructions: vec![
            InstructionV1::CallMethod {
                address: DynamicGlobalAddress::Static(account(1).into()),
                method_name: ACCOUNT_WITHDRAW_IDENT.to_owned(),
                args: to_manifest_value_and_unwrap!(&AccountWithdrawInput {
                    resource_address: XRD,
                    amount: dec!("999")
                }),
            },
            InstructionV1::CallMethod {
                address: DynamicGlobalAddress::Static(FAUCET.into()),
                method_name: "free".to_owned(),
                args: to_manifest_value_and_unwrap!(&()),
            },
            InstructionV1::CallMethod {
                address: DynamicGlobalAddress::Static(account(1).into()),
                method_name: "deposit_batch".to_owned(),
                args: manifest_args!(ManifestExpression::EntireWorktop).into(),
            },
        ],
        blobs: Default::default(),
    };
    let modifications = TransactionManifestModifications {
        add_access_controller_proofs: Default::default(),
        add_lock_fee: Some((account(2), dec!("10"))),
        add_assertions: Default::default(),
    };

    // Act
    let modified_manifest =
        radix_engine_toolkit_core::functions::manifest::modify(&manifest, modifications).unwrap();

    // Assert
    let expected_instructions = vec![
        InstructionV1::CallMethod {
            address: DynamicGlobalAddress::Static(account(2).into()),
            method_name: ACCOUNT_LOCK_FEE_IDENT.to_owned(),
            args: to_manifest_value_and_unwrap!(&AccountLockFeeInput { amount: dec!("10") }),
        },
        InstructionV1::CallMethod {
            address: DynamicGlobalAddress::Static(account(1).into()),
            method_name: ACCOUNT_WITHDRAW_IDENT.to_owned(),
            args: to_manifest_value_and_unwrap!(&AccountWithdrawInput {
                resource_address: XRD,
                amount: dec!("999")
            }),
        },
        InstructionV1::CallMethod {
            address: DynamicGlobalAddress::Static(FAUCET.into()),
            method_name: "free".to_owned(),
            args: to_manifest_value_and_unwrap!(&()),
        },
        InstructionV1::CallMethod {
            address: DynamicGlobalAddress::Static(account(1).into()),
            method_name: "deposit_batch".to_owned(),
            args: manifest_args!(ManifestExpression::EntireWorktop).into(),
        },
    ];
    assert_eq!(modified_manifest.instructions, expected_instructions);
}

#[test]
fn access_controller_proofs_are_added_as_expected() {
    let manifest = TransactionManifestV1 {
        instructions: vec![
            InstructionV1::CallMethod {
                address: DynamicGlobalAddress::Static(account(1).into()),
                method_name: ACCOUNT_WITHDRAW_IDENT.to_owned(),
                args: to_manifest_value_and_unwrap!(&AccountWithdrawInput {
                    resource_address: XRD,
                    amount: dec!("999")
                }),
            },
            InstructionV1::CallMethod {
                address: DynamicGlobalAddress::Static(FAUCET.into()),
                method_name: "free".to_owned(),
                args: to_manifest_value_and_unwrap!(&()),
            },
            InstructionV1::CallMethod {
                address: DynamicGlobalAddress::Static(account(1).into()),
                method_name: "deposit_batch".to_owned(),
                args: manifest_args!(ManifestExpression::EntireWorktop).into(),
            },
        ],
        blobs: Default::default(),
    };
    let modifications = TransactionManifestModifications {
        add_access_controller_proofs: vec![access_controller(1)],
        add_lock_fee: None,
        add_assertions: Default::default(),
    };

    // Act
    let modified_manifest =
        radix_engine_toolkit_core::functions::manifest::modify(&manifest, modifications).unwrap();

    // Assert
    let expected_instructions = vec![
        InstructionV1::CallMethod {
            address: DynamicGlobalAddress::Static(access_controller(1).into()),
            method_name: ACCESS_CONTROLLER_CREATE_PROOF_IDENT.to_owned(),
            args: to_manifest_value_and_unwrap!(&AccessControllerCreateProofInput {}),
        },
        InstructionV1::CallMethod {
            address: DynamicGlobalAddress::Static(account(1).into()),
            method_name: ACCOUNT_WITHDRAW_IDENT.to_owned(),
            args: to_manifest_value_and_unwrap!(&AccountWithdrawInput {
                resource_address: XRD,
                amount: dec!("999")
            }),
        },
        InstructionV1::CallMethod {
            address: DynamicGlobalAddress::Static(FAUCET.into()),
            method_name: "free".to_owned(),
            args: to_manifest_value_and_unwrap!(&()),
        },
        InstructionV1::CallMethod {
            address: DynamicGlobalAddress::Static(account(1).into()),
            method_name: "deposit_batch".to_owned(),
            args: manifest_args!(ManifestExpression::EntireWorktop).into(),
        },
    ];
    assert_eq!(modified_manifest.instructions, expected_instructions);
}

#[test]
fn access_controller_calls_are_always_at_the_top() {
    let manifest = TransactionManifestV1 {
        instructions: vec![
            InstructionV1::CallMethod {
                address: DynamicGlobalAddress::Static(account(1).into()),
                method_name: ACCOUNT_WITHDRAW_IDENT.to_owned(),
                args: to_manifest_value_and_unwrap!(&AccountWithdrawInput {
                    resource_address: XRD,
                    amount: dec!("999")
                }),
            },
            InstructionV1::CallMethod {
                address: DynamicGlobalAddress::Static(FAUCET.into()),
                method_name: "free".to_owned(),
                args: to_manifest_value_and_unwrap!(&()),
            },
            InstructionV1::CallMethod {
                address: DynamicGlobalAddress::Static(account(1).into()),
                method_name: "deposit_batch".to_owned(),
                args: manifest_args!(ManifestExpression::EntireWorktop).into(),
            },
        ],
        blobs: Default::default(),
    };
    let modifications = TransactionManifestModifications {
        add_access_controller_proofs: vec![access_controller(1)],
        add_lock_fee: Some((account(2), dec!("10"))),
        add_assertions: Default::default(),
    };

    // Act
    let modified_manifest =
        radix_engine_toolkit_core::functions::manifest::modify(&manifest, modifications).unwrap();

    // Assert
    let expected_instructions = vec![
        InstructionV1::CallMethod {
            address: DynamicGlobalAddress::Static(access_controller(1).into()),
            method_name: ACCESS_CONTROLLER_CREATE_PROOF_IDENT.to_owned(),
            args: to_manifest_value_and_unwrap!(&AccessControllerCreateProofInput {}),
        },
        InstructionV1::CallMethod {
            address: DynamicGlobalAddress::Static(account(2).into()),
            method_name: ACCOUNT_LOCK_FEE_IDENT.to_owned(),
            args: to_manifest_value_and_unwrap!(&AccountLockFeeInput { amount: dec!("10") }),
        },
        InstructionV1::CallMethod {
            address: DynamicGlobalAddress::Static(account(1).into()),
            method_name: ACCOUNT_WITHDRAW_IDENT.to_owned(),
            args: to_manifest_value_and_unwrap!(&AccountWithdrawInput {
                resource_address: XRD,
                amount: dec!("999")
            }),
        },
        InstructionV1::CallMethod {
            address: DynamicGlobalAddress::Static(FAUCET.into()),
            method_name: "free".to_owned(),
            args: to_manifest_value_and_unwrap!(&()),
        },
        InstructionV1::CallMethod {
            address: DynamicGlobalAddress::Static(account(1).into()),
            method_name: "deposit_batch".to_owned(),
            args: manifest_args!(ManifestExpression::EntireWorktop).into(),
        },
    ];
    assert_eq!(modified_manifest.instructions, expected_instructions);
}

#[test]
fn assertions_are_added_at_expected_indices_even_with_other_things_need_to_be_added() {
    // Arrange
    let manifest = TransactionManifestV1 {
        instructions: vec![
            InstructionV1::CallMethod {
                address: DynamicGlobalAddress::Static(FAUCET.into()),
                method_name: "free".to_owned(),
                args: to_manifest_value_and_unwrap!(&()),
            },
            InstructionV1::CallMethod {
                address: DynamicGlobalAddress::Static(account(1).into()),
                method_name: "deposit_batch".to_owned(),
                args: manifest_args!(ManifestExpression::EntireWorktop).into(),
            },
            InstructionV1::CallMethod {
                address: DynamicGlobalAddress::Static(FAUCET.into()),
                method_name: "free".to_owned(),
                args: to_manifest_value_and_unwrap!(&()),
            },
            InstructionV1::CallMethod {
                address: DynamicGlobalAddress::Static(account(2).into()),
                method_name: "deposit_batch".to_owned(),
                args: manifest_args!(ManifestExpression::EntireWorktop).into(),
            },
            InstructionV1::CallMethod {
                address: DynamicGlobalAddress::Static(FAUCET.into()),
                method_name: "free".to_owned(),
                args: to_manifest_value_and_unwrap!(&()),
            },
            InstructionV1::CallMethod {
                address: DynamicGlobalAddress::Static(account(3).into()),
                method_name: "deposit_batch".to_owned(),
                args: manifest_args!(ManifestExpression::EntireWorktop).into(),
            },
        ],
        blobs: Default::default(),
    };
    let modifications = TransactionManifestModifications {
        add_access_controller_proofs: vec![access_controller(1)],
        add_lock_fee: Some((account(2), dec!("10"))),
        add_assertions: vec![
            (
                1,
                Assertion::Amount {
                    resource_address: XRD,
                    amount: dec!("100"),
                },
            ),
            (
                3,
                Assertion::Amount {
                    resource_address: XRD,
                    amount: dec!("200"),
                },
            ),
            (
                5,
                Assertion::Amount {
                    resource_address: XRD,
                    amount: dec!("300"),
                },
            ),
        ],
    };

    // Act
    let modified_manifest =
        radix_engine_toolkit_core::functions::manifest::modify(&manifest, modifications).unwrap();

    // Assert
    let expected_instructions = vec![
        InstructionV1::CallMethod {
            address: DynamicGlobalAddress::Static(access_controller(1).into()),
            method_name: ACCESS_CONTROLLER_CREATE_PROOF_IDENT.to_owned(),
            args: to_manifest_value_and_unwrap!(&AccessControllerCreateProofInput {}),
        },
        InstructionV1::CallMethod {
            address: DynamicGlobalAddress::Static(account(2).into()),
            method_name: ACCOUNT_LOCK_FEE_IDENT.to_owned(),
            args: to_manifest_value_and_unwrap!(&AccountLockFeeInput { amount: dec!("10") }),
        },
        InstructionV1::CallMethod {
            address: DynamicGlobalAddress::Static(FAUCET.into()),
            method_name: "free".to_owned(),
            args: to_manifest_value_and_unwrap!(&()),
        },
        InstructionV1::AssertWorktopContains {
            resource_address: XRD,
            amount: dec!("100"),
        },
        InstructionV1::CallMethod {
            address: DynamicGlobalAddress::Static(account(1).into()),
            method_name: "deposit_batch".to_owned(),
            args: manifest_args!(ManifestExpression::EntireWorktop).into(),
        },
        InstructionV1::CallMethod {
            address: DynamicGlobalAddress::Static(FAUCET.into()),
            method_name: "free".to_owned(),
            args: to_manifest_value_and_unwrap!(&()),
        },
        InstructionV1::AssertWorktopContains {
            resource_address: XRD,
            amount: dec!("200"),
        },
        InstructionV1::CallMethod {
            address: DynamicGlobalAddress::Static(account(2).into()),
            method_name: "deposit_batch".to_owned(),
            args: manifest_args!(ManifestExpression::EntireWorktop).into(),
        },
        InstructionV1::CallMethod {
            address: DynamicGlobalAddress::Static(FAUCET.into()),
            method_name: "free".to_owned(),
            args: to_manifest_value_and_unwrap!(&()),
        },
        InstructionV1::AssertWorktopContains {
            resource_address: XRD,
            amount: dec!("300"),
        },
        InstructionV1::CallMethod {
            address: DynamicGlobalAddress::Static(account(3).into()),
            method_name: "deposit_batch".to_owned(),
            args: manifest_args!(ManifestExpression::EntireWorktop).into(),
        },
    ];
    assert_eq!(modified_manifest.instructions, expected_instructions);
}

fn account(id: u64) -> ComponentAddress {
    ComponentAddress::virtual_account_from_public_key(
        &Secp256k1PrivateKey::from_u64(id).unwrap().public_key(),
    )
}

fn access_controller(id: u64) -> ComponentAddress {
    let mut bytes = account(id).as_node_id().0;
    bytes[0] = EntityType::GlobalAccessController as u8;
    ComponentAddress::new_or_panic(bytes)
}
