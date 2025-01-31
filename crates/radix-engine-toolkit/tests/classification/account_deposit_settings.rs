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
fn setting_default_deposit_rule_classifies_as_account_deposit_settings_transaction(
) {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (_, _, account) = ledger.new_account(true);

    let manifest = ManifestBuilder::new()
        .call_method(
            account,
            ACCOUNT_SET_DEFAULT_DEPOSIT_RULE_IDENT,
            AccountSetDefaultDepositRuleManifestInput {
                default: DefaultDepositRule::Reject,
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
    ) = ledger.analyze(manifest);

    // Assert
    assert!(manifest_classification
        .contains(&ManifestClassification::AccountDepositSettingsUpdate));
    let Some(DetailedManifestClassification::AccountDepositSettingsUpdate(
        AccountSettingsUpdateOutput {
            resource_preference_updates,
            default_deposit_rule_updates,
            authorized_depositor_updates,
        },
    )) = detailed_manifest_classification.last()
    else {
        panic!("Not a pool redemption transaction")
    };

    assert!(resource_preference_updates.is_empty());
    assert!(authorized_depositor_updates.is_empty());
    assert_eq!(
        default_deposit_rule_updates,
        &indexmap! {
            ManifestGlobalAddress::Static(account.into()) => DefaultDepositRule::Reject
        }
    )
}

#[test]
fn setting_resource_preference_classifies_as_account_deposit_settings_transaction(
) {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (_, _, account) = ledger.new_account(true);

    let manifest = ManifestBuilder::new()
        .call_method(
            account,
            ACCOUNT_SET_RESOURCE_PREFERENCE_IDENT,
            AccountSetResourcePreferenceManifestInput {
                resource_address: XRD.into(),
                resource_preference: ResourcePreference::Allowed,
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
    ) = ledger.analyze(manifest);

    // Assert
    assert!(manifest_classification
        .contains(&ManifestClassification::AccountDepositSettingsUpdate));
    let Some(DetailedManifestClassification::AccountDepositSettingsUpdate(
        AccountSettingsUpdateOutput {
            resource_preference_updates,
            default_deposit_rule_updates,
            authorized_depositor_updates,
        },
    )) = detailed_manifest_classification.last()
    else {
        panic!("Not a pool redemption transaction")
    };

    assert!(default_deposit_rule_updates.is_empty());
    assert!(authorized_depositor_updates.is_empty());
    assert_eq!(
        resource_preference_updates,
        &indexmap! {
            (
                ManifestGlobalAddress::Static(account.into()),
                ManifestResourceAddress::Static(XRD),
            ) => Update::Set(ResourcePreference::Allowed)
        }
    )
}

#[test]
fn removing_resource_preference_classifies_as_account_deposit_settings_transaction(
) {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (_, _, account) = ledger.new_account(true);

    let manifest = ManifestBuilder::new()
        .call_method(
            account,
            ACCOUNT_REMOVE_RESOURCE_PREFERENCE_IDENT,
            AccountRemoveResourcePreferenceManifestInput {
                resource_address: XRD.into(),
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
    ) = ledger.analyze(manifest);

    // Assert
    assert!(manifest_classification
        .contains(&ManifestClassification::AccountDepositSettingsUpdate));
    let Some(DetailedManifestClassification::AccountDepositSettingsUpdate(
        AccountSettingsUpdateOutput {
            resource_preference_updates,
            default_deposit_rule_updates,
            authorized_depositor_updates,
        },
    )) = detailed_manifest_classification.last()
    else {
        panic!("Not a pool redemption transaction")
    };

    assert!(default_deposit_rule_updates.is_empty());
    assert!(authorized_depositor_updates.is_empty());
    assert_eq!(
        resource_preference_updates,
        &indexmap! {
            (
                ManifestGlobalAddress::Static(account.into()),
                ManifestResourceAddress::Static(XRD),
            ) => Update::Remove
        }
    )
}

#[test]
fn adding_authorized_depositor_classifies_as_account_deposit_settings_transaction(
) {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (_, _, account) = ledger.new_account(true);

    let manifest = ManifestBuilder::new()
        .call_method(
            account,
            ACCOUNT_ADD_AUTHORIZED_DEPOSITOR_IDENT,
            AccountAddAuthorizedDepositorManifestInput {
                badge: ManifestResourceOrNonFungible::Resource(XRD.into()),
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
    ) = ledger.analyze(manifest);

    // Assert
    assert!(manifest_classification
        .contains(&ManifestClassification::AccountDepositSettingsUpdate));
    let Some(DetailedManifestClassification::AccountDepositSettingsUpdate(
        AccountSettingsUpdateOutput {
            resource_preference_updates,
            default_deposit_rule_updates,
            authorized_depositor_updates,
        },
    )) = detailed_manifest_classification.last()
    else {
        panic!("Not a pool redemption transaction")
    };

    assert!(default_deposit_rule_updates.is_empty());
    assert!(resource_preference_updates.is_empty());
    assert_eq!(
        authorized_depositor_updates,
        &indexmap! {
            (
                ManifestGlobalAddress::Static(account.into()),
                ManifestResourceOrNonFungible::Resource(XRD.into()),
            ) => Operation::Added
        }
    )
}

#[test]
fn removing_authorized_depositor_classifies_as_account_deposit_settings_transaction(
) {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (_, _, account) = ledger.new_account(true);

    let manifest = ManifestBuilder::new()
        .call_method(
            account,
            ACCOUNT_REMOVE_AUTHORIZED_DEPOSITOR_IDENT,
            AccountRemoveAuthorizedDepositorManifestInput {
                badge: ManifestResourceOrNonFungible::Resource(XRD.into()),
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
    ) = ledger.analyze(manifest);

    // Assert
    assert!(manifest_classification
        .contains(&ManifestClassification::AccountDepositSettingsUpdate));
    let Some(DetailedManifestClassification::AccountDepositSettingsUpdate(
        AccountSettingsUpdateOutput {
            resource_preference_updates,
            default_deposit_rule_updates,
            authorized_depositor_updates,
        },
    )) = detailed_manifest_classification.last()
    else {
        panic!("Not a pool redemption transaction")
    };

    assert!(default_deposit_rule_updates.is_empty());
    assert!(resource_preference_updates.is_empty());
    assert_eq!(
        authorized_depositor_updates,
        &indexmap! {
            (
                ManifestGlobalAddress::Static(account.into()),
                ManifestResourceOrNonFungible::Resource(XRD.into()),
            ) => Operation::Removed
        }
    )
}
