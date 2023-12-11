use scrypto::prelude::*;
use transaction::prelude::*;

use radix_engine_interface::blueprints::account::*;

use crate::transaction_types::*;

pub struct GeneralDetector {
    is_valid: bool,
}

impl ManifestSummaryCallback for GeneralDetector {
    fn on_instruction(
        &mut self,
        instruction: &InstructionV1,
        instruction_index: usize,
    ) {
        // Control whether or not this is allowed or not based on:
        // 1. Whether the instruction is allowed.
        // 2. Whether the instruction contents are allowed.
        self.is_valid &= match instruction {
            /* Maybe Permitted - Need more info */
            InstructionV1::CallMethod {
                address,
                method_name,
                ..
            } => {
                Self::construct_fn_rules(address).is_fn_permitted(&method_name)
            }
            /* Not Permitted */
            InstructionV1::TakeFromWorktop { .. }
            | InstructionV1::TakeNonFungiblesFromWorktop { .. }
            | InstructionV1::TakeAllFromWorktop { .. }
            | InstructionV1::ReturnToWorktop { .. }
            | InstructionV1::AssertWorktopContainsAny { .. }
            | InstructionV1::AssertWorktopContains { .. }
            | InstructionV1::AssertWorktopContainsNonFungibles { .. }
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
            | InstructionV1::CallFunction { .. } => true,
            /* Not Permitted */
            InstructionV1::BurnResource { .. }
            | InstructionV1::CallRoyaltyMethod { .. }
            | InstructionV1::CallMetadataMethod { .. }
            | InstructionV1::CallRoleAssignmentMethod { .. }
            | InstructionV1::CallDirectVaultMethod { .. }
            | InstructionV1::AllocateGlobalAddress { .. } => false,
        }
    }
}

impl ExecutionSummaryCallback for GeneralDetector {}

impl GeneralDetector {
    pub fn is_valid(&self) -> bool {
        self.is_valid
    }

    fn construct_fn_rules(address: &DynamicGlobalAddress) -> FnRules {
        match address {
            DynamicGlobalAddress::Named(..) => FnRules::all_disallowed(),
            DynamicGlobalAddress::Static(address) => {
                address.as_node_id().entity_type().map(|entity_type| {
                    match entity_type {
                        EntityType::GlobalAccount
                        | EntityType::GlobalVirtualSecp256k1Account
                        | EntityType::GlobalVirtualEd25519Account => FnRules {
                            allowed: vec![
                                /* All withdraw methods */
                                ACCOUNT_WITHDRAW_IDENT,
                                ACCOUNT_WITHDRAW_NON_FUNGIBLES_IDENT,
                                /* All deposit methods */
                                ACCOUNT_DEPOSIT_IDENT,
                                ACCOUNT_DEPOSIT_BATCH_IDENT,
                                ACCOUNT_TRY_DEPOSIT_OR_ABORT_IDENT,
                                ACCOUNT_TRY_DEPOSIT_BATCH_OR_ABORT_IDENT,
                                /* All proof creation methods */
                                ACCOUNT_CREATE_PROOF_OF_AMOUNT_IDENT,
                                ACCOUNT_CREATE_PROOF_OF_NON_FUNGIBLES_IDENT,
                            ],
                            disallowed: vec![
                                /* Securification */
                                ACCOUNT_SECURIFY_IDENT,
                                /* Direct Burn from Account */
                                ACCOUNT_BURN_IDENT,
                                ACCOUNT_BURN_NON_FUNGIBLES_IDENT,
                                /* Manipulation of the Authorized Depositors list */
                                ACCOUNT_ADD_AUTHORIZED_DEPOSITOR,
                                ACCOUNT_REMOVE_AUTHORIZED_DEPOSITOR,
                                /* Deposit or Refund Methods */
                                ACCOUNT_TRY_DEPOSIT_OR_REFUND_IDENT,
                                ACCOUNT_TRY_DEPOSIT_BATCH_OR_REFUND_IDENT,
                                /* Manipulation of the Resource Preferences */
                                ACCOUNT_SET_DEFAULT_DEPOSIT_RULE_IDENT,
                                ACCOUNT_SET_RESOURCE_PREFERENCE_IDENT,
                                ACCOUNT_REMOVE_RESOURCE_PREFERENCE_IDENT,
                                ACCOUNT_ADD_AUTHORIZED_DEPOSITOR,
                                ACCOUNT_REMOVE_AUTHORIZED_DEPOSITOR,
                                /* Deposit or Refund */
                                ACCOUNT_TRY_DEPOSIT_OR_REFUND_IDENT,
                                ACCOUNT_TRY_DEPOSIT_BATCH_OR_REFUND_IDENT,
                                /* All fee locking methods */
                                ACCOUNT_LOCK_FEE_IDENT,
                                ACCOUNT_LOCK_CONTINGENT_FEE_IDENT,
                                ACCOUNT_LOCK_FEE_AND_WITHDRAW_IDENT,
                                ACCOUNT_LOCK_FEE_AND_WITHDRAW_NON_FUNGIBLES_IDENT,
                            ],
                            default: FnRule::Disallowed,
                        },
                        EntityType::GlobalGenericComponent
                        | EntityType::GlobalIdentity
                        | EntityType::GlobalVirtualSecp256k1Identity
                        | EntityType::GlobalVirtualEd25519Identity
                        | EntityType::InternalGenericComponent => FnRules::all_allowed(),
                        /* Disallowed */
                        EntityType::GlobalPackage
                        | EntityType::GlobalValidator
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
                        | EntityType::GlobalMultiResourcePool => FnRules::all_disallowed(),
                    }
                }).unwrap_or(FnRules::all_disallowed())
            }
        }
    }
}
