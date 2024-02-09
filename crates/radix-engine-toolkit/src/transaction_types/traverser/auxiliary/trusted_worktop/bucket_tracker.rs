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
struct Bucket {
    resources: Option<ResourceSpecifier>,
    named: bool,
}
impl Bucket {
    fn new(resources: Option<ResourceSpecifier>, named: bool) -> Self {
        Self { resources, named }
    }
    fn try_remove_amount(&mut self, amount: &Decimal) -> Option<Decimal> {
        if let Some(res) = self.resources.as_mut() {
            res.amount().expect("Must succeed").checked_sub(*amount)
        } else {
            None
        }
    }
    fn try_remove_non_fungible(
        &mut self,
        ids: &Vec<NonFungibleLocalId>,
    ) -> bool {
        if let Some(res) = self.resources.as_mut() {
            match res {
                ResourceSpecifier::Ids(_, bucket_ids) => {
                    ids.iter().filter(|item| bucket_ids.contains(*item)).count()
                        == 0
                }
                _ => false,
            }
        } else {
            false
        }
    }
}

#[derive(Default)]
pub struct BucketTracker {
    // Buckates tracking
    buckets: IndexMap<ManifestBucket, Bucket>,
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

    pub fn new_named_bucket_known_resources(
        &mut self,
        resources: ResourceSpecifier,
    ) {
        if !self.untracked_mode {
            self.buckets.insert(
                self.id_allocator.new_bucket_id(),
                Bucket::new(Some(resources), true),
            );
        }
    }

    /*pub fn new_unnamed_bucket_known_resources(&mut self, resources: ResourceSpecifier) {
        if !self.untracked_mode {
            self.buckets.insert(
                self.id_allocator.new_bucket_id(),
                Bucket::new(Some(resources), false),
            );
        }
    }*/

    pub fn new_named_bucket_unknown_resources(&mut self) {
        if !self.untracked_mode {
            self.buckets.insert(
                self.id_allocator.new_bucket_id(),
                Bucket::new(None, true),
            );
        }
    }

    pub fn new_unnamed_bucket_unknown_resources(&mut self) {
        if !self.untracked_mode {
            self.buckets.insert(
                self.id_allocator.new_bucket_id(),
                Bucket::new(None, false),
            );
        }
    }

    // returns option to indicate buckets with unknown resources
    pub fn consume_all_buckets(&mut self) -> Vec<Option<ResourceSpecifier>> {
        let ret = self
            .buckets
            .iter()
            .map(|(_, v)| v.resources.to_owned())
            .collect();
        self.buckets.clear();
        self.untracked_mode = false;
        ret
    }

    pub fn consume_unnamed_buckets(
        &mut self,
        address: &ResourceAddress,
    ) -> Vec<Option<ResourceSpecifier>> {
        let unnamed_buckets: Vec<(ManifestBucket, Bucket)> = self
            .buckets
            .iter()
            .filter(|(_, bucket)| {
                if bucket.resources.is_some() {
                    bucket.resources.as_ref().unwrap().resource_address()
                        == *address
                        && !bucket.named
                } else {
                    false
                }
            })
            .map(|(k, v)| (*k, v.clone()))
            .collect();

        unnamed_buckets.iter().for_each(|(k, _)| {
            self.buckets.remove(k);
        });

        unnamed_buckets
            .iter()
            .map(|(_, v)| v.resources.to_owned())
            .collect()
    }

    pub fn is_bucket_with_unknown_resources(&self) -> bool {
        self.buckets
            .iter()
            .find(|i| i.1.resources.is_none())
            .is_some()
    }

    /*pub fn is_unnamed_bucket(&self) -> bool {
        self.buckets.iter().find(|i| !i.1.named).is_some()
    }*/

    // returns consumed resources if found
    pub fn bucket_consumed(
        &mut self,
        bucket_id: &ManifestBucket,
    ) -> Option<Option<ResourceSpecifier>> {
        self.buckets
            .remove(bucket_id)
            .map(|bucket| bucket.resources)
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
        ids: &Vec<NonFungibleLocalId>,
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
