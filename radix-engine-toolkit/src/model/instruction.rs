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

use std::collections::BTreeSet;

use crate::address::Bech32Coder;
use crate::error::Result;
use crate::Value;

use native_transaction::manifest::ast;

use serializable::serializable;

// NOTE: The model below should ALWAYS be kept up to date with that present in the Scrypto repo.
//       this model was authored for commit: e497a8b8c19fea8266337c5b3e5ada2e723153fc. When you
//       update the toolkit, do a diff against the commit above and the latest commit and update
//       based on that. Also, make sure to update the commit hash above.
//       https://github.com/radixdlt/radixdlt-scrypto/compare/old_commit_hash..new_commit_hash

// =================
// Model Definition
// =================

/// The Instruction model defines the structure that transaction manifest instructions follow during
/// communication with the Radix Engine Toolkit
#[serializable]
#[serde(tag = "instruction", rename_all = "SCREAMING_SNAKE_CASE")]
#[derive(Eq, PartialEq)]
pub enum Instruction {
    /// An instruction to call a function with the given list of arguments on the given package
    /// address and blueprint name.
    CallFunction {
        /// The address of the package containing the blueprint that contains the desired function.
        /// This package address is serialized as the `PackageAddress` variant of the `Value`
        /// model.
        package_address: Value,

        /// A string of the name of the blueprint containing the desired function. This field is
        /// serialized as a `String` from the Value model.
        blueprint_name: Value,

        /// A string of the name of the function to call. This field is serialized as a `String`
        /// from the Value model.
        function_name: Value,

        /// An optional array of `Value` arguments to call the function with. If this array is
        /// empty or is not provided, then the function is called with no arguments.
        arguments: Option<Vec<Value>>,
    },

    /// An instruction to call a method with a given name on a given component address with the
    /// given list of arguments.
    CallMethod {
        /// The address of the component which contains the method to be invoked. This field is
        /// serialized as a `ComponentAddress` from the Value model.
        component_address: Value,

        /// A string of the name of the method to call. his field is serialized as a `String` from
        /// the Value model.
        method_name: Value,

        /// An optional array of `Value` arguments to call the method with. If this array is empty
        /// or is not provided, then the method is called with no arguments.
        arguments: Option<Vec<Value>>,
    },

    /// An instruction to take the entire amount of a given resource address from the worktop and
    /// put it in a bucket.
    TakeFromWorktop {
        /// The address of the resource to take from the worktop. This field is serialized as a
        /// `ResourceAddress` from the Value model.
        resource_address: Value,

        /// A bucket to put the taken resources into. This field is serialized as a `Bucket` from
        /// the Value model.
        into_bucket: Value,
    },

    /// An instruction to take the an amount of a given resource address from the worktop and put
    /// it in a bucket.
    TakeFromWorktopByAmount {
        /// The address of the resource to take from the worktop. This field is serialized as a
        /// `ResourceAddress` from the Value model.
        resource_address: Value,

        /// The amount of the resource to take from the worktop. This field is serialized as a
        /// `Decimal` from the Value model.
        amount: Value,

        /// A bucket to put the taken resources into. This field is serialized as a `Bucket` from
        /// the Value model.
        into_bucket: Value,
    },

    /// An instruction to take the a set of non-fungible ids of a given resource address from the
    /// worktop and put it in a bucket.
    TakeFromWorktopByIds {
        /// The address of the resource to take from the worktop. This field is serialized as a
        /// `ResourceAddress` from the Value model.
        resource_address: Value,

        /// The non-fungible ids to take from the worktop. This is a set (serialized as a JSON
        /// array) of `NonFungibleLocalId`s from the Value model.
        #[schemars(with = "BTreeSet<crate::model::address::NonFungibleLocalId>")]
        ids: Vec<Value>,

        /// A bucket to put the taken resources into. This field is serialized as a `Bucket` from
        /// the Value model.
        into_bucket: Value,
    },

    /// Returns a bucket of tokens to the worktop.
    ReturnToWorktop {
        /// The bucket to return to the worktop.
        bucket: Value,
    },

    /// An instruction to assert that a given resource exists in the worktop.
    AssertWorktopContains {
        /// The address of the resource to perform the assertion on. This field is serialized as a
        /// `ResourceAddress` from the Value model.
        resource_address: Value,
    },

    /// An instruction to assert that a specific amount of a specific resource address exists in
    /// the worktop.
    AssertWorktopContainsByAmount {
        /// The address of the resource to perform the assertion on. This field is serialized as a
        /// `ResourceAddress` from the Value model.
        resource_address: Value,

        /// The amount of the resource to assert their existence in the worktop. This field is
        /// serialized as a `Decimal` from the Value model.
        amount: Value,
    },

    /// An instruction to assert that a set ids of a specific resource address exists in the
    /// worktop.
    AssertWorktopContainsByIds {
        /// The address of the resource to perform the assertion on. This field is serialized as a
        /// `ResourceAddress` from the Value model.
        resource_address: Value,

        /// The non-fungible ids of the resource to assert their existence in the worktop. This is
        /// a set (serialized as a JSON array) of `NonFungibleLocalId`s from the Value model.
        #[schemars(with = "BTreeSet<crate::model::address::NonFungibleLocalId>")]
        ids: Vec<Value>,
    },

    /// An instruction which pops a proof from the AuthZone stack and into an identifiable proof
    PopFromAuthZone {
        /// The proof to put the popped proof into. This is serialized as a `Proof` from the Value
        /// model.
        into_proof: Value,
    },

    /// An instruction that pushes a proof to the auth zone stack.
    PushToAuthZone {
        /// The proof to push to the auth zone stack. This is serialized as a `Proof` from the
        /// Value model.
        proof: Value,
    },

    /// An instruction which clears the auth zone stack by dropping all of the proofs in that
    /// stack.
    ClearAuthZone,

    /// An instruction to create a proof of the entire amount of a given resource address from the
    /// auth zone.
    CreateProofFromAuthZone {
        /// The address of the resource to create a proof of. This field is serialized as a
        /// `ResourceAddress` from the Value model.
        resource_address: Value,

        /// A proof to put the resource proof into. This field is serialized as a `Proof` from the
        /// Value model.
        into_proof: Value,
    },

    /// An instruction to create a proof of the an amount of a given resource address from the auth
    /// zone.
    CreateProofFromAuthZoneByAmount {
        /// The address of the resource to create a proof of. This field is serialized as a
        /// `ResourceAddress` from the Value model.
        resource_address: Value,

        /// The amount of the resource to create a proof of. This field is serialized as a
        /// `Decimal` from the Value model.
        amount: Value,

        /// A proof to put the resource proof into. This field is serialized as a `Proof` from the
        /// Value model.
        into_proof: Value,
    },

    /// An instruction to create a proof of the a set of non-fungible ids of a given resource
    /// address from the auth zone.
    CreateProofFromAuthZoneByIds {
        /// The address of the resource to create a proof of. This field is serialized as a
        /// `ResourceAddress` from the Value model.
        resource_address: Value,

        /// The non-fungible ids to create a proof of. This is a set (serialized as a JSON array)
        /// of `NonFungibleLocalId`s from the Value model.
        #[schemars(with = "BTreeSet<crate::model::address::NonFungibleLocalId>")]
        ids: Vec<Value>,

        /// A proof to put the resource proof into. This field is serialized as a `Proof` from the
        /// Value model.
        into_proof: Value,
    },

    /// An instruction to create a proof given a bucket of some resources
    CreateProofFromBucket {
        /// The bucket of resources to create a proof from. This field is serialized as a `Bucket`
        /// from the Value model.
        bucket: Value,

        /// The proof variable that the proof should go to. This field is serialized as a `Proof`
        /// from the Value model.
        into_proof: Value,
    },

    /// An instruction to clone a proof creating a second proof identical to the original
    CloneProof {
        /// The original proof, or the proof to be cloned. This field is serialized as a `Proof`
        /// from the Value model.
        proof: Value,

        /// The proof variable that the proof should go to. This field is serialized as a `Proof`
        /// from the Value model.
        into_proof: Value,
    },

    /// An instruction to drop a proof.
    DropProof {
        /// The proof to drop. This field is serialized as a `Proof` from the Value model.
        proof: Value,
    },

    /// An instruction to drop all proofs currently present in the transaction context.
    DropAllProofs,

    /// An instruction to publish a package and set it's associated royalty configs, metadata,
    /// and access rules.
    PublishPackage {
        /// The blob of the package code. This field is serialized as a `Blob` from the Value
        /// model.
        code: Value,

        /// The blob of the package ABI. This field is serialized as a `Blob` from the Value model.
        abi: Value,

        /// The configurations of the royalty for the package. The underlying type of this is a Map
        /// where the key is a string of the blueprint name and the value is a `RoyaltyConfig`.
        /// This is serialized as an `Map` from the Value model.
        royalty_config: Value,

        /// The metadata to use for the package. The underlying type of this is a string-string Map
        /// of the metadata. This is serialized as an `Map` from the Value model.
        metadata: Value,

        /// The access rules to use for the package. This is serialized as a `Tuple` from the Value
        /// model.
        access_rules: Value,
    },

    /// An instruction to publish a package with an associated "owner" badge where all of the
    /// authority on the package is in the hands of said owner.
    PublishPackageWithOwner {
        /// The blob of the package code. This field is serialized as a `Blob` from the Value
        /// model.
        code: Value,

        /// The blob of the package ABI. This field is serialized as a `Blob` from the Value model.
        abi: Value,

        /// The non-fungible address of the owner badge of this package. This field is serialized
        /// as a `NonFungibleGlobalId` from the Value model.
        owner_badge: Value,
    },

    /// An instruction to burn a bucket of tokens.
    BurnResource {
        /// The bucket of tokens to burn.
        bucket: Value,
    },

    /// An instruction ot recall resources from a known vault.
    RecallResource {
        /// The id of the vault of the tokens to recall. This field is serialized as an `Own` from
        /// the value model and is expected to be an `Own::Vault`.
        vault_id: Value,

        /// The amount of tokens to recall from the vault. This field is serialized as a `Decimal`
        /// field from the Value model.
        amount: Value,
    },

    /// An instruction to set the metadata on an entity.
    SetMetadata {
        /// The address of the entity to set metadata on. This is a discriminated union of types
        /// where it can either be a `ResourceAddress`, `ComponentAddress`, `PackageAddress` or
        /// a `ComponentAddress`.
        entity_address: Value,

        /// A string of the key to set the metadata for. This field is serialized as a `String`
        /// from the Value model.
        key: Value,

        /// A string of the value to set the metadata for. This field is serialized as a `String`
        /// from the Value model.
        value: Value,
    },

    /// An instruction to modify the royalties of a package.
    SetPackageRoyaltyConfig {
        /// The address of the package to set the royalty on. This is serialized as a
        /// `PackageAddress` from the Value model.
        package_address: Value,

        /// The configurations of the royalty for the package. The underlying type of this is a Map
        /// where the key is a string of the blueprint name and the value is a `RoyaltyConfig`.
        /// This is serialized as an `Map` from the Value model.
        royalty_config: Value,
    },

    /// An instruction to modify the royalties on a component
    SetComponentRoyaltyConfig {
        /// The component address of the component to modify royalties for. This field is
        /// serialized as a `ComponentAddress` from the Value model.
        component_address: Value,

        /// The royalty config to set on the component. This is an `Enum` from the `Value` model.
        royalty_config: Value,
    },

    /// An instruction to claim royalties of a package
    ClaimPackageRoyalty {
        /// The package address of the package to claim royalties for. This field is serialized as
        /// a `PackageAddress` from the Value model.
        package_address: Value,
    },

    /// An instruction to claim royalties of a component
    ClaimComponentRoyalty {
        /// The component address of the component to claim royalties for. This field is serialized
        /// as a `ComponentAddress` from the Value model.
        component_address: Value,
    },

    /// An instruction to modify the access rules of a method that an entity has.
    SetMethodAccessRule {
        /// The entity address of the entity to modify the access rules for.
        entity_address: Value,

        /// Entity access rules is a stack of access rules, this index allows referring to a
        /// specific "layer" in said stack. This field is serialized as a `U32` from the `Value`
        /// model.
        index: Value,

        /// The method key for the method to set the access rule of. This field is serialized as an
        /// `Enum` from the Value model
        key: Value,

        /// The new access rule to set in-place of the old one. This field is serialized as an
        /// `Enum` from the Value model
        rule: Value,
    },

    /// An instruction to mint fungible resources
    MintFungible {
        /// The address of the resource to mint tokens of. This field is serialized as a
        /// `ResourceAddress` from the Value model.
        resource_address: Value,

        /// The amount of fungible tokens to mint of this resource. This field is serialized as a
        /// `Decimal` from the Value model.
        amount: Value,
    },

    /// An instruction to mint non-fungibles of a resource
    MintNonFungible {
        /// The address of the resource to mint tokens of. This field is serialized as a
        /// `ResourceAddress` from the Value model.
        resource_address: Value,

        /// The non-fungible tokens to mint. The underlying type of this is a map which maps a
        /// `NonFungibleLocalId` to a tuple of two `Value` elements where each element is a struct
        /// of the immutable and mutable parts of the non-fungible data.
        entries: Value,
    },

    /// An instruction to mint non-fungibles of a non-fungible resource that uses UUID as the type
    /// id and perform auto incrimination of ID.
    MintUuidNonFungible {
        /// The address of the resource to mint tokens of. This field is serialized as a
        /// `ResourceAddress` from the Value model.
        resource_address: Value,

        /// The non-fungible tokens to mint. The underlying type is a vector of tuples of two
        /// `Value` elements where each element is a struct of the immutable and mutable
        /// parts of the non-fungible data.
        entries: Value,
    },

    /// An instruction to create a new fungible resource.
    CreateFungibleResource {
        /// The divisibility of the resource. This field is serialized as a `U8` from the Value
        /// model.
        divisibility: Value,

        /// The metadata to set on the resource. The underlying type of this is a string-string Map
        /// of the metadata. This is serialized as an `Map` from the Value model.
        metadata: Value,

        /// The access rules to use for the resource. The underlying type of this is a map which
        /// maps a `ResourceMethodAuthKey` enum to a tuple of two `AccessRule`s denoting the
        /// current behavior and the mutability. This is serialized as an `Map` from the
        /// Value model.
        access_rules: Value,

        /// An optional decimal value of the initial supply to mint during resource creation. If
        /// present, this is serialized as a `Decimal` from the value model.
        initial_supply: Value,
    },

    /// An instruction to create a fungible resource with an associated "owner" badge where all of
    /// the authority on the resource is in the hands of said owner.
    CreateFungibleResourceWithOwner {
        /// The divisibility of the resource. This field is serialized as a `U8` from the Value
        /// model.
        divisibility: Value,

        /// The metadata to set on the resource. The underlying type of this is a string-string Map
        /// of the metadata. This is serialized as an `Map` from the Value model.
        metadata: Value,

        /// The non-fungible address of the owner badge of this resource. This field is serialized
        /// as a `NonFungibleGlobalId` from the Value model.
        owner_badge: Value,

        /// An optional decimal value of the initial supply to mint during resource creation. If
        /// present, this is serialized as a `Decimal` from the value model.
        initial_supply: Value,
    },

    /// An instruction to create a new non-fungible resource.
    CreateNonFungibleResource {
        /// The type of the non-fungible id to use for this resource. This field is serialized as
        /// an `Enum` from the Value model.
        id_type: Value,

        /// The metadata to set on the resource. The underlying type of this is a string-string Map
        /// of the metadata. This is serialized as an `Map` from the Value model.
        metadata: Value,

        /// The access rules to use for the resource. The underlying type of this is a map which
        /// maps a `ResourceMethodAuthKey` enum to a tuple of two `AccessRule`s denoting the
        /// current behavior and the mutability. This is serialized as an `Map` from the
        /// Value model.
        access_rules: Value,

        /// An optional initial supply for the non-fungible resource being created. The underlying
        /// type of this is a map which maps a `NonFungibleLocalId` to a tuple of two `Value`
        /// elements where each element is a struct of the immutable and mutable parts of
        /// the non-fungible data.
        initial_supply: Value,
    },

    /// An instruction to create a non-fungible resource with an associated "owner" badge where all
    /// of the authority on the resource is in the hands of said owner.
    CreateNonFungibleResourceWithOwner {
        /// The type of the non-fungible id to use for this resource. This field is serialized as
        /// an `Enum` from the Value model.
        id_type: Value,

        /// The metadata to set on the resource. The underlying type of this is a string-string Map
        /// of the metadata. This is serialized as an `Map` from the Value model.
        metadata: Value,

        /// The non-fungible address of the owner badge of this resource. This field is serialized
        /// as a `NonFungibleGlobalId` from the Value model.
        owner_badge: Value,

        /// An optional initial supply for the non-fungible resource being created. The underlying
        /// type of this is a map which maps a `NonFungibleLocalId` to a tuple of two `Value`
        /// elements where each element is a struct of the immutable and mutable parts of
        /// the non-fungible data.
        initial_supply: Value,
    },

    /// Creates a new access controller native component with the passed set of rules as the
    /// current active rule set and the specified timed recovery delay in minutes.
    CreateAccessController {
        /// A bucket of the asset that will be controlled by the access controller. The underlying
        /// type of this is a `Bucket` from the `Value` model.
        controlled_asset: Value,

        /// The access rule to use for the primary role of the access controller. The underlying
        /// type of this is an `Enum` from the `Value` model.
        primary_role: Value,

        /// The access rule to use for the recovery role of the access controller. The underlying
        /// type of this is an `Enum` from the `Value` model.
        recovery_role: Value,

        /// The access rule to use for the confirmation role of the access controller. The
        /// underlying type of this is an `Enum` from the `Value` model.
        confirmation_role: Value,

        /// The recovery delay in minutes to use for the access controller. The underlying type of
        /// this is an `Enum` or an `Option` from the `Value` model of an unsigned 32-bit integer
        /// of the time in minutes.
        timed_recovery_delay_in_minutes: Value,
    },

    /// Creates a new identity native component with the passed access rule.
    CreateIdentity {
        /// The access rule to protect the identity with. The underlying type of this is an `Enum`
        /// from the `Value` model.
        access_rule: Value,
    },

    /// Assert that the given access rule is currently fulfilled by the proofs in the Auth Zone of
    /// the transaction
    AssertAccessRule {
        /// The access rule to assert. The underlying type of this is an `Enum` from the `Value`
        /// model which represents the access rule to assert.
        access_rule: Value,
    },

    /// Creates a validator given the public key of the owner who controls it
    CreateValidator {
        /// The ECDSA Secp256k1 public key of the owner of the validator. The underlying type of
        /// this is an `EcdsaSecp256k1PublicKey` from the `Value` model.
        key: Value,

        /// The access rule to protect the validator with. The underlying type of this is an `Enum`
        /// from the `Value` model which represents the access rule to assert.
        owner_access_rule: Value,
    },
}

// ============
// Conversions
// ============

impl Instruction {
    pub fn to_ast_instruction(&self, bech32_coder: &Bech32Coder) -> Result<ast::Instruction> {
        let ast_instruction = match self.clone() {
            Self::CallFunction {
                package_address,
                blueprint_name,
                function_name,
                arguments,
            } => ast::Instruction::CallFunction {
                package_address: package_address.to_ast_value(bech32_coder)?,
                blueprint_name: blueprint_name.to_ast_value(bech32_coder)?,
                function_name: function_name.to_ast_value(bech32_coder)?,
                args: arguments
                    .unwrap_or_default()
                    .iter()
                    .map(|value| value.to_ast_value(bech32_coder))
                    .collect::<Result<Vec<ast::Value>>>()?,
            },
            Self::CallMethod {
                component_address,
                method_name,
                arguments,
            } => ast::Instruction::CallMethod {
                component_address: component_address.to_ast_value(bech32_coder)?,
                method_name: method_name.to_ast_value(bech32_coder)?,
                args: arguments
                    .unwrap_or_default()
                    .iter()
                    .map(|value| value.to_ast_value(bech32_coder))
                    .collect::<Result<Vec<ast::Value>>>()?,
            },
            Self::TakeFromWorktop {
                resource_address,
                into_bucket,
            } => ast::Instruction::TakeFromWorktop {
                resource_address: resource_address.to_ast_value(bech32_coder)?,
                new_bucket: into_bucket.to_ast_value(bech32_coder)?,
            },
            Self::TakeFromWorktopByAmount {
                amount,
                resource_address,
                into_bucket,
            } => ast::Instruction::TakeFromWorktopByAmount {
                amount: amount.to_ast_value(bech32_coder)?,
                resource_address: resource_address.to_ast_value(bech32_coder)?,
                new_bucket: into_bucket.to_ast_value(bech32_coder)?,
            },
            Self::TakeFromWorktopByIds {
                ids,
                resource_address,
                into_bucket,
            } => ast::Instruction::TakeFromWorktopByIds {
                ids: Value::Array {
                    element_kind: crate::model::value::ValueKind::NonFungibleLocalId,
                    elements: ids.into_iter().collect::<Vec<_>>(),
                }
                .to_ast_value(bech32_coder)?,
                resource_address: resource_address.to_ast_value(bech32_coder)?,
                new_bucket: into_bucket.to_ast_value(bech32_coder)?,
            },
            Self::ReturnToWorktop { bucket } => ast::Instruction::ReturnToWorktop {
                bucket: bucket.to_ast_value(bech32_coder)?,
            },

            Self::AssertWorktopContains { resource_address } => {
                ast::Instruction::AssertWorktopContains {
                    resource_address: resource_address.to_ast_value(bech32_coder)?,
                }
            }
            Self::AssertWorktopContainsByAmount {
                amount,
                resource_address,
            } => ast::Instruction::AssertWorktopContainsByAmount {
                amount: amount.to_ast_value(bech32_coder)?,
                resource_address: resource_address.to_ast_value(bech32_coder)?,
            },
            Self::AssertWorktopContainsByIds {
                ids,
                resource_address,
            } => ast::Instruction::AssertWorktopContainsByIds {
                ids: Value::Array {
                    // TODO: This was `ValueKind::Bucket` by mistake. What kind of test can we
                    // introduce to catch this?
                    element_kind: crate::model::value::ValueKind::NonFungibleLocalId,
                    elements: ids.into_iter().collect::<Vec<_>>(),
                }
                .to_ast_value(bech32_coder)?,
                resource_address: resource_address.to_ast_value(bech32_coder)?,
            },

            Self::PopFromAuthZone { into_proof } => ast::Instruction::PopFromAuthZone {
                new_proof: into_proof.to_ast_value(bech32_coder)?,
            },
            Self::PushToAuthZone { proof } => ast::Instruction::PushToAuthZone {
                proof: proof.to_ast_value(bech32_coder)?,
            },
            Self::ClearAuthZone => ast::Instruction::ClearAuthZone,

            Self::CreateProofFromAuthZone {
                resource_address,
                into_proof,
            } => ast::Instruction::CreateProofFromAuthZone {
                resource_address: resource_address.to_ast_value(bech32_coder)?,
                new_proof: into_proof.to_ast_value(bech32_coder)?,
            },
            Self::CreateProofFromAuthZoneByAmount {
                amount,
                resource_address,
                into_proof,
            } => ast::Instruction::CreateProofFromAuthZoneByAmount {
                amount: amount.to_ast_value(bech32_coder)?,
                resource_address: resource_address.to_ast_value(bech32_coder)?,
                new_proof: into_proof.to_ast_value(bech32_coder)?,
            },
            Self::CreateProofFromAuthZoneByIds {
                ids,
                resource_address,
                into_proof,
            } => ast::Instruction::CreateProofFromAuthZoneByIds {
                ids: Value::Array {
                    element_kind: crate::model::value::ValueKind::NonFungibleLocalId,
                    elements: ids.into_iter().collect::<Vec<_>>(),
                }
                .to_ast_value(bech32_coder)?,
                resource_address: resource_address.to_ast_value(bech32_coder)?,
                new_proof: into_proof.to_ast_value(bech32_coder)?,
            },
            Self::CreateProofFromBucket { bucket, into_proof } => {
                ast::Instruction::CreateProofFromBucket {
                    bucket: bucket.to_ast_value(bech32_coder)?,
                    new_proof: into_proof.to_ast_value(bech32_coder)?,
                }
            }

            Self::CloneProof { proof, into_proof } => ast::Instruction::CloneProof {
                proof: proof.to_ast_value(bech32_coder)?,
                new_proof: into_proof.to_ast_value(bech32_coder)?,
            },

            Self::DropProof { proof } => ast::Instruction::DropProof {
                proof: proof.to_ast_value(bech32_coder)?,
            },
            Self::DropAllProofs => ast::Instruction::DropAllProofs,
            Self::PublishPackageWithOwner {
                code,
                abi,
                owner_badge,
            } => ast::Instruction::PublishPackageWithOwner {
                owner_badge: owner_badge.to_ast_value(bech32_coder)?,
                code: code.to_ast_value(bech32_coder)?,
                abi: abi.to_ast_value(bech32_coder)?,
            },
            Self::BurnResource { bucket } => ast::Instruction::BurnResource {
                bucket: bucket.to_ast_value(bech32_coder)?,
            },
            Self::PublishPackage {
                code,
                abi,
                royalty_config,
                metadata,
                access_rules,
            } => ast::Instruction::PublishPackage {
                code: code.to_ast_value(bech32_coder)?,
                abi: abi.to_ast_value(bech32_coder)?,
                royalty_config: royalty_config.to_ast_value(bech32_coder)?,
                metadata: metadata.to_ast_value(bech32_coder)?,
                access_rules: access_rules.to_ast_value(bech32_coder)?,
            },

            Self::RecallResource { vault_id, amount } => ast::Instruction::RecallResource {
                vault_id: vault_id.to_ast_value(bech32_coder)?,
                amount: amount.to_ast_value(bech32_coder)?,
            },

            Self::SetMetadata {
                entity_address,
                key,
                value,
            } => ast::Instruction::SetMetadata {
                entity_address: entity_address.to_ast_value(bech32_coder)?,
                key: key.to_ast_value(bech32_coder)?,
                value: value.to_ast_value(bech32_coder)?,
            },

            Self::SetPackageRoyaltyConfig {
                package_address,
                royalty_config,
            } => ast::Instruction::SetPackageRoyaltyConfig {
                package_address: package_address.to_ast_value(bech32_coder)?,
                royalty_config: royalty_config.to_ast_value(bech32_coder)?,
            },

            Self::SetComponentRoyaltyConfig {
                component_address,
                royalty_config,
            } => ast::Instruction::SetComponentRoyaltyConfig {
                component_address: component_address.to_ast_value(bech32_coder)?,
                royalty_config: royalty_config.to_ast_value(bech32_coder)?,
            },

            Self::ClaimPackageRoyalty { package_address } => {
                ast::Instruction::ClaimPackageRoyalty {
                    package_address: package_address.to_ast_value(bech32_coder)?,
                }
            }

            Self::ClaimComponentRoyalty { component_address } => {
                ast::Instruction::ClaimComponentRoyalty {
                    component_address: component_address.to_ast_value(bech32_coder)?,
                }
            }

            Self::SetMethodAccessRule {
                entity_address,
                index,
                key,
                rule,
            } => ast::Instruction::SetMethodAccessRule {
                entity_address: entity_address.to_ast_value(bech32_coder)?,
                index: index.to_ast_value(bech32_coder)?,
                key: key.to_ast_value(bech32_coder)?,
                rule: rule.to_ast_value(bech32_coder)?,
            },

            Self::CreateFungibleResource {
                divisibility,
                metadata,
                access_rules,
                initial_supply,
            } => ast::Instruction::CreateFungibleResource {
                divisibility: divisibility.to_ast_value(bech32_coder)?,
                metadata: metadata.to_ast_value(bech32_coder)?,
                access_rules: access_rules.to_ast_value(bech32_coder)?,
                initial_supply: initial_supply.to_ast_value(bech32_coder)?,
            },
            Self::CreateFungibleResourceWithOwner {
                divisibility,
                metadata,
                owner_badge,
                initial_supply,
            } => ast::Instruction::CreateFungibleResourceWithOwner {
                divisibility: divisibility.to_ast_value(bech32_coder)?,
                metadata: metadata.to_ast_value(bech32_coder)?,
                owner_badge: owner_badge.to_ast_value(bech32_coder)?,
                initial_supply: initial_supply.to_ast_value(bech32_coder)?,
            },
            Self::CreateNonFungibleResource {
                id_type,
                metadata,
                access_rules,
                initial_supply,
            } => ast::Instruction::CreateNonFungibleResource {
                id_type: id_type.to_ast_value(bech32_coder)?,
                metadata: metadata.to_ast_value(bech32_coder)?,
                access_rules: access_rules.to_ast_value(bech32_coder)?,
                initial_supply: initial_supply.to_ast_value(bech32_coder)?,
            },
            Self::CreateNonFungibleResourceWithOwner {
                id_type,
                metadata,
                owner_badge,
                initial_supply,
            } => ast::Instruction::CreateNonFungibleResourceWithOwner {
                id_type: id_type.to_ast_value(bech32_coder)?,
                metadata: metadata.to_ast_value(bech32_coder)?,
                owner_badge: owner_badge.to_ast_value(bech32_coder)?,
                initial_supply: initial_supply.to_ast_value(bech32_coder)?,
            },
            Self::MintFungible {
                resource_address,
                amount,
            } => ast::Instruction::MintFungible {
                resource_address: resource_address.to_ast_value(bech32_coder)?,
                amount: amount.to_ast_value(bech32_coder)?,
            },
            Self::MintNonFungible {
                resource_address,
                entries,
            } => ast::Instruction::MintNonFungible {
                resource_address: resource_address.to_ast_value(bech32_coder)?,
                entries: entries.to_ast_value(bech32_coder)?,
            },
            Self::MintUuidNonFungible {
                resource_address,
                entries,
            } => ast::Instruction::MintUuidNonFungible {
                resource_address: resource_address.to_ast_value(bech32_coder)?,
                entries: entries.to_ast_value(bech32_coder)?,
            },
            Self::AssertAccessRule { access_rule } => ast::Instruction::AssertAccessRule {
                access_rule: access_rule.to_ast_value(bech32_coder)?,
            },
            Self::CreateAccessController {
                controlled_asset,
                primary_role,
                recovery_role,
                confirmation_role,
                timed_recovery_delay_in_minutes,
            } => ast::Instruction::CreateAccessController {
                controlled_asset: controlled_asset.to_ast_value(bech32_coder)?,
                primary_role: primary_role.to_ast_value(bech32_coder)?,
                recovery_role: recovery_role.to_ast_value(bech32_coder)?,
                confirmation_role: confirmation_role.to_ast_value(bech32_coder)?,
                timed_recovery_delay_in_minutes: timed_recovery_delay_in_minutes
                    .to_ast_value(bech32_coder)?,
            },
            Self::CreateIdentity { access_rule } => ast::Instruction::CreateIdentity {
                access_rule: access_rule.to_ast_value(bech32_coder)?,
            },
            Self::CreateValidator {
                key,
                owner_access_rule,
            } => ast::Instruction::CreateValidator {
                key: key.to_ast_value(bech32_coder)?,
                owner_access_rule: owner_access_rule.to_ast_value(bech32_coder)?,
            },
        };
        Ok(ast_instruction)
    }

    pub fn from_ast_instruction(
        ast_instruction: &ast::Instruction,
        bech32_coder: &Bech32Coder,
    ) -> Result<Self> {
        let instruction = match ast_instruction {
            ast::Instruction::CallFunction {
                package_address,
                blueprint_name,
                function_name,
                args,
            } => Self::CallFunction {
                package_address: Value::from_ast_value(package_address, bech32_coder)?,
                blueprint_name: Value::from_ast_value(blueprint_name, bech32_coder)?,
                function_name: Value::from_ast_value(function_name, bech32_coder)?,
                arguments: {
                    let arguments = args
                        .iter()
                        .map(|v| Value::from_ast_value(v, bech32_coder))
                        .collect::<Result<Vec<Value>>>()?;
                    match arguments.len() {
                        0 => None,
                        _ => Some(arguments),
                    }
                },
            },
            ast::Instruction::CallMethod {
                component_address,
                method_name,
                args,
            } => Self::CallMethod {
                component_address: Value::from_ast_value(component_address, bech32_coder)?,
                method_name: Value::from_ast_value(method_name, bech32_coder)?,
                arguments: {
                    let arguments = args
                        .iter()
                        .map(|v| Value::from_ast_value(v, bech32_coder))
                        .collect::<Result<Vec<Value>>>()?;
                    match arguments.len() {
                        0 => None,
                        _ => Some(arguments),
                    }
                },
            },

            ast::Instruction::TakeFromWorktop {
                resource_address,
                new_bucket,
            } => Self::TakeFromWorktop {
                resource_address: Value::from_ast_value(resource_address, bech32_coder)?,
                into_bucket: Value::from_ast_value(new_bucket, bech32_coder)?,
            },
            ast::Instruction::TakeFromWorktopByAmount {
                amount,
                resource_address,
                new_bucket,
            } => Self::TakeFromWorktopByAmount {
                amount: Value::from_ast_value(amount, bech32_coder)?,
                resource_address: Value::from_ast_value(resource_address, bech32_coder)?,
                into_bucket: Value::from_ast_value(new_bucket, bech32_coder)?,
            },
            ast::Instruction::TakeFromWorktopByIds {
                ids,
                resource_address,
                new_bucket,
            } => Self::TakeFromWorktopByIds {
                ids: if let Value::Array {
                    element_kind: _,
                    elements,
                } = Value::from_ast_value(ids, bech32_coder)?
                {
                    elements.into_iter().collect::<Vec<Value>>()
                } else {
                    panic!("Expected type Array!")
                },
                resource_address: Value::from_ast_value(resource_address, bech32_coder)?,
                into_bucket: Value::from_ast_value(new_bucket, bech32_coder)?,
            },
            ast::Instruction::ReturnToWorktop { bucket } => Self::ReturnToWorktop {
                bucket: Value::from_ast_value(bucket, bech32_coder)?,
            },

            ast::Instruction::AssertWorktopContains { resource_address } => {
                Self::AssertWorktopContains {
                    resource_address: Value::from_ast_value(resource_address, bech32_coder)?,
                }
            }
            ast::Instruction::AssertWorktopContainsByAmount {
                amount,
                resource_address,
            } => Self::AssertWorktopContainsByAmount {
                amount: Value::from_ast_value(amount, bech32_coder)?,
                resource_address: Value::from_ast_value(resource_address, bech32_coder)?,
            },
            ast::Instruction::AssertWorktopContainsByIds {
                ids,
                resource_address,
            } => Self::AssertWorktopContainsByIds {
                ids: if let Value::Array {
                    element_kind: _,
                    elements,
                } = Value::from_ast_value(ids, bech32_coder)?
                {
                    elements.into_iter().collect::<Vec<Value>>()
                } else {
                    panic!("Expected type Array!")
                },
                resource_address: Value::from_ast_value(resource_address, bech32_coder)?,
            },

            ast::Instruction::PopFromAuthZone { new_proof } => Self::PopFromAuthZone {
                into_proof: Value::from_ast_value(new_proof, bech32_coder)?,
            },
            ast::Instruction::PushToAuthZone { proof } => Self::PushToAuthZone {
                proof: Value::from_ast_value(proof, bech32_coder)?,
            },
            ast::Instruction::ClearAuthZone => Self::ClearAuthZone,

            ast::Instruction::CreateProofFromAuthZone {
                resource_address,
                new_proof,
            } => Self::CreateProofFromAuthZone {
                resource_address: Value::from_ast_value(resource_address, bech32_coder)?,
                into_proof: Value::from_ast_value(new_proof, bech32_coder)?,
            },
            ast::Instruction::CreateProofFromAuthZoneByAmount {
                amount,
                resource_address,
                new_proof,
            } => Self::CreateProofFromAuthZoneByAmount {
                amount: Value::from_ast_value(amount, bech32_coder)?,
                resource_address: Value::from_ast_value(resource_address, bech32_coder)?,
                into_proof: Value::from_ast_value(new_proof, bech32_coder)?,
            },
            ast::Instruction::CreateProofFromAuthZoneByIds {
                ids,
                resource_address,
                new_proof,
            } => Self::CreateProofFromAuthZoneByIds {
                ids: if let Value::Array {
                    element_kind: _,
                    elements,
                } = Value::from_ast_value(ids, bech32_coder)?
                {
                    elements.into_iter().collect::<Vec<Value>>()
                } else {
                    panic!("Expected type Array!")
                },
                resource_address: Value::from_ast_value(resource_address, bech32_coder)?,
                into_proof: Value::from_ast_value(new_proof, bech32_coder)?,
            },
            ast::Instruction::CreateProofFromBucket { bucket, new_proof } => {
                Self::CreateProofFromBucket {
                    bucket: Value::from_ast_value(bucket, bech32_coder)?,
                    into_proof: Value::from_ast_value(new_proof, bech32_coder)?,
                }
            }

            ast::Instruction::CloneProof { proof, new_proof } => Self::CloneProof {
                proof: Value::from_ast_value(proof, bech32_coder)?,
                into_proof: Value::from_ast_value(new_proof, bech32_coder)?,
            },
            ast::Instruction::DropProof { proof } => Self::DropProof {
                proof: Value::from_ast_value(proof, bech32_coder)?,
            },
            ast::Instruction::DropAllProofs => Self::DropAllProofs,
            ast::Instruction::PublishPackageWithOwner {
                code,
                abi,
                owner_badge,
            } => Self::PublishPackageWithOwner {
                owner_badge: Value::from_ast_value(owner_badge, bech32_coder)?,
                code: Value::from_ast_value(code, bech32_coder)?,
                abi: Value::from_ast_value(abi, bech32_coder)?,
            },
            ast::Instruction::BurnResource { bucket } => Self::BurnResource {
                bucket: Value::from_ast_value(bucket, bech32_coder)?,
            },
            ast::Instruction::PublishPackage {
                code,
                abi,
                royalty_config,
                metadata,
                access_rules,
            } => Self::PublishPackage {
                code: Value::from_ast_value(code, bech32_coder)?,
                abi: Value::from_ast_value(abi, bech32_coder)?,
                royalty_config: Value::from_ast_value(royalty_config, bech32_coder)?,
                metadata: Value::from_ast_value(metadata, bech32_coder)?,
                access_rules: Value::from_ast_value(access_rules, bech32_coder)?,
            },
            ast::Instruction::RecallResource { vault_id, amount } => Self::RecallResource {
                vault_id: Value::from_ast_value(vault_id, bech32_coder)?,
                amount: Value::from_ast_value(amount, bech32_coder)?,
            },
            ast::Instruction::SetMetadata {
                entity_address,
                key,
                value,
            } => Self::SetMetadata {
                entity_address: Value::from_ast_value(entity_address, bech32_coder)?,
                key: Value::from_ast_value(key, bech32_coder)?,
                value: Value::from_ast_value(value, bech32_coder)?,
            },

            ast::Instruction::SetPackageRoyaltyConfig {
                package_address,
                royalty_config,
            } => Self::SetPackageRoyaltyConfig {
                package_address: Value::from_ast_value(package_address, bech32_coder)?,
                royalty_config: Value::from_ast_value(royalty_config, bech32_coder)?,
            },

            ast::Instruction::SetComponentRoyaltyConfig {
                component_address,
                royalty_config,
            } => Self::SetComponentRoyaltyConfig {
                component_address: Value::from_ast_value(component_address, bech32_coder)?,
                royalty_config: Value::from_ast_value(royalty_config, bech32_coder)?,
            },

            ast::Instruction::ClaimPackageRoyalty { package_address } => {
                Self::ClaimPackageRoyalty {
                    package_address: Value::from_ast_value(package_address, bech32_coder)?,
                }
            }

            ast::Instruction::ClaimComponentRoyalty { component_address } => {
                Self::ClaimComponentRoyalty {
                    component_address: Value::from_ast_value(component_address, bech32_coder)?,
                }
            }

            ast::Instruction::SetMethodAccessRule {
                entity_address,
                index,
                key,
                rule,
            } => Self::SetMethodAccessRule {
                entity_address: Value::from_ast_value(entity_address, bech32_coder)?,
                index: Value::from_ast_value(index, bech32_coder)?,
                key: Value::from_ast_value(key, bech32_coder)?,
                rule: Value::from_ast_value(rule, bech32_coder)?,
            },

            ast::Instruction::CreateFungibleResource {
                divisibility,
                metadata,
                access_rules,
                initial_supply,
            } => Self::CreateFungibleResource {
                divisibility: Value::from_ast_value(divisibility, bech32_coder)?,
                metadata: Value::from_ast_value(metadata, bech32_coder)?,
                access_rules: Value::from_ast_value(access_rules, bech32_coder)?,
                initial_supply: Value::from_ast_value(initial_supply, bech32_coder)?,
            },
            ast::Instruction::CreateFungibleResourceWithOwner {
                divisibility,
                metadata,
                owner_badge,
                initial_supply,
            } => Self::CreateFungibleResourceWithOwner {
                divisibility: Value::from_ast_value(divisibility, bech32_coder)?,
                metadata: Value::from_ast_value(metadata, bech32_coder)?,
                owner_badge: Value::from_ast_value(owner_badge, bech32_coder)?,
                initial_supply: Value::from_ast_value(initial_supply, bech32_coder)?,
            },
            ast::Instruction::CreateNonFungibleResource {
                id_type,
                metadata,
                access_rules,
                initial_supply,
            } => Self::CreateNonFungibleResource {
                id_type: Value::from_ast_value(id_type, bech32_coder)?,
                metadata: Value::from_ast_value(metadata, bech32_coder)?,
                access_rules: Value::from_ast_value(access_rules, bech32_coder)?,
                initial_supply: Value::from_ast_value(initial_supply, bech32_coder)?,
            },
            ast::Instruction::CreateNonFungibleResourceWithOwner {
                id_type,
                metadata,
                owner_badge,
                initial_supply,
            } => Self::CreateNonFungibleResourceWithOwner {
                id_type: Value::from_ast_value(id_type, bech32_coder)?,
                metadata: Value::from_ast_value(metadata, bech32_coder)?,
                owner_badge: Value::from_ast_value(owner_badge, bech32_coder)?,
                initial_supply: Value::from_ast_value(initial_supply, bech32_coder)?,
            },

            ast::Instruction::MintFungible {
                resource_address,
                amount,
            } => Self::MintFungible {
                resource_address: Value::from_ast_value(resource_address, bech32_coder)?,
                amount: Value::from_ast_value(amount, bech32_coder)?,
            },
            ast::Instruction::MintNonFungible {
                resource_address,
                entries,
            } => Self::MintNonFungible {
                resource_address: Value::from_ast_value(resource_address, bech32_coder)?,
                entries: Value::from_ast_value(entries, bech32_coder)?,
            },
            ast::Instruction::MintUuidNonFungible {
                resource_address,
                entries,
            } => Self::MintUuidNonFungible {
                resource_address: Value::from_ast_value(resource_address, bech32_coder)?,
                entries: Value::from_ast_value(entries, bech32_coder)?,
            },

            ast::Instruction::CreateIdentity { access_rule } => Self::CreateIdentity {
                access_rule: Value::from_ast_value(access_rule, bech32_coder)?,
            },
            ast::Instruction::AssertAccessRule { access_rule } => Self::AssertAccessRule {
                access_rule: Value::from_ast_value(access_rule, bech32_coder)?,
            },
            ast::Instruction::CreateAccessController {
                controlled_asset,
                primary_role,
                recovery_role,
                confirmation_role,
                timed_recovery_delay_in_minutes,
            } => Self::CreateAccessController {
                controlled_asset: Value::from_ast_value(controlled_asset, bech32_coder)?,
                primary_role: Value::from_ast_value(primary_role, bech32_coder)?,
                recovery_role: Value::from_ast_value(recovery_role, bech32_coder)?,
                confirmation_role: Value::from_ast_value(confirmation_role, bech32_coder)?,
                timed_recovery_delay_in_minutes: Value::from_ast_value(
                    timed_recovery_delay_in_minutes,
                    bech32_coder,
                )?,
            },
            ast::Instruction::CreateValidator {
                key,
                owner_access_rule,
            } => Self::CreateValidator {
                key: Value::from_ast_value(key, bech32_coder)?,
                owner_access_rule: Value::from_ast_value(owner_access_rule, bech32_coder)?,
            },
        };
        Ok(instruction)
    }
}
