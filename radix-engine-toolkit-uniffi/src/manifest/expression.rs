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

#[derive(Clone, Debug, Enum, Copy)]
pub enum ManifestExpression {
    EntireWorktop,
    EntireAuthZone,
}

impl From<NativeManifestExpression> for ManifestExpression {
    fn from(value: NativeManifestExpression) -> Self {
        match value {
            NativeManifestExpression::EntireAuthZone => Self::EntireAuthZone,
            NativeManifestExpression::EntireWorktop => Self::EntireWorktop,
        }
    }
}

impl From<ManifestExpression> for NativeManifestExpression {
    fn from(value: ManifestExpression) -> Self {
        match value {
            ManifestExpression::EntireAuthZone => Self::EntireAuthZone,
            ManifestExpression::EntireWorktop => Self::EntireWorktop,
        }
    }
}
