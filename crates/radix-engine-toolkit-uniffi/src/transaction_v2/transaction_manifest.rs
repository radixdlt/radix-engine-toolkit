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

use radix_engine_toolkit_common::receipt::{
    RuntimeToolkitTransactionReceipt, SerializableToolkitTransactionReceipt,
};
use sbor::Versioned;

use crate::prelude::*;

#[derive(Clone, Debug, Object)]
pub struct TransactionManifestV2 {
    pub instructions: Arc<InstructionsV2>,
    pub blobs: Vec<Vec<u8>>,
    pub children: Vec<Arc<Hash>>,
}

#[uniffi::export]
impl TransactionManifestV2 {
    #[uniffi::constructor]
    pub fn new(
        instructions: Arc<InstructionsV2>,
        blobs: Vec<Vec<u8>>,
        children: Vec<Arc<Hash>>,
    ) -> Arc<Self> {
        Arc::new(Self {
            instructions,
            blobs,
            children,
        })
    }

    pub fn instructions(&self) -> Arc<InstructionsV2> {
        self.instructions.clone()
    }

    pub fn blobs(&self) -> Vec<Vec<u8>> {
        self.blobs.clone()
    }

    pub fn to_payload_bytes(&self) -> Result<Vec<u8>> {
        let native = self.clone().to_native();
        Ok(core_transaction_v2_transaction_manifest_to_payload_bytes(
            &native,
        )?)
    }

    #[uniffi::constructor]
    pub fn from_payload_bytes(
        compiled: Vec<u8>,
        network_id: u8,
    ) -> Result<Arc<Self>> {
        let decompiled =
            core_transaction_v2_transaction_manifest_from_payload_bytes(
                compiled,
            )
            .map_err(|error| {
                RadixEngineToolkitError::ManifestSborError { error }
            })?;
        Ok(Arc::new(Self::from_native(&decompiled, network_id)))
    }

    pub fn extract_addresses(&self) -> HashMap<EntityType, Vec<Arc<Address>>> {
        let network_id = self.instructions.1;
        let (addresses, _) = core_transaction_v2_instructions_extract_addresses(
            &self.instructions.0,
        );

        let mut map = HashMap::<EntityType, Vec<Arc<Address>>>::new();
        for address in addresses {
            let entity_type = EntityType::from(address.entity_type());
            let address =
                Arc::new(Address::from_typed_node_id(address, network_id));
            map.entry(entity_type).or_default().push(address);
        }
        map
    }

    pub fn statically_analyze(&self, network_id: u8) -> StaticAnalysis {
        let native = self.clone().to_native();
        StaticAnalysis::from_native(
            core_transaction_v2_transaction_manifest_statically_analyze(
                &native,
            ),
            network_id,
        )
    }

    pub fn statically_analyze_and_validate(
        &self,
        network_id: u8,
    ) -> Result<StaticAnalysisWithResourceMovements> {
        let native = self.clone().to_native();
        core_transaction_v2_transaction_manifest_statically_analyze_and_validate(&native)
            .map_err(RadixEngineToolkitError::from)
            .map(|static_analysis| {
                StaticAnalysisWithResourceMovements::from_native(
                    static_analysis,
                    network_id,
                )
            })
    }

    pub fn dynamically_analyze(
        &self,
        network_id: u8,
        toolkit_receipt: String,
    ) -> Result<DynamicAnalysis> {
        let native = self.clone().to_native();
        let network_definition =
            NativeNetworkDefinition::from_network_id(network_id);
        let receipt = serde_json::from_str::<
            SerializableToolkitTransactionReceipt,
        >(&toolkit_receipt)
        .ok()
        .and_then(|receipt| {
            receipt
                .into_runtime_receipt(&NativeAddressBech32Decoder::new(
                    &network_definition,
                ))
                .ok()
        })
        .ok_or(RadixEngineToolkitError::InvalidReceipt)?;
        core_transaction_v2_transaction_manifest_dynamically_analyze(
            &native, &receipt,
        )
        .map_err(|_| RadixEngineToolkitError::InvalidReceipt)
        .map(|summary| DynamicAnalysis::from_native(summary, network_id))?
    }

    pub fn statically_validate(&self) -> Result<()> {
        core_transaction_v2_transaction_manifest_statically_validate(
            &self.clone().to_native(),
        )
        .map_err(Into::into)
    }
}

impl TransactionManifestV2 {
    pub fn from_native(
        NativeTransactionManifestV2 {
            instructions,
            blobs,
            children,
            ..
        }: &NativeTransactionManifestV2,
        network_id: u8,
    ) -> Self {
        let blobs = blobs.iter().map(|(_, v)| v.clone()).collect::<Vec<_>>();
        let instructions = InstructionsV2(instructions.clone(), network_id);
        Self {
            instructions: Arc::new(instructions),
            blobs,
            children: children
                .iter()
                .map(|hash| Arc::new(Hash(hash.hash.0)))
                .collect(),
        }
    }

    pub fn to_native(&self) -> NativeTransactionManifestV2 {
        let blobs = self
            .blobs
            .iter()
            .map(|blob| (native_hash(blob), blob.clone()))
            .collect::<IndexMap<_, _>>();
        let instructions = self.instructions.0.clone();

        NativeTransactionManifestV2 {
            instructions,
            blobs,
            children: self
                .children
                .iter()
                .map(|value| NativeSubintentHash(value.as_ref().0))
                .map(|value| NativeChildSubintentSpecifier { hash: value })
                .collect(),
            object_names: Default::default(),
        }
    }
}
