use crate::prelude::*;

#[test]
fn static_entities_in_invocation_addresses_are_discovered() {
    // Arrange
    let mut allocator = TestAddressAllocator::new();
    let account = allocator.new_account_address();

    let manifest = ManifestBuilder::new()
        .create_proof_from_account_of_amount(account, XRD, 10)
        .build();

    // Act
    let StaticAnalysis {
        entities_encountered_summary,
        ..
    } = statically_analyze(&manifest).expect("Can't fail!");

    // Assert
    assert!(entities_encountered_summary
        .entities
        .contains(&ManifestAddress::Static(account.into_node_id())));
}

#[test]
fn static_entities_in_invocation_args_are_discovered() {
    // Arrange
    let mut allocator = TestAddressAllocator::new();
    let account = allocator.new_account_address();

    let manifest = ManifestBuilder::new()
        .call_method(FAUCET, "some_method", (account,))
        .build();

    // Act
    let StaticAnalysis {
        entities_encountered_summary,
        ..
    } = statically_analyze(&manifest).expect("Can't fail!");

    // Assert
    assert!(entities_encountered_summary
        .entities
        .contains(&ManifestAddress::Static(account.into_node_id())));
    assert!(entities_encountered_summary
        .entities
        .contains(&ManifestAddress::Static(FAUCET.into_node_id())));
}

#[test]
fn static_and_dynamic_addresses_are_discovered_in_address_allocation() {
    // Arrange
    let manifest = ManifestBuilder::new()
        .allocate_global_address(
            ACCOUNT_PACKAGE,
            ACCOUNT_BLUEPRINT,
            "reservation",
            "address",
        )
        .create_account_with_owner(
            "reservation",
            OwnerRole::Fixed(rule!(allow_all)),
        )
        .call_method(
            "address",
            ACCOUNT_CREATE_PROOF_OF_AMOUNT_IDENT,
            AccountCreateProofOfAmountManifestInput {
                resource_address: XRD.into(),
                amount: 1.into(),
            },
        )
        .build();

    // Act
    let StaticAnalysis {
        entities_encountered_summary,
        ..
    } = statically_analyze(&manifest).expect("Can't fail!");

    // Assert
    assert!(entities_encountered_summary
        .entities
        .contains(&ManifestAddress::Static(ACCOUNT_PACKAGE.into_node_id())));
    assert!(entities_encountered_summary
        .entities
        .contains(&ManifestAddress::Named(ManifestNamedAddress(0))));
}
