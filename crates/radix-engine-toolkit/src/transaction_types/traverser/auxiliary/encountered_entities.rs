use scrypto::prelude::*;

use crate::transaction_types::*;

pub struct EncounteredGlobalEntities {
    entities: IndexSet<GlobalAddress>,
}

impl ManifestSummaryCallback for EncounteredGlobalEntities {
    fn on_global_entity_encounter(&mut self, address: GlobalAddress) {
        self.entities.insert(address);
    }
}

impl ExecutionSummaryCallback for EncounteredGlobalEntities {}
