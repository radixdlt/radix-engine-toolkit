//! Defines the request and response models used in an information request. This is a simple request
//! which has no arguments and returns an information response containing the current version of the
//! package. You may treat this request as a "hello world" request of sorts as it can be used to
//! check if the communication with this library is successful or not.

use crate::error::Error;
use crate::export_request;
use crate::traits::{Request, Validate};
use serde::{Deserialize, Serialize};

// ==========================
// Request & Response Models
// ==========================

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InformationRequest {}

#[derive(Serialize, Deserialize, Clone, Debug)]
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

// =======================
// Request Implementation
// =======================

impl<'r> Request<'r, InformationResponse> for InformationRequest {
    fn handle_request(self) -> Result<InformationResponse, Error> {
        Ok(InformationResponse {
            package_version: env!("CARGO_PKG_VERSION").into(),
        })
    }
}

export_request!(InformationRequest as information);

// ======
// Tests
// ======

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn information_handler_returns_ok() {
        let response: Result<InformationResponse, Error> = InformationRequest {}.fulfill_request();
        assert!(matches!(response, Ok(_)));
    }
}
