//! Defines the request and response models for the convert manifest request. This request is made
//! when the client has a manifest in one format (JSON as an example) and they wish to convert
//! the manifest to another format (String as an example). The conversion between the supported
//! formats is dependent on two main factors: the transaction version, and the network id.

use crate::address::Bech32Manager;
use crate::error::Error;
use crate::export_handler;
use crate::models::manifest::{ManifestInstructions, ManifestInstructionsKind};
use crate::models::serde::TransactionManifest;
use crate::traits::Validate;
use crate::validation::{validate_manifest, validate_transaction_version};
use serde::{Deserialize, Serialize};

// ==========================
// Request & Response Models
// ==========================

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConvertManifestRequest {
    /// The version of the passed transaction manifest. Used to determine how the manifest is
    /// interpreted by the library.
    pub transaction_version: u8,

    /// The network id of the network that this transaction manifest is meant for. This is used for
    /// the Bech32 address encoding and decoding.
    pub network_id: u8,

    /// Defines the output format that we would like the manifest to be in after this request is
    /// performed.
    pub manifest_instructions_output_format: ManifestInstructionsKind,

    /// The manifest that the conversion will happen on
    pub manifest: TransactionManifest,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConvertManifestResponse {
    /// The manifest after it has been converted to the type specified in the [ConvertManifestRequest]
    #[serde(flatten)]
    pub manifest: TransactionManifest,
}

// ===========
// Validation
// ===========

impl Validate for ConvertManifestRequest {
    fn validate(&self) -> Result<(), Error> {
        validate_transaction_version(self.transaction_version)?;
        validate_manifest(&self.manifest, self.network_id)?;
        Ok(())
    }
}

impl Validate for ConvertManifestResponse {
    fn validate(&self) -> Result<(), Error> {
        Ok(())
    }
}

// ========
// Handler
// ========

pub fn handle_convert_manifest(
    request: ConvertManifestRequest,
) -> Result<ConvertManifestResponse, Error> {
    let bech32_manager: Bech32Manager = Bech32Manager::new(request.network_id);

    // Process the request Convert between the manifest formats.
    // TODO: This needs to be dependent on the version of the manifest. For now, the
    // `transaction_version` in the request is ignored.
    let converted_manifest_instructions: ManifestInstructions = request.manifest.instructions.to(
        request.manifest_instructions_output_format,
        &bech32_manager,
        request.manifest.blobs.clone(),
    )?;

    let response: ConvertManifestResponse = ConvertManifestResponse {
        manifest: TransactionManifest {
            instructions: converted_manifest_instructions,
            blobs: request.manifest.blobs,
        },
    };

    Ok(response)
}

export_handler!(handle_convert_manifest as convert_manifest);

// ======
// Tests
// ======

#[cfg(test)]
mod tests {
    // TODO: Unit tests for this request type
}
