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

use radix_engine::transaction::{
    execute_and_commit_transaction, CostingParameters, ExecutionConfig, TransactionReceipt,
};
use radix_engine::types::ResourceOrNonFungible;
use radix_engine::vm::Vm;
use radix_engine::vm::{wasm::DefaultWasmEngine, DefaultNativeVm, NoExtension, ScryptoVm};
use radix_engine_common::prelude::*;
use radix_engine_stores::memory_db::InMemorySubstateDatabase;
use radix_engine_toolkit::prelude::*;
use scrypto::blueprints::account::*;
use scrypto_unit::*;
use transaction::prelude::*;
use transaction::validation::{NotarizedTransactionValidator, ValidationConfig};
use transaction_scenarios::scenario::*;
use transaction_scenarios::scenarios::get_builder_for_every_scenario;

use super::traits::HasExamples;

impl<'f> HasExamples<'f, 65> for ExecutionAnalyze {
    fn example_inputs() -> [Self::Input; 65] {
        let op1 = {
            let mut test_runner = TestRunnerBuilder::new().without_trace().build();
            let (public_key1, _, account1) = test_runner.new_account(true);
            let (public_key2, _, account2) = test_runner.new_account(true);

            let manifest = ManifestBuilder::new()
                .lock_fee(account1, "10")
                .withdraw_from_account(account1, XRD, "10")
                .take_from_worktop(XRD, "10", "bucket")
                .with_bucket("bucket", |builder, bucket| {
                    builder.call_method(
                        account2,
                        ACCOUNT_TRY_DEPOSIT_OR_ABORT_IDENT,
                        manifest_args!(bucket, Option::<ResourceOrNonFungible>::None),
                    )
                })
                .build();
            let receipt = test_runner.preview_manifest(
                manifest.clone(),
                vec![public_key1.into(), public_key2.into()],
                0,
                PreviewFlags::default(),
            );
            receipt.expect_commit_success();

            (manifest, receipt)
        };

        let op2 = {
            let mut test_runner = TestRunnerBuilder::new().without_trace().build();
            let (public_key1, _, account1) = test_runner.new_account(true);
            let (public_key2, _, account2) = test_runner.new_account(true);
            let (public_key3, _, account3) = test_runner.new_account(true);

            let manifest = ManifestBuilder::new()
                .lock_fee(account1, "10")
                .withdraw_from_account(account1, XRD, "20")
                .take_from_worktop(XRD, "10", "bucket")
                .with_bucket("bucket", |builder, bucket| {
                    builder.call_method(
                        account2,
                        ACCOUNT_TRY_DEPOSIT_OR_ABORT_IDENT,
                        manifest_args!(bucket, Option::<ResourceOrNonFungible>::None),
                    )
                })
                .take_from_worktop(XRD, "10", "bucket1")
                .with_bucket("bucket1", |builder, bucket| {
                    builder.call_method(
                        account3,
                        ACCOUNT_TRY_DEPOSIT_OR_ABORT_IDENT,
                        manifest_args!(bucket, Option::<ResourceOrNonFungible>::None),
                    )
                })
                .build();
            let receipt = test_runner.preview_manifest(
                manifest.clone(),
                vec![public_key1.into(), public_key2.into(), public_key3.into()],
                0,
                PreviewFlags::default(),
            );
            receipt.expect_commit_success();

            (manifest, receipt)
        };

        let op3 = {
            let mut test_runner = TestRunnerBuilder::new().without_trace().build();
            let (public_key1, _, account1) = test_runner.new_account(true);
            let (public_key2, _, account2) = test_runner.new_account(true);
            let (public_key3, _, account3) = test_runner.new_account(true);

            let manifest = ManifestBuilder::new()
                .lock_fee(account1, "10")
                .withdraw_from_account(account1, XRD, "10")
                .withdraw_from_account(account2, XRD, "10")
                .take_from_worktop(XRD, "10", "bucket")
                .with_bucket("bucket", |builder, bucket| {
                    builder.call_method(
                        account2,
                        ACCOUNT_TRY_DEPOSIT_OR_ABORT_IDENT,
                        manifest_args!(bucket, Option::<ResourceOrNonFungible>::None),
                    )
                })
                .take_from_worktop(XRD, "10", "bucket1")
                .with_bucket("bucket1", |builder, bucket| {
                    builder.call_method(
                        account3,
                        ACCOUNT_TRY_DEPOSIT_OR_ABORT_IDENT,
                        manifest_args!(bucket, Option::<ResourceOrNonFungible>::None),
                    )
                })
                .build();
            let receipt = test_runner.preview_manifest(
                manifest.clone(),
                vec![public_key1.into(), public_key2.into(), public_key3.into()],
                0,
                PreviewFlags::default(),
            );
            receipt.expect_commit_success();

            (manifest, receipt)
        };

        [op1, op2, op3]
            .into_iter()
            .chain(execute_scenarios(
                &mut TestRunnerBuilder::new().without_trace().build(),
            ))
            .map(|(manifest, receipt)| {
                let instructions =
                    to_serializable_instructions(&manifest.instructions, 0xf2).unwrap();
                let instructions = SerializableInstructions::Parsed(instructions);
                let preview_receipt = scrypto_encode(&receipt).unwrap();

                Self::Input {
                    instructions,
                    network_id: 0xf2.into(),
                    preview_receipt: preview_receipt.into(),
                }
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap_or_else(|v: Vec<Self::Input>| {
                panic!("Execution Analysis generation should be: {}", v.len())
            })
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
