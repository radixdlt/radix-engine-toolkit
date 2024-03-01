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

use super::TrackedResource;

#[derive(Default, Clone)]
pub struct BucketContent {
    resources: TrackedResource,
}

impl BucketContent {
    fn new(resources: TrackedResource) -> Self {
        Self { resources }
    }

    pub fn is_empty(&self) -> bool {
        matches!(self.resources, TrackedResource::StaticallyKnownEmpty(..))
    }

    pub fn is_known_resources(&self) -> bool {
        !matches!(self.resources, TrackedResource::Unknown)
    }

    pub fn take_resources(&self) -> Option<ResourceSpecifier> {
        match &self.resources {
            TrackedResource::StaticallyKnown(res) => Some(res.clone()),
            TrackedResource::StaticallyKnownEmpty(address) => {
                let resource = if address.is_fungible() {
                    ResourceSpecifier::Amount(*address, dec!(0))
                } else {
                    ResourceSpecifier::Ids(*address, indexset! {})
                };
                Some(resource)
            }
            _ => None,
        }
    }

    fn try_remove_amount(&mut self, amount: &Decimal) -> Option<Decimal> {
        match &self.resources {
            TrackedResource::StaticallyKnown(res) => {
                res.amount().expect("Must succeed").checked_sub(*amount)
            }
            _ => None,
        }
    }

    fn try_remove_non_fungible(&mut self, ids: &[NonFungibleLocalId]) -> bool {
        match &self.resources {
            TrackedResource::StaticallyKnown(res) => {
                ids.iter()
                    .filter(|item| {
                        !res.ids().expect("Must succeed").contains(*item)
                    })
                    .count()
                    == 0
            }
            _ => false,
        }
    }
}

#[derive(Default)]
pub struct BucketsTracker {
    // Buckates tracking
    buckets: IndexMap<ManifestBucket, BucketContent>,
    // Buckets id generation
    id_allocator: ManifestIdAllocator,
    // Information if we are in 'untracked buckets' mode which is enabled
    // by use of buckets with unknown content triggered by the upper layer.
    untracked_mode: bool,
}

impl BucketsTracker {
    pub fn is_untracked_mode(&self) -> bool {
        self.untracked_mode
    }

    pub fn enter_untracked_mode(&mut self) {
        self.untracked_mode = true;
    }

    pub fn new_bucket_known_resources(&mut self, resources: ResourceSpecifier) {
        if !self.untracked_mode {
            self.buckets.insert(
                self.id_allocator.new_bucket_id(),
                BucketContent::new(TrackedResource::StaticallyKnown(resources)),
            );
        }
    }

    pub fn new_empty_bucket_known_resources(
        &mut self,
        resource_address: &ResourceAddress,
    ) {
        if !self.untracked_mode {
            self.buckets.insert(
                self.id_allocator.new_bucket_id(),
                BucketContent::new(TrackedResource::StaticallyKnownEmpty(
                    *resource_address,
                )),
            );
        }
    }

    pub fn new_bucket_unknown_resources(&mut self) {
        if !self.untracked_mode {
            self.buckets.insert(
                self.id_allocator.new_bucket_id(),
                BucketContent::new(TrackedResource::Unknown),
            );
        }
    }

    pub fn is_any_bucket_with_unknown_resources(&self) -> bool {
        self.buckets
            .iter()
            .find(|i| matches!(i.1.resources, TrackedResource::Unknown))
            .is_some()
    }

    // returns consumed resources if found
    pub fn bucket_consumed(
        &mut self,
        bucket_id: &ManifestBucket,
    ) -> Option<BucketContent> {
        self.buckets.remove(bucket_id)
    }

    pub fn try_consume_fungible_from_bucket(
        &mut self,
        bucket_id: &ManifestBucket,
        amount: &Decimal,
    ) -> Option<ResourceSpecifier> {
        if !self.untracked_mode {
            let bucket =
                self.buckets.get_mut(bucket_id).expect("Bucket not found");

            if let Some(resource) = bucket.take_resources() {
                let address = resource.resource_address();
                // if operation is done on fungible resource then try to remove amount from specified bucket
                if address.is_fungible() {
                    if bucket.try_remove_amount(amount).is_some() {
                        // successfully subtracted amount
                        return Some(ResourceSpecifier::Amount(
                            address, *amount,
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
        ids: &[NonFungibleLocalId],
    ) -> Option<ResourceSpecifier> {
        if !self.untracked_mode {
            let bucket =
                self.buckets.get_mut(bucket_id).expect("Bucket not found");

            if let Some(resource) = bucket.take_resources() {
                let address = resource.resource_address();
                if !address.is_fungible() {
                    if bucket.try_remove_non_fungible(ids) {
                        // all ids are found in this bucket -> operation succeeded
                        return Some(ResourceSpecifier::Ids(
                            address,
                            IndexSet::from_iter(ids.iter().cloned()),
                        ));
                    }
                }
            }
        }
        None
    }
}
