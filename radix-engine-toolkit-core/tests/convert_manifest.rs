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

use radix_engine_toolkit_core::model::TransactionManifest;
use radix_engine_toolkit_core::requests::ConvertManifestRequest;
use radix_engine_toolkit_core::traits::Request;

#[test]
pub fn basic_manifest_conversion_succeeds() {
    // Arrange
    let test_vectors = vec![
        (
            include_str!("test_manifests/manifest1.rtm").to_string(),
            vec![
                include_bytes!("test_manifests/manifest1_code.blob").to_vec(),
                include_bytes!("test_manifests/manifest1_abi.blob").to_vec(),
            ],
        ),
        (
            include_str!("test_manifests/manifest2.rtm").to_string(),
            vec![],
        ),
        (
            include_str!("test_manifests/manifest3.rtm").to_string(),
            vec![],
        ),
        (
            include_str!("test_manifests/manifest4.rtm").to_string(),
            vec![include_bytes!("test_manifests/manifest4.blob").to_vec()],
        ),
    ];

    let requests = test_vectors
        .iter()
        .map(|x| ConvertManifestRequest {
            transaction_version: 0x01,
            network_id: 0xF2,
            manifest_instructions_output_format:
                radix_engine_toolkit_core::model::ManifestInstructionsKind::JSON,
            manifest: TransactionManifest {
                instructions: radix_engine_toolkit_core::model::ManifestInstructions::String(
                    x.0.clone(),
                ),
                blobs: x.1.clone(),
            },
        })
        .collect::<Vec<_>>();

    for request in requests {
        // Act
        let response = request.fulfill_request();

        // Assert
        assert!(response.is_ok());
    }
}
