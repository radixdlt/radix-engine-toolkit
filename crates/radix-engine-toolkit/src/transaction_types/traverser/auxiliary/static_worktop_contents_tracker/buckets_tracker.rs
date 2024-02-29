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

#[derive(Default, Clone)]
pub struct BucketContent {
    // resources can be None for empty buckets
    resources: Option<ResourceSpecifier>,
    unknown_resources: bool,
}
impl BucketContent {
    fn new(
        resources: Option<ResourceSpecifier>,
        unknown_resources: bool,
    ) -> Self {
        Self {
            resources,
            unknown_resources,
        }
    }
    // Empty bucket means known resource is None
    pub fn is_empty(&self) -> bool {
        self.resources.is_none() && !self.unknown_resources
    }
    pub fn is_known_resources(&self) -> bool {
        !self.unknown_resources
    }
    pub fn take_resources(&self) -> Option<ResourceSpecifier> {
        if self.is_empty() {
            None
        } else {
            self.resources.to_owned()
        }
    }
    fn try_remove_amount(&mut self, amount: &Decimal) -> Option<Decimal> {
        if let Some(res) = self.resources.as_mut() {
            res.amount().expect("Must succeed").checked_sub(*amount)
        } else {
            None
        }
    }
    fn try_remove_non_fungible(&mut self, ids: &[NonFungibleLocalId]) -> bool {
        if let Some(ResourceSpecifier::Ids(_, bucket_ids)) =
            self.resources.as_mut()
        {
            ids.iter().filter(|item| bucket_ids.contains(*item)).count() == 0
        } else {
            false
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
                BucketContent::new(Some(resources), false),
            );
        }
    }

    pub fn new_empty_bucket_known_resources(
        &mut self,
        resource_address: &ResourceAddress,
    ) {
        if !self.untracked_mode {
            let resource = if resource_address.is_fungible() {
                ResourceSpecifier::Amount(*resource_address, dec!(0))
            } else {
                ResourceSpecifier::Ids(*resource_address, indexset! {})
            };
            self.buckets.insert(
                self.id_allocator.new_bucket_id(),
                BucketContent::new(Some(resource), false),
            );
        }
    }

    pub fn new_bucket_unknown_resources(&mut self) {
        if !self.untracked_mode {
            self.buckets.insert(
                self.id_allocator.new_bucket_id(),
                BucketContent::new(None, true),
            );
        }
    }

    pub fn is_any_bucket_with_unknown_resources(&self) -> bool {
        self.buckets
            .iter()
            .find(|i| i.1.unknown_resources)
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
            if bucket.resources.is_some() {
                let address =
                    bucket.resources.as_ref().unwrap().resource_address();
                // if operation is done on fungible resource then try to remove amount from specified bucket
                if address.is_fungible() {
                    if let Some(value) = bucket.try_remove_amount(amount) {
                        return Some(ResourceSpecifier::Amount(address, value));
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
            if bucket.resources.is_some() {
                let address =
                    bucket.resources.as_ref().unwrap().resource_address();
                if bucket.try_remove_non_fungible(ids) {
                    // all ids are found in this bucket -> operation succeeded
                    return Some(ResourceSpecifier::Ids(
                        address,
                        IndexSet::from_iter(ids.iter().cloned()),
                    ));
                }
            }
        }
        None
    }
}
