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

use crate::prelude::*;
use core::array::from_fn;
use scrypto_test::prelude::*;

#[test]
fn non_fungibles_minted_with_initial_supply_show_up() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (_, _, account) = ledger.new_account(true);

    // Act
    let [id1, id2] = from_fn(|i| i as _).map(NonFungibleLocalId::integer);
    let manifest = new_non_fungible_manifest(
        account,
        Some(
            [&id1, &id2]
                .into_iter()
                .cloned()
                .map(|id| (id, ()))
                .collect::<Vec<_>>(),
        ),
    );
    let (_, dynamic_analysis) = ledger.analyze(manifest);
    let address = dynamic_analysis
        .entities_newly_created_summary
        .new_resource_entities
        .first()
        .copied()
        .unwrap();

    // Assert
    let new_non_fungibles = dynamic_analysis
        .entities_newly_created_summary
        .new_non_fungibles;
    assert_eq!(new_non_fungibles.len(), 2);
    assert!(new_non_fungibles.contains(&NonFungibleGlobalId::new(address, id1)));
    assert!(new_non_fungibles.contains(&NonFungibleGlobalId::new(address, id2)));
}

#[test]
fn non_fungibles_minted_from_resource_manager_show_up() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (_, _, account) = ledger.new_account(true);

    let manifest = new_non_fungible_manifest(account, None::<Vec<(_, ())>>);
    let receipt = ledger.execute_manifest(manifest, vec![]);
    let address = receipt
        .expect_commit_success()
        .new_resource_addresses()
        .first()
        .copied()
        .unwrap();

    // Act
    let id = NonFungibleLocalId::integer(1);
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .mint_non_fungible(address, [(id.clone(), ())])
        .try_deposit_entire_worktop_or_abort(account, None)
        .build();
    let (_, dynamic_analysis) = ledger.analyze(manifest);

    // Assert
    let new_non_fungibles = dynamic_analysis
        .entities_newly_created_summary
        .new_non_fungibles;
    assert_eq!(new_non_fungibles.len(), 1);
    assert!(new_non_fungibles.contains(&NonFungibleGlobalId::new(address, id)));
}

#[test]
fn non_fungibles_minted_and_burned_dont_show_up() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (_, _, account) = ledger.new_account(true);

    let manifest = new_non_fungible_manifest(account, None::<Vec<(_, ())>>);
    let receipt = ledger.execute_manifest(manifest, vec![]);
    let address = receipt
        .expect_commit_success()
        .new_resource_addresses()
        .first()
        .copied()
        .unwrap();

    // Act
    let id = NonFungibleLocalId::Integer(IntegerNonFungibleLocalId::new(1));
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .mint_non_fungible(address, [(id.clone(), ())])
        .burn_all_from_worktop(address)
        .build();
    let (_, dynamic_analysis) = ledger.analyze(manifest);

    // Assert
    assert_eq!(
        dynamic_analysis
            .entities_newly_created_summary
            .new_non_fungibles
            .len(),
        0
    );
}

#[test]
fn non_fungible_data_updates_dont_show_up_as_newly_minted_tokens() {
    // Arrange
    #[derive(Clone, PartialEq, Eq, ScryptoSbor, ManifestSbor)]
    pub struct NonFungibleData {
        pub name: String,
    }
    impl radix_common::prelude::NonFungibleData for NonFungibleData {
        const MUTABLE_FIELDS: &'static [&'static str] = &["name"];
    }

    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (_, _, account) = ledger.new_account(true);

    let id = NonFungibleLocalId::integer(1);
    let manifest = new_non_fungible_manifest(
        account,
        Some([(
            id.clone(),
            NonFungibleData {
                name: "data_1".to_string(),
            },
        )]),
    );
    let receipt = ledger.execute_manifest(manifest, vec![]);
    let address = receipt
        .expect_commit_success()
        .new_resource_addresses()
        .first()
        .copied()
        .unwrap();

    // Act
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .update_non_fungible_data(address, id, "name", "new_data")
        .build();
    let (_, dynamic_analysis) = ledger.analyze(manifest);

    // Assert
    assert_eq!(
        dynamic_analysis
            .entities_newly_created_summary
            .new_non_fungibles
            .len(),
        0
    );
}

fn new_non_fungible_manifest(
    account: ComponentAddress,
    initial_supply: Option<
        impl IntoIterator<
            Item = (NonFungibleLocalId, impl NonFungibleData + ManifestEncode),
        >,
    >,
) -> TransactionManifestV1 {
    ManifestBuilder::new()
        .lock_fee_from_faucet()
        .create_non_fungible_resource(
            Default::default(),
            NonFungibleIdType::Integer,
            Default::default(),
            NonFungibleResourceRoles::single_locked_rule(AccessRule::AllowAll),
            Default::default(),
            initial_supply,
        )
        .try_deposit_entire_worktop_or_abort(account, None)
        .build()
}
