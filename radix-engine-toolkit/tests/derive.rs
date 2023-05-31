use radix_engine_toolkit::functions::derive::{self, OlympiaNetwork};
use scrypto::prelude::*;
use transaction::ecdsa_secp256k1::EcdsaSecp256k1PrivateKey;

#[test]
fn virtual_account_address_can_be_derived_from_public_key() {
    // Arrange
    let public_key = public_key();

    // Act
    let virtual_account_address = derive::virtual_account_address_from_public_key(&public_key);

    // Assert
    assert_eq!(
        virtual_account_address,
        ComponentAddress::try_from_hex(
            "d1d28b92b6e84499b83b0797ef5235553eeb7edaa0cea243c1128c2fe737"
        )
        .unwrap()
    )
}

#[test]
fn virtual_identity_address_can_be_derived_from_public_key() {
    // Arrange
    let public_key = public_key();

    // Act
    let virtual_identity_address = derive::virtual_identity_address_from_public_key(&public_key);

    // Assert
    assert_eq!(
        virtual_identity_address,
        ComponentAddress::try_from_hex(
            "d2d28b92b6e84499b83b0797ef5235553eeb7edaa0cea243c1128c2fe737"
        )
        .unwrap()
    )
}

#[test]
fn virtual_account_address_can_be_derived_from_olympia_account_address() {
    // Arrange
    let olympia_address = "rdx1qspx7zxmnrh36q33av24srdfzg7m3cj65968erpjuh7ja3rm3kmn6hq4j9842";
    let public_key = "026f08db98ef1d0231eb15580da9123db8e25aa1747c8c32e5fd2ec47b8db73d5c"
        .parse::<EcdsaSecp256k1PublicKey>()
        .unwrap();

    // Act
    let virtual_account_address =
        derive::virtual_account_address_from_olympia_account_address(olympia_address).unwrap();

    // Assert
    assert_eq!(
        virtual_account_address,
        derive::virtual_account_address_from_public_key(&public_key)
    )
}

#[test]
fn public_key_can_be_derived_from_olympia_account_address() {
    // Arrange
    let olympia_address = "rdx1qspx7zxmnrh36q33av24srdfzg7m3cj65968erpjuh7ja3rm3kmn6hq4j9842";
    let expected_public_key = "026f08db98ef1d0231eb15580da9123db8e25aa1747c8c32e5fd2ec47b8db73d5c"
        .parse::<EcdsaSecp256k1PublicKey>()
        .unwrap();

    // Act
    let public_key = derive::public_key_from_olympia_account_address(olympia_address).unwrap();

    // Assert
    assert_eq!(public_key, expected_public_key)
}

#[test]
fn olympia_address_can_be_derived_from_public_key() {
    // Arrange
    let expected_olympia_address =
        "rdx1qspx7zxmnrh36q33av24srdfzg7m3cj65968erpjuh7ja3rm3kmn6hq4j9842";
    let public_key = "026f08db98ef1d0231eb15580da9123db8e25aa1747c8c32e5fd2ec47b8db73d5c"
        .parse::<EcdsaSecp256k1PublicKey>()
        .unwrap();

    // Act
    let olympia_address =
        derive::olympia_account_address_from_public_key(&public_key, OlympiaNetwork::Mainnet);

    // Assert
    assert_eq!(olympia_address, expected_olympia_address)
}

fn public_key() -> PublicKey {
    let private_key = EcdsaSecp256k1PrivateKey::from_u64(1).unwrap();
    private_key.public_key().into()
}
