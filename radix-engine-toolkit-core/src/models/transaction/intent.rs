use radix_transaction::model::TransactionIntent as NativeTransactionIntent;
use radix_transaction::validation::{NotarizedTransactionValidator, TestIntentHashManager};
use scrypto::prelude::{scrypto_decode, scrypto_encode};

use serde::{Deserialize, Serialize};

use crate::address::Bech32Manager;
use crate::error::Error;
use crate::models::transaction::{TransactionHeader, TransactionManifest};
use crate::models::ManifestInstructionsKind;
use crate::traits::{CompilableIntent, TryIntoWithContext, Validate, ValidateWithContext};
use crate::utils::validation_config_from_header;

// =================
// Model Definition
// =================

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransactionIntent {
    pub header: TransactionHeader,
    pub manifest: TransactionManifest,
}

// ============
// Conversions
// ============

impl TryInto<NativeTransactionIntent> for TransactionIntent {
    type Error = Error;

    fn try_into(self) -> Result<NativeTransactionIntent, Self::Error> {
        let bech32_manager: Bech32Manager = Bech32Manager::new(self.header.network_id);

        let transaction_intent: NativeTransactionIntent = NativeTransactionIntent {
            header: self.header.into(),
            manifest: self.manifest.try_into_with_context(&bech32_manager)?,
        };
        Ok(transaction_intent)
    }
}

impl TryIntoWithContext<TransactionIntent, ManifestInstructionsKind> for NativeTransactionIntent {
    type Error = Error;

    fn try_into_with_context(
        self,
        manifest_output_format: ManifestInstructionsKind,
    ) -> Result<TransactionIntent, Self::Error> {
        let bech32_manager: Bech32Manager = Bech32Manager::new(self.header.network_id);

        let transaction_intent: TransactionIntent = TransactionIntent {
            header: self.header.into(),
            manifest: self
                .manifest
                .try_into_with_context((manifest_output_format, &bech32_manager))?,
        };
        Ok(transaction_intent)
    }
}

// ==============================
// Compilation and Decompilation
// ==============================

impl CompilableIntent for TransactionIntent {
    fn compile(&self) -> Result<Vec<u8>, Error> {
        // Convert the transaction intent into a native transaction intent.
        let transaction_intent: NativeTransactionIntent = self.clone().try_into()?;

        // Compile the native transaction intent
        Ok(scrypto_encode(&transaction_intent).expect("Failed to encode trusted payload"))
    }

    fn decompile<T>(
        data: &T,
        output_manifest_format: ManifestInstructionsKind,
    ) -> Result<Self, Error>
    where
        Self: Sized,
        T: AsRef<[u8]>,
    {
        // Decompile to a native transaction intent
        let data: &[u8] = data.as_ref();
        let transaction_intent: NativeTransactionIntent = scrypto_decode(data)?;

        // Convert to this type
        transaction_intent.try_into_with_context(output_manifest_format)
    }
}

// ===========
// Validation
// ===========

impl Validate for TransactionIntent {
    fn validate(&self) -> Result<(), Error> {
        self.header.validate()?;
        self.manifest.validate(self.header.network_id)?;
        NotarizedTransactionValidator::new(validation_config_from_header(&self.header))
            .validate_intent(
                &self.hash()?,
                &self.clone().try_into()?,
                &TestIntentHashManager::new(),
            )?;

        Ok(())
    }
}
