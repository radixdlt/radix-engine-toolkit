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

use radix_engine_toolkit::functions::transaction_v2::subintent_manifest::statically_analyze_and_validate;
use radix_engine_toolkit::transaction_types::ManifestClass;
use radix_transactions::manifest::{MockBlobProvider, compile_manifest};
use scrypto::prelude::*;
use scrypto_test::prelude::SubintentManifestV2;

mod test_data;

#[test]
fn manifest_can_be_compiled() {
    // Arrange
    let manifest = test_data::manifest();

    // Act
    let compiled =
        radix_engine_toolkit::functions::transaction_v1::manifest::to_payload_bytes(
            &manifest,
        );

    // Assert
    assert!(compiled.is_ok())
}

#[test]
fn manifest_can_be_compiled_and_later_decompiled() {
    // Arrange
    let manifest = test_data::manifest();
    let compiled =
        radix_engine_toolkit::functions::transaction_v1::manifest::to_payload_bytes(
            &manifest,
        )
        .unwrap();

    // Act
    let decompiled =
        radix_engine_toolkit::functions::transaction_v1::manifest::from_payload_bytes(
            compiled,
        );

    // Assert
    assert!(decompiled.is_ok());
    assert_eq!(decompiled, Ok(manifest))
}

#[test]
fn manifest_can_be_statically_validated() {
    // Arrange
    let manifest = test_data::manifest();

    // Act
    let validation_result =
        radix_engine_toolkit::functions::transaction_v1::manifest::statically_validate(
            &manifest,
            &NetworkDefinition::mainnet()
        );

    // Assert
    assert!(validation_result.is_ok())
}

#[test]
fn transfer_subintent_has_general_non_enclosed_type() {
    // Arrange
    let manifest = r#"
    CALL_METHOD
        Address("account_tdx_2_128rl4glqhuf6gm5vv6769hppt3u35mrrz4mshqc4qjqms45nranynd")
        "withdraw"
        Address("resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc")
        Decimal("10");
    TAKE_FROM_WORKTOP
        Address("resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc")
        Decimal("10")
        Bucket("bucket");
    CALL_METHOD
        Address("account_tdx_2_12yj94f9g3dpn9x36l7wf8tjndp4nt53r5hj8ks8y9mf4ul3fgq7agl")
        "try_deposit_or_abort"
        Bucket("bucket")
        None;
    YIELD_TO_PARENT;
    "#;

    let manifest = compile_manifest::<SubintentManifestV2>(
        manifest,
        &NetworkDefinition::stokenet(),
        MockBlobProvider::new(),
    )
    .unwrap();

    // Act
    let analysis = statically_analyze_and_validate(&manifest);

    // Assert
    assert!(
        analysis
            .unwrap()
            .classification
            .contains(&ManifestClass::GeneralNonEnclosed)
    );
}

#[test]
fn transfer_subintent_with_metadata_update_has_general_non_enclosed_type() {
    // Arrange
    let manifest = r#"
    CALL_METHOD
        Address("account_tdx_2_128rl4glqhuf6gm5vv6769hppt3u35mrrz4mshqc4qjqms45nranynd")
        "withdraw"
        Address("resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc")
        Decimal("10");
    TAKE_FROM_WORKTOP
        Address("resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc")
        Decimal("10")
        Bucket("bucket");
    CALL_METHOD
        Address("account_tdx_2_12yj94f9g3dpn9x36l7wf8tjndp4nt53r5hj8ks8y9mf4ul3fgq7agl")
        "try_deposit_or_abort"
        Bucket("bucket")
        None;
    SET_METADATA
        Address("account_tdx_2_128rl4glqhuf6gm5vv6769hppt3u35mrrz4mshqc4qjqms45nranynd")
        "account_type"
        Enum<0u8>(
            "dapp definition"
        );
    YIELD_TO_PARENT;
    "#;

    let manifest = compile_manifest::<SubintentManifestV2>(
        manifest,
        &NetworkDefinition::stokenet(),
        MockBlobProvider::new(),
    )
    .unwrap();

    // Act
    let analysis = statically_analyze_and_validate(&manifest);

    // Assert
    assert!(
        !analysis
            .unwrap()
            .classification
            .contains(&ManifestClass::GeneralNonEnclosed)
    );
}
