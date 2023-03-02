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

use native_transaction::model as native;
use toolkit_derive::serializable;

/// A transaction header containing metadata and other transaction information.
#[serializable]
#[schemars(
    example = "crate::example::transaction::header::header1",
    example = "crate::example::transaction::header::header2"
)]
pub struct TransactionHeader {
    /// An 8 bit unsigned integer serialized as a string which represents the transaction version.
    /// Currently, this value is always 1.
    #[schemars(with = "String")]
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub version: u8,

    /// An 8 bit unsigned integer serialized as a string which represents the id of the network
    /// that this transaction is meant for.
    #[schemars(with = "String")]
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub network_id: u8,

    /// A 64 bit unsigned integer serialized as a string which represents the start of the epoch
    /// window in which this transaction executes. This value is inclusive.
    #[schemars(with = "String")]
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub start_epoch_inclusive: u64,

    /// A 64 bit unsigned integer serialized as a string which represents the end of the epoch
    /// window in which this transaction executes. This value is exclusive.
    #[schemars(with = "String")]
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub end_epoch_exclusive: u64,

    /// A 64 bit unsigned integer serialized as a string which represents a random nonce used for
    /// this transaction.
    #[schemars(with = "String")]
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub nonce: u64,

    /// The public key of the entity that will be notarizing this transaction.
    #[schemars(with = "crate::model::crypto::PublicKey")]
    #[serde_as(as = "serde_with::FromInto<crate::model::crypto::PublicKey>")]
    pub notary_public_key: scrypto::prelude::PublicKey,

    /// When `true` the notary's signature is also treated as an intent signature and therefore a
    /// virtual badge of the signature is added to the auth zone when the transaction auth zone at
    /// the beginning of the transaction.
    pub notary_as_signatory: bool,

    /// A 32 bit unsigned integer serialized as a string which represents the limit or maximum
    /// amount of cost units that the transaction is allowed to use.
    #[schemars(with = "String")]
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub cost_unit_limit: u32,

    /// A 16 bit unsigned integer serialized as a string which represents the percentage of tips
    /// given to validators for this transaction.
    #[schemars(with = "String")]
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub tip_percentage: u16,
}

// ============
// Conversions
// ============

impl From<native::TransactionHeader> for TransactionHeader {
    fn from(value: native::TransactionHeader) -> Self {
        Self {
            version: value.version,
            network_id: value.network_id,
            start_epoch_inclusive: value.start_epoch_inclusive,
            end_epoch_exclusive: value.end_epoch_exclusive,
            nonce: value.nonce,
            notary_public_key: value.notary_public_key,
            notary_as_signatory: value.notary_as_signatory,
            cost_unit_limit: value.cost_unit_limit,
            tip_percentage: value.tip_percentage,
        }
    }
}

impl From<TransactionHeader> for native::TransactionHeader {
    fn from(value: TransactionHeader) -> Self {
        Self {
            version: value.version,
            network_id: value.network_id,
            start_epoch_inclusive: value.start_epoch_inclusive,
            end_epoch_exclusive: value.end_epoch_exclusive,
            nonce: value.nonce,
            notary_public_key: value.notary_public_key,
            notary_as_signatory: value.notary_as_signatory,
            cost_unit_limit: value.cost_unit_limit,
            tip_percentage: value.tip_percentage,
        }
    }
}
