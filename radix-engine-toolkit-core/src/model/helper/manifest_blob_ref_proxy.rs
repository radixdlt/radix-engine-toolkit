use scrypto::{prelude::Hash, runtime::ManifestBlobRef};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct ManifestBlobRefProxy(#[serde_as(as = "serde_with::hex::Hex")] [u8; 32]);

impl From<ManifestBlobRef> for ManifestBlobRefProxy {
    fn from(value: ManifestBlobRef) -> Self {
        Self(value.0 .0)
    }
}

impl From<ManifestBlobRefProxy> for ManifestBlobRef {
    fn from(value: ManifestBlobRefProxy) -> Self {
        Self(Hash(value.0))
    }
}
