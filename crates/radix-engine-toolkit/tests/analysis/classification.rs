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

use std::{fs::read_to_string, path::PathBuf};

use radix_transactions::manifest::{compile, MockBlobProvider};

use crate::prelude::*;

#[test]
fn empty_manifest_has_no_manifest_classification() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let manifest = ManifestBuilder::new().build();

    // Act
    let (static_analysis, dynamic_analysis) = ledger.analyze(manifest);

    // Assert
    assert_eq!(static_analysis.manifest_classification.len(), 0);
    assert_eq!(dynamic_analysis.detailed_manifest_classification.len(), 0);
}

#[test]
fn check_classifications_of_troublesome_manifests() {
    // Arrange
    let mut failures = 0;
    for index in 1..=29 {
        let file_name = format!("{index}.rtm");
        let file_path = PathBuf::from(".")
            .canonicalize()
            .unwrap()
            .join("tests")
            .join("assets")
            .join(file_name.as_str());
        let manifest_string = read_to_string(file_path).unwrap();
        let manifest = compile(
            &manifest_string,
            &NetworkDefinition::mainnet(),
            MockBlobProvider,
        )
        .unwrap();

        // Act
        let static_analysis =
            radix_engine_toolkit::prelude::statically_analyze(&manifest);

        // Assert
        match static_analysis {
            Ok(static_analysis) => {
                println!(
                    "✅ Succeeded: {} - Classifications: {:?}",
                    file_name, static_analysis.manifest_classification
                );
            }
            Err(err) => {
                println!("❌ Failed: {} - {:?}", file_name, err);
                failures += 1;
            }
        }
    }

    assert_eq!(failures, 0)
}
