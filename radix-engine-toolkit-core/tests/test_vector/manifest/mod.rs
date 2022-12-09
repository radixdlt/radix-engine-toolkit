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

// All of the manifests in this module are obtained from the original Scrypto repo. The hopes of the
// test vectors here is to make sure that Scrypto can understand can also be understood by the Radix
// Engine Toolkit. Source of manifests:
// https://github.com/radixdlt/radixdlt-scrypto/tree/release/betanet-v1/transaction/examples

use radix_engine_toolkit_core::model::TransactionManifest;

pub struct TransactionManifestTestVector {
    pub original_manifest: String,
    pub manifest: TransactionManifest,
}

impl TransactionManifestTestVector {
    pub fn new<S: AsRef<str>>(manifest: S, blobs: &[&[u8]]) -> Self {
        Self {
            original_manifest: manifest.as_ref().to_string(),
            manifest: TransactionManifest {
                instructions: radix_engine_toolkit_core::model::ManifestInstructions::String(
                    manifest.as_ref().to_string(),
                ),
                blobs: blobs.iter().map(|x| (*x).to_owned()).collect(),
            },
        }
    }
}

lazy_static::lazy_static! {
    pub static ref TRANSACTION_MANIFEST_TEST_VECTORS: Vec<TransactionManifestTestVector> = vec![
        TransactionManifestTestVector::new(
            include_str!("./complex.rtm"),
            &[include_bytes!("./complex.code"), include_bytes!("./complex.abi")]
        ),
        TransactionManifestTestVector::new(
            include_str!("./call_method.rtm"),
            &[]
        ),
        TransactionManifestTestVector::new(
            include_str!("./call_function.rtm"),
            &[]
        ),
        TransactionManifestTestVector::new(
            include_str!("./any_value.rtm"),
            &[include_bytes!("./any_value.blob")]
        ),
        TransactionManifestTestVector::new(
            include_str!("./non_fungible_ids_canonical.rtm"),
            &[]
        ),
    ];
}
