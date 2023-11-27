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

use radix_engine_common::prelude::*;
use radix_engine_toolkit_json::prelude::*;
use transaction::prelude::*;

use super::traits::HasExamples;

const EXAMPLE_SIZE: usize = 28;

impl<'f> HasExamples<'f, EXAMPLE_SIZE> for AddressEntityType {
    fn example_inputs() -> [Self::Input; EXAMPLE_SIZE] {
        [
            SerializableNodeId::new(XRD.into_node_id(), 0xf2),
            SerializableNodeId::new(
                SECP256K1_SIGNATURE_VIRTUAL_BADGE.into_node_id(),
                0xf2,
            ),
            SerializableNodeId::new(
                ED25519_SIGNATURE_VIRTUAL_BADGE.into_node_id(),
                0xf2,
            ),
            SerializableNodeId::new(
                PACKAGE_OF_DIRECT_CALLER_VIRTUAL_BADGE.into_node_id(),
                0xf2,
            ),
            SerializableNodeId::new(
                GLOBAL_CALLER_VIRTUAL_BADGE.into_node_id(),
                0xf2,
            ),
            SerializableNodeId::new(
                SYSTEM_TRANSACTION_BADGE.into_node_id(),
                0xf2,
            ),
            SerializableNodeId::new(PACKAGE_OWNER_BADGE.into_node_id(), 0xf2),
            SerializableNodeId::new(VALIDATOR_OWNER_BADGE.into_node_id(), 0xf2),
            SerializableNodeId::new(ACCOUNT_OWNER_BADGE.into_node_id(), 0xf2),
            SerializableNodeId::new(IDENTITY_OWNER_BADGE.into_node_id(), 0xf2),
            SerializableNodeId::new(PACKAGE_PACKAGE.into_node_id(), 0xf2),
            SerializableNodeId::new(RESOURCE_PACKAGE.into_node_id(), 0xf2),
            SerializableNodeId::new(ACCOUNT_PACKAGE.into_node_id(), 0xf2),
            SerializableNodeId::new(IDENTITY_PACKAGE.into_node_id(), 0xf2),
            SerializableNodeId::new(
                CONSENSUS_MANAGER_PACKAGE.into_node_id(),
                0xf2,
            ),
            SerializableNodeId::new(
                ACCESS_CONTROLLER_PACKAGE.into_node_id(),
                0xf2,
            ),
            SerializableNodeId::new(POOL_PACKAGE.into_node_id(), 0xf2),
            SerializableNodeId::new(
                TRANSACTION_PROCESSOR_PACKAGE.into_node_id(),
                0xf2,
            ),
            SerializableNodeId::new(
                METADATA_MODULE_PACKAGE.into_node_id(),
                0xf2,
            ),
            SerializableNodeId::new(
                ROYALTY_MODULE_PACKAGE.into_node_id(),
                0xf2,
            ),
            SerializableNodeId::new(
                ROLE_ASSIGNMENT_MODULE_PACKAGE.into_node_id(),
                0xf2,
            ),
            SerializableNodeId::new(
                GENESIS_HELPER_PACKAGE.into_node_id(),
                0xf2,
            ),
            SerializableNodeId::new(FAUCET_PACKAGE.into_node_id(), 0xf2),
            SerializableNodeId::new(
                TRANSACTION_TRACKER_PACKAGE.into_node_id(),
                0xf2,
            ),
            SerializableNodeId::new(CONSENSUS_MANAGER.into_node_id(), 0xf2),
            SerializableNodeId::new(GENESIS_HELPER.into_node_id(), 0xf2),
            SerializableNodeId::new(FAUCET.into_node_id(), 0xf2),
            SerializableNodeId::new(TRANSACTION_TRACKER.into_node_id(), 0xf2),
        ]
    }
}

impl<'f> HasExamples<'f, EXAMPLE_SIZE> for AddressDecode {
    fn example_inputs() -> [Self::Input; EXAMPLE_SIZE] {
        [
            SerializableNodeId::new(XRD.into_node_id(), 0xf2),
            SerializableNodeId::new(
                SECP256K1_SIGNATURE_VIRTUAL_BADGE.into_node_id(),
                0xf2,
            ),
            SerializableNodeId::new(
                ED25519_SIGNATURE_VIRTUAL_BADGE.into_node_id(),
                0xf2,
            ),
            SerializableNodeId::new(
                PACKAGE_OF_DIRECT_CALLER_VIRTUAL_BADGE.into_node_id(),
                0xf2,
            ),
            SerializableNodeId::new(
                GLOBAL_CALLER_VIRTUAL_BADGE.into_node_id(),
                0xf2,
            ),
            SerializableNodeId::new(
                SYSTEM_TRANSACTION_BADGE.into_node_id(),
                0xf2,
            ),
            SerializableNodeId::new(PACKAGE_OWNER_BADGE.into_node_id(), 0xf2),
            SerializableNodeId::new(VALIDATOR_OWNER_BADGE.into_node_id(), 0xf2),
            SerializableNodeId::new(ACCOUNT_OWNER_BADGE.into_node_id(), 0xf2),
            SerializableNodeId::new(IDENTITY_OWNER_BADGE.into_node_id(), 0xf2),
            SerializableNodeId::new(PACKAGE_PACKAGE.into_node_id(), 0xf2),
            SerializableNodeId::new(RESOURCE_PACKAGE.into_node_id(), 0xf2),
            SerializableNodeId::new(ACCOUNT_PACKAGE.into_node_id(), 0xf2),
            SerializableNodeId::new(IDENTITY_PACKAGE.into_node_id(), 0xf2),
            SerializableNodeId::new(
                CONSENSUS_MANAGER_PACKAGE.into_node_id(),
                0xf2,
            ),
            SerializableNodeId::new(
                ACCESS_CONTROLLER_PACKAGE.into_node_id(),
                0xf2,
            ),
            SerializableNodeId::new(POOL_PACKAGE.into_node_id(), 0xf2),
            SerializableNodeId::new(
                TRANSACTION_PROCESSOR_PACKAGE.into_node_id(),
                0xf2,
            ),
            SerializableNodeId::new(
                METADATA_MODULE_PACKAGE.into_node_id(),
                0xf2,
            ),
            SerializableNodeId::new(
                ROYALTY_MODULE_PACKAGE.into_node_id(),
                0xf2,
            ),
            SerializableNodeId::new(
                ROLE_ASSIGNMENT_MODULE_PACKAGE.into_node_id(),
                0xf2,
            ),
            SerializableNodeId::new(
                GENESIS_HELPER_PACKAGE.into_node_id(),
                0xf2,
            ),
            SerializableNodeId::new(FAUCET_PACKAGE.into_node_id(), 0xf2),
            SerializableNodeId::new(
                TRANSACTION_TRACKER_PACKAGE.into_node_id(),
                0xf2,
            ),
            SerializableNodeId::new(CONSENSUS_MANAGER.into_node_id(), 0xf2),
            SerializableNodeId::new(GENESIS_HELPER.into_node_id(), 0xf2),
            SerializableNodeId::new(FAUCET.into_node_id(), 0xf2),
            SerializableNodeId::new(TRANSACTION_TRACKER.into_node_id(), 0xf2),
        ]
        .map(|serializable| serializable.0.to_string())
    }
}
