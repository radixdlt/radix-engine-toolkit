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

use radix_engine::blueprints::package::*;
use radix_engine_common::prelude::*;
use scrypto::api::node_modules::metadata::*;
use scrypto::prelude::*;

#[derive(ScryptoSbor, Clone, Debug)]
pub struct NonFungibleResourceManagerCreateIndexMapInput {
    pub id_type: NonFungibleIdType,
    pub track_total_supply: bool,
    pub non_fungible_schema: NonFungibleDataSchema,
    pub metadata: IndexMap<String, MetadataValue>,
    pub access_rules: IndexMap<ResourceMethodAuthKey, (AccessRule, AccessRule)>,
}

#[derive(ManifestSbor, Clone, Debug)]
pub struct NonFungibleResourceManagerCreateWithInitialSupplyManifestIndexMapInput {
    pub id_type: NonFungibleIdType,
    pub track_total_supply: bool,
    pub non_fungible_schema: NonFungibleDataSchema,
    pub metadata: IndexMap<String, MetadataValue>,
    pub access_rules: IndexMap<ResourceMethodAuthKey, (AccessRule, AccessRule)>,
    pub entries: IndexMap<NonFungibleLocalId, (ManifestValue,)>,
}

#[derive(ScryptoSbor, Clone, Debug)]
pub struct NonFungibleResourceManagerCreateWithInitialSupplyIndexMapInput {
    pub id_type: NonFungibleIdType,
    pub track_total_supply: bool,
    pub non_fungible_schema: NonFungibleDataSchema,
    pub metadata: IndexMap<String, MetadataValue>,
    pub access_rules: IndexMap<ResourceMethodAuthKey, (AccessRule, AccessRule)>,
    pub entries: IndexMap<NonFungibleLocalId, (ScryptoValue,)>,
}

#[derive(ScryptoSbor, Clone, Debug)]
pub struct NonFungibleResourceManagerCreateWithAddressIndexMapInput {
    pub id_type: NonFungibleIdType,
    pub track_total_supply: bool,
    pub non_fungible_schema: NonFungibleDataSchema,
    pub metadata: IndexMap<String, MetadataValue>,
    pub access_rules: IndexMap<ResourceMethodAuthKey, (AccessRule, AccessRule)>,
    pub resource_address: GlobalAddressReservation,
}

#[derive(ManifestSbor, Clone, Debug)]
pub struct NonFungibleResourceManagerCreateWithAddressManifestIndexMapInput {
    pub id_type: NonFungibleIdType,
    pub track_total_supply: bool,
    pub non_fungible_schema: NonFungibleDataSchema,
    pub metadata: IndexMap<String, MetadataValue>,
    pub access_rules: IndexMap<ResourceMethodAuthKey, (AccessRule, AccessRule)>,
    pub resource_address: ManifestAddressReservation,
}

#[derive(ScryptoSbor, Clone, Debug)]
pub struct NonFungibleResourceManagerCreateRuidWithInitialSupplyIndexMapInput {
    pub track_total_supply: bool,
    pub non_fungible_schema: NonFungibleDataSchema,
    pub metadata: IndexMap<String, MetadataValue>,
    pub access_rules: IndexMap<ResourceMethodAuthKey, (AccessRule, AccessRule)>,
    pub entries: Vec<(ScryptoValue,)>,
}

#[derive(ManifestSbor, Clone, Debug)]
pub struct NonFungibleResourceManagerMintManifestIndexMapInput {
    pub entries: IndexMap<NonFungibleLocalId, (ManifestValue,)>,
}

#[derive(ScryptoSbor, Clone, Debug)]
pub struct NonFungibleResourceManagerMintIndexMapInput {
    pub entries: IndexMap<NonFungibleLocalId, (ScryptoValue,)>,
}

#[derive(ScryptoSbor, Clone, Debug)]
pub struct FungibleResourceManagerCreateIndexMapInput {
    pub track_total_supply: bool,
    pub divisibility: u8,
    pub metadata: IndexMap<String, MetadataValue>,
    pub access_rules: IndexMap<ResourceMethodAuthKey, (AccessRule, AccessRule)>,
}

#[derive(ScryptoSbor, Clone, Debug)]
pub struct FungibleResourceManagerCreateWithInitialSupplyIndexMapInput {
    pub track_total_supply: bool,
    pub divisibility: u8,
    pub metadata: IndexMap<String, MetadataValue>,
    pub access_rules: IndexMap<ResourceMethodAuthKey, (AccessRule, AccessRule)>,
    pub initial_supply: Decimal,
}

#[derive(ScryptoSbor, Clone, Debug)]
pub struct FungibleResourceManagerCreateWithInitialSupplyAndAddressIndexMapInput {
    pub track_total_supply: bool,
    pub divisibility: u8,
    pub metadata: IndexMap<String, MetadataValue>,
    pub access_rules: IndexMap<ResourceMethodAuthKey, (AccessRule, AccessRule)>,
    pub initial_supply: Decimal,
    pub resource_address: GlobalAddressReservation,
}

#[derive(ManifestSbor, Clone, Debug)]
pub struct FungibleResourceManagerCreateWithInitialSupplyAndAddressManifestIndexMapInput {
    pub track_total_supply: bool,
    pub divisibility: u8,
    pub metadata: IndexMap<String, MetadataValue>,
    pub access_rules: IndexMap<ResourceMethodAuthKey, (AccessRule, AccessRule)>,
    pub initial_supply: Decimal,
    pub resource_address: ManifestAddressReservation,
}

#[derive(ScryptoSbor, Clone, Debug)]
pub struct PackagePublishWasmIndexMapInput {
    pub code: Vec<u8>,
    pub setup: PackageDefinition,
    pub metadata: IndexMap<String, MetadataValue>,
}

#[derive(ManifestSbor, Clone, Debug)]
pub struct PackagePublishWasmManifestIndexMapInput {
    pub code: ManifestBlobRef,
    pub setup: PackageDefinition,
    pub metadata: IndexMap<String, MetadataValue>,
}

#[derive(ScryptoSbor, Clone, Debug)]
pub struct PackagePublishWasmAdvancedIndexMapInput {
    pub package_address: Option<GlobalAddressReservation>,
    pub code: Vec<u8>,
    pub setup: PackageDefinition,
    pub metadata: IndexMap<String, MetadataValue>,
    pub owner_rule: OwnerRole,
}

#[derive(ManifestSbor, Clone, Debug)]
pub struct PackagePublishWasmAdvancedManifestIndexMapInput {
    pub package_address: Option<ManifestAddressReservation>,
    pub code: ManifestBlobRef,
    pub setup: PackageDefinition,
    pub metadata: IndexMap<String, MetadataValue>,
    pub owner_rule: OwnerRole,
}

#[derive(ScryptoSbor, Clone, Debug)]
pub struct PackagePublishNativeIndexMapInput {
    pub package_address: Option<GlobalAddressReservation>,
    pub native_package_code_id: u8,
    pub setup: PackageDefinition,
    pub metadata: IndexMap<String, MetadataValue>,
}

#[derive(ManifestSbor, Clone, Debug)]
pub struct PackagePublishNativeManifestIndexMapInput {
    pub package_address: Option<ManifestAddressReservation>,
    pub native_package_code_id: u8,
    pub setup: PackageDefinition,
    pub metadata: IndexMap<String, MetadataValue>,
}

#[derive(ScryptoSbor, Clone, Debug)]
pub struct AccessRulesCreateIndexMapInput {
    pub owner_role: OwnerRole,
    pub roles: IndexMap<ObjectModuleId, Roles>,
}

#[derive(ScryptoSbor, Clone, Debug)]
pub struct MetadataCreateWithDataIndexMapInput {
    pub data: IndexMap<String, MetadataValue>,
}
