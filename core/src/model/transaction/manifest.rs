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

use crate::{InstructionList, ValueRef};
use serializable::serializable;

// =================
// Model Definition
// =================

/// A transaction intent consisting of instructions as well as blobs
#[serializable]
pub struct TransactionManifest {
    /// The transaction manifest instructions to be executed in the transaction.
    pub instructions: InstructionList,

    /// An array of byte arrays which is serialized as an array of hex strings which represents the
    /// blobs included in the transaction.
    #[schemars(with = "Vec<String>")]
    #[serde_as(as = "Vec<serde_with::hex::Hex>")]
    pub blobs: Vec<Vec<u8>>,
}

// ===============
// Implementation
// ===============

impl ValueRef for TransactionManifest {
    fn borrow_values(&self) -> Vec<&crate::Value> {
        self.instructions.borrow_values()
    }

    fn borrow_values_mut(&mut self) -> Vec<&mut crate::Value> {
        self.instructions.borrow_values_mut()
    }
}
