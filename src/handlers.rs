use crate::error::Error;
use crate::link_handler;
use crate::models::{
    ConvertManifestRequest, ConvertManifestResponse, InformationRequest, InformationResponse,
    Manifest, ManifestKind,
};
use crate::validation::{validate_request, validate_response};

// Links the extern functions to the handlers that will handle their requests.
link_handler! {
    information => handle_information,
    convert_manifest => handle_convert_manifest
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
    let network_id: u8 = request.network_id;
    let converted_manifest: Manifest = {
        match request.manifest_output_format {
            ManifestKind::JSON => request.manifest.to_json_manifest(network_id)?,
            ManifestKind::String => request.manifest.to_string_manifest(network_id)?,
        }
    };

    let response: ConvertManifestResponse = ConvertManifestResponse {
        manifest: converted_manifest,
    };

    // Validate the response
    validate_response(&response)?;
    Ok(response)
}
