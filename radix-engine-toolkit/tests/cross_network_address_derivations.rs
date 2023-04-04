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

use radix_engine_common::crypto::{EcdsaSecp256k1PublicKey, PublicKey};
use radix_engine_common::data::scrypto::model::ComponentAddress;
use radix_engine_toolkit::request::{
    DeriveBabylonAddressFromOlympiaAddressHandler, DeriveBabylonAddressFromOlympiaAddressRequest,
    DeriveOlympiaAddressFromPublicKeyHandler, DeriveOlympiaAddressFromPublicKeyRequest, Handler,
    OlympiaNetwork,
};
use radix_engine_toolkit::utils::checked_copy_u8_slice;

#[test]
pub fn deriving_babylon_address_from_olympia_address_succeeds_and_produces_expected_address() {
    // Arrange
    let olympia_address = "rdx1qspx7zxmnrh36q33av24srdfzg7m3cj65968erpjuh7ja3rm3kmn6hq4j9842";
    let expected_public_key = PublicKey::EcdsaSecp256k1(EcdsaSecp256k1PublicKey(
        checked_copy_u8_slice(
            hex::decode("026f08db98ef1d0231eb15580da9123db8e25aa1747c8c32e5fd2ec47b8db73d5c")
                .unwrap(),
        )
        .unwrap(),
    ));
    let expected_nebunet_address =
        ComponentAddress::virtual_account_from_public_key(&expected_public_key);

    // Act
    let (public_key, account_address) = {
        let request = DeriveBabylonAddressFromOlympiaAddressRequest {
            network_id: 0x0b,
            olympia_account_address: olympia_address.to_owned(),
        };
        let response = DeriveBabylonAddressFromOlympiaAddressHandler::fulfill(request).unwrap();
        (response.public_key, response.babylon_account_address)
    };

    // Assert
    assert_eq!(expected_public_key, public_key);
    assert_eq!(expected_nebunet_address, account_address.address);
}

#[test]
pub fn deriving_olympia_mainnet_address_from_public_key_succeeds_and_produces_expected_address() {
    // Arrange
    let expected_olympia_address =
        "rdx1qspx7zxmnrh36q33av24srdfzg7m3cj65968erpjuh7ja3rm3kmn6hq4j9842";
    let public_key = PublicKey::EcdsaSecp256k1(EcdsaSecp256k1PublicKey(
        checked_copy_u8_slice(
            hex::decode("026f08db98ef1d0231eb15580da9123db8e25aa1747c8c32e5fd2ec47b8db73d5c")
                .unwrap(),
        )
        .unwrap(),
    ));

    // Act
    let olympia_address = {
        let request = DeriveOlympiaAddressFromPublicKeyRequest {
            network: OlympiaNetwork::Mainnet,
            public_key,
        };
        let response = DeriveOlympiaAddressFromPublicKeyHandler::fulfill(request).unwrap();
        response.olympia_account_address
    };

    // Assert
    assert_eq!(expected_olympia_address, olympia_address);
}
