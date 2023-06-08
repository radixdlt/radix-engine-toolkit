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

use std::str::FromStr;

use native_json_library::models::non_fungible_global_id::{
    SerializableNonFungibleGlobalId, SerializableNonFungibleGlobalIdInternal,
};
use radix_engine_common::prelude::{NonFungibleLocalId, ACCOUNT_OWNER_BADGE};
use transaction::prelude::NonFungibleGlobalId;

use super::traits::HasExamples;

impl<'d> HasExamples<'d> for SerializableNonFungibleGlobalId {
    fn examples() -> Vec<Self> {
        vec![
            NonFungibleLocalId::string("Hello").unwrap(),
            NonFungibleLocalId::integer(1),
            NonFungibleLocalId::bytes(vec![100]).unwrap(),
            NonFungibleLocalId::from_str("{86cc8d24-194d-4393-85ee-91ee00000005}").unwrap(),
        ]
        .into_iter()
        .map(|local_id| NonFungibleGlobalId::new(ACCOUNT_OWNER_BADGE, local_id))
        .map(|global_id| {
            SerializableNonFungibleGlobalId(SerializableNonFungibleGlobalIdInternal {
                network_id: 0x01,
                non_fungible_global_id: global_id,
            })
        })
        .collect()
    }
}
