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

use crate::function_examples::notarized_transaction::notarized_transactions;
use radix_engine_toolkit::prelude::*;

use super::traits::HasExamples;

impl<'f> HasExamples<'f> for SerializableNotarizedTransaction {
    fn examples() -> Vec<Self> {
        notarized_transactions().to_vec()
    }
}

impl<'f> HasExamples<'f> for SerializableSignedIntent {
    fn examples() -> Vec<Self> {
        notarized_transactions()
            .iter()
            .cloned()
            .map(|tx| tx.signed_intent)
            .collect()
    }
}

impl<'f> HasExamples<'f> for SerializableIntent {
    fn examples() -> Vec<Self> {
        notarized_transactions()
            .iter()
            .cloned()
            .map(|tx| tx.signed_intent.intent)
            .collect()
    }
}

impl<'f> HasExamples<'f> for SerializableTransactionManifest {
    fn examples() -> Vec<Self> {
        notarized_transactions()
            .iter()
            .cloned()
            .map(|tx| tx.signed_intent.intent.manifest)
            .collect()
    }
}

impl<'f> HasExamples<'f> for SerializableInstructions {
    fn examples() -> Vec<Self> {
        notarized_transactions()
            .iter()
            .cloned()
            .map(|tx| tx.signed_intent.intent.manifest.instructions)
            .collect()
    }
}
