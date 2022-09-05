use scrypto::prelude::{scrypto_decode, scrypto_encode};

use crate::error::Error;
use crate::link_handler;
use crate::models::*;
use crate::validation::{validate_request, validate_response};

// Links the extern functions to the handlers that will handle their requests.
link_handler! {
    information => handle_information,
    convert_manifest => handle_convert_manifest,

    compile_transaction_intent => handle_compile_transaction_intent,
    decompile_transaction_intent => handle_decompile_transaction_intent,

    compile_signed_transaction_intent => handle_compile_signed_transaction_intent,
    decompile_signed_transaction_intent => handle_decompile_signed_transaction_intent
}

fn handle_information(request: InformationRequest) -> Result<InformationResponse, Error> {
    // Validate the passed request
    validate_request(&request)?;

    // Process the request
    let response: InformationResponse = InformationResponse {
        package_version: env!("CARGO_PKG_VERSION").into(),
    };

    // Validate the response
    validate_response(&response)?;
    Ok(response)
}

fn handle_convert_manifest(
    request: ConvertManifestRequest,
) -> Result<ConvertManifestResponse, Error> {
    // Validate the passed request
    validate_request(&request)?;

    // Process the request Convert between the manifest formats.
    // TODO: This needs to be dependent on the version of the manifest. For now, the
    // `transaction_version` in the request is ignored.
    let converted_manifest: Manifest = request
        .manifest
        .to(request.manifest_output_format, request.network_id)?;

    let response: ConvertManifestResponse = ConvertManifestResponse {
        manifest: converted_manifest,
    };

    // Validate the response
    validate_response(&response)?;
    Ok(response)
}

fn handle_compile_transaction_intent(
    request: CompileTransactionIntentRequest,
) -> Result<CompileTransactionIntentResponse, Error> {
    // Validate the passed request
    validate_request(&request)?;

    // Convert the instructions to a transaction manifest to then create a scrypto transaction
    // intent from it.
    let manifest: transaction::model::TransactionManifest = request
        .transaction_intent
        .manifest
        .to_scrypto_transaction_manifest(request.transaction_intent.header.network_id)?;
    let transaction_intent: transaction::model::TransactionIntent =
        transaction::model::TransactionIntent {
            header: request.transaction_intent.header,
            manifest,
        };
    let compiled_intent: Vec<u8> = scrypto_encode(&transaction_intent);

    let response: CompileTransactionIntentResponse =
        CompileTransactionIntentResponse { compiled_intent };

    // Validate the response
    validate_response(&response)?;
    Ok(response)
}

fn handle_decompile_transaction_intent(
    request: DecompileTransactionIntentRequest,
) -> Result<DecompileTransactionIntentResponse, Error> {
    // Validate the passed request
    validate_request(&request)?;

    let transaction_intent: transaction::model::TransactionIntent =
        scrypto_decode(&request.compiled_intent)?;
    let manifest: Manifest = Manifest::from_scrypto_transaction_manifest(
        transaction_intent.manifest,
        transaction_intent.header.network_id,
        request.manifest_output_format,
    )?;

    let response: DecompileTransactionIntentResponse = DecompileTransactionIntentResponse {
        transaction_intent: TransactionIntent {
            header: transaction_intent.header,
            manifest: manifest,
        },
    };

    // Validate the response
    validate_response(&response)?;
    Ok(response)
}

fn handle_compile_signed_transaction_intent(
    request: CompileSignedTransactionIntentRequest,
) -> Result<CompileSignedTransactionIntentResponse, Error> {
    // Validate the passed request
    validate_request(&request)?;

    let manifest: transaction::model::TransactionManifest = request
        .transaction_intent
        .manifest
        .to_scrypto_transaction_manifest(request.transaction_intent.header.network_id)?;
    let transaction_intent: transaction::model::TransactionIntent =
        transaction::model::TransactionIntent {
            header: request.transaction_intent.header,
            manifest,
        };

    let signatures: Vec<(_, _)> = request
        .signatures
        .into_iter()
        .map(|signature| (signature.public_key, signature.signature))
        .collect();
    let signed_transaction_intent: transaction::model::SignedTransactionIntent =
        transaction::model::SignedTransactionIntent {
            intent: transaction_intent,
            intent_signatures: signatures,
        };
    let compiled_signed_intent: Vec<u8> = scrypto_encode(&signed_transaction_intent);

    let response: CompileSignedTransactionIntentResponse = CompileSignedTransactionIntentResponse {
        compiled_signed_intent,
    };

    // Validate the response
    validate_response(&response)?;
    Ok(response)
}

fn handle_decompile_signed_transaction_intent(
    request: DecompileSignedTransactionIntentRequest,
) -> Result<DecompileSignedTransactionIntentResponse, Error> {
    // Validate the passed request
    validate_request(&request)?;

    let signed_transaction_intent: transaction::model::SignedTransactionIntent =
        scrypto_decode(&request.compiled_signed_intent)?;

    let signatures: Vec<Signature> = signed_transaction_intent
        .intent_signatures
        .into_iter()
        .map(|(public_key, signature)| Signature {
            signature,
            public_key,
        })
        .collect();
    let manifest: Manifest = Manifest::from_scrypto_transaction_manifest(
        signed_transaction_intent.intent.manifest,
        signed_transaction_intent.intent.header.network_id,
        request.manifest_output_format,
    )?;

    let response: DecompileSignedTransactionIntentResponse =
        DecompileSignedTransactionIntentResponse {
            signatures,
            transaction_intent: TransactionIntent {
                header: signed_transaction_intent.intent.header,
                manifest,
            },
        };

    // Validate the response
    validate_response(&response)?;
    Ok(response)
}
