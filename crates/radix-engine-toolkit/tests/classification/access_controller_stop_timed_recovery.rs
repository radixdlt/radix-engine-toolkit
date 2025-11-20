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
fn stop_timed_recovery_manifest_classified_as_stop_timed_recovery() {
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
                timed_recovery_delay_in_minutes: None,
            },
        )
        .build();

    ledger
        .execute_manifest(
            iniate_timed_recovery_manifest,
            vec![NonFungibleGlobalId::from_public_key(&pk)],
        )
        .expect_commit_success();

    let stop_timed_recovery_manifest = ManifestBuilder::new()
        .lock_fee(account, 10)
        .call_method(
            access_controller,
            ACCESS_CONTROLLER_STOP_TIMED_RECOVERY_IDENT,
            AccessControllerStopTimedRecoveryManifestInput {
                rule_set: RuleSet {
                    primary_role: rule!(allow_all),
                    recovery_role: rule!(allow_all),
                    confirmation_role: rule!(allow_all),
                },
                timed_recovery_delay_in_minutes: None,
            },
        )
        .call_method(
            access_controller,
            ACCESS_CONTROLLER_CANCEL_RECOVERY_ROLE_RECOVERY_PROPOSAL_IDENT,
            AccessControllerCancelRecoveryRoleRecoveryProposalManifestInput {},
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
    ) = ledger.analyze(stop_timed_recovery_manifest);

    assert!(manifest_classification
        .contains(&ManifestClassification::AccessControllerStopTimedRecovery));

    let Some(
        DetailedManifestClassification::AccessControllerStopTimedRecovery(
            AccessControllerStopTimedRecoveryAnalyzerOutput {
                access_controllers,
            },
        ),
    ) = detailed_manifest_classification.last()
    else {
        panic!("Not an access controller recovery transaction")
    };

    assert_eq!(access_controllers.clone(), vec![access_controller]);
}
