use scrypto::prelude::*;
use transaction::prelude::*;

use radix_engine::system::system_modules::execution_trace::*;
use radix_engine_interface::blueprints::account::*;
use radix_engine_interface::blueprints::consensus_manager::*;

use crate::transaction_types::types::*;
use crate::transaction_types::*;
use crate::utils::*;

pub struct TrackedValidatorStake {
    pub validator_address: ComponentAddress,
    /* Input */
    pub xrd_amount: Decimal,
    /* Output */
    pub liquid_stake_unit_address: ResourceAddress,
    pub liquid_stake_unit_amount: Decimal,
}

pub struct ValidatorStakeDetector {
    is_valid: bool,
    /// The validators encountered in this manifest that were staked to.
    validators: IndexSet<GlobalAddress>,
    /// Tracks the stake operations that ocurred in the transaction.
    tracked_stake: Vec<TrackedValidatorStake>,
}

impl ManifestSummaryCallback for ValidatorStakeDetector {
    fn on_instruction(
        &mut self,
        instruction: &InstructionV1,
        _instruction_index: usize,
    ) {
        self.is_valid &= match instruction {
            /* Maybe Permitted - Need more info */
            InstructionV1::CallMethod {
                address,
                method_name,
                ..
            } => Self::construct_fn_rules(address).is_fn_permitted(method_name),
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
                matches!(entity_type, EntityType::GlobalValidator)
            })
        {
            self.validators.insert(address);
        }
    }
}

impl ExecutionSummaryCallback for ValidatorStakeDetector {
    fn on_instruction(
        &mut self,
        instruction: &InstructionV1,
        _instruction_index: usize,
        input_resources: &[ResourceSpecifier],
        output_resources: &[ResourceSpecifier],
    ) {
        match instruction {
            InstructionV1::CallMethod {
                address: dynamic_address @ DynamicGlobalAddress::Static(address),
                method_name,
                ..
            } if is_validator(dynamic_address)
                && (method_name == VALIDATOR_STAKE_IDENT) =>
            {
                let validator_component = ComponentAddress::try_from(*address)
                    .expect("Must succeed!");

                // TODO: Filter + Pattern matching is perhaps redundant?
                let Some(ResourceSpecifier::Amount(XRD, xrd_amount)) =
                    input_resources
                        .iter()
                        .find(|specifier| specifier.resource_address() == XRD)
                else {
                    // Can happen if an empty bucket of XRD is provided as
                    // input.
                    return;
                };

                let Some(ResourceSpecifier::Amount(
                    stake_units_resource_address,
                    stake_units_amount,
                )) = output_resources.first()
                else {
                    // TODO: Error? Panic? This is especially concerning because
                    // we can never get no stake units back when there is some
                    // XRD input. So, is this even a real case? Maybe just
                    // unwrap since it's impossible to get here?
                    return;
                };

                self.tracked_stake.push(TrackedValidatorStake {
                    validator_address: validator_component,
                    xrd_amount: *xrd_amount,
                    liquid_stake_unit_address: *stake_units_resource_address,
                    liquid_stake_unit_amount: *stake_units_amount,
                });
            }
            _ => { /* No-op */ }
        }
    }
}

impl ValidatorStakeDetector {
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
                        EntityType::GlobalValidator => FnRules {
                            allowed: &[
                                VALIDATOR_STAKE_IDENT
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
                        | EntityType::GlobalFungibleResourceManager
                        | EntityType::GlobalNonFungibleResourceManager
                        | EntityType::GlobalConsensusManager
                        | EntityType::InternalFungibleVault
                        | EntityType::InternalNonFungibleVault
                        | EntityType::InternalKeyValueStore
                        | EntityType::GlobalTransactionTracker
                        | EntityType::GlobalAccessController
                        | EntityType::GlobalOneResourcePool
                        | EntityType::GlobalTwoResourcePool
                        | EntityType::GlobalMultiResourcePool
                         => FnRules::all_disallowed(),
                    }
                    })
                    .unwrap_or(FnRules::all_disallowed())
            }
        }
    }
}
