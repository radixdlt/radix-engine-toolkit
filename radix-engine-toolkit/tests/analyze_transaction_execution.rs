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

use std::collections::BTreeMap;

use native_transaction::{builder::ManifestBuilder, manifest::decompile};
use radix_engine::types::NetworkDefinition;
use radix_engine_toolkit::{
    functions::{analyze_transaction_execution, InvocationHandler},
    model::{
        resource_quantifier::{ResourceManagerSpecifier, ResourceQuantifier},
        transaction::{InstructionList, TransactionManifest},
    },
    visitor::{AccountDeposit, ExactnessSpecifier},
};
use scrypto::prelude::*;
use scrypto_unit::TestRunner;

#[test]
pub fn analyze_create_resources_transaction() {
    // Arrange
    let mut test_runner = TestRunner::builder().without_trace().build();
    let (_, _, account) = test_runner.new_account(false);

    let manifest = ManifestBuilder::new()
        .lock_fee(test_runner.faucet_component(), 10.into())
        .create_fungible_resource(
            18,
            [
                ("name".into(), "my first resource".into()),
                ("description".into(), "my exciting resource".into()),
            ]
            .into(),
            BTreeMap::<ResourceMethodAuthKey, (AccessRule, AccessRule)>::new(),
            Some(1.into()),
        )
        .create_fungible_resource(
            18,
            [
                ("name".into(), "my second resource".into()),
                ("description".into(), "my exciting resource".into()),
            ]
            .into(),
            BTreeMap::<ResourceMethodAuthKey, (AccessRule, AccessRule)>::new(),
            Some(2.into()),
        )
        .call_method(
            account,
            "deposit_batch",
            manifest_args!(ManifestExpression::EntireWorktop),
        )
        .build();

    let receipt = test_runner.execute_manifest(manifest.clone(), vec![]);
    receipt.expect_commit_success();

    // Act
    let manifest = TransactionManifest {
        instructions: InstructionList::String(
            decompile(&manifest.instructions, &NetworkDefinition::simulator()).unwrap(),
        ),
        blobs: Default::default(),
    };
    let output =
        analyze_transaction_execution::Handler::fulfill(analyze_transaction_execution::Input {
            network_id: 0xf2,
            manifest,
            transaction_receipt: scrypto_encode(&receipt).unwrap(),
        });

    // Assert
    let output = output.expect("Should not fail");
    assert_eq!(output.created_entities.resource_addresses.len(), 2);
    assert_eq!(
        output.account_deposits[0],
        AccountDeposit {
            component_address: as_network_aware_node_id!(account),
            // TODO: This should be exact and not an estimate.
            deposited: ExactnessSpecifier::Estimate {
                instruction_index: 3,
                resource_quantifier: ResourceQuantifier::Amount {
                    resource_address: ResourceManagerSpecifier::NewlyCreated { index: 1 },
                    amount: 2.into()
                }
            }
        }
    );
    assert_eq!(
        output.account_deposits[1],
        AccountDeposit {
            component_address: as_network_aware_node_id!(account),
            // TODO: This should be exact and not an estimate.
            deposited: ExactnessSpecifier::Estimate {
                instruction_index: 3,
                resource_quantifier: ResourceQuantifier::Amount {
                    resource_address: ResourceManagerSpecifier::NewlyCreated { index: 0 },
                    amount: 1.into()
                }
            }
        }
    );
}

#[macro_export]
macro_rules! as_network_aware_node_id {
    ($value: expr) => {
        radix_engine_toolkit::model::address::NetworkAwareNodeId($value.as_node_id().0, 0xf2)
    };
}
