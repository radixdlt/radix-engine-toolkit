// TODO: Convert to use Bech32 manager

use radix_engine::constants::DEFAULT_MAX_COST_UNIT_LIMIT;
use scrypto::address::Bech32Decoder;
use transaction::manifest::ast::Instruction as AstInstruction;
use transaction::validation::ValidationConfig;

use crate::address::Bech32Manager;
use crate::error::Error;
use crate::models::*;
use crate::utils::network_definition_from_network_id;

// ========
// Request
// ========

pub fn validate_request<R: Into<Request> + Clone>(request: &R) -> Result<(), Error> {
    let request: Request = request.clone().into();
    match request {
        Request::InformationRequest(_) => Ok(()),
        Request::ConvertManifestRequest(request) => validate_convert_manifest_request(&request),
        Request::CompileTransactionIntentRequest(request) => {
            validate_compile_transaction_intent_request(&request)
        }
        Request::DecompileTransactionIntentRequest(_) => Ok(()),
        Request::CompileSignedTransactionIntentRequest(request) => {
            validate_compile_signed_transaction_intent_request(&request)
        }
        Request::DecompileSignedTransactionIntentRequest(_) => Ok(()),
        Request::CompileNotarizedTransactionIntentRequest(request) => {
            validate_compile_notarized_transaction_intent_request(&request)
        }
        Request::DecompileNotarizedTransactionIntentRequest(_) => Ok(()), //TODO: Implement.
        Request::DecompileUnknownTransactionIntentRequest(_) => Ok(()),   //TODO: Implement.
        Request::DecodeAddressRequest(_) => Ok(()),                       //TODO: Implement.
        Request::EncodeAddressRequest(_) => Ok(()),                       //TODO: Implement.
    }
}

fn request_transaction_validation_config(request: &Request) -> Option<ValidationConfig> {
    match request {
        Request::InformationRequest(_) => None,
        Request::ConvertManifestRequest(request) => Some(ValidationConfig {
            network_id: request.network_id,
            current_epoch: 1,
            max_cost_unit_limit: DEFAULT_MAX_COST_UNIT_LIMIT,
            min_tip_percentage: 0,
        }),
        Request::CompileTransactionIntentRequest(request) => Some(ValidationConfig {
            network_id: request.transaction_intent.header.network_id,
            // Putting it to the epoch upper limit since we have no way of knowing what epoch it
            // currently is. So, the epoch validation is not performed by the library.
            current_epoch: request.transaction_intent.header.end_epoch_exclusive - 1,
            max_cost_unit_limit: DEFAULT_MAX_COST_UNIT_LIMIT,
            // This depends on the current state of the network which we have no way of querying
            // from the WASM. Therefore, we assume it to be zero and let this validation part be
            // handled by the client.
            min_tip_percentage: 0,
        }),
        Request::DecompileTransactionIntentRequest(_) => None,
        Request::CompileSignedTransactionIntentRequest(request) => Some(ValidationConfig {
            network_id: request.signed_intent.transaction_intent.header.network_id,
            // Putting it to the epoch upper limit since we have no way of knowing what epoch it
            // currently is. So, the epoch validation is not performed by the library.
            current_epoch: request
                .signed_intent
                .transaction_intent
                .header
                .end_epoch_exclusive
                - 1,
            max_cost_unit_limit: DEFAULT_MAX_COST_UNIT_LIMIT,
            // This depends on the current state of the network which we have no way of querying
            // from the WASM. Therefore, we assume it to be zero and let this validation part be
            // handled by the client.
            min_tip_percentage: 0,
        }),
        Request::DecompileSignedTransactionIntentRequest(_) => None,
        Request::CompileNotarizedTransactionIntentRequest(request) => Some(ValidationConfig {
            network_id: request.signed_intent.transaction_intent.header.network_id,
            // Putting it to the epoch upper limit since we have no way of knowing what epoch it
            // currently is. So, the epoch validation is not performed by the library.
            current_epoch: request
                .signed_intent
                .transaction_intent
                .header
                .end_epoch_exclusive
                - 1,
            max_cost_unit_limit: DEFAULT_MAX_COST_UNIT_LIMIT,
            // This depends on the current state of the network which we have no way of querying
            // from the WASM. Therefore, we assume it to be zero and let this validation part be
            // handled by the client.
            min_tip_percentage: 0,
        }),
        Request::DecompileNotarizedTransactionIntentRequest(_) => None,
        Request::DecompileUnknownTransactionIntentRequest(_) => None,
        Request::DecodeAddressRequest(_) => None,
        Request::EncodeAddressRequest(_) => None,
    }
}

fn validate_convert_manifest_request(request: &ConvertManifestRequest) -> Result<(), Error> {
    validate_transaction_version(request.transaction_version)?;
    validate_manifest(&request.manifest, request.network_id)?;
    Ok(())
}

fn validate_compile_transaction_intent_request(
    request: &CompileTransactionIntentRequest,
) -> Result<(), Error> {
    let bech32_manager: Bech32Manager =
        Bech32Manager::new(request.transaction_intent.header.network_id);

    let transaction_intent = transaction::model::TransactionIntent {
        header: request.transaction_intent.header.clone(),
        manifest: request
            .transaction_intent
            .manifest
            .to_scrypto_transaction_manifest(&bech32_manager)?,
    };

    validate_transaction_version(request.transaction_intent.header.version)?;
    validate_manifest(
        &request.transaction_intent.manifest,
        request.transaction_intent.header.network_id,
    )?;
    transaction::validation::TransactionValidator::validate_intent(
        &transaction_intent,
        &transaction::validation::TestIntentHashManager::new(),
        &request_transaction_validation_config(&request.clone().into())
            .expect("Obtaining the validation configuration for this request should succeed"),
    )?;
    // Signature validation requires that a notarized transaction is provided
    Ok(())
}

fn validate_compile_signed_transaction_intent_request(
    request: &CompileSignedTransactionIntentRequest,
) -> Result<(), Error> {
    validate_transaction_version(request.signed_intent.transaction_intent.header.version)?;
    validate_manifest(
        &request.signed_intent.transaction_intent.manifest,
        request.signed_intent.transaction_intent.header.network_id,
    )?;
    validate_transaction_intent(
        &request.signed_intent.transaction_intent,
        request.signed_intent.transaction_intent.header.network_id,
        &request_transaction_validation_config(&request.clone().into())
            .expect("Obtaining the validation configuration for this request should succeed"),
    )?;
    Ok(())
}

fn validate_compile_notarized_transaction_intent_request(
    request: &CompileNotarizedTransactionIntentRequest,
) -> Result<(), Error> {
    validate_transaction_version(request.signed_intent.transaction_intent.header.version)?;
    validate_manifest(
        &request.signed_intent.transaction_intent.manifest,
        request.signed_intent.transaction_intent.header.network_id,
    )?;
    validate_notarized_transaction(
        &request.signed_intent,
        &request.notary_signature,
        request.signed_intent.transaction_intent.header.network_id,
        &request_transaction_validation_config(&request.clone().into())
            .expect("Obtaining the validation configuration for this request should succeed"),
    )?;
    Ok(())
}

// =========
// Response
// =========

pub fn validate_response<R: Into<Response> + Clone>(response: &R) -> Result<(), Error> {
    let response: Response = response.clone().into();
    match response {
        Response::InformationResponse(_) => Ok(()),
        Response::ConvertManifestResponse(_) => Ok(()),
        Response::CompileTransactionIntentResponse(_) => Ok(()),
        Response::DecompileTransactionIntentResponse(response) => {
            validate_decompile_transaction_intent_response(&response)?;
            Ok(())
        },
        Response::CompileSignedTransactionIntentResponse(_) => Ok(()), //TODO: Implement.
        Response::DecompileSignedTransactionIntentResponse(_) => Ok(()), //TODO: Implement.
        Response::CompileNotarizedTransactionIntentResponse(_) => Ok(()), //TODO: Implement.
        Response::DecompileNotarizedTransactionIntentResponse(_) => Ok(()), //TODO: Implement.
        Response::DecompileUnknownTransactionIntentResponse(_) => Ok(()), //TODO: Implement.
        Response::DecodeAddressResponse(_) => Ok(()),                  //TODO: Implement.
        Response::EncodeAddressResponse(_) => Ok(()),                  //TODO: Implement.
    }
}

fn validate_decompile_transaction_intent_response(
    response: &DecompileTransactionIntentResponse,
) -> Result<(), Error> {
    // TODO: Add transaction intent validation through the `TransactionValidator` struct from the
    // scrypto library.
    validate_transaction_version(response.transaction_intent.header.version)?;
    validate_manifest(
        &response.transaction_intent.manifest,
        response.transaction_intent.header.network_id,
    )?;
    Ok(())
}

// ========
// Neither
// ========

fn validate_transaction_version(transaction_version: u8) -> Result<(), Error> {
    // Validating the transaction version provided in the request to verify that it is a supported
    // transaction version
    match transaction_version {
        transaction::model::TRANSACTION_VERSION_V1 => Ok(()),
        i => Err(Error::UnsupportedTransactionVersion(i)),
    }
}

fn validate_manifest(manifest: &Manifest, network_id: u8) -> Result<(), Error> {
    // The `generate_instruction` from the transaction::generator performs validation and converts
    // the instructions to a different format. In this case, the instruction conversion is not what
    // we are after, but the validation that it performs. If the conversion succeeds, then this
    // validation step is completed
    let ast_instructions: Vec<AstInstruction> =
        manifest.to_ast_instructions(&Bech32Manager::new(network_id))?;
    let bech32_decoder: Bech32Decoder =
        Bech32Decoder::new(&network_definition_from_network_id(network_id));
    transaction::manifest::generator::generate_manifest(&ast_instructions, &bech32_decoder)?;

    Ok(())
}

fn validate_transaction_intent(
    intent: &TransactionIntent,
    network_id: u8,
    validation_config: &ValidationConfig,
) -> Result<(), Error> {
    let bech32_manager: Bech32Manager = Bech32Manager::new(network_id);

    let transaction_intent = transaction::model::TransactionIntent {
        header: intent.header.clone(),
        manifest: intent
            .manifest
            .to_scrypto_transaction_manifest(&bech32_manager)?,
    };

    transaction::validation::TransactionValidator::validate_intent(
        &transaction_intent,
        &transaction::validation::TestIntentHashManager::new(),
        validation_config,
    )?;
    Ok(())
}

fn validate_notarized_transaction(
    signed_intent: &SignedTransactionIntent,
    notary_signature: &scrypto::prelude::EcdsaSignature,
    network_id: u8,
    validation_config: &ValidationConfig,
) -> Result<(), Error> {
    let bech32_manager: Bech32Manager = Bech32Manager::new(network_id);

    let transaction_intent = transaction::model::TransactionIntent {
        header: signed_intent.transaction_intent.header.clone(),
        manifest: signed_intent
            .transaction_intent
            .manifest
            .to_scrypto_transaction_manifest(&bech32_manager)?,
    };
    let signed_intent = transaction::model::SignedTransactionIntent {
        intent: transaction_intent.clone(),
        intent_signatures: signed_intent
            .signatures
            .clone()
            .into_iter()
            .map(|signature| (signature.public_key, signature.signature))
            .collect(),
    };
    let notarized_transaction = transaction::model::NotarizedTransaction {
        notary_signature: notary_signature.clone(),
        signed_intent,
    };

    transaction::validation::TransactionValidator::validate(
        notarized_transaction,
        &transaction::validation::TestIntentHashManager::new(),
        &validation_config,
    )?;
    Ok(())
}
