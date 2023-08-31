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

#[derive(Clone, Debug, Enum)]
pub enum ManifestAddress {
    Named { value: u32 },
    Static { value: Arc<Address> },
}

impl From<ManifestAddress> for NativeManifestAddress {
    fn from(value: ManifestAddress) -> Self {
        match value {
            ManifestAddress::Named { value } => Self::Named(value),
            ManifestAddress::Static { value } => Self::Static((*value).into()),
        }
    }
}

impl ManifestAddress {
    pub fn new(native: &NativeManifestAddress, network_id: u8) -> Self {
        match native {
            NativeManifestAddress::Named(value) => Self::Named { value: *value },
            NativeManifestAddress::Static(value) => Self::Static {
                // The NativeManifestAddress has a custom implementation of decoding that ensures
                // that the address indeed has an entity type.
                value: Arc::new(Address::from_typed_node_id(
                    CoreTypedNodeId::new(*value)
                        .expect("Failed to create a TypedNodeId from a trusted manifest address"),
                    network_id,
                )),
            },
        }
    }

    pub fn from_dynamic_global_address(
        native: &NativeDynamicGlobalAddress,
        network_id: u8,
    ) -> Self {
        match native {
            NativeDynamicGlobalAddress::Named(value) => Self::Named { value: *value },
            NativeDynamicGlobalAddress::Static(value) => Self::Static {
                value: Arc::new(Address::from_typed_node_id(*value, network_id)),
            },
        }
    }

    pub fn from_dynamic_package_address(
        native: &NativeDynamicPackageAddress,
        network_id: u8,
    ) -> Self {
        match native {
            NativeDynamicPackageAddress::Named(value) => Self::Named { value: *value },
            NativeDynamicPackageAddress::Static(value) => Self::Static {
                value: Arc::new(Address::from_typed_node_id(*value, network_id)),
            },
        }
    }
}

impl TryFrom<ManifestAddress> for NativeDynamicPackageAddress {
    type Error = RadixEngineToolkitError;

    fn try_from(value: ManifestAddress) -> std::result::Result<Self, Self::Error> {
        match value {
            ManifestAddress::Named { value } => Ok(Self::Named(value)),
            ManifestAddress::Static { value } => {
                (*value).try_into().map(Self::Static).map_err(Into::into)
            }
        }
    }
}

impl TryFrom<ManifestAddress> for NativeDynamicGlobalAddress {
    type Error = RadixEngineToolkitError;

    fn try_from(value: ManifestAddress) -> std::result::Result<Self, Self::Error> {
        match value {
            ManifestAddress::Named { value } => Ok(Self::Named(value)),
            ManifestAddress::Static { value } => {
                (*value).try_into().map(Self::Static).map_err(Into::into)
            }
        }
    }
}
