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

use crate::internal_prelude::*;

pub fn entity_type(node_id: TypedNodeId) -> EntityType {
    node_id.entity_type()
}

pub fn decode(node_id: &str) -> Option<(u8, EntityType, String, [u8; 30])> {
    let network_definition = NetworkDefinition::from_address_string(node_id)?;
    let decoder = AddressBech32Decoder::new(&network_definition);
    let (hrp, _, _) =
        AddressBech32Decoder::validate_and_decode_ignore_hrp(node_id).ok()?;
    let (entity_type, data) = decoder.validate_and_decode(node_id).ok()?;
    data.try_into()
        .map(|data| (network_definition.id, entity_type, hrp, data))
        .ok()
}
