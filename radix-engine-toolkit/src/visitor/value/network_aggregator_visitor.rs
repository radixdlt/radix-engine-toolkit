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

use crate::error::Error;
use crate::model::value::ast::ManifestAstValue;
use crate::visitor::ManifestAstValueVisitor;

/// A value visitor whose main job is to find all of the different network IDs that the different
/// addresses use. This is typically used in operations where we wish to check for network id
/// mismatches.
#[derive(Debug, Default)]
pub struct ValueNetworkAggregatorVisitor(pub BTreeSet<u8>);

impl ManifestAstValueVisitor for ValueNetworkAggregatorVisitor {
    fn visit_component_address(
        &mut self,
        value: &mut crate::model::value::ast::ManifestAstValue,
    ) -> crate::error::Result<()> {
        if let ManifestAstValue::ComponentAddress { address } = value {
            self.0.insert(address.network_id);
            Ok(())
        } else {
            Err(Error::Infallible {
                message: "Expected component address!".into(),
            })
        }
    }

    fn visit_resource_address(
        &mut self,
        value: &mut crate::model::value::ast::ManifestAstValue,
    ) -> crate::error::Result<()> {
        if let ManifestAstValue::ResourceAddress { address } = value {
            self.0.insert(address.network_id);
            Ok(())
        } else {
            Err(Error::Infallible {
                message: "Expected resource address!".into(),
            })
        }
    }

    fn visit_package_address(
        &mut self,
        value: &mut crate::model::value::ast::ManifestAstValue,
    ) -> crate::error::Result<()> {
        if let ManifestAstValue::PackageAddress { address } = value {
            self.0.insert(address.network_id);
            Ok(())
        } else {
            Err(Error::Infallible {
                message: "Expected package address!".into(),
            })
        }
    }

    fn visit_non_fungible_global_id(
        &mut self,
        value: &mut crate::model::value::ast::ManifestAstValue,
    ) -> crate::error::Result<()> {
        if let ManifestAstValue::NonFungibleGlobalId { address } = value {
            self.0.insert(address.resource_address.network_id);
            Ok(())
        } else {
            Err(Error::Infallible {
                message: "Expected non-fungible global id!".into(),
            })
        }
    }

    fn visit_address(
        &mut self,
        value: &mut crate::model::value::ast::ManifestAstValue,
    ) -> crate::error::Result<()> {
        if let ManifestAstValue::Address { address } = value {
            self.0.insert(address.network_id());
            Ok(())
        } else {
            Err(Error::Infallible {
                message: "Expected an address".into(),
            })
        }
    }
}
