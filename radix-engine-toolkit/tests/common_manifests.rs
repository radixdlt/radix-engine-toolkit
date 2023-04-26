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

use std::path::{Path, PathBuf};

use radix_engine_toolkit::functions::convert_manifest;
use radix_engine_toolkit::functions::traits::InvocationHandler;
use radix_engine_toolkit::model::transaction::{
    InstructionKind, InstructionList, TransactionManifest,
};

const MANIFESTS_PATH: &str = "./tests/test_vector/manifest";

#[test]
pub fn common_manifests_can_be_converted_to_parsed_manifests() {
    // Arrange
    for file_path in rtm_file_paths(MANIFESTS_PATH) {
        let manifest_str = std::fs::read_to_string(&file_path)
            .map(manifest_replace)
            .unwrap();

        let manifest = TransactionManifest {
            instructions: InstructionList::String(manifest_str),
            blobs: vec![[10].into(), [10].into()],
        };

        let request = convert_manifest::Input {
            manifest,
            instructions_output_kind: InstructionKind::Parsed,
            network_id: 0xf2,
        };

        // Act
        let response = convert_manifest::Handler::fulfill(request);

        // Assert
        assert!(matches!(response, Ok(..)));
    }
}

#[test]
pub fn common_manifests_can_be_converted_to_parsed_and_then_back_to_string_manifests() {
    // Arrange
    for file_path in rtm_file_paths(MANIFESTS_PATH) {
        let manifest_str = std::fs::read_to_string(&file_path)
            .map(manifest_replace)
            .unwrap();

        let manifest = TransactionManifest {
            instructions: InstructionList::String(manifest_str),
            blobs: vec![[10].into()],
        };

        let request = convert_manifest::Input {
            manifest,
            instructions_output_kind: InstructionKind::Parsed,
            network_id: 0xf2,
        };
        let response = convert_manifest::Handler::fulfill(request).unwrap();

        let request = convert_manifest::Input {
            manifest: response.manifest,
            instructions_output_kind: InstructionKind::String,
            network_id: 0xf2,
        };

        // Act
        let response = convert_manifest::Handler::fulfill(request);

        // Assert
        assert!(matches!(response, Ok(..)));
    }
}

fn manifest_replace(string: String) -> String {
    string
        .replace("${", "{")
        .replace(
            "{xrd_resource_address}",
            "resource_sim1q2k2fa4x7rjy9e57wff58dr3uydvypdgf2m3kvec2uv5uxwhjta2dm",
        )
        .replace(
            "{faucet_component_address}",
            "component_sim1pyakxvzls3cwkfp25xz7dufp9jnw6wzxe3cxaku2ju7tlyuvusk6y9",
        )
        .replace(
            "{account_address}",
            "account_sim1quft09whj3nzlsd80n28fasr92pup8z4hkflrkqtagelu3g4ywxhjl",
        )
        .replace(
            "{this_account_address}",
            "account_sim1quft09whj3nzlsd80n28fasr92pup8z4hkflrkqtagelu3g4ywxhjl",
        )
        .replace(
            "{account_component_address}",
            "account_sim1quft09whj3nzlsd80n28fasr92pup8z4hkflrkqtagelu3g4ywxhjl",
        )
        .replace(
            "{other_account_address}",
            "account_sim1quft09whj3nzlsd80n28fasr92pup8z4hkflrkqtagelu3g4ywxhjl",
        )
        .replace(
            "{account_a_component_address}",
            "account_sim1quft09whj3nzlsd80n28fasr92pup8z4hkflrkqtagelu3g4ywxhjl",
        )
        .replace(
            "{account_b_component_address}",
            "account_sim1quft09whj3nzlsd80n28fasr92pup8z4hkflrkqtagelu3g4ywxhjl",
        )
        .replace(
            "{account_c_component_address}",
            "account_sim1quft09whj3nzlsd80n28fasr92pup8z4hkflrkqtagelu3g4ywxhjl",
        )
        .replace(
            "{owner_badge_resource_address}",
            "resource_sim1q2k2fa4x7rjy9e57wff58dr3uydvypdgf2m3kvec2uv5uxwhjta2dm",
        )
        .replace(
            "{minter_badge_resource_address}",
            "resource_sim1q2k2fa4x7rjy9e57wff58dr3uydvypdgf2m3kvec2uv5uxwhjta2dm",
        )
        .replace(
            "{auth_badge_resource_address}",
            "resource_sim1q2k2fa4x7rjy9e57wff58dr3uydvypdgf2m3kvec2uv5uxwhjta2dm",
        )
        .replace(
            "{mintable_resource_address}",
            "resource_sim1q2k2fa4x7rjy9e57wff58dr3uydvypdgf2m3kvec2uv5uxwhjta2dm",
        )
        .replace("{owner_badge_non_fungible_local_id}", "#1#")
        .replace("{auth_badge_non_fungible_local_id}", "#1#")
        .replace(
            "{code_blob_hash}",
            "5b4b01a4a3892ea3751793da57f072ae08eec694ddcda872239fc8239e4bcd1b",
        )
        .replace(
            "{schema_blob_hash}",
            "5b4b01a4a3892ea3751793da57f072ae08eec694ddcda872239fc8239e4bcd1b",
        )
        .replace("{initial_supply}", "12")
        .replace("{mint_amount}", "12")
        .replace("{non_fungible_local_id}", "#1#")
}

fn rtm_file_paths<P: AsRef<Path>>(path: P) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    for entry in std::fs::read_dir(path.as_ref()).unwrap() {
        let entry = entry.unwrap();
        if entry.metadata().unwrap().is_dir() {
            paths.extend(rtm_file_paths(entry.path()))
        } else if entry.file_name().to_str().unwrap().ends_with(".rtm") {
            paths.push(entry.path())
        }
    }
    paths
}
