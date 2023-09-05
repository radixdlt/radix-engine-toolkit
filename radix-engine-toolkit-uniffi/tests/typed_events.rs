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

use radix_engine::transaction::*;
use radix_engine::types::*;
use radix_engine::vm::wasm::*;
use radix_engine::vm::*;
use radix_engine_stores::memory_db::*;
use radix_engine_toolkit_uniffi::Address;
use scrypto_unit::*;
use transaction::prelude::*;
use transaction::validation::*;
use transaction_scenarios::scenario::*;
use transaction_scenarios::scenarios::*;

#[test]
pub fn events_emitted_from_native_entities_can_be_converted_to_typed() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().without_trace().build();
    for (_, receipt) in execute_scenarios(&mut test_runner) {
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
                Some(EntityType::GlobalGenericComponent | EntityType::InternalGenericComponent)
            ) {
                continue;
            }

            let event_type_identifier =
                radix_engine_toolkit_uniffi::functions::EventTypeIdentifier {
                    emitter: match event_identifier.0 {
                        radix_engine::types::Emitter::Function(ref blueprint_id) => {
                            radix_engine_toolkit_uniffi::functions::Emitter::Function {
                                address: Address::from_raw(
                                    blueprint_id.package_address.to_vec(),
                                    0xf2,
                                )
                                .unwrap(),
                                blueprint_name: blueprint_id.blueprint_name.clone(),
                            }
                        }
                        radix_engine::types::Emitter::Method(node_id, module_id) => {
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
                    event_type_identifier,
                    event_data.clone(),
                    0xf2,
                );

            // Assert
            match typed_event {
                Ok(_typed_event) => {}
                _ => panic!("Failed to convert to a typed event"),
            }
        }
    }
}

pub fn execute_scenarios(
    test_runner: &mut TestRunner<NoExtension, InMemorySubstateDatabase>,
) -> Vec<(TransactionManifestV1, TransactionReceipt)> {
    let mut vec = Vec::new();

    let mut next_nonce: u32 = 0;
    for scenario_builder in get_builder_for_every_scenario() {
        let mut scenario = scenario_builder(ScenarioCore::new(
            NetworkDefinition::simulator(),
            test_runner.get_current_epoch(),
            next_nonce,
        ));

        let validator = NotarizedTransactionValidator::new(ValidationConfig::simulator());
        let substate_db = test_runner.substate_db_mut();
        let scrypto_vm = ScryptoVm::<DefaultWasmEngine>::default();
        let native_vm = DefaultNativeVm::new();
        let vm = Vm::new(&scrypto_vm, native_vm);
        let fee_reserve_config = CostingParameters::default();
        let execution_config = ExecutionConfig::for_preview(NetworkDefinition::simulator());

        let mut previous = None;
        loop {
            let next = scenario
                .next(previous.as_ref())
                .map_err(|err| err.into_full(&scenario))
                .unwrap();
            match next {
                NextAction::Transaction(next) => {
                    let transaction = next
                        .validate(&validator)
                        .map_err(|err| err.into_full(&scenario))
                        .unwrap();
                    let transaction_receipt = execute_and_commit_transaction(
                        substate_db,
                        vm.clone(),
                        &fee_reserve_config,
                        &execution_config,
                        &transaction.get_executable(),
                    );
                    if transaction_receipt.is_commit_success() {
                        vec.push((next.manifest.clone(), transaction_receipt.clone()));
                    }
                    previous = Some(transaction_receipt);
                }
                NextAction::Completed(_) => break,
            }
        }

        // TODO(RCnet-V3): Change it so that each scenario starts at a different fixed nonce value,
        // hard-coded for that scenario, to minimize separate scenarios causing
        // non-determinism in others
        next_nonce += 1000;
    }

    vec
}
