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

use radix_transactions::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[typeshare::typeshare]
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
pub struct SerializablePartialTransactionV2 {
    pub root_subintent: SerializableSubintentV2,
    pub non_root_subintents: Vec<SerializableSubintentV2>,
}

impl FromNative for SerializablePartialTransactionV2 {
    type Native = PartialTransactionV2;
    type Error = SerializableInstructionsError;
    type Context = ();

    fn to_native(&self, network_id: u8) -> Result<Self::Native, Self::Error> {
        let root_subintent = self.root_subintent.to_native(network_id)?;
        let non_root_subintents = self
            .non_root_subintents
            .iter()
            .map(|s| s.to_native(network_id))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(PartialTransactionV2 {
            root_subintent,
            non_root_subintents: NonRootSubintentsV2(non_root_subintents),
        })
    }

    fn from_native(
        native: &Self::Native,
        network_id: u8,
        _context: Self::Context,
    ) -> Result<Self, Self::Error> {
        let root_subintent = SerializableSubintentV2::from_native(
            &native.root_subintent,
            network_id,
            (),
        )?;
        let non_root_subintents = native
            .non_root_subintents
            .0
            .iter()
            .map(|s| SerializableSubintentV2::from_native(s, network_id, ()))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            root_subintent,
            non_root_subintents,
        })
    }
}
