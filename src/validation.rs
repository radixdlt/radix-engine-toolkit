// TODO: Convert to use Bech32 manager

use std::convert::TryInto;

use radix_engine::constants::DEFAULT_MAX_COST_UNIT_LIMIT;
use scrypto::address::Bech32Decoder;
use transaction::manifest::ast::Instruction as AstInstruction;
use transaction::validation::ValidationConfig;

use crate::address::Bech32Manager;
use crate::error::Error;
use crate::models::serde::NotarizedTransaction;
use crate::models::*;
use crate::utils::network_definition_from_network_id;

pub fn validate_transaction_version(transaction_version: u8) -> Result<(), Error> {
    // Validating the transaction version provided in the request to verify that it is a supported
    // transaction version
    match transaction_version {
        transaction::model::TRANSACTION_VERSION_V1 => Ok(()),
        i => Err(Error::UnsupportedTransactionVersion(i)),
    }
}

pub fn validate_manifest(manifest: &TransactionManifest, network_id: u8) -> Result<(), Error> {
    // The `generate_instruction` from the transaction::generator performs validation and converts
    // the instructions to a different format. In this case, the instruction conversion is not what
    // we are after, but the validation that it performs. If the conversion succeeds, then this
    // validation step is completed
    let ast_instructions: Vec<AstInstruction> = manifest
        .instructions
        .to_ast_instructions(&Bech32Manager::new(network_id))?;
    let bech32_decoder: Bech32Decoder =
        Bech32Decoder::new(&network_definition_from_network_id(network_id));
    transaction::manifest::generator::generate_manifest(
        &ast_instructions,
        &bech32_decoder,
        manifest
            .blobs
            .iter()
            .map(|x| (radix_engine::types::hash(x), x.clone()))
            .collect(),
    )?;
    manifest
        .instructions
        .to_instructions(&Bech32Manager::new(network_id))?
        .iter()
        .map(|x| x.validate_instruction_argument_network(network_id))
        .collect::<Result<Vec<_>, _>>()?;
    manifest
        .instructions
        .to_instructions(&Bech32Manager::new(network_id))?
        .iter()
        .map(|x| x.validate_instruction_argument_kind())
        .collect::<Result<Vec<_>, _>>()?;
    Ok(())
}

pub fn validate_transaction_intent(intent: &TransactionIntent) -> Result<(), Error> {
    let network_id: u8 = intent.header.network_id;
    let end_epoch: u64 = intent.header.end_epoch_exclusive;
    let transaction_version: u8 = intent.header.version;

    validate_transaction_version(transaction_version)?;
    validate_manifest(&intent.manifest, network_id)?;

    let validation_config: ValidationConfig = new_validation_config(network_id, end_epoch);
    let transaction_intent: transaction::model::TransactionIntent = intent.clone().try_into()?;

    transaction::validation::TransactionValidator::validate_intent(
        &transaction_intent,
        &transaction::validation::TestIntentHashManager::new(),
        &validation_config,
    )?;
    Ok(())
}

pub fn validate_notarized_transaction(
    notarized_transaction: &NotarizedTransaction,
) -> Result<(), Error> {
    let network_id: u8 = notarized_transaction
        .signed_intent
        .transaction_intent
        .header
        .network_id;
    let end_epoch: u64 = notarized_transaction
        .signed_intent
        .transaction_intent
        .header
        .end_epoch_exclusive;

    let transaction_intent: transaction::model::TransactionIntent = notarized_transaction
        .signed_intent
        .transaction_intent
        .clone()
        .try_into()?;
    let signed_intent = transaction::model::SignedTransactionIntent {
        intent: transaction_intent.clone(),
        intent_signatures: notarized_transaction.signed_intent.signatures.clone(),
    };
    validate_transaction_intent(&notarized_transaction.signed_intent.transaction_intent)?;
    let notarized_transaction = transaction::model::NotarizedTransaction {
        notary_signature: notarized_transaction.notary_signature.clone(),
        signed_intent,
    };

    let validation_config: ValidationConfig = new_validation_config(network_id, end_epoch);
    transaction::validation::TransactionValidator::validate(
        notarized_transaction,
        &transaction::validation::TestIntentHashManager::new(),
        &validation_config,
    )?;
    Ok(())
}

fn new_validation_config(network_id: u8, end_epoch: u64) -> ValidationConfig {
    ValidationConfig {
        network_id: network_id,
        current_epoch: end_epoch - 1,
        max_cost_unit_limit: DEFAULT_MAX_COST_UNIT_LIMIT,
        min_tip_percentage: 0,
    }
}
