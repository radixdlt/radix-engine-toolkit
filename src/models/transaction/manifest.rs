use radix_transaction::manifest::decompile;
use radix_transaction::manifest::generator::generate_manifest;
use radix_transaction::model::TransactionManifest as NativeTransactionManifest;

use scrypto::prelude::hash;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::address::Bech32Manager;
use crate::error::Error;
use crate::models::{ManifestInstructions, ManifestInstructionsKind};
use crate::traits::{TryIntoWithContext, ValidateWithContext};

// =================
// Model Definition
// =================

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransactionManifest {
    pub instructions: ManifestInstructions,
    #[serde_as(as = "Vec<serde_with::hex::Hex>")]
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub blobs: Vec<Vec<u8>>,
}

// ============
// Conversions
// ============

impl<T> TryIntoWithContext<NativeTransactionManifest, T> for TransactionManifest
where
    T: AsRef<Bech32Manager>,
{
    type Error = Error;

    fn try_into_with_context(
        self,
        bech32_manager: T,
    ) -> Result<NativeTransactionManifest, Self::Error> {
        let bech32_manager: &Bech32Manager = bech32_manager.as_ref();

        let transaction_manifest: NativeTransactionManifest = NativeTransactionManifest {
            instructions: self
                .instructions
                .transaction_instructions(bech32_manager, self.blobs.clone())?,
            blobs: self.blobs,
        };
        Ok(transaction_manifest)
    }
}

impl<T> TryIntoWithContext<TransactionManifest, (ManifestInstructionsKind, T)>
    for NativeTransactionManifest
where
    T: AsRef<Bech32Manager>,
{
    type Error = Error;

    fn try_into_with_context(
        self,
        (manifest_instruction_kind, bech32_manager): (ManifestInstructionsKind, T),
    ) -> Result<TransactionManifest, Self::Error> {
        let bech32_manager: &Bech32Manager = bech32_manager.as_ref();

        // Converting a TransactionInstruction to a string is rather easy to do, so we will convert
        // them into a String, and then we will convert the entire manifest instructions into the
        // requested format provided in the context.
        let manifest_instructions: ManifestInstructions = ManifestInstructions::String(decompile(
            &self.instructions,
            &bech32_manager.network_definition,
        )?);

        // Converting the manifest instructions according to the requested manifest instructions
        // kind.
        let manifest_instructions: ManifestInstructions = manifest_instructions
            .convert_to_manifest_instructions_kind(
                manifest_instruction_kind,
                bech32_manager,
                self.blobs.clone(),
            )?;

        let transaction_manifest: TransactionManifest = TransactionManifest {
            instructions: manifest_instructions,
            blobs: self.blobs,
        };
        Ok(transaction_manifest)
    }
}

// ===========
// Validation
// ===========

impl ValidateWithContext<u8> for TransactionManifest {
    fn validate(&self, network_id: u8) -> Result<(), Error> {
        let bech32_manager: Bech32Manager = Bech32Manager::new(network_id);

        self.instructions
            .instructions(&bech32_manager)?
            .iter()
            .map(|instruction| instruction.validate(network_id))
            .collect::<Result<Vec<_>, _>>()?;
        generate_manifest(
            &self.instructions.ast_instructions(&bech32_manager)?,
            &bech32_manager.decoder,
            self.blobs
                .iter()
                .map(|x| (hash(x), x.clone()))
                .collect(),
        )?;
        Ok(())
    }
}
