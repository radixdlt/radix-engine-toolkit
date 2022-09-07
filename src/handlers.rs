use scrypto::prelude::{scrypto_decode, scrypto_encode};

use crate::address::Bech32Manager;
use crate::error::Error;
use crate::link_handler;
use crate::models::*;

// Links the extern functions to the handlers that will handle their requests.
link_handler! {
    information => handle_information,
    convert_manifest => handle_convert_manifest,

    compile_transaction_intent => handle_compile_transaction_intent,
    decompile_transaction_intent => handle_decompile_transaction_intent,

    compile_signed_transaction_intent => handle_compile_signed_transaction_intent,
    decompile_signed_transaction_intent => handle_decompile_signed_transaction_intent,

    compile_notarized_transaction_intent => handle_compile_notarized_transaction_intent,
    decompile_notarized_transaction_intent => handle_decompile_notarized_transaction_intent,

    decompile_unknown_transaction_intent => handle_decompile_unknown_transaction_intent
}

fn handle_information(_request: InformationRequest) -> Result<InformationResponse, Error> {
    // Process the request
    let response: InformationResponse = InformationResponse {
        package_version: env!("CARGO_PKG_VERSION").into(),
    };

    // Return the response
    Ok(response)
}

fn handle_convert_manifest(
    request: ConvertManifestRequest,
) -> Result<ConvertManifestResponse, Error> {
    let bech32_manager: Bech32Manager = Bech32Manager::new(request.network_id);

    // Process the request Convert between the manifest formats.
    // TODO: This needs to be dependent on the version of the manifest. For now, the
    // `transaction_version` in the request is ignored.
    let converted_manifest: Manifest = request
        .manifest
        .to(request.manifest_output_format, &bech32_manager)?;

    let response: ConvertManifestResponse = ConvertManifestResponse {
        manifest: converted_manifest,
    };

    // Return the response
    Ok(response)
}

fn handle_compile_transaction_intent(
    request: CompileTransactionIntentRequest,
) -> Result<CompileTransactionIntentResponse, Error> {
    let bech32_manager: Bech32Manager =
        Bech32Manager::new(request.transaction_intent.header.network_id);

    // Convert the instructions to a transaction manifest to then create a scrypto transaction
    // intent from it.
    let manifest: transaction::model::TransactionManifest = request
        .transaction_intent
        .manifest
        .to_scrypto_transaction_manifest(&bech32_manager)?;
    let transaction_intent: transaction::model::TransactionIntent =
        transaction::model::TransactionIntent {
            header: request.transaction_intent.header,
            manifest,
        };
    let compiled_intent: Vec<u8> = scrypto_encode(&transaction_intent);

    let response: CompileTransactionIntentResponse =
        CompileTransactionIntentResponse { compiled_intent };

    // Return the response
    Ok(response)
}

fn handle_decompile_transaction_intent(
    request: DecompileTransactionIntentRequest,
) -> Result<DecompileTransactionIntentResponse, Error> {
    let transaction_intent: transaction::model::TransactionIntent =
        scrypto_decode(&request.compiled_intent)?;
    let manifest: Manifest = Manifest::from_scrypto_transaction_manifest(
        transaction_intent.manifest,
        &Bech32Manager::new(transaction_intent.header.network_id),
        request.manifest_output_format,
    )?;

    let response: DecompileTransactionIntentResponse = DecompileTransactionIntentResponse {
        transaction_intent: TransactionIntent {
            header: transaction_intent.header,
            manifest: manifest,
        },
    };

    // Return the response
    Ok(response)
}

fn handle_compile_signed_transaction_intent(
    request: CompileSignedTransactionIntentRequest,
) -> Result<CompileSignedTransactionIntentResponse, Error> {
    let bech32_manager: Bech32Manager =
        Bech32Manager::new(request.signed_intent.transaction_intent.header.network_id);

    let manifest: transaction::model::TransactionManifest = request
        .signed_intent
        .transaction_intent
        .manifest
        .to_scrypto_transaction_manifest(&bech32_manager)?;
    let transaction_intent: transaction::model::TransactionIntent =
        transaction::model::TransactionIntent {
            header: request.signed_intent.transaction_intent.header,
            manifest,
        };

    let signatures: Vec<(_, _)> = request
        .signed_intent
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

    // Return the response
    Ok(response)
}

fn handle_decompile_signed_transaction_intent(
    request: DecompileSignedTransactionIntentRequest,
) -> Result<DecompileSignedTransactionIntentResponse, Error> {
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
        &Bech32Manager::new(signed_transaction_intent.intent.header.network_id),
        request.manifest_output_format,
    )?;

    let response: DecompileSignedTransactionIntentResponse =
        DecompileSignedTransactionIntentResponse {
            signed_intent: SignedTransactionIntent {
                signatures,
                transaction_intent: TransactionIntent {
                    header: signed_transaction_intent.intent.header,
                    manifest,
                },
            },
        };

    // Return the response
    Ok(response)
}

fn handle_compile_notarized_transaction_intent(
    request: CompileNotarizedTransactionIntentRequest,
) -> Result<CompileNotarizedTransactionIntentResponse, Error> {
    let bech32_manager: Bech32Manager =
        Bech32Manager::new(request.signed_intent.transaction_intent.header.network_id);

    let manifest: transaction::model::TransactionManifest = request
        .signed_intent
        .transaction_intent
        .manifest
        .to_scrypto_transaction_manifest(&bech32_manager)?;
    let transaction_intent: transaction::model::TransactionIntent =
        transaction::model::TransactionIntent {
            header: request.signed_intent.transaction_intent.header,
            manifest,
        };

    let signatures: Vec<(_, _)> = request
        .signed_intent
        .signatures
        .into_iter()
        .map(|signature| (signature.public_key, signature.signature))
        .collect();
    let notarized_transaction: transaction::model::NotarizedTransaction =
        transaction::model::NotarizedTransaction {
            signed_intent: transaction::model::SignedTransactionIntent {
                intent: transaction_intent,
                intent_signatures: signatures,
            },
            notary_signature: request.notary_signature,
        };
    let compiled_notarized_intent: Vec<u8> = scrypto_encode(&notarized_transaction);

    let response: CompileNotarizedTransactionIntentResponse =
        CompileNotarizedTransactionIntentResponse {
            compiled_notarized_intent,
        };

    // Return the response
    Ok(response)
}

fn handle_decompile_notarized_transaction_intent(
    request: DecompileNotarizedTransactionIntentRequest,
) -> Result<DecompileNotarizedTransactionIntentResponse, Error> {
    let notarized_transaction_intent: transaction::model::NotarizedTransaction =
        scrypto_decode(&request.compiled_notarized_intent)?;

    let signatures: Vec<Signature> = notarized_transaction_intent
        .signed_intent
        .intent_signatures
        .into_iter()
        .map(|(public_key, signature)| Signature {
            signature,
            public_key,
        })
        .collect();
    let manifest: Manifest = Manifest::from_scrypto_transaction_manifest(
        notarized_transaction_intent.signed_intent.intent.manifest,
        &Bech32Manager::new(
            notarized_transaction_intent
                .signed_intent
                .intent
                .header
                .network_id,
        ),
        request.manifest_output_format,
    )?;

    let response: DecompileNotarizedTransactionIntentResponse =
        DecompileNotarizedTransactionIntentResponse {
            signed_intent: SignedTransactionIntent {
                signatures,
                transaction_intent: TransactionIntent {
                    header: notarized_transaction_intent.signed_intent.intent.header,
                    manifest,
                },
            },
            notary_signature: notarized_transaction_intent.notary_signature,
        };

    // Return the response
    Ok(response)
}

fn handle_decompile_unknown_transaction_intent(
    request: DecompileUnknownTransactionIntentRequest,
) -> Result<DecompileUnknownTransactionIntentResponse, Error> {
    let response: DecompileUnknownTransactionIntentResponse = if let Ok(response) =
        handle_decompile_transaction_intent(request.clone().into())
    {
        Ok(response.into())
    } else if let Ok(response) = handle_decompile_signed_transaction_intent(request.clone().into())
    {
        Ok(response.into())
    } else if let Ok(response) = handle_decompile_notarized_transaction_intent(request.into()) {
        Ok(response.into())
    } else {
        Err(Error::UnrecognizedCompiledIntentFormat)
    }?;

    // Return the response
    Ok(response)
}
