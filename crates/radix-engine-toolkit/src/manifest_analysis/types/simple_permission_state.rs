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

use crate::internal_prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SimplePermissionState(bool);

impl Default for SimplePermissionState {
    fn default() -> Self {
        Self(true)
    }
}

impl ManifestAnalyzerPermissionState for SimplePermissionState {
    fn all_instructions_permitted(&self) -> bool {
        self.0
    }
}

impl SimplePermissionState {
    pub fn new(value: bool) -> Self {
        Self(value)
    }

    pub fn next_instruction_is_not_permitted(&mut self) {
        self.next_instruction_status(false)
    }

    pub fn next_instruction_is_permitted(&mut self) {
        self.next_instruction_status(true)
    }

    pub fn next_instruction_status(&mut self, is_permitted: bool) {
        self.0 &= is_permitted
    }
}
