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
use radix_engine::blueprints::package::*;
use radix_engine::types::*;
use scrypto::api::node_modules::auth::*;
use scrypto::api::node_modules::royalty::*;
use scrypto::blueprints::account::*;
use scrypto::blueprints::identity::*;

use crate::schema_visitor::core::traverser::traverse;
use crate::schema_visitor::visitors::bucket_in_path_visitor::BucketInPathVisitor;
use crate::schema_visitor::visitors::proof_in_path_visitor::ProofInPathVisitor;

lazy_static! {
    // Account package
    pub static ref ACCOUNT_BLUEPRINT_SCHEMA: BlueprintDefinitionInit = account_blueprint_schema();

    pub static ref ACCOUNT_METHODS_THAT_REQUIRE_AUTH: Vec<MethodKey> = account_methods_that_require_auth();

    pub static ref ACCOUNT_DEPOSIT_METHODS: Vec<String> = account_deposit_methods();

    pub static ref ACCOUNT_WITHDRAW_METHODS: Vec<String> = account_withdraw_methods();

    pub static ref ACCOUNT_PROOF_CREATION_METHODS: Vec<String> = account_proof_creation_methods();

    // Identity Package
    pub static ref IDENTITY_BLUEPRINT_SCHEMA: BlueprintDefinitionInit = identity_blueprint_schema();

    pub static ref IDENTITY_METHODS_THAT_REQUIRE_AUTH: Vec<MethodKey> = identity_methods_that_require_auth();

    // Modules Package
    pub static ref ROLE_ASSIGNMENT_BLUEPRINT_SCHEMA: BlueprintDefinitionInit = role_assignment_blueprint_schema();
    pub static ref METADATA_BLUEPRINT_SCHEMA: BlueprintDefinitionInit = metadata_blueprint_schema();
    pub static ref ROYALTY_BLUEPRINT_SCHEMA: BlueprintDefinitionInit = royalty_blueprint_schema();

    pub static ref ROLE_ASSIGNMENT_METHODS_THAT_REQUIRE_AUTH: Vec<MethodKey> = role_assignment_methods_that_require_auth();
    pub static ref METADATA_METHODS_THAT_REQUIRE_AUTH: Vec<MethodKey> = metadata_methods_that_require_auth();
    pub static ref ROYALTY_METHODS_THAT_REQUIRE_AUTH: Vec<MethodKey> = royalty_methods_that_require_auth();
}

fn account_blueprint_schema() -> BlueprintDefinitionInit {
    ACCOUNT_PACKAGE_DEFINITION
        .blueprints
        .get(ACCOUNT_BLUEPRINT)
        .expect("Account package has no schema for the account blueprint?")
        .clone()
}

fn account_methods_that_require_auth() -> Vec<MethodKey> {
    methods_that_require_auth(&ACCOUNT_BLUEPRINT_SCHEMA)
}

fn account_deposit_methods() -> Vec<String> {
    ACCOUNT_BLUEPRINT_SCHEMA
        .schema
        .functions
        .functions
        .iter()
        .filter_map(|(function_ident, function_schema)| {
            // A function that doesn't have a mutable reference to self can not
            // be a withdraw method
            if function_schema.receiver != Some(ReceiverInfo::normal_ref_mut())
            {
                return None;
            }

            let local_type_id =
                type_ref_static_or_panic(&function_schema.input);
            if path_contains_a_bucket(
                *local_type_id,
                &ACCOUNT_BLUEPRINT_SCHEMA.schema.schema,
            ) {
                Some(function_ident.to_owned())
            } else {
                None
            }
        })
        .collect::<Vec<String>>()
}

fn account_withdraw_methods() -> Vec<String> {
    ACCOUNT_BLUEPRINT_SCHEMA
        .schema
        .functions
        .functions
        .iter()
        .filter_map(|(function_ident, function_schema)| {
            if function_schema.receiver != Some(ReceiverInfo::normal_ref_mut())
            {
                return None;
            }

            if function_ident == ACCOUNT_SECURIFY_IDENT {
                return None;
            }

            if path_contains_a_bucket(
                *type_ref_static_or_panic(&function_schema.output),
                &ACCOUNT_BLUEPRINT_SCHEMA.schema.schema,
            ) && !path_contains_a_bucket(
                *type_ref_static_or_panic(&function_schema.input),
                &ACCOUNT_BLUEPRINT_SCHEMA.schema.schema,
            ) {
                Some(function_ident.to_owned())
            } else {
                None
            }
        })
        .collect::<Vec<String>>()
}

fn account_proof_creation_methods() -> Vec<String> {
    ACCOUNT_BLUEPRINT_SCHEMA
        .schema
        .functions
        .functions
        .iter()
        .filter_map(|(function_ident, function_schema)| {
            if function_schema.receiver != Some(ReceiverInfo::normal_ref()) {
                return None;
            }

            if path_contains_a_proof(
                *type_ref_static_or_panic(&function_schema.output),
                &ACCOUNT_BLUEPRINT_SCHEMA.schema.schema,
            ) {
                Some(function_ident.to_owned())
            } else {
                None
            }
        })
        .collect::<Vec<String>>()
}

fn path_contains_a_bucket(
    local_type_id: LocalTypeId,
    schema: &VersionedScryptoSchema,
) -> bool {
    let VersionedScryptoSchema::V1(schema) = schema;
    let mut visitor = BucketInPathVisitor::default();
    traverse(schema, local_type_id, &mut [&mut visitor]).unwrap();
    visitor.path_contains_bucket()
}

fn path_contains_a_proof(
    local_type_id: LocalTypeId,
    schema: &VersionedScryptoSchema,
) -> bool {
    let VersionedScryptoSchema::V1(schema) = schema;
    let mut visitor = ProofInPathVisitor::default();
    traverse(schema, local_type_id, &mut [&mut visitor]).unwrap();
    visitor.path_contains_proof()
}

fn identity_blueprint_schema() -> BlueprintDefinitionInit {
    IDENTITY_PACKAGE_DEFINITION
        .blueprints
        .get(IDENTITY_BLUEPRINT)
        .expect("Identity package has no schema for the identity blueprint?")
        .clone()
}

fn identity_methods_that_require_auth() -> Vec<MethodKey> {
    methods_that_require_auth(&IDENTITY_BLUEPRINT_SCHEMA)
}

fn role_assignment_blueprint_schema() -> BlueprintDefinitionInit {
    ROLE_ASSIGNMENT_PACKAGE_DEFINITION
        .blueprints
        .get(ROLE_ASSIGNMENT_BLUEPRINT)
        .expect("RoleAssignment package has no schema for the RoleAssignment blueprint?")
        .clone()
}

fn metadata_blueprint_schema() -> BlueprintDefinitionInit {
    METADATA_PACKAGE_DEFINITION
        .blueprints
        .get(METADATA_BLUEPRINT)
        .expect("Metadata package has no schema for the Metadata blueprint?")
        .clone()
}

fn royalty_blueprint_schema() -> BlueprintDefinitionInit {
    ROYALTY_PACKAGE_DEFINITION
        .blueprints
        .get(COMPONENT_ROYALTY_BLUEPRINT)
        .expect("ComponentRoyalty package has no schema for the ComponentRoyalty blueprint?")
        .clone()
}

fn role_assignment_methods_that_require_auth() -> Vec<MethodKey> {
    methods_that_require_auth(&ROLE_ASSIGNMENT_BLUEPRINT_SCHEMA)
}

fn metadata_methods_that_require_auth() -> Vec<MethodKey> {
    methods_that_require_auth(&METADATA_BLUEPRINT_SCHEMA)
}

fn royalty_methods_that_require_auth() -> Vec<MethodKey> {
    methods_that_require_auth(&ROYALTY_BLUEPRINT_SCHEMA)
}

fn methods_that_require_auth(
    blueprint_schema: &BlueprintDefinitionInit,
) -> Vec<MethodKey> {
    if let MethodAuthTemplate::StaticRoleDefinition(StaticRoleDefinition {
        ref methods,
        ..
    }) = blueprint_schema.auth_config.method_auth
    {
        methods
            .iter()
            .filter_map(|(key, value)| {
                if let MethodAccessibility::Public = value {
                    None
                } else {
                    Some(key.clone())
                }
            })
            .collect()
    } else {
        vec![]
    }
}

fn type_ref_static_or_panic<T>(type_ref: &TypeRef<T>) -> &T {
    match type_ref {
        TypeRef::Static(item) => item,
        TypeRef::Generic(_) => panic!("TypeRef is not static!"),
    }
}
