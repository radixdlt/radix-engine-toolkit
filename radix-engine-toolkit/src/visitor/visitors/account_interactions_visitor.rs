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

use std::convert::Infallible;

use crate::{utils::is_account, visitor::core::traits::InstructionVisitor};
use scrypto::{api::ObjectModuleId, prelude::*};

pub struct AccountInteractionsVisitor {
    accounts_requiring_auth: HashSet<ComponentAddress>,
    accounts_withdrawn_from: HashSet<ComponentAddress>,
    accounts_deposited_into: HashSet<ComponentAddress>,
}

impl InstructionVisitor for AccountInteractionsVisitor {
    type Error = Infallible;
    type Output = (
        HashSet<ComponentAddress>,
        HashSet<ComponentAddress>,
        HashSet<ComponentAddress>,
    );

    fn output(self) -> Self::Output {
        (
            self.accounts_requiring_auth,
            self.accounts_withdrawn_from,
            self.accounts_deposited_into,
        )
    }

    fn visit_call_method(
        &mut self,
        address: &GlobalAddress,
        method_name: &str,
        _: &ManifestValue,
    ) -> Result<(), Self::Error> {
        if is_account(address.as_node_id()) {
            let component_address = ComponentAddress::new_or_panic(address.as_node_id().0);

            if constants::ACCOUNT_METHODS_THAT_REQUIRE_AUTH
                .iter()
                .filter_map(|schema_method_key| {
                    if schema_method_key.module_id == ObjectModuleId::Main.to_u8() {
                        Some(&schema_method_key.ident)
                    } else {
                        None
                    }
                })
                .any(|ident| ident.as_str() == method_name)
            {
                self.accounts_requiring_auth.insert(component_address);
            } else if constants::ACCOUNT_DEPOSIT_METHODS.contains(&method_name.to_owned()) {
                self.accounts_deposited_into.insert(component_address);
            } else if constants::ACCOUNT_WITHDRAW_METHODS.contains(&method_name.to_owned()) {
                self.accounts_withdrawn_from.insert(component_address);
            }
        };
        Ok(())
    }

    fn visit_call_access_rules_method(
        &mut self,
        address: &GlobalAddress,
        method_name: &str,
        _: &ManifestValue,
    ) -> Result<(), Self::Error> {
        if is_account(address.as_node_id()) {
            let component_address = ComponentAddress::new_or_panic(address.as_node_id().0);

            if constants::ACCOUNT_METHODS_THAT_REQUIRE_AUTH
                .iter()
                .filter_map(|schema_method_key| {
                    if schema_method_key.module_id == ObjectModuleId::AccessRules.to_u8() {
                        Some(&schema_method_key.ident)
                    } else {
                        None
                    }
                })
                .any(|ident| ident.as_str() == method_name)
            {
                self.accounts_requiring_auth.insert(component_address);
            }
        }
        Ok(())
    }

    fn visit_call_metadata_method(
        &mut self,
        address: &GlobalAddress,
        method_name: &str,
        _: &ManifestValue,
    ) -> Result<(), Self::Error> {
        if is_account(address.as_node_id()) {
            let component_address = ComponentAddress::new_or_panic(address.as_node_id().0);

            if constants::ACCOUNT_METHODS_THAT_REQUIRE_AUTH
                .iter()
                .filter_map(|schema_method_key| {
                    if schema_method_key.module_id == ObjectModuleId::Metadata.to_u8() {
                        Some(&schema_method_key.ident)
                    } else {
                        None
                    }
                })
                .any(|ident| ident.as_str() == method_name)
            {
                self.accounts_requiring_auth.insert(component_address);
            }
        }
        Ok(())
    }

    fn visit_call_royalty_method(
        &mut self,
        address: &GlobalAddress,
        method_name: &str,
        _: &ManifestValue,
    ) -> Result<(), Self::Error> {
        if is_account(address.as_node_id()) {
            let component_address = ComponentAddress::new_or_panic(address.as_node_id().0);

            if constants::ACCOUNT_METHODS_THAT_REQUIRE_AUTH
                .iter()
                .filter_map(|schema_method_key| {
                    if schema_method_key.module_id == ObjectModuleId::Royalty.to_u8() {
                        Some(&schema_method_key.ident)
                    } else {
                        None
                    }
                })
                .any(|ident| ident.as_str() == method_name)
            {
                self.accounts_requiring_auth.insert(component_address);
            }
        }
        Ok(())
    }
}

mod constants {
    use radix_engine::blueprints::account::AccountNativePackage;
    use radix_engine_common::prelude::{
        OwnValidation, ScryptoCustomSchema, ScryptoCustomTypeKind, ScryptoCustomTypeValidation,
    };
    use sbor::{LocalTypeIndex, Schema, SchemaTypeKind, TypeValidation};
    use scrypto::blueprints::account::*;
    use scrypto::schema::{BlueprintSchema, ReceiverInfo, SchemaMethodKey, SchemaMethodPermission};

    lazy_static::lazy_static! {
        static ref ACCOUNT_BLUEPRINT_SCHEMA: BlueprintSchema = get_account_blueprint_schema();

        pub static ref ACCOUNT_METHODS_THAT_REQUIRE_AUTH: Vec<SchemaMethodKey> =
            get_methods_that_require_auth_from_schema();

        pub static ref ACCOUNT_DEPOSIT_METHODS: Vec<String> =
            get_methods_that_perform_deposit_to_account_from_schema();

        pub static ref ACCOUNT_WITHDRAW_METHODS: Vec<String> =
            get_methods_that_perform_withdraw_from_account_from_schema();
    }

    fn get_account_blueprint_schema() -> BlueprintSchema {
        AccountNativePackage::definition()
            .schema
            .blueprints
            .get(ACCOUNT_BLUEPRINT)
            .unwrap()
            .clone()
    }

    /// An account method that requires auth is a method whose method permission is not public in
    /// the schema.
    fn get_methods_that_require_auth_from_schema() -> Vec<SchemaMethodKey> {
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

    /// Methods that do deposit from the account are methods with buckets in the arguments.
    fn get_methods_that_perform_deposit_to_account_from_schema() -> Vec<String> {
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

    /// Methods that do withdraws from the account are methods with buckets in the returns and no
    /// buckets in the arguments.
    fn get_methods_that_perform_withdraw_from_account_from_schema() -> Vec<String> {
        ACCOUNT_BLUEPRINT_SCHEMA
            .functions
            .iter()
            .filter_map(|(function_ident, function_schema)| {
                // A function that doesn't have a mutable reference to self can not be a withdraw
                // method
                if function_schema.receiver != Some(ReceiverInfo::normal_ref_mut()) {
                    return None;
                }

                // Ignore the securify method, it's a special case
                if function_ident == ACCOUNT_SECURIFY_IDENT {
                    return None;
                }

                if path_contains_a_bucket(function_schema.output, &ACCOUNT_BLUEPRINT_SCHEMA.schema)
                    && !path_contains_a_bucket(
                        function_schema.input,
                        &ACCOUNT_BLUEPRINT_SCHEMA.schema,
                    )
                {
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
        let type_kind = schema.resolve_type_kind(local_type_index).unwrap();
        match type_kind {
            SchemaTypeKind::<ScryptoCustomSchema>::Any
            | SchemaTypeKind::<ScryptoCustomSchema>::Bool
            | SchemaTypeKind::<ScryptoCustomSchema>::I8
            | SchemaTypeKind::<ScryptoCustomSchema>::I16
            | SchemaTypeKind::<ScryptoCustomSchema>::I32
            | SchemaTypeKind::<ScryptoCustomSchema>::I64
            | SchemaTypeKind::<ScryptoCustomSchema>::I128
            | SchemaTypeKind::<ScryptoCustomSchema>::U8
            | SchemaTypeKind::<ScryptoCustomSchema>::U16
            | SchemaTypeKind::<ScryptoCustomSchema>::U32
            | SchemaTypeKind::<ScryptoCustomSchema>::U64
            | SchemaTypeKind::<ScryptoCustomSchema>::U128
            | SchemaTypeKind::<ScryptoCustomSchema>::String => false,
            SchemaTypeKind::<ScryptoCustomSchema>::Array { element_type } => {
                path_contains_a_bucket(*element_type, schema)
            }
            SchemaTypeKind::<ScryptoCustomSchema>::Tuple { field_types } => {
                for field_type in field_types {
                    let contains_bucket = path_contains_a_bucket(*field_type, schema);
                    if contains_bucket {
                        return true;
                    }
                }
                false
            }
            SchemaTypeKind::<ScryptoCustomSchema>::Enum { variants } => {
                #[allow(clippy::for_kv_map)]
                for (_, local_type_indices) in variants {
                    for local_type_index in local_type_indices {
                        let contains_bucket = path_contains_a_bucket(*local_type_index, schema);
                        if contains_bucket {
                            return true;
                        }
                    }
                }
                false
            }
            SchemaTypeKind::<ScryptoCustomSchema>::Map {
                key_type,
                value_type,
            } => {
                path_contains_a_bucket(*key_type, schema)
                    || path_contains_a_bucket(*value_type, schema)
            }
            SchemaTypeKind::<ScryptoCustomSchema>::Custom(ScryptoCustomTypeKind::Own) => {
                let type_validation = schema.resolve_type_validation(local_type_index).unwrap();
                matches!(
                    type_validation,
                    TypeValidation::<ScryptoCustomTypeValidation>::Custom(
                        ScryptoCustomTypeValidation::Own(OwnValidation::IsBucket),
                    )
                )
            }
            SchemaTypeKind::<ScryptoCustomSchema>::Custom(_) => false,
        }
    }
}
