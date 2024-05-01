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

use radix_common::prelude::AddressBech32Decoder;
use radix_engine_toolkit::functions::derive::{self, OlympiaNetwork};
use scrypto::prelude::*;

#[test]
fn virtual_account_address_can_be_derived_from_public_key() {
    // Arrange
    let public_key = public_key();

    // Act
    let virtual_account_address =
        derive::virtual_account_address_from_public_key(&public_key);

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
    let virtual_identity_address =
        derive::virtual_identity_address_from_public_key(&public_key);

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
    let olympia_address =
        "rdx1qspx7zxmnrh36q33av24srdfzg7m3cj65968erpjuh7ja3rm3kmn6hq4j9842";
    let public_key =
        "026f08db98ef1d0231eb15580da9123db8e25aa1747c8c32e5fd2ec47b8db73d5c"
            .parse::<Secp256k1PublicKey>()
            .unwrap();

    // Act
    let virtual_account_address =
        derive::virtual_account_address_from_olympia_account_address(
            olympia_address,
        )
        .unwrap();

    // Assert
    assert_eq!(
        virtual_account_address,
        derive::virtual_account_address_from_public_key(&public_key)
    )
}

#[test]
fn public_key_can_be_derived_from_olympia_account_address() {
    // Arrange
    let olympia_address =
        "rdx1qspx7zxmnrh36q33av24srdfzg7m3cj65968erpjuh7ja3rm3kmn6hq4j9842";
    let expected_public_key =
        "026f08db98ef1d0231eb15580da9123db8e25aa1747c8c32e5fd2ec47b8db73d5c"
            .parse::<Secp256k1PublicKey>()
            .unwrap();

    // Act
    let public_key =
        derive::public_key_from_olympia_account_address(olympia_address)
            .unwrap();

    // Assert
    assert_eq!(public_key, expected_public_key)
}

#[test]
fn resource_address_can_be_derived_from_olympia_resource_address() {
    // Arrange
    let olympia_address =
        "floop_rr1q0p0hzap6ckxqdk6khesyft62w34e0vdd06msn9snhfqknl370";
    let expected_babylon_resource_address = AddressBech32Decoder::validate_and_decode_ignore_hrp(
        "resource_rdx1tkhseye4w0hmf2af5enwurkxu4x29zk73yckyzhndv8xdk8tp2tn8q",
    )
    .map(|(_, _, data)| NodeId(data.try_into().unwrap()))
    .map(|node_id| ResourceAddress::new_or_panic(node_id.0))
    .unwrap();

    // Act
    let resource_address =
        derive::resource_address_from_olympia_resource_address(olympia_address)
            .unwrap();

    // Assert
    assert_eq!(resource_address, expected_babylon_resource_address)
}

#[test]
fn olympia_address_can_be_derived_from_public_key() {
    // Arrange
    let expected_olympia_address =
        "rdx1qspx7zxmnrh36q33av24srdfzg7m3cj65968erpjuh7ja3rm3kmn6hq4j9842";
    let public_key =
        "026f08db98ef1d0231eb15580da9123db8e25aa1747c8c32e5fd2ec47b8db73d5c"
            .parse::<Secp256k1PublicKey>()
            .unwrap();

    // Act
    let olympia_address = derive::olympia_account_address_from_public_key(
        &public_key,
        OlympiaNetwork::Mainnet,
    );

    // Assert
    assert_eq!(olympia_address, expected_olympia_address)
}

#[test]
fn node_address_can_be_derived_from_public_key() {
    // Arrange
    let expected_node_address =
        "node_tdx_21_1qfk895krd3l8t8z7z7p9sxpjdszpal24f6y2sjtqe7mdkhdele5az658ak2";
    let public_key =
        "026c72d2c36c7e759c5e17825818326c041efd554e88a84960cfb6db5db9fe69d1"
            .parse::<Secp256k1PublicKey>()
            .unwrap();

    // Act
    let node_address = derive::node_address_from_public_key(&public_key, 0x21);

    // Assert
    assert_eq!(node_address, expected_node_address)
}

fn public_key() -> PublicKey {
    let private_key = Secp256k1PrivateKey::from_u64(1).unwrap();
    private_key.public_key().into()
}
