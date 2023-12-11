use radix_engine::system::system_modules::execution_trace::*;
use scrypto::prelude::*;
use transaction::prelude::*;

use crate::transaction_types::types::*;

/// A callback for information that can be obtained statically from the manifest
/// without the need for the receipt or execution trace.
pub trait ManifestSummaryCallback {
    /// Called when the traverser starts going through a new instruction with
    /// the new instruction and the index that it is at.
    #[inline]
    fn on_instruction(
        &mut self,
        instruction: &InstructionV1,
        instruction_index: usize,
    ) {
    }

    /// Called when the instructions in the manifest have finished.
    #[inline]
    fn on_finish(&mut self, instructions_count: usize) {}

    /// Called when a global entity is encountered in the manifest for the first
    /// time to inform other observers of it.
    #[inline]
    fn on_global_entity_encounter(&mut self, address: GlobalAddress) {}

    /* Higher-level abstractions & resource movements */

    /// Called when a proof is created either out of calling an account method
    /// or from a bucket.
    #[inline]
    fn on_create_proof(&mut self, resource_specifier: &ResourceSpecifier) {}

    /// Called when resources are withdrawn from an account with the account and
    /// withdraw information.
    #[inline]
    fn on_account_withdraw(
        &mut self,
        account: &ComponentAddress,
        resource_address: &ResourceAddress,
        withdraw_information: &WithdrawInformation,
    ) {
    }
}

pub trait ExecutionSummaryCallback {
    /// Called when a deposit is performed into an account with the information
    /// of the deposited resources.
    #[inline]
    fn on_account_deposit(
        &mut self,
        account: &ComponentAddress,
        deposit_information: &DepositInformation,
    ) {
    }
}
