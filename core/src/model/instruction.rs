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

use std::collections::HashSet;

use crate::engine_identifier::{BucketId, ProofId};
use crate::model::address::entity_address::*;
use crate::model::address::network_aware_address::*;
use crate::NonFungibleAddress;
use scrypto::prelude::{Decimal, EcdsaSecp256k1PublicKey, NonFungibleId};
use scrypto::runtime::{ManifestBlobRef, Own};
use serializable::serializable;

// NOTE: The model below should ALWAYS be kept up to date with that present in the Scrypto repo.
//       this model was authored for commit: e497a8b8c19fea8266337c5b3e5ada2e723153fc. When you
//       update the toolkit, do a diff against the commit above and the latest commit and update
//       based on that. Also, make sure to update the commit hash above.
//       https://github.com/radixdlt/radixdlt-scrypto/compare/old_commit_hash..new_commit_hash

/// The Instruction model defines the structure that transaction manifest instructions follow during
/// communication with the Radix Engine Toolkit
#[serializable]
#[serde(tag = "instruction", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Instruction {
    /// An instruction to call a function with the given list of arguments on the given package
    /// address and blueprint name.
    CallFunction {
        /// The address of the package containing the blueprint that contains the desired function.
        /// This package address is serialized as the `PackageAddress` variant of the `Value`
        /// model.
        #[schemars(with = "crate::Value")]
        #[serde_as(as = "serde_with::TryFromInto<crate::Value>")]
        package_address: NetworkAwarePackageAddress,

        /// A string of the name of the blueprint containing the desired function. This field is
        /// serialized as a `String` from the Value model.
        #[schemars(with = "crate::Value")]
        #[serde_as(as = "serde_with::TryFromInto<crate::Value>")]
        blueprint_name: String,

        /// A string of the name of the function to call. This field is serialized as a `String`
        /// from the Value model.
        #[schemars(with = "crate::Value")]
        #[serde_as(as = "serde_with::TryFromInto<crate::Value>")]
        function_name: String,

        /// An optional array of `Value` arguments to call the function with. If this array is
        /// empty or is not provided, then the function is called with no arguments.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        arguments: Option<Vec<crate::Value>>,
    },

    /// An instruction to call a method with a given name on a given component address with the
    /// given list of arguments.
    CallMethod {
        /// The address of the component which contains the method to be invoked. This field is
        /// serialized as a `ComponentAddress` from the Value model.
        #[schemars(with = "crate::Value")]
        #[serde_as(as = "serde_with::TryFromInto<crate::Value>")]
        component_address: NetworkAwareComponentAddress,

        /// A string of the name of the method to call. his field is serialized as a `String` from
        /// the Value model.
        #[schemars(with = "crate::Value")]
        #[serde_as(as = "serde_with::TryFromInto<crate::Value>")]
        method_name: String,

        /// An optional array of `Value` arguments to call the method with. If this array is empty
        /// or is not provided, then the method is called with no arguments.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        arguments: Option<Vec<crate::Value>>,
    },

    /// An instruction to take the entire amount of a given resource address from the worktop and
    /// put it in a bucket.
    TakeFromWorktop {
        /// The address of the resource to take from the worktop. This field is serialized as a
        /// `ResourceAddress` from the Value model.
        #[schemars(with = "crate::Value")]
        #[serde_as(as = "serde_with::TryFromInto<crate::Value>")]
        resource_address: NetworkAwareResourceAddress,

        /// A bucket to put the taken resources into. This field is serialized as a `Bucket` from
        /// the Value model.
        #[schemars(with = "crate::Value")]
        #[serde_as(as = "serde_with::TryFromInto<crate::Value>")]
        into_bucket: BucketId,
    },

    /// An instruction to take the an amount of a given resource address from the worktop and put
    /// it in a bucket.
    TakeFromWorktopByAmount {
        /// The address of the resource to take from the worktop. This field is serialized as a
        /// `ResourceAddress` from the Value model.
        #[schemars(with = "crate::Value")]
        #[serde_as(as = "serde_with::TryFromInto<crate::Value>")]
        resource_address: NetworkAwareResourceAddress,

        /// The amount of the resource to take from the worktop. This field is serialized as a
        /// `Decimal` from the Value model.
        #[schemars(with = "crate::Value")]
        #[serde_as(as = "serde_with::TryFromInto<crate::Value>")]
        amount: Decimal,

        /// A bucket to put the taken resources into. This field is serialized as a `Bucket` from
        /// the Value model.
        #[schemars(with = "crate::Value")]
        #[serde_as(as = "serde_with::TryFromInto<crate::Value>")]
        into_bucket: BucketId,
    },

    /// An instruction to take the a set of non-fungible ids of a given resource address from the
    /// worktop and put it in a bucket.
    TakeFromWorktopByIds {
        /// The address of the resource to take from the worktop. This field is serialized as a
        /// `ResourceAddress` from the Value model.
        #[schemars(with = "crate::Value")]
        #[serde_as(as = "serde_with::TryFromInto<crate::Value>")]
        resource_address: NetworkAwareResourceAddress,

        /// The non-fungible ids to take from the worktop. This is a set (serialized as a JSON
        /// array) of `NonFungibleId`s from the Value model.
        #[schemars(with = "HashSet<crate::Value>")]
        #[serde_as(as = "HashSet<serde_with::TryFromInto<crate::Value>>")]
        ids: HashSet<NonFungibleId>,

        /// A bucket to put the taken resources into. This field is serialized as a `Bucket` from
        /// the Value model.
        #[schemars(with = "crate::Value")]
        #[serde_as(as = "serde_with::TryFromInto<crate::Value>")]
        into_bucket: BucketId,
    },

    /// Returns a bucket of tokens to the worktop.
    ReturnToWorktop {
        /// The bucket to return to the worktop.
        #[schemars(with = "crate::Value")]
        #[serde_as(as = "serde_with::TryFromInto<crate::Value>")]
        bucket: BucketId,
    },

    /// An instruction to assert that a given resource exists in the worktop.
    AssertWorktopContains {
        /// The address of the resource to perform the assertion on. This field is serialized as a
        /// `ResourceAddress` from the Value model.
        #[schemars(with = "crate::Value")]
        #[serde_as(as = "serde_with::TryFromInto<crate::Value>")]
        resource_address: NetworkAwareResourceAddress,
    },

    /// An instruction to assert that a specific amount of a specific resource address exists in
    /// the worktop.
    AssertWorktopContainsByAmount {
        /// The address of the resource to perform the assertion on. This field is serialized as a
        /// `ResourceAddress` from the Value model.
        #[schemars(with = "crate::Value")]
        #[serde_as(as = "serde_with::TryFromInto<crate::Value>")]
        resource_address: NetworkAwareResourceAddress,

        /// The amount of the resource to assert their existence in the worktop. This field is
        /// serialized as a `Decimal` from the Value model.
        #[schemars(with = "crate::Value")]
        #[serde_as(as = "serde_with::TryFromInto<crate::Value>")]
        amount: Decimal,
    },

    /// An instruction to assert that a set ids of a specific resource address exists in the
    /// worktop.
    AssertWorktopContainsByIds {
        /// The address of the resource to perform the assertion on. This field is serialized as a
        /// `ResourceAddress` from the Value model.
        #[schemars(with = "crate::Value")]
        #[serde_as(as = "serde_with::TryFromInto<crate::Value>")]
        resource_address: NetworkAwareResourceAddress,

        /// The non-fungible ids of the resource to assert their existence in the worktop. This is
        /// a set (serialized as a JSON array) of `NonFungibleId`s from the Value model.
        #[schemars(with = "HashSet<crate::Value>")]
        #[serde_as(as = "HashSet<serde_with::TryFromInto<crate::Value>>")]
        ids: HashSet<NonFungibleId>,
    },

    /// An instruction which pops a proof from the AuthZone stack and into an identifiable proof
    PopFromAuthZone {
        /// The proof to put the popped proof into. This is serialized as a `Proof` from the Value
        /// model.
        #[schemars(with = "crate::Value")]
        #[serde_as(as = "serde_with::TryFromInto<crate::Value>")]
        into_proof: ProofId,
    },

    /// An instruction that pushes a proof to the auth zone stack.
    PushToAuthZone {
        /// The proof to push to the auth zone stack. This is serialized as a `Proof` from the
        /// Value model.
        #[schemars(with = "crate::Value")]
        #[serde_as(as = "serde_with::TryFromInto<crate::Value>")]
        proof: ProofId,
    },

    /// An instruction which clears the auth zone stack by dropping all of the proofs in that
    /// stack.
    ClearAuthZone,

    /// An instruction to create a proof of the entire amount of a given resource address from the
    /// auth zone.
    CreateProofFromAuthZone {
        /// The address of the resource to create a proof of. This field is serialized as a
        /// `ResourceAddress` from the Value model.
        #[schemars(with = "crate::Value")]
        #[serde_as(as = "serde_with::TryFromInto<crate::Value>")]
        resource_address: NetworkAwareResourceAddress,

        /// A proof to put the resource proof into. This field is serialized as a `Proof` from the
        /// Value model.
        #[schemars(with = "crate::Value")]
        #[serde_as(as = "serde_with::TryFromInto<crate::Value>")]
        into_proof: ProofId,
    },

    /// An instruction to create a proof of the an amount of a given resource address from the auth
    /// zone.
    CreateProofFromAuthZoneByAmount {
        /// The address of the resource to create a proof of. This field is serialized as a
        /// `ResourceAddress` from the Value model.
        #[schemars(with = "crate::Value")]
        #[serde_as(as = "serde_with::TryFromInto<crate::Value>")]
        resource_address: NetworkAwareResourceAddress,

        /// The amount of the resource to create a proof of. This field is serialized as a
        /// `Decimal` from the Value model.
        #[schemars(with = "crate::Value")]
        #[serde_as(as = "serde_with::TryFromInto<crate::Value>")]
        amount: Decimal,

        /// A proof to put the resource proof into. This field is serialized as a `Proof` from the
        /// Value model.
        #[schemars(with = "crate::Value")]
        #[serde_as(as = "serde_with::TryFromInto<crate::Value>")]
        into_proof: ProofId,
    },

    /// An instruction to create a proof of the a set of non-fungible ids of a given resource
    /// address from the auth zone.
    CreateProofFromAuthZoneByIds {
        /// The address of the resource to create a proof of. This field is serialized as a
        /// `ResourceAddress` from the Value model.
        #[schemars(with = "crate::Value")]
        #[serde_as(as = "serde_with::TryFromInto<crate::Value>")]
        resource_address: NetworkAwareResourceAddress,

        /// The non-fungible ids to create a proof of. This is a set (serialized as a JSON array)
        /// of `NonFungibleId`s from the Value model.
        #[schemars(with = "HashSet<crate::Value>")]
        #[serde_as(as = "HashSet<serde_with::TryFromInto<crate::Value>>")]
        ids: HashSet<NonFungibleId>,

        /// A proof to put the resource proof into. This field is serialized as a `Proof` from the
        /// Value model.
        #[schemars(with = "crate::Value")]
        #[serde_as(as = "serde_with::TryFromInto<crate::Value>")]
        into_proof: ProofId,
    },

    /// An instruction to create a proof given a bucket of some resources
    CreateProofFromBucket {
        /// The bucket of resources to create a proof from. This field is serialized as a `Bucket`
        /// from the Value model.
        #[schemars(with = "crate::Value")]
        #[serde_as(as = "serde_with::TryFromInto<crate::Value>")]
        bucket: BucketId,

        /// The proof variable that the proof should go to. This field is serialized as a `Proof`
        /// from the Value model.
        #[schemars(with = "crate::Value")]
        #[serde_as(as = "serde_with::TryFromInto<crate::Value>")]
        into_proof: ProofId,
    },

    /// An instruction to clone a proof creating a second proof identical to the original
    CloneProof {
        /// The original proof, or the proof to be cloned. This field is serialized as a `Proof`
        /// from the Value model.
        #[schemars(with = "crate::Value")]
        #[serde_as(as = "serde_with::TryFromInto<crate::Value>")]
        proof: ProofId,

        /// The proof variable that the proof should go to. This field is serialized as a `Proof`
        /// from the Value model.
        #[schemars(with = "crate::Value")]
        #[serde_as(as = "serde_with::TryFromInto<crate::Value>")]
        into_proof: ProofId,
    },

    /// An instruction to drop a proof.
    DropProof {
        /// The proof to drop. This field is serialized as a `Proof` from the Value model.
        #[schemars(with = "crate::Value")]
        #[serde_as(as = "serde_with::TryFromInto<crate::Value>")]
        proof: ProofId,
    },

    /// An instruction to drop all proofs currently present in the transaction context.
    DropAllProofs,

    /// An instruction to publish a package and set it's associated royalty configs, metadata,
    /// and access rules.
    PublishPackage {
        /// The blob of the package code. This field is serialized as a `Blob` from the Value
        /// model.
        #[schemars(with = "crate::Value")]
        #[serde_as(as = "serde_with::TryFromInto<crate::Value>")]
        code: ManifestBlobRef,

        /// The blob of the package ABI. This field is serialized as a `Blob` from the Value model.
        #[schemars(with = "crate::Value")]
        #[serde_as(as = "serde_with::TryFromInto<crate::Value>")]
        abi: ManifestBlobRef,

        /// The configurations of the royalty for the package. The underlying type of this is a Map
        /// where the key is a string of the blueprint name and the value is a `RoyaltyConfig`.
        /// This is serialized as an `Array` from the Value model.
        #[schemars(with = "crate::Value")]
        royalty_config: crate::Value,

        /// The metadata to use for the package. The underlying type of this is a string-string Map
        /// of the metadata. This is serialized as an `Array` from the Value model.
        #[schemars(with = "crate::Value")]
        metadata: crate::Value,

        /// The access rules to use for the package. This is serialized as a `Tuple` from the Value
        /// model.
        #[schemars(with = "crate::Value")]
        access_rules: crate::Value,
    },

    /// An instruction to publish a package with an associated "owner" badge where all of the
    /// authority on the package is in the hands of said owner.
    PublishPackageWithOwner {
        /// The blob of the package code. This field is serialized as a `Blob` from the Value
        /// model.
        #[schemars(with = "crate::Value")]
        #[serde_as(as = "serde_with::TryFromInto<crate::Value>")]
        code: ManifestBlobRef,

        /// The blob of the package ABI. This field is serialized as a `Blob` from the Value model.
        #[schemars(with = "crate::Value")]
        #[serde_as(as = "serde_with::TryFromInto<crate::Value>")]
        abi: ManifestBlobRef,

        /// The non-fungible address of the owner badge of this package. This field is serialized
        /// as a `NonFungibleAddress` from the Value model.
        #[schemars(with = "crate::Value")]
        #[serde_as(as = "serde_with::TryFromInto<crate::Value>")]
        owner_badge: NonFungibleAddress,
    },

    /// An instruction to burn a bucket of tokens.
    BurnResource {
        /// The bucket of tokens to burn.
        #[schemars(with = "crate::Value")]
        #[serde_as(as = "serde_with::TryFromInto<crate::Value>")]
        bucket: BucketId,
    },

    /// An instruction ot recall resources from a known vault.
    RecallResource {
        /// The id of the vault of the tokens to recall. This field is serialized as an `Own` from
        /// the value model and is expected to be an `Own::Vault`.
        #[schemars(with = "crate::Value")]
        #[serde_as(as = "serde_with::TryFromInto<crate::Value>")]
        vault_id: Own,

        /// The amount of tokens to recall from the vault. This field is serialized as a `Decimal`
        /// field from the Value model.
        #[schemars(with = "crate::Value")]
        #[serde_as(as = "serde_with::TryFromInto<crate::Value>")]
        amount: Decimal,
    },

    /// An instruction to set the metadata on an entity.
    SetMetadata {
        /// The address of the entity to set metadata on. This is a discriminated union of types
        /// where it can either be a `ResourceAddress`, `ComponentAddress`, `PackageAddress` or
        /// a `SystemAddress`.
        entity_address: EntityAddress,

        /// A string of the key to set the metadata for. This field is serialized as a `String`
        /// from the Value model.
        #[schemars(with = "crate::Value")]
        #[serde_as(as = "serde_with::TryFromInto<crate::Value>")]
        key: String,

        /// A string of the value to set the metadata for. This field is serialized as a `String`
        /// from the Value model.
        #[schemars(with = "crate::Value")]
        #[serde_as(as = "serde_with::TryFromInto<crate::Value>")]
        value: String,
    },

    /// An instruction to modify the royalties of a package.
    SetPackageRoyaltyConfig {
        /// The address of the package to set the royalty on. This is serialized as a
        /// `PackageAddress` from the Value model.
        #[schemars(with = "crate::Value")]
        #[serde_as(as = "serde_with::TryFromInto<crate::Value>")]
        package_address: NetworkAwarePackageAddress,

        /// The configurations of the royalty for the package. The underlying type of this is a Map
        /// where the key is a string of the blueprint name and the value is a `RoyaltyConfig`.
        /// This is serialized as an `Array` from the Value model.
        #[schemars(with = "crate::Value")]
        royalty_config: crate::Value,
    },

    /// An instruction to modify the royalties on a component
    SetComponentRoyaltyConfig {
        /// The component address of the component to modify royalties for. This field is
        /// serialized as a `ComponentAddress` from the Value model.
        #[schemars(with = "crate::Value")]
        #[serde_as(as = "serde_with::TryFromInto<crate::Value>")]
        component_address: NetworkAwareComponentAddress,

        /// The royalty config to set on the component. This is an `Enum` from the `Value` model.
        #[schemars(with = "crate::Value")]
        royalty_config: crate::Value,
    },

    /// An instruction to claim royalties of a package
    ClaimPackageRoyalty {
        /// The package address of the package to claim royalties for. This field is serialized as
        /// a `PackageAddress` from the Value model.
        #[schemars(with = "crate::Value")]
        #[serde_as(as = "serde_with::TryFromInto<crate::Value>")]
        package_address: NetworkAwarePackageAddress,
    },

    /// An instruction to claim royalties of a component
    ClaimComponentRoyalty {
        /// The component address of the component to claim royalties for. This field is serialized
        /// as a `ComponentAddress` from the Value model.
        #[schemars(with = "crate::Value")]
        #[serde_as(as = "serde_with::TryFromInto<crate::Value>")]
        component_address: NetworkAwareComponentAddress,
    },

    /// An instruction to modify the access rules of a method that an entity has.
    SetMethodAccessRule {
        /// The entity address of the entity to modify the access rules for.
        entity_address: EntityAddress,

        /// Entity access rules is a stack of access rules, this index allows referring to a
        /// specific "layer" in said stack. This field is serialized as a `U32` from the `Value`
        /// model.
        #[schemars(with = "crate::Value")]
        #[serde_as(as = "serde_with::TryFromInto<crate::Value>")]
        index: u32,

        /// The method key for the method to set the access rule of. This field is serialized as an
        /// `Enum` from the Value model
        #[schemars(with = "crate::Value")]
        key: crate::Value,

        /// The new access rule to set in-place of the old one. This field is serialized as an
        /// `Enum` from the Value model
        #[schemars(with = "crate::Value")]
        rule: crate::Value,
    },

    /// An instruction to mint fungible resources
    MintFungible {
        /// The address of the resource to mint tokens of. This field is serialized as a
        /// `ResourceAddress` from the Value model.
        #[schemars(with = "crate::Value")]
        #[serde_as(as = "serde_with::TryFromInto<crate::Value>")]
        resource_address: NetworkAwareResourceAddress,

        /// The amount of fungible tokens to mint of this resource. This field is serialized as a
        /// `Decimal` from the Value model.
        #[schemars(with = "crate::Value")]
        #[serde_as(as = "serde_with::TryFromInto<crate::Value>")]
        amount: Decimal,
    },

    /// An instruction to mind non-fungibles of a resource
    MintNonFungible {
        /// The address of the resource to mint tokens of. This field is serialized as a
        /// `ResourceAddress` from the Value model.
        #[schemars(with = "crate::Value")]
        #[serde_as(as = "serde_with::TryFromInto<crate::Value>")]
        resource_address: NetworkAwareResourceAddress,

        /// The non-fungible tokens to mint. The underlying type of this is a map which maps a
        /// `NonFungibleId` to a tuple of two `Value` elements where each element is a struct of
        /// the immutable and mutable parts of the non-fungible data.
        #[schemars(with = "crate::Value")]
        entries: crate::Value,
    },

    /// An instruction to create a new fungible resource.
    CreateFungibleResource {
        /// The divisibility of the resource. This field is serialized as a `U8` from the Value
        /// model.
        #[schemars(with = "crate::Value")]
        #[serde_as(as = "serde_with::TryFromInto<crate::Value>")]
        divisibility: u8,

        /// The metadata to set on the resource. The underlying type of this is a string-string Map
        /// of the metadata. This is serialized as an `Array` from the Value model.
        #[schemars(with = "crate::Value")]
        metadata: crate::Value,

        /// The access rules to use for the resource. The underlying type of this is a map which
        /// maps a `ResourceMethodAuthKey` enum to a tuple of two `AccessRule`s denoting the
        /// current behavior and the mutability. This is serialized as an `Array` from the
        /// Value model.
        #[schemars(with = "crate::Value")]
        access_rules: crate::Value,

        /// An optional decimal value of the initial supply to mint during resource creation. If
        /// present, this is serialized as a `Decimal` from the value model.
        #[schemars(with = "Option<crate::Value>")]
        #[serde_as(as = "Option<serde_with::TryFromInto<crate::Value>>")]
        initial_supply: Option<Decimal>,
    },

    /// An instruction to create a fungible resource with an associated "owner" badge where all of
    /// the authority on the resource is in the hands of said owner.
    CreateFungibleResourceWithOwner {
        /// The divisibility of the resource. This field is serialized as a `U8` from the Value
        /// model.
        #[schemars(with = "crate::Value")]
        #[serde_as(as = "serde_with::TryFromInto<crate::Value>")]
        divisibility: u8,

        /// The metadata to set on the resource. The underlying type of this is a string-string Map
        /// of the metadata. This is serialized as an `Array` from the Value model.
        #[schemars(with = "crate::Value")]
        metadata: crate::Value,

        /// The non-fungible address of the owner badge of this resource. This field is serialized
        /// as a `NonFungibleAddress` from the Value model.
        #[schemars(with = "crate::Value")]
        #[serde_as(as = "serde_with::TryFromInto<crate::Value>")]
        owner_badge: NonFungibleAddress,

        /// An optional decimal value of the initial supply to mint during resource creation. If
        /// present, this is serialized as a `Decimal` from the value model.
        #[schemars(with = "Option<crate::Value>")]
        #[serde_as(as = "Option<serde_with::TryFromInto<crate::Value>>")]
        initial_supply: Option<Decimal>,
    },

    /// An instruction to create a new non-fungible resource.
    CreateNonFungibleResource {
        /// The type of the non-fungible id to use for this resource. This field is serialized as
        /// an `Enum` from the Value model.
        #[schemars(with = "crate::Value")]
        id_type: crate::Value,

        /// The metadata to set on the resource. The underlying type of this is a string-string Map
        /// of the metadata. This is serialized as an `Array` from the Value model.
        #[schemars(with = "crate::Value")]
        metadata: crate::Value,

        /// The access rules to use for the resource. The underlying type of this is a map which
        /// maps a `ResourceMethodAuthKey` enum to a tuple of two `AccessRule`s denoting the
        /// current behavior and the mutability. This is serialized as an `Array` from the
        /// Value model.
        #[schemars(with = "crate::Value")]
        access_rules: crate::Value,

        /// An optional initial supply for the non-fungible resource being created. The underlying
        /// type of this is a map which maps a `NonFungibleId` to a tuple of two `Value`
        /// elements where each element is a struct of the immutable and mutable parts of
        /// the non-fungible data.
        #[schemars(with = "Option<crate::Value>")]
        initial_supply: crate::Value,
    },

    /// An instruction to create a non-fungible resource with an associated "owner" badge where all
    /// of the authority on the resource is in the hands of said owner.
    CreateNonFungibleResourceWithOwner {
        /// The type of the non-fungible id to use for this resource. This field is serialized as
        /// an `Enum` from the Value model.
        #[schemars(with = "crate::Value")]
        id_type: crate::Value,

        /// The metadata to set on the resource. The underlying type of this is a string-string Map
        /// of the metadata. This is serialized as an `Array` from the Value model.
        #[schemars(with = "crate::Value")]
        metadata: crate::Value,

        /// The non-fungible address of the owner badge of this resource. This field is serialized
        /// as a `NonFungibleAddress` from the Value model.
        #[schemars(with = "crate::Value")]
        #[serde_as(as = "serde_with::TryFromInto<crate::Value>")]
        owner_badge: NonFungibleAddress,

        /// An optional initial supply for the non-fungible resource being created. The underlying
        /// type of this is a map which maps a `NonFungibleId` to a tuple of two `Value`
        /// elements where each element is a struct of the immutable and mutable parts of
        /// the non-fungible data.
        #[schemars(with = "Option<crate::Value>")]
        initial_supply: crate::Value,
    },

    /// An instruction to registers a new validator given the public key of the validator
    RegisterValidator {
        /// The public key of the validator
        #[schemars(with = "crate::Value")]
        #[serde_as(as = "serde_with::TryFromInto<crate::Value>")]
        validator: EcdsaSecp256k1PublicKey,
    },

    /// An instruction to unregister a validator given it's public key
    UnregisterValidator {
        /// The public key of the validator to unregister
        #[schemars(with = "crate::Value")]
        #[serde_as(as = "serde_with::TryFromInto<crate::Value>")]
        validator: EcdsaSecp256k1PublicKey,
    },
}
