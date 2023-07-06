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

#[uniffi::export]
pub fn build_information() -> BuildInformation {
    core_information().into()
}

#[derive(Clone, Record)]
pub struct BuildInformation {
    pub version: String,
    pub scrypto_dependency: DependencyInformation,
}

#[derive(Clone, Debug, PartialEq, Eq, Enum)]
pub enum DependencyInformation {
    Version { value: String },
    Tag { value: String },
    Branch { value: String },
    Rev { value: String },
}

impl From<CoreBuildInformation> for BuildInformation {
    fn from(value: CoreBuildInformation) -> Self {
        Self {
            version: value.version,
            scrypto_dependency: match value.scrypto_dependency {
                CoreDependencyInformation::Branch(value) => DependencyInformation::Branch { value },
                CoreDependencyInformation::Tag(value) => DependencyInformation::Tag { value },
                CoreDependencyInformation::Rev(value) => DependencyInformation::Rev { value },
                CoreDependencyInformation::Version(value) => {
                    DependencyInformation::Version { value }
                }
            },
        }
    }
}
