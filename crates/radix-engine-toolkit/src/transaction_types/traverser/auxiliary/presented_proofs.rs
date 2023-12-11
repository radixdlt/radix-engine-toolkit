use radix_engine::system::system_modules::execution_trace::*;
use scrypto::prelude::*;

use crate::transaction_types::*;

pub struct PresentedProofsDetector {
    presented_proofs: IndexSet<ResourceAddress>,
}

impl ManifestSummaryCallback for PresentedProofsDetector {
    fn on_create_proof(&mut self, resource_specifier: &ResourceSpecifier) {
        let resource_address = match resource_specifier {
            ResourceSpecifier::Amount(resource_address, ..)
            | ResourceSpecifier::Ids(resource_address, ..) => resource_address,
        };
        self.presented_proofs.insert(*resource_address);
    }
}

impl ExecutionSummaryCallback for PresentedProofsDetector {}
