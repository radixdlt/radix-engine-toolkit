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

use crate::error::VisitorError;
use crate::model::address::utils::is_account;
use crate::model::address::NetworkAwareNodeId;
use crate::model::instruction::Instruction;
use crate::model::transaction::{InstructionKind, InstructionList, TransactionManifest};
use crate::visitor::{
    traverse_instruction, AccountInteractionsInstructionVisitor, AddressAggregatorVisitor,
    ValueNetworkAggregatorVisitor,
};
use toolkit_derive::serializable;

use super::convert_manifest;
use super::traits::InvocationHandler;

// =================
// Model Definition
// =================

/// Analyzes the passed manifest to determine the entities that this manifest interacts with.
#[serializable]
pub struct Input {
    /// An unsigned 8 bit integer serialized as a string which represents the ID of the network
    /// that the manifest will be used on. The primary use of this is for any Bech32m encoding
    /// or decoding of addresses
    #[schemars(with = "String")]
    #[schemars(regex(pattern = "[0-9]+"))]
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub network_id: u8,

    /// The manifest to analyze.
    pub manifest: TransactionManifest,
}

/// The response of the [`Input`]
#[serializable]
pub struct Output {
    /// A set of all of the package addresses seen in the manifest. The underlying type of this is
    /// an array of `Address`es from the `Value` model.
    #[schemars(with = "BTreeSet<String>")]
    #[serde_as(as = "BTreeSet<serde_with::DisplayFromStr>")]
    pub package_addresses: BTreeSet<NetworkAwareNodeId>,

    /// A set of all of the component addresses seen in the manifest. The underlying type of this
    /// is an array of `Address`es from the `Value` model.
    #[schemars(with = "BTreeSet<String>")]
    #[serde_as(as = "BTreeSet<serde_with::DisplayFromStr>")]
    pub component_addresses: BTreeSet<NetworkAwareNodeId>,

    /// A set of all of the resource addresses seen in the manifest. The underlying type of this is
    /// an array of `Address`es from the `Value` model.
    #[schemars(with = "BTreeSet<String>")]
    #[serde_as(as = "BTreeSet<serde_with::DisplayFromStr>")]
    pub resource_addresses: BTreeSet<NetworkAwareNodeId>,

    /// A set of all of the account component addresses seen in the manifest. The underlying type
    /// of this is an array of `Address`es from the `Value` model.
    #[schemars(with = "BTreeSet<String>")]
    #[serde_as(as = "BTreeSet<serde_with::DisplayFromStr>")]
    pub account_addresses: BTreeSet<NetworkAwareNodeId>,

    /// A set of all of the account component addresses in the manifest which had methods invoked
    /// on them that would typically require auth (or a signature) to be called successfully.
    /// This is a subset of the addresses seen in `account_addresses`. The underlying type of
    /// this  is an array of `Address`es from the `Value` model.
    #[schemars(with = "BTreeSet<String>")]
    #[serde_as(as = "BTreeSet<serde_with::DisplayFromStr>")]
    pub accounts_requiring_auth: BTreeSet<NetworkAwareNodeId>,

    /// A set of all of the account component addresses in the manifest which were withdrawn from.
    /// This is a subset of the addresses seen in `account_addresses`. The underlying type  of this
    /// is an array of `Address`es from the `Value` model.
    #[schemars(with = "BTreeSet<String>")]
    #[serde_as(as = "BTreeSet<serde_with::DisplayFromStr>")]
    pub accounts_withdrawn_from: BTreeSet<NetworkAwareNodeId>,

    /// A set of all of the account component addresses in the manifest which were deposited into.
    /// This is a subset of the addresses seen in `account_addresses`. The underlying type  of this
    /// is an array of `Address`es from the `Value` model.
    #[schemars(with = "BTreeSet<String>")]
    #[serde_as(as = "BTreeSet<serde_with::DisplayFromStr>")]
    pub accounts_deposited_into: BTreeSet<NetworkAwareNodeId>,
}

// ===============
// Implementation
// ===============

pub struct Handler;
impl InvocationHandler<Input, Output> for Handler {
    type Error = Error;

    fn pre_process(mut request: Input) -> Result<Input, Error> {
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
            .collect::<Result<Vec<_>, _>>()
            .map_err(Self::Error::PreProcessingError)?;

        // Check for network mismatches
        if let Some(network_id) = network_aggregator_visitor
            .0
            .iter()
            .find(|network_id| **network_id != request.network_id)
        {
            return Err(Self::Error::InvalidNetworkIdEncountered {
                found: *network_id,
                expected: request.network_id,
            });
        }
        Ok(request)
    }

    fn handle(request: &Input) -> Result<Output, Error> {
        // Getting the instructions in the passed manifest as parsed instructions
        let mut instructions = {
            let manifest = convert_manifest::Handler::fulfill(convert_manifest::Input {
                network_id: request.network_id,
                instructions_output_kind: InstructionKind::Parsed,
                manifest: request.manifest.clone(),
            })?
            .manifest;

            match manifest.instructions {
                InstructionList::Parsed(instructions) => instructions,
                InstructionList::String(..) => panic!("Impossible case"),
            }
        };

        // Setting up the visitors and traversing the instructions
        let mut address_aggregator_visitor = AddressAggregatorVisitor::default();
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
            .collect::<Result<Vec<_>, _>>()
            .map_err(Self::Error::VisitorError)?;

        let response = Output {
            package_addresses: address_aggregator_visitor.package_addresses,
            resource_addresses: address_aggregator_visitor.resource_addresses,
            component_addresses: address_aggregator_visitor.component_addresses.clone(),
            account_addresses: address_aggregator_visitor
                .component_addresses
                .into_iter()
                .filter(|address| is_account(address))
                .collect(),
            accounts_requiring_auth: account_interactions_visitor.auth_required,
            accounts_withdrawn_from: account_interactions_visitor.accounts_withdrawn_from,
            accounts_deposited_into: account_interactions_visitor.accounts_deposited_into,
        };
        Ok(response)
    }

    fn post_process(_: &Input, response: Output) -> Result<Output, Error> {
        Ok(response)
    }
}

#[serializable]
#[serde(tag = "type")]
pub enum Error {
    /// An error emitted during the pre processing of the invocation
    PreProcessingError(VisitorError),

    /// An error emitted if the visitors fail during the handling of the invocation.
    VisitorError(VisitorError),

    /// An error emitted when the conversion of the manifest instructions to parsed instructions
    /// fails.
    ConvertManifestError(convert_manifest::Error),

    /// An error emitted when an address is encountered in the manifest with an invalid network id
    InvalidNetworkIdEncountered { expected: u8, found: u8 },
}

impl From<convert_manifest::Error> for Error {
    fn from(value: convert_manifest::Error) -> Self {
        Self::ConvertManifestError(value)
    }
}
