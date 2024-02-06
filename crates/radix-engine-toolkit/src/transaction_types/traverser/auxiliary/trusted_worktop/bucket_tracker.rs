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

use crate::transaction_types::types::ResourceSpecifierExt;
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

    pub fn try_consume_fungible_from_bucket(
        &mut self,
        bucket_id: &ManifestBucket,
        amount: &Decimal,
    ) -> Option<ResourceSpecifier> {
        if !self.untracked_mode {
            if let Some(resources) =
                self.buckets.get_mut(bucket_id).expect("Bucket not found")
            {
                // if operation is done on fungible resource then try to remove amount from specified bucket
                if resources.resource_address().is_fungible() {
                    if let Some(value) = resources
                        .amount()
                        .expect("Must succeed")
                        .checked_sub(*amount)
                    {
                        return Some(ResourceSpecifier::Amount(
                            resources.resource_address(),
                            value,
                        ));
                    }
                }
            }
        }
        None
    }

    pub fn try_consume_non_fungible_from_bucket(
        &mut self,
        bucket_id: &ManifestBucket,
        ids: &Vec<NonFungibleLocalId>,
    ) -> Option<ResourceSpecifier> {
        if !self.untracked_mode {
            if let Some(resources) =
                self.buckets.get_mut(bucket_id).expect("Bucket not found")
            {
                match resources {
                    ResourceSpecifier::Ids(address, bucket_ids) => {
                        if ids
                            .iter()
                            .filter(|item| bucket_ids.contains(*item))
                            .count()
                            == 0
                        {
                            // all ids are found in this bucket -> operation succeeded
                            return Some(ResourceSpecifier::Ids(
                                *address,
                                IndexSet::from_iter(ids.iter().cloned()),
                            ));
                        }
                    }
                    _ => (),
                }
            }
        }
        None
    }
}
