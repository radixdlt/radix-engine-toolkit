//! This module contains tests for the [`PresentedProofsAnalyzer`] from the
//! radix engine toolkit. The following have been identified as the various
//! cases that the analyzer might encounter and they are among the cases that
//! we test for in this module.
//!
//! | Resource Address | Account Address | Resource Type |
//! | ---------------- | --------------- | ------------- |
//! | Existing         | Existing        | Fungible      |
//! | Existing         | Existing        | Non Fungible  |
//! | Existing         | Allocated       | Fungible      |
//! | Existing         | Allocated       | Non Fungible  |
//! | Allocated        | Existing        | Fungible      |
//! | Allocated        | Existing        | Non Fungible  |
//! | Allocated        | Allocated       | Fungible      |
//! | Allocated        | Allocated       | Non Fungible  |
//!
//! For each one of the cases above we have added a test. We have a single test
//! for the resource type of fungible and a proof being created by ids since its
//! invalid but we wanted to make sure that it's valid in the analyzer itself.

use crate::prelude::*;

/// We're testing the case where a manifest creates a proof of a fungible
/// resource by ids. This case is pretty much impossible since this manifest
/// would fail. However, we're testing it to make sure that the analyzer itself
/// doesn't have an issue with this, which it should not. If anything, a higher
/// layer should catch issues like this not the analyzer that extracts the info
/// from the manifest.
#[test]
fn creating_fungible_proof_is_picked_up() {
    // Arrange
    let mut address_allocator = TestAddressAllocator::new();
    let account = address_allocator.new_account_address();
    let ids = indexset![NonFungibleLocalId::integer(1)];

    let manifest = ManifestBuilder::new()
        .create_proof_from_account_of_non_fungibles(account, XRD, ids.clone())
        .build();

    // Act
    let StaticAnalysis {
        proofs_created_summary,
        ..
    } = statically_analyze(&manifest).unwrap();

    // Assert
    let proofs_created_from_account = proofs_created_summary
        .created_proofs
        .get(&ManifestGlobalAddress::Static(account.into()))
        .expect("No proofs created against account");
    assert_eq!(proofs_created_from_account.len(), 1);
    assert_eq!(
        proofs_created_from_account.first(),
        Some(&ManifestResourceSpecifier::Ids(
            ManifestResourceAddress::Static(XRD),
            ids
        ))
    )
}

#[test]
fn creating_proof_of_an_existing_fungible_resource_from_an_existing_account_is_picked_up(
) {
    proof_analyzer_test(
        Allocation::UseExisting,
        Allocation::UseExisting,
        ResourceType::Fungible,
    )
}

#[test]
fn creating_proof_of_an_existing_non_fungible_resource_from_an_existing_account_is_picked_up(
) {
    proof_analyzer_test(
        Allocation::UseExisting,
        Allocation::UseExisting,
        ResourceType::NonFungible,
    )
}

#[test]
fn creating_proof_of_an_existing_fungible_resource_from_a_newly_allocated_account_is_picked_up(
) {
    proof_analyzer_test(
        Allocation::ReserveAddress,
        Allocation::UseExisting,
        ResourceType::Fungible,
    )
}

#[test]
fn creating_proof_of_an_existing_non_fungible_resource_from_a_newly_allocated_account_is_picked_up(
) {
    proof_analyzer_test(
        Allocation::ReserveAddress,
        Allocation::UseExisting,
        ResourceType::NonFungible,
    )
}

#[test]
fn creating_proof_of_a_newly_allocated_fungible_resource_from_an_existing_account_is_picked_up(
) {
    proof_analyzer_test(
        Allocation::UseExisting,
        Allocation::ReserveAddress,
        ResourceType::Fungible,
    )
}

#[test]
fn creating_proof_of_a_newly_allocated_non_fungible_resource_from_an_existing_account_is_picked_up(
) {
    proof_analyzer_test(
        Allocation::UseExisting,
        Allocation::ReserveAddress,
        ResourceType::NonFungible,
    )
}

#[test]
fn creating_proof_of_a_newly_allocated_fungible_resource_from_a_newly_allocated_account_is_picked_up(
) {
    proof_analyzer_test(
        Allocation::ReserveAddress,
        Allocation::ReserveAddress,
        ResourceType::Fungible,
    )
}

#[test]
fn creating_proof_of_a_newly_allocated_non_fungible_resource_from_a_newly_allocated_account_is_picked_up(
) {
    proof_analyzer_test(
        Allocation::ReserveAddress,
        Allocation::ReserveAddress,
        ResourceType::NonFungible,
    )
}

fn proof_analyzer_test(
    account_handling: Allocation,
    resource_handling: Allocation,
    resource_type: ResourceType,
) {
    // Arrange
    let mut ledger = LedgerSimulatorBuilder::new().build();
    let (_, _, stub_account) = ledger.new_account(true);
    let account_address = match account_handling {
        Allocation::ReserveAddress => {
            ManifestComponentAddress::Named(ManifestNamedAddress(0))
        }
        Allocation::UseExisting => {
            ManifestComponentAddress::Static(ledger.new_account(true).2)
        }
    };
    let resource_address = match (resource_handling, resource_type) {
        (Allocation::UseExisting, ResourceType::Fungible) => {
            ManifestResourceAddress::Static(
                ledger.create_freely_mintable_and_burnable_fungible_resource(
                    OwnerRole::Fixed(rule!(allow_all)),
                    None,
                    18,
                    stub_account,
                ),
            )
        }
        (Allocation::UseExisting, ResourceType::NonFungible) => {
            ManifestResourceAddress::Static(
                ledger.create_everything_allowed_non_fungible_resource(
                    OwnerRole::Fixed(rule!(allow_all)),
                ),
            )
        }
        (Allocation::ReserveAddress, _) => match account_handling {
            Allocation::ReserveAddress => {
                ManifestResourceAddress::Named(ManifestNamedAddress(1))
            }
            Allocation::UseExisting => {
                ManifestResourceAddress::Named(ManifestNamedAddress(0))
            }
        },
    };

    let manifest = ManifestBuilder::new()
        .with_name_lookup(|mut builder, lookup| {
            // If we need to allocate an account address we do so and we create
            // the account.
            builder = match account_handling {
                Allocation::ReserveAddress => builder
                    .allocate_global_address(
                        ACCOUNT_PACKAGE,
                        ACCOUNT_BLUEPRINT,
                        "account_reservation",
                        "account_address",
                    )
                    .create_account_with_owner("account_reservation", OwnerRole::Fixed(rule!(allow_all))),
                Allocation::UseExisting => builder,
            };

            // If we need to allocate a resource address we do so and we create
            // the resource.
            builder = match (resource_handling, resource_type) {
                (Allocation::ReserveAddress, ResourceType::Fungible) => builder
                    .allocate_global_address(
                        RESOURCE_PACKAGE,
                        FUNGIBLE_RESOURCE_MANAGER_BLUEPRINT,
                        "resource_reservation",
                        "resource_address",
                    )
                    .call_function(
                        RESOURCE_PACKAGE,
                        FUNGIBLE_RESOURCE_MANAGER_BLUEPRINT,
                        FUNGIBLE_RESOURCE_MANAGER_CREATE_IDENT,
                        FungibleResourceManagerCreateManifestInput {
                            owner_role: OwnerRole::Fixed(rule!(allow_all)),
                            track_total_supply: true,
                            divisibility: 18,
                            resource_roles: FungibleResourceRoles::single_locked_rule(rule!(allow_all)),
                            metadata: Default::default(),
                            address_reservation: Some(lookup.address_reservation("resource_reservation")),
                        },
                    ),
                (Allocation::ReserveAddress, ResourceType::NonFungible) => builder
                    .allocate_global_address(
                        RESOURCE_PACKAGE,
                        NON_FUNGIBLE_RESOURCE_MANAGER_BLUEPRINT,
                        "resource_reservation",
                        "resource_address",
                    )
                    .call_function(
                        RESOURCE_PACKAGE,
                        NON_FUNGIBLE_RESOURCE_MANAGER_BLUEPRINT,
                        NON_FUNGIBLE_RESOURCE_MANAGER_CREATE_IDENT,
                        NonFungibleResourceManagerCreateManifestInput {
                            owner_role: OwnerRole::Fixed(rule!(allow_all)),
                            track_total_supply: true,
                            resource_roles: NonFungibleResourceRoles::single_locked_rule(rule!(allow_all)),
                            metadata: Default::default(),
                            address_reservation: Some(lookup.address_reservation("resource_reservation")),
                            id_type: NonFungibleIdType::Integer,
                            non_fungible_schema:
                                NonFungibleDataSchema::new_local_without_self_package_replacement::<()>(),
                        },
                    ),
                (Allocation::UseExisting, _) => builder,
            };

            // Minting the resource, depositing it into the account, and create
            // a proof.
            match resource_type {
                ResourceType::Fungible => builder
                    .mint_fungible(resource_address, dec!(1))
                    .try_deposit_entire_worktop_or_abort(account_address, None)
                    .call_method(
                        account_address,
                        ACCOUNT_CREATE_PROOF_OF_AMOUNT_IDENT,
                        (resource_address, dec!(1)),
                    ),
                ResourceType::NonFungible => builder
                    .mint_non_fungible(resource_address, [(NonFungibleLocalId::integer(1), ())])
                    .try_deposit_entire_worktop_or_abort(account_address, None)
                    .call_method(
                        account_address,
                        ACCOUNT_CREATE_PROOF_OF_NON_FUNGIBLES_IDENT,
                        (resource_address, [NonFungibleLocalId::integer(1)]),
                    ),
            }
        })
        .build();

    // Act
    let (static_analysis, dynamic_analysis) = ledger.analyze(manifest);

    // Assert
    assert_eq!(
        static_analysis.proofs_created_summary,
        dynamic_analysis.proofs_created_summary
    );
    let proofs_created_from_account = static_analysis
        .proofs_created_summary
        .created_proofs
        .get(&ManifestGlobalAddress::from(account_address))
        .expect("No proofs created against account");
    assert_eq!(proofs_created_from_account.len(), 1);

    match resource_type {
        ResourceType::Fungible => assert_eq!(
            proofs_created_from_account.first(),
            Some(&ManifestResourceSpecifier::Amount(
                resource_address,
                dec!(1)
            ))
        ),
        ResourceType::NonFungible => assert_eq!(
            proofs_created_from_account.first(),
            Some(&ManifestResourceSpecifier::Ids(
                resource_address,
                indexset![NonFungibleLocalId::integer(1)]
            ))
        ),
    }
}

#[derive(Clone, Copy)]
pub enum Allocation {
    ReserveAddress,
    UseExisting,
}

#[derive(Clone, Copy)]
pub enum ResourceType {
    Fungible,
    NonFungible,
}
