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

use crate::{
    models::transaction::instructions::SerializableInstructionsError, utils::debug_string,
};

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(tag = "kind", content = "error")]
pub enum Error {
    InvocationInterpretationError(InvocationInterpretationError),
    InvocationHandlingError(InvocationHandlingError),
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(tag = "kind", content = "error")]
pub enum InvocationInterpretationError {
    SerializationError(String),
    DeserializationError(String),
    Utf8Error(String),
    FailedToAllocateJniString(String),
    FailedToReadJniString(String),
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(tag = "kind", content = "error")]
pub enum InvocationHandlingError {
    DerivationError(String),
    SerializableInstructionsError(String),
    EncodeError(String, String),
    DecodeError(String, String),
}

impl From<InvocationHandlingError> for Error {
    fn from(value: InvocationHandlingError) -> Self {
        Self::InvocationHandlingError(value)
    }
}

impl From<InvocationInterpretationError> for Error {
    fn from(value: InvocationInterpretationError) -> Self {
        Self::InvocationInterpretationError(value)
    }
}

impl From<SerializableInstructionsError> for InvocationHandlingError {
    fn from(value: SerializableInstructionsError) -> Self {
        Self::SerializableInstructionsError(debug_string(value))
    }
}
