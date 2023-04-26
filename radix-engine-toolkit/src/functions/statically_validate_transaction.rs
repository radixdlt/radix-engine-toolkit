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

use crate::model::transaction::{
    InstructionKind, NotarizedTransaction, NotarizedTransactionConversionError,
};
use crate::traits::CompilableIntent;
use native_transaction::validation::{
    NotarizedTransactionValidator, TestIntentHashManager, TransactionValidator, ValidationConfig,
};
use toolkit_derive::serializable;

use crate::functions::traits::InvocationHandler;

// =================
// Model Definition
// =================

/// Performs static validation on the given notarized transaction. This request checks that: the
/// header is valid by ensuring that all elements of the header are valid, that the signatures are
/// valid, and that the manifest is valid.
#[serializable]
pub struct Input {
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

/// The response from [`Input`].
#[serializable]
#[serde(tag = "validity")]
#[derive(PartialEq, Eq)]
pub enum Output {
    Valid,
    Invalid { error: String },
}

// ===============
// Implementation
// ===============

pub struct Handler;
impl InvocationHandler<Input, Output> for Handler {
    type Error = Error;

    fn pre_process(request: Input) -> Result<Input, Error> {
        Ok(request)
    }

    fn handle(request: &Input) -> Result<Output, Error> {
        let notarized_transaction = NotarizedTransaction::decompile(
            &request.compiled_notarized_intent,
            InstructionKind::String,
        )?;

        let intent_hash_manager = TestIntentHashManager::new();

        if let Err(ref error) = NotarizedTransactionValidator::new(request.validation_config)
            .validate(
                &notarized_transaction.to_native_notarized_transaction_intent()?,
                request.compiled_notarized_intent.len(),
                &intent_hash_manager,
            )
        {
            Ok(Output::Invalid {
                error: format!("{:?}", error),
            })
        } else {
            Ok(Output::Valid)
        }
    }

    fn post_process(_: &Input, response: Output) -> Result<Output, Error> {
        Ok(response)
    }
}

#[serializable]
#[serde(tag = "type")]
pub enum Error {
    /// An error emitted when the decompilation of the notarized transaction intent fails
    FailedToDecompileNotarizedIntent(NotarizedTransactionConversionError),
}

impl From<NotarizedTransactionConversionError> for Error {
    fn from(value: NotarizedTransactionConversionError) -> Self {
        Self::FailedToDecompileNotarizedIntent(value)
    }
}