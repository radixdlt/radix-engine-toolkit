//! Defines the request and response models used in an information request. This is a simple request
//! which has no arguments and returns an information response containing the current version of the
//! package. You may treat this request as a "hello world" request of sorts as it can be used to
//! check if the communication with this library is successful or not.

use crate::error::Error;
use crate::export_handler;
use crate::traits::Validate;
use serde::{Deserialize, Serialize};

// ==========================
// Request & Response Models
// ==========================

#[derive(Serialize, Deserialize, Clone)]
pub struct InformationRequest {}

#[derive(Serialize, Deserialize, Clone)]
pub struct InformationResponse {
    pub package_version: String,
}

// ===========
// Validation
// ===========

impl Validate for InformationRequest {
    fn validate(&self) -> Result<(), Error> {
        Ok(())
    }
}

impl Validate for InformationResponse {
    fn validate(&self) -> Result<(), Error> {
        Ok(())
    }
}

// ========
// Handler
// ========

pub fn handle_information(_request: InformationRequest) -> Result<InformationResponse, Error> {
    let response: InformationResponse = InformationResponse {
        package_version: env!("CARGO_PKG_VERSION").into(),
    };

    Ok(response)
}

export_handler!(handle_information(InformationRequest) as information);

// ======
// Tests
// ======

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn information_handler_returns_ok() {
        let response: Result<InformationResponse, Error> =
            handle_information(InformationRequest {});
        assert!(matches!(response, Ok(_)));
    }
}
