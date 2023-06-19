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

use native_json_library::prelude::*;
use radix_engine_common::types::*;
use transaction::prelude::{
    Ed25519PrivateKey, Secp256k1PrivateKey, TransactionBuilder, TransactionHeaderV1,
    TransactionManifestV1,
};
use transaction::validation::ValidationConfig;

use super::manifest_provider::*;
use super::traits::HasExamples;

impl<'f> HasExamples<'f, NUMBER_OF_MANIFESTS_DOUBLE> for NotarizedTransactionHash {
    fn example_inputs() -> [Self::Input; NUMBER_OF_MANIFESTS_DOUBLE] {
        notarized_transactions().map(|transaction| Self::Input {
            notarized_transaction: transaction,
            network_id: 0xf2.into(),
        })
    }
}

impl<'f> HasExamples<'f, NUMBER_OF_MANIFESTS_DOUBLE> for NotarizedTransactionCompile {
    fn example_inputs() -> [Self::Input; NUMBER_OF_MANIFESTS_DOUBLE] {
        notarized_transactions().map(|transaction| Self::Input {
            notarized_transaction: transaction,
            network_id: 0xf2.into(),
        })
    }
}

impl<'f> HasExamples<'f, NUMBER_OF_MANIFESTS_DOUBLE> for NotarizedTransactionDecompile {
    fn example_inputs() -> [Self::Input; NUMBER_OF_MANIFESTS_DOUBLE] {
        NotarizedTransactionCompile::example_outputs().map(|output| {
            NotarizedTransactionDecompileInput {
                compiled: output,
                instructions_kind: SerializableInstructionsKind::String,
                network_id: 0xf2.into(),
            }
        })
    }
}

impl<'f> HasExamples<'f, NUMBER_OF_MANIFESTS_DOUBLE> for NotarizedTransactionStaticallyValidate {
    fn example_inputs() -> [Self::Input; NUMBER_OF_MANIFESTS_DOUBLE] {
        notarized_transactions().map(|transaction| Self::Input {
            notarized_transaction: transaction,
            network_id: 0xf2.into(),
            validation_config: ValidationConfig::default(0xf2).into(),
        })
    }
}

fn build_transaction(instructions: SerializableInstructions) -> SerializableNotarizedTransaction {
    let instructions_kind = match instructions {
        SerializableInstructions::String(..) => SerializableInstructionsKind::String,
        SerializableInstructions::Parsed(..) => SerializableInstructionsKind::Parsed,
    };
    let instructions = instructions.to_instructions(0xf2).unwrap();
    let manifest = TransactionManifestV1 {
        instructions,
        blobs: Default::default(),
    };

    let notary_private_key = Secp256k1PrivateKey::from_u64(1).unwrap();
    let signer1_private_key = Secp256k1PrivateKey::from_u64(2).unwrap();
    let signer2_private_key = Ed25519PrivateKey::from_u64(2).unwrap();

    let header = TransactionHeaderV1 {
        network_id: 0xf2,
        nonce: 100,
        end_epoch_exclusive: Epoch::of(100),
        start_epoch_inclusive: Epoch::of(90),
        notary_is_signatory: true,
        notary_public_key: notary_private_key.public_key().into(),
        tip_percentage: 0,
    };

    let transaction = TransactionBuilder::new()
        .manifest(manifest)
        .header(header)
        .sign(&signer1_private_key)
        .sign(&signer2_private_key)
        .notarize(&notary_private_key)
        .build();

    SerializableNotarizedTransaction::from_native(&transaction, 0xf2, instructions_kind).unwrap()
}

pub fn notarized_transactions() -> [SerializableNotarizedTransaction; NUMBER_OF_MANIFESTS_DOUBLE] {
    get_serializable_instructions().map(build_transaction)
}
