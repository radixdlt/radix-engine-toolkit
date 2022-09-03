use crate::error::Error;
use crate::link_handler;
use crate::models::{InformationRequest, InformationResponse};
use crate::validation::{validate_request, validate_response};

// Links the extern functions to the handlers that will handle their requests.
link_handler! {
    information => handle_information
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
