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

use crate::model::value::ast::ManifestAstValue;
use std::collections::BTreeSet;
use toolkit_derive::serializable;

/// The Instruction model defines the structure that transaction manifest instructions follow during
/// communication with the Radix Engine Toolkit
#[serializable]
#[serde(tag = "instruction", rename_all = "SCREAMING_SNAKE_CASE")]
#[derive(Eq, PartialEq)]
pub enum Instruction {
    /// An instruction to call a function with the given list of arguments on the given package
    /// address and blueprint name.
    #[schemars(
        example = "crate::example::instruction::call_function1",
        example = "crate::example::instruction::call_function2",
        example = "crate::example::instruction::call_function3",
        example = "crate::example::instruction::call_function4"
    )]
    CallFunction {
        /// The address of the package containing the blueprint that contains the desired function.
        /// This package address is serialized as the `Address` variant of the
        /// `ManifestAstValue` model.
        package_address: ManifestAstValue,

        /// A string of the name of the blueprint containing the desired function. This field is
        /// serialized as a `String` from the ManifestAstValue model.
        blueprint_name: ManifestAstValue,

        /// A string of the name of the function to call. This field is serialized as a `String`
        /// from the ManifestAstValue model.
        function_name: ManifestAstValue,

        /// An optional array of `ManifestAstValue` arguments to call the function with. If this
        /// array is empty or is not provided, then the function is called with no
        /// arguments.
        arguments: Option<Vec<ManifestAstValue>>,
    },

    /// An instruction to call a method with a given name on a given component address with the
    /// given list of arguments.
    #[schemars(
        example = "crate::example::instruction::call_method1",
        example = "crate::example::instruction::call_method2",
        example = "crate::example::instruction::call_method3",
        example = "crate::example::instruction::call_method4"
    )]
    CallMethod {
        /// The address of the component which contains the method to be invoked. This field is
        /// serialized as an `Address` from the ManifestAstValue model.
        component_address: ManifestAstValue,

        /// A string of the name of the method to call. his field is serialized as a `String` from
        /// the ManifestAstValue model.
        method_name: ManifestAstValue,

        /// An optional array of `ManifestAstValue` arguments to call the method with. If this
        /// array is empty or is not provided, then the method is called with no arguments.
        arguments: Option<Vec<ManifestAstValue>>,
    },

    /// An instruction to take the entire amount of a given resource address from the worktop and
    /// put it in a bucket.
    #[schemars(
        example = "crate::example::instruction::take_from_worktop1",
        example = "crate::example::instruction::take_from_worktop2"
    )]
    TakeFromWorktop {
        /// The address of the resource to take from the worktop. This field is serialized as a
        /// `Address` from the ManifestAstValue model.
        resource_address: ManifestAstValue,

        /// A bucket to put the taken resources into. This field is serialized as a `Bucket` from
        /// the ManifestAstValue model.
        into_bucket: ManifestAstValue,
    },

    /// An instruction to take the an amount of a given resource address from the worktop and put
    /// it in a bucket.
    #[schemars(
        example = "crate::example::instruction::take_from_worktop_by_amount1",
        example = "crate::example::instruction::take_from_worktop_by_amount2"
    )]
    TakeFromWorktopByAmount {
        /// The address of the resource to take from the worktop. This field is serialized as a
        /// `Address` from the ManifestAstValue model.
        resource_address: ManifestAstValue,

        /// The amount of the resource to take from the worktop. This field is serialized as a
        /// `Decimal` from the ManifestAstValue model.
        amount: ManifestAstValue,

        /// A bucket to put the taken resources into. This field is serialized as a `Bucket` from
        /// the ManifestAstValue model.
        into_bucket: ManifestAstValue,
    },

    /// An instruction to take the a set of non-fungible ids of a given resource address from the
    /// worktop and put it in a bucket.
    #[schemars(
        example = "crate::example::instruction::take_from_worktop_by_ids1",
        example = "crate::example::instruction::take_from_worktop_by_ids2"
    )]
    TakeFromWorktopByIds {
        /// The address of the resource to take from the worktop. This field is serialized as a
        /// `Address` from the ManifestAstValue model.
        resource_address: ManifestAstValue,

        /// The non-fungible ids to take from the worktop. This is a set (serialized as a JSON
        /// array) of `NonFungibleLocalId`s from the ManifestAstValue model.
        #[schemars(with = "BTreeSet<crate::model::address::NonFungibleLocalId>")]
        ids: Vec<ManifestAstValue>,

        /// A bucket to put the taken resources into. This field is serialized as a `Bucket` from
        /// the ManifestAstValue model.
        into_bucket: ManifestAstValue,
    },

    /// Returns a bucket of tokens to the worktop.
    #[schemars(example = "crate::example::instruction::return_to_worktop")]
    ReturnToWorktop {
        /// The bucket to return to the worktop.
        bucket: ManifestAstValue,
    },

    /// An instruction to assert that a given resource exists in the worktop.
    #[schemars(
        example = "crate::example::instruction::assert_worktop_contains1",
        example = "crate::example::instruction::assert_worktop_contains2"
    )]
    AssertWorktopContains {
        /// The address of the resource to perform the assertion on. This field is serialized as a
        /// `Address` from the ManifestAstValue model.
        resource_address: ManifestAstValue,
    },

    /// An instruction to assert that a specific amount of a specific resource address exists in
    /// the worktop.
    #[schemars(
        example = "crate::example::instruction::assert_worktop_contains_by_amount1",
        example = "crate::example::instruction::assert_worktop_contains_by_amount2"
    )]
    AssertWorktopContainsByAmount {
        /// The address of the resource to perform the assertion on. This field is serialized as a
        /// `Address` from the ManifestAstValue model.
        resource_address: ManifestAstValue,

        /// The amount of the resource to assert their existence in the worktop. This field is
        /// serialized as a `Decimal` from the ManifestAstValue model.
        amount: ManifestAstValue,
    },

    /// An instruction to assert that a set ids of a specific resource address exists in the
    /// worktop.
    #[schemars(
        example = "crate::example::instruction::assert_worktop_contains_by_ids1",
        example = "crate::example::instruction::assert_worktop_contains_by_ids2"
    )]
    AssertWorktopContainsByIds {
        /// The address of the resource to perform the assertion on. This field is serialized as a
        /// `Address` from the ManifestAstValue model.
        resource_address: ManifestAstValue,

        /// The non-fungible ids of the resource to assert their existence in the worktop. This is
        /// a set (serialized as a JSON array) of `NonFungibleLocalId`s from the ManifestAstValue
        /// model.
        #[schemars(with = "BTreeSet<crate::model::address::NonFungibleLocalId>")]
        ids: Vec<ManifestAstValue>,
    },

    /// An instruction which pops a proof from the AuthZone stack and into an identifiable proof
    #[schemars(example = "crate::example::instruction::pop_from_auth_zone")]
    PopFromAuthZone {
        /// The proof to put the popped proof into. This is serialized as a `Proof` from the
        /// ManifestAstValue model.
        into_proof: ManifestAstValue,
    },

    /// An instruction that pushes a proof to the auth zone stack.
    #[schemars(example = "crate::example::instruction::push_to_auth_zone")]
    PushToAuthZone {
        /// The proof to push to the auth zone stack. This is serialized as a `Proof` from the
        /// ManifestAstValue model.
        proof: ManifestAstValue,
    },

    /// An instruction which clears the auth zone stack by dropping all of the proofs in that
    /// stack.
    #[schemars(example = "crate::example::instruction::clear_auth_zone")]
    ClearAuthZone,

    /// Clears all the proofs of signature virtual badges.
    #[schemars(example = "crate::example::instruction::clear_signature_proofs")]
    ClearSignatureProofs,

    /// An instruction to create a proof of the entire amount of a given resource address from the
    /// auth zone.
    #[schemars(
        example = "crate::example::instruction::create_proof_from_auth_zone1",
        example = "crate::example::instruction::create_proof_from_auth_zone2"
    )]
    CreateProofFromAuthZone {
        /// The address of the resource to create a proof of. This field is serialized as a
        /// `Address` from the ManifestAstValue model.
        resource_address: ManifestAstValue,

        /// A proof to put the resource proof into. This field is serialized as a `Proof` from the
        /// ManifestAstValue model.
        into_proof: ManifestAstValue,
    },

    /// An instruction to create a proof of the an amount of a given resource address from the auth
    /// zone.
    #[schemars(
        example = "crate::example::instruction::create_proof_from_auth_zone_by_amount1",
        example = "crate::example::instruction::create_proof_from_auth_zone_by_amount2"
    )]
    CreateProofFromAuthZoneByAmount {
        /// The address of the resource to create a proof of. This field is serialized as a
        /// `Address` from the ManifestAstValue model.
        resource_address: ManifestAstValue,

        /// The amount of the resource to create a proof of. This field is serialized as a
        /// `Decimal` from the ManifestAstValue model.
        amount: ManifestAstValue,

        /// A proof to put the resource proof into. This field is serialized as a `Proof` from the
        /// ManifestAstValue model.
        into_proof: ManifestAstValue,
    },

    /// An instruction to create a proof of the a set of non-fungible ids of a given resource
    /// address from the auth zone.
    #[schemars(
        example = "crate::example::instruction::create_proof_from_auth_zone_by_ids1",
        example = "crate::example::instruction::create_proof_from_auth_zone_by_ids2"
    )]
    CreateProofFromAuthZoneByIds {
        /// The address of the resource to create a proof of. This field is serialized as a
        /// `Address` from the ManifestAstValue model.
        resource_address: ManifestAstValue,

        /// The non-fungible ids to create a proof of. This is a set (serialized as a JSON array)
        /// of `NonFungibleLocalId`s from the ManifestAstValue model.
        #[schemars(with = "BTreeSet<crate::model::address::NonFungibleLocalId>")]
        ids: Vec<ManifestAstValue>,

        /// A proof to put the resource proof into. This field is serialized as a `Proof` from the
        /// ManifestAstValue model.
        into_proof: ManifestAstValue,
    },

    /// An instruction to create a proof given a bucket of some resources
    #[schemars(example = "crate::example::instruction::create_proof_from_bucket")]
    CreateProofFromBucket {
        /// The bucket of resources to create a proof from. This field is serialized as a `Bucket`
        /// from the ManifestAstValue model.
        bucket: ManifestAstValue,

        /// The proof variable that the proof should go to. This field is serialized as a `Proof`
        /// from the ManifestAstValue model.
        into_proof: ManifestAstValue,
    },

    /// An instruction to clone a proof creating a second proof identical to the original
    #[schemars(example = "crate::example::instruction::clone_proof")]
    CloneProof {
        /// The original proof, or the proof to be cloned. This field is serialized as a `Proof`
        /// from the ManifestAstValue model.
        proof: ManifestAstValue,

        /// The proof variable that the proof should go to. This field is serialized as a `Proof`
        /// from the ManifestAstValue model.
        into_proof: ManifestAstValue,
    },

    /// An instruction to drop a proof.
    #[schemars(example = "crate::example::instruction::drop_proof")]
    DropProof {
        /// The proof to drop. This field is serialized as a `Proof` from the ManifestAstValue
        /// model.
        proof: ManifestAstValue,
    },

    /// An instruction to drop all proofs currently present in the transaction context.
    #[schemars(example = "crate::example::instruction::drop_all_proofs")]
    DropAllProofs,

    /// An instruction to publish a package and set it's associated royalty configs, metadata,
    /// and access rules.
    #[schemars(example = "crate::example::instruction::publish_package")]
    PublishPackage {
        /// The blob of the package code. This field is serialized as a `Blob` from the
        /// ManifestAstValue model.
        code: ManifestAstValue,

        /// The blob of the package Schema. This is serialized as `Bytes` from the ManifestAstValue
        /// model.
        schema: ManifestAstValue,

        /// The configurations of the royalty for the package. The underlying type of this is a Map
        /// where the key is a string of the blueprint name and the value is a `RoyaltyConfig`.
        /// This is serialized as an `Map` from the ManifestAstValue model.
        royalty_config: ManifestAstValue,

        /// The metadata to use for the package. The underlying type of this is a string-string Map
        /// of the metadata. This is serialized as an `Map` from the ManifestAstValue model.
        metadata: ManifestAstValue,
    },

    /// An instruction to publish a package and set it's associated royalty configs, metadata,
    /// and access rules.
    #[schemars(example = "crate::example::instruction::publish_package_advanced")]
    PublishPackageAdvanced {
        /// The blob of the package code. This field is serialized as a `Blob` from the
        /// ManifestAstValue model.
        code: ManifestAstValue,

        /// The blob of the package Schema. This is serialized as `Bytes` from the ManifestAstValue
        /// model.
        schema: ManifestAstValue,

        /// The configurations of the royalty for the package. The underlying type of this is a Map
        /// where the key is a string of the blueprint name and the value is a `RoyaltyConfig`.
        /// This is serialized as an `Map` from the ManifestAstValue model.
        royalty_config: ManifestAstValue,

        /// The metadata to use for the package. The underlying type of this is a string-string Map
        /// of the metadata. This is serialized as an `Map` from the ManifestAstValue model.
        metadata: ManifestAstValue,

        /// The access rules to use for the package. This is serialized as a `Tuple` from the
        /// ManifestAstValue model.
        access_rules: ManifestAstValue,
    },

    /// An instruction to burn a bucket of tokens.
    #[schemars(example = "crate::example::instruction::burn_resource")]
    BurnResource {
        /// The bucket of tokens to burn.
        bucket: ManifestAstValue,
    },

    /// An instruction ot recall resources from a known vault.
    #[schemars(example = "crate::example::instruction::recall_resource")]
    RecallResource {
        /// The id of the vault of the tokens to recall. This field is serialized as an `Own` from
        /// the value model and is expected to be an `Own::Vault`.
        vault_id: ManifestAstValue,

        /// The amount of tokens to recall from the vault. This field is serialized as a `Decimal`
        /// field from the ManifestAstValue model.
        amount: ManifestAstValue,
    },

    /// An instruction to set the metadata on an entity.
    #[schemars(example = "crate::example::instruction::set_metadata")]
    SetMetadata {
        /// The address of the entity to set metadata on. This is a discriminated union of types
        /// where it can either be an `Address`, `Address`, `Address` or
        /// an `Address`.
        entity_address: ManifestAstValue,

        /// A string of the key to set the metadata for. This field is serialized as a `String`
        /// from the ManifestAstValue model.
        key: ManifestAstValue,

        /// A string of the value to set the metadata for. This field is serialized as a `String`
        /// from the ManifestAstValue model.
        value: ManifestAstValue,
    },

    /// An instruction to set the metadata on an entity.
    #[schemars(example = "crate::example::instruction::remove_metadata")]
    RemoveMetadata {
        /// The address of the entity to set metadata on. This is a discriminated union of types
        /// where it can either be an `Address`, `Address`, `Address` or
        /// an `Address`.
        entity_address: ManifestAstValue,

        /// A string of the key to remove the metadata for. This field is serialized as a `String`
        /// from the ManifestAstValue model.
        key: ManifestAstValue,
    },

    /// An instruction to modify the royalties of a package.
    #[schemars(example = "crate::example::instruction::set_package_royalty_config")]
    SetPackageRoyaltyConfig {
        /// The address of the package to set the royalty on. This is serialized as a
        /// `Address` from the ManifestAstValue model.
        package_address: ManifestAstValue,

        /// The configurations of the royalty for the package. The underlying type of this is a Map
        /// where the key is a string of the blueprint name and the value is a `RoyaltyConfig`.
        /// This is serialized as an `Map` from the ManifestAstValue model.
        royalty_config: ManifestAstValue,
    },

    /// An instruction to modify the royalties on a component
    #[schemars(example = "crate::example::instruction::set_component_royalty_config")]
    SetComponentRoyaltyConfig {
        /// The component address of the component to modify royalties for. This field is
        /// serialized as an `Address` from the ManifestAstValue model.
        component_address: ManifestAstValue,

        /// The royalty config to set on the component. This is an `Enum` from the
        /// `ManifestAstValue` model.
        royalty_config: ManifestAstValue,
    },

    /// An instruction to claim royalties of a package
    #[schemars(example = "crate::example::instruction::claim_package_royalty")]
    ClaimPackageRoyalty {
        /// The package address of the package to claim royalties for. This field is serialized as
        /// an `Address` from the ManifestAstValue model.
        package_address: ManifestAstValue,
    },

    /// An instruction to claim royalties of a component
    #[schemars(example = "crate::example::instruction::claim_component_royalty")]
    ClaimComponentRoyalty {
        /// The component address of the component to claim royalties for. This field is serialized
        /// as an `Address` from the ManifestAstValue model.
        component_address: ManifestAstValue,
    },

    /// An instruction to modify the access rules of a method that an entity has.
    #[schemars(example = "crate::example::instruction::set_method_access_rule")]
    SetMethodAccessRule {
        /// The entity address of the entity to modify the access rules for.
        entity_address: ManifestAstValue,

        /// The method key for the method to set the access rule of. This field is serialized as an
        /// `Enum` from the ManifestAstValue model
        key: ManifestAstValue,

        /// The new access rule to set in-place of the old one. This field is serialized as an
        /// `Enum` from the ManifestAstValue model
        rule: ManifestAstValue,
    },

    /// An instruction to mint fungible resources
    #[schemars(example = "crate::example::instruction::mint_fungible")]
    MintFungible {
        /// The address of the resource to mint tokens of. This field is serialized as a
        /// `Address` from the ManifestAstValue model.
        resource_address: ManifestAstValue,

        /// The amount of fungible tokens to mint of this resource. This field is serialized as a
        /// `Decimal` from the ManifestAstValue model.
        amount: ManifestAstValue,
    },

    /// An instruction to mint non-fungibles of a resource
    #[schemars(example = "crate::example::instruction::mint_non_fungible")]
    MintNonFungible {
        /// The address of the resource to mint tokens of. This field is serialized as a
        /// `Address` from the ManifestAstValue model.
        resource_address: ManifestAstValue,

        /// The non-fungible tokens to mint. The underlying type of this is a map which maps a
        /// `NonFungibleLocalId` to a tuple of two `ManifestAstValue` elements where each element
        /// is a struct of the immutable and mutable parts of the non-fungible data.
        entries: ManifestAstValue,
    },

    /// An instruction to mint non-fungibles of a non-fungible resource that uses UUID as the type
    /// id and perform auto incrimination of ID.
    #[schemars(example = "crate::example::instruction::mint_uuid_non_fungible")]
    MintUuidNonFungible {
        /// The address of the resource to mint tokens of. This field is serialized as a
        /// `Address` from the ManifestAstValue model.
        resource_address: ManifestAstValue,

        /// The non-fungible tokens to mint. The underlying type is a vector of tuples of two
        /// `ManifestAstValue` elements where each element is a struct of the immutable and mutable
        /// parts of the non-fungible data.
        entries: ManifestAstValue,
    },

    /// An instruction to create a new fungible resource.
    #[schemars(example = "crate::example::instruction::create_fungible_resource")]
    CreateFungibleResource {
        /// The divisibility of the resource. This field is serialized as a `U8` from the
        /// ManifestAstValue model.
        divisibility: ManifestAstValue,

        /// The metadata to set on the resource. The underlying type of this is a string-string Map
        /// of the metadata. This is serialized as an `Map` from the ManifestAstValue model.
        metadata: ManifestAstValue,

        /// The access rules to use for the resource. The underlying type of this is a map which
        /// maps a `ResourceMethodAuthKey` enum to a tuple of two `AccessRule`s denoting the
        /// current behavior and the mutability. This is serialized as an `Map` from the
        /// ManifestAstValue model.
        access_rules: ManifestAstValue,
    },

    /// An instruction to create a fungible resource with initial supply
    #[schemars(
        example = "crate::example::instruction::create_fungible_resource_with_initial_supply"
    )]
    CreateFungibleResourceWithInitialSupply {
        /// The divisibility of the resource. This field is serialized as a `U8` from the
        /// ManifestAstValue model.
        divisibility: ManifestAstValue,

        /// The metadata to set on the resource. The underlying type of this is a string-string Map
        /// of the metadata. This is serialized as an `Map` from the ManifestAstValue model.
        metadata: ManifestAstValue,

        /// The access rules to use for the resource. The underlying type of this is a map which
        /// maps a `ResourceMethodAuthKey` enum to a tuple of two `AccessRule`s denoting the
        /// current behavior and the mutability. This is serialized as an `Map` from the
        /// ManifestAstValue model.
        access_rules: ManifestAstValue,

        /// A decimal value of the initial supply to mint during resource creation. If present,
        /// this is serialized as a `Decimal` from the value model.
        initial_supply: ManifestAstValue,
    },

    /// An instruction to create a new non-fungible resource.
    #[schemars(example = "crate::example::instruction::create_non_fungible_resource")]
    CreateNonFungibleResource {
        /// The type of the non-fungible id to use for this resource. This field is serialized as
        /// an `Enum` from the ManifestAstValue model.
        id_type: ManifestAstValue,

        /// The schema that all non-fungibles of this resource must adhere to.
        schema: ManifestAstValue,

        /// The metadata to set on the resource. The underlying type of this is a string-string Map
        /// of the metadata. This is serialized as an `Map` from the ManifestAstValue model.
        metadata: ManifestAstValue,

        /// The access rules to use for the resource. The underlying type of this is a map which
        /// maps a `ResourceMethodAuthKey` enum to a tuple of two `AccessRule`s denoting the
        /// current behavior and the mutability. This is serialized as an `Map` from the
        /// ManifestAstValue model.
        access_rules: ManifestAstValue,
    },

    /// An instruction to create a non-fungible resource with an initial supply
    // #[schemars(
    //     example =
    // "crate::example::instruction::create_non_fungible_resource_with_initial_supply" )]
    CreateNonFungibleResourceWithInitialSupply {
        /// The type of the non-fungible id to use for this resource. This field is serialized as
        /// an `Enum` from the ManifestAstValue model.
        id_type: ManifestAstValue,

        /// The schema that all non-fungibles of this resource must adhere to.
        schema: ManifestAstValue,

        /// The metadata to set on the resource. The underlying type of this is a string-string Map
        /// of the metadata. This is serialized as an `Map` from the ManifestAstValue model.
        metadata: ManifestAstValue,

        /// The access rules to use for the resource. The underlying type of this is a map which
        /// maps a `ResourceMethodAuthKey` enum to a tuple of two `AccessRule`s denoting the
        /// current behavior and the mutability. This is serialized as an `Map` from the
        /// ManifestAstValue model.
        access_rules: ManifestAstValue,

        /// An optional initial supply for the non-fungible resource being created. The underlying
        /// type of this is a map which maps a `NonFungibleLocalId` to a tuple of two
        /// `ManifestAstValue` elements where each element is a struct of the immutable and
        /// mutable parts of the non-fungible data.
        initial_supply: ManifestAstValue,
    },

    /// Creates a new access controller native component with the passed set of rules as the
    /// current active rule set and the specified timed recovery delay in minutes.
    #[schemars(example = "crate::example::instruction::create_access_controller")]
    CreateAccessController {
        /// A bucket of the asset that will be controlled by the access controller. The underlying
        /// type of this is a `Bucket` from the `ManifestAstValue` model.
        controlled_asset: ManifestAstValue,

        /// The set of rules to use for the access controller's primary, confirmation, and recovery
        /// roles.
        rule_set: ManifestAstValue,

        /// The recovery delay in minutes to use for the access controller. The underlying type of
        /// this is an `Enum` or an `Option` from the `ManifestAstValue` model of an unsigned
        /// 32-bit integer of the time in minutes.
        timed_recovery_delay_in_minutes: ManifestAstValue,
    },

    /// Creates a validator given the public key of the owner who controls it
    #[schemars(example = "crate::example::instruction::create_validator")]
    CreateValidator {
        /// The ECDSA Secp256k1 public key of the owner of the validator. The underlying type of
        /// this is an `EcdsaSecp256k1PublicKey` from the `ManifestAstValue` model.
        key: ManifestAstValue,
    },

    /// Creates a new identity native component.
    #[schemars(example = "crate::example::instruction::create_identity")]
    CreateIdentity,

    /// Creates a new identity native component with the specified access rules config.
    #[schemars(example = "crate::example::instruction::create_identity_advanced")]
    CreateIdentityAdvanced { config: ManifestAstValue },

    /// Creates a new global account component.
    #[schemars(example = "crate::example::instruction::create_account")]
    CreateAccount,

    /// Creates a new global account component with the specified access rules config.
    #[schemars(example = "crate::example::instruction::create_account_advanced")]
    CreateAccountAdvanced { config: ManifestAstValue },
}
