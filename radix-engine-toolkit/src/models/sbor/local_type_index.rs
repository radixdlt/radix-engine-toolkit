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
use sbor::{LocalTypeIndex, WellKnownTypeIndex};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[typeshare::typeshare]
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
#[serde(tag = "kind", content = "value")]
pub enum SerializableLocalTypeIndex {
    WellKnown(SerializableU8),
    SchemaLocalIndex(SerializableU64),
}

impl From<LocalTypeIndex> for SerializableLocalTypeIndex {
    fn from(value: LocalTypeIndex) -> Self {
        match value {
            LocalTypeIndex::SchemaLocalIndex(value) => {
                Self::SchemaLocalIndex((value as u64).into())
            }
            LocalTypeIndex::WellKnown(value) => Self::WellKnown((value.as_index() as u8).into()),
        }
    }
}

impl From<SerializableLocalTypeIndex> for LocalTypeIndex {
    fn from(value: SerializableLocalTypeIndex) -> Self {
        match value {
            SerializableLocalTypeIndex::SchemaLocalIndex(value) => {
                Self::SchemaLocalIndex(*value as usize)
            }
            SerializableLocalTypeIndex::WellKnown(value) => {
                Self::WellKnown(WellKnownTypeIndex::of(*value))
            }
        }
    }
}
