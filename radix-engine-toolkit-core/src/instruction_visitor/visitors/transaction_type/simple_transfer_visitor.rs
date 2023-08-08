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

use crate::instruction_visitor::core::error::InstructionVisitorError;
use crate::instruction_visitor::core::traits::InstructionVisitor;
use crate::utils::*;
use radix_engine::system::system_modules::execution_trace::ResourceSpecifier;
use scrypto::blueprints::account::*;
use scrypto::prelude::*;
use transaction::prelude::{DynamicGlobalAddress, DynamicPackageAddress};

#[derive(Default, Debug, Clone)]
pub struct SimpleTransactionTypeVisitor {
    illegal_instruction_encountered: bool,
    state_machine: StateMachine,
}

impl SimpleTransactionTypeVisitor {
    pub fn output(self) -> Option<(ComponentAddress, ComponentAddress, ResourceSpecifier)> {
        if self.illegal_instruction_encountered {
            None
        } else if let Ok((from_account, to_account, resource_specifier)) =
            self.state_machine.output()
        {
            Some((from_account, to_account, resource_specifier))
        } else {
            None
        }
    }
}

impl InstructionVisitor for SimpleTransactionTypeVisitor {
    fn is_enabled(&self) -> bool {
        !self.illegal_instruction_encountered
    }

    #[inline]
    fn visit_call_method(
        &mut self,
        address: &DynamicGlobalAddress,
        method_name: &str,
        args: &ManifestValue,
    ) -> Result<(), InstructionVisitorError> {
        if is_account(address) {
            let component_address = match address {
                DynamicGlobalAddress::Static(address) => {
                    ComponentAddress::new_or_panic(address.as_node_id().0)
                }
                DynamicGlobalAddress::Named(_) => {
                    self.illegal_instruction_encountered = true;
                    return Ok(());
                }
            };

            if let (
                ACCOUNT_WITHDRAW_IDENT,
                Some(AccountWithdrawInput {
                    resource_address,
                    amount,
                }),
            ) = (method_name, to_manifest_type(args))
            {
                let _ = self
                    .state_machine
                    .transition_to_account_withdraw(
                        component_address,
                        ResourceSpecifier::Amount(resource_address, amount),
                    )
                    .map_err(|_| {
                        self.illegal_instruction_encountered = true;
                    });
            } else if let (
                ACCOUNT_WITHDRAW_NON_FUNGIBLES_IDENT,
                Some(AccountWithdrawNonFungiblesInput {
                    resource_address,
                    ids,
                }),
            ) = (method_name, to_manifest_type(args))
            {
                let _ = self
                    .state_machine
                    .transition_to_account_withdraw(
                        component_address,
                        ResourceSpecifier::Ids(resource_address, ids),
                    )
                    .map_err(|_| {
                        self.illegal_instruction_encountered = true;
                    });
            } else if let (
                ACCOUNT_DEPOSIT_IDENT | ACCOUNT_TRY_DEPOSIT_OR_ABORT_IDENT,
                (Ok(()), _) | (_, Ok(())),
            ) = (
                method_name,
                (
                    validate_manifest_value_against_schema::<AccountDepositInput>(args),
                    validate_manifest_value_against_schema::<AccountTryDepositOrAbortInput>(args),
                ),
            ) {
                let _ = self
                    .state_machine
                    .transition_to_deposit(component_address)
                    .map_err(|_| {
                        self.illegal_instruction_encountered = true;
                    });
            } else {
                self.illegal_instruction_encountered = true;
            }
        } else {
            self.illegal_instruction_encountered = true
        }

        Ok(())
    }

    #[inline]
    fn visit_take_from_worktop(
        &mut self,
        _: &ResourceAddress,
        _: &Decimal,
    ) -> Result<(), InstructionVisitorError> {
        let _ = self
            .state_machine
            .transition_to_take_from_worktop()
            .map_err(|_| {
                self.illegal_instruction_encountered = true;
            });

        Ok(())
    }

    /* Illegal Instructions */

    #[inline]
    fn visit_take_all_from_worktop(
        &mut self,
        _: &ResourceAddress,
    ) -> Result<(), InstructionVisitorError> {
        self.illegal_instruction_encountered = true;
        Ok(())
    }

    #[inline]
    fn visit_take_non_fungibles_from_worktop(
        &mut self,
        _: &ResourceAddress,
        _: &[NonFungibleLocalId],
    ) -> Result<(), InstructionVisitorError> {
        self.illegal_instruction_encountered = true;
        Ok(())
    }

    #[inline]
    fn visit_return_to_worktop(
        &mut self,
        _: &ManifestBucket,
    ) -> Result<(), InstructionVisitorError> {
        self.illegal_instruction_encountered = true;
        Ok(())
    }

    #[inline]
    fn visit_assert_worktop_contains(
        &mut self,
        _: &ResourceAddress,
        _: &Decimal,
    ) -> Result<(), InstructionVisitorError> {
        self.illegal_instruction_encountered = true;
        Ok(())
    }

    #[inline]
    fn visit_assert_worktop_contains_non_fungibles(
        &mut self,
        _: &ResourceAddress,
        _: &[NonFungibleLocalId],
    ) -> Result<(), InstructionVisitorError> {
        self.illegal_instruction_encountered = true;
        Ok(())
    }

    #[inline]
    fn visit_pop_from_auth_zone(&mut self) -> Result<(), InstructionVisitorError> {
        self.illegal_instruction_encountered = true;
        Ok(())
    }

    #[inline]
    fn visit_push_to_auth_zone(
        &mut self,
        _: &ManifestProof,
    ) -> Result<(), InstructionVisitorError> {
        self.illegal_instruction_encountered = true;
        Ok(())
    }

    #[inline]
    fn visit_drop_auth_zone_proofs(&mut self) -> Result<(), InstructionVisitorError> {
        self.illegal_instruction_encountered = true;
        Ok(())
    }

    #[inline]
    fn visit_create_proof_from_auth_zone_of_amount(
        &mut self,
        _: &ResourceAddress,
        _: &Decimal,
    ) -> Result<(), InstructionVisitorError> {
        self.illegal_instruction_encountered = true;
        Ok(())
    }

    #[inline]
    fn visit_create_proof_from_auth_zone_of_non_fungibles(
        &mut self,
        _: &ResourceAddress,
        _: &[NonFungibleLocalId],
    ) -> Result<(), InstructionVisitorError> {
        self.illegal_instruction_encountered = true;
        Ok(())
    }

    #[inline]
    fn visit_create_proof_from_auth_zone_of_all(
        &mut self,
        _: &ResourceAddress,
    ) -> Result<(), InstructionVisitorError> {
        self.illegal_instruction_encountered = true;
        Ok(())
    }

    #[inline]
    fn visit_drop_auth_zone_signature_proofs(&mut self) -> Result<(), InstructionVisitorError> {
        self.illegal_instruction_encountered = true;
        Ok(())
    }

    #[inline]
    fn visit_create_proof_from_bucket_of_amount(
        &mut self,
        _: &ManifestBucket,
        _: &Decimal,
    ) -> Result<(), InstructionVisitorError> {
        self.illegal_instruction_encountered = true;
        Ok(())
    }

    #[inline]
    fn visit_create_proof_from_bucket_of_non_fungibles(
        &mut self,
        _: &ManifestBucket,
        _: &[NonFungibleLocalId],
    ) -> Result<(), InstructionVisitorError> {
        self.illegal_instruction_encountered = true;
        Ok(())
    }

    #[inline]
    fn visit_create_proof_from_bucket_of_all(
        &mut self,
        _: &ManifestBucket,
    ) -> Result<(), InstructionVisitorError> {
        self.illegal_instruction_encountered = true;
        Ok(())
    }

    #[inline]
    fn visit_burn_resource(&mut self, _: &ManifestBucket) -> Result<(), InstructionVisitorError> {
        self.illegal_instruction_encountered = true;
        Ok(())
    }

    #[inline]
    fn visit_clone_proof(&mut self, _: &ManifestProof) -> Result<(), InstructionVisitorError> {
        self.illegal_instruction_encountered = true;
        Ok(())
    }

    #[inline]
    fn visit_drop_proof(&mut self, _: &ManifestProof) -> Result<(), InstructionVisitorError> {
        self.illegal_instruction_encountered = true;
        Ok(())
    }

    #[inline]
    fn visit_call_function(
        &mut self,
        _: &DynamicPackageAddress,
        _: &str,
        _: &str,
        _: &ManifestValue,
    ) -> Result<(), InstructionVisitorError> {
        self.illegal_instruction_encountered = true;
        Ok(())
    }

    #[inline]
    fn visit_call_royalty_method(
        &mut self,
        _: &DynamicGlobalAddress,
        _: &str,
        _: &ManifestValue,
    ) -> Result<(), InstructionVisitorError> {
        self.illegal_instruction_encountered = true;
        Ok(())
    }

    #[inline]
    fn visit_call_metadata_method(
        &mut self,
        _: &DynamicGlobalAddress,
        _: &str,
        _: &ManifestValue,
    ) -> Result<(), InstructionVisitorError> {
        self.illegal_instruction_encountered = true;
        Ok(())
    }

    #[inline]
    fn visit_call_role_assignment_method(
        &mut self,
        _: &DynamicGlobalAddress,
        _: &str,
        _: &ManifestValue,
    ) -> Result<(), InstructionVisitorError> {
        self.illegal_instruction_encountered = true;
        Ok(())
    }

    #[inline]
    fn visit_call_direct_vault_method(
        &mut self,
        _: &InternalAddress,
        _: &str,
        _: &ManifestValue,
    ) -> Result<(), InstructionVisitorError> {
        self.illegal_instruction_encountered = true;
        Ok(())
    }

    #[inline]
    fn visit_drop_all_proofs(&mut self) -> Result<(), InstructionVisitorError> {
        self.illegal_instruction_encountered = true;
        Ok(())
    }
}

#[derive(Clone, Debug, Default)]
enum StateMachine {
    #[default]
    None,
    AccountWithdraw(ComponentAddress, ResourceSpecifier),
    TakeFromWorktop(ComponentAddress, ResourceSpecifier),
    Deposit(ComponentAddress, ComponentAddress, ResourceSpecifier),
}

impl StateMachine {
    pub fn transition_to_account_withdraw(
        &mut self,
        component_address: ComponentAddress,
        resource_specifier: ResourceSpecifier,
    ) -> Result<(), StateMachineError> {
        match self {
            Self::None => {
                *self = Self::AccountWithdraw(component_address, resource_specifier);
                Ok(())
            }
            Self::AccountWithdraw(..) | Self::TakeFromWorktop(..) | Self::Deposit(..) => {
                Err(StateMachineError)
            }
        }
    }

    pub fn transition_to_take_from_worktop(&mut self) -> Result<(), StateMachineError> {
        match self {
            Self::AccountWithdraw(from_account, resources) => {
                *self = Self::TakeFromWorktop(*from_account, resources.clone());
                Ok(())
            }
            Self::None | Self::TakeFromWorktop(..) | Self::Deposit(..) => Err(StateMachineError),
        }
    }

    pub fn transition_to_deposit(
        &mut self,
        to_account: ComponentAddress,
    ) -> Result<(), StateMachineError> {
        match self {
            Self::TakeFromWorktop(from_account, resources) => {
                *self = Self::Deposit(*from_account, to_account, resources.clone());
                Ok(())
            }
            Self::None | Self::Deposit(..) | Self::AccountWithdraw(..) => Err(StateMachineError),
        }
    }

    pub fn output(
        self,
    ) -> Result<(ComponentAddress, ComponentAddress, ResourceSpecifier), StateMachineError> {
        match self {
            Self::Deposit(from, to, resources) => Ok((from, to, resources)),
            Self::None | Self::TakeFromWorktop(..) | Self::AccountWithdraw(..) => {
                Err(StateMachineError)
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct StateMachineError;
