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

#![allow(unused)]

use super::error::InstructionVisitorError;
use scrypto::prelude::*;
use transaction::prelude::{DynamicGlobalAddress, DynamicPackageAddress, InstructionV1};

pub trait InstructionVisitor {
    //=====
    // Raw
    //=====

    #[inline]
    fn visit_instruction(
        &mut self,
        instruction: &InstructionV1,
    ) -> Result<(), InstructionVisitorError> {
        Ok(())
    }

    //==============
    // Instructions
    //==============

    #[inline]
    fn visit_take_all_from_worktop(
        &mut self,
        resource_address: &ResourceAddress,
    ) -> Result<(), InstructionVisitorError> {
        Ok(())
    }

    #[inline]
    fn visit_take_from_worktop(
        &mut self,
        resource_address: &ResourceAddress,
        amount: &Decimal,
    ) -> Result<(), InstructionVisitorError> {
        Ok(())
    }

    #[inline]
    fn visit_take_non_fungibles_from_worktop(
        &mut self,
        resource_address: &ResourceAddress,
        ids: &[NonFungibleLocalId],
    ) -> Result<(), InstructionVisitorError> {
        Ok(())
    }

    #[inline]
    fn visit_return_to_worktop(
        &mut self,
        bucket_id: &ManifestBucket,
    ) -> Result<(), InstructionVisitorError> {
        Ok(())
    }

    #[inline]
    fn visit_assert_worktop_contains(
        &mut self,
        resource_address: &ResourceAddress,
        amount: &Decimal,
    ) -> Result<(), InstructionVisitorError> {
        Ok(())
    }

    #[inline]
    fn visit_assert_worktop_contains_any(
        &mut self,
        resource_address: &ResourceAddress,
    ) -> Result<(), InstructionVisitorError> {
        Ok(())
    }

    #[inline]
    fn visit_assert_worktop_contains_non_fungibles(
        &mut self,
        resource_address: &ResourceAddress,
        ids: &[NonFungibleLocalId],
    ) -> Result<(), InstructionVisitorError> {
        Ok(())
    }

    #[inline]
    fn visit_pop_from_auth_zone(&mut self) -> Result<(), InstructionVisitorError> {
        Ok(())
    }

    #[inline]
    fn visit_push_to_auth_zone(
        &mut self,
        proof_id: &ManifestProof,
    ) -> Result<(), InstructionVisitorError> {
        Ok(())
    }

    #[inline]
    fn visit_drop_auth_zone_proofs(&mut self) -> Result<(), InstructionVisitorError> {
        Ok(())
    }

    #[inline]
    fn visit_create_proof_from_auth_zone_of_amount(
        &mut self,
        resource_address: &ResourceAddress,
        amount: &Decimal,
    ) -> Result<(), InstructionVisitorError> {
        Ok(())
    }

    #[inline]
    fn visit_create_proof_from_auth_zone_of_non_fungibles(
        &mut self,
        resource_address: &ResourceAddress,
        ids: &[NonFungibleLocalId],
    ) -> Result<(), InstructionVisitorError> {
        Ok(())
    }

    #[inline]
    fn visit_create_proof_from_auth_zone_of_all(
        &mut self,
        resource_address: &ResourceAddress,
    ) -> Result<(), InstructionVisitorError> {
        Ok(())
    }

    #[inline]
    fn visit_drop_named_proofs(&mut self) -> Result<(), InstructionVisitorError> {
        Ok(())
    }

    #[inline]
    fn visit_drop_auth_zone_signature_proofs(&mut self) -> Result<(), InstructionVisitorError> {
        Ok(())
    }

    #[inline]
    fn visit_drop_auth_zone_regular_proofs(&mut self) -> Result<(), InstructionVisitorError> {
        Ok(())
    }

    #[inline]
    fn visit_create_proof_from_bucket_of_amount(
        &mut self,
        bucket_id: &ManifestBucket,
        amount: &Decimal,
    ) -> Result<(), InstructionVisitorError> {
        Ok(())
    }

    #[inline]
    fn visit_create_proof_from_bucket_of_non_fungibles(
        &mut self,
        bucket_id: &ManifestBucket,
        ids: &[NonFungibleLocalId],
    ) -> Result<(), InstructionVisitorError> {
        Ok(())
    }

    #[inline]
    fn visit_create_proof_from_bucket_of_all(
        &mut self,
        bucket_id: &ManifestBucket,
    ) -> Result<(), InstructionVisitorError> {
        Ok(())
    }

    #[inline]
    fn visit_burn_resource(
        &mut self,
        bucket_id: &ManifestBucket,
    ) -> Result<(), InstructionVisitorError> {
        Ok(())
    }

    #[inline]
    fn visit_clone_proof(
        &mut self,
        proof_id: &ManifestProof,
    ) -> Result<(), InstructionVisitorError> {
        Ok(())
    }

    #[inline]
    fn visit_drop_proof(
        &mut self,
        proof_id: &ManifestProof,
    ) -> Result<(), InstructionVisitorError> {
        Ok(())
    }

    #[inline]
    fn visit_call_function(
        &mut self,
        package_address: &DynamicPackageAddress,
        blueprint_name: &str,
        function_name: &str,
        args: &ManifestValue,
    ) -> Result<(), InstructionVisitorError> {
        Ok(())
    }

    #[inline]
    fn visit_call_method(
        &mut self,
        address: &DynamicGlobalAddress,
        method_name: &str,
        args: &ManifestValue,
    ) -> Result<(), InstructionVisitorError> {
        Ok(())
    }

    #[inline]
    fn visit_call_royalty_method(
        &mut self,
        address: &DynamicGlobalAddress,
        method_name: &str,
        args: &ManifestValue,
    ) -> Result<(), InstructionVisitorError> {
        Ok(())
    }

    #[inline]
    fn visit_call_metadata_method(
        &mut self,
        address: &DynamicGlobalAddress,
        method_name: &str,
        args: &ManifestValue,
    ) -> Result<(), InstructionVisitorError> {
        Ok(())
    }

    #[inline]
    fn visit_call_role_assignment_method(
        &mut self,
        address: &DynamicGlobalAddress,
        method_name: &str,
        args: &ManifestValue,
    ) -> Result<(), InstructionVisitorError> {
        Ok(())
    }

    #[inline]
    fn visit_call_direct_vault_method(
        &mut self,
        vault_id: &InternalAddress,
        method_name: &str,
        args: &ManifestValue,
    ) -> Result<(), InstructionVisitorError> {
        Ok(())
    }

    #[inline]
    fn visit_drop_all_proofs(&mut self) -> Result<(), InstructionVisitorError> {
        Ok(())
    }

    #[inline]
    fn visit_allocate_global_address(
        &mut self,
        package_address: &PackageAddress,
        blueprint_name: &str,
    ) -> Result<(), InstructionVisitorError> {
        Ok(())
    }

    //========
    // Events
    //========

    #[inline]
    fn post_visit(&mut self) -> Result<(), InstructionVisitorError> {
        Ok(())
    }

    //=======
    // State
    //=======

    #[inline]
    fn is_enabled(&self) -> bool {
        true
    }
}
