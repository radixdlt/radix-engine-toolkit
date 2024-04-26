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

use radix_common::prelude::ScryptoCustomSchema;
use radix_common::ScryptoSbor;
use radix_engine_toolkit::schema_visitor::core::traverser::traverse;
use radix_engine_toolkit::schema_visitor::visitors::bucket_in_path_visitor::BucketInPathVisitor;
use radix_engine_toolkit::schema_visitor::visitors::proof_in_path_visitor::ProofInPathVisitor;
use sbor::generate_full_schema_from_single_type;
use scrypto::blueprints::account::*;
use scrypto::prelude::*;
use std::collections::BTreeMap;

#[derive(ScryptoSbor)]
struct MyStruct {
    item: (u32, BTreeMap<u32, Bucket>, BTreeMap<u32, Proof>),
}

#[test]
fn bucket_in_path_visitor_can_detect_a_bucket_in_the_schema() {
    // Arrange
    let (local_type_id, schema) = generate_full_schema_from_single_type::<
        AccountDepositBatchInput,
        ScryptoCustomSchema,
    >();

    // Act
    let mut visitor = BucketInPathVisitor::default();
    traverse(&schema.v1(), local_type_id, &mut [&mut visitor]).unwrap();

    // Assert
    assert!(visitor.path_contains_bucket())
}

#[test]
fn bucket_in_path_visitor_can_detect_a_bucket_thats_nested() {
    // Arrange
    let (local_type_id, schema) =
        generate_full_schema_from_single_type::<MyStruct, ScryptoCustomSchema>(
        );

    // Act
    let mut visitor = BucketInPathVisitor::default();
    traverse(&schema.v1(), local_type_id, &mut [&mut visitor]).unwrap();

    // Assert
    assert!(visitor.path_contains_bucket())
}

#[test]
fn bucket_in_path_visitor_does_not_detect_non_existent_buckets() {
    // Arrange
    let (local_type_id, schema) = generate_full_schema_from_single_type::<
        AccountLockFeeInput,
        ScryptoCustomSchema,
    >();

    // Act
    let mut visitor = BucketInPathVisitor::default();
    traverse(&schema.v1(), local_type_id, &mut [&mut visitor]).unwrap();

    // Assert
    assert!(!visitor.path_contains_bucket())
}

#[test]
fn proof_in_path_visitor_can_detect_a_proof_in_the_schema() {
    // Arrange
    let (local_type_id, schema) = generate_full_schema_from_single_type::<
        AccountCreateProofOfAmountOutput,
        ScryptoCustomSchema,
    >();

    // Act
    let mut visitor = ProofInPathVisitor::default();
    traverse(&schema.v1(), local_type_id, &mut [&mut visitor]).unwrap();

    // Assert
    assert!(visitor.path_contains_proof())
}

#[test]
fn proof_in_path_visitor_can_detect_a_proof_thats_nested() {
    // Arrange
    let (local_type_id, schema) =
        generate_full_schema_from_single_type::<MyStruct, ScryptoCustomSchema>(
        );

    // Act
    let mut visitor = ProofInPathVisitor::default();
    traverse(&schema.v1(), local_type_id, &mut [&mut visitor]).unwrap();

    // Assert
    assert!(visitor.path_contains_proof())
}

#[test]
fn proof_in_path_visitor_does_not_detect_non_existent_proofs() {
    // Arrange
    let (local_type_id, schema) = generate_full_schema_from_single_type::<
        AccountLockFeeInput,
        ScryptoCustomSchema,
    >();

    // Act
    let mut visitor = ProofInPathVisitor::default();
    traverse(&schema.v1(), local_type_id, &mut [&mut visitor]).unwrap();

    // Assert
    assert!(!visitor.path_contains_proof())
}
