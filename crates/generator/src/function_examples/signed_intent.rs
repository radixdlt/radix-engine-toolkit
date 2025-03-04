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

use radix_engine_toolkit_json::prelude::*;
use radix_transactions::validation::ValidationConfig;

use super::manifest_provider::*;
use super::traits::HasExamples;

impl<'f> HasExamples<'f, NUMBER_OF_MANIFESTS_DOUBLE> for SignedTransactionIntentHash {
    fn example_inputs() -> [Self::Input; NUMBER_OF_MANIFESTS_DOUBLE] {
        NotarizedTransactionHash::example_inputs().map(|tx| tx.signed_intent)
    }
}

impl<'f> HasExamples<'f, NUMBER_OF_MANIFESTS_DOUBLE> for SignedIntentCompile {
    fn example_inputs() -> [Self::Input; NUMBER_OF_MANIFESTS_DOUBLE] {
        NotarizedTransactionHash::example_inputs().map(|tx| tx.signed_intent)
    }
}

impl<'f> HasExamples<'f, NUMBER_OF_MANIFESTS_DOUBLE> for SignedIntentDecompile {
    fn example_inputs() -> [Self::Input; NUMBER_OF_MANIFESTS_DOUBLE] {
        SignedIntentCompile::example_outputs().map(|output| {
            SignedIntentDecompileInput {
                compiled: output,
                instructions_kind: SerializableInstructionsKind::String,
            }
        })
    }
}

impl<'f> HasExamples<'f, NUMBER_OF_MANIFESTS_DOUBLE>
    for SignedIntentStaticallyValidate
{
    fn example_inputs() -> [Self::Input; NUMBER_OF_MANIFESTS_DOUBLE] {
        NotarizedTransactionHash::example_inputs()
            .map(|tx| tx.signed_intent)
            .map(|transaction| Self::Input {
                signed_intent: transaction,
                validation_config: ValidationConfig::default(0xf2).into(),
            })
    }
}
