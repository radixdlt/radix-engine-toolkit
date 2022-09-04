use crate::models::manifest::{Manifest, ManifestKind};
use serde::{Deserialize, Serialize};

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
    pub manifest_output_format: ManifestKind,

    /// The manifest that the conversion will happen on
    pub manifest: Manifest,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConvertManifestResponse {
    /// The manifest after it has been converted to the type specified in the [ConvertManifestRequest]
    #[serde(flatten)]
    pub manifest: Manifest,
}
