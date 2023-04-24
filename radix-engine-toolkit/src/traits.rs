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

use std::fmt::Debug;

use crate::{model::transaction::InstructionKind, utils::debug_string};
use sbor::{DecodeError, EncodeError};
use scrypto::prelude::{hash, Hash};

/// A trait that defines the common interface of all compile-able intents
pub trait CompilableIntent {
    type Error: Debug;

    fn compile(&self) -> Result<Vec<u8>, Self::Error>;

    fn decompile<T>(data: &T, instructions_kind: InstructionKind) -> Result<Self, Self::Error>
    where
        Self: Sized,
        T: AsRef<[u8]>;

    fn hash(&self) -> Result<Hash, Self::Error> {
        self.compile().map(hash)
    }
}
