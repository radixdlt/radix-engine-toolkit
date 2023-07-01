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

use radix_engine_common::prelude::*;
use scrypto::api::node_modules::metadata::*;
use scrypto::prelude::*;

#[derive(ScryptoSbor, Clone, Debug)]
pub struct NonFungibleResourceManagerCreateIndexMapInput {
    pub owner_role: OwnerRole,
    pub id_type: NonFungibleIdType,
    pub track_total_supply: bool,
    pub non_fungible_schema: NonFungibleDataSchema,
    pub access_rules: IndexMap<ResourceAction, (AccessRule, AccessRule)>,
    pub metadata: ModuleConfig<MetadataInit>,
    pub address_reservation: Option<GlobalAddressReservation>,
}

#[derive(ManifestSbor, Clone, Debug)]
pub struct NonFungibleResourceManagerCreateManifestIndexMapInput {
    pub owner_role: OwnerRole,
    pub id_type: NonFungibleIdType,
    pub track_total_supply: bool,
    pub non_fungible_schema: NonFungibleDataSchema,
    pub access_rules: IndexMap<ResourceAction, (AccessRule, AccessRule)>,
    pub metadata: ModuleConfig<MetadataInit>,
    pub address_reservation: Option<ManifestAddressReservation>,
}

#[derive(ScryptoSbor, Clone, Debug)]
pub struct NonFungibleResourceManagerCreateWithInitialSupplyIndexMapInput {
    pub owner_role: OwnerRole,
    pub id_type: NonFungibleIdType,
    pub track_total_supply: bool,
    pub non_fungible_schema: NonFungibleDataSchema,
    pub entries: IndexMap<NonFungibleLocalId, (ScryptoValue,)>,
    pub access_rules: IndexMap<ResourceAction, (AccessRule, AccessRule)>,
    pub metadata: ModuleConfig<MetadataInit>,
    pub address_reservation: Option<GlobalAddressReservation>,
}

#[derive(ManifestSbor, Clone, Debug)]
pub struct NonFungibleResourceManagerCreateWithInitialSupplyManifestIndexMapInput {
    pub owner_role: OwnerRole,
    pub id_type: NonFungibleIdType,
    pub track_total_supply: bool,
    pub non_fungible_schema: NonFungibleDataSchema,
    pub entries: IndexMap<NonFungibleLocalId, (ManifestValue,)>,
    pub access_rules: IndexMap<ResourceAction, (AccessRule, AccessRule)>,
    pub metadata: ModuleConfig<MetadataInit>,
    pub address_reservation: Option<ManifestAddressReservation>,
}

#[derive(ScryptoSbor, Clone, Debug)]
pub struct NonFungibleResourceManagerCreateRuidWithInitialSupplyIndexMapInput {
    pub owner_role: OwnerRole,
    pub track_total_supply: bool,
    pub non_fungible_schema: NonFungibleDataSchema,
    pub entries: Vec<(ScryptoValue,)>,
    pub access_rules: IndexMap<ResourceAction, (AccessRule, AccessRule)>,
    pub metadata: ModuleConfig<MetadataInit>,
    pub address_reservation: Option<GlobalAddressReservation>,
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
    pub owner_role: OwnerRole,
    pub track_total_supply: bool,
    pub divisibility: u8,
    pub access_rules: IndexMap<ResourceAction, (AccessRule, AccessRule)>,
    pub metadata: ModuleConfig<MetadataInit>,
    pub address_reservation: Option<GlobalAddressReservation>,
}

#[derive(ManifestSbor, Clone, Debug)]
pub struct FungibleResourceManagerCreateManifestIndexMapInput {
    pub owner_role: OwnerRole,
    pub track_total_supply: bool,
    pub divisibility: u8,
    pub access_rules: IndexMap<ResourceAction, (AccessRule, AccessRule)>,
    pub metadata: ModuleConfig<MetadataInit>,
    pub address_reservation: Option<ManifestAddressReservation>,
}

#[derive(ScryptoSbor, Clone, Debug)]
pub struct FungibleResourceManagerCreateWithInitialSupplyIndexMapInput {
    pub owner_role: OwnerRole,
    pub track_total_supply: bool,
    pub divisibility: u8,
    pub initial_supply: Decimal,
    pub access_rules: IndexMap<ResourceAction, (AccessRule, AccessRule)>,
    pub metadata: ModuleConfig<MetadataInit>,
    pub address_reservation: Option<GlobalAddressReservation>,
}

#[derive(ManifestSbor, Clone, Debug)]
pub struct FungibleResourceManagerCreateWithInitialSupplyManifestIndexMapInput {
    pub owner_role: OwnerRole,
    pub track_total_supply: bool,
    pub divisibility: u8,
    pub initial_supply: Decimal,
    pub access_rules: IndexMap<ResourceAction, (AccessRule, AccessRule)>,
    pub metadata: ModuleConfig<MetadataInit>,
    pub address_reservation: Option<ManifestAddressReservation>,
}

#[derive(ScryptoSbor, Clone, Debug)]
pub struct AccessRulesCreateIndexMapInput {
    pub owner_role: OwnerRole,
    pub roles: IndexMap<scrypto::api::ObjectModuleId, RolesInit>,
}
