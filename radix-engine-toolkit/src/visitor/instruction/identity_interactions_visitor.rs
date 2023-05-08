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

use crate::error::Result;
use crate::model::address::{EntityAddress, NetworkAwareComponentAddress};
use crate::model::value::ast::ManifestAstValue;
use crate::utils::is_identity;
use crate::visitor::InstructionVisitor;

/// A visitor whose main responsibility is determining the kind of interactions involved with
/// accounts
#[derive(Debug, Default)]
pub struct IdentityInteractionsInstructionVisitor(pub BTreeSet<NetworkAwareComponentAddress>);

impl InstructionVisitor for IdentityInteractionsInstructionVisitor {
    fn visit_set_metadata(
        &mut self,
        entity_address: &mut ManifestAstValue,
        _: &mut ManifestAstValue,
        _: &mut ManifestAstValue,
    ) -> Result<()> {
        match entity_address {
            ManifestAstValue::Address {
                address:
                    EntityAddress::ComponentAddress {
                        address: ref component_address,
                    },
            } if is_identity(component_address) => {
                self.0.insert(*component_address);
            }
            _ => {}
        }

        Ok(())
    }

    fn visit_set_component_royalty_config(
        &mut self,
        component_address: &mut crate::model::value::ast::ManifestAstValue,
        _: &mut crate::model::value::ast::ManifestAstValue,
    ) -> Result<()> {
        match component_address {
            ManifestAstValue::Address {
                address:
                    EntityAddress::ComponentAddress {
                        address: ref component_address,
                    },
            } if is_identity(component_address) => {
                self.0.insert(*component_address);
            }
            _ => {}
        }
        Ok(())
    }

    fn visit_claim_component_royalty(
        &mut self,
        component_address: &mut crate::model::value::ast::ManifestAstValue,
    ) -> Result<()> {
        match component_address {
            ManifestAstValue::Address {
                address:
                    EntityAddress::ComponentAddress {
                        address: ref component_address,
                    },
            } if is_identity(component_address) => {
                self.0.insert(*component_address);
            }
            _ => {}
        }
        Ok(())
    }

    fn visit_set_method_access_rule(
        &mut self,
        entity_address: &mut crate::model::value::ast::ManifestAstValue,
        _: &mut crate::model::value::ast::ManifestAstValue,
        _: &mut crate::model::value::ast::ManifestAstValue,
    ) -> Result<()> {
        match entity_address {
            ManifestAstValue::Address {
                address:
                    EntityAddress::ComponentAddress {
                        address: ref component_address,
                    },
            } if is_identity(component_address) => {
                self.0.insert(*component_address);
            }
            _ => {}
        }
        Ok(())
    }
}
