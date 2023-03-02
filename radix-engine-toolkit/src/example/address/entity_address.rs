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

use scrypto::prelude::{FAUCET_COMPONENT, FAUCET_PACKAGE, RADIX_TOKEN};

use crate::model::address::{
    EntityAddress, NetworkAwareComponentAddress, NetworkAwarePackageAddress,
    NetworkAwareResourceAddress,
};

pub fn entity_component_address() -> EntityAddress {
    EntityAddress::ComponentAddress {
        address: component_address(),
    }
}

pub fn entity_resource_address() -> EntityAddress {
    EntityAddress::ResourceAddress {
        address: resource_address(),
    }
}

pub fn entity_package_address() -> EntityAddress {
    EntityAddress::PackageAddress {
        address: package_address(),
    }
}

pub fn resource_address() -> NetworkAwareResourceAddress {
    NetworkAwareResourceAddress {
        network_id: 0x01,
        address: RADIX_TOKEN,
    }
}

pub fn component_address() -> NetworkAwareComponentAddress {
    NetworkAwareComponentAddress {
        network_id: 0x01,
        address: FAUCET_COMPONENT,
    }
}

pub fn package_address() -> NetworkAwarePackageAddress {
    NetworkAwarePackageAddress {
        network_id: 0x01,
        address: FAUCET_PACKAGE,
    }
}
