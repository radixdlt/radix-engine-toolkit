use radix_transaction::model::TransactionHeader as NativeTransactionHeader;
use radix_transaction::validation::NotarizedTransactionValidator;

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

use crate::models::{TransactionIntent, TransactionManifest};
use crate::traits::Validate;
use crate::utils::validation_config_from_header;

// =================
// Model Definition
// =================

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransactionHeader {
    #[serde_as(as = "DisplayFromStr")]
    pub version: u8,
    #[serde_as(as = "DisplayFromStr")]
    pub network_id: u8,
    #[serde_as(as = "DisplayFromStr")]
    pub start_epoch_inclusive: u64,
    #[serde_as(as = "DisplayFromStr")]
    pub end_epoch_exclusive: u64,
    #[serde_as(as = "DisplayFromStr")]
    pub nonce: u64,
    pub notary_public_key: scrypto::prelude::PublicKey,
    pub notary_as_signatory: bool,
    #[serde_as(as = "DisplayFromStr")]
    pub cost_unit_limit: u32,
    #[serde_as(as = "DisplayFromStr")]
    pub tip_percentage: u32,
}

// ============
// Conversions
// ============

impl From<NativeTransactionHeader> for TransactionHeader {
    fn from(header: NativeTransactionHeader) -> Self {
        Self {
            version: header.version,
            network_id: header.network_id,
            start_epoch_inclusive: header.start_epoch_inclusive,
            end_epoch_exclusive: header.end_epoch_exclusive,
            nonce: header.nonce,
            notary_public_key: header.notary_public_key,
            notary_as_signatory: header.notary_as_signatory,
            cost_unit_limit: header.cost_unit_limit,
            tip_percentage: header.tip_percentage,
        }
    }
}

impl From<TransactionHeader> for NativeTransactionHeader {
    fn from(header: TransactionHeader) -> Self {
        Self {
            version: header.version,
            network_id: header.network_id,
            start_epoch_inclusive: header.start_epoch_inclusive,
            end_epoch_exclusive: header.end_epoch_exclusive,
            nonce: header.nonce,
            notary_public_key: header.notary_public_key,
            notary_as_signatory: header.notary_as_signatory,
            cost_unit_limit: header.cost_unit_limit,
            tip_percentage: header.tip_percentage,
        }
    }
}

// ===========
// Validation
// ===========

impl Validate for TransactionHeader {
    fn validate(&self) -> Result<(), crate::error::Error> {
        NotarizedTransactionValidator::new(validation_config_from_header(self))
            .validate_header(
                &TransactionIntent {
                    header: self.clone(),
                    manifest: TransactionManifest {
                        instructions: crate::models::ManifestInstructions::JSON(Vec::new()),
                        blobs: Vec::new(),
                    },
                }
                .try_into()?,
            )
            .map_err(
                radix_transaction::errors::TransactionValidationError::HeaderValidationError,
            )?;
        Ok(())
    }
}
