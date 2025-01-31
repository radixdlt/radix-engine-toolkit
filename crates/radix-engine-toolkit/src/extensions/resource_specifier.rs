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

#[ext_sized]
pub impl ResourceSpecifier {
    fn new_empty(resource_address: ResourceAddress) -> Self {
        match resource_address.is_fungible() {
            true => Self::Amount(resource_address, Default::default()),
            false => Self::Ids(resource_address, Default::default()),
        }
    }

    fn resource_address(&self) -> &ResourceAddress {
        match self {
            Self::Amount(address, ..) | Self::Ids(address, ..) => address,
        }
    }

    fn amount(&self) -> Decimal {
        match self {
            Self::Amount(.., amount) => *amount,
            Self::Ids(.., ids) => ids.len().into(),
        }
    }

    fn ids(&self) -> Option<&IndexSet<NonFungibleLocalId>> {
        match self {
            Self::Ids(.., ids) => Some(ids),
            Self::Amount(..) => None,
        }
    }

    fn is_empty(&self) -> bool {
        self.amount().is_zero()
    }
}
