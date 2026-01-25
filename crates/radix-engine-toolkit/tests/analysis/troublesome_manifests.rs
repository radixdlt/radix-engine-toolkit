use std::sync::LazyLock;

use radix_transactions::manifest::{compile_manifest, MockBlobProvider};
use scrypto_test::prelude::*;
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct TroublesomeManifest {
    pub state_version: u64,
    pub intent_hash: IntentHash,
    pub is_committed_success: bool,
    pub error_message: Option<String>,
    pub manifest: TransactionManifestV2,
}

static TROUBLESOME_MANIFESTS: LazyLock<Vec<TroublesomeManifest>> =
    LazyLock::new(|| {
        #[derive(Deserialize)]
        struct TroublesomeManifestSerde {
            state_version: u64,
            intent_hash: String,
            receipt_status: String,
            receipt_error_message: Option<String>,
            manifest_instructions: String,
        }

        static TROUBLESOME_MANIFESTS_JSON: &str =
            include_str!("../assets/all_failed_manifests.json");

        let troublesome_manifests = serde_json::from_str::<
            Vec<TroublesomeManifestSerde>,
        >(TROUBLESOME_MANIFESTS_JSON)
        .unwrap();

        let network_definition = NetworkDefinition::mainnet();
        let bech32m_decoder =
            TransactionHashBech32Decoder::new(&network_definition);
        troublesome_manifests
            .into_iter()
            .map(
                |TroublesomeManifestSerde {
                     state_version,
                     intent_hash,
                     receipt_status,
                     receipt_error_message,
                     manifest_instructions,
                 }| TroublesomeManifest {
                    state_version,
                    intent_hash: bech32m_decoder
                        .validate_and_decode(&intent_hash)
                        .unwrap(),
                    is_committed_success: receipt_status == "succeeded",
                    error_message: receipt_error_message,
                    manifest: compile_manifest(
                        &manifest_instructions,
                        &network_definition,
                        MockBlobProvider,
                    )
                    .unwrap(),
                },
            )
            .filter(|troublesome_manifest| {
                !troublesome_manifest.manifest.instructions.iter().any(
                    |instruction| {
                        matches!(instruction, InstructionV2::YieldToChild(..))
                    },
                )
            })
            .collect()
    });

#[test]
fn all_statically_invalid_transactions_are_a_committed_failure() {
    for manifest in TROUBLESOME_MANIFESTS.iter() {
        // Act
        let static_analysis = radix_engine_toolkit::prelude::statically_analyze(
            &manifest.manifest,
        );

        // Assert
        if let Err(error) = static_analysis {
            assert!(
                !manifest.is_committed_success,
                "Expected the manifest to be a comitted failure: {} - {:?}",
                manifest.state_version, error
            )
        }
    }
}

#[test]
fn not_all_committed_failures_are_statically_invalid() {
    for manifest in TROUBLESOME_MANIFESTS.iter() {
        // Act
        let static_analysis = radix_engine_toolkit::prelude::statically_analyze(
            &manifest.manifest,
        );

        // Assert
        if static_analysis.is_ok() && !manifest.is_committed_success {
            return;
        }
    }
    panic!()
}
