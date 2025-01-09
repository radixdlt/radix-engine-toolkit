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

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Sbor)]
#[sbor(transparent)]
pub struct InstructionIndex(usize);

impl InstructionIndex {
    pub const fn of(index: usize) -> Self {
        Self(index)
    }

    pub const fn value(&self) -> &usize {
        &self.0
    }

    pub const fn add(&self, instructions: usize) -> Option<Self> {
        match self.0.checked_add(instructions) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }
}

impl From<usize> for InstructionIndex {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl From<InstructionIndex> for usize {
    fn from(value: InstructionIndex) -> Self {
        value.0
    }
}
