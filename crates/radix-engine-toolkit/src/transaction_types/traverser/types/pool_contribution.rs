use std::ops::{AddAssign, Deref, SubAssign};

use scrypto::prelude::*;
use transaction::prelude::*;

use radix_engine_interface::blueprints::account::*;
use radix_engine_interface::blueprints::pool::*;

use crate::transaction_types::*;

pub struct PoolContributionDetector {
    is_valid: bool,
    /// The pools encountered in this manifest that were contributed to.
    pools: IndexSet<GlobalAddress>,
    /// A map of all of the contributions made in the transaction.
    tracked_pool_units: IndexMap<
        (ComponentAddress, ResourceAddress),
        IndexMap<ResourceAddress, Decimal>,
    >,
}

impl ManifestSummaryCallback for PoolContributionDetector {
    fn on_instruction(
        &mut self,
        instruction: &InstructionV1,
        instruction_index: usize,
    ) {
        self.is_valid &= match instruction {
            /* Maybe Permitted - Need more info */
            InstructionV1::CallMethod {
                address,
                method_name,
                ..
            } => {
                Self::construct_fn_rules(address).is_fn_permitted(&method_name)
            }
            /* Permitted */
            InstructionV1::TakeFromWorktop { .. }
            | InstructionV1::TakeNonFungiblesFromWorktop { .. }
            | InstructionV1::TakeAllFromWorktop { .. }
            | InstructionV1::AssertWorktopContainsAny { .. }
            | InstructionV1::AssertWorktopContains { .. }
            | InstructionV1::AssertWorktopContainsNonFungibles { .. } => true,
            /* Not Permitted */
            InstructionV1::BurnResource { .. }
            | InstructionV1::CallRoyaltyMethod { .. }
            | InstructionV1::CallMetadataMethod { .. }
            | InstructionV1::CallRoleAssignmentMethod { .. }
            | InstructionV1::CallDirectVaultMethod { .. }
            | InstructionV1::AllocateGlobalAddress { .. }
            | InstructionV1::ReturnToWorktop { .. }
            | InstructionV1::PopFromAuthZone
            | InstructionV1::PushToAuthZone { .. }
            | InstructionV1::CreateProofFromAuthZoneOfAmount { .. }
            | InstructionV1::CreateProofFromAuthZoneOfNonFungibles { .. }
            | InstructionV1::CreateProofFromAuthZoneOfAll { .. }
            | InstructionV1::DropAuthZoneProofs
            | InstructionV1::DropAuthZoneRegularProofs
            | InstructionV1::DropAuthZoneSignatureProofs
            | InstructionV1::CreateProofFromBucketOfAmount { .. }
            | InstructionV1::CreateProofFromBucketOfNonFungibles { .. }
            | InstructionV1::CreateProofFromBucketOfAll { .. }
            | InstructionV1::CloneProof { .. }
            | InstructionV1::DropProof { .. }
            | InstructionV1::DropNamedProofs
            | InstructionV1::DropAllProofs
            | InstructionV1::CallFunction { .. } => false,
        }
    }

    fn on_global_entity_encounter(&mut self, address: GlobalAddress) {
        if address
            .as_node_id()
            .entity_type()
            .is_some_and(|entity_type| {
                matches!(
                    entity_type,
                    EntityType::GlobalOneResourcePool
                        | EntityType::GlobalTwoResourcePool
                        | EntityType::GlobalMultiResourcePool
                )
            })
        {
            self.pools.insert(address);
        }
    }
}

impl ExecutionSummaryCallback for PoolContributionDetector {
    fn on_instruction(
        &mut self,
        instruction: &InstructionV1,
        instruction_index: usize,
        input_resources: Vec<&SourceResourceSpecifier>,
        output_resources: Vec<&SourceResourceSpecifier>,
    ) {
        match instruction {
            InstructionV1::CallMethod {
                address: dynamic_address @ DynamicGlobalAddress::Static(address),
                method_name,
                ..
            } if is_pool(dynamic_address)
                && (method_name == ONE_RESOURCE_POOL_CONTRIBUTE_IDENT
                    || method_name == TWO_RESOURCE_POOL_CONTRIBUTE_IDENT
                    || method_name == MULTI_RESOURCE_POOL_CONTRIBUTE_IDENT) =>
            {
                // The pool unit is the only res
                let input_resource_addresses = input_resources
                    .iter()
                    .map(|resource_specifier| {
                        resource_specifier.resource_address()
                    })
                    .collect::<IndexSet<ResourceAddress>>();
                let Some(pool_unit_resource_specifier) = output_resources
                    .iter()
                    .filter(|item| {
                        !input_resource_addresses
                            .contains(&item.resource_address())
                    })
                    .next()
                else {
                    // This is for the case when no pool units were returned
                    // which happens if the input is already empty. Wallet
                    // should be able to deal with this with no issues.
                    return;
                };

                // Accounting for the resource inputs and outputs.
                for resource_specifier in input_resources {
                    let SourceResourceSpecifier::Amount(
                        resource_address,
                        amount,
                    ) = resource_specifier
                    else {
                        continue;
                    };
                    self.tracked_pool_units
                        .entry((
                            ComponentAddress::try_from(*address)
                                .expect("Must be a valid component address"),
                            pool_unit_resource_specifier.resource_address(),
                        ))
                        .or_default()
                        .entry(resource_specifier.resource_address())
                        .or_default()
                        .add_assign(*amount.deref());
                }
                for resource_specifier in output_resources {
                    let SourceResourceSpecifier::Amount(
                        resource_address,
                        amount,
                    ) = resource_specifier
                    else {
                        continue;
                    };
                    let Some(entry) = self
                        .tracked_pool_units
                        .get_mut(&(
                            ComponentAddress::try_from(*address)
                                .expect("Must be a valid component address"),
                            pool_unit_resource_specifier.resource_address(),
                        ))
                        .and_then(|entry| entry.get_mut(resource_address))
                    else {
                        continue;
                    };
                    entry.sub_assign(*amount.deref());
                }
            }
            _ => { /* No-op */ }
        }
    }
}

impl PoolContributionDetector {
    pub fn is_valid(&self) -> bool {
        self.is_valid
    }

    fn construct_fn_rules(address: &DynamicGlobalAddress) -> FnRules {
        match address {
            DynamicGlobalAddress::Named(..) => FnRules::all_disallowed(),
            DynamicGlobalAddress::Static(address) => {
                address
                    .as_node_id()
                    .entity_type()
                    .map(|entity_type| {
                        match entity_type {
                        EntityType::GlobalAccount
                        | EntityType::GlobalVirtualSecp256k1Account
                        | EntityType::GlobalVirtualEd25519Account => FnRules {
                            allowed: &[
                                /* All withdraw methods */
                                ACCOUNT_WITHDRAW_IDENT,
                                ACCOUNT_WITHDRAW_NON_FUNGIBLES_IDENT,
                                /* All deposit methods */
                                ACCOUNT_DEPOSIT_IDENT,
                                ACCOUNT_DEPOSIT_BATCH_IDENT,
                                ACCOUNT_TRY_DEPOSIT_OR_ABORT_IDENT,
                                ACCOUNT_TRY_DEPOSIT_BATCH_OR_ABORT_IDENT,
                            ],
                            disallowed: &[],
                            default: FnRule::Disallowed,
                        },
                        EntityType::GlobalOneResourcePool => FnRules {
                            allowed: &[
                                ONE_RESOURCE_POOL_CONTRIBUTE_IDENT
                            ],
                            disallowed: &[],
                            default: FnRule::Disallowed
                        },
                        EntityType::GlobalTwoResourcePool => FnRules {
                            allowed: &[
                                TWO_RESOURCE_POOL_CONTRIBUTE_IDENT
                            ],
                            disallowed: &[],
                            default: FnRule::Disallowed
                        },
                        EntityType::GlobalMultiResourcePool => FnRules {
                            allowed: &[
                                MULTI_RESOURCE_POOL_CONTRIBUTE_IDENT
                            ],
                            disallowed: &[],
                            default: FnRule::Disallowed
                        },
                        /* Disallowed */
                        EntityType::GlobalGenericComponent
                        | EntityType::GlobalIdentity
                        | EntityType::GlobalVirtualSecp256k1Identity
                        | EntityType::GlobalVirtualEd25519Identity
                        | EntityType::InternalGenericComponent
                        | EntityType::GlobalPackage
                        | EntityType::GlobalValidator
                        | EntityType::GlobalFungibleResourceManager
                        | EntityType::GlobalNonFungibleResourceManager
                        | EntityType::GlobalConsensusManager
                        | EntityType::InternalFungibleVault
                        | EntityType::InternalNonFungibleVault
                        | EntityType::InternalKeyValueStore
                        | EntityType::GlobalTransactionTracker
                        | EntityType::GlobalAccessController
                         => FnRules::all_disallowed(),
                    }
                    })
                    .unwrap_or(FnRules::all_disallowed())
            }
        }
    }
}

fn is_pool(address: &DynamicGlobalAddress) -> bool {
    match address {
        DynamicGlobalAddress::Static(address) => address
            .as_node_id()
            .entity_type()
            .is_some_and(|entity_type| {
                matches!(
                    entity_type,
                    EntityType::GlobalOneResourcePool
                        | EntityType::GlobalTwoResourcePool
                        | EntityType::GlobalMultiResourcePool
                )
            }),
        DynamicGlobalAddress::Named(_) => false,
    }
}
