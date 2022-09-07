// TODO: Convert to use Bech32 manager

use scrypto::address::Bech32Decoder;
use transaction::manifest::ast::Instruction as AstInstruction;

use crate::address::Bech32Manager;
use crate::error::Error;
use crate::models::{
    CompileTransactionIntentRequest, ConvertManifestRequest, DecompileTransactionIntentResponse,
    Manifest, Request, Response,
};
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
        Request::CompileSignedTransactionIntentRequest(_) => Ok(()), //TODO: Implement.
        Request::DecompileSignedTransactionIntentRequest(_) => Ok(()), //TODO: Implement.
        Request::CompileNotarizedTransactionIntentRequest(_) => Ok(()), //TODO: Implement.
        Request::DecompileNotarizedTransactionIntentRequest(_) => Ok(()), //TODO: Implement.
        Request::DecompileUnknownTransactionIntentRequest(_) => Ok(()), //TODO: Implement.
    }
}

pub fn validate_convert_manifest_request(request: &ConvertManifestRequest) -> Result<(), Error> {
    validate_transaction_version(request.transaction_version)?;
    validate_manifest(&request.manifest, request.network_id)?;
    Ok(())
}

pub fn validate_compile_transaction_intent_request(
    request: &CompileTransactionIntentRequest,
) -> Result<(), Error> {
    // TODO: Add transaction intent validation through the `TransactionValidator` struct from the
    // scrypto library.
    validate_transaction_version(request.transaction_intent.header.version)?;
    validate_manifest(
        &request.transaction_intent.manifest,
        request.transaction_intent.header.network_id,
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
        Response::DecompileTransactionIntentResponse(_) => Ok(()),
        Response::CompileSignedTransactionIntentResponse(_) => Ok(()), //TODO: Implement.
        Response::DecompileSignedTransactionIntentResponse(_) => Ok(()), //TODO: Implement.
        Response::CompileNotarizedTransactionIntentResponse(_) => Ok(()), //TODO: Implement.
        Response::DecompileNotarizedTransactionIntentResponse(_) => Ok(()), //TODO: Implement.
        Response::DecompileUnknownTransactionIntentResponse(_) => Ok(()), //TODO: Implement.
    }
}

pub fn validate_decompile_transaction_intent_response(
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

pub fn validate_transaction_version(transaction_version: u8) -> Result<(), Error> {
    // Validating the transaction version provided in the request to verify that it is a supported
    // transaction version
    match transaction_version {
        transaction::model::TRANSACTION_VERSION_V1 => Ok(()),
        i => Err(Error::UnsupportedTransactionVersion(i)),
    }
}

pub fn validate_manifest(manifest: &Manifest, network_id: u8) -> Result<(), Error> {
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
