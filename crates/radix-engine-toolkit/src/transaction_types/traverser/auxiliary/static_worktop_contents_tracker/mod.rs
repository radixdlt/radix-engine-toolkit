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

use self::{buckets_tracker::*, worktop_content_tracker::*};
use crate::transaction_types::*;
use radix_engine::system::system_modules::execution_trace::ResourceSpecifier;
use radix_transactions::prelude::*;
use scrypto::prelude::*;

mod buckets_tracker;
mod handler_function_calls;
mod handler_method_calls;
mod worktop_content_tracker;
use buckets_tracker::BucketContent;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrustedWorktopInstruction {
    /// Information if instruction is trusted.
    /// Instruction is trusted if we know exact resources transfer assiociated
    /// to that instruction (so we need to know what instruction is doing and if
    /// it transfers resources including exact count/list of these resources or not
    /// deals with resources at all).
    pub is_trusted: bool,
    /// Resources moved in context of the instruction.
    pub resources: Vec<ResourceSpecifier>,
}

#[derive(Default)]
/// Static Worktop Contents Tracker analyses manifest instructions to track worktop content
/// and buckets list and basing on that it decides if manifest instruction is trusted
/// (definition in TrustedWorktopInstruction).
///
/// The worktop is said to be trusted so long as we can statically tell what the effect
/// of the instructions are on the worktop. A single instruction whose effect on the worktop
/// can't be determined statically turns the worktop into an untrusted worktop.
///
/// Worktop content tracker operation logic:
///  If Instruction doesn't change worktop state and doesn't use buckets then it is trusted.
///  If Instruction changes worktop state:
///    1. Puts resources on the worktop (ex. Account withdraws, Return to workotop, etc.)
///       - if we know what resources has been put on the worktop then instruction is trusted
///       - if we don't know what has been put on the worktop then instruction is untrusted
///         and we are entering into untracked worktop content mode (from now we don't know
///         exactly what is on the worktop)
///    2. Takes resources from the worktop (ex. Take from worktop instructions)
///       - if we are in untracked worktop content mode then instruction is untrusted
///       - if we know the resources then instruction is trusted
///  If Instruction uses a bucket and we are not in bucket untracked mode:
///    1. If bucket is known and resources are known, then it is consumed and instruction is trusted
///    2. If bucket is known but resources are unknown then it is consumed and instruction is untrusted
///    3. If bucket is unknown then we are entering into bucket untracked mode and instruction is untrusted
///
/// Bucket tracker operaion logic:
///  Function/method call
///    1. Returns a bucket and we are not in untracked buckets mode:
///       - Put resources on the worktop
///    2. We don't know what is returned:
///       - enter untracked worktop mode
///
/// We can identify an instruction as trusted if we are in untracked worktop mode in
/// case of an instruction which returns known bucket and that bucket is later consumed.
/// Taking all from worktop switches back from untracked mode.
///
pub struct StaticWorktopContentsTracker {
    trusted_state_per_instruction: Vec<TrustedWorktopInstruction>,

    // Buckets tracking
    bucket_tracker: BucketsTracker,

    // Worktop content tracking
    worktop_content_tracker: WorktopContentTracker,
}

#[derive(Clone, Default)]
enum TrackedResource {
    StaticallyKnown(ResourceSpecifier),
    StaticallyKnownMany(Vec<ResourceSpecifier>),
    StaticallyKnownEmpty(ResourceAddress),
    StaticallyKnownNone,
    #[default]
    Unknown,
}

impl From<&BucketContent> for TrackedResource {
    fn from(bucket: &BucketContent) -> Self {
        if !bucket.is_known_resources() {
            TrackedResource::Unknown
        } else {
            match bucket.take_resources() {
                Some(res) => TrackedResource::StaticallyKnown(res),
                None => TrackedResource::Unknown,
            }
        }
    }
}

impl StaticWorktopContentsTracker {
    pub fn output(self) -> Vec<TrustedWorktopInstruction> {
        self.trusted_state_per_instruction
    }

    fn add_new_instruction(&mut self, input_resources: TrackedResource) {
        let (is_trusted, resources) = match input_resources {
            TrackedResource::StaticallyKnown(resource) => {
                (true, vec![resource])
            }
            TrackedResource::StaticallyKnownMany(resources) => {
                (true, resources.to_vec())
            }
            TrackedResource::StaticallyKnownEmpty(address) => {
                let resource = if address.is_fungible() {
                    ResourceSpecifier::Amount(address, dec!(0))
                } else {
                    ResourceSpecifier::Ids(address, indexset! {})
                };
                (true, vec![resource])
            }
            TrackedResource::StaticallyKnownNone => (true, vec![]),
            TrackedResource::Unknown => (false, vec![]),
        };
        self.trusted_state_per_instruction
            .push(TrustedWorktopInstruction {
                is_trusted,
                resources,
            });
    }

    fn add_new_instruction_from_bucket(&mut self, bucket: &BucketContent) {
        self.add_new_instruction(bucket.into());
    }
}

impl ManifestSummaryCallback for StaticWorktopContentsTracker {
    fn on_instruction(
        &mut self,
        instruction: &InstructionV1,
        instruction_index: usize,
    ) {
        match instruction {
            InstructionV1::TakeAllFromWorktop { resource_address } => {
                if !self.worktop_content_tracker.is_untracked_mode() {
                    if let Some(resources) = self
                        .worktop_content_tracker
                        .take_from_worktop_by_address(*resource_address)
                    {
                        self.bucket_tracker
                            .new_bucket_known_resources(resources.clone());
                        self.add_new_instruction(
                            TrackedResource::StaticallyKnown(
                                resources.to_owned(),
                            ),
                        );
                    } else {
                        self.bucket_tracker
                            .new_empty_bucket_known_resources(resource_address);
                        self.add_new_instruction(
                            TrackedResource::StaticallyKnownEmpty(
                                *resource_address,
                            ),
                        )
                    }
                } else {
                    // we don't know what is exactly on the worktop
                    self.bucket_tracker.new_bucket_unknown_resources();
                    self.add_new_instruction(TrackedResource::Unknown)
                }
            }
            InstructionV1::TakeFromWorktop {
                resource_address,
                amount,
            } => {
                if !self.worktop_content_tracker.is_untracked_mode() {
                    if resource_address.is_fungible() {
                        let resources = ResourceSpecifier::Amount(
                            *resource_address,
                            *amount,
                        );
                        if amount.is_zero()
                            || self
                                .worktop_content_tracker
                                .take_from_worktop(resources.clone())
                        {
                            self.bucket_tracker
                                .new_bucket_known_resources(resources.clone());
                            self.add_new_instruction(
                                TrackedResource::StaticallyKnown(resources),
                            );
                        } else {
                            // unabe to take fungible by amount
                            self.bucket_tracker.new_bucket_unknown_resources();
                            self.add_new_instruction(TrackedResource::Unknown)
                        }
                    } else if amount.is_zero() {
                        let resources = ResourceSpecifier::Ids(
                            *resource_address,
                            indexset! {},
                        );
                        self.bucket_tracker
                            .new_bucket_known_resources(resources.clone());
                        self.add_new_instruction(
                            TrackedResource::StaticallyKnown(resources),
                        );
                    } else {
                        // non fungible take by amount
                        self.bucket_tracker.new_bucket_unknown_resources();
                        self.add_new_instruction(TrackedResource::Unknown)
                    }
                } else {
                    // we don't know what is taken from worktop
                    self.bucket_tracker.new_bucket_unknown_resources();
                    self.add_new_instruction(TrackedResource::Unknown);
                }
            }
            InstructionV1::TakeNonFungiblesFromWorktop {
                resource_address,
                ids,
            } => {
                if !self.worktop_content_tracker.is_untracked_mode() {
                    let indexed_ids: IndexSet<NonFungibleLocalId> =
                        ids.iter().cloned().collect();
                    let resources =
                        ResourceSpecifier::Ids(*resource_address, indexed_ids);

                    if ids.is_empty()
                        || self
                            .worktop_content_tracker
                            .take_from_worktop(resources.clone())
                    {
                        self.bucket_tracker
                            .new_bucket_known_resources(resources.clone());
                        self.add_new_instruction(
                            TrackedResource::StaticallyKnown(resources),
                        );
                    } else {
                        // invalid operation fungible take by ammount
                        self.bucket_tracker.new_bucket_unknown_resources();
                        self.add_new_instruction(TrackedResource::Unknown)
                    }
                } else {
                    // we don't know what is taken from worktop
                    self.bucket_tracker.new_bucket_unknown_resources();
                    self.add_new_instruction(TrackedResource::Unknown);
                }
            }

            InstructionV1::ReturnToWorktop { bucket_id } => {
                if !self.bucket_tracker.is_untracked_mode() {
                    let bucket = self
                        .bucket_tracker
                        .bucket_consumed(bucket_id)
                        .expect("Must succeed");
                    self.add_new_instruction_from_bucket(&bucket);
                    if let Some(resources) = bucket.take_resources() {
                        self.worktop_content_tracker.put_to_worktop(resources);
                    } else {
                        // we don't know exactly what is put on worktop
                        self.worktop_content_tracker.enter_untracked_mode();
                    }
                } else {
                    // we don't know exactly what is put on worktop
                    self.worktop_content_tracker.enter_untracked_mode();
                    self.add_new_instruction(TrackedResource::Unknown);
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
                self.add_new_instruction(TrackedResource::StaticallyKnownNone);
            }

            InstructionV1::CreateProofFromBucketOfAmount {
                bucket_id,
                amount,
            } => {
                // doesn't consume the bucket
                // We are trying to consume amount of fungible resource from bucket if it is possible
                // (for fungible resources only) then instruction is trusted
                if let Some(res) = self
                    .bucket_tracker
                    .try_consume_fungible_from_bucket(bucket_id, amount)
                {
                    self.add_new_instruction(TrackedResource::StaticallyKnown(
                        res,
                    ));
                } else {
                    self.add_new_instruction(TrackedResource::Unknown);
                }
            }
            InstructionV1::CreateProofFromBucketOfNonFungibles {
                bucket_id,
                ids,
            } => {
                // doesn't consume the bucket
                // We are trying to consume ids of non fungible resource from bucket if it is possible
                // then instruction is trusted
                if let Some(res) = self
                    .bucket_tracker
                    .try_consume_non_fungible_from_bucket(bucket_id, ids)
                {
                    self.add_new_instruction(TrackedResource::StaticallyKnown(
                        res,
                    ));
                } else {
                    self.add_new_instruction(TrackedResource::Unknown);
                }
            }
            InstructionV1::CreateProofFromBucketOfAll { bucket_id }
            | InstructionV1::BurnResource { bucket_id } => {
                if !self.bucket_tracker.is_untracked_mode() {
                    let bucket = self
                        .bucket_tracker
                        .bucket_consumed(bucket_id)
                        .expect("Bucket not found");
                    self.add_new_instruction_from_bucket(&bucket);
                } else {
                    self.add_new_instruction(TrackedResource::Unknown);
                }
            }

            InstructionV1::CallMethod {
                address,
                method_name,
                args,
            } => self.handle_call_methods(address, method_name, args),

            InstructionV1::CallFunction {
                package_address,
                blueprint_name,
                function_name,
                args,
            } => self.handle_call_functions(
                package_address,
                blueprint_name,
                function_name,
                args,
            ),

            InstructionV1::CallRoyaltyMethod {
                method_name, args, ..
            } => self.handle_call_royalty_methods(method_name, args),

            InstructionV1::CallRoleAssignmentMethod { .. }
            | InstructionV1::CallMetadataMethod { .. } => {
                // methods are trusted as they doesn't change the worktop state
                self.add_new_instruction(TrackedResource::StaticallyKnownNone)
            }

            InstructionV1::CallDirectVaultMethod { .. } => {
                // we don't know if something was put on worktop -> enter untracked worktop content mode
                self.worktop_content_tracker.enter_untracked_mode();
                self.bucket_tracker.enter_untracked_mode();
                self.add_new_instruction(TrackedResource::Unknown)
            }
        }

        assert_eq!(
            self.trusted_state_per_instruction.len(),
            instruction_index + 1,
            "Manifest instruction not covered by trusted worktop!"
        );
    }
}

impl ExecutionSummaryCallback for StaticWorktopContentsTracker {}
