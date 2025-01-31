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
fn regular_transfer_has_no_reserved_instructions() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (_, _, account1) = ledger.new_account(true);
    let (_, _, account2) = ledger.new_account(true);

    let manifest = ManifestBuilder::new()
        .withdraw_from_account(account1, XRD, 10)
        .try_deposit_entire_worktop_or_abort(account2, None)
        .build();

    // Act
    let (static_analysis, dynamic_analysis) = ledger.analyze(manifest);

    // Assert
    assert_eq!(
        static_analysis.reserved_instructions_summary,
        dynamic_analysis.reserved_instructions_summary
    );
    assert!(!static_analysis
        .reserved_instructions_summary
        .is_any_reserved_instruction_present());
}

#[test]
fn generic_component_calls_has_no_reserved_instructions() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (_, _, account) = ledger.new_account(true);

    let manifest = ManifestBuilder::new()
        .get_free_xrd_from_faucet()
        .try_deposit_entire_worktop_or_abort(account, None)
        .build();

    // Act
    let (static_analysis, dynamic_analysis) = ledger.analyze(manifest);

    // Assert
    assert_eq!(
        static_analysis.reserved_instructions_summary,
        dynamic_analysis.reserved_instructions_summary
    );
    assert!(!static_analysis
        .reserved_instructions_summary
        .is_any_reserved_instruction_present());
}

#[test]
fn securifying_account_is_a_reserved_instruction() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (_, _, account) = ledger.new_account(true);

    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .call_method(
            account,
            ACCOUNT_SECURIFY_IDENT,
            AccountSecurifyManifestInput {},
        )
        .try_deposit_entire_worktop_or_abort(account, None)
        .build();

    // Act
    let (static_analysis, dynamic_analysis) = ledger.analyze(manifest);

    // Assert
    assert_eq!(
        static_analysis.reserved_instructions_summary,
        dynamic_analysis.reserved_instructions_summary
    );
    assert!(static_analysis
        .reserved_instructions_summary
        .has_account_securify_invocations());
}

#[test]
fn locking_fee_from_account_is_a_reserved_instruction() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (_, _, account) = ledger.new_account(true);

    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .lock_fee(account, 10)
        .build();

    // Act
    let (static_analysis, dynamic_analysis) = ledger.analyze(manifest);

    // Assert
    assert_eq!(
        static_analysis.reserved_instructions_summary,
        dynamic_analysis.reserved_instructions_summary
    );
    assert!(static_analysis
        .reserved_instructions_summary
        .has_account_lock_fee_invocations());
}

#[test]
fn locking_contingent_fee_from_account_is_a_reserved_instruction() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (_, _, account) = ledger.new_account(true);

    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .lock_contingent_fee(account, 10)
        .build();

    // Act
    let (static_analysis, dynamic_analysis) = ledger.analyze(manifest);

    // Assert
    assert_eq!(
        static_analysis.reserved_instructions_summary,
        dynamic_analysis.reserved_instructions_summary
    );
    assert!(static_analysis
        .reserved_instructions_summary
        .has_account_lock_fee_invocations());
}

#[test]
fn locking_fee_and_withdrawing_from_account_is_a_reserved_instruction() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (_, _, account) = ledger.new_account(true);

    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .lock_fee_and_withdraw(account, 10, XRD, 10)
        .try_deposit_entire_worktop_or_abort(account, None)
        .build();

    // Act
    let (static_analysis, dynamic_analysis) = ledger.analyze(manifest);

    // Assert
    assert_eq!(
        static_analysis.reserved_instructions_summary,
        dynamic_analysis.reserved_instructions_summary
    );
    assert!(static_analysis
        .reserved_instructions_summary
        .has_account_lock_fee_invocations());
}

#[test]
fn locking_fee_and_withdrawing_non_fungibles_from_account_is_a_reserved_instruction(
) {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (_, _, account) = ledger.new_account(true);
    let resource_address = ledger.create_non_fungible_resource(account);

    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .lock_fee_and_withdraw_non_fungibles(account, 10, resource_address, [])
        .try_deposit_entire_worktop_or_abort(account, None)
        .build();

    // Act
    let (static_analysis, dynamic_analysis) = ledger.analyze(manifest);

    // Assert
    assert_eq!(
        static_analysis.reserved_instructions_summary,
        dynamic_analysis.reserved_instructions_summary
    );
    assert!(static_analysis
        .reserved_instructions_summary
        .has_account_lock_fee_invocations());
}

#[test]
fn locking_the_owner_keys_metadata_field_of_account_is_a_reserved_instruction()
{
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (_, _, account) = ledger.new_account(true);

    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .lock_metadata(account, "owner_keys")
        .build();

    // Act
    let (static_analysis, dynamic_analysis) = ledger.analyze(manifest);

    // Assert
    assert_eq!(
        static_analysis.reserved_instructions_summary,
        dynamic_analysis.reserved_instructions_summary
    );
    assert!(static_analysis
        .reserved_instructions_summary
        .has_account_lock_owner_keys_metadata_field_invocations());
}

#[test]
fn locking_non_owner_keys_metadata_field_of_account_is_not_a_reserved_instruction(
) {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (_, _, account) = ledger.new_account(true);

    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .lock_metadata(account, "some_field")
        .build();

    // Act
    let (static_analysis, dynamic_analysis) = ledger.analyze(manifest);

    // Assert
    assert_eq!(
        static_analysis.reserved_instructions_summary,
        dynamic_analysis.reserved_instructions_summary
    );
    assert!(!static_analysis
        .reserved_instructions_summary
        .has_account_lock_owner_keys_metadata_field_invocations());
}

#[test]
fn setting_the_owner_keys_metadata_field_of_account_is_a_reserved_instruction()
{
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (_, _, account) = ledger.new_account(true);

    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .set_metadata(account, "owner_keys", "not important")
        .build();

    // Act
    let (static_analysis, dynamic_analysis) = ledger.analyze(manifest);

    // Assert
    assert_eq!(
        static_analysis.reserved_instructions_summary,
        dynamic_analysis.reserved_instructions_summary
    );
    assert!(static_analysis
        .reserved_instructions_summary
        .has_account_update_owner_keys_metadata_field_invocations());
}

#[test]
fn setting_non_owner_keys_metadata_field_of_account_is_not_a_reserved_instruction(
) {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (_, _, account) = ledger.new_account(true);

    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .set_metadata(account, "some_field", "not important")
        .build();

    // Act
    let (static_analysis, dynamic_analysis) = ledger.analyze(manifest);

    // Assert
    assert_eq!(
        static_analysis.reserved_instructions_summary,
        dynamic_analysis.reserved_instructions_summary
    );
    assert!(!static_analysis
        .reserved_instructions_summary
        .has_account_lock_owner_keys_metadata_field_invocations());
}

#[test]
fn securifying_identity_is_a_reserved_instruction() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (pk, _, account) = ledger.new_account(true);
    let identity = ledger.new_identity(pk, true);

    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .call_method(
            identity,
            IDENTITY_SECURIFY_IDENT,
            IdentitySecurifyToSingleBadgeManifestInput {},
        )
        .try_deposit_entire_worktop_or_abort(account, None)
        .build();

    // Act
    let (static_analysis, dynamic_analysis) = ledger.analyze(manifest);

    // Assert
    assert_eq!(
        static_analysis.reserved_instructions_summary,
        dynamic_analysis.reserved_instructions_summary
    );
    assert!(static_analysis
        .reserved_instructions_summary
        .has_identity_securify_invocations());
}

#[test]
fn locking_the_owner_keys_metadata_field_of_identity_is_a_reserved_instruction()
{
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (pk, _) = ledger.new_key_pair();
    let identity = ledger.new_identity(pk, true);

    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .lock_metadata(identity, "owner_keys")
        .build();

    // Act
    let (static_analysis, dynamic_analysis) = ledger.analyze(manifest);

    // Assert
    assert_eq!(
        static_analysis.reserved_instructions_summary,
        dynamic_analysis.reserved_instructions_summary
    );
    assert!(static_analysis
        .reserved_instructions_summary
        .has_identity_lock_owner_keys_metadata_field_invocations());
}

#[test]
fn locking_non_owner_keys_metadata_field_of_identity_is_not_a_reserved_instruction(
) {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (pk, _) = ledger.new_key_pair();
    let identity = ledger.new_identity(pk, true);

    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .lock_metadata(identity, "some_field")
        .build();

    // Act
    let (static_analysis, dynamic_analysis) = ledger.analyze(manifest);

    // Assert
    assert_eq!(
        static_analysis.reserved_instructions_summary,
        dynamic_analysis.reserved_instructions_summary
    );
    assert!(!static_analysis
        .reserved_instructions_summary
        .has_identity_lock_owner_keys_metadata_field_invocations());
}

#[test]
fn setting_the_owner_keys_metadata_field_of_identity_is_a_reserved_instruction()
{
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (pk, _) = ledger.new_key_pair();
    let identity = ledger.new_identity(pk, true);

    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .set_metadata(identity, "owner_keys", "not important")
        .build();

    // Act
    let (static_analysis, dynamic_analysis) = ledger.analyze(manifest);

    // Assert
    assert_eq!(
        static_analysis.reserved_instructions_summary,
        dynamic_analysis.reserved_instructions_summary
    );
    assert!(static_analysis
        .reserved_instructions_summary
        .has_identity_update_owner_keys_metadata_field_invocations());
}

#[test]
fn setting_non_owner_keys_metadata_field_of_identity_is_not_a_reserved_instruction(
) {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (pk, _) = ledger.new_key_pair();
    let identity = ledger.new_identity(pk, true);

    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .set_metadata(identity, "some_field", "not important")
        .build();

    // Act
    let (static_analysis, dynamic_analysis) = ledger.analyze(manifest);

    // Assert
    assert_eq!(
        static_analysis.reserved_instructions_summary,
        dynamic_analysis.reserved_instructions_summary
    );
    assert!(!static_analysis
        .reserved_instructions_summary
        .has_identity_lock_owner_keys_metadata_field_invocations());
}

/// We're testing that the `create_proof` method is not special cased and that
/// it's considered a reserved instruction.
#[test]
fn creating_access_controller_proof_is_a_reserved_instruction() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (_, access_controller) = ledger.new_allow_all_access_controller();

    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .call_method(
            access_controller,
            ACCESS_CONTROLLER_CREATE_PROOF_IDENT,
            AccessControllerCreateProofManifestInput {},
        )
        .build();

    // Act
    let (static_analysis, dynamic_analysis) = ledger.analyze(manifest);

    // Assert
    assert_eq!(
        static_analysis.reserved_instructions_summary,
        dynamic_analysis.reserved_instructions_summary
    );
    assert!(static_analysis
        .reserved_instructions_summary
        .has_access_controller_invocations());
}

/// We're testing that methods that are not the `create_proof` method are
/// also classified as reserved instructions.
#[test]
fn locking_primary_role_on_access_controller_is_a_reserved_instruction() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (_, access_controller) = ledger.new_allow_all_access_controller();

    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .call_method(
            access_controller,
            ACCESS_CONTROLLER_LOCK_PRIMARY_ROLE_IDENT,
            AccessControllerLockPrimaryRoleManifestInput {},
        )
        .build();

    // Act
    let (static_analysis, dynamic_analysis) = ledger.analyze(manifest);

    // Assert
    assert_eq!(
        static_analysis.reserved_instructions_summary,
        dynamic_analysis.reserved_instructions_summary
    );
    assert!(static_analysis
        .reserved_instructions_summary
        .has_access_controller_invocations());
}

#[test]
fn account_function_invocations_are_not_reserved_instructions() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();

    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .create_account_with_owner(None, Default::default())
        .build();

    // Act
    let (static_analysis, dynamic_analysis) = ledger.analyze(manifest);

    // Assert
    assert_eq!(
        static_analysis.reserved_instructions_summary,
        dynamic_analysis.reserved_instructions_summary
    );
    assert!(!static_analysis
        .reserved_instructions_summary
        .is_any_reserved_instruction_present());
}

#[test]
fn identity_function_invocations_are_not_reserved_instructions() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();

    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .create_identity_advanced(Default::default())
        .build();

    // Act
    let (static_analysis, dynamic_analysis) = ledger.analyze(manifest);

    // Assert
    assert_eq!(
        static_analysis.reserved_instructions_summary,
        dynamic_analysis.reserved_instructions_summary
    );
    assert!(!static_analysis
        .reserved_instructions_summary
        .is_any_reserved_instruction_present());
}

#[test]
fn access_controller_function_invocations_are_not_reserved_instructions() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();

    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .take_all_from_worktop(XRD, "bucket")
        .create_access_controller(
            "bucket",
            rule!(allow_all),
            rule!(allow_all),
            rule!(allow_all),
            None,
        )
        .build();

    // Act
    let (static_analysis, dynamic_analysis) = ledger.analyze(manifest);

    // Assert
    assert_eq!(
        static_analysis.reserved_instructions_summary,
        dynamic_analysis.reserved_instructions_summary
    );
    assert!(!static_analysis
        .reserved_instructions_summary
        .is_any_reserved_instruction_present());
}
