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

use sbor::representations::SerializationMode;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[typeshare::typeshare]
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
pub enum SerializableSerializationMode {
    Programmatic,
    Model,
    Natural,
}

impl From<SerializationMode> for SerializableSerializationMode {
    fn from(value: SerializationMode) -> Self {
        match value {
            SerializationMode::Model => Self::Model,
            SerializationMode::Natural => Self::Natural,
            SerializationMode::Programmatic => Self::Programmatic,
        }
    }
}

impl From<SerializableSerializationMode> for SerializationMode {
    fn from(value: SerializableSerializationMode) -> Self {
        match value {
            SerializableSerializationMode::Model => Self::Model,
            SerializableSerializationMode::Natural => Self::Natural,
            SerializableSerializationMode::Programmatic => Self::Programmatic,
        }
    }
}
