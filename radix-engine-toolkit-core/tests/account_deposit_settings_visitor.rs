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

use radix_engine_interface::blueprints::account::*;
use radix_engine_toolkit_core::instruction_visitor::core::traverser::*;
use radix_engine_toolkit_core::instruction_visitor::visitors::transaction_type::account_deposit_settings_visitor::*;
use scrypto::prelude::*;
use transaction::prelude::{ManifestBuilder, Secp256k1PrivateKey};

#[test]
fn account_deposit_settings_visitor_functions_as_expected() {
    // Arrange
    let account1 = account_from_u64_private_key(1);
    let manifest = ManifestBuilder::new()
        .call_method(
            account1,
            ACCOUNT_ADD_AUTHORIZED_DEPOSITOR,
            AccountAddAuthorizedDepositorInput {
                badge: ResourceOrNonFungible::Resource(XRD),
            },
        )
        .call_method(
            account1,
            ACCOUNT_REMOVE_AUTHORIZED_DEPOSITOR,
            AccountRemoveAuthorizedDepositorInput {
                badge: ResourceOrNonFungible::Resource(ACCOUNT_OWNER_BADGE),
            },
        )
        .call_method(
            account1,
            ACCOUNT_CONFIGURE_RESOURCE_DEPOSIT_RULE_IDENT,
            AccountConfigureResourceDepositRuleInput {
                resource_address: XRD,
                resource_deposit_configuration: ResourceDepositRule::Allowed,
            },
        )
        .call_method(
            account1,
            ACCOUNT_CONFIGURE_RESOURCE_DEPOSIT_RULE_IDENT,
            AccountConfigureResourceDepositRuleInput {
                resource_address: XRD,
                resource_deposit_configuration: ResourceDepositRule::Disallowed,
            },
        )
        .call_method(
            account1,
            ACCOUNT_CONFIGURE_RESOURCE_DEPOSIT_RULE_IDENT,
            AccountConfigureResourceDepositRuleInput {
                resource_address: XRD,
                resource_deposit_configuration: ResourceDepositRule::Neither,
            },
        )
        .call_method(
            account1,
            ACCOUNT_CHANGE_DEFAULT_DEPOSIT_RULE_IDENT,
            AccountChangeDefaultDepositRuleInput {
                default_deposit_rule: AccountDefaultDepositRule::Accept,
            },
        )
        .call_method(
            account1,
            ACCOUNT_CHANGE_DEFAULT_DEPOSIT_RULE_IDENT,
            AccountChangeDefaultDepositRuleInput {
                default_deposit_rule: AccountDefaultDepositRule::Reject,
            },
        )
        .build();

    // Act
    let mut visitor = AccountDepositSettingsVisitor::default();
    traverse(&manifest.instructions, &mut [&mut visitor]).unwrap();

    // Assert
    let (resource_preference_changes, default_deposit_rule_changes, authorized_depositors_changes) =
        visitor.output().unwrap();
    assert_eq!(
        authorized_depositors_changes,
        hashmap! {
            account1 => AuthorizedDepositorsChanges {
                added: vec![
                    ResourceOrNonFungible::Resource(XRD)
                ],
                removed: vec![
                    ResourceOrNonFungible::Resource(ACCOUNT_OWNER_BADGE)
                ]
            },
        }
    );
    assert_eq!(
        resource_preference_changes,
        hashmap! {
            account1 => ResourcePreferencesChanges {
                allowed: vec![XRD],
                disallowed: vec![XRD],
                preference_removed: vec![XRD]
            },
        }
    );
    assert_eq!(
        default_deposit_rule_changes,
        hashmap! {
            account1 => AccountDefaultDepositRule::Reject
        }
    )
}

fn account_from_u64_private_key(private_key: u64) -> ComponentAddress {
    ComponentAddress::virtual_account_from_public_key(
        &Secp256k1PrivateKey::from_u64(private_key)
            .unwrap()
            .public_key(),
    )
}
