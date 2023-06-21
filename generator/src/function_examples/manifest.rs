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

use radix_engine_toolkit::prelude::*;

use super::manifest_provider::*;
use super::traits::HasExamples;

impl<'f> HasExamples<'f, NUMBER_OF_MANIFESTS_DOUBLE> for ManifestHash {
    fn example_inputs() -> [Self::Input; NUMBER_OF_MANIFESTS_DOUBLE] {
        NotarizedTransactionHash::example_inputs()
            .map(|tx| tx.notarized_transaction.signed_intent.intent.manifest)
            .map(|transaction| Self::Input {
                manifest: transaction,
                network_id: 0xf2.into(),
            })
    }
}

impl<'f> HasExamples<'f, NUMBER_OF_MANIFESTS_DOUBLE> for ManifestCompile {
    fn example_inputs() -> [Self::Input; NUMBER_OF_MANIFESTS_DOUBLE] {
        NotarizedTransactionHash::example_inputs()
            .map(|tx| tx.notarized_transaction.signed_intent.intent.manifest)
            .map(|transaction| Self::Input {
                manifest: transaction,
                network_id: 0xf2.into(),
            })
    }
}

impl<'f> HasExamples<'f, NUMBER_OF_MANIFESTS_DOUBLE> for ManifestDecompile {
    fn example_inputs() -> [Self::Input; NUMBER_OF_MANIFESTS_DOUBLE] {
        ManifestCompile::example_outputs().map(|output| ManifestDecompileInput {
            compiled: output,
            instructions_kind: SerializableInstructionsKind::String,
            network_id: 0xf2.into(),
        })
    }
}

impl<'f> HasExamples<'f, NUMBER_OF_MANIFESTS_DOUBLE> for ManifestStaticallyValidate {
    fn example_inputs() -> [Self::Input; NUMBER_OF_MANIFESTS_DOUBLE] {
        NotarizedTransactionHash::example_inputs()
            .map(|tx| tx.notarized_transaction.signed_intent.intent.manifest)
            .map(|transaction| Self::Input {
                manifest: transaction,
                network_id: 0xf2.into(),
            })
    }
}
