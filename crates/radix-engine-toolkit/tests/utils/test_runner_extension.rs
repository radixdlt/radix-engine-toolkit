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

use radix_engine::system::bootstrap::*;
use radix_engine::transaction::*;
use radix_engine::vm::*;
use radix_engine_interface::blueprints::consensus_manager::*;
use radix_engine_toolkit::prelude::*;
use radix_engine_toolkit_common::receipt::*;
use scrypto_test::prelude::*;

#[extend::ext]
pub impl<E, D> LedgerSimulator<E, D>
where
    E: NativeVmExtension,
    D: TestDatabase,
{
    fn preview(
        &mut self,
        manifest: TransactionManifestV1,
    ) -> TransactionReceiptV1 {
        self.preview_manifest(
            manifest,
            vec![],
            0,
            PreviewFlags {
                use_free_credit: true,
                assume_all_signature_proofs: true,
                skip_epoch_check: true,
                disable_auth: true,
            },
        )
    }

    fn analyze(
        &mut self,
        manifest: TransactionManifestV1,
    ) -> (StaticAnalysis, DynamicAnalysis) {
        let receipt = LedgerSimulatorEDExt::preview(self, manifest.clone());
        if !receipt.is_commit_success() {
            panic!("Not commit success: {receipt:?}")
        }

        let static_analysis =
            radix_engine_toolkit::prelude::statically_analyze(&manifest)
                .unwrap();
        let dynamic_analysis =
            radix_engine_toolkit::prelude::dynamically_analyze(
                &manifest,
                RuntimeToolkitTransactionReceipt::try_from(receipt).unwrap(),
            )
            .unwrap();

        (static_analysis, dynamic_analysis)
    }

    fn new_validator(
        &mut self,
        pub_key: Secp256k1PublicKey,
        account: ComponentAddress,
    ) -> (ComponentAddress, ResourceAddress, ResourceAddress) {
        let manifest = ManifestBuilder::new()
            .lock_fee_from_faucet()
            .get_free_xrd_from_faucet()
            .take_from_worktop(
                XRD,
                *DEFAULT_VALIDATOR_XRD_COST,
                "xrd_creation_fee",
            )
            .create_validator(pub_key, Decimal::ONE, "xrd_creation_fee")
            .try_deposit_entire_worktop_or_abort(account, None)
            .build();
        let receipt = self.execute_manifest(manifest, vec![]);
        let commit_result = receipt.expect_commit_success();

        let address = commit_result.new_component_addresses()[0];
        let claim_nft = commit_result.new_resource_addresses()[0];
        let lsu = commit_result.new_resource_addresses()[1];

        self.execute_manifest(
            ManifestBuilder::new()
                .lock_fee_from_faucet()
                .withdraw_from_account(account, VALIDATOR_OWNER_BADGE, 1)
                .take_from_worktop(VALIDATOR_OWNER_BADGE, 1, "badge")
                .create_proof_from_bucket_of_all("badge", "proof")
                .push_to_auth_zone("proof")
                .register_validator(address)
                .call_method(
                    address,
                    VALIDATOR_UPDATE_ACCEPT_DELEGATED_STAKE_IDENT,
                    ValidatorUpdateAcceptDelegatedStakeInput {
                        accept_delegated_stake: true,
                    },
                )
                .drop_auth_zone_proofs()
                .return_to_worktop("badge")
                .try_deposit_entire_worktop_or_abort(account, None)
                .build(),
            vec![NonFungibleGlobalId::from_public_key(&pub_key)],
        )
        .expect_commit_success();

        self.advance_epoch(10);

        (address, lsu, claim_nft)
    }

    fn advance_epoch(&mut self, by: u64) {
        let current_epoch = self.get_current_epoch();
        self.set_current_epoch(current_epoch.after(by).unwrap());
    }

    fn new_allow_all_access_controller(
        &mut self,
    ) -> (ComponentAddress, ComponentAddress) {
        let (pk, _, account) = self.new_account(true);
        let manifest = ManifestBuilder::new()
            .lock_fee_from_faucet()
            .call_method(
                account,
                ACCOUNT_SECURIFY_IDENT,
                AccountSecurifyManifestInput {},
            )
            .take_all_from_worktop(ACCOUNT_OWNER_BADGE, "bucket")
            .then(|builder| {
                let bucket = builder.bucket("bucket");

                builder.call_function(
                    ACCESS_CONTROLLER_PACKAGE,
                    ACCESS_CONTROLLER_BLUEPRINT,
                    ACCESS_CONTROLLER_CREATE_IDENT,
                    AccessControllerCreateManifestInput {
                        controlled_asset: bucket,
                        rule_set: RuleSet {
                            primary_role: rule!(allow_all),
                            recovery_role: rule!(allow_all),
                            confirmation_role: rule!(allow_all),
                        },
                        timed_recovery_delay_in_minutes: None,
                        address_reservation: None,
                    },
                )
            })
            .build();
        let receipt = self.execute_manifest(
            manifest,
            vec![NonFungibleGlobalId::from_public_key(&pk)],
        );
        let access_controller = receipt
            .expect_commit_success()
            .new_component_addresses()
            .first()
            .copied()
            .unwrap();

        (account, access_controller)
    }

    fn new_account_locker(
        &mut self,
        owner_role: OwnerRole,
    ) -> ComponentAddress {
        let manifest = ManifestBuilder::new()
            .lock_fee_from_faucet()
            .call_function(
                LOCKER_PACKAGE,
                ACCOUNT_LOCKER_BLUEPRINT,
                ACCOUNT_LOCKER_INSTANTIATE_IDENT,
                AccountLockerInstantiateManifestInput {
                    owner_role,
                    storer_role: rule!(allow_all),
                    storer_updater_role: rule!(allow_all),
                    recoverer_role: rule!(allow_all),
                    recoverer_updater_role: rule!(allow_all),
                    address_reservation: None,
                },
            )
            .build();
        self.execute_manifest(manifest, vec![])
            .expect_commit_success()
            .new_component_addresses()
            .first()
            .copied()
            .unwrap()
    }
}
