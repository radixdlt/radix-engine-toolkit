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

use radix_engine_toolkit_core::error::Error;
use radix_engine_toolkit_core::model::ManifestInstructionsKind;
use radix_engine_toolkit_core::requests::ConvertManifestRequest;

mod test_vector;
use radix_engine_toolkit_core::traits::Request;
use test_vector::TRANSACTION_MANIFEST_TEST_VECTORS;

#[test]
pub fn manifests_converted_from_string_to_json_match_expected() {
    // Arrange
    let network_id = 0xF2;

    for test_vector in TRANSACTION_MANIFEST_TEST_VECTORS.iter() {
        let request = ConvertManifestRequest {
            manifest: test_vector.manifest.clone(),
            network_id,
            transaction_version: 0x01,
            manifest_instructions_output_format: ManifestInstructionsKind::JSON,
        };

        // Act
        let response = request
            .fulfill_request()
            .expect("Failed to convert a trusted manifest");

        // Assert
        assert_eq!(test_vector.expected_json_representation, response.manifest);
    }
}

#[test]
pub fn validation_fails_on_a_convert_manifest_request_with_network_mismatches() {
    // Arrange
    let network_id = 0x01; // Incorrect network. All test vectors use local simulator (0xF2).

    for test_vector in TRANSACTION_MANIFEST_TEST_VECTORS.iter() {
        let request = ConvertManifestRequest {
            manifest: test_vector.manifest.clone(),
            network_id,
            transaction_version: 0x01,
            manifest_instructions_output_format: ManifestInstructionsKind::JSON,
        };

        // Act
        let response = request.fulfill_request();

        // Assert
        // TODO: Feels like this should be a network mismatch error and not an "invalid HRP" error.
        assert!(matches!(response, Err(Error::AddressError(..))));
    }
}
