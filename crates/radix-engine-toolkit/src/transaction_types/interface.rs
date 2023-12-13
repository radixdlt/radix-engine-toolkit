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
    traverser::manifest_summary::traverse(
        &mut [
            &mut presented_proofs_detector,
            &mut encountered_entities_detector,
            &mut requiring_auth_detector,
            &mut reserved_instructions_detector,
            &mut general_transaction_detector,
            &mut transfer_transaction_detector,
            &mut pool_contribution_detector,
            &mut pool_redemption_detector,
            &mut validator_stake_detector,
            &mut validator_unstake_detector,
            &mut validator_claim_detector,
        ],
        manifest,
    );

    // Extracting the data out of the detectors and into the ManifestSummary
    let presented_proofs = presented_proofs_detector.output();
    let encountered_entities = encountered_entities_detector.output();
    let (accounts_requiring_auth, identities_requiring_auth) =
        requiring_auth_detector.output();
    let reserved_instructions = reserved_instructions_detector.output();
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
        encountered_entities,
        accounts_requiring_auth,
        identities_requiring_auth,
        reserved_instructions,
        classification,
    }
}

pub fn execution_summary(
    _manifest: &TransactionManifestV1,
    _receipt: &TransactionReceipt,
) -> Result<ExecutionSummary, TransactionTypesError> {
    todo!()
}
