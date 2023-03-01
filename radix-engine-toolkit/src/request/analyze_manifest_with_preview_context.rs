// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at

//   http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use std::collections::{BTreeSet, HashMap};

use crate::error::Result;
use crate::model::address::{
    EntityAddress, NetworkAwareComponentAddress, NetworkAwarePackageAddress,
    NetworkAwareResourceAddress,
};
use crate::model::engine_identifier::RENodeId;
use crate::model::instruction::Instruction;
use crate::model::transaction::{InstructionKind, InstructionList, TransactionManifest};
use crate::request::convert_manifest::ConvertManifestRequest;
use crate::utils::is_account;
use crate::visitor::{
    traverse_instruction, AccountInteractionsInstructionVisitor, AccountProofsInstructionVisitor,
    AddressAggregatorVisitor, ValueNetworkAggregatorVisitor,
};
use scrypto::prelude::Decimal;
use toolkit_derive::serializable;

use super::convert_manifest::ConvertManifestHandler;
use super::traits::Handler;

// =================
// Model Definition
// =================

/// Analyzes the passed manifest to determine the entities that this manifest interacts with.
#[serializable]
pub struct AnalyzeManifestWithPreviewContextRequest {
    /// An unsigned 8 bit integer serialized as a string which represents the ID of the network
    /// that the manifest will be used on. The primary use of this is for any Bech32m encoding
    /// or decoding of addresses
    #[schemars(with = "String")]
    #[schemars(regex(pattern = "[0-9]+"))]
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub network_id: u8,

    /// The manifest to analyze.
    pub manifest: TransactionManifest,

    /// The resource changes as seen in the transaction preview.
    pub resource_changes: Vec<ResourceChange>,
}

/// The response of the [`AnalyzeManifestWithPreviewContextRequest`]
#[serializable]
pub struct AnalyzeManifestWithPreviewContextResponse {
    /// The set of global addresses seen in the manifest.
    ///
    /// This encountered addresses are obtained by the toolkit by doing static analysis on the
    /// manifest to determine what the different addresses seen there are. So, if an address is
    /// in the manifest, then it should be here as well. Anything that is not in the manifest is
    /// not included here even if there are calls that happen to it.
    addresses_encountered: AddressesEncountered,

    /// The set of proofs which are created from accounts in this manifest.
    ///
    /// This field is populated by statically analyzing the transaction manifest and parsing the
    /// `CALL_METHOD` instructions to determine which ones create proofs from accounts, what the
    /// account addresses are, what the resource addresses are, as well as the quantity of resources
    /// created in that proof.
    ///
    /// With the above in mind, it means that this field will not show proofs which are not created
    /// from accounts or those created from interactions with other components. This is strictly
    /// for which proofs originated from which accounts.
    #[schemars(with = "Vec<ManifestProof>")]
    #[serde_as(as = "Vec<serde_with::TryFromInto<ManifestProof>>")]
    proofs: Vec<ManifestProof>,

    /// The account actions which took place during the transaction such as withdraws and deposits.
    ///
    /// This field is populated by aggregating, filtering, and parsing the data data in the preview.
    /// Thus, this information is not as trustworthy as information obtained by statically analyzing
    /// the manifest.
    account_actions: AccountActions,
}

/// Defines a set of entities encountered when parsing the transaction manifest for global addresses
#[serializable]
pub struct AddressesEncountered {
    /// A set of all of the package addresses seen in the manifest. The underlying type of this is
    /// an array of `PackageAddress`es from the `Value` model.
    #[schemars(with = "BTreeSet<EntityAddress>")]
    #[serde_as(as = "BTreeSet<serde_with::TryFromInto<EntityAddress>>")]
    pub package_addresses: BTreeSet<NetworkAwarePackageAddress>,

    /// A set of all of the component addresses seen in the manifest. The underlying type of this
    /// is an array of `ComponentAddress`es from the `Value` model.
    #[schemars(with = "BTreeSet<EntityAddress>")]
    #[serde_as(as = "BTreeSet<serde_with::TryFromInto<EntityAddress>>")]
    pub component_addresses: BTreeSet<NetworkAwareComponentAddress>,

    /// A set of all of the resource addresses seen in the manifest. The underlying type of this is
    /// an array of `ResourceAddress`es from the `Value` model.
    #[schemars(with = "BTreeSet<EntityAddress>")]
    #[serde_as(as = "BTreeSet<serde_with::TryFromInto<EntityAddress>>")]
    pub resource_addresses: BTreeSet<NetworkAwareResourceAddress>,
}

/// Defines a manifest proof which originates from some component, is of some resource address,
/// and has some amount or non-fungible id set.
#[serializable]
#[derive(PartialEq, PartialOrd, Eq, Ord)]
pub struct ManifestProof {
    /// The address of the component which the proof originated from.
    #[schemars(with = "EntityAddress")]
    #[serde_as(as = "serde_with::TryFromInto<EntityAddress>")]
    pub origin: NetworkAwareComponentAddress,

    /// The address of the resource that this proof is created from.
    #[schemars(with = "EntityAddress")]
    #[serde_as(as = "serde_with::TryFromInto<EntityAddress>")]
    pub resource_address: NetworkAwareResourceAddress,

    /// The quantity of resources that the proof was created from.
    pub quantity: ResourceSpecifier,
}

/// Defines the set of account actions that can be seen in the transaction preview. This mainly
/// describes the withdraws and deposits.
#[serializable]
#[derive(PartialEq, PartialOrd, Eq, Ord, Default)]
pub struct AccountActions {
    pub withdraws: Vec<AccountAction>,
    pub deposits: Vec<AccountAction>,
}

/// Describes some account action.
#[serializable]
#[derive(PartialEq, PartialOrd, Eq, Ord)]
pub struct AccountAction {
    /// The address of the account component that the action was performed against.
    #[schemars(with = "EntityAddress")]
    #[serde_as(as = "serde_with::TryFromInto<EntityAddress>")]
    pub component_address: NetworkAwareComponentAddress,

    /// The resource address of the resource that the action was performed against (e.g. if we're
    /// describing a withdraw action, then this is a the address of the resource that was withdrawn).
    #[schemars(with = "EntityAddress")]
    #[serde_as(as = "serde_with::TryFromInto<EntityAddress>")]
    pub resource_address: NetworkAwareResourceAddress,

    /// The amount of the action (always a positive amount) (e.g., if we're describing a withdraw
    /// action, then this field describes the amount of funds withdrawn from the account)
    #[schemars(regex(pattern = "[+-]?([0-9]*[.])?[0-9]+"))]
    #[schemars(with = "String")]
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub amount: Decimal,
}

/// A specifier which is used to quantify resources
#[serializable]
#[serde(tag = "type")]
#[derive(PartialEq, PartialOrd, Eq, Ord)]
pub enum ResourceSpecifier {
    /// Specifies that some operation was performed with all of that resource and not a specific
    /// amount or some specific non-fungible ids.
    All,

    /// Specifies that some operation was performed with an amount of a resource of the given
    /// resource address.
    Amount {
        /// Represents an amount of the resource. This is a decimal serialized as a string.
        #[schemars(regex(pattern = "[+-]?([0-9]*[.])?[0-9]+"))]
        #[schemars(with = "String")]
        #[serde_as(as = "serde_with::DisplayFromStr")]
        amount: Decimal,
    },

    /// Specifies that some operation was performed with some non-fungible ids.
    Ids {
        #[schemars(with = "BTreeSet<crate::model::address::NonFungibleLocalId>")]
        #[serde_as(
            as = "BTreeSet<serde_with::TryFromInto<crate::model::address::NonFungibleLocalId>>"
        )]
        ids: BTreeSet<scrypto::prelude::NonFungibleLocalId>,
    },
}

#[serializable]
#[derive(PartialEq, PartialOrd, Eq, Ord)]
pub struct ResourceChange {
    /// The component id of the component that owns the vault which had the resource changes.
    pub owner_id: RENodeId,

    /// The resource address of the resource contained in the vault.
    #[schemars(with = "String")]
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub resource_address: NetworkAwareResourceAddress,

    /// The amount of change.
    #[schemars(regex(pattern = "[+-]?([0-9]*[.])?[0-9]+"))]
    #[schemars(with = "String")]
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub amount: Decimal,
}

// ===============
// Implementation
// ===============

pub struct AnalyzeManifestWithPreviewContextHandler;

impl Handler<AnalyzeManifestWithPreviewContextRequest, AnalyzeManifestWithPreviewContextResponse>
    for AnalyzeManifestWithPreviewContextHandler
{
    fn pre_process(
        mut request: AnalyzeManifestWithPreviewContextRequest,
    ) -> Result<AnalyzeManifestWithPreviewContextRequest> {
        // Visitors
        let mut network_aggregator_visitor = ValueNetworkAggregatorVisitor::default();

        // Instructions
        let instructions: &mut [Instruction] = match request.manifest.instructions {
            InstructionList::Parsed(ref mut instructions) => instructions,
            InstructionList::String(..) => &mut [],
        };

        // Traverse instructions with visitors
        instructions
            .iter_mut()
            .map(|instruction| {
                traverse_instruction(instruction, &mut [&mut network_aggregator_visitor], &mut [])
            })
            .collect::<Result<Vec<_>>>()?;

        // Check for network mismatches
        if let Some(network_id) = network_aggregator_visitor
            .0
            .iter()
            .find(|network_id| **network_id != request.network_id)
        {
            return Err(crate::error::Error::NetworkMismatchError {
                found: *network_id,
                expected: request.network_id,
            });
        }
        Ok(request)
    }

    fn handle(
        request: &AnalyzeManifestWithPreviewContextRequest,
    ) -> Result<AnalyzeManifestWithPreviewContextResponse> {
        // Getting the instructions in the passed manifest as parsed instructions
        let mut instructions = {
            let manifest = ConvertManifestHandler::fulfill(ConvertManifestRequest {
                network_id: request.network_id,
                instructions_output_kind: InstructionKind::Parsed,
                manifest: request.manifest.clone(),
            })?
            .manifest;

            match manifest.instructions {
                InstructionList::Parsed(instructions) => Ok(instructions),
                InstructionList::String(..) => Err(crate::error::Error::Infallible {
                    message: "Impossible Case! We converted to parsed but it's still a string!"
                        .into(),
                }),
            }
        }?;

        // Setting up the visitors and traversing the instructions
        let mut address_aggregator_visitor = AddressAggregatorVisitor::default();
        let mut account_interactions_visitor = AccountInteractionsInstructionVisitor::default();
        let mut account_proofs_visitor = AccountProofsInstructionVisitor::default();
        instructions
            .iter_mut()
            .map(|instruction| {
                traverse_instruction(
                    instruction,
                    &mut [&mut address_aggregator_visitor],
                    &mut [
                        &mut account_interactions_visitor,
                        &mut account_proofs_visitor,
                    ],
                )
            })
            .collect::<Result<Vec<_>>>()?;

        // Analyzing the transaction preview to determine what the resource changes and get the
        // deposit and withdraw amounts.
        let account_actions = {
            // Filtering the resource changes for the changes that only on accounts
            let mut withdrawn_resources = HashMap::new();
            let mut deposited_resources = HashMap::new();
            for resource_change in request.resource_changes.clone().into_iter() {
                // We only care about resource changes on accounts, so, we skip everything else.
                let component_address = match resource_change.owner_id {
                    RENodeId::GlobalComponent { address } if is_account(address) => address,
                    _ => continue,
                };

                // If the amount is negative, then this was withdraw operation. If it is positive
                // then this was a deposit
                if resource_change.amount.is_positive() {
                    *deposited_resources
                        .entry((component_address, resource_change.resource_address))
                        .or_default() += resource_change.amount;
                } else {
                    *withdrawn_resources
                        .entry((component_address, resource_change.resource_address))
                        .or_default() -= resource_change.amount;
                }
            }

            AccountActions {
                withdraws: withdrawn_resources
                    .into_iter()
                    .map(
                        |((component_address, resource_address), amount)| AccountAction {
                            resource_address,
                            component_address,
                            amount,
                        },
                    )
                    .collect(),
                deposits: deposited_resources
                    .into_iter()
                    .map(
                        |((component_address, resource_address), amount)| AccountAction {
                            resource_address,
                            component_address,
                            amount,
                        },
                    )
                    .collect(),
            }
        };

        let response = AnalyzeManifestWithPreviewContextResponse {
            addresses_encountered: AddressesEncountered {
                package_addresses: address_aggregator_visitor.package_addresses,
                resource_addresses: address_aggregator_visitor.resource_addresses,
                component_addresses: address_aggregator_visitor.component_addresses,
            },
            proofs: account_proofs_visitor.created_proofs,
            account_actions,
        };
        Ok(response)
    }

    fn post_process(
        _: &AnalyzeManifestWithPreviewContextRequest,
        response: AnalyzeManifestWithPreviewContextResponse,
    ) -> Result<AnalyzeManifestWithPreviewContextResponse> {
        Ok(response)
    }
}
