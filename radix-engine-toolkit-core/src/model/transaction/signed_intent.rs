use radix_transaction::model::SignedTransactionIntent as NativeSignedTransactionIntent;
use radix_transaction::validation::{NotarizedTransactionValidator, TestIntentHashManager};
use scrypto::prelude::{scrypto_decode, scrypto_encode, SignatureWithPublicKey};

use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::model::transaction::TransactionIntent;
use crate::model::ManifestInstructionsKind;
use crate::traits::{CompilableIntent, TryIntoWithContext, Validate, ValidateWithContext};
use crate::utils::validation_config_from_header;

// =================
// Model Definition
// =================

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SignedTransactionIntent {
    pub intent: TransactionIntent,
    pub intent_signatures: Vec<SignatureWithPublicKey>,
}

// ============
// Conversions
// ============

impl TryInto<NativeSignedTransactionIntent> for SignedTransactionIntent {
    type Error = Error;

    fn try_into(self) -> Result<NativeSignedTransactionIntent, Self::Error> {
        let signed_transaction_intent = NativeSignedTransactionIntent {
            intent: self.intent.try_into()?,
            intent_signatures: self.intent_signatures,
        };
        Ok(signed_transaction_intent)
    }
}

impl TryIntoWithContext<SignedTransactionIntent, ManifestInstructionsKind>
    for NativeSignedTransactionIntent
{
    type Error = Error;

    fn try_into_with_context(
        self,
        manifest_output_format: ManifestInstructionsKind,
    ) -> Result<SignedTransactionIntent, Self::Error> {
        let signed_transaction_intent = SignedTransactionIntent {
            intent: self.intent.try_into_with_context(manifest_output_format)?,
            intent_signatures: self.intent_signatures,
        };
        Ok(signed_transaction_intent)
    }
}

// ==============================
// Compilation and Decompilation
// ==============================

impl CompilableIntent for SignedTransactionIntent {
    fn compile(&self) -> Result<Vec<u8>, Error> {
        // Convert the signed transaction intent into a native signed transaction intent.
        let signed_transaction_intent: NativeSignedTransactionIntent = self.clone().try_into()?;

        // Compile the native signed transaction intent
        Ok(scrypto_encode(&signed_transaction_intent)?)
    }

    fn decompile<T>(
        data: &T,
        output_manifest_format: ManifestInstructionsKind,
    ) -> Result<Self, Error>
    where
        Self: Sized,
        T: AsRef<[u8]>,
    {
        // Decompile to a native signed transaction intent
        let data = data.as_ref();
        let signed_transaction_intent = scrypto_decode::<NativeSignedTransactionIntent>(data)?;

        // Convert to this type
        signed_transaction_intent.try_into_with_context(output_manifest_format)
    }
}

// ===========
// Validation
// ===========

impl Validate for SignedTransactionIntent {
    fn validate(&self) -> Result<(), Error> {
        self.intent.header.validate()?;
        self.intent
            .manifest
            .validate(self.intent.header.network_id)?;
        NotarizedTransactionValidator::new(validation_config_from_header(&self.intent.header))
            .validate_intent(
                &self.hash()?,
                &self.intent.clone().try_into()?,
                &TestIntentHashManager::new(),
            )?;

        Ok(())
    }
}
