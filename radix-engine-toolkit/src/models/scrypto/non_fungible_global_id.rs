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

use radix_engine_common::prelude::{AddressBech32Decoder, AddressBech32Encoder};
use radix_engine_toolkit_core::utils::network_definition_from_network_id;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::fmt::{Debug, Display};
use std::str::FromStr;
use transaction::prelude::NonFungibleGlobalId;

#[serde_as]
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
#[serde(transparent)]
#[schemars(transparent)]
#[typeshare::typeshare(serialized_as = "String")]
pub struct SerializableNonFungibleGlobalId(
    #[schemars(with = "String")]
    #[typeshare(serialized_as = "String")]
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub SerializableNonFungibleGlobalIdInternal,
);

impl SerializableNonFungibleGlobalId {
    pub fn new(id: NonFungibleGlobalId, network_id: u8) -> Self {
        Self(SerializableNonFungibleGlobalIdInternal {
            non_fungible_global_id: id,
            network_id,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SerializableNonFungibleGlobalIdInternal {
    pub non_fungible_global_id: NonFungibleGlobalId,
    pub network_id: u8,
}

impl Display for SerializableNonFungibleGlobalIdInternal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let network_definition = network_definition_from_network_id(self.network_id);
        let bech32_encoder = AddressBech32Encoder::new(&network_definition);
        write!(
            f,
            "{}",
            self.non_fungible_global_id
                .to_canonical_string(&bech32_encoder)
        )
    }
}

impl FromStr for SerializableNonFungibleGlobalIdInternal {
    type Err = SerializableNonFungibleGlobalIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let resource_address_string =
            s.split(':')
                .next()
                .ok_or(SerializableNonFungibleGlobalIdError::NoSeparatorFound(
                    s.to_owned(),
                ))?;

        let network_id = radix_engine_toolkit_core::utils::network_id_from_address_string(
            resource_address_string,
        )
        .ok_or(SerializableNonFungibleGlobalIdError::InvalidResourceAddress)?;
        let network_definition = network_definition_from_network_id(network_id);
        let bech32_decoder = AddressBech32Decoder::new(&network_definition);

        let non_fungible_global_id =
            NonFungibleGlobalId::try_from_canonical_string(&bech32_decoder, s).map_err(
                |error| SerializableNonFungibleGlobalIdError::ParsingError(debug_string(error)),
            )?;

        Ok(SerializableNonFungibleGlobalIdInternal {
            non_fungible_global_id,
            network_id,
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(tag = "kind", content = "error")]
pub enum SerializableNonFungibleGlobalIdError {
    NoSeparatorFound(String),
    InvalidResourceAddress,
    ParsingError(String),
}

impl Display for SerializableNonFungibleGlobalIdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self, f)
    }
}
