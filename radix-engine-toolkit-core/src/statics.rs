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

use scrypto::api::node_modules::auth::*;
use scrypto::api::node_modules::metadata::*;
use scrypto::api::node_modules::royalty::*;
use scrypto::blueprints::account::*;
use scrypto::blueprints::identity::*;

pub const ACCOUNT_METHODS_THAT_REQUIRE_AUTH: [&str; 16] = [
    ACCOUNT_SECURIFY_IDENT,
    ACCOUNT_LOCK_FEE_IDENT,
    ACCOUNT_LOCK_CONTINGENT_FEE_IDENT,
    ACCOUNT_DEPOSIT_IDENT,
    ACCOUNT_DEPOSIT_BATCH_IDENT,
    ACCOUNT_WITHDRAW_IDENT,
    ACCOUNT_WITHDRAW_NON_FUNGIBLES_IDENT,
    ACCOUNT_LOCK_FEE_AND_WITHDRAW_IDENT,
    ACCOUNT_LOCK_FEE_AND_WITHDRAW_NON_FUNGIBLES_IDENT,
    ACCOUNT_CREATE_PROOF_IDENT,
    ACCOUNT_CREATE_PROOF_OF_AMOUNT_IDENT,
    ACCOUNT_CREATE_PROOF_OF_NON_FUNGIBLES_IDENT,
    ACCOUNT_CHANGE_DEFAULT_DEPOSIT_RULE_IDENT,
    ACCOUNT_CONFIGURE_RESOURCE_DEPOSIT_RULE_IDENT,
    ACCOUNT_BURN_IDENT,
    ACCOUNT_BURN_NON_FUNGIBLES_IDENT,
];
pub const ACCOUNT_DEPOSIT_METHODS: [&str; 6] = [
    ACCOUNT_DEPOSIT_IDENT,
    ACCOUNT_DEPOSIT_BATCH_IDENT,
    ACCOUNT_TRY_DEPOSIT_OR_REFUND_IDENT,
    ACCOUNT_TRY_DEPOSIT_BATCH_OR_REFUND_IDENT,
    ACCOUNT_TRY_DEPOSIT_OR_ABORT_IDENT,
    ACCOUNT_TRY_DEPOSIT_BATCH_OR_ABORT_IDENT,
];
pub const ACCOUNT_WITHDRAW_METHODS: [&str; 4] = [
    ACCOUNT_WITHDRAW_IDENT,
    ACCOUNT_WITHDRAW_NON_FUNGIBLES_IDENT,
    ACCOUNT_LOCK_FEE_AND_WITHDRAW_IDENT,
    ACCOUNT_LOCK_FEE_AND_WITHDRAW_NON_FUNGIBLES_IDENT,
];
pub const ACCOUNT_PROOF_CREATION_METHODS: [&str; 3] = [
    ACCOUNT_CREATE_PROOF_IDENT,
    ACCOUNT_CREATE_PROOF_OF_AMOUNT_IDENT,
    ACCOUNT_CREATE_PROOF_OF_NON_FUNGIBLES_IDENT,
];
pub const IDENTITY_METHODS_THAT_REQUIRE_AUTH: [&str; 1] = [IDENTITY_SECURIFY_IDENT];

pub const ACCESS_RULES_METHODS_THAT_REQUIRE_AUTH: [&str; 3] = [
    ACCESS_RULES_SET_AUTHORITY_RULE_AND_MUTABILITY_IDENT,
    ACCESS_RULES_SET_AUTHORITY_RULE_IDENT,
    ACCESS_RULES_SET_AUTHORITY_MUTABILITY_IDENT,
];
pub const ROYALTY_METHODS_THAT_REQUIRE_AUTH: [&str; 2] = [
    COMPONENT_ROYALTY_SET_ROYALTY_CONFIG_IDENT,
    COMPONENT_ROYALTY_CLAIM_ROYALTY_IDENT,
];
pub const METADATA_METHODS_THAT_REQUIRE_AUTH: [&str; 2] =
    [METADATA_SET_IDENT, METADATA_REMOVE_IDENT];
