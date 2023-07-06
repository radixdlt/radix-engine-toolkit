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

#[derive(Clone, Debug, Record)]
pub struct TransactionHeader {
    pub network_id: u8,
    pub start_epoch_inclusive: u64,
    pub end_epoch_exclusive: u64,
    pub nonce: u32,
    pub notary_public_key: PublicKey,
    pub notary_is_signatory: bool,
    pub tip_percentage: u16,
}

impl TryFrom<TransactionHeader> for NativeTransactionHeader {
    type Error = RadixEngineToolkitError;

    fn try_from(value: TransactionHeader) -> Result<Self> {
        Ok(Self {
            network_id: value.network_id,
            start_epoch_inclusive: NativeEpoch::of(value.start_epoch_inclusive),
            end_epoch_exclusive: NativeEpoch::of(value.end_epoch_exclusive),
            nonce: value.nonce,
            notary_public_key: value.notary_public_key.try_into()?,
            notary_is_signatory: value.notary_is_signatory,
            tip_percentage: value.tip_percentage,
        })
    }
}

impl From<NativeTransactionHeader> for TransactionHeader {
    fn from(value: NativeTransactionHeader) -> Self {
        Self {
            network_id: value.network_id,
            start_epoch_inclusive: value.start_epoch_inclusive.number(),
            end_epoch_exclusive: value.end_epoch_exclusive.number(),
            nonce: value.nonce,
            notary_public_key: value.notary_public_key.into(),
            notary_is_signatory: value.notary_is_signatory,
            tip_percentage: value.tip_percentage,
        }
    }
}
