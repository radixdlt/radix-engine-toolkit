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

use crate::prelude::*;

#[derive(Debug, Clone, Record)]
pub struct FungibleResourceRoles {
    pub mint_roles: Option<ResourceManagerRole>,
    pub burn_roles: Option<ResourceManagerRole>,
    pub freeze_roles: Option<ResourceManagerRole>,
    pub recall_roles: Option<ResourceManagerRole>,
    pub withdraw_roles: Option<ResourceManagerRole>,
    pub deposit_roles: Option<ResourceManagerRole>,
}

#[derive(Debug, Clone, Record)]
pub struct ResourceManagerRole {
    pub role: Option<Arc<AccessRule>>,
    pub role_updater: Option<Arc<AccessRule>>,
}

impl ResourceManagerRole {
    resource_manager_role_conversion! {MintRoles, minter}
    resource_manager_role_conversion! {BurnRoles, burner}
    resource_manager_role_conversion! {FreezeRoles, freezer}
    resource_manager_role_conversion! {RecallRoles, recaller}
    resource_manager_role_conversion! {WithdrawRoles, withdrawer}
    resource_manager_role_conversion! {DepositRoles, depositor}
}

impl ToNative for FungibleResourceRoles {
    type Native = engine::FungibleResourceRoles;

    fn to_native(self) -> Result<Self::Native> {
        Ok(engine::FungibleResourceRoles {
            mint_roles: self
                .mint_roles
                .map(|value| value.to_native_mint_roles()),
            burn_roles: self
                .burn_roles
                .map(|value| value.to_native_burn_roles()),
            freeze_roles: self
                .freeze_roles
                .map(|value| value.to_native_freeze_roles()),
            recall_roles: self
                .recall_roles
                .map(|value| value.to_native_recall_roles()),
            withdraw_roles: self
                .withdraw_roles
                .map(|value| value.to_native_withdraw_roles()),
            deposit_roles: self
                .deposit_roles
                .map(|value| value.to_native_deposit_roles()),
        })
    }
}

macro_rules! resource_manager_role_conversion {
    ($ty: ident, $name: ident) => {
        paste::paste! {
            pub fn [< to_native_ $ty: snake >](&self) -> engine::$ty<engine::RoleDefinition> {
                engine::$ty {
                    $name: self.role.as_ref().map(|value| value.0.clone()),
                    [< $name _updater >]: self.role_updater.as_ref().map(|value| value.0.clone()),
                }
            }
        }
    };
}
use resource_manager_role_conversion;
