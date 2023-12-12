use radix_engine_interface::blueprints::account::*;
use radix_engine_interface::blueprints::identity::*;

use scrypto::prelude::*;
use transaction::prelude::*;

use crate::transaction_types::*;
use crate::utils::*;
use crate::*;

pub struct ReservedInstructionsDetector {
    reserved_instructions: IndexSet<ReservedInstruction>,
}

impl ManifestSummaryCallback for ReservedInstructionsDetector {
    fn on_instruction(
        &mut self,
        instruction: &InstructionV1,
        instruction_index: usize,
    ) {
        let InstructionV1::CallMethod {
            address,
            method_name,
            ..
        } = instruction
        else {
            return;
        };

        if is_account(address)
            && contains!(
                method_name => [
                    ACCOUNT_LOCK_FEE_IDENT,
                    ACCOUNT_LOCK_FEE_AND_WITHDRAW_IDENT,
                    ACCOUNT_LOCK_FEE_AND_WITHDRAW_NON_FUNGIBLES_IDENT,
                ]
            )
        {
            self.reserved_instructions
                .insert(ReservedInstruction::AccountLockFee);
        } else if is_account(address) && method_name == ACCOUNT_SECURIFY_IDENT {
            self.reserved_instructions
                .insert(ReservedInstruction::AccountLockFee);
        } else if is_identity(address) && method_name == IDENTITY_SECURIFY_IDENT
        {
            self.reserved_instructions
                .insert(ReservedInstruction::AccountLockFee);
        } else if is_account(address)
            && contains!(
                method_name => [
                    ACCOUNT_ADD_AUTHORIZED_DEPOSITOR,
                    ACCOUNT_REMOVE_AUTHORIZED_DEPOSITOR,
                    ACCOUNT_SET_RESOURCE_PREFERENCE_IDENT,
                    ACCOUNT_REMOVE_RESOURCE_PREFERENCE_IDENT,
                    ACCOUNT_SET_DEFAULT_DEPOSIT_RULE_IDENT,
                ]
            )
        {
            self.reserved_instructions
                .insert(ReservedInstruction::AccountUpdateSettings);
        } else if is_access_controller(address) {
            self.reserved_instructions
                .insert(ReservedInstruction::AccessControllerMethod);
        }
    }
}

impl ExecutionSummaryCallback for ReservedInstructionsDetector {}
