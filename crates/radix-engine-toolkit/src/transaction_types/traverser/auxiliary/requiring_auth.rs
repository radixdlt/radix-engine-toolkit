use scrypto::prelude::*;
use transaction::prelude::*;

use crate::statics::*;
use crate::transaction_types::*;

pub struct RequiringAuthDetector {
    accounts: IndexSet<ComponentAddress>,
    identities: IndexSet<ComponentAddress>,
}

impl ManifestSummaryCallback for RequiringAuthDetector {
    fn on_instruction(&mut self, instruction: &InstructionV1, _: usize) {
        match instruction {
            InstructionV1::CallMethod {
                address: DynamicGlobalAddress::Static(address),
                method_name,
                ..
            } => {
                if is_account(address)
                    && ACCOUNT_METHODS_THAT_REQUIRE_AUTH.contains(method_name)
                {
                    self.accounts.insert(component_address!(*address));
                } else if is_identity(address)
                    && IDENTITY_METHODS_THAT_REQUIRE_AUTH.contains(method_name)
                {
                    self.identities.insert(component_address!(*address));
                }
            }
            InstructionV1::CallRoyaltyMethod {
                address: DynamicGlobalAddress::Static(address),
                method_name,
                ..
            } => {
                if ROYALTY_METHODS_THAT_REQUIRE_AUTH.contains(method_name) {
                    if is_account(address) {
                        self.accounts.insert(component_address!(*address));
                    } else if is_identity(address) {
                        self.identities.insert(component_address!(*address));
                    }
                }
            }
            InstructionV1::CallMetadataMethod {
                address: DynamicGlobalAddress::Static(address),
                method_name,
                ..
            } => {
                if METADATA_METHODS_THAT_REQUIRE_AUTH.contains(method_name) {
                    if is_account(address) {
                        self.accounts.insert(component_address!(*address));
                    } else if is_identity(address) {
                        self.identities.insert(component_address!(*address));
                    }
                }
            }
            InstructionV1::CallRoleAssignmentMethod {
                address: DynamicGlobalAddress::Static(address),
                method_name,
                ..
            } => {
                if ROLE_ASSIGNMENT_METHODS_THAT_REQUIRE_AUTH
                    .contains(method_name)
                {
                    if is_account(address) {
                        self.accounts.insert(component_address!(*address));
                    } else if is_identity(address) {
                        self.identities.insert(component_address!(*address));
                    }
                }
            }
            _ => { /* No-op */ }
        }
    }
}

impl ExecutionSummaryCallback for RequiringAuthDetector {}

fn is_account(address: &GlobalAddress) -> bool {
    address
        .as_node_id()
        .entity_type()
        .is_some_and(|entity_type| {
            matches!(
                entity_type,
                EntityType::GlobalAccount
                    | EntityType::GlobalVirtualSecp256k1Account
                    | EntityType::GlobalVirtualEd25519Account
            )
        })
}

fn is_identity(address: &GlobalAddress) -> bool {
    address
        .as_node_id()
        .entity_type()
        .is_some_and(|entity_type| {
            matches!(
                entity_type,
                EntityType::GlobalIdentity
                    | EntityType::GlobalVirtualSecp256k1Identity
                    | EntityType::GlobalVirtualEd25519Identity
            )
        })
}

macro_rules! component_address {
    ($expr: expr) => {
        ::scrypto::prelude::ComponentAddress::try_from($expr)
            .expect("Must be a valid component address")
    };
}
use component_address;
