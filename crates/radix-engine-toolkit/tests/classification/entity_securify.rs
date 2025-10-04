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
fn securify_account_classifies_as_entity_securify() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (_, _, account) = ledger.new_account(true);

    let builder = ManifestBuilder::new()
        .call_method(
            account,
            ACCOUNT_SECURIFY_IDENT,
            AccountSecurifyManifestInput {},
        )
        .take_from_worktop(ACCOUNT_OWNER_BADGE, 1, "bucket")
        .create_proof_from_bucket_of_all("bucket", "proof")
        .push_to_auth_zone("proof")
        .set_metadata(account, "owner_keys", "not important")
        .drop_auth_zone_proofs()
        .allocate_global_address(
            ACCESS_CONTROLLER_PACKAGE,
            ACCESS_CONTROLLER_BLUEPRINT,
            "ac_reservation",
            "ac_address",
        );

    let reservation = builder.address_reservation("ac_reservation");
    let bucket = builder.bucket("bucket");

    let builder = builder.call_function(
        ACCESS_CONTROLLER_PACKAGE,
        ACCESS_CONTROLLER_BLUEPRINT,
        ACCESS_CONTROLLER_CREATE_IDENT,
        AccessControllerCreateManifestInput {
            controlled_asset: bucket,
            rule_set: RuleSet {
                primary_role: rule!(allow_all),
                recovery_role: rule!(allow_all),
                confirmation_role: rule!(allow_all),
            },
            timed_recovery_delay_in_minutes: None,
            address_reservation: Some(reservation),
        },
    );

    let manifest = builder.build();

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
    ) = ledger.analyze(manifest);

    assert!(manifest_classification
        .contains(&ManifestClassification::EntitySecurify));

    let Some(DetailedManifestClassification::EntitySecurify(
        EntitySecurifyOutput {
            securified_accounts,
            securified_identities,
        },
    )) = detailed_manifest_classification.last()
    else {
        panic!("Not an entity securify transaction")
    };

    assert!(securified_identities.is_empty());
    assert_eq!(
        securified_accounts.clone(),
        vec![ManifestGlobalAddress::Static(account.into())]
    );
}

#[test]
fn securify_identity_classifies_as_entity_securify() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (pk, _, _) = ledger.new_account(true);
    let identity = ledger.new_identity(pk, true);

    let builder = ManifestBuilder::new()
        .call_method(
            identity,
            IDENTITY_SECURIFY_IDENT,
            (),
        )
        .take_from_worktop(IDENTITY_OWNER_BADGE, 1, "bucket")
        .create_proof_from_bucket_of_all("bucket", "proof")
        .push_to_auth_zone("proof")
        .set_metadata(identity, "owner_keys", "not important")
        .drop_auth_zone_proofs()
        .allocate_global_address(
            ACCESS_CONTROLLER_PACKAGE,
            ACCESS_CONTROLLER_BLUEPRINT,
            "ac_reservation",
            "ac_address",
        );

    let reservation = builder.address_reservation("ac_reservation");
    let bucket = builder.bucket("bucket");

    let builder = builder.call_function(
        ACCESS_CONTROLLER_PACKAGE,
        ACCESS_CONTROLLER_BLUEPRINT,
        ACCESS_CONTROLLER_CREATE_IDENT,
        AccessControllerCreateManifestInput {
            controlled_asset: bucket,
            rule_set: RuleSet {
                primary_role: rule!(allow_all),
                recovery_role: rule!(allow_all),
                confirmation_role: rule!(allow_all),
            },
            timed_recovery_delay_in_minutes: None,
            address_reservation: Some(reservation),
        },
    );

    let manifest = builder.build();

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
    ) = ledger.analyze(manifest);

    assert!(manifest_classification
        .contains(&ManifestClassification::EntitySecurify));

    let Some(DetailedManifestClassification::EntitySecurify(
        EntitySecurifyOutput {
            securified_accounts,
            securified_identities,
        },
    )) = detailed_manifest_classification.last()
    else {
        panic!("Not an entity securify transaction")
    };

    assert!(securified_accounts.is_empty());
    assert_eq!(
        securified_identities.clone(),
        vec![ManifestGlobalAddress::Static(identity.into())]
    );
}
