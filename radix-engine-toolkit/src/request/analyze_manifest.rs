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

use std::collections::BTreeSet;

use crate::error::Result;
use crate::model::instruction_list::InstructionKind;
use crate::model::TransactionManifest;
use crate::{
    traverse_instruction, AccountInteractionsInstructionVisitor, AddressValueAggregator,
    ConvertManifestHandler, Handler, Instruction, InstructionList, NetworkAwareComponentAddress,
    NetworkAwarePackageAddress, NetworkAwareResourceAddress, ValueNetworkAggregatorVisitor,
};
use scrypto::prelude::ComponentAddress;
use serializable::serializable;

// =================
// Model Definition
// =================

/// Analyzes the passed manifest to determine the entities that this manifest interacts with.
#[serializable]
pub struct AnalyzeManifestRequest {
    /// An unsigned 8 bit integer serialized as a string which represents the ID of the network
    /// that the manifest will be used on. The primary use of this is for any Bech32m encoding
    /// or decoding of addresses
    #[schemars(with = "String")]
    #[schemars(regex(pattern = "[0-9]+"))]
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub network_id: u8,

    /// The manifest to convert to the format described by `instructions_output_kind`
    pub manifest: TransactionManifest,
}

/// The response of the [`AnalyzeManifestRequest`]
#[serializable]
pub struct AnalyzeManifestResponse {
    /// A set of all of the package addresses seen in the manifest. The underlying type of this is
    /// an array of `PackageAddress`es from the `Value` model.
    #[schemars(with = "BTreeSet<crate::model::Value>")]
    #[serde_as(as = "BTreeSet<serde_with::TryFromInto<crate::model::Value>>")]
    pub package_addresses: BTreeSet<NetworkAwarePackageAddress>,

    /// A set of all of the component addresses seen in the manifest. The underlying type of this
    /// is an array of `ComponentAddress`es from the `Value` model.
    #[schemars(with = "BTreeSet<crate::model::Value>")]
    #[serde_as(as = "BTreeSet<serde_with::TryFromInto<crate::model::Value>>")]
    pub component_addresses: BTreeSet<NetworkAwareComponentAddress>,

    /// A set of all of the resource addresses seen in the manifest. The underlying type of this is
    /// an array of `ResourceAddress`es from the `Value` model.
    #[schemars(with = "BTreeSet<crate::model::Value>")]
    #[serde_as(as = "BTreeSet<serde_with::TryFromInto<crate::model::Value>>")]
    pub resource_addresses: BTreeSet<NetworkAwareResourceAddress>,

    /// A set of all of the account component addresses seen in the manifest. The underlying type
    /// of this is an array of `ComponentAddress`es from the `Value` model.
    #[schemars(with = "BTreeSet<crate::model::Value>")]
    #[serde_as(as = "BTreeSet<serde_with::TryFromInto<crate::model::Value>>")]
    pub account_addresses: BTreeSet<NetworkAwareComponentAddress>,

    /// A set of all of the account component addresses in the manifest which had methods invoked
    /// on them that would typically require auth (or a signature) to be called successfully.
    /// This is a subset of the addresses seen in `account_addresses`. The underlying type of
    /// this  is an array of `ComponentAddress`es from the `Value` model.
    #[schemars(with = "BTreeSet<crate::model::Value>")]
    #[serde_as(as = "BTreeSet<serde_with::TryFromInto<crate::model::Value>>")]
    pub accounts_requiring_auth: BTreeSet<NetworkAwareComponentAddress>,

    /// A set of all of the account component addresses in the manifest which were withdrawn from.
    /// This is a subset of the addresses seen in `account_addresses`. The underlying type  of this
    /// is an array of `ComponentAddress`es from the `Value` model.
    #[schemars(with = "BTreeSet<crate::model::Value>")]
    #[serde_as(as = "BTreeSet<serde_with::TryFromInto<crate::model::Value>>")]
    pub accounts_withdrawn_from: BTreeSet<NetworkAwareComponentAddress>,

    /// A set of all of the account component addresses in the manifest which were deposited into.
    /// This is a subset of the addresses seen in `account_addresses`. The underlying type  of this
    /// is an array of `ComponentAddress`es from the `Value` model.
    #[schemars(with = "BTreeSet<crate::model::Value>")]
    #[serde_as(as = "BTreeSet<serde_with::TryFromInto<crate::model::Value>>")]
    pub accounts_deposited_into: BTreeSet<NetworkAwareComponentAddress>,
}

// ===============
// Implementation
// ===============

pub struct AnalyzeManifestHandler;

impl Handler<AnalyzeManifestRequest, AnalyzeManifestResponse> for AnalyzeManifestHandler {
    fn pre_process(mut request: AnalyzeManifestRequest) -> Result<AnalyzeManifestRequest> {
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
            return Err(crate::Error::NetworkMismatchError {
                found: *network_id,
                expected: request.network_id,
            });
        }
        Ok(request)
    }

    fn handle(request: &AnalyzeManifestRequest) -> Result<AnalyzeManifestResponse> {
        // Getting the instructions in the passed manifest as parsed instructions
        let mut instructions = {
            let manifest = ConvertManifestHandler::fulfill(crate::ConvertManifestRequest {
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
        let mut address_aggregator_visitor = AddressValueAggregator::default();
        let mut account_interactions_visitor = AccountInteractionsInstructionVisitor::default();
        instructions
            .iter_mut()
            .map(|instruction| {
                traverse_instruction(
                    instruction,
                    &mut [&mut address_aggregator_visitor],
                    &mut [&mut account_interactions_visitor],
                )
            })
            .collect::<Result<Vec<_>>>()?;

        let response = AnalyzeManifestResponse {
            package_addresses: address_aggregator_visitor.package_addresses,
            resource_addresses: address_aggregator_visitor.resource_addresses,
            component_addresses: address_aggregator_visitor.component_addresses.clone(),
            account_addresses: address_aggregator_visitor
                .component_addresses
                .into_iter()
                .filter(|address| {
                    matches!(
                        address.address,
                        ComponentAddress::Account(..)
                            | ComponentAddress::EcdsaSecp256k1VirtualAccount(..)
                            | ComponentAddress::EddsaEd25519VirtualAccount(..)
                    )
                })
                .collect(),
            accounts_requiring_auth: account_interactions_visitor.auth_required,
            accounts_withdrawn_from: account_interactions_visitor.accounts_withdrawn_from,
            accounts_deposited_into: account_interactions_visitor.accounts_deposited_into,
        };
        Ok(response)
    }

    fn post_process(
        _: &AnalyzeManifestRequest,
        response: AnalyzeManifestResponse,
    ) -> Result<AnalyzeManifestResponse> {
        Ok(response)
    }
}
