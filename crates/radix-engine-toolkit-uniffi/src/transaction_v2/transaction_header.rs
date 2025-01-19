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
pub struct TransactionHeaderV2 {
    pub notary_public_key: PublicKey,
    pub notary_is_signatory: bool,
    pub tip_basis_points: u32,
}

impl TryFrom<TransactionHeaderV2> for engine::TransactionHeaderV2 {
    type Error = RadixEngineToolkitError;

    fn try_from(value: TransactionHeaderV2) -> Result<Self> {
        Ok(Self {
            notary_public_key: value.notary_public_key.try_into()?,
            notary_is_signatory: value.notary_is_signatory,
            tip_basis_points: value.tip_basis_points,
        })
    }
}

impl From<engine::TransactionHeaderV2> for TransactionHeaderV2 {
    fn from(value: engine::TransactionHeaderV2) -> Self {
        Self {
            notary_public_key: value.notary_public_key.into(),
            notary_is_signatory: value.notary_is_signatory,
            tip_basis_points: value.tip_basis_points,
        }
    }
}
