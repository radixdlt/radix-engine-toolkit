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

//! This is an internal prelude module meant to only be public within this crate which re-exports
//! types and functions from the core toolkit and Scrypto crates with a prefix. This is done to
//! distinguish between the core toolkit types, scrypto types, and types defined in this crate.
//! We choose the types in this crate to have short un-prefixed names since those are the names that
//! the types will be generated with when using UniFFI

#[rustfmt::skip]
mod core {
    /* Modules */
    pub use radix_engine_toolkit_core::functions::information::{
        information as core_information, 
        BuildInformation as CoreBuildInformation,
        DependencyInformation as CoreDependencyInformation,
    };
    pub use radix_engine_toolkit_core::functions::derive::{
        virtual_account_address_from_public_key as core_virtual_account_address_from_public_key,
        virtual_identity_address_from_public_key as core_virtual_identity_address_from_public_key,
        virtual_signature_non_fungible_global_id_from_public_key as core_virtual_signature_non_fungible_global_id_from_public_key,
        virtual_account_address_from_olympia_account_address as core_virtual_account_address_from_olympia_account_address,
        resource_address_from_olympia_resource_address as core_resource_address_from_olympia_resource_address,
        public_key_from_olympia_account_address as core_public_key_from_olympia_account_address,
        olympia_account_address_from_public_key as core_olympia_account_address_from_public_key,
        node_address_from_public_key as core_node_address_from_public_key,
        DerivationError as CoreDerivationError,
        OlympiaNetwork as CoreOlympiaNetwork,
    };

    /* Utils */
    pub use radix_engine_toolkit_core::utils::{
        manifest_from_intent as core_manifest_from_intent,
        network_definition_from_network_id as core_network_definition_from_network_id,
        network_id_from_hrp as core_network_id_from_hrp,
        network_id_from_address_string as core_network_id_from_address_string,
        to_manifest_type as core_to_manifest_type,
        validate_manifest_value_against_schema as core_validate_manifest_value_against_schema,
        is_account as core_is_account,
        is_identity as core_is_identity,
        metadata_of_newly_created_entities as core_metadata_of_newly_created_entities,
        data_of_newly_minted_non_fungibles as core_data_of_newly_minted_non_fungibles,
    };
}

#[rustfmt::skip]
mod native {
    pub use transaction::prelude::{
        /* Cryptography */
        Ed25519PrivateKey as NativeEd25519PrivateKey,
        Secp256k1PrivateKey as NativeSecp256k1PrivateKey,
        
        PublicKey as NativePublicKey, 
        Ed25519PublicKey as NativeEd25519PublicKey,
        Secp256k1PublicKey as NativeSecp256k1PublicKey,
        
        PublicKeyHash as NativePublicKeyHash,
        HasPublicKeyHash as NativeHasPublicKeyHash,
        Ed25519PublicKeyHash as NativeEd25519PublicKeyHash,
        Secp256k1PublicKeyHash as NativeSecp256k1PublicKeyHash,

        SignatureV1 as NativeSignature,
        Ed25519Signature as NativeEd25519Signature, 
        Secp256k1Signature as NativeSecp256k1Signature, 

        SignatureWithPublicKeyV1 as NativeSignatureWithPublicKey,
    };

    pub use scrypto::prelude::{
        NodeId as NativeNodeId,
        EntityType as NativeEntityType,
        
        GlobalAddress as NativeGlobalAddress,
        InternalAddress as NativeInternalAddress,
        ComponentAddress as NativeComponentAddress,
        ResourceAddress as NativeResourceAddress,
        PackageAddress as NativePackageAddress,
        
        NonFungibleLocalId as NativeNonFungibleLocalId,
        NonFungibleGlobalId as NativeNonFungibleGlobalId,

        ParseDecimalError as NativeParseDecimalError,
        ParsePreciseDecimalError as NativeParsePreciseDecimalError,
        ParseNonFungibleLocalIdError as NativeParseNonFungibleLocalIdError,
        ParseNonFungibleGlobalIdError as NativeParseNonFungibleGlobalIdError,

        Decimal as NativeDecimal,
    };
    pub use scrypto::address::{
        Bech32Decoder as NativeBech32Decoder,
        Bech32Encoder as NativeBech32Encoder,
    };
    pub use radix_engine_common::data::scrypto::model::{
        ContentValidationError as NativeContentValidationError
    };
}

pub use self::core::*;
pub use self::native::*;
