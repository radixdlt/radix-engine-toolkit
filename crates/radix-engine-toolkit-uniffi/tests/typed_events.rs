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

use radix_engine_toolkit_uniffi::Address;
use radix_substate_store_impls::memory_db::*;
use radix_transaction_scenarios::executor::DefaultTransactionScenarioExecutor;
use scrypto_test::prelude::*;

#[test]
pub fn events_emitted_from_native_entities_can_be_converted_to_typed() {
    // Arrange
    DefaultTransactionScenarioExecutor::new( InMemorySubstateDatabase::standard(), NetworkDefinition::simulator())
    .on_transaction_executed(|_, _, receipt, _| {
        for (event_identifier, event_data) in receipt
        .expect_commit_ignore_outcome()
        .application_events
        .iter()
        {
            let node_id = match event_identifier.0 {
                Emitter::Function(BlueprintId {
                    package_address, ..
                }) => package_address.into_node_id(),
                Emitter::Method(node_id, ..) => node_id,
            };
            if matches!(
                node_id.entity_type(),
                Some(
                    EntityType::GlobalGenericComponent
                        | EntityType::InternalGenericComponent
                        | EntityType::GlobalPackage
                )
            ) {
                continue;
            }

            let event_type_identifier =
                radix_engine_toolkit_uniffi::functions::EventTypeIdentifier {
                    emitter: match event_identifier.0 {
                        radix_engine_interface::prelude::Emitter::Function(ref blueprint_id) => {
                            radix_engine_toolkit_uniffi::functions::Emitter::Function {
                                address: Address::from_raw(
                                    blueprint_id.package_address.to_vec(),
                                    0xf2,
                                )
                                .unwrap(),
                                blueprint_name: blueprint_id.blueprint_name.clone(),
                            }
                        }
                        radix_engine_interface::prelude::Emitter::Method(node_id, module_id) => {
                            radix_engine_toolkit_uniffi::functions::Emitter::Method {
                                address: Address::from_raw(node_id.to_vec(), 0xf2).unwrap(),
                                object_module_id: module_id.into(),
                            }
                        }
                    },
                    event_name: event_identifier.1.clone(),
                };

            // Act
            let typed_event =
                radix_engine_toolkit_uniffi::functions::sbor_decode_to_typed_native_event(
                    event_type_identifier.clone(),
                    event_data.clone(),
                    0xf2,
                );

            // Assert
            match typed_event {
                Ok(_typed_event) => {}
                _ => panic!("Failed to convert to a typed event: {event_type_identifier:?}"),
            }
        }
    }).execute_all().expect("Transaction scenarios execution failed.");
}
