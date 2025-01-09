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

use crate::internal_prelude::*;

pub fn to_manifest_type<D: ManifestDecode>(value: &ManifestValue) -> Option<D> {
    manifest_encode(value)
        .ok()
        .and_then(|encoded| manifest_decode(&encoded).ok())
}

pub fn is_account<A: Into<DynamicGlobalAddress> + Clone>(node_id: &A) -> bool {
    match node_id.clone().into() {
        DynamicGlobalAddress::Named(_) => false,
        DynamicGlobalAddress::Static(address) => {
            matches!(
                address.as_node_id().entity_type(),
                Some(
                    EntityType::GlobalAccount
                        | EntityType::GlobalPreallocatedSecp256k1Account
                        | EntityType::GlobalPreallocatedEd25519Account
                )
            )
        }
    }
}

pub fn is_validator<A: Into<DynamicGlobalAddress> + Clone>(
    node_id: &A,
) -> bool {
    match node_id.clone().into() {
        DynamicGlobalAddress::Named(_) => false,
        DynamicGlobalAddress::Static(address) => {
            matches!(
                address.as_node_id().entity_type(),
                Some(EntityType::GlobalValidator)
            )
        }
    }
}

pub fn is_access_controller<A: Into<DynamicGlobalAddress> + Clone>(
    node_id: &A,
) -> bool {
    match node_id.clone().into() {
        DynamicGlobalAddress::Named(_) => false,
        DynamicGlobalAddress::Static(address) => {
            matches!(
                address.as_node_id().entity_type(),
                Some(EntityType::GlobalAccessController)
            )
        }
    }
}

pub fn is_identity<A: Into<DynamicGlobalAddress> + Clone>(node_id: &A) -> bool {
    match node_id.clone().into() {
        DynamicGlobalAddress::Named(_) => false,
        DynamicGlobalAddress::Static(address) => {
            matches!(
                address.as_node_id().entity_type(),
                Some(
                    EntityType::GlobalIdentity
                        | EntityType::GlobalPreallocatedSecp256k1Identity
                        | EntityType::GlobalPreallocatedEd25519Identity
                )
            )
        }
    }
}

#[macro_export]
macro_rules! contains {
    (
        $item: expr =>
        [
            $($other: expr),* $(,)?
        ] $(,)?
    ) => {
        $(
            $item == $other
        )||*
    };
}
