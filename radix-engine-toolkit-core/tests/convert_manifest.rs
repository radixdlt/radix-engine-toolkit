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

use radix_engine_toolkit_core::model::ManifestInstructionsKind;
use radix_engine_toolkit_core::requests::ConvertManifestRequest;

mod test_vector;
use radix_engine_toolkit_core::traits::Request;
use test_vector::TRANSACTION_MANIFEST_TEST_VECTORS;

#[test]
pub fn conversion_of_manifests_succeeds() {
    // Arrange
    let network_id: u8 = 0xF2;

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
        assert!(matches!(response, Ok(..)));
    }
}
