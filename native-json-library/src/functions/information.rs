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

use super::macros::{export_function, export_jni_function};
use super::traits::Function;
use radix_engine_toolkit::functions::information::DependencyInformation;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema, Clone)]
pub struct BuildInformationInput {}

#[derive(Serialize, Deserialize, JsonSchema, Clone)]
pub struct BuildInformationOutput {
    pub version: String,
    pub scrypto_dependency: SerializableDependencyInformation,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", content = "value")]
pub enum SerializableDependencyInformation {
    Version(String),
    Tag(String),
    Branch(String),
    Rev(String),
}

pub struct BuildInformation;
impl<'a> Function<'a> for BuildInformation {
    type Input = BuildInformationInput;
    type Output = BuildInformationOutput;

    fn handle(_: Self::Input) -> Result<Self::Output, crate::error::InvocationHandlingError> {
        let build_information = radix_engine_toolkit::functions::information::information();
        Ok(BuildInformationOutput {
            version: build_information.version,
            scrypto_dependency: match build_information.scrypto_dependency {
                DependencyInformation::Branch(string) => {
                    SerializableDependencyInformation::Branch(string)
                }
                DependencyInformation::Tag(string) => {
                    SerializableDependencyInformation::Tag(string)
                }
                DependencyInformation::Version(string) => {
                    SerializableDependencyInformation::Version(string)
                }
                DependencyInformation::Rev(string) => {
                    SerializableDependencyInformation::Rev(string)
                }
            },
        })
    }
}

export_function!(BuildInformation as build_information);
export_jni_function!(BuildInformation as buildInformation);
