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

use crate::prelude::*;
use radix_transactions::model::TransactionPayload;

//==================================
// Preview Transaction V2 Compile
//==================================

#[typeshare::typeshare]
pub type PreviewTransactionV2CompileInput = SerializablePreviewTransactionV2;
#[typeshare::typeshare]
pub type PreviewTransactionV2CompileOutput = SerializableBytes;

pub struct PreviewTransactionV2Compile;
impl<'f> Function<'f> for PreviewTransactionV2Compile {
    type Input = PreviewTransactionV2CompileInput;
    type Output = PreviewTransactionV2CompileOutput;

    fn handle(
        preview_transaction: Self::Input,
    ) -> Result<Self::Output, crate::error::InvocationHandlingError> {
        let network_id = *preview_transaction
            .transaction_intent
            .root_intent_core
            .header
            .network_id;
        let preview_transaction = preview_transaction.to_native(network_id)?;
        let compile = preview_transaction
            .to_raw()
            .map(|raw| raw.to_vec())
            .map_err(|error| {
                InvocationHandlingError::EncodeError(
                    debug_string(error),
                    debug_string(preview_transaction),
                )
            })?;
        Ok(compile.into())
    }
}

export_function!(PreviewTransactionV2Compile as preview_transaction_v2_compile);
export_jni_function!(
    PreviewTransactionV2Compile as previewTransactionV2Compile
);
