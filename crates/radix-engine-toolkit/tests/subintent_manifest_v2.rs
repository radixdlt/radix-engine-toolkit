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

use radix_engine_toolkit::functions::transaction_v2::subintent_manifest::{
    as_enclosed, statically_analyze_and_validate,
};
use scrypto_test::prelude::*;

#[test]
fn subintent_manifest_with_no_initial_resources_and_a_final_yield_is_considered_enclosed()
 {
    // Arrange
    let manifest = ManifestBuilder::new_subintent_v2()
        .assert_worktop_is_empty()
        .drop_all_proofs()
        .yield_to_parent(())
        .build();

    // Act
    let enclosed_manifest = as_enclosed(&manifest);

    // Assert
    assert!(enclosed_manifest.is_some());
}

#[test]
fn static_analysis_succeeds_for_subintent() {
    // Arrange
    let manifest = ManifestBuilder::new_subintent_v2()
        .assert_worktop_is_empty()
        .drop_all_proofs()
        .yield_to_parent(())
        .build();

    // Act
    let enclosed_manifest = statically_analyze_and_validate(&manifest);

    // Assert
    assert!(enclosed_manifest.is_ok());
}
