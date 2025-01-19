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
pub struct TransactionManifestV1 {
    pub instructions: Arc<InstructionsV1>,
    pub blobs: Vec<Vec<u8>>,
}

#[uniffi::export]
impl TransactionManifestV1 {
    #[uniffi::constructor]
    pub fn new(
        instructions: Arc<InstructionsV1>,
        blobs: Vec<Vec<u8>>,
    ) -> Arc<Self> {
        Arc::new(Self {
            instructions,
            blobs,
        })
    }

    pub fn instructions(&self) -> Arc<InstructionsV1> {
        self.instructions.clone()
    }

    pub fn blobs(&self) -> Vec<Vec<u8>> {
        self.blobs.clone()
    }

    pub fn to_payload_bytes(&self) -> Result<Vec<u8>> {
        let native = self.clone().to_native();
        Ok(
            toolkit::functions::transaction_v1::manifest::to_payload_bytes(
                &native,
            )?,
        )
    }

    #[uniffi::constructor]
    pub fn from_payload_bytes(
        compiled: Vec<u8>,
        network_id: u8,
    ) -> Result<Arc<Self>> {
        let decompiled =
            toolkit::functions::transaction_v1::manifest::from_payload_bytes(
                compiled,
            )
            .map_err(|error| {
                RadixEngineToolkitError::ManifestSborError { error }
            })?;
        Ok(Arc::new(Self::from_native(&decompiled, network_id)))
    }

    pub fn statically_validate(&self, network_id: u8) -> Result<()> {
        toolkit::functions::transaction_v1::instructions::statically_validate(
            &self.instructions.0,
            &self
                .blobs
                .iter()
                .cloned()
                .map(|blob| (engine::hash(&blob), blob))
                .collect(),
            &engine::NetworkDefinition::from_network_id(network_id),
        )?;
        toolkit::functions::transaction_v1::manifest::statically_validate(
            &self.to_native(),
            &engine::NetworkDefinition::from_network_id(network_id),
        )?;
        Ok(())
    }

    pub fn extract_addresses(&self) -> HashMap<EntityType, Vec<Arc<Address>>> {
        let network_id = self.instructions.1;
        let (addresses, _) =
            toolkit::functions::transaction_v1::instructions::extract_addresses(
                &self.instructions.0,
            );

        addresses.into_iter().fold(
            HashMap::<EntityType, Vec<Arc<Address>>>::new(),
            |mut map, node_id| {
                if let Some(entity_type) = node_id.entity_type() {
                    let entity_type = EntityType::from(entity_type);
                    map.entry(entity_type).or_default().push(Arc::new(
                        Address::from_node_id(node_id, network_id),
                    ));
                    map
                } else {
                    map
                }
            },
        )
    }

    pub fn statically_analyze(&self, network_id: u8) -> Result<StaticAnalysis> {
        let native = self.clone().to_native();
        let static_analysis =
            toolkit::functions::transaction_v1::manifest::statically_analyze(
                &native,
            )?;
        Ok(StaticAnalysis::from_native(static_analysis, network_id))
    }

    pub fn dynamically_analyze(
        &self,
        network_id: u8,
        toolkit_receipt: String,
    ) -> Result<DynamicAnalysis> {
        let native = self.clone().to_native();
        let network_definition =
            engine::NetworkDefinition::from_network_id(network_id);
        let receipt = serde_json::from_str::<
            SerializableToolkitTransactionReceipt,
        >(&toolkit_receipt)
        .ok()
        .and_then(|receipt| {
            receipt
                .into_runtime_receipt(&engine::AddressBech32Decoder::new(
                    &network_definition,
                ))
                .ok()
        })
        .ok_or(RadixEngineToolkitError::InvalidReceipt)?;
        toolkit::functions::transaction_v1::manifest::dynamically_analyze(
            &native, receipt,
        )
        .map_err(|_| RadixEngineToolkitError::InvalidReceipt)
        .map(|summary| DynamicAnalysis::from_native(summary, network_id))
    }
}

impl TransactionManifestV1 {
    pub fn from_native(
        engine::TransactionManifestV1 {
            instructions,
            blobs,
            ..
        }: &engine::TransactionManifestV1,
        network_id: u8,
    ) -> Self {
        let blobs = blobs.iter().map(|(_, v)| v.clone()).collect::<Vec<_>>();
        let instructions = InstructionsV1(instructions.clone(), network_id);
        Self {
            instructions: Arc::new(instructions),
            blobs,
        }
    }

    pub fn to_native(&self) -> engine::TransactionManifestV1 {
        let blobs = self
            .blobs
            .iter()
            .map(|blob| (engine::hash(blob), blob.clone()))
            .collect::<IndexMap<_, _>>();
        let instructions = self.instructions.0.clone();

        engine::TransactionManifestV1 {
            instructions,
            blobs,
            ..Default::default()
        }
    }
}
