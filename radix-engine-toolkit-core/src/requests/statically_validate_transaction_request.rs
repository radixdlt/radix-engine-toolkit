// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at

//   http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use crate::error::Error;
use crate::model::{ManifestInstructionsKind, NotarizedTransaction, ValidationConfigProxy};
use crate::traits::{CompilableIntent, Request, Validate};

use radix_transaction::validation::{
    NotarizedTransactionValidator, TestIntentHashManager, TransactionValidator, ValidationConfig,
};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, FromInto};

// ==========================
// Request & Response Models
// ==========================

#[serde_as]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StaticallyValidateTransactionRequest {
    #[serde_as(as = "serde_with::hex::Hex")]
    pub compiled_notarized_intent: Vec<u8>,

    #[serde_as(as = "FromInto<ValidationConfigProxy>")]
    pub validation_config: ValidationConfig,
}

#[serde_as]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "validity")]
pub enum StaticallyValidateTransactionResponse {
    Valid,
    Invalid { error: String },
}

// ===========
// Validation
// ===========

impl Validate for StaticallyValidateTransactionRequest {
    fn validate(&self) -> Result<(), Error> {
        Ok(())
    }
}

impl Validate for StaticallyValidateTransactionResponse {
    fn validate(&self) -> Result<(), Error> {
        Ok(())
    }
}

// =======================
// Request Implementation
// =======================

impl<'r> Request<'r, StaticallyValidateTransactionResponse>
    for StaticallyValidateTransactionRequest
{
    fn handle_request(self) -> Result<StaticallyValidateTransactionResponse, Error> {
        let notarized_transaction = NotarizedTransaction::decompile(
            &self.compiled_notarized_intent,
            ManifestInstructionsKind::String,
        )?;

        let intent_hash_manager = TestIntentHashManager::new();

        if let Err(ref error) = NotarizedTransactionValidator::new(self.validation_config)
            .validate(&notarized_transaction.try_into()?, &intent_hash_manager)
        {
            Ok(StaticallyValidateTransactionResponse::Invalid {
                error: format!("{:?}", error),
            })
        } else {
            Ok(StaticallyValidateTransactionResponse::Valid)
        }
    }
}
