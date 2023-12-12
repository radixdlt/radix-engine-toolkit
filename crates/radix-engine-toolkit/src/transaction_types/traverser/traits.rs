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

    /// Called when a proof is created out of an account.
    #[inline]
    fn on_create_proof(&mut self, resource_address: &ResourceAddress) {}

    /// Called when a global entity is encountered in the manifest
    #[inline]
    fn on_global_entity_encounter(&mut self, address: GlobalAddress) {}

    /// Called when the instructions in the manifest have finished.
    #[inline]
    fn on_finish(&mut self, instructions_count: usize) {}
}

pub trait ExecutionSummaryCallback
where
    Self: ManifestSummaryCallback,
{
    /// Called when the traverser starts going through a new instruction with
    /// the new instruction and the index that it is at and information on the
    /// input resources that this instruction took and the output resources.
    #[inline]
    fn on_instruction(
        &mut self,
        instruction: &InstructionV1,
        instruction_index: usize,
        input_resources: &[ResourceSpecifier],
        output_resources: &[ResourceSpecifier],
    ) {
    }

    /// Called when resources are withdrawn from an account with the account and
    /// withdraw information.
    #[inline]
    fn on_account_withdraw(
        &mut self,
        account: &ComponentAddress,
        resource_indicator: &ResourceIndicator,
    ) {
    }

    /// Called when a deposit is performed into an account with the information
    /// of the deposited resources.
    #[inline]
    fn on_account_deposit(
        &mut self,
        account: &ComponentAddress,
        resource_indicator: &ResourceIndicator,
    ) {
    }
}
