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

use super::traits::HasExamples;
use radix_engine_toolkit::prelude::*;
use scrypto::prelude::*;

impl<'f> HasExamples<'f> for SerializableNodeId {
    fn examples() -> Vec<Self> {
        [
            XRD.as_node_id(),
            SECP256K1_SIGNATURE_VIRTUAL_BADGE.as_node_id(),
            ED25519_SIGNATURE_VIRTUAL_BADGE.as_node_id(),
            PACKAGE_OF_DIRECT_CALLER_VIRTUAL_BADGE.as_node_id(),
            GLOBAL_CALLER_VIRTUAL_BADGE.as_node_id(),
            SYSTEM_TRANSACTION_BADGE.as_node_id(),
            PACKAGE_OWNER_BADGE.as_node_id(),
            VALIDATOR_OWNER_BADGE.as_node_id(),
            ACCOUNT_OWNER_BADGE.as_node_id(),
            IDENTITY_OWNER_BADGE.as_node_id(),
            PACKAGE_PACKAGE.as_node_id(),
            RESOURCE_PACKAGE.as_node_id(),
            ACCOUNT_PACKAGE.as_node_id(),
            IDENTITY_PACKAGE.as_node_id(),
            CONSENSUS_MANAGER_PACKAGE.as_node_id(),
            ACCESS_CONTROLLER_PACKAGE.as_node_id(),
            POOL_PACKAGE.as_node_id(),
            TRANSACTION_PROCESSOR_PACKAGE.as_node_id(),
            METADATA_MODULE_PACKAGE.as_node_id(),
            ROYALTY_MODULE_PACKAGE.as_node_id(),
            ROLE_ASSIGNMENT_MODULE_PACKAGE.as_node_id(),
            GENESIS_HELPER_PACKAGE.as_node_id(),
            FAUCET_PACKAGE.as_node_id(),
            CONSENSUS_MANAGER.as_node_id(),
            GENESIS_HELPER.as_node_id(),
            FAUCET.as_node_id(),
        ]
        .into_iter()
        .map(|node_id| {
            SerializableNodeId(SerializableNodeIdInternal {
                network_id: 0x01,
                node_id: *node_id,
            })
        })
        .collect()
    }
}
