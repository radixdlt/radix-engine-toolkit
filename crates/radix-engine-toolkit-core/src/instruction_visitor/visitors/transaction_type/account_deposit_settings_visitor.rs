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
use scrypto::prelude::*;
use transaction::prelude::{DynamicGlobalAddress, InstructionV1};

use crate::instruction_visitor::core::traits::InstructionVisitor;
use crate::utils::{is_account, to_manifest_type};

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub enum ResourcePreferenceAction {
    Set(ResourcePreference),
    Remove,
}

#[derive(Clone, Debug, Default)]
pub struct AccountDepositSettingsVisitor {
    /// Maps the account address to the changes in resource preferences
    /// encountered in the transaction.
    resource_preference_changes: HashMap<
        ComponentAddress,
        HashMap<ResourceAddress, ResourcePreferenceAction>,
    >,
    /// Maps the account address to the updated default deposit rule
    /// encountered in the transaction.
    default_deposit_rule_changes: HashMap<ComponentAddress, DefaultDepositRule>,
    /// Maps the account address to the changes in the authorized depositors in
    /// the transaction.
    authorized_depositors_changes:
        HashMap<ComponentAddress, AuthorizedDepositorsChanges>,
    /// Tracks if the visitor is currently in an illegal state or not.
    is_illegal_state: bool,
}

#[allow(clippy::type_complexity)]
impl AccountDepositSettingsVisitor {
    pub fn output(
        self,
    ) -> Option<(
        HashMap<
            ComponentAddress,
            HashMap<ResourceAddress, ResourcePreferenceAction>,
        >,
        HashMap<ComponentAddress, DefaultDepositRule>,
        HashMap<ComponentAddress, AuthorizedDepositorsChanges>,
    )> {
        if !self.is_illegal_state {
            Some((
                self.resource_preference_changes,
                self.default_deposit_rule_changes,
                self.authorized_depositors_changes,
            ))
        } else {
            None
        }
    }
}

impl InstructionVisitor for AccountDepositSettingsVisitor {
    fn is_enabled(&self) -> bool {
        !self.is_illegal_state
    }

    fn visit_instruction(
        &mut self,
        instruction: &transaction::prelude::InstructionV1,
    ) -> Result<
        (),
        crate::instruction_visitor::core::error::InstructionVisitorError,
    > {
        match instruction {
            InstructionV1::CallMethod {
                address,
                method_name,
                args,
            } => {
                // Filter: We only permit static address - no dynamic or named
                // addresses are allowed
                let global_address =
                    if let DynamicGlobalAddress::Static(address) = address {
                        address
                    } else {
                        self.is_illegal_state = true;
                        return Ok(());
                    };

                // Filter: Any method call to something that is not an account
                // is not permitted.
                if !is_account(global_address) {
                    self.is_illegal_state = true;
                    return Ok(());
                }
                // This never panics. We have already checked that this is an
                // account when we called `is_account`.
                let component_address = ComponentAddress::new_or_panic(
                    global_address.as_node_id().0,
                );

                // Process the calls to the appropriate account methods, any
                // other method that is encountered is not
                // permitted
                match method_name.as_str() {
                    ACCOUNT_SET_RESOURCE_PREFERENCE_IDENT => {
                        if let Some(AccountSetResourcePreferenceInput {
                            resource_address,
                            resource_preference,
                        }) = to_manifest_type(args)
                        {
                            self.resource_preference_changes
                                .entry(component_address)
                                .or_default()
                                .insert(
                                    resource_address,
                                    ResourcePreferenceAction::Set(
                                        resource_preference,
                                    ),
                                );
                        }
                    }
                    ACCOUNT_REMOVE_RESOURCE_PREFERENCE_IDENT => {
                        if let Some(AccountRemoveResourcePreferenceInput {
                            resource_address,
                        }) = to_manifest_type(args)
                        {
                            self.resource_preference_changes
                                .entry(component_address)
                                .or_default()
                                .insert(
                                    resource_address,
                                    ResourcePreferenceAction::Remove,
                                );
                        }
                    }
                    ACCOUNT_SET_DEFAULT_DEPOSIT_RULE_IDENT => {
                        if let Some(AccountSetDefaultDepositRuleInput {
                            default,
                        }) = to_manifest_type(args)
                        {
                            self.default_deposit_rule_changes
                                .insert(component_address, default);
                        }
                    }
                    ACCOUNT_ADD_AUTHORIZED_DEPOSITOR => {
                        if let Some(AccountAddAuthorizedDepositorInput {
                            badge,
                        }) = to_manifest_type(args)
                        {
                            self.authorized_depositors_changes
                                .entry(component_address)
                                .or_default()
                                .added
                                .push(badge)
                        }
                    }
                    ACCOUNT_REMOVE_AUTHORIZED_DEPOSITOR => {
                        if let Some(AccountRemoveAuthorizedDepositorInput {
                            badge,
                        }) = to_manifest_type(args)
                        {
                            self.authorized_depositors_changes
                                .entry(component_address)
                                .or_default()
                                .removed
                                .push(badge)
                        }
                    }
                    _ => {
                        self.is_illegal_state = true;
                        return Ok(());
                    }
                };
                Ok(())
            }
            InstructionV1::AssertWorktopContains { .. }
            | InstructionV1::AssertWorktopContainsAny { .. }
            | InstructionV1::AssertWorktopContainsNonFungibles { .. }
            | InstructionV1::PopFromAuthZone
            | InstructionV1::PushToAuthZone { .. }
            | InstructionV1::DropNamedProofs
            | InstructionV1::DropAuthZoneProofs
            | InstructionV1::DropAuthZoneRegularProofs
            | InstructionV1::DropAuthZoneSignatureProofs
            | InstructionV1::CreateProofFromAuthZoneOfAmount { .. }
            | InstructionV1::CreateProofFromAuthZoneOfNonFungibles { .. }
            | InstructionV1::CreateProofFromAuthZoneOfAll { .. }
            | InstructionV1::CreateProofFromBucketOfAmount { .. }
            | InstructionV1::CreateProofFromBucketOfNonFungibles { .. }
            | InstructionV1::CreateProofFromBucketOfAll { .. }
            | InstructionV1::CloneProof { .. }
            | InstructionV1::DropProof { .. }
            | InstructionV1::DropAllProofs
            | InstructionV1::AllocateGlobalAddress { .. }
            | InstructionV1::TakeAllFromWorktop { .. }
            | InstructionV1::TakeFromWorktop { .. }
            | InstructionV1::TakeNonFungiblesFromWorktop { .. }
            | InstructionV1::ReturnToWorktop { .. }
            | InstructionV1::BurnResource { .. }
            | InstructionV1::CallFunction { .. }
            | InstructionV1::CallRoyaltyMethod { .. }
            | InstructionV1::CallMetadataMethod { .. }
            | InstructionV1::CallRoleAssignmentMethod { .. }
            | InstructionV1::CallDirectVaultMethod { .. } => {
                self.is_illegal_state = true;
                Ok(())
            }
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AuthorizedDepositorsChanges {
    /// Badges that have been added as authorized depositors.
    pub added: Vec<ResourceOrNonFungible>,
    /// Badges that have been removed from the authorized depositors.
    pub removed: Vec<ResourceOrNonFungible>,
}
