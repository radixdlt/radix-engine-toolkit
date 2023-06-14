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

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use transaction::prelude::ManifestExpression;

use super::macros::serializable_string_wrapper;
use super::node_id::SerializableNodeId;

serializable_string_wrapper!(String, SerializableNamedAddress);
serializable_string_wrapper!(String, SerializableBucketId);
serializable_string_wrapper!(String, SerializableProofId);
serializable_string_wrapper!(String, SerializableAddressReservation);

#[serde_with::serde_as]
#[derive(Serialize, Deserialize, JsonSchema, Clone, Copy, Debug)]
pub enum SerializableExpression {
    EntireWorktop,
    EntireAuthZone,
}

impl From<ManifestExpression> for SerializableExpression {
    fn from(value: ManifestExpression) -> Self {
        match value {
            ManifestExpression::EntireAuthZone => Self::EntireAuthZone,
            ManifestExpression::EntireWorktop => Self::EntireWorktop,
        }
    }
}

impl From<SerializableExpression> for ManifestExpression {
    fn from(value: SerializableExpression) -> Self {
        match value {
            SerializableExpression::EntireAuthZone => Self::EntireAuthZone,
            SerializableExpression::EntireWorktop => Self::EntireWorktop,
        }
    }
}

#[serde_with::serde_as]
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
#[serde(tag = "kind")]
pub enum SerializableManifestAddress {
    Static { value: SerializableNodeId },
    Named { value: SerializableNamedAddress },
}
