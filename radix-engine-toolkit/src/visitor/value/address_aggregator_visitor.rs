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

use std::collections::BTreeSet;

use crate::error::VisitorError;
use crate::model::address::NetworkAwareNodeId;
use crate::model::value::ast::ManifestAstValue;
use crate::visitor::ManifestAstValueVisitor;

/// An address aggregator visitor which collects all of the encountered global entity addresses and
/// stored them in its state.
#[derive(Debug, Default)]
pub struct AddressAggregatorVisitor {
    pub component_addresses: BTreeSet<NetworkAwareNodeId>,
    pub resource_addresses: BTreeSet<NetworkAwareNodeId>,
    pub package_addresses: BTreeSet<NetworkAwareNodeId>,
}

impl ManifestAstValueVisitor for AddressAggregatorVisitor {
    fn visit_address(&mut self, value: &mut ManifestAstValue) -> Result<(), VisitorError> {
        if let ManifestAstValue::Address { value: address } = value {
            let node_id = address.node_id();
            if node_id.is_global_component() {
                self.component_addresses.insert(*address);
            } else if node_id.is_global_resource() {
                self.resource_addresses.insert(*address);
            } else if node_id.is_global_package() {
                self.package_addresses.insert(*address);
            };
        }
        Ok(())
    }

    fn visit_non_fungible_global_id(
        &mut self,
        value: &mut ManifestAstValue,
    ) -> Result<(), VisitorError> {
        if let ManifestAstValue::NonFungibleGlobalId {
            resource_address, ..
        } = value
        {
            self.resource_addresses.insert(*resource_address);
        }
        Ok(())
    }
}
