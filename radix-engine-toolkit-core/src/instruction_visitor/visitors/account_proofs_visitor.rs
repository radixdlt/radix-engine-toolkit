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

use sbor::prelude::*;
use scrypto::prelude::*;
use transaction::prelude::DynamicGlobalAddress;

use crate::instruction_visitor::core::error::InstructionVisitorError;
use crate::instruction_visitor::core::traits::InstructionVisitor;
use crate::sbor::indexed_manifest_value::IndexedManifestValue;
use crate::statics::ACCOUNT_PROOF_CREATION_METHODS;
use crate::utils::is_account;

#[derive(Default, Clone)]
pub struct AccountProofsVisitor(HashSet<ResourceAddress>);

impl AccountProofsVisitor {
    pub fn output(self) -> HashSet<ResourceAddress> {
        self.0
    }
}

impl InstructionVisitor for AccountProofsVisitor {
    fn visit_call_method(
        &mut self,
        address: &DynamicGlobalAddress,
        method_name: &str,
        args: &ManifestValue,
    ) -> Result<(), InstructionVisitorError> {
        if is_account(address) && ACCOUNT_PROOF_CREATION_METHODS.contains(&method_name.to_owned()) {
            self.0.extend(
                IndexedManifestValue::from_manifest_value(args)
                    .static_addresses()
                    .iter()
                    .filter_map(|node_id| ResourceAddress::try_from(*node_id).ok()),
            )
        }
        Ok(())
    }
}
