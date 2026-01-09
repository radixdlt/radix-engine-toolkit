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

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Enum)]
pub enum ManifestAddress {
    Named { named_address_id: u32 },
    Static { static_address: Arc<Address> },
}

impl From<ManifestAddress> for engine::ManifestAddress {
    fn from(value: ManifestAddress) -> Self {
        match value {
            ManifestAddress::Named {
                named_address_id: value,
            } => Self::Named(engine::ManifestNamedAddress(value)),
            ManifestAddress::Static {
                static_address: value,
            } => Self::Static((*value).into()),
        }
    }
}

impl ManifestAddress {
    pub fn new(native: &engine::ManifestAddress, network_id: u8) -> Self {
        match native {
            engine::ManifestAddress::Named(value) => Self::Named {
                named_address_id: value.0,
            },
            engine::ManifestAddress::Static(value) => Self::Static {
                static_address: Arc::new(Address::from_node_id(
                    *value, network_id,
                )),
            },
        }
    }

    pub fn from_dynamic_global_address(
        native: &engine::DynamicGlobalAddress,
        network_id: u8,
    ) -> Self {
        match native {
            engine::DynamicGlobalAddress::Named(value) => Self::Named {
                named_address_id: value.0,
            },
            engine::DynamicGlobalAddress::Static(value) => Self::Static {
                static_address: Arc::new(Address::from_node_id(
                    *value, network_id,
                )),
            },
        }
    }

    pub fn from_dynamic_package_address(
        native: &engine::DynamicPackageAddress,
        network_id: u8,
    ) -> Self {
        match native {
            engine::DynamicPackageAddress::Named(value) => Self::Named {
                named_address_id: value.0,
            },
            engine::DynamicPackageAddress::Static(value) => Self::Static {
                static_address: Arc::new(Address::from_node_id(
                    *value, network_id,
                )),
            },
        }
    }
}

impl TryFrom<ManifestAddress> for engine::DynamicPackageAddress {
    type Error = RadixEngineToolkitError;

    fn try_from(
        value: ManifestAddress,
    ) -> std::result::Result<Self, Self::Error> {
        match value {
            ManifestAddress::Named {
                named_address_id: value,
            } => Ok(Self::Named(engine::ManifestNamedAddress(value))),
            ManifestAddress::Static {
                static_address: value,
            } => (*value).try_into().map(Self::Static),
        }
    }
}

impl TryFrom<ManifestAddress> for engine::DynamicGlobalAddress {
    type Error = RadixEngineToolkitError;

    fn try_from(
        value: ManifestAddress,
    ) -> std::result::Result<Self, Self::Error> {
        match value {
            ManifestAddress::Named {
                named_address_id: value,
            } => Ok(Self::Named(engine::ManifestNamedAddress(value))),
            ManifestAddress::Static {
                static_address: value,
            } => (*value).try_into().map(Self::Static),
        }
    }
}
