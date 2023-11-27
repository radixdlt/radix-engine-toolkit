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
use scrypto::prelude::*;

use schemars::*;
use serde::*;

#[typeshare::typeshare]
pub type AddressEntityTypeInput = SerializableNodeId;

#[typeshare::typeshare]
pub type AddressEntityTypeOutput = SerializableEntityType;

pub struct AddressEntityType;
impl<'f> Function<'f> for AddressEntityType {
    type Input = AddressEntityTypeInput;
    type Output = AddressEntityTypeOutput;

    fn handle(
        input: Self::Input,
    ) -> Result<Self::Output, crate::error::InvocationHandlingError> {
        let node_id = input.0.node_id;
        let entity_type = node_id.entity_type().ok_or(
            InvocationHandlingError::InvalidAddress(input.0.to_string()),
        )?;
        Ok(entity_type.into())
    }
}

export_function!(AddressEntityType as address_entity_type);
export_jni_function!(AddressEntityType as addressEntityType);

#[typeshare::typeshare]
pub type AddressDecodeInput = String;

#[typeshare::typeshare]
#[derive(
    Serialize,
    Deserialize,
    JsonSchema,
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub struct AddressDecodeOutput {
    pub network_id: SerializableU8,
    pub entity_type: SerializableEntityType,
    pub hrp: String,
    pub data: SerializableBytes,
}

pub struct AddressDecode;
impl<'f> Function<'f> for AddressDecode {
    type Input = AddressDecodeInput;
    type Output = AddressDecodeOutput;

    fn handle(
        input: Self::Input,
    ) -> Result<Self::Output, crate::error::InvocationHandlingError> {
        let (network_id, entity_type, hrp, data) =
            radix_engine_toolkit_core::functions::address::decode(&input)
                .ok_or(InvocationHandlingError::InvalidAddress(input))?;

        Ok(Self::Output {
            network_id: network_id.into(),
            entity_type: entity_type.into(),
            hrp,
            data: data.to_vec().into(),
        })
    }
}

export_function!(AddressDecode as address_decode);
export_jni_function!(AddressDecode as addressDecode);
