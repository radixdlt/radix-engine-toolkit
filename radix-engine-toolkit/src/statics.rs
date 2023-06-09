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

use lazy_static::lazy_static;
use radix_engine::blueprints::native_schema::*;
use radix_engine_common::prelude::ScryptoCustomSchema;
use sbor::*;
use scrypto::blueprints::account::*;
use scrypto::blueprints::identity::IDENTITY_BLUEPRINT;
use scrypto::schema::*;

use crate::schema_visitor::core::traverser::traverse;
use crate::schema_visitor::visitors::bucket_in_path_visitor::BucketInPathVisitor;
use crate::schema_visitor::visitors::proof_in_path_visitor::ProofInPathVisitor;

lazy_static! {
    // Account package
    pub static ref ACCOUNT_BLUEPRINT_SCHEMA: BlueprintSchema = account_blueprint_schema();

    pub static ref ACCOUNT_METHODS_THAT_REQUIRE_AUTH: Vec<SchemaMethodKey> = account_methods_that_require_auth();

    pub static ref ACCOUNT_DEPOSIT_METHODS: Vec<String> = account_deposit_methods();

    pub static ref ACCOUNT_WITHDRAW_METHODS: Vec<String> = account_withdraw_methods();

    pub static ref ACCOUNT_PROOF_CREATION_METHODS: Vec<String> = account_proof_creation_methods();

    // Identity Package
    pub static ref IDENTITY_BLUEPRINT_SCHEMA: BlueprintSchema = identity_blueprint_schema();

    pub static ref IDENTITY_METHODS_THAT_REQUIRE_AUTH: Vec<SchemaMethodKey> = identity_methods_that_require_auth();
}

fn account_blueprint_schema() -> BlueprintSchema {
    ACCOUNT_PACKAGE_DEFINITION
        .schema
        .blueprints
        .get(ACCOUNT_BLUEPRINT)
        .unwrap()
        .clone()
}

fn account_methods_that_require_auth() -> Vec<SchemaMethodKey> {
    ACCOUNT_BLUEPRINT_SCHEMA
        .method_auth_template
        .iter()
        .filter_map(|(key, value)| {
            if let SchemaMethodPermission::Public = value {
                None
            } else {
                Some(key.clone())
            }
        })
        .collect()
}

fn account_deposit_methods() -> Vec<String> {
    ACCOUNT_BLUEPRINT_SCHEMA
        .functions
        .iter()
        .filter_map(|(function_ident, function_schema)| {
            // A function that doesn't have a mutable reference to self can not be a withdraw
            // method
            if function_schema.receiver != Some(ReceiverInfo::normal_ref_mut()) {
                return None;
            }

            let local_type_index = function_schema.input;
            if path_contains_a_bucket(local_type_index, &ACCOUNT_BLUEPRINT_SCHEMA.schema) {
                Some(function_ident.to_owned())
            } else {
                None
            }
        })
        .collect::<Vec<String>>()
}

fn account_withdraw_methods() -> Vec<String> {
    ACCOUNT_BLUEPRINT_SCHEMA
        .functions
        .iter()
        .filter_map(|(function_ident, function_schema)| {
            if function_schema.receiver != Some(ReceiverInfo::normal_ref_mut()) {
                return None;
            }

            if function_ident == ACCOUNT_SECURIFY_IDENT {
                return None;
            }

            if path_contains_a_bucket(function_schema.output, &ACCOUNT_BLUEPRINT_SCHEMA.schema)
                && !path_contains_a_bucket(function_schema.input, &ACCOUNT_BLUEPRINT_SCHEMA.schema)
            {
                Some(function_ident.to_owned())
            } else {
                None
            }
        })
        .collect::<Vec<String>>()
}

fn account_proof_creation_methods() -> Vec<String> {
    ACCOUNT_BLUEPRINT_SCHEMA
        .functions
        .iter()
        .filter_map(|(function_ident, function_schema)| {
            if function_schema.receiver != Some(ReceiverInfo::normal_ref_mut()) {
                return None;
            }

            if path_contains_a_proof(function_schema.output, &ACCOUNT_BLUEPRINT_SCHEMA.schema) {
                Some(function_ident.to_owned())
            } else {
                None
            }
        })
        .collect::<Vec<String>>()
}

fn path_contains_a_bucket(
    local_type_index: LocalTypeIndex,
    schema: &Schema<ScryptoCustomSchema>,
) -> bool {
    let mut visitor = BucketInPathVisitor::default();
    traverse(schema, local_type_index, &mut [&mut visitor]).unwrap();
    visitor.path_contains_bucket()
}

fn path_contains_a_proof(
    local_type_index: LocalTypeIndex,
    schema: &Schema<ScryptoCustomSchema>,
) -> bool {
    let mut visitor = ProofInPathVisitor::default();
    traverse(schema, local_type_index, &mut [&mut visitor]).unwrap();
    visitor.path_contains_proof()
}

fn identity_blueprint_schema() -> BlueprintSchema {
    IDENTITY_PACKAGE_DEFINITION
        .schema
        .blueprints
        .get(IDENTITY_BLUEPRINT)
        .unwrap()
        .clone()
}

fn identity_methods_that_require_auth() -> Vec<SchemaMethodKey> {
    IDENTITY_BLUEPRINT_SCHEMA
        .method_auth_template
        .iter()
        .filter_map(|(key, value)| {
            if let SchemaMethodPermission::Public = value {
                None
            } else {
                Some(key.clone())
            }
        })
        .collect()
}
