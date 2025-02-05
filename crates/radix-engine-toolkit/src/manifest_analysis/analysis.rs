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

use crate::internal_prelude::*;
use composite_analyzer::*;

pub fn statically_analyze(
    manifest: &impl ReadableManifest,
) -> Result<StaticAnalysis, ManifestAnalysisError> {
    // Analyzing the manifest using the composite visitor defined below.
    let StaticAnalyzerState {
        analyzer,
        static_permission_state,
        static_requirement_state,
    } = static_analyzer_traverse::<CompositeAnalyzer>(
        manifest,
        Default::default(),
    )?;
    let resolved_composite_output = CompositeResolvedStaticOutput::new(
        ManifestStaticAnalyzer::output(analyzer),
        &static_permission_state,
        &static_requirement_state,
    );

    // Getting the static invocation IOs from the static analyzer
    let account_static_resource_movements_summary = {
        // The initial worktop state is only unknown if the manifest is a
        // subintent manifest. Otherwise, in the case of a v1 or v2 manifest the
        // initial worktop state is known to be zero since they can't be used as
        // subintents and can't be yielded into.
        let initial_worktop_state_is_unknown = manifest.is_subintent();
        let interpreter = StaticManifestInterpreter::new(
            ValidationRuleset::babylon_equivalent(),
            manifest,
        );
        let mut visitor = StaticResourceMovementsVisitor::new(
            initial_worktop_state_is_unknown,
        );
        interpreter.validate_and_apply_visitor(&mut visitor)?;
        let output = visitor.output();
        AccountStaticResourceMovementsOutput {
            account_withdraws: output.resolve_account_withdraws(),
            account_deposits: output.resolve_account_deposits(),
        }
    };

    // Unwrapping some of the outputs that we know do exist. Note, the pattern
    // below is constructed based on knowledge of the requirements and
    // permissions of what the various analyzers allow and disallow for. If
    // these rules were to be changed (which they're unlikely to), then this
    // pattern would not be correct. I say that the rules are unlikely to change
    // since I can't ever see us changing the "account interactions" analyzer as
    // an example to have a rule or requirement other than allow all.
    let CompositeResolvedStaticOutput {
        /* Data Retrieval */
        account_interactions: Some(account_interactions_summary),
        encountered_entities: Some(entities_encountered_summary),
        entities_requiring_auth: Some(entities_requiring_auth_summary),
        presented_proofs: Some(proofs_created_summary),
        reserved_instructions: Some(reserved_instructions_summary),
        account_dynamic_resource_movements: Some(..),
        /* Manifest Classification */
        general_classification,
        general_subintent_classification,
        transfer_classification,
        simple_transfer_classification: _,
        validator_stake_classification,
        validator_unstake_classification,
        validator_claim_classification,
        pool_contribution_classification,
        pool_redemption_classification,
        account_settings_update_classification,
    } = resolved_composite_output
    else {
        unreachable!()
    };

    let static_analysis = StaticAnalysis {
        account_interactions_summary,
        account_static_resource_movements_summary,
        proofs_created_summary,
        entities_encountered_summary,
        entities_requiring_auth_summary,
        reserved_instructions_summary,
        manifest_classification: [
            general_classification.map(|_| ManifestClassification::General),
            general_subintent_classification
                .map(|_| ManifestClassification::GeneralSubintent),
            transfer_classification.map(|_| ManifestClassification::Transfer),
            validator_stake_classification
                .map(|_| ManifestClassification::ValidatorStake),
            validator_unstake_classification
                .map(|_| ManifestClassification::ValidatorUnstake),
            validator_claim_classification
                .map(|_| ManifestClassification::ValidatorClaimXrd),
            pool_contribution_classification
                .map(|_| ManifestClassification::PoolContribution),
            pool_redemption_classification
                .map(|_| ManifestClassification::PoolRedemption),
            account_settings_update_classification
                .map(|_| ManifestClassification::AccountDepositSettingsUpdate),
        ]
        .into_iter()
        .flatten()
        .collect(),
    };
    Ok(static_analysis)
}

pub fn dynamically_analyze(
    manifest: &impl ReadableManifest,
    receipt: RuntimeToolkitTransactionReceipt,
) -> Result<DynamicAnalysis, ManifestAnalysisError> {
    // Creating an analysis receipt from the runtime receipt passed to this
    // function.
    let analysis_receipt = AnalysisTransactionReceipt::new(receipt)
        .ok_or(ManifestAnalysisError::NotACommitSuccessReceipt)?;

    // Analyzing the manifest using the composite visitor defined below.
    let DynamicAnalyzerState {
        analyzer,
        static_permission_state,
        static_requirement_state,
    } = dynamic_analyzer_traverse::<CompositeAnalyzer>(
        manifest,
        &analysis_receipt,
        Default::default(),
    )?;
    let resolved_composite_output = CompositeResolvedDynamicOutput::new(
        ManifestDynamicAnalyzer::output(analyzer),
        &static_permission_state,
        &static_requirement_state,
    );

    // Getting the static invocation IOs from the static analyzer
    let account_static_resource_movements_summary = {
        // The initial worktop state is only unknown if the manifest is a
        // subintent manifest. Otherwise, in the case of a v1 or v2 manifest the
        // initial worktop state is known to be zero since they can't be used as
        // subintents and can't be yielded into.
        let initial_worktop_state_is_unknown = manifest.is_subintent();
        let interpreter = StaticManifestInterpreter::new(
            ValidationRuleset::babylon_equivalent(),
            manifest,
        );
        let mut visitor = StaticResourceMovementsVisitor::new(
            initial_worktop_state_is_unknown,
        );
        interpreter.validate_and_apply_visitor(&mut visitor)?;
        let output = visitor.output();
        AccountStaticResourceMovementsOutput {
            account_withdraws: output.resolve_account_withdraws(),
            account_deposits: output.resolve_account_deposits(),
        }
    };

    // Unwrapping some of the outputs that we know do exist. Note, the pattern
    // below is constructed based on the fact that we know which analyzers have
    // dynamic analysis implemented and which do not. If we ever implement
    // dynamic analysis for any of these types then we need to come here and
    // make a change.
    let CompositeResolvedDynamicOutput {
        /* Data Retrieval */
        account_interactions:
            Some(CombinedAnalysisOutput {
                static_analyzer_output: account_interactions_summary,
                dynamic_analyzer_output: _,
            }),
        encountered_entities:
            Some(CombinedAnalysisOutput {
                static_analyzer_output: entities_encountered_summary,
                dynamic_analyzer_output: _,
            }),
        entities_requiring_auth:
            Some(CombinedAnalysisOutput {
                static_analyzer_output: entities_requiring_auth_summary,
                dynamic_analyzer_output: _,
            }),
        presented_proofs:
            Some(CombinedAnalysisOutput {
                static_analyzer_output: proofs_created_summary,
                dynamic_analyzer_output: _,
            }),
        reserved_instructions:
            Some(CombinedAnalysisOutput {
                static_analyzer_output: reserved_instructions_summary,
                dynamic_analyzer_output: _,
            }),
        account_dynamic_resource_movements:
            Some(CombinedAnalysisOutput {
                static_analyzer_output: _,
                dynamic_analyzer_output:
                    account_dynamic_resource_movements_summary,
            }),
        /* Manifest Classification */
        general_classification,
        general_subintent_classification,
        transfer_classification,
        simple_transfer_classification,
        validator_stake_classification,
        validator_unstake_classification,
        validator_claim_classification,
        pool_contribution_classification,
        pool_redemption_classification,
        account_settings_update_classification,
    } = resolved_composite_output
    else {
        unreachable!()
    };

    let dynamic_analysis = DynamicAnalysis {
        account_interactions_summary,
        account_static_resource_movements_summary,
        account_dynamic_resource_movements_summary,
        proofs_created_summary,
        entities_newly_created_summary: analysis_receipt.new_entities_summary(),
        entities_encountered_summary,
        entities_requiring_auth_summary,
        reserved_instructions_summary,
        fee_locks_summary: analysis_receipt.fee_locks(),
        fee_consumption_summary: analysis_receipt.fee_summary(),
        detailed_manifest_classification: vec![
            general_classification.map(|_| DetailedManifestClassification::General),
            general_subintent_classification
                .map(|_| DetailedManifestClassification::GeneralSubintent),
            transfer_classification.map(|_| DetailedManifestClassification::Transfer {
                is_one_to_one_transfer: simple_transfer_classification.is_some(),
            }),
            validator_stake_classification
                .map(CombinedAnalysisOutput::into_dynamic_analyzer_output)
                .map(DetailedManifestClassification::ValidatorStake),
            validator_unstake_classification
                .map(CombinedAnalysisOutput::into_dynamic_analyzer_output)
                .map(DetailedManifestClassification::ValidatorUnstake),
            validator_claim_classification
                .map(CombinedAnalysisOutput::into_dynamic_analyzer_output)
                .map(DetailedManifestClassification::ValidatorClaimXrd),
            pool_contribution_classification
                .map(CombinedAnalysisOutput::into_dynamic_analyzer_output)
                .map(DetailedManifestClassification::PoolContribution),
            pool_redemption_classification
                .map(CombinedAnalysisOutput::into_dynamic_analyzer_output)
                .map(DetailedManifestClassification::PoolRedemption),
            account_settings_update_classification
                .map(CombinedAnalysisOutput::into_static_analyzer_output)
                .map(DetailedManifestClassification::AccountDepositSettingsUpdate),
        ]
        .into_iter()
        .flatten()
        .collect(),
    };
    Ok(dynamic_analysis)
}

/// A private module that defines an analyzer that we only use in this module
/// for the purposes of analysis.
#[allow(dead_code)]
mod composite_analyzer {
    use super::*;

    define_composite_analyzer! {
        type_ident: Composite,
        analyzers: {
            /* Data Retrieval */
            account_interactions: (
                DynamicAnalyzerWrapper<AccountInteractionsAnalyzer>,
                ()
            ),
            encountered_entities: (
                DynamicAnalyzerWrapper<EncounteredEntitiesAnalyzer>,
                ()
            ),
            entities_requiring_auth: (
                DynamicAnalyzerWrapper<EntitiesRequiringAuthAnalyzer>,
                ()
            ),
            presented_proofs: (
                DynamicAnalyzerWrapper<PresentedProofsAnalyzer>,
                ()
            ),
            reserved_instructions: (
                DynamicAnalyzerWrapper<ReservedInstructionsAnalyzer>,
                ()
            ),
            account_dynamic_resource_movements: (
                AccountDynamicResourceMovementsAnalyzer,
                ()
            ),
            /* Manifest Classification */
            general_classification: (
                DynamicAnalyzerWrapper<GeneralAnalyzer>,
                GeneralInitializer { for_subintent: false }
            ),
            general_subintent_classification: (
                DynamicAnalyzerWrapper<GeneralAnalyzer>,
                GeneralInitializer { for_subintent: true }
            ),
            transfer_classification: (
                DynamicAnalyzerWrapper<TransferAnalyzer>,
                ()
            ),
            simple_transfer_classification: (
                DynamicAnalyzerWrapper<SimpleTransferAnalyzer>,
                ()
            ),
            validator_stake_classification: (ValidatorStakeAnalyzer, ()),
            validator_unstake_classification: (ValidatorUnstakeAnalyzer, ()),
            validator_claim_classification: (ValidatorClaimAnalyzer, ()),
            pool_contribution_classification: (PoolContributionAnalyzer, ()),
            pool_redemption_classification: (PoolRedemptionAnalyzer, ()),
            account_settings_update_classification: (
                DynamicAnalyzerWrapper<AccountSettingsUpdateAnalyzer>,
                ()
            ),
        }
    }
}
