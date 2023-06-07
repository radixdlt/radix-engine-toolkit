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

use crate::{utils::is_identity, visitor::core::traits::InstructionVisitor};
use scrypto::{api::ObjectModuleId, prelude::*};

pub struct IdentityInteractionsVisitor(HashSet<ComponentAddress>);

impl InstructionVisitor for IdentityInteractionsVisitor {
    type Error = Infallible;
    type Output = HashSet<ComponentAddress>;

    fn output(self) -> Self::Output {
        self.0
    }

    fn visit_call_method(
        &mut self,
        address: &GlobalAddress,
        method_name: &str,
        _: &ManifestValue,
    ) -> Result<(), Self::Error> {
        if is_identity(address.as_node_id()) {
            let component_address = ComponentAddress::new_or_panic(address.as_node_id().0);

            if constants::IDENTITY_METHODS_THAT_REQUIRE_AUTH
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
                self.0.insert(component_address);
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
        if is_identity(address.as_node_id()) {
            let component_address = ComponentAddress::new_or_panic(address.as_node_id().0);

            if constants::IDENTITY_METHODS_THAT_REQUIRE_AUTH
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
                self.0.insert(component_address);
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
        if is_identity(address.as_node_id()) {
            let component_address = ComponentAddress::new_or_panic(address.as_node_id().0);

            if constants::IDENTITY_METHODS_THAT_REQUIRE_AUTH
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
                self.0.insert(component_address);
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
        if is_identity(address.as_node_id()) {
            let component_address = ComponentAddress::new_or_panic(address.as_node_id().0);

            if constants::IDENTITY_METHODS_THAT_REQUIRE_AUTH
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
                self.0.insert(component_address);
            }
        }
        Ok(())
    }
}

mod constants {
    use radix_engine::blueprints::identity::IdentityNativePackage;
    use scrypto::blueprints::identity::*;
    use scrypto::schema::{BlueprintSchema, SchemaMethodKey, SchemaMethodPermission};

    lazy_static::lazy_static! {
        static ref IDENTITY_BLUEPRINT_SCHEMA: BlueprintSchema = get_identity_blueprint_schema();

        pub static ref IDENTITY_METHODS_THAT_REQUIRE_AUTH: Vec<SchemaMethodKey> =
            get_methods_that_require_auth_from_schema();
    }

    fn get_identity_blueprint_schema() -> BlueprintSchema {
        IdentityNativePackage::definition()
            .schema
            .blueprints
            .get(IDENTITY_BLUEPRINT)
            .unwrap()
            .clone()
    }

    /// An identity method that requires auth is a method whose method permission is not public in
    /// the schema.
    fn get_methods_that_require_auth_from_schema() -> Vec<SchemaMethodKey> {
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
}
