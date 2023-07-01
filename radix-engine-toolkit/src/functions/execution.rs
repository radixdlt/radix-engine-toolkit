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

use crate::prelude::*;

use radix_engine::system::system_modules::execution_trace::*;
use radix_engine::transaction::*;
use radix_engine_common::prelude::*;
use radix_engine_toolkit_core::functions::execution::*;
use radix_engine_toolkit_core::instruction_visitor::visitors::transaction_type::transfer_visitor::*;
use radix_engine_toolkit_core::instruction_visitor::visitors::transaction_type::general_transaction_visitor::*;
use schemars::*;
use scrypto::api::node_modules::metadata::*;
use serde::*;

//===================
// Execution Analyze
//===================

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
pub struct ExecutionAnalyzeInput {
    pub instructions: SerializableInstructions,
    pub network_id: SerializableU8,
    pub preview_receipt: SerializableBytes,
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
pub struct ExecutionAnalyzeOutput {
    pub fee_locks: SerializableFeeLocks,
    pub fee_summary: SerializableFeeSummary,
    pub transaction_type: SerializableTransactionType,
}

pub struct ExecutionAnalyze;
impl<'f> Function<'f> for ExecutionAnalyze {
    type Input = ExecutionAnalyzeInput;
    type Output = ExecutionAnalyzeOutput;

    fn handle(
        ExecutionAnalyzeInput {
            instructions,
            network_id,
            preview_receipt,
        }: Self::Input,
    ) -> Result<Self::Output, crate::error::InvocationHandlingError> {
        let instructions = instructions.to_instructions(*network_id)?;
        let receipt = scrypto_decode::<TransactionReceipt>(&preview_receipt).map_err(|error| {
            InvocationHandlingError::DecodeError(debug_string(error), debug_string(preview_receipt))
        })?;

        let execution_analysis =
            radix_engine_toolkit_core::functions::execution::analyze(&instructions, &receipt)
                .map_err(|error| {
                    InvocationHandlingError::InstructionVisitorError(debug_string(error))
                })?;

        let transaction_type = match execution_analysis.transaction_type {
            TransactionType::NonConforming => SerializableTransactionType::NonConforming,
            TransactionType::SimpleTransfer(simple_transfer) => {
                SerializableTransactionType::SimpleTransfer(Box::new(
                    SerializableSimpleTransferTransactionType {
                        from: SerializableNodeId::new(
                            simple_transfer.from.into_node_id(),
                            *network_id,
                        ),
                        to: SerializableNodeId::new(simple_transfer.to.into_node_id(), *network_id),
                        transferred: SerializableResourceSpecifier::new(
                            simple_transfer.transferred,
                            *network_id,
                        ),
                    },
                ))
            }
            TransactionType::Transfer(transfer) => SerializableTransactionType::Transfer(Box::new(
                SerializableTransferTransactionType {
                    from: SerializableNodeId::new(transfer.from.into_node_id(), *network_id),
                    transfers: transfer
                        .transfers
                        .into_iter()
                        .map(|(key, value)| {
                            (
                                SerializableNodeId::new(key.into_node_id(), *network_id),
                                value
                                    .into_iter()
                                    .map(|(key, value)| {
                                        (
                                            SerializableNodeId::new(
                                                key.into_node_id(),
                                                *network_id,
                                            ),
                                            value.into(),
                                        )
                                    })
                                    .collect(),
                            )
                        })
                        .collect(),
                },
            )),
            TransactionType::GeneralTransaction(general_transaction) => {
                SerializableTransactionType::GeneralTransaction(Box::new(
                    SerializableGeneralTransactionType {
                        account_proofs: general_transaction
                            .account_proofs
                            .into_iter()
                            .map(|address| {
                                SerializableNodeId::new(address.into_node_id(), *network_id)
                            })
                            .collect(),
                        account_withdraws: general_transaction
                            .account_withdraws
                            .into_iter()
                            .map(|(key, value)| {
                                (
                                    SerializableNodeId::new(key.into_node_id(), *network_id),
                                    value
                                        .into_iter()
                                        .map(|value| {
                                            SerializableResourceSpecifier::new(value, *network_id)
                                        })
                                        .collect(),
                                )
                            })
                            .collect(),
                        account_deposits: general_transaction
                            .account_deposits
                            .into_iter()
                            .map(|(key, value)| {
                                (
                                    SerializableNodeId::new(key.into_node_id(), *network_id),
                                    value
                                        .into_iter()
                                        .map(|value| match value {
                                            Source::Guaranteed(value) => {
                                                SerializableSource::Guaranteed {
                                                    value: SerializableResourceSpecifier::new(
                                                        value,
                                                        *network_id,
                                                    ),
                                                }
                                            }
                                            Source::Predicted(instruction_index, value) => {
                                                SerializableSource::Predicted {
                                                    instruction_index: (instruction_index as u64)
                                                        .into(),
                                                    value: SerializableResourceSpecifier::new(
                                                        value,
                                                        *network_id,
                                                    ),
                                                }
                                            }
                                        })
                                        .collect(),
                                )
                            })
                            .collect(),
                        addresses_in_manifest: InstructionsExtractAddressesOutput {
                            addresses: transform_addresses_set_to_map(
                                general_transaction.addresses_in_manifest.0,
                                *network_id,
                            ),
                            named_addresses: array_into!(
                                general_transaction.addresses_in_manifest.1
                            ),
                        },
                        metadata_of_newly_created_entities: general_transaction
                            .metadata_of_newly_created_entities
                            .into_iter()
                            .map(|(key, value)| {
                                (
                                    SerializableNodeId::new(key.into_node_id(), *network_id),
                                    value
                                        .into_iter()
                                        .map(|(key, value)| {
                                            (
                                                key,
                                                SerializableMetadataValue::new(value, *network_id),
                                            )
                                        })
                                        .collect(),
                                )
                            })
                            .collect(),
                        data_of_newly_minted_non_fungibles: general_transaction
                            .data_of_newly_minted_non_fungibles
                            .into_iter()
                            .map(|(key, value)| {
                                (
                                    SerializableNodeId::new(key.into_node_id(), *network_id),
                                    value
                                        .into_iter()
                                        .map(|(key, value)| {
                                            (key.into(), scrypto_encode(&value).unwrap().into())
                                        })
                                        .collect(),
                                )
                            })
                            .collect(),
                    },
                ))
            }
        };

        let fee_summary = execution_analysis.fee_summary.into();
        let fee_locks = execution_analysis.fee_locks.into();

        Ok(Self::Output {
            fee_locks,
            fee_summary,
            transaction_type,
        })
    }
}

export_function!(ExecutionAnalyze as execution_analyze);
export_jni_function!(ExecutionAnalyze as executionAnalyze);

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
#[serde(tag = "kind", content = "value")]
pub enum SerializableTransactionType {
    SimpleTransfer(Box<SerializableSimpleTransferTransactionType>),
    Transfer(Box<SerializableTransferTransactionType>),
    GeneralTransaction(Box<SerializableGeneralTransactionType>),
    NonConforming,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct SerializableFeeSummary {
    pub network_fee: SerializableDecimal,
    pub royalty_fee: SerializableDecimal,
}

impl From<FeeSummary> for SerializableFeeSummary {
    fn from(value: FeeSummary) -> Self {
        Self {
            network_fee: value.network_fee.into(),
            royalty_fee: value.royalty_fee.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct SerializableFeeLocks {
    pub lock: SerializableDecimal,
    pub contingent_lock: SerializableDecimal,
}

impl From<radix_engine_toolkit_core::functions::execution::FeeLocks> for SerializableFeeLocks {
    fn from(value: radix_engine_toolkit_core::functions::execution::FeeLocks) -> Self {
        Self {
            lock: value.lock.into(),
            contingent_lock: value.contingent_lock.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind")]
pub enum SerializableResourceSpecifier {
    Amount {
        resource_address: SerializableNodeId,
        amount: SerializableDecimal,
    },
    Ids {
        resource_address: SerializableNodeId,
        ids: HashSet<SerializableNonFungibleLocalId>,
    },
}

impl SerializableResourceSpecifier {
    pub fn new(resource_specifier: ResourceSpecifier, network_id: u8) -> Self {
        match resource_specifier {
            ResourceSpecifier::Amount(resource_address, amount) => Self::Amount {
                resource_address: SerializableNodeId::new(
                    resource_address.into_node_id(),
                    network_id,
                ),
                amount: amount.into(),
            },
            ResourceSpecifier::Ids(resource_address, ids) => Self::Ids {
                resource_address: SerializableNodeId::new(
                    resource_address.into_node_id(),
                    network_id,
                ),
                ids: ids.into_iter().map(Into::into).collect(),
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", content = "value")]
pub enum SerializableResources {
    Amount(SerializableDecimal),
    Ids(HashSet<SerializableNonFungibleLocalId>),
}

impl From<Resources> for SerializableResources {
    fn from(value: Resources) -> Self {
        match value {
            Resources::Amount(amount) => Self::Amount(amount.into()),
            Resources::Ids(ids) => Self::Ids(ids.into_iter().map(Into::into).collect()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct SerializableSimpleTransferTransactionType {
    pub from: SerializableNodeId,
    pub to: SerializableNodeId,
    pub transferred: SerializableResourceSpecifier,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct SerializableTransferTransactionType {
    pub from: SerializableNodeId,
    pub transfers: HashMap<SerializableNodeId, HashMap<SerializableNodeId, SerializableResources>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct SerializableGeneralTransactionType {
    pub account_proofs: HashSet<SerializableNodeId>,
    pub account_withdraws: HashMap<SerializableNodeId, Vec<SerializableResourceSpecifier>>,
    pub account_deposits:
        HashMap<SerializableNodeId, Vec<SerializableSource<SerializableResourceSpecifier>>>,
    pub addresses_in_manifest: InstructionsExtractAddressesOutput,
    pub metadata_of_newly_created_entities:
        HashMap<SerializableNodeId, HashMap<String, SerializableMetadataValue>>,
    pub data_of_newly_minted_non_fungibles:
        HashMap<SerializableNodeId, HashMap<SerializableNonFungibleLocalId, SerializableBytes>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", content = "value")]
pub enum SerializableMetadataValue {
    String(String),
    Bool(bool),
    U8(SerializableU8),
    U32(SerializableU32),
    U64(SerializableU64),
    I32(SerializableI32),
    I64(SerializableI64),
    Decimal(SerializableDecimal),
    GlobalAddress(SerializableNodeId),
    PublicKey(SerializablePublicKey),
    NonFungibleGlobalId(SerializableNonFungibleGlobalId),
    NonFungibleLocalId(SerializableNonFungibleLocalId),
    Instant(SerializableI64),
    Url(String),
    Origin(String),
    PublicKeyHash(SerializablePublicKeyHash),

    StringArray(Vec<String>),
    BoolArray(Vec<bool>),
    U8Array(Vec<u8>),
    U32Array(Vec<u32>),
    U64Array(Vec<u64>),
    I32Array(Vec<i32>),
    I64Array(Vec<i64>),
    DecimalArray(Vec<SerializableDecimal>),
    GlobalAddressArray(Vec<SerializableNodeId>),
    PublicKeyArray(Vec<SerializablePublicKey>),
    NonFungibleGlobalIdArray(Vec<SerializableNonFungibleGlobalId>),
    NonFungibleLocalIdArray(Vec<SerializableNonFungibleLocalId>),
    InstantArray(Vec<SerializableI64>),
    UrlArray(Vec<String>),
    OriginArray(Vec<String>),
    PublicKeyHashArray(Vec<SerializablePublicKeyHash>),
}

impl SerializableMetadataValue {
    pub fn new(metadata: MetadataValue, network_id: u8) -> Self {
        match metadata {
            MetadataValue::String(value) => SerializableMetadataValue::String(value),
            MetadataValue::Bool(value) => SerializableMetadataValue::Bool(value),
            MetadataValue::U8(value) => SerializableMetadataValue::U8(value.into()),
            MetadataValue::U32(value) => SerializableMetadataValue::U32(value.into()),
            MetadataValue::U64(value) => SerializableMetadataValue::U64(value.into()),
            MetadataValue::I32(value) => SerializableMetadataValue::I32(value.into()),
            MetadataValue::I64(value) => SerializableMetadataValue::I64(value.into()),
            MetadataValue::Decimal(value) => SerializableMetadataValue::Decimal(value.into()),
            MetadataValue::GlobalAddress(value) => SerializableMetadataValue::GlobalAddress(
                SerializableNodeId::new(value.into_node_id(), network_id),
            ),
            MetadataValue::PublicKey(value) => SerializableMetadataValue::PublicKey(value.into()),
            MetadataValue::NonFungibleGlobalId(value) => {
                SerializableMetadataValue::NonFungibleGlobalId(
                    SerializableNonFungibleGlobalId::new(value, network_id),
                )
            }
            MetadataValue::NonFungibleLocalId(value) => {
                SerializableMetadataValue::NonFungibleLocalId(value.into())
            }
            MetadataValue::Instant(value) => {
                SerializableMetadataValue::Instant(value.seconds_since_unix_epoch.into())
            }
            MetadataValue::Url(value) => SerializableMetadataValue::Url(value.0),
            MetadataValue::Origin(value) => SerializableMetadataValue::Origin(value.0),
            MetadataValue::PublicKeyHash(value) => {
                SerializableMetadataValue::PublicKeyHash(value.into())
            }

            MetadataValue::StringArray(value) => SerializableMetadataValue::StringArray(value),
            MetadataValue::BoolArray(value) => SerializableMetadataValue::BoolArray(value),
            MetadataValue::U8Array(value) => SerializableMetadataValue::U8Array(array_into!(value)),
            MetadataValue::U32Array(value) => {
                SerializableMetadataValue::U32Array(array_into!(value))
            }
            MetadataValue::U64Array(value) => {
                SerializableMetadataValue::U64Array(array_into!(value))
            }
            MetadataValue::I32Array(value) => {
                SerializableMetadataValue::I32Array(array_into!(value))
            }
            MetadataValue::I64Array(value) => {
                SerializableMetadataValue::I64Array(array_into!(value))
            }
            MetadataValue::DecimalArray(value) => {
                SerializableMetadataValue::DecimalArray(array_into!(value))
            }
            MetadataValue::GlobalAddressArray(value) => {
                SerializableMetadataValue::GlobalAddressArray(
                    value
                        .into_iter()
                        .map(|address| SerializableNodeId::new(address.into_node_id(), network_id))
                        .collect(),
                )
            }
            MetadataValue::PublicKeyArray(value) => {
                SerializableMetadataValue::PublicKeyArray(array_into!(value))
            }
            MetadataValue::NonFungibleGlobalIdArray(value) => {
                SerializableMetadataValue::NonFungibleGlobalIdArray(
                    value
                        .into_iter()
                        .map(|id| SerializableNonFungibleGlobalId::new(id, network_id))
                        .collect(),
                )
            }
            MetadataValue::NonFungibleLocalIdArray(value) => {
                SerializableMetadataValue::NonFungibleLocalIdArray(array_into!(value))
            }
            MetadataValue::InstantArray(value) => SerializableMetadataValue::InstantArray(
                value
                    .into_iter()
                    .map(|id| id.seconds_since_unix_epoch.into())
                    .collect(),
            ),
            MetadataValue::UrlArray(value) => SerializableMetadataValue::UrlArray(
                value.into_iter().map(|value| value.0).collect(),
            ),
            MetadataValue::OriginArray(value) => SerializableMetadataValue::OriginArray(
                value.into_iter().map(|value| value.0).collect(),
            ),
            MetadataValue::PublicKeyHashArray(value) => {
                SerializableMetadataValue::PublicKeyHashArray(array_into!(value))
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind")]
pub enum SerializableSource<T> {
    Guaranteed {
        value: T,
    },
    Predicted {
        value: T,
        instruction_index: SerializableU64,
    },
}

macro_rules! array_into {
    ($array: expr) => {
        $array.into_iter().map(Into::into).collect()
    };
}
use array_into;
