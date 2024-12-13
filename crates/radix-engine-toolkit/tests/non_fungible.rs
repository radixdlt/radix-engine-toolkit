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

use scrypto_test::prelude::*;

mod test_runner_extension;
use test_runner_extension::LedgerSimulatorEDExt;

#[test]
fn dynamic_analysis_new_non_fungible_list_initial_supply() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (_, _, account) = ledger.new_allocated_account();

    // Act
    let nf_id_1 =
        NonFungibleLocalId::Integer(IntegerNonFungibleLocalId::new(1));
    let nf_id_2 =
        NonFungibleLocalId::Integer(IntegerNonFungibleLocalId::new(2));
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .create_non_fungible_resource(
            OwnerRole::None,
            NonFungibleIdType::Integer,
            false,
            NonFungibleResourceRoles::default(),
            ModuleConfig::default(),
            Some(vec![(nf_id_1.clone(), ()), (nf_id_2.clone(), ())]),
        )
        .try_deposit_entire_worktop_or_abort(account, None)
        .build();
    let (_, dynamic_analysis) = ledger.summarize(manifest);
    let address = dynamic_analysis
        .new_entities
        .resource_addresses
        .first()
        .unwrap();

    // Assert
    assert_eq!(dynamic_analysis.newly_created_non_fungibles.len(), 2);
    assert!(
        dynamic_analysis
            .newly_created_non_fungibles
            .contains(&NonFungibleGlobalId::new(*address, nf_id_1))
    );
    assert!(
        dynamic_analysis
            .newly_created_non_fungibles
            .contains(&NonFungibleGlobalId::new(*address, nf_id_2))
    );
}

#[test]
fn dynamic_analysis_new_non_fungible_list_after_mint() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (_, _, account) = ledger.new_allocated_account();

    let nf_id_1 =
        NonFungibleLocalId::Integer(IntegerNonFungibleLocalId::new(1));
    let nf_id_2 =
        NonFungibleLocalId::Integer(IntegerNonFungibleLocalId::new(2));
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .create_non_fungible_resource(
            OwnerRole::None,
            NonFungibleIdType::Integer,
            false,
            NonFungibleResourceRoles::single_locked_rule(AccessRule::AllowAll),
            ModuleConfig::default(),
            Some(vec![(nf_id_1.clone(), ())]),
        )
        .try_deposit_entire_worktop_or_abort(account, None)
        .build();
    let receipt = ledger.execute_manifest(manifest, vec![]);
    let address = receipt
        .expect_commit_success()
        .new_resource_addresses()
        .first()
        .unwrap();

    // Act
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .mint_non_fungible(*address, vec![(nf_id_2.clone(), ())])
        .try_deposit_entire_worktop_or_abort(account, None)
        .build();
    let (_, dynamic_analysis) = ledger.summarize(manifest);

    // Assert
    assert_eq!(dynamic_analysis.newly_created_non_fungibles.len(), 1);
    assert!(
        dynamic_analysis
            .newly_created_non_fungibles
            .contains(&NonFungibleGlobalId::new(*address, nf_id_2))
    );
}

#[test]
fn dynamic_analysis_new_non_fungible_list_after_burn() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (_, _, account) = ledger.new_allocated_account();

    let nf_id_1 =
        NonFungibleLocalId::Integer(IntegerNonFungibleLocalId::new(1));
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .create_non_fungible_resource(
            OwnerRole::None,
            NonFungibleIdType::Integer,
            false,
            NonFungibleResourceRoles::single_locked_rule(AccessRule::AllowAll),
            ModuleConfig::default(),
            Some(vec![(nf_id_1.clone(), ())]),
        )
        .try_deposit_entire_worktop_or_abort(account, None)
        .build();
    let receipt = ledger.execute_manifest(manifest, vec![]);
    let address = receipt
        .expect_commit_success()
        .new_resource_addresses()
        .first()
        .unwrap();

    // Act
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .burn_non_fungible_in_account(
            account,
            NonFungibleGlobalId::new(*address, nf_id_1),
        )
        .build();
    let (_, dynamic_analysis) = ledger.summarize(manifest);

    // Assert
    assert_eq!(dynamic_analysis.newly_created_non_fungibles.len(), 0);
}

#[test]
fn dynamic_analysis_new_non_fungible_list_after_mint_and_burn() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (_, _, account) = ledger.new_allocated_account();

    let nf_id_1 =
        NonFungibleLocalId::Integer(IntegerNonFungibleLocalId::new(1));
    let nf_id_2 =
        NonFungibleLocalId::Integer(IntegerNonFungibleLocalId::new(2));
    let nf_id_3 =
        NonFungibleLocalId::Integer(IntegerNonFungibleLocalId::new(3));
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .create_non_fungible_resource(
            OwnerRole::None,
            NonFungibleIdType::Integer,
            false,
            NonFungibleResourceRoles::single_locked_rule(AccessRule::AllowAll),
            ModuleConfig::default(),
            Some(vec![(nf_id_1.clone(), ()), (nf_id_2.clone(), ())]),
        )
        .try_deposit_entire_worktop_or_abort(account, None)
        .build();
    let receipt = ledger.execute_manifest(manifest, vec![]);
    let address = receipt
        .expect_commit_success()
        .new_resource_addresses()
        .first()
        .unwrap();

    // Act
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .mint_non_fungible(*address, vec![(nf_id_3.clone(), ())])
        .burn_non_fungible_in_account(
            account,
            NonFungibleGlobalId::new(*address, nf_id_2),
        )
        .try_deposit_entire_worktop_or_abort(account, None)
        .build();
    let (_, dynamic_analysis) = ledger.summarize(manifest);

    // Assert
    assert_eq!(dynamic_analysis.newly_created_non_fungibles.len(), 1);
    assert!(
        dynamic_analysis
            .newly_created_non_fungibles
            .contains(&NonFungibleGlobalId::new(*address, nf_id_3))
    );
}

#[test]
fn dynamic_analysis_new_non_fungible_list_after_update() {
    // Arrange
    #[derive(Clone, PartialEq, Eq, ScryptoSbor, ManifestSbor)]
    pub struct NfData {
        pub name: String,
    }
    impl NonFungibleData for NfData {
        const MUTABLE_FIELDS: &'static [&'static str] = &["name"];
    }
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (_, _, account) = ledger.new_allocated_account();

    let nf_id_1 =
        NonFungibleLocalId::Integer(IntegerNonFungibleLocalId::new(1));
    let nf_id_2 =
        NonFungibleLocalId::Integer(IntegerNonFungibleLocalId::new(2));
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .create_non_fungible_resource(
            OwnerRole::Updatable(AccessRule::AllowAll),
            NonFungibleIdType::Integer,
            false,
            NonFungibleResourceRoles::single_locked_rule(AccessRule::AllowAll),
            ModuleConfig::default(),
            Some(vec![
                (
                    nf_id_1.clone(),
                    NfData {
                        name: "data_1".to_string(),
                    },
                ),
                (
                    nf_id_2.clone(),
                    NfData {
                        name: "data_2".to_string(),
                    },
                ),
            ]),
        )
        .try_deposit_entire_worktop_or_abort(account, None)
        .build();
    let receipt = ledger.execute_manifest(manifest, vec![]);
    let address = receipt
        .expect_commit_success()
        .new_resource_addresses()
        .first()
        .unwrap();

    // Act
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .update_non_fungible_data(*address, nf_id_1, "name", "new_data")
        .build();
    let (_, dynamic_analysis) = ledger.summarize(manifest);

    // Assert
    assert_eq!(dynamic_analysis.newly_created_non_fungibles.len(), 0);
}
