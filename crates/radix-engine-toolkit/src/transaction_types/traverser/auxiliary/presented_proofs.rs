use scrypto::prelude::*;

use crate::transaction_types::*;

pub struct PresentedProofsDetector {
    presented_proofs: IndexSet<ResourceAddress>,
}

impl ManifestSummaryCallback for PresentedProofsDetector {
    fn on_create_proof(&mut self, resource_address: &ResourceAddress) {
        self.presented_proofs.insert(*resource_address);
    }
}

impl ExecutionSummaryCallback for PresentedProofsDetector {}
