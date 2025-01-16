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

use scrypto::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TestAddressAllocator(usize);

impl TestAddressAllocator {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn new_account_address(&mut self) -> ComponentAddress {
        self.next_address(EntityType::GlobalPreallocatedEd25519Account)
    }

    pub fn next_address<T: TryFrom<NodeId, Error: Debug>>(
        &mut self,
        entity_type: EntityType,
    ) -> T {
        T::try_from(self.new_node_id(entity_type)).unwrap()
    }

    pub fn new_node_id(&mut self, entity_type: EntityType) -> NodeId {
        let hash = self.new_hash();
        let mut bytes = hash.lower_bytes();
        bytes[0] = entity_type as u8;
        NodeId(bytes)
    }

    pub fn new_hash(&mut self) -> Hash {
        hash(scrypto_encode(&self.next_id().to_le_bytes()).unwrap())
    }

    pub fn next_id(&mut self) -> usize {
        let id = self.0;
        self.0 += 1;
        id
    }
}
