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
use crate::utils::{is_account, to_manifest_type, validate_manifest_value_against_schema};
use radix_engine::system::system_modules::execution_trace::ResourceSpecifier;
use scrypto::blueprints::account::*;
use scrypto::prelude::*;
use transaction::prelude::{DynamicGlobalAddress, DynamicPackageAddress};

#[derive(Default, Debug, Clone)]
pub struct SimpleTransferVisitor {
    withdraw: Option<(ComponentAddress, ResourceSpecifier)>,
    deposit: Option<ComponentAddress>,

    illegal_instruction_encountered: bool,
    instruction_index: usize,
}

impl SimpleTransferVisitor {
    pub fn output(self) -> Option<(ComponentAddress, ComponentAddress, ResourceSpecifier)> {
        if self.illegal_instruction_encountered {
            None
        } else if let (Some((from_account, resource_specifier)), Some(to_account)) =
            (self.withdraw, self.deposit)
        {
            Some((from_account, to_account, resource_specifier))
        } else {
            None
        }
    }
}

impl InstructionVisitor for SimpleTransferVisitor {
    fn is_enabled(&self) -> bool {
        !self.illegal_instruction_encountered
    }

    fn post_visit(&mut self) -> Result<(), InstructionVisitorError> {
        self.instruction_index += 1;
        Ok(())
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

            // Two account methods are allowed: Withdraw and Withdraw non-fungibles.
            if let (
                ACCOUNT_WITHDRAW_IDENT,
                Some(AccountWithdrawInput {
                    resource_address,
                    amount,
                }),
                0,
                None,
            ) = (
                method_name,
                to_manifest_type(args),
                self.instruction_index,
                &self.withdraw,
            ) {
                self.withdraw = Some((
                    component_address,
                    ResourceSpecifier::Amount(resource_address, amount),
                ))
            } else if let (
                ACCOUNT_WITHDRAW_NON_FUNGIBLES_IDENT,
                Some(AccountWithdrawNonFungiblesInput {
                    resource_address,
                    ids,
                }),
                0,
                None,
            ) = (
                method_name,
                to_manifest_type(args),
                self.instruction_index,
                &self.withdraw,
            ) {
                self.withdraw = Some((
                    component_address,
                    ResourceSpecifier::Ids(resource_address, ids),
                ))
            } else if let (
                ACCOUNT_DEPOSIT_IDENT
                | ACCOUNT_TRY_DEPOSIT_OR_ABORT_IDENT
                | ACCOUNT_TRY_DEPOSIT_OR_REFUND_IDENT,
                Ok(_),
                2,
                Some(..),
                None,
            ) = (
                method_name,
                validate_manifest_value_against_schema::<AccountDepositInput>(args),
                self.instruction_index,
                &self.withdraw,
                &self.deposit,
            ) {
                self.deposit = Some(component_address)
            } else {
                self.illegal_instruction_encountered = true
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
        if self.instruction_index != 1 {
            self.illegal_instruction_encountered = true
        }

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
    fn visit_clear_auth_zone(&mut self) -> Result<(), InstructionVisitorError> {
        self.illegal_instruction_encountered = true;
        Ok(())
    }

    #[inline]
    fn visit_create_proof_from_auth_zone(
        &mut self,
        _: &ResourceAddress,
    ) -> Result<(), InstructionVisitorError> {
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
    fn visit_clear_signature_proofs(&mut self) -> Result<(), InstructionVisitorError> {
        self.illegal_instruction_encountered = true;
        Ok(())
    }

    #[inline]
    fn visit_create_proof_from_bucket(
        &mut self,
        _: &ManifestBucket,
    ) -> Result<(), InstructionVisitorError> {
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
    fn visit_call_access_rules_method(
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
