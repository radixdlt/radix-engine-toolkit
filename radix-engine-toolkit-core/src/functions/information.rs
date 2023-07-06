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

pub fn information() -> BuildInformation {
    let version = env!("CARGO_PKG_VERSION").into();
    let scrypto_dependency = DependencyInformation::from_environment_variable();

    BuildInformation {
        version,
        scrypto_dependency,
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BuildInformation {
    pub version: String,
    pub scrypto_dependency: DependencyInformation,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DependencyInformation {
    // Crates.io
    Version(String),

    // Github
    Tag(String),
    Branch(String),
    Rev(String),
}

impl DependencyInformation {
    fn from_environment_variable() -> Self {
        let version = env!("SCRYPTO_DEPENDENCY");

        let mut splitted = version.split('=');
        let identifier = splitted.next().unwrap();
        let value = splitted.next().unwrap();

        match identifier {
            "version" => Self::Version(value.into()),
            "tag" => Self::Tag(value.into()),
            "branch" => Self::Branch(value.into()),
            "rev" => Self::Rev(value.into()),
            _ => panic!("Unknown identifier encountered: {}", identifier),
        }
    }
}
