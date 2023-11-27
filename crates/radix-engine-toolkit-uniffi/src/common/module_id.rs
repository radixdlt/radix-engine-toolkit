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

#[derive(Clone, Debug, Enum)]
pub enum ModuleId {
    Main,
    Metadata,
    Royalty,
    RoleAssignment,
}

impl From<NativeObjectModuleId> for ModuleId {
    fn from(value: NativeObjectModuleId) -> Self {
        match value {
            NativeObjectModuleId::Main => Self::Main,
            NativeObjectModuleId::Metadata => Self::Metadata,
            NativeObjectModuleId::Royalty => Self::Royalty,
            NativeObjectModuleId::RoleAssignment => Self::RoleAssignment,
        }
    }
}

impl From<ModuleId> for NativeObjectModuleId {
    fn from(value: ModuleId) -> Self {
        match value {
            ModuleId::Main => Self::Main,
            ModuleId::Metadata => Self::Metadata,
            ModuleId::Royalty => Self::Royalty,
            ModuleId::RoleAssignment => Self::RoleAssignment,
        }
    }
}
