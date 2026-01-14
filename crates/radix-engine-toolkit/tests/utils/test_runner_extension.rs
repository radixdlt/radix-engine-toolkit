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

        let validator_owner_badge = NonFungibleGlobalId::new(
            VALIDATOR_OWNER_BADGE,
            NonFungibleLocalId::bytes(address.as_bytes()).unwrap(),
        );

        self.execute_manifest(
            ManifestBuilder::new()
                .lock_fee_from_faucet()
                .withdraw_non_fungible_from_account(
                    account,
                    validator_owner_badge,
                )
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

    fn new_allow_all_access_controller_for_account(
        &mut self,
        account: (Secp256k1PublicKey, ComponentAddress),
    ) -> ComponentAddress {
        let (pk, account) = account;

        let manifest = ManifestBuilder::new()
            .lock_fee_from_faucet()
            .get_free_xrd_from_faucet()
            .call_method(
                account,
                ACCOUNT_SECURIFY_IDENT,
                AccountSecurifyManifestInput {},
            )
            .take_all_from_worktop(ACCOUNT_OWNER_BADGE, "bucket")
            .take_all_from_worktop(XRD, "free_xrd")
            .allocate_global_address(
                ACCESS_CONTROLLER_PACKAGE,
                ACCESS_CONTROLLER_BLUEPRINT,
                "ac_reservation",
                "ac_address",
            )
            .then(|builder| {
                let bucket = builder.bucket("bucket");
                let address_reservation =
                    builder.address_reservation("ac_reservation");
                let xrd_bucket = builder.bucket("free_xrd");

                builder
                    .call_function(
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
                            timed_recovery_delay_in_minutes: Some(10),
                            address_reservation: Some(address_reservation),
                        },
                    )
                    .call_method(
                        ManifestGlobalAddress::Named(ManifestNamedAddress(
                            address_reservation.0,
                        )),
                        ACCESS_CONTROLLER_CONTRIBUTE_RECOVERY_FEE_IDENT,
                        AccessControllerContributeRecoveryFeeManifestInput {
                            bucket: xrd_bucket,
                        },
                    )
            })
            .build();
        let receipt = self.execute_manifest(
            manifest,
            vec![NonFungibleGlobalId::from_public_key(&pk)],
        );
        receipt
            .expect_commit_success()
            .new_component_addresses()
            .first()
            .copied()
            .unwrap()
    }

    fn new_allow_all_access_controller(
        &mut self,
    ) -> (ComponentAddress, ComponentAddress) {
        let (pk, _, account) = self.new_account(true);
        let access_controller =
            self.new_allow_all_access_controller_for_account((pk, account));

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
                    owner_role: owner_role.into(),
                    storer_role: rule!(allow_all).into(),
                    storer_updater_role: rule!(allow_all).into(),
                    recoverer_role: rule!(allow_all).into(),
                    recoverer_updater_role: rule!(allow_all).into(),
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

    fn create_pool_entities(
        &mut self,
        account: ComponentAddress,
    ) -> CreatedPoolEntities {
        let resource_address1 = self.create_freely_mintable_fungible_resource(
            Default::default(),
            Some(dec!(1000)),
            18,
            account,
        );
        let resource_address2 = self.create_freely_mintable_fungible_resource(
            Default::default(),
            Some(dec!(1000)),
            18,
            account,
        );

        let manifest = ManifestBuilder::new()
            .lock_fee_from_faucet()
            .call_function(
                POOL_PACKAGE,
                ONE_RESOURCE_POOL_BLUEPRINT,
                ONE_RESOURCE_POOL_INSTANTIATE_IDENT,
                OneResourcePoolInstantiateManifestInput {
                    owner_role: OwnerRole::default().into(),
                    pool_manager_rule: rule!(allow_all).into(),
                    resource_address: resource_address1.into(),
                    address_reservation: None,
                },
            )
            .call_function(
                POOL_PACKAGE,
                TWO_RESOURCE_POOL_BLUEPRINT,
                TWO_RESOURCE_POOL_INSTANTIATE_IDENT,
                TwoResourcePoolInstantiateManifestInput {
                    owner_role: OwnerRole::default().into(),
                    pool_manager_rule: rule!(allow_all).into(),
                    resource_addresses: (
                        resource_address1.into(),
                        resource_address2.into(),
                    ),
                    address_reservation: None,
                },
            )
            .call_function(
                POOL_PACKAGE,
                MULTI_RESOURCE_POOL_BLUEPRINT,
                MULTI_RESOURCE_POOL_INSTANTIATE_IDENT,
                MultiResourcePoolInstantiateManifestInput {
                    owner_role: OwnerRole::default().into(),
                    pool_manager_rule: rule!(allow_all).into(),
                    resource_addresses: indexset![
                        resource_address1.into(),
                        resource_address2.into()
                    ],
                    address_reservation: None,
                },
            )
            .build();
        let receipt = self.execute_manifest(manifest, vec![]);
        let commit_result = receipt.expect_commit_success();
        let [one_pool, two_pool, multi_pool] =
            [0, 1, 2].map(|i| commit_result.new_component_addresses()[i]);
        let [one_pool_unit, two_pool_unit, multi_pool_unit] =
            [0, 1, 2].map(|i| commit_result.new_resource_addresses()[i]);

        CreatedPoolEntities {
            resource_address1,
            resource_address2,
            one_resource_pool: PoolInformation {
                component_address: one_pool,
                pool_unit_resource_address: one_pool_unit,
            },
            two_resource_pool: PoolInformation {
                component_address: two_pool,
                pool_unit_resource_address: two_pool_unit,
            },
            multi_resource_pool: PoolInformation {
                component_address: multi_pool,
                pool_unit_resource_address: multi_pool_unit,
            },
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PoolInformation {
    pub component_address: ComponentAddress,
    pub pool_unit_resource_address: ResourceAddress,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CreatedPoolEntities {
    pub resource_address1: ResourceAddress,
    pub resource_address2: ResourceAddress,
    pub one_resource_pool: PoolInformation,
    pub two_resource_pool: PoolInformation,
    pub multi_resource_pool: PoolInformation,
}
