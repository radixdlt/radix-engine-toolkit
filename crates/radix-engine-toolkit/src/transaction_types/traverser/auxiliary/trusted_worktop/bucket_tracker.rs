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

use radix_engine::system::system_modules::execution_trace::ResourceSpecifier;
use scrypto::prelude::*;
use transaction::validation::ManifestIdAllocator;

#[derive(Default)]
pub struct BucketTracker {
    // Buckates tracking
    buckets: IndexMap<ManifestBucket, Option<ResourceSpecifier>>,
    // Buckets id generation
    id_allocator: ManifestIdAllocator,
    // Information if we are in 'untracked buckets' mode which is enabled
    // by use of buckets with unknown content triggered by the upper layer.
    untracked_mode: bool,
}

impl BucketTracker {
    pub fn is_untracked_mode(&self) -> bool {
        self.untracked_mode
    }

    pub fn enter_untracked_mode(&mut self) {
        self.untracked_mode = true;
    }

    pub fn new_bucket_known_resources(&mut self, resources: ResourceSpecifier) {
        if !self.untracked_mode {
            self.buckets
                .insert(self.id_allocator.new_bucket_id(), Some(resources));
        }
    }

    pub fn new_bucket_unknown_resources(&mut self) {
        if !self.untracked_mode {
            self.buckets.insert(self.id_allocator.new_bucket_id(), None);
        }
    }

    // returns consumed resources if found
    pub fn bucket_consumed(
        &mut self,
        bucket_id: &ManifestBucket,
    ) -> Option<Option<ResourceSpecifier>> {
        self.buckets.remove(bucket_id)
    }
}
