//! Defines the request and response models used in an information request. This is a simple request
//! which has no arguments and returns an information response containing the current version of the
//! package. You may treat this request as a "hello world" request of sorts as it can be used to 
//! check if the communication with this library is successful or not.

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct InformationRequest {}

#[derive(Serialize, Deserialize, Clone)]
pub struct InformationResponse {
    pub package_version: String,
}
