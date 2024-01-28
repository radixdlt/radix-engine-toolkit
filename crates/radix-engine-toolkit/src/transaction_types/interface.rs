// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

//! Functions that expose the transaction types functionality without exposing
//! any of the implementation details of how the module finds and determines
//! the transaction types.

use radix_engine::prelude::*;
use radix_engine::transaction::*;
use transaction::prelude::*;

use crate::transaction_types::*;

use super::error::*;
use super::types::*;

pub fn summary(manifest: &TransactionManifestV1) -> ManifestSummary {
    // Settings up the various detectors
    let mut presented_proofs_detector = PresentedProofsDetector::default();
    let mut encountered_entities_detector =
        EncounteredGlobalEntities::default();
    let mut requiring_auth_detector = RequiringAuthDetector::default();
    let mut reserved_instructions_detector =
        ReservedInstructionsDetector::default();
    let mut account_resource_movements_detector =
        StaticAccountResourceMovementsDetector::default();

    let mut general_transaction_detector = GeneralDetector::default();
    let mut transfer_transaction_detector = TransferDetector::default();
    let mut pool_contribution_detector = PoolContributionDetector::default();
    let mut pool_redemption_detector = PoolRedemptionDetector::default();
    let mut validator_stake_detector = ValidatorStakeDetector::default();
    let mut validator_unstake_detector = ValidatorUnstakeDetector::default();
    let mut validator_claim_detector = ValidatorClaimDetector::default();
    let mut accounts_settings_detector =
        AccountSettingsUpdateDetector::default();
    let mut trusted_worktop = TrustedWorktop::default();

    // Traversing the manifest with the passed detectors
    traverser::manifest_summary::traverse(
        &mut [
            &mut presented_proofs_detector,
            &mut encountered_entities_detector,
            &mut requiring_auth_detector,
            &mut reserved_instructions_detector,
            &mut account_resource_movements_detector,
            &mut general_transaction_detector,
            &mut transfer_transaction_detector,
            &mut pool_contribution_detector,
            &mut pool_redemption_detector,
            &mut validator_stake_detector,
            &mut validator_unstake_detector,
            &mut validator_claim_detector,
            &mut accounts_settings_detector,
            &mut trusted_worktop,
        ],
        manifest,
    );

    // Extracting the data out of the detectors and into the ManifestSummary
    let presented_proofs = presented_proofs_detector.output();
    let encountered_entities = encountered_entities_detector.output();
    let (accounts_requiring_auth, identities_requiring_auth) =
        requiring_auth_detector.output();
    let reserved_instructions = reserved_instructions_detector.output();
    let (account_withdraws, account_deposits) =
        account_resource_movements_detector.output();
    let classification = [
        (
            ManifestClass::General,
            general_transaction_detector.is_valid(),
        ),
        (
            ManifestClass::Transfer,
            transfer_transaction_detector.is_valid(),
        ),
        (
            ManifestClass::PoolContribution,
            pool_contribution_detector.is_valid(),
        ),
        (
            ManifestClass::PoolRedemption,
            pool_redemption_detector.is_valid(),
        ),
        (
            ManifestClass::ValidatorStake,
            validator_stake_detector.is_valid(),
        ),
        (
            ManifestClass::ValidatorUnstake,
            validator_unstake_detector.is_valid(),
        ),
        (
            ManifestClass::ValidatorClaim,
            validator_claim_detector.is_valid(),
        ),
        (
            ManifestClass::AccountDepositSettingsUpdate,
            accounts_settings_detector.is_valid(),
        ),
    ]
    .into_iter()
    .filter_map(
        |(class, is_valid)| {
            if is_valid {
                Some(class)
            } else {
                None
            }
        },
    )
    .rev()
    .collect::<IndexSet<ManifestClass>>();

    ManifestSummary {
        presented_proofs,
        accounts_withdrawn_from: account_withdraws,
        accounts_deposited_into: account_deposits,
        encountered_entities,
        accounts_requiring_auth,
        identities_requiring_auth,
        reserved_instructions,
        classification,
    }
}

pub fn execution_summary(
    manifest: &TransactionManifestV1,
    receipt: &TransactionReceipt,
) -> Result<ExecutionSummary, TransactionTypesError> {
    // Attempt to create a tx types receipt from the passed receipt
    let receipt = TransactionTypesReceipt::new(receipt)
        .ok_or(TransactionTypesError::InvalidReceipt)?;

    // Settings up the various detectors
    let mut presented_proofs_detector = PresentedProofsDetector::default();
    let mut encountered_entities_detector =
        EncounteredGlobalEntities::default();
    let mut requiring_auth_detector = RequiringAuthDetector::default();
    let mut reserved_instructions_detector =
        ReservedInstructionsDetector::default();
    let mut account_resource_movements_detector =
        AccountResourceMovementsDetector::default();
    let newly_created_non_fungibles = receipt.new_non_fungibles();

    let mut general_transaction_detector = GeneralDetector::default();
    let mut transfer_transaction_detector = TransferDetector::default();
    let mut pool_contribution_detector = PoolContributionDetector::default();
    let mut pool_redemption_detector = PoolRedemptionDetector::default();
    let mut validator_stake_detector = ValidatorStakeDetector::default();
    let mut validator_unstake_detector = ValidatorUnstakeDetector::default();
    let mut validator_claim_detector = ValidatorClaimDetector::default();
    let mut accounts_settings_detector =
        AccountSettingsUpdateDetector::default();

    // Traversing the manifest with the passed detectors
    traverser::execution_summary::traverse(
        &mut [
            &mut presented_proofs_detector,
            &mut encountered_entities_detector,
            &mut requiring_auth_detector,
            &mut reserved_instructions_detector,
            &mut account_resource_movements_detector,
            &mut general_transaction_detector,
            &mut transfer_transaction_detector,
            &mut pool_contribution_detector,
            &mut pool_redemption_detector,
            &mut validator_stake_detector,
            &mut validator_unstake_detector,
            &mut validator_claim_detector,
            &mut accounts_settings_detector,
        ],
        manifest,
        &receipt,
    );

    // Extracting the data into an ExecutionSummary
    let (account_withdraws, account_deposits) =
        account_resource_movements_detector.output();
    let presented_proofs = presented_proofs_detector.output();
    let new_entities = NewEntities {
        component_addresses: receipt.new_components().clone(),
        resource_addresses: receipt.new_resources().clone(),
        package_addresses: receipt.new_packages().clone(),
        metadata: receipt.metadata_of_new_entities(),
    };
    let encountered_entities = encountered_entities_detector.output();
    let (accounts_requiring_auth, identities_requiring_auth) =
        requiring_auth_detector.output();
    let reserved_instructions = reserved_instructions_detector.output();

    let detailed_classification = [
        general_transaction_detector
            .output()
            .map(|_| DetailedManifestClass::General),
        transfer_transaction_detector.output().map(|is_one_to_one| {
            DetailedManifestClass::Transfer { is_one_to_one }
        }),
        pool_contribution_detector.output().map(
            |(pool_addresses, pool_contributions)| {
                DetailedManifestClass::PoolContribution {
                    pool_addresses,
                    pool_contributions,
                }
            },
        ),
        pool_redemption_detector.output().map(
            |(pool_addresses, pool_redemptions)| {
                DetailedManifestClass::PoolRedemption {
                    pool_addresses,
                    pool_redemptions,
                }
            },
        ),
        validator_stake_detector.output().map(
            |(validator_addresses, validator_stakes)| {
                DetailedManifestClass::ValidatorStake {
                    validator_addresses,
                    validator_stakes,
                }
            },
        ),
        validator_unstake_detector.output().map(
            |(validator_addresses, validator_unstakes)| {
                DetailedManifestClass::ValidatorUnstake {
                    validator_addresses,
                    validator_unstakes,
                }
            },
        ),
        validator_claim_detector.output().map(
            |(validator_addresses, validator_claims)| {
                DetailedManifestClass::ValidatorClaim {
                    validator_addresses,
                    validator_claims,
                }
            },
        ),
        accounts_settings_detector.output().map(
            |(
                resource_preferences_updates,
                deposit_mode_updates,
                authorized_depositors_updates,
            )| {
                DetailedManifestClass::AccountDepositSettingsUpdate {
                    resource_preferences_updates,
                    deposit_mode_updates,
                    authorized_depositors_updates:
                        authorized_depositors_updates
                            .into_iter()
                            .map(|(k, v)| {
                                (
                                    k,
                                    v.into_iter()
                                        .map(|(badge, operation)| {
                                            (
                                                badge,
                                                match operation {
                                                    Update::Set(()) => {
                                                        Operation::Added
                                                    }
                                                    Update::Remove => {
                                                        Operation::Removed
                                                    }
                                                },
                                            )
                                        })
                                        .collect(),
                                )
                            })
                            .collect(),
                }
            },
        ),
    ]
    .into_iter()
    .flatten()
    .rev()
    .collect::<Vec<DetailedManifestClass>>();

    let fee_locks = FeeLocks {
        lock: receipt.execution_trace().fee_locks.lock,
        contingent_lock: receipt.execution_trace().fee_locks.contingent_lock,
    };
    let fee_summary = FeeSummary {
        execution_cost: receipt.fee_summary.total_execution_cost_in_xrd,
        finalization_cost: receipt.fee_summary.total_finalization_cost_in_xrd,
        storage_expansion_cost: receipt.fee_summary.total_storage_cost_in_xrd,
        royalty_cost: receipt.fee_summary.total_royalty_cost_in_xrd,
    };

    Ok(ExecutionSummary {
        account_withdraws,
        account_deposits,
        presented_proofs,
        new_entities,
        encountered_entities,
        accounts_requiring_auth,
        identities_requiring_auth,
        reserved_instructions,
        fee_locks,
        fee_summary,
        detailed_classification,
        newly_created_non_fungibles,
    })
}
