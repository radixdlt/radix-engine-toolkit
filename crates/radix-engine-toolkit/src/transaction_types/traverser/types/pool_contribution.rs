use std::ops::*;

use scrypto::prelude::*;
use transaction::prelude::*;

use radix_engine_interface::blueprints::account::*;
use radix_engine_interface::blueprints::pool::*;

use crate::transaction_types::*;

struct TrackedPoolContribution {
    pool_address: ComponentAddress,
    /* Input */
    contributed_resources: IndexMap<ResourceAddress, Decimal>,
    /* Output */
    pool_units_resource_address: ResourceAddress,
    pool_units_amount: Decimal,
}

pub struct PoolContributionDetector {
    is_valid: bool,
    /// The pools encountered in this manifest that were contributed to.
    pools: IndexSet<GlobalAddress>,
    /// Tracks the contributions that occurred in the transaction
    tracked_contributions: Vec<TrackedPoolContribution>,
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
                let pool_address = ComponentAddress::try_from(*address)
                    .expect("Must be a valid component address");

                // Determine the output pool units. If we can't find them then
                // it means that no pool units were returned and that nothing
                // was contributed.
                let Some(SourceResourceSpecifier::Amount(
                    pool_unit_resource_address,
                    pool_unit_amount,
                )) = Self::pool_unit_resource_specifier(
                    &input_resources,
                    &output_resources,
                )
                else {
                    return;
                };

                let mut tracked_contribution = TrackedPoolContribution {
                    pool_address,
                    pool_units_resource_address: pool_unit_resource_address,
                    pool_units_amount: *pool_unit_amount,
                    contributed_resources: Default::default(),
                };

                // Accounting for how much resources were contributed from the
                // input and the output (the change).
                for resource_specifier in input_resources.iter() {
                    let SourceResourceSpecifier::Amount(
                        resource_address,
                        amount,
                    ) = resource_specifier
                    else {
                        continue;
                    };

                    tracked_contribution
                        .contributed_resources
                        .entry(*resource_address)
                        .or_default()
                        .add_assign(**amount);
                }
                for resource_specifier in output_resources.iter() {
                    let SourceResourceSpecifier::Amount(
                        resource_address,
                        amount,
                    ) = resource_specifier
                    else {
                        continue;
                    };
                    let Some(entry) = tracked_contribution
                        .contributed_resources
                        .get_mut(resource_address)
                    else {
                        continue;
                    };
                    entry.sub_assign(**amount);
                }
                tracked_contribution.contributed_resources =
                    tracked_contribution
                        .contributed_resources
                        .into_iter()
                        .filter(|(k, v)| !v.is_zero())
                        .collect();

                self.tracked_contributions.push(tracked_contribution);
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

    fn pool_unit_resource_specifier(
        input: &Vec<&SourceResourceSpecifier>,
        output: &Vec<&SourceResourceSpecifier>,
    ) -> Option<SourceResourceSpecifier> {
        // The pool unit resource specifier is that which is only present in the
        // output and not in the input. We also account for the pool returning
        // change so we do not use index based detection as it would't reliable
        // in this case.
        let input_resources = input
            .iter()
            .map(|specifier| specifier.resource_address())
            .collect::<IndexSet<_>>();

        output
            .iter()
            .filter(|specifier| {
                !input_resources.contains(&specifier.resource_address())
            })
            .cloned()
            .cloned()
            .next()
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
