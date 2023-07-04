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

use radix_engine::types::*;
use scrypto::api::node_modules::metadata::MetadataValue;
use scrypto::*;
use scrypto_unit::*;
use transaction::prelude::*;

#[test]
fn extraction_of_metadata_from_receipts_succeeds() {
    // Arrange
    let mut test_runner = TestRunner::builder().without_trace().build();

    // Act
    let manifest = ManifestBuilder::new()
        .create_fungible_resource::<AccessRule>(
            18,
            btreemap!(
                "name".to_owned() => MetadataValue::Bool(true)
            ),
            BTreeMap::new(),
            None,
        )
        .build();
    let receipt = test_runner.execute_manifest_ignoring_fee(manifest, vec![]);
    let metadata = radix_engine_toolkit_core::utils::metadata_of_newly_created_entities(&receipt);

    // Assert
    let metadata = metadata.expect("Cant be none");
    let global_address = GlobalAddress::from(
        *receipt
            .expect_commit_success()
            .new_resource_addresses()
            .get(0)
            .unwrap(),
    );
    assert_eq!(
        metadata,
        hashmap! {
            global_address => hashmap!(
                "name".to_string() => MetadataValue::Bool(true),
            )
        }
    )
}

#[test]
fn extraction_of_non_fungible_data_from_receipts_succeeds() {
    // Arrange
    let mut test_runner = TestRunner::builder().without_trace().build();
    let (_, _, account) = test_runner.new_account(false);

    // Act
    let manifest = ManifestBuilder::new()
        .create_non_fungible_resource(
            NonFungibleIdType::Integer,
            btreemap!(
                "name".to_owned() => MetadataValue::Bool(true)
            ),
            BTreeMap::<ResourceMethodAuthKey, (AccessRule, AccessRule)>::new(),
            Some(btreemap!(
                NonFungibleLocalId::integer(1) => Owl {
                    name: "an example name".to_string(),
                    age: 100,
                    country: "Japan".to_string()
                },
                NonFungibleLocalId::integer(2) => Owl {
                    name: "a second example".to_string(),
                    age: 100,
                    country: "Canada".to_string()
                },
            )),
        )
        .try_deposit_batch_or_abort(account)
        .build();
    let receipt = test_runner.execute_manifest_ignoring_fee(manifest, vec![]);
    let new_non_fungibles =
        radix_engine_toolkit_core::utils::data_of_newly_minted_non_fungibles(&receipt);

    // Assert
    let non_fungible_data = new_non_fungibles.expect("Cant be none");
    let resource_address = *receipt
        .expect_commit_success()
        .new_resource_addresses()
        .get(0)
        .unwrap();
    assert_eq!(
        non_fungible_data,
        hashmap! {
            resource_address => hashmap! {
                NonFungibleLocalId::integer(1) => ScryptoValue::Tuple {
                    fields: vec![
                        ScryptoValue::String {
                            value: "an example name".to_string(),
                        },
                        ScryptoValue::U32 { value: 100 },
                        ScryptoValue::String {
                            value: "Japan".to_string(),
                        },
                    ],
                },
                NonFungibleLocalId::integer(2) => ScryptoValue::Tuple {
                    fields: vec![
                        ScryptoValue::String {
                            value: "a second example".to_string(),
                        },
                        ScryptoValue::U32 { value: 100 },
                        ScryptoValue::String {
                            value: "Canada".to_string(),
                        },
                    ],
                },
            }
        }
    );
}

#[derive(NonFungibleData, ScryptoSbor, ManifestSbor)]
struct Owl {
    name: String,
    age: u32,
    country: String,
}
