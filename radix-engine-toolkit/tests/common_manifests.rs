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

        let input = convert_manifest::Input {
            manifest,
            instructions_output_kind: InstructionKind::Parsed,
            network_id: 0xf2,
        };

        // Act
        let output = convert_manifest::Handler::fulfill(input);

        // Assert
        if output.is_err() {
            println!("{:?}", output);
            println!("{:?}", file_path);
        }
        assert!(matches!(output, Ok(..)));
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

        let input = convert_manifest::Input {
            manifest,
            instructions_output_kind: InstructionKind::Parsed,
            network_id: 0xf2,
        };
        let output = convert_manifest::Handler::fulfill(input).unwrap();

        let input = convert_manifest::Input {
            manifest: output.manifest,
            instructions_output_kind: InstructionKind::String,
            network_id: 0xf2,
        };

        // Act
        let output = convert_manifest::Handler::fulfill(input);

        // Assert
        assert!(matches!(output, Ok(..)));
    }
}

fn manifest_replace(string: String) -> String {
    string
        .replace(
            "${xrd_resource_address}",
            "resource_sim1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxakj8n3",
        )
        .replace(
            "${fungible_resource_address}",
            "resource_sim1thvwu8dh6lk4y9mntemkvj25wllq8adq42skzufp4m8wxxuemugnez",
        )
        .replace(
            "${resource_address}",
            "resource_sim1thvwu8dh6lk4y9mntemkvj25wllq8adq42skzufp4m8wxxuemugnez",
        )
        .replace(
            "${gumball_resource_address}",
            "resource_sim1thvwu8dh6lk4y9mntemkvj25wllq8adq42skzufp4m8wxxuemugnez",
        )
        .replace(
            "${non_fungible_resource_address}",
            "resource_sim1ngktvyeenvvqetnqwysevcx5fyvl6hqe36y3rkhdfdn6uzvt5366ha",
        )
        .replace(
            "${badge_resource_address}",
            "resource_sim1ngktvyeenvvqetnqwysevcx5fyvl6hqe36y3rkhdfdn6uzvt5366ha",
        )
        .replace(
            "${account_address}",
            "account_sim1cyvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cve475w0q",
        )
        .replace(
            "${this_account_address}",
            "account_sim1cyvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cve475w0q",
        )
        .replace(
            "${account_a_component_address}",
            "account_sim1cyvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cve475w0q",
        )
        .replace(
            "${account_b_component_address}",
            "account_sim1cyvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cve475w0q",
        )
        .replace(
            "${account_c_component_address}",
            "account_sim1cyvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cve475w0q",
        )
        .replace(
            "${other_account_address}",
            "account_sim1cyzfj6p254jy6lhr237s7pcp8qqz6c8ahq9mn6nkdjxxxat5syrgz9",
        )
        .replace(
            "${component_address}",
            "component_sim1cqvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cvemygpmu",
        )
        .replace(
            "${faucet_component_address}",
            "component_sim1cqvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cvemygpmu",
        )
        .replace(
            "${package_address}",
            "package_sim1p4r4955skdjq9swg8s5jguvcjvyj7tsxct87a9z6sw76cdfd2jg3zk",
        )
        .replace(
            "${minter_badge_resource_address}",
            "resource_sim1ngktvyeenvvqetnqwysevcx5fyvl6hqe36y3rkhdfdn6uzvt5366ha",
        )
        .replace(
            "${mintable_resource_address}",
            "resource_sim1nfhtg7ttszgjwysfglx8jcjtvv8q02fg9s2y6qpnvtw5jsy3wvlhj6",
        )
        .replace(
            "${vault_address}",
            "internal_vault_sim1tqvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cvevp72ff",
        )
        .replace("${owner_badge_non_fungible_local_id}", "#1#")
        .replace(
            "${code_blob_hash}",
            "5b4b01a4a3892ea3751793da57f072ae08eec694ddcda872239fc8239e4bcd1b",
        )
        .replace("${initial_supply}", "12")
        .replace("${mint_amount}", "12")
        .replace("${non_fungible_local_id}", "#12#")
        .replace(
            "${auth_badge_resource_address}",
            "resource_sim1n24hvnrgmhj6j8dpjuu85vfsagdjafcl5x4ewc9yh436jh2hpu4qdj",
        )
        .replace("${auth_badge_non_fungible_local_id}", "#1#")
        .replace(
            "${package_address}",
            "package_sim1p4r4955skdjq9swg8s5jguvcjvyj7tsxct87a9z6sw76cdfd2jg3zk",
        )
        .replace(
            "${epochmanager_address}",
            "epochmanager_sim1sexxxxxxxxxxephmgrxxxxxxxxx009352500589xxxxxxxxx82g6cl",
        )
        .replace(
            "${clock_address}",
            "clock_sim1skxxxxxxxxxxclckxxxxxxxxxxx002253583992xxxxxxxxxx58hk6",
        )
        .replace(
            "${validator_address}",
            "validator_sim1sgvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cvedzgr3l",
        )
        .replace(
            "${accesscontroller_address}",
            "accesscontroller_sim1cvvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cvexaj7at",
        )
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
