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

use super::error::InstructionVisitorError;
use super::traits::InstructionVisitor;
use transaction::prelude::InstructionV1;

pub fn traverse(
    instructions: &[InstructionV1],
    visitors: &mut [&mut dyn InstructionVisitor],
) -> Result<(), InstructionVisitorError> {
    for instruction in instructions {
        for_each_enabled_visitor!(visitors, visit_instruction(instruction));
        match instruction {
            InstructionV1::TakeAllFromWorktop { resource_address } => {
                for_each_enabled_visitor!(
                    visitors,
                    visit_take_all_from_worktop(resource_address)
                );
            }
            InstructionV1::TakeFromWorktop {
                resource_address,
                amount,
            } => {
                for_each_enabled_visitor!(
                    visitors,
                    visit_take_from_worktop(resource_address, amount)
                );
            }
            InstructionV1::TakeNonFungiblesFromWorktop {
                resource_address,
                ids,
            } => {
                for_each_enabled_visitor!(
                    visitors,
                    visit_take_non_fungibles_from_worktop(
                        resource_address,
                        ids
                    )
                );
            }
            InstructionV1::ReturnToWorktop { bucket_id } => {
                for_each_enabled_visitor!(
                    visitors,
                    visit_return_to_worktop(bucket_id)
                )
            }
            InstructionV1::AssertWorktopContainsAny { resource_address } => {
                for_each_enabled_visitor!(
                    visitors,
                    visit_assert_worktop_contains_any(resource_address)
                )
            }
            InstructionV1::AssertWorktopContains {
                resource_address,
                amount,
            } => {
                for_each_enabled_visitor!(
                    visitors,
                    visit_assert_worktop_contains(resource_address, amount)
                )
            }
            InstructionV1::AssertWorktopContainsNonFungibles {
                resource_address,
                ids,
            } => {
                for_each_enabled_visitor!(
                    visitors,
                    visit_assert_worktop_contains_non_fungibles(
                        resource_address,
                        ids
                    )
                )
            }
            InstructionV1::PopFromAuthZone {} => {
                for_each_enabled_visitor!(visitors, visit_pop_from_auth_zone())
            }
            InstructionV1::PushToAuthZone { proof_id } => {
                for_each_enabled_visitor!(
                    visitors,
                    visit_push_to_auth_zone(proof_id)
                )
            }
            InstructionV1::DropNamedProofs => {
                for_each_enabled_visitor!(visitors, visit_drop_named_proofs())
            }
            InstructionV1::DropAuthZoneProofs => {
                for_each_enabled_visitor!(
                    visitors,
                    visit_drop_auth_zone_proofs()
                )
            }
            InstructionV1::DropAuthZoneSignatureProofs {} => {
                for_each_enabled_visitor!(
                    visitors,
                    visit_drop_auth_zone_signature_proofs()
                )
            }
            InstructionV1::DropAuthZoneRegularProofs {} => {
                for_each_enabled_visitor!(
                    visitors,
                    visit_drop_auth_zone_regular_proofs()
                )
            }

            InstructionV1::CreateProofFromAuthZoneOfAmount {
                resource_address,
                amount,
            } => {
                for_each_enabled_visitor!(
                    visitors,
                    visit_create_proof_from_auth_zone_of_amount(
                        resource_address,
                        amount
                    )
                )
            }
            InstructionV1::CreateProofFromAuthZoneOfNonFungibles {
                resource_address,
                ids,
            } => {
                for_each_enabled_visitor!(
                    visitors,
                    visit_create_proof_from_auth_zone_of_non_fungibles(
                        resource_address,
                        ids
                    )
                )
            }
            InstructionV1::CreateProofFromAuthZoneOfAll {
                resource_address,
            } => {
                for_each_enabled_visitor!(
                    visitors,
                    visit_create_proof_from_auth_zone_of_all(resource_address)
                )
            }
            InstructionV1::CreateProofFromBucketOfAmount {
                bucket_id,
                amount,
            } => {
                for_each_enabled_visitor!(
                    visitors,
                    visit_create_proof_from_bucket_of_amount(bucket_id, amount)
                )
            }
            InstructionV1::CreateProofFromBucketOfNonFungibles {
                bucket_id,
                ids,
            } => {
                for_each_enabled_visitor!(
                    visitors,
                    visit_create_proof_from_bucket_of_non_fungibles(
                        bucket_id, ids
                    )
                )
            }
            InstructionV1::CreateProofFromBucketOfAll { bucket_id } => {
                for_each_enabled_visitor!(
                    visitors,
                    visit_create_proof_from_bucket_of_all(bucket_id)
                )
            }
            InstructionV1::BurnResource { bucket_id } => {
                for_each_enabled_visitor!(
                    visitors,
                    visit_burn_resource(bucket_id)
                )
            }
            InstructionV1::CloneProof { proof_id } => {
                for_each_enabled_visitor!(visitors, visit_clone_proof(proof_id))
            }
            InstructionV1::DropProof { proof_id } => {
                for_each_enabled_visitor!(visitors, visit_drop_proof(proof_id))
            }
            InstructionV1::CallFunction {
                package_address,
                blueprint_name,
                function_name,
                args,
            } => {
                for_each_enabled_visitor!(
                    visitors,
                    visit_call_function(
                        package_address,
                        blueprint_name,
                        function_name,
                        args
                    )
                )
            }
            InstructionV1::CallMethod {
                address,
                method_name,
                args,
            } => {
                for_each_enabled_visitor!(
                    visitors,
                    visit_call_method(address, method_name, args)
                )
            }
            InstructionV1::CallRoyaltyMethod {
                address,
                method_name,
                args,
            } => {
                for_each_enabled_visitor!(
                    visitors,
                    visit_call_royalty_method(address, method_name, args)
                )
            }
            InstructionV1::CallMetadataMethod {
                address,
                method_name,
                args,
            } => {
                for_each_enabled_visitor!(
                    visitors,
                    visit_call_metadata_method(address, method_name, args)
                )
            }
            InstructionV1::CallRoleAssignmentMethod {
                address,
                method_name,
                args,
            } => {
                for_each_enabled_visitor!(
                    visitors,
                    visit_call_role_assignment_method(
                        address,
                        method_name,
                        args
                    )
                )
            }
            InstructionV1::CallDirectVaultMethod {
                address,
                method_name,
                args,
            } => {
                for_each_enabled_visitor!(
                    visitors,
                    visit_call_direct_vault_method(address, method_name, args)
                )
            }
            InstructionV1::DropAllProofs => {
                for_each_enabled_visitor!(visitors, visit_drop_all_proofs())
            }
            InstructionV1::AllocateGlobalAddress {
                package_address,
                blueprint_name,
            } => {
                for_each_enabled_visitor!(
                    visitors,
                    visit_allocate_global_address(
                        package_address,
                        blueprint_name
                    )
                )
            }
        }

        for visitor in visitors.iter_mut() {
            if visitor.is_enabled() {
                visitor.post_visit()?;
            }
        }
    }

    Ok(())
}

macro_rules! for_each_enabled_visitor {
    ($visitors: expr, $method_ident: ident ( $($arg: ident),* $(,)? )) => {
        for visitor in $visitors.iter_mut() {
            if visitor.is_enabled() {
                visitor.$method_ident(
                    $($arg),*
                )?;
            }
        }
    };
}
use for_each_enabled_visitor;
