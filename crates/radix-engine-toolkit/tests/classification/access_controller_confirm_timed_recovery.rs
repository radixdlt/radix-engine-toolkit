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
fn confirm_timed_recovery_manifest_classified_as_confirm_timed_recovery() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();

    let (pk, _, account) = ledger.new_account(true);
    let access_controller =
        ledger.new_allow_all_access_controller_for_account((pk, account));

    let iniate_timed_recovery_manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .call_method(
            access_controller,
            ACCESS_CONTROLLER_INITIATE_RECOVERY_AS_RECOVERY_IDENT,
            AccessControllerInitiateRecoveryAsPrimaryManifestInput {
                rule_set: RuleSet {
                    primary_role: rule!(allow_all),
                    recovery_role: rule!(allow_all),
                    confirmation_role: rule!(allow_all),
                },
                timed_recovery_delay_in_minutes: Some(10),
            },
        )
        .build();

    ledger
        .execute_manifest(
            iniate_timed_recovery_manifest,
            vec![NonFungibleGlobalId::from_public_key(&pk)],
        )
        .expect_commit_success();

    // Advance time by 10 minutes (+1 ms) by moving to the next consensus round
    let now_ms = ledger.get_current_proposer_timestamp_ms();
    let current_round = ledger.get_consensus_manager_state().round;
    let target_ts = now_ms + (10 * 60_000) + 1;
    let next_round =
        radix_engine_interface::prelude::Round::of(current_round.number() + 1);
    ledger.advance_to_round_at_timestamp(next_round, target_ts);

    let confirm_timed_recovery_manifest = ManifestBuilder::new()
        .lock_fee(account, 10)
        .call_method(
            access_controller,
            ACCESS_CONTROLLER_TIMED_CONFIRM_RECOVERY_IDENT,
            AccessControllerTimedConfirmRecoveryManifestInput {
                rule_set: RuleSet {
                    primary_role: rule!(allow_all),
                    recovery_role: rule!(allow_all),
                    confirmation_role: rule!(allow_all),
                },
                timed_recovery_delay_in_minutes: Some(10),
            },
        )
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
    ) = ledger.analyze(confirm_timed_recovery_manifest);

    assert!(manifest_classification.contains(
        &ManifestClassification::AccessControllerConfirmTimedRecovery
    ));

    let Some(
        DetailedManifestClassification::AccessControllerConfirmTimedRecovery(
            AccessControllerConfirmTimedRecoveryOutput { access_controllers },
        ),
    ) = detailed_manifest_classification.last()
    else {
        panic!("Not an access controller recovery transaction")
    };

    assert_eq!(access_controllers.clone(), vec![access_controller]);
}
