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

// TODO(manifest-model): This type is no longer needed when the manifest model
// work is completed. Once done, we should remove this type.
#[derive(Debug, Clone, PartialEq, Eq, ManifestSbor)]
pub enum ManifestResourceSpecifier {
    Amount(ManifestResourceAddress, Decimal),
    Ids(ManifestResourceAddress, IndexSet<NonFungibleLocalId>),
}

impl ManifestResourceSpecifier {
    pub fn resource_address(&self) -> &ManifestResourceAddress {
        match self {
            Self::Amount(address, ..) | Self::Ids(address, ..) => address,
        }
    }

    pub fn amount(&self) -> Decimal {
        match self {
            Self::Amount(.., amount) => *amount,
            Self::Ids(.., ids) => ids.len().into(),
        }
    }

    pub fn ids(&self) -> Option<&IndexSet<NonFungibleLocalId>> {
        match self {
            Self::Ids(.., ids) => Some(ids),
            Self::Amount(..) => None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.amount().is_zero()
    }
}

impl From<ResourceSpecifier> for ManifestResourceSpecifier {
    fn from(value: ResourceSpecifier) -> Self {
        match value {
            ResourceSpecifier::Amount(resource_address, amount) => {
                Self::Amount(resource_address.into(), amount)
            }
            ResourceSpecifier::Ids(resource_address, ids) => {
                Self::Ids(resource_address.into(), ids)
            }
        }
    }
}
