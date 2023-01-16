// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use crate::error::Result;
use crate::transaction::NotarizedTransaction;
use native_transaction::validation::{
    NotarizedTransactionValidator, TestIntentHashManager, TransactionValidator, ValidationConfig,
};
use serializable::serializable;

use crate::{CompilableIntent, Handler};

// =================
// Model Definition
// =================

/// Performs static validation on the given notarized transaction. This request checks that: the
/// header is valid by ensuring that all elements of the header are valid, that the signatures are
/// valid, and that the manifest is valid.
#[serializable]
pub struct StaticallyValidateTransactionRequest {
    /// A byte array serialized as a hex string which represents the compiled notarized intent
    /// to perform static validation on.
    #[schemars(with = "String")]
    #[schemars(regex(pattern = "[0-9a-fA-F]+"))]
    #[serde_as(as = "serde_with::hex::Hex")]
    pub compiled_notarized_intent: Vec<u8>,

    /// The validation configuration which is the parameters and limits to use for the static
    /// validation
    #[schemars(with = "crate::model::transaction::ValidationConfig")]
    #[serde_as(as = "serde_with::FromInto<crate::model::transaction::ValidationConfig>")]
    pub validation_config: ValidationConfig,
}

/// The response from [`StaticallyValidateTransactionRequest`].
#[serializable]
#[serde(tag = "validity")]
pub enum StaticallyValidateTransactionResponse {
    Valid,
    Invalid { error: String },
}

// ===============
// Implementation
// ===============

pub struct StaticallyValidateTransactionHandler;

impl Handler<StaticallyValidateTransactionRequest, StaticallyValidateTransactionResponse>
    for StaticallyValidateTransactionHandler
{
    fn pre_process(
        request: StaticallyValidateTransactionRequest,
    ) -> Result<StaticallyValidateTransactionRequest> {
        Ok(request)
    }

    fn handle(
        request: &StaticallyValidateTransactionRequest,
    ) -> Result<StaticallyValidateTransactionResponse> {
        let notarized_transaction = NotarizedTransaction::decompile(
            &request.compiled_notarized_intent,
            crate::InstructionKind::String,
        )?;

        let intent_hash_manager = TestIntentHashManager::new();

        if let Err(ref error) = NotarizedTransactionValidator::new(request.validation_config)
            .validate(
                &notarized_transaction.to_native_notarized_transaction_intent()?,
                request.compiled_notarized_intent.len(),
                &intent_hash_manager,
            )
        {
            Ok(StaticallyValidateTransactionResponse::Invalid {
                error: format!("{:?}", error),
            })
        } else {
            Ok(StaticallyValidateTransactionResponse::Valid)
        }
    }

    fn post_process(
        _: &StaticallyValidateTransactionRequest,
        response: StaticallyValidateTransactionResponse,
    ) -> StaticallyValidateTransactionResponse {
        response
    }
}
