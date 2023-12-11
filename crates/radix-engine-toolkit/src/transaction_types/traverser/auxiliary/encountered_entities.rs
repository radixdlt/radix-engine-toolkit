use scrypto::prelude::*;

use crate::transaction_types::*;

pub struct EncounteredGlobalEntities {
    entities: IndexSet<GlobalAddress>,
}

impl ExecutionSummaryCallback for EncounteredGlobalEntities {
    fn on_global_entity_encounter(&mut self, address: GlobalAddress) {
        self.entities.insert(address);
    }
}
