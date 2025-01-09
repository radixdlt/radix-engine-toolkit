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

use radix_common::types::*;

/// Defines a typed NodeId type which is a NodeId guaranteed to have an
/// EntityType and guaranteed to be Bech32m encodable. The stored [`EntityType`]
/// always matches that of the stored [`NodeId`]. This type is to be used
/// everywhere in the core toolkit whether as arguments, returns, or part
/// of other structs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TypedNodeId(EntityType, NodeId);

impl TypedNodeId {
    pub fn new<T>(node_id: T) -> Result<Self, InvalidEntityTypeIdError>
    where
        T: Into<NodeId>,
    {
        let node_id = node_id.into();
        if let Some(entity_type) = node_id.entity_type() {
            Ok(Self(entity_type, node_id))
        } else {
            Err(InvalidEntityTypeIdError(node_id))
        }
    }
}

impl TypedNodeId {
    pub fn entity_type(&self) -> EntityType {
        self.0
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.1.to_vec()
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.1.as_bytes()
    }

    pub fn as_node_id(&self) -> &NodeId {
        &self.1
    }
}

typed_node_id_to_typed_address! {GlobalAddress, ParseGlobalAddressError}
typed_node_id_to_typed_address! {PackageAddress, ParsePackageAddressError}
typed_node_id_to_typed_address! {InternalAddress, ParseInternalAddressError}
typed_node_id_to_typed_address! {ResourceAddress, ParseResourceAddressError}
typed_node_id_to_typed_address! {ComponentAddress, ParseComponentAddressError}

typed_address_to_typed_node_id! {GlobalAddress}
typed_address_to_typed_node_id! {PackageAddress}
typed_address_to_typed_node_id! {InternalAddress}
typed_address_to_typed_node_id! {ResourceAddress}
typed_address_to_typed_node_id! {ComponentAddress}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct InvalidEntityTypeIdError(NodeId);

macro_rules! typed_node_id_to_typed_address {
    ($type: ty, $err: ty) => {
        impl TryFrom<TypedNodeId> for $type {
            type Error = $err;

            fn try_from(value: TypedNodeId) -> Result<Self, Self::Error> {
                value.1.try_into()
            }
        }
    };
}

macro_rules! typed_address_to_typed_node_id {
    ($type: ty) => {
        impl From<$type> for TypedNodeId {
            fn from(value: $type) -> Self {
                let node_id = value.into_node_id();
                Self(
                    node_id.entity_type().expect("Must be available!"),
                    node_id,
                )
            }
        }
    };
}

use {typed_address_to_typed_node_id, typed_node_id_to_typed_address};
