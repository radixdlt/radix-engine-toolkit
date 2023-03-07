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
use crate::model::address::{EntityAddress, NetworkAwarePackageAddress};
use crate::model::address::{NetworkAwareComponentAddress, NetworkAwareResourceAddress};
use crate::model::engine_identifier::RENodeId;
use crate::model::instruction::Instruction;
use crate::model::transaction::{InstructionKind, InstructionList, TransactionManifest};
use crate::visitor::{
    traverse_instruction, AccountDeposit, AccountDepositsInstructionVisitor,
    AccountInteractionsInstructionVisitor, AccountProofsInstructionVisitor, AccountWithdraw,
    AccountWithdrawsInstructionVisitor, AddressAggregatorVisitor, ValueNetworkAggregatorVisitor,
};
use scrypto::prelude::Decimal;
use toolkit_derive::serializable;

use super::traits::Handler;
use super::{ConvertManifestHandler, ConvertManifestRequest};

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
    pub resource_changes: Vec<Vec<ResourceChange>>,
}

/// The response of the [`AnalyzeManifestWithPreviewContextRequest`]
#[serializable]
pub struct AnalyzeManifestWithPreviewContextResponse {
    // TODO: Should we remove all native packages and components from this list?
    /// The set of addresses encountered in the manifest.
    ///
    /// This field is populated through static analysis of the manifest and captures the set of all
    /// addresses encountered in the manifest. This captures addresses if they're used in calls,
    /// used as arguments, or contained as parts of some list or array.
    pub encountered_addresses: EncounteredAddresses,

    /// A set of account component addresses which were involved in actions that require auth.
    ///
    /// This field is obtained through static analysis of the manifest by the Radix Engine Toolkit.
    /// When the toolkit encounters an instruction being performed on an account that requires auth
    /// (e.g., withdrawing funds, locking fee, creating proofs), it is added to this address set.
    ///
    /// It is then the job of the wallet to determine whether the account has been securified and
    /// uses an access controller or is still operating in signature mode and produce the correct
    /// auth based on that.
    #[schemars(with = "BTreeSet<EntityAddress>")]
    #[serde_as(as = "BTreeSet<serde_with::TryFromInto<EntityAddress>>")]
    pub accounts_requiring_auth: BTreeSet<NetworkAwareComponentAddress>,

    /// A set of the resource addresses of which proofs were created from accounts in this
    /// manifest.
    ///
    /// This field is populated through static analysis of the manifest instruction. This field
    /// captures the resource addresses of all of the proofs created from accounts throughout the
    /// manifest. This field does not capture the amount of the proof created nor which account the
    /// proof was created from.
    #[schemars(with = "BTreeSet<EntityAddress>")]
    #[serde_as(as = "BTreeSet<serde_with::TryFromInto<EntityAddress>>")]
    pub account_proof_resources: BTreeSet<NetworkAwareResourceAddress>,

    /// A list of the account withdraws seen in the manifest.
    ///
    /// This field is populated through static analysis of the manifest and it captures information
    /// relating to the resources withdrawn from accounts such as the component address of the
    /// account, the resource address of the withdrawn, and either an amount or set of non-fungible
    /// local ids of the withdrawn resources.
    pub account_withdraws: Vec<AccountWithdraw>,

    /// A list of the account deposits which occur in the transaction.
    ///
    /// This field is populated through both static analysis of the manifest and through the
    /// context provided by the transaction preview. All deposits referred to as "exact" are
    /// deposits which are guaranteed by the static analysis while the ones referred to as
    /// "estimate" are deposits which are primarily obtained from the context of the previews
    pub account_deposits: Vec<AccountDeposit>,
}

/// The set of addresses encountered in the manifest
#[serializable]
#[derive(PartialEq, PartialOrd, Eq, Ord)]
pub struct EncounteredAddresses {
    /// The set of component addresses encountered in the manifest
    #[schemars(with = "BTreeSet<EntityAddress>")]
    #[serde_as(as = "BTreeSet<serde_with::TryFromInto<EntityAddress>>")]
    pub component_addresses: BTreeSet<NetworkAwareComponentAddress>,

    /// The set of resource addresses encountered in the manifest
    #[schemars(with = "BTreeSet<EntityAddress>")]
    #[serde_as(as = "BTreeSet<serde_with::TryFromInto<EntityAddress>>")]
    pub resource_addresses: BTreeSet<NetworkAwareResourceAddress>,

    /// The set of package addresses encountered in the manifest
    #[schemars(with = "BTreeSet<EntityAddress>")]
    #[serde_as(as = "BTreeSet<serde_with::TryFromInto<EntityAddress>>")]
    pub package_addresses: BTreeSet<NetworkAwarePackageAddress>,
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
        // Getting the instructions in the passed manifest as Parsed instructions.
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

        // Setting up the visitors that will be used on the instructions
        let mut account_interactions_visitor = AccountInteractionsInstructionVisitor::default();
        let mut account_withdraws_visitor = AccountWithdrawsInstructionVisitor::default();
        let mut account_proofs_visitor = AccountProofsInstructionVisitor::default();
        let mut address_aggregator_visitor = AddressAggregatorVisitor::default();
        let mut account_deposits_visitor = AccountDepositsInstructionVisitor::new(
            request
                .resource_changes
                .iter()
                .enumerate()
                .filter_map(|(index, resource_changes)| {
                    if !resource_changes.is_empty() {
                        Some((index as u32, resource_changes.clone()))
                    } else {
                        None
                    }
                })
                .collect(),
        );
        instructions
            .iter_mut()
            .map(|instruction| {
                traverse_instruction(
                    instruction,
                    &mut [&mut address_aggregator_visitor],
                    &mut [
                        &mut account_interactions_visitor,
                        &mut account_withdraws_visitor,
                        &mut account_deposits_visitor,
                        &mut account_proofs_visitor,
                    ],
                )
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(AnalyzeManifestWithPreviewContextResponse {
            accounts_requiring_auth: account_interactions_visitor.auth_required,
            account_proof_resources: account_proofs_visitor.created_proofs,
            encountered_addresses: EncounteredAddresses {
                component_addresses: address_aggregator_visitor.component_addresses,
                resource_addresses: address_aggregator_visitor.resource_addresses,
                package_addresses: address_aggregator_visitor.package_addresses,
            },
            account_withdraws: account_withdraws_visitor.0,
            account_deposits: account_deposits_visitor.deposits,
        })
    }

    fn post_process(
        _: &AnalyzeManifestWithPreviewContextRequest,
        response: AnalyzeManifestWithPreviewContextResponse,
    ) -> Result<AnalyzeManifestWithPreviewContextResponse> {
        Ok(response)
    }
}

#[test]
pub fn x() {
    let request_string = r##"
    {
        "network_id": "11",
        "manifest": {
          "instructions": {
            "type": "String",
            "value": "CALL_METHOD ComponentAddress(\"account_tdx_b_1pp3eaya2hehlxqgmva6vutzec68cv7vuaye5rl9nqunsutnvhm\") \"lock_fee\" Decimal(\"10\");CALL_METHOD ComponentAddress(\"account_tdx_b_1pp3eaya2hehlxqgmva6vutzec68cv7vuaye5rl9nqunsutnvhm\") \"create_proof_by_amount\" ResourceAddress(\"resource_tdx_b_1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq8z96qp\") Decimal(\"250\");CALL_METHOD ComponentAddress(\"account_tdx_b_1pp3eaya2hehlxqgmva6vutzec68cv7vuaye5rl9nqunsutnvhm\") \"withdraw\" ResourceAddress(\"resource_tdx_b_1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq8z96qp\") Decimal(\"100\");TAKE_FROM_WORKTOP_BY_AMOUNT Decimal(\"100\") ResourceAddress(\"resource_tdx_b_1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq8z96qp\") Bucket(\"bucket1\");CALL_METHOD ComponentAddress(\"component_tdx_b_1qt7c7ws0a4f3wd3mwtcj4acvn87w4as9zyvkx3wwq8lskwe5zm\") \"swap\" Bucket(\"bucket1\") ResourceAddress(\"resource_tdx_b_1qre9sv98scqut4k9g3j6kxuvscczv0lzumefwgwhuf6qdu4c3r\");CALL_METHOD ComponentAddress(\"account_tdx_b_1pp3eaya2hehlxqgmva6vutzec68cv7vuaye5rl9nqunsutnvhm\") \"deposit_batch\" Expression(\"ENTIRE_WORKTOP\"); "
          },
          "blobs": []
        },
        "resource_changes": [
            [],
            [],
            [],
            [],
            [],
            [
          {
            "owner_id": {
              "type": "GlobalComponent",
              "address": "account_tdx_b_1pp3eaya2hehlxqgmva6vutzec68cv7vuaye5rl9nqunsutnvhm"
            },
            "resource_address": "resource_tdx_b_1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq8z96qp",
            "amount": "-100"
          },
          {
            "owner_id": {
              "type": "GlobalComponent",
              "address": "account_tdx_b_1pp3eaya2hehlxqgmva6vutzec68cv7vuaye5rl9nqunsutnvhm"
            },
            "resource_address": "resource_tdx_b_1qre9sv98scqut4k9g3j6kxuvscczv0lzumefwgwhuf6qdu4c3r",
            "amount": "0.760757908055004258"
          },
          {
            "owner_id": {
              "type": "GlobalComponent",
              "address": "component_tdx_b_1qt7c7ws0a4f3wd3mwtcj4acvn87w4as9zyvkx3wwq8lskwe5zm"
            },
            "resource_address": "resource_tdx_b_1qre9sv98scqut4k9g3j6kxuvscczv0lzumefwgwhuf6qdu4c3r",
            "amount": "-0.760757908055004258"
          },
          {
            "owner_id": {
              "type": "GlobalComponent",
              "address": "component_tdx_b_1qt7c7ws0a4f3wd3mwtcj4acvn87w4as9zyvkx3wwq8lskwe5zm"
            },
            "resource_address": "resource_tdx_b_1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq8z96qp",
            "amount": "0.255"
          },
          {
            "owner_id": {
              "type": "GlobalComponent",
              "address": "component_tdx_b_1qt7c7ws0a4f3wd3mwtcj4acvn87w4as9zyvkx3wwq8lskwe5zm"
            },
            "resource_address": "resource_tdx_b_1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq8z96qp",
            "amount": "0.045"
          },
          {
            "owner_id": {
              "type": "GlobalComponent",
              "address": "component_tdx_b_1qt7c7ws0a4f3wd3mwtcj4acvn87w4as9zyvkx3wwq8lskwe5zm"
            },
            "resource_address": "resource_tdx_b_1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq8z96qp",
            "amount": "99.7"
          }
          ]
        ]
      }
    "##;
    let request =
        serde_json::from_str::<AnalyzeManifestWithPreviewContextRequest>(request_string).unwrap();
    let response = AnalyzeManifestWithPreviewContextHandler::fulfill(request).unwrap();
    println!("{}", serde_json::to_string_pretty(&response).unwrap())
}
