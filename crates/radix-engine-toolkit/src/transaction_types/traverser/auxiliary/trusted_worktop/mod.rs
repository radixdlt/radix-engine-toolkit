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

use crate::transaction_types::*;
use radix_engine::system::system_modules::execution_trace::ResourceSpecifier;
use scrypto::prelude::*;
use transaction::prelude::*;
use transaction::validation::ManifestIdAllocator;

mod handler_function_calls;
mod handler_method_calls;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrustedWorktopInstruction {
    // Information if instruction is trusted.
    // Instruction is trusted if we know exact resources transfer assiociated 
    // to that instruction (so we need to know what instruction is doing and if
    // it transfers resources including exact count/list of these resources or not
    // deals with resources at all).
    pub trusted: bool,
    // Resources moved in context of the instruction.
    pub resources: Vec<ResourceSpecifier>,
}

#[derive(Default)]
// Trusted Worktop analyzes manifest instruction to tracks worktop content and
// buckets list and basing on that it decides if manifest instruction is trusted
// (definition in TrustedWorktopInstruction).
// 
// Worktop content tracker operation logic:
//  If Instruction doesn't change worktop state and doesn't use buckets then it is trusted.
//  If Instruction changes worktop state:
//    1. Puts resources on the worktop (ex. Account withdraws, Return to workotop, etc.)
//       - if we know what resources has been put on the worktop then instruction is trusted
//       - if we don't know what has been put on the worktop then instruction is untrasted
//         and we are entering into untracked worktop content mode (from now we don't know 
//         exactly what is on the worktop)
//    2. Takes resources from the worktop (ex. Take from worktop instructions)
//       - if we are in untracked worktop content mode then instruction is untrasted
//       - if we know the resources then instruction is trusted
//  If Instruction uses a bucket and we are not in bucket untracked mode:
//    1. If bucket is known and resources are known, then it is consumed and instruction is trusted
//    2. If bucket is known but resources are unknown then it is consumed and instruction is untrasted
//    3. If bucket is unknown then we are entering into bucket untracked mode and instruction is untrusted
//
// Bucket tracker operaion logic:
//  Function/method call 
//    1. Returns a bucket and we are not in untracked buckets mode:
//       - if we know what is in the bucket -> call new_bucket_known_resources()
//       - if we don't know what is in the bucket -> call new_bucket_unknown_resources()
//    2. We don't know what is returned: 
//       - enter untracked buckets mode
//
// We can indentify an instruction as trusted if we are in untracked worktop mode in
// case of an instruction which returns known bucket and that bucket is later consumed.
//
pub struct TrustedWorktop {
    trusted_state_per_instruction: Vec<TrustedWorktopInstruction>,

    // Buckates tracking
    buckets: IndexMap<ManifestBucket, Option<ResourceSpecifier>>,
    // Buckets id generation
    id_allocator: ManifestIdAllocator,
    // Information if we are in 'untracked buckets' mode which is triggered
    // by use of buckets with unknown content.
    untrack_buckets: bool,

    // Worktop content tracking
    worktop_content: IndexMap<ResourceAddress, ResourceSpecifier>,
    // Information if we are in 'untracked worktop' mode which is triggered
    // when we don't know what was put or taken from the worktop.
    untrack_worktop_content: bool,
}

impl TrustedWorktop {
    pub fn output(self) -> Vec<TrustedWorktopInstruction> {
        self.trusted_state_per_instruction
    }

    fn add_new_instruction(
        &mut self,
        trusted: bool,
        input_resources: Option<ResourceSpecifier>,
    ) {
        let resources = match input_resources {
            Some(res) => vec![res],
            None => vec![],
        };
        self.trusted_state_per_instruction
            .push(TrustedWorktopInstruction { trusted, resources });
    }

    fn add_new_instruction_with_many_resources(
        &mut self,
        trusted: bool,
        resources: Vec<ResourceSpecifier>,
    ) {
        self.trusted_state_per_instruction
            .push(TrustedWorktopInstruction { trusted, resources });
    }

    fn new_bucket_known_resources(&mut self, resources: ResourceSpecifier) {
        if !self.untrack_buckets {
            self.buckets
                .insert(self.id_allocator.new_bucket_id(), Some(resources));
        }
    }

    fn new_bucket_unknown_resources(&mut self) {
        if !self.untrack_buckets {
            self.buckets.insert(self.id_allocator.new_bucket_id(), None);
        }
    }

    // returns consumed resources if found
    fn bucket_consumed(
        &mut self,
        bucket_id: &ManifestBucket,
    ) -> Option<Option<ResourceSpecifier>> {
        self.buckets.remove(bucket_id)
    }

    fn put_to_worktop(&mut self, resources: ResourceSpecifier) {
        if !self.untrack_worktop_content {
            if let Some(res) =
                self.worktop_content.get(&resources.resource_address())
            {
                // if found then exted with passed values
                match res {
                    ResourceSpecifier::Amount(_address, amount) => {
                        self.worktop_content.insert(
                            resources.resource_address(),
                            ResourceSpecifier::Amount(
                                resources.resource_address(),
                                amount
                                    .checked_add(*resources.amount().unwrap())
                                    .unwrap(),
                            ),
                        );
                    }
                    ResourceSpecifier::Ids(_address, ids) => {
                        let mut new_ids = ids.clone();
                        new_ids.extend(resources.ids().unwrap().clone());
                        self.worktop_content.insert(
                            resources.resource_address(),
                            ResourceSpecifier::Ids(
                                resources.resource_address(),
                                new_ids,
                            ),
                        );
                    }
                }
            } else {
                self.worktop_content
                    .insert(resources.resource_address(), resources);
            }
        }
    }

    // return true in case of success
    fn take_from_worktop(&mut self, resources: ResourceSpecifier) -> bool {
        if let Some(res) =
            self.worktop_content.get(&resources.resource_address())
        {
            // if found then subtract passed values
            match res {
                ResourceSpecifier::Amount(_address, amount) => {
                    if resources.resource_address().is_fungible() {
                        self.worktop_content.insert(
                            resources.resource_address(),
                            ResourceSpecifier::Amount(
                                resources.resource_address(),
                                amount
                                    .checked_sub(*resources.amount().unwrap())
                                    .unwrap(),
                            ),
                        );
                        true
                    } else {
                        // don't know which non fungibles will be taken
                        // not setting untracked worktop content mode, as other instructions can still be valid
                        false
                    }
                }
                ResourceSpecifier::Ids(_address, ids) => {
                    if !resources.resource_address().is_fungible() {
                        let mut new_ids = ids.clone();
                        new_ids.retain(|item| {
                            !resources.ids().unwrap().contains(item)
                        });
                        self.worktop_content.insert(
                            resources.resource_address(),
                            ResourceSpecifier::Ids(
                                resources.resource_address(),
                                new_ids,
                            ),
                        );
                        true
                    } else {
                        // cannot take fungible -> worktop content is invalid
                        self.untrack_worktop_content = true;
                        false
                    }
                }
            }
        } else {
            false
        }
    }

    fn take_from_worktop_by_address(
        &mut self,
        resource_address: ResourceAddress,
    ) -> Option<ResourceSpecifier> {
        self.worktop_content
            .remove(&resource_address)
            .map(|item| item.clone())
    }

    fn take_all_from_worktop(&mut self) -> Vec<ResourceSpecifier> {
        let ret = self
            .worktop_content
            .iter()
            .map(|(_k, v)| v.to_owned())
            .collect();
        // worktop is cleared so we can start tracking it back (if untracked)
        self.untrack_worktop_content = false;
        self.worktop_content.clear();
        ret
    }

    fn merge_same_resources(
        resources: &[ResourceSpecifier],
    ) -> Vec<ResourceSpecifier> {
        let mut set: IndexMap<ResourceAddress, Vec<&ResourceSpecifier>> =
            IndexMap::new();

        resources.iter().for_each(|resource| {
            if let Some((_, key, item)) =
                set.get_full_mut(&resource.resource_address())
            {
                assert_eq!(
                    resource.resource_address().is_fungible(),
                    key.is_fungible()
                );
                item.push(resource);
            } else {
                set.insert(resource.resource_address(), vec![resource]);
            }
        });

        let mut ret: Vec<ResourceSpecifier> = Vec::new();
        for (k, v) in set.iter() {
            if !v.is_empty() {
                ret.push(match v[0] {
                    ResourceSpecifier::Amount(_, _) => {
                        let mut amount = dec!(0);
                        for resource in v {
                            amount = amount
                                .checked_add(*resource.amount().unwrap())
                                .unwrap();
                        }
                        ResourceSpecifier::Amount(*k, amount)
                    }
                    ResourceSpecifier::Ids(_, _) => {
                        let mut new_ids: IndexSet<NonFungibleLocalId> =
                            IndexSet::new();
                        for resource in v {
                            new_ids.extend(resource.ids().unwrap().clone());
                        }
                        ResourceSpecifier::Ids(*k, new_ids)
                    }
                })
            }
        }
        ret
    }
}

impl ManifestSummaryCallback for TrustedWorktop {
    fn on_instruction(
        &mut self,
        instruction: &InstructionV1,
        instruction_index: usize,
    ) {
        match instruction {
            InstructionV1::TakeAllFromWorktop { resource_address } => {
                if !self.untrack_worktop_content {
                    let resources = self
                        .take_from_worktop_by_address(*resource_address)
                        .expect("Expected resources");
                    self.new_bucket_known_resources(resources.clone());
                    self.add_new_instruction(true, Some(resources));
                } else {
                    // we don't know what is exactly on the worktop
                    self.new_bucket_unknown_resources();
                    self.add_new_instruction(false, None)
                }
            }
            InstructionV1::TakeFromWorktop {
                resource_address,
                amount,
            } => {
                if !self.untrack_worktop_content {
                    let resources =
                        ResourceSpecifier::Amount(*resource_address, *amount);
                    if self.take_from_worktop(resources.clone()) {
                        self.new_bucket_known_resources(resources.clone());
                        self.add_new_instruction(true, Some(resources));
                    } else {
                        // non fungible take by ammount
                        self.new_bucket_unknown_resources();
                        self.add_new_instruction(false, None)
                    }
                } else {
                    // we don't know what is taken from worktop
                    self.new_bucket_unknown_resources();
                    self.add_new_instruction(false, None);
                }
            }
            InstructionV1::TakeNonFungiblesFromWorktop {
                resource_address,
                ids,
            } => {
                if !self.untrack_worktop_content {
                    let indexed_ids: IndexSet<NonFungibleLocalId> =
                        ids.iter().map(|i| i.clone()).collect();
                    let resources =
                        ResourceSpecifier::Ids(*resource_address, indexed_ids);

                    if self.take_from_worktop(resources.clone()) {
                        self.new_bucket_known_resources(resources.clone());
                        self.add_new_instruction(true, Some(resources));
                    } else {
                        // invalid operation fungible take by ammount
                        self.new_bucket_unknown_resources();
                        self.add_new_instruction(false, None)
                    }
                } else {
                    // we don't know what is taken from worktop
                    self.new_bucket_unknown_resources();
                    self.add_new_instruction(false, None);
                }
            }

            InstructionV1::ReturnToWorktop { bucket_id } => {
                if !self.untrack_buckets {
                    if let Some(resources) =
                        self.bucket_consumed(bucket_id).expect("Must succeed")
                    {
                        self.add_new_instruction(true, Some(resources.clone()));
                        if !self.untrack_worktop_content {
                            self.put_to_worktop(resources);
                        }
                    } else {
                        // we don't know exactly what is put on worktop
                        self.untrack_worktop_content = true;
                        self.add_new_instruction(false, None);
                    }
                } else {
                    // we don't know exactly what is put on worktop
                    self.untrack_worktop_content = true;
                    self.add_new_instruction(false, None);
                }
            }

            InstructionV1::AssertWorktopContainsAny { .. }
            | InstructionV1::AssertWorktopContains { .. }
            | InstructionV1::AssertWorktopContainsNonFungibles { .. }
            | InstructionV1::PopFromAuthZone
            | InstructionV1::PushToAuthZone { .. }
            | InstructionV1::CreateProofFromAuthZoneOfAmount { .. }
            | InstructionV1::CreateProofFromAuthZoneOfNonFungibles { .. }
            | InstructionV1::CreateProofFromAuthZoneOfAll { .. }
            | InstructionV1::DropAuthZoneProofs
            | InstructionV1::DropAuthZoneRegularProofs
            | InstructionV1::DropAuthZoneSignatureProofs
            | InstructionV1::CloneProof { .. }
            | InstructionV1::DropProof { .. }
            | InstructionV1::DropNamedProofs
            | InstructionV1::DropAllProofs
            | InstructionV1::AllocateGlobalAddress { .. } => {
                self.add_new_instruction(true, None);
            }

            InstructionV1::CreateProofFromBucketOfAmount {
                bucket_id,
                amount,
            } => { // todo: change to trusted instruction
                if !self.untrack_buckets {
                    if let Some(resources) =
                        self.buckets.get_mut(bucket_id).expect("Must succeed")
                    {
                        if resources.resource_address().is_fungible() {
                            // if operation is done on fungible resource then remove amount from specified bucket
                            resources
                                .amount()
                                .expect("Must succeed")
                                .checked_sub(*amount);
                        } else {
                            // otherwise set bucket resources as unknown
                            self.buckets.insert(*bucket_id, None);
                        }
                    } // else we already don't know what is in the bucket
                }
                self.add_new_instruction(true, None);
            }
            InstructionV1::CreateProofFromBucketOfNonFungibles {
                bucket_id,
                ids,
            } => {// todo: change to trusted instruction
                if !self.untrack_buckets {
                    if let Some(resources) =
                        self.buckets.get_mut(bucket_id).expect("Must succeed")
                    {
                        match resources {
                            ResourceSpecifier::Ids(_, bucket_ids) => {
                                // preserve in bucket non fungibles not used to create a proof
                                bucket_ids.retain(|item| !ids.contains(item));
                            }
                            _ => panic!("Expected non fungible"),
                        }
                    } // else we already don't know what is in the bucket
                }
                self.add_new_instruction(true, None);
            }
            InstructionV1::CreateProofFromBucketOfAll { bucket_id } // todo: change to trusted instruction
            | InstructionV1::BurnResource { bucket_id } => {
                self.buckets.remove(bucket_id); // use bucket conume fn
                self.add_new_instruction(true, None);
            }

            InstructionV1::CallMethod {
                address,
                method_name,
                args,
            } => self.handle_call_methods(address, method_name, args),

            // call of a function from unknown blueprint
            InstructionV1::CallFunction { 
                package_address, 
                blueprint_name, 
                function_name,
                args,
             } => self.handle_call_functions(package_address, blueprint_name, function_name,
                args),

            InstructionV1::CallRoyaltyMethod {
                method_name, args, ..
            } => self.handle_call_royalty_methods(method_name, args),

            InstructionV1::CallRoleAssignmentMethod { .. }
            | InstructionV1::CallMetadataMethod { .. } => {
                // methods are trusted as they doesn't change the worktop state
                self.add_new_instruction(true, None)
            }

            InstructionV1::CallDirectVaultMethod { .. } => {
                // we don't know if something was put on worktop -> enter untracked worktop content mode
                self.untrack_worktop_content = true;
                self.untrack_buckets = true;
                self.add_new_instruction(false, None)
            }
        }

        assert_eq!(
            self.trusted_state_per_instruction.len(),
            instruction_index + 1
        );
    }
}
