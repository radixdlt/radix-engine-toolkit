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

use scrypto::prelude::{Decimal, NonFungibleLocalId};
use std::collections::BTreeSet;
use toolkit_derive::serializable;

use super::address::NetworkAwareNodeId;

/// Specifies resources either through amounts for fungible and non-fungible resources or through
/// ids for non-fungible resources.
#[serializable]
#[derive(PartialEq, PartialOrd, Eq, Ord)]
#[serde(tag = "type")]
pub enum ResourceQuantifier {
    // Specifies resources using a decimal quantity.
    Amount {
        /// A specifier of the resource manager, can either be an address for already existing
        /// resource managers or an index for newly created resource managers that can not be
        /// queried through the network gateway.
        resource_address: ResourceManagerSpecifier,

        /// The amount of resources withdrawn from the account. This is a decimal value which is
        /// serialized as a string.
        #[schemars(regex(pattern = "[+-]?([0-9]*[.])?[0-9]+"))]
        #[schemars(with = "String")]
        #[serde_as(as = "serde_with::DisplayFromStr")]
        amount: Decimal,
    },
    // Specifies resources through a set of non-fungible local id.
    Ids {
        /// A specifier of the resource manager, can either be an address for already existing
        /// resource managers or an index for newly created resource managers that can not be
        /// queried through the network gateway.
        resource_address: ResourceManagerSpecifier,

        /// The set of non-fungible ids
        #[schemars(regex(pattern = "[+-]?([0-9]*[.])?[0-9]+"))]
        #[schemars(with = "BTreeSet<crate::model::address::NonFungibleLocalId>")]
        #[serde_as(
            as = "BTreeSet<serde_with::TryFromInto<crate::model::address::NonFungibleLocalId>>"
        )]
        ids: BTreeSet<NonFungibleLocalId>,
    },
}

#[serializable]
#[derive(PartialEq, PartialOrd, Eq, Ord)]
#[serde(tag = "type")]
pub enum ResourceManagerSpecifier {
    Existing {
        #[schemars(with = "String")]
        #[serde_as(as = "serde_with::DisplayFromStr")]
        address: NetworkAwareNodeId,
    },
    NewlyCreated {
        #[schemars(with = "String")]
        #[serde_as(as = "serde_with::DisplayFromStr")]
        index: u32,
    },
}
