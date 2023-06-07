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

use super::error::Error;
use scrypto::prelude::*;

pub trait InstructionVisitor {
    type Error: Debug + Into<Error>;
    type Output: Sized;

    //==============
    // Instructions
    //==============

    #[inline]
    fn visit_take_all_from_worktop(
        &mut self,
        resource_address: &ResourceAddress,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    #[inline]
    fn visit_take_from_worktop(
        &mut self,
        resource_address: &ResourceAddress,
        amount: &Decimal,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    #[inline]
    fn visit_take_non_fungibles_from_worktop(
        &mut self,
        resource_address: &ResourceAddress,
        ids: &[NonFungibleLocalId],
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    #[inline]
    fn visit_return_to_worktop(&mut self, bucket_id: &ManifestBucket) -> Result<(), Self::Error> {
        Ok(())
    }

    #[inline]
    fn visit_assert_worktop_contains(
        &mut self,
        resource_address: &ResourceAddress,
        amount: &Decimal,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    #[inline]
    fn visit_assert_worktop_contains_non_fungibles(
        &mut self,
        resource_address: &ResourceAddress,
        ids: &[NonFungibleLocalId],
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    #[inline]
    fn visit_pop_from_auth_zone(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    #[inline]
    fn visit_push_to_auth_zone(&mut self, proof_id: &ManifestProof) -> Result<(), Self::Error> {
        Ok(())
    }

    #[inline]
    fn visit_clear_auth_zone(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    #[inline]
    fn visit_create_proof_from_auth_zone(
        &mut self,
        resource_address: &ResourceAddress,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    #[inline]
    fn visit_create_proof_from_auth_zone_of_amount(
        &mut self,
        resource_address: &ResourceAddress,
        amount: &Decimal,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    #[inline]
    fn visit_create_proof_from_auth_zone_of_non_fungibles(
        &mut self,
        resource_address: &ResourceAddress,
        ids: &[NonFungibleLocalId],
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    #[inline]
    fn visit_create_proof_from_auth_zone_of_all(
        &mut self,
        resource_address: &ResourceAddress,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    #[inline]
    fn visit_clear_signature_proofs(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    #[inline]
    fn visit_create_proof_from_bucket(
        &mut self,
        bucket_id: &ManifestBucket,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    #[inline]
    fn visit_create_proof_from_bucket_of_amount(
        &mut self,
        bucket_id: &ManifestBucket,
        amount: &Decimal,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    #[inline]
    fn visit_create_proof_from_bucket_of_non_fungibles(
        &mut self,
        bucket_id: &ManifestBucket,
        ids: &[NonFungibleLocalId],
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    #[inline]
    fn visit_create_proof_from_bucket_of_all(
        &mut self,
        bucket_id: &ManifestBucket,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    #[inline]
    fn visit_burn_resource(&mut self, bucket_id: &ManifestBucket) -> Result<(), Self::Error> {
        Ok(())
    }

    #[inline]
    fn visit_clone_proof(&mut self, proof_id: &ManifestProof) -> Result<(), Self::Error> {
        Ok(())
    }

    #[inline]
    fn visit_drop_proof(&mut self, proof_id: &ManifestProof) -> Result<(), Self::Error> {
        Ok(())
    }

    #[inline]
    fn visit_call_function(
        &mut self,
        package_address: &PackageAddress,
        blueprint_name: &str,
        function_name: &str,
        args: &ManifestValue,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    #[inline]
    fn visit_call_method(
        &mut self,
        address: &GlobalAddress,
        method_name: &str,
        args: &ManifestValue,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    #[inline]
    fn visit_call_royalty_method(
        &mut self,
        address: &GlobalAddress,
        method_name: &str,
        args: &ManifestValue,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    #[inline]
    fn visit_call_metadata_method(
        &mut self,
        address: &GlobalAddress,
        method_name: &str,
        args: &ManifestValue,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    #[inline]
    fn visit_call_access_rules_method(
        &mut self,
        address: &GlobalAddress,
        method_name: &str,
        args: &ManifestValue,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    #[inline]
    fn visit_call_direct_vault_method(
        &mut self,
        vault_id: &InternalAddress,
        method_name: &str,
        args: &ManifestValue,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    #[inline]
    fn visit_drop_all_proofs(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    //========
    // Events
    //========

    #[inline]
    fn post_visit(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    //=======
    // State
    //=======

    fn output(self) -> Self::Output;

    #[inline]
    fn is_enabled(&self) -> bool {
        true
    }
}
