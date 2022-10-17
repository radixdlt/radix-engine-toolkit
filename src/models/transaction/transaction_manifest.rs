use radix_transaction::model::TransactionManifest as NativeTransactionManifest;

use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::error::Error;
use crate::models::{ManifestInstructions, ManifestInstructionsKind};
use crate::traits::TryIntoWithContext;

// // =================
// // Model Definition
// // =================

// #[serde_as]
// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct TransactionManifest {
//     pub instructions: ManifestInstructions,
//     #[serde_as(as = "Vec<serde_with::hex::Hex>")]
//     #[serde(skip_serializing_if = "Vec::is_empty", default)]
//     pub blobs: Vec<Vec<u8>>,
// }

// // ============
// // Conversions
// // ============

// impl TryIntoWithContext<TransactionManifest, ManifestInstructionsKind>
//     for NativeTransactionManifest
// {
//     type Error = Error;

//     fn try_into_with_context(
//         self,
//         context: ManifestInstructionsKind,
//     ) -> Result<TransactionManifest, Self::Error> {

//     }
// }

// // =====
// // SBOR
// // =====
