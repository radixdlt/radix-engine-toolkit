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

use scrypto::radix_engine_interface::blueprints::account::*;

use crate::error::Result;
use crate::model::address::EntityAddress;
use crate::model::value::ast::{ManifestAstValue, ManifestAstValueKind};
use crate::request::analyze_manifest_with_preview_context::ManifestProof;
use crate::request::ResourceSpecifier;
use crate::utils::is_account;
use crate::visitor::InstructionVisitor;

/// A visitor whose main responsibility is analyzing the call-method instructions for proof creation
#[derive(Debug, Default)]
pub struct AccountProofsInstructionVisitor {
    pub created_proofs: Vec<ManifestProof>,
}

impl InstructionVisitor for AccountProofsInstructionVisitor {
    fn visit_call_method(
        &mut self,
        component_address: &mut ManifestAstValue,
        method_name: &mut ManifestAstValue,
        args: &mut Option<Vec<ManifestAstValue>>,
    ) -> Result<()> {
        // Checking for created proofs
        let args = args.clone().unwrap_or_default();
        match (component_address, method_name, args.get(0), args.get(1)) {
            (
                ManifestAstValue::ComponentAddress {
                    address: ref component_address,
                }
                | ManifestAstValue::Address {
                    address:
                        EntityAddress::ComponentAddress {
                            address: ref component_address,
                        },
                },
                ManifestAstValue::String {
                    value: ref method_name,
                },
                Some(
                    ManifestAstValue::ResourceAddress {
                        address: resource_address,
                    }
                    | ManifestAstValue::Address {
                        address:
                            EntityAddress::ResourceAddress {
                                address: resource_address,
                            },
                    },
                ),
                None,
            ) if is_account(component_address) && method_name == ACCOUNT_CREATE_PROOF_IDENT => {
                self.created_proofs.push(ManifestProof {
                    origin: *component_address,
                    resource_address: *resource_address,
                    quantity: ResourceSpecifier::All,
                });
            }
            (
                ManifestAstValue::ComponentAddress {
                    address: component_address,
                }
                | ManifestAstValue::Address {
                    address:
                        EntityAddress::ComponentAddress {
                            address: component_address,
                        },
                },
                ManifestAstValue::String { value: method_name },
                Some(
                    ManifestAstValue::ResourceAddress {
                        address: resource_address,
                    }
                    | ManifestAstValue::Address {
                        address:
                            EntityAddress::ResourceAddress {
                                address: resource_address,
                            },
                    },
                ),
                Some(ManifestAstValue::Decimal { value: amount }),
            ) if is_account(component_address.address)
                && method_name == ACCOUNT_CREATE_PROOF_BY_AMOUNT_IDENT =>
            {
                self.created_proofs.push(ManifestProof {
                    origin: *component_address,
                    resource_address: *resource_address,
                    quantity: ResourceSpecifier::Amount { amount: *amount },
                });
            }
            (
                ManifestAstValue::ComponentAddress {
                    address: component_address,
                }
                | ManifestAstValue::Address {
                    address:
                        EntityAddress::ComponentAddress {
                            address: component_address,
                        },
                },
                ManifestAstValue::String { value: method_name },
                Some(
                    ManifestAstValue::ResourceAddress {
                        address: resource_address,
                    }
                    | ManifestAstValue::Address {
                        address:
                            EntityAddress::ResourceAddress {
                                address: resource_address,
                            },
                    },
                ),
                Some(ManifestAstValue::Array {
                    element_kind: ManifestAstValueKind::NonFungibleLocalId,
                    elements,
                }),
            ) if is_account(component_address.address)
                && method_name == ACCOUNT_CREATE_PROOF_BY_IDS_IDENT =>
            {
                // Attempt to get the non-fungible local ids. If one of them is wrong, return from
                // the visit
                let non_fungible_local_ids = {
                    let mut ids = BTreeSet::new();

                    for element in elements {
                        if let ManifestAstValue::NonFungibleLocalId { value } = element {
                            ids.insert(value.clone());
                        } else {
                            return Ok(());
                        }
                    }

                    ids
                };

                self.created_proofs.push(ManifestProof {
                    origin: *component_address,
                    resource_address: *resource_address,
                    quantity: ResourceSpecifier::Ids {
                        ids: non_fungible_local_ids,
                    },
                });
            }
            _ => {}
        }

        Ok(())
    }
}
