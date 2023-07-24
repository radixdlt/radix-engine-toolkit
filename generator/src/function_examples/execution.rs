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

use radix_engine_common::prelude::*;
use radix_engine_toolkit::prelude::*;
use scrypto::blueprints::account::*;
use scrypto_unit::*;
use transaction::prelude::*;

use super::traits::HasExamples;

impl<'f> HasExamples<'f, 3> for ExecutionAnalyze {
    fn example_inputs() -> [Self::Input; 3] {
        let op1 = {
            let mut test_runner = TestRunnerBuilder::new().without_trace().build();
            let (public_key1, _, account1) = test_runner.new_account(true);
            let (public_key2, _, account2) = test_runner.new_account(true);

            let manifest = ManifestBuilder::new()
                .lock_fee(account1, "10")
                .withdraw_from_account(account1, RADIX_TOKEN, "10")
                .take_from_worktop(RADIX_TOKEN, "10", "bucket")
                .with_bucket("bucket", |builder, bucket| {
                    builder.call_method(
                        account2,
                        ACCOUNT_TRY_DEPOSIT_OR_ABORT_IDENT,
                        manifest_args!(bucket),
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
                .withdraw_from_account(account1, RADIX_TOKEN, "20")
                .take_from_worktop(RADIX_TOKEN, "10", "bucket")
                .with_bucket("bucket", |builder, bucket| {
                    builder.call_method(
                        account2,
                        ACCOUNT_TRY_DEPOSIT_OR_ABORT_IDENT,
                        manifest_args!(bucket),
                    )
                })
                .take_from_worktop(RADIX_TOKEN, "10", "bucket1")
                .with_bucket("bucket1", |builder, bucket| {
                    builder.call_method(
                        account3,
                        ACCOUNT_TRY_DEPOSIT_OR_ABORT_IDENT,
                        manifest_args!(bucket),
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
                .withdraw_from_account(account1, RADIX_TOKEN, "10")
                .withdraw_from_account(account2, RADIX_TOKEN, "10")
                .take_from_worktop(RADIX_TOKEN, "10", "bucket")
                .with_bucket("bucket", |builder, bucket| {
                    builder.call_method(
                        account2,
                        ACCOUNT_TRY_DEPOSIT_OR_ABORT_IDENT,
                        manifest_args!(bucket),
                    )
                })
                .take_from_worktop(RADIX_TOKEN, "10", "bucket1")
                .with_bucket("bucket1", |builder, bucket| {
                    builder.call_method(
                        account3,
                        ACCOUNT_TRY_DEPOSIT_OR_ABORT_IDENT,
                        manifest_args!(bucket),
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

        [op1, op2, op3].map(|(manifest, receipt)| {
            let instructions = to_serializable_instructions(&manifest.instructions, 0xf2).unwrap();
            let instructions = SerializableInstructions::Parsed(instructions);
            let preview_receipt = scrypto_encode(&receipt).unwrap();

            Self::Input {
                instructions,
                network_id: 0xf2.into(),
                preview_receipt: preview_receipt.into(),
            }
        })
    }
}
