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

#[typeshare::typeshare]
pub type AddressEntityTypeInput = SerializableNodeId;

#[typeshare::typeshare]
pub type AddressEntityTypeOutput = SerializableEntityType;

pub struct AddressEntityType;
impl<'f> Function<'f> for AddressEntityType {
    type Input = AddressEntityTypeInput;
    type Output = AddressEntityTypeOutput;

    fn handle(input: Self::Input) -> Result<Self::Output, crate::error::InvocationHandlingError> {
        let node_id = input.0.node_id;
        let entity_type = radix_engine_toolkit_core::functions::address::entity_type(node_id)
            .ok_or(InvocationHandlingError::InvalidAddress(input.0.to_string()))?;
        Ok(entity_type.into())
    }
}

export_function!(AddressEntityType as address_entity_type);
export_jni_function!(AddressEntityType as addressEntityType);
