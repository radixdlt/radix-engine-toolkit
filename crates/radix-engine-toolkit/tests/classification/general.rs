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

#[test]
fn faucet_free_xrd_manifest_classifies_as_general() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();

    let (_, _, account) = ledger.new_account(false);
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .get_free_xrd_from_faucet()
        .take_all_from_worktop(XRD, "xrd")
        .try_deposit_or_abort(account, None, "xrd")
        .build();

    // Act
    let (static_analysis, dynamic_analysis) = ledger.analyze(manifest);

    // Assert
    assert_eq!(static_analysis.manifest_classification.len(), 1);
    assert_eq!(dynamic_analysis.detailed_manifest_classification.len(), 1);
    assert!(static_analysis.manifest_classification.iter().any(
        |classification| matches!(
            classification,
            ManifestClassification::General
        )
    ));
    assert!(dynamic_analysis
        .detailed_manifest_classification
        .iter()
        .any(|classification| matches!(
            classification,
            DetailedManifestClassification::General
        )));
}
