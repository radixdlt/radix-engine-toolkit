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

use radix_engine_interface::blueprints::access_controller::*;
use radix_engine_interface::blueprints::account::*;
use radix_engine_interface::blueprints::consensus_manager::*;
use radix_engine_interface::blueprints::identity::*;
use radix_engine_interface::blueprints::locker::*;
use radix_engine_interface::blueprints::pool::*;
use radix_engine_interface::blueprints::resource::*;
use radix_engine_interface::blueprints::transaction_processor::*;

use radix_engine::blueprints::package::*;
use radix_engine::blueprints::transaction_tracker::*;

use crate::internal_prelude::*;

#[ext_sized]
pub impl BlueprintId {
    fn entity_type(&self) -> Option<EntityType> {
        let Self {
            package_address,
            blueprint_name,
        } = self;
        match (*package_address, blueprint_name.as_str()) {
            // Access Controller Package
            (ACCESS_CONTROLLER_PACKAGE, ACCESS_CONTROLLER_BLUEPRINT) => {
                Some(EntityType::GlobalAccessController)
            }
            // Account Package
            (ACCOUNT_PACKAGE, ACCOUNT_BLUEPRINT) => {
                Some(EntityType::GlobalAccount)
            }
            // Consensus Manager Package
            (CONSENSUS_MANAGER_PACKAGE, CONSENSUS_MANAGER_BLUEPRINT) => {
                Some(EntityType::GlobalConsensusManager)
            }
            (CONSENSUS_MANAGER_PACKAGE, VALIDATOR_BLUEPRINT) => {
                Some(EntityType::GlobalValidator)
            }
            // Identity Package
            (IDENTITY_PACKAGE, IDENTITY_BLUEPRINT) => {
                Some(EntityType::GlobalIdentity)
            }
            // Account Locker Package
            (LOCKER_PACKAGE, ACCOUNT_LOCKER_BLUEPRINT) => {
                Some(EntityType::GlobalAccountLocker)
            }
            // Package Package
            (PACKAGE_PACKAGE, PACKAGE_BLUEPRINT) => {
                Some(EntityType::GlobalPackage)
            }
            // Pool Package
            (PACKAGE_PACKAGE, ONE_RESOURCE_POOL_BLUEPRINT) => {
                Some(EntityType::GlobalOneResourcePool)
            }
            (PACKAGE_PACKAGE, TWO_RESOURCE_POOL_BLUEPRINT) => {
                Some(EntityType::GlobalTwoResourcePool)
            }
            (PACKAGE_PACKAGE, MULTI_RESOURCE_POOL_BLUEPRINT) => {
                Some(EntityType::GlobalMultiResourcePool)
            }
            // Resource Package
            (RESOURCE_PACKAGE, FUNGIBLE_RESOURCE_MANAGER_BLUEPRINT) => {
                Some(EntityType::GlobalFungibleResourceManager)
            }
            (RESOURCE_PACKAGE, NON_FUNGIBLE_RESOURCE_MANAGER_BLUEPRINT) => {
                Some(EntityType::GlobalNonFungibleResourceManager)
            }
            (RESOURCE_PACKAGE, FUNGIBLE_VAULT_BLUEPRINT) => {
                Some(EntityType::InternalFungibleVault)
            }
            (RESOURCE_PACKAGE, NON_FUNGIBLE_VAULT_BLUEPRINT) => {
                Some(EntityType::InternalNonFungibleVault)
            }
            (RESOURCE_PACKAGE, FUNGIBLE_BUCKET_BLUEPRINT)
            | (RESOURCE_PACKAGE, NON_FUNGIBLE_BUCKET_BLUEPRINT)
            | (RESOURCE_PACKAGE, FUNGIBLE_PROOF_BLUEPRINT)
            | (RESOURCE_PACKAGE, NON_FUNGIBLE_PROOF_BLUEPRINT)
            | (RESOURCE_PACKAGE, AUTH_ZONE_BLUEPRINT)
            | (RESOURCE_PACKAGE, WORKTOP_BLUEPRINT) => {
                Some(EntityType::InternalGenericComponent)
            }
            // Transaction Processor
            (
                TRANSACTION_PROCESSOR_PACKAGE,
                TRANSACTION_PROCESSOR_BLUEPRINT,
            ) => Some(EntityType::GlobalGenericComponent),
            // Transaction Tracker
            (TRANSACTION_TRACKER_PACKAGE, TRANSACTION_TRACKER_BLUEPRINT) => {
                Some(EntityType::GlobalTransactionTracker)
            }
            // Else
            _ => None,
        }
    }
}
