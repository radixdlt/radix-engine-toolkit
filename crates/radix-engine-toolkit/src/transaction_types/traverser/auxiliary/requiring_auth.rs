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

use radix_transactions::manifest::*;
use radix_transactions::prelude::*;
use scrypto::prelude::*;

use crate::statics::*;
use crate::transaction_types::*;
use crate::utils::to_manifest_type;

#[derive(Default)]
pub struct RequiringAuthDetector {
    accounts: IndexSet<ComponentAddress>,
    identities: IndexSet<ComponentAddress>,
}

impl RequiringAuthDetector {
    pub fn output(
        self,
    ) -> (IndexSet<ComponentAddress>, IndexSet<ComponentAddress>) {
        (self.accounts, self.identities)
    }
}

impl StaticAnalysisCallback for RequiringAuthDetector {
    fn on_instruction(&mut self, instruction: &InstructionV2, _: usize) {
        match instruction {
            InstructionV2::CallMethod(CallMethod {
                address: DynamicGlobalAddress::Static(address),
                method_name,
                args,
                ..
            }) => {
                if is_account(address)
                    && ACCOUNT_METHODS_THAT_REQUIRE_AUTH.contains(method_name)
                {
                    self.accounts.insert(component_address!(*address));
                } else if is_identity(address)
                    && IDENTITY_METHODS_THAT_REQUIRE_AUTH.contains(method_name)
                {
                    self.identities.insert(component_address!(*address));
                } else if is_account_locker(address) {
                    match method_name.as_str() {
                        ACCOUNT_LOCKER_CLAIM_IDENT => {
                            let Some(AccountLockerClaimManifestInput {
                                claimant:
                                    DynamicComponentAddress::Static(claimant),
                                ..
                            }) = to_manifest_type(args)
                            else {
                                return;
                            };
                            self.accounts.insert(claimant);
                        }
                        ACCOUNT_LOCKER_CLAIM_NON_FUNGIBLES_IDENT => {
                            let Some(
                                AccountLockerClaimNonFungiblesManifestInput {
                                    claimant:
                                        DynamicComponentAddress::Static(claimant),
                                    ..
                                },
                            ) = to_manifest_type(args)
                            else {
                                return;
                            };
                            self.accounts.insert(claimant);
                        }
                        _ => {}
                    }
                }
            }
            InstructionV2::CallRoyaltyMethod(CallRoyaltyMethod {
                address: DynamicGlobalAddress::Static(address),
                method_name,
                ..
            }) => {
                if ROYALTY_METHODS_THAT_REQUIRE_AUTH.contains(method_name) {
                    if is_account(address) {
                        self.accounts.insert(component_address!(*address));
                    } else if is_identity(address) {
                        self.identities.insert(component_address!(*address));
                    }
                }
            }
            InstructionV2::CallMetadataMethod(CallMetadataMethod {
                address: DynamicGlobalAddress::Static(address),
                method_name,
                ..
            }) => {
                if METADATA_METHODS_THAT_REQUIRE_AUTH.contains(method_name) {
                    if is_account(address) {
                        self.accounts.insert(component_address!(*address));
                    } else if is_identity(address) {
                        self.identities.insert(component_address!(*address));
                    }
                }
            }
            InstructionV2::CallRoleAssignmentMethod(
                CallRoleAssignmentMethod {
                    address: DynamicGlobalAddress::Static(address),
                    method_name,
                    ..
                },
            ) => {
                if ROLE_ASSIGNMENT_METHODS_THAT_REQUIRE_AUTH
                    .contains(method_name)
                {
                    if is_account(address) {
                        self.accounts.insert(component_address!(*address));
                    } else if is_identity(address) {
                        self.identities.insert(component_address!(*address));
                    }
                }
            }
            _ => { /* No-op */ }
        }
    }
}

impl DynamicAnalysisCallback for RequiringAuthDetector {}

fn is_account(address: &GlobalAddress) -> bool {
    address
        .as_node_id()
        .entity_type()
        .is_some_and(|entity_type| {
            matches!(
                entity_type,
                EntityType::GlobalAccount
                    | EntityType::GlobalPreallocatedSecp256k1Account
                    | EntityType::GlobalPreallocatedEd25519Account
            )
        })
}

fn is_account_locker(address: &GlobalAddress) -> bool {
    address
        .as_node_id()
        .entity_type()
        .is_some_and(|entity_type| {
            matches!(entity_type, EntityType::GlobalAccountLocker)
        })
}

fn is_identity(address: &GlobalAddress) -> bool {
    address
        .as_node_id()
        .entity_type()
        .is_some_and(|entity_type| {
            matches!(
                entity_type,
                EntityType::GlobalIdentity
                    | EntityType::GlobalPreallocatedSecp256k1Identity
                    | EntityType::GlobalPreallocatedEd25519Identity
            )
        })
}

macro_rules! component_address {
    ($expr: expr) => {
        ::scrypto::prelude::ComponentAddress::try_from($expr)
            .expect("Must be a valid component address")
    };
}
use component_address;
