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

use crate::prelude::*;

#[derive(Clone, Debug, Object)]
pub struct SignedPartialTransactionV2 {
    pub partial_transaction: Arc<PartialTransactionV2>,
    pub root_subintent_signatures: Vec<SignatureWithPublicKeyV1>,
    pub non_root_subintent_signatures: Vec<Vec<SignatureWithPublicKeyV1>>,
}

#[uniffi::export]
impl SignedPartialTransactionV2 {
    #[uniffi::constructor]
    pub fn new(
        partial_transaction: Arc<PartialTransactionV2>,
        root_subintent_signatures: Vec<SignatureWithPublicKeyV1>,
        non_root_subintent_signatures: Vec<Vec<SignatureWithPublicKeyV1>>,
    ) -> Arc<Self> {
        Arc::new(Self {
            partial_transaction,
            root_subintent_signatures,
            non_root_subintent_signatures,
        })
    }

    #[uniffi::constructor]
    pub fn from_payload_bytes(compiled_intent: Vec<u8>) -> Result<Arc<Self>> {
        toolkit::functions::transaction_v2::signed_partial_transaction::from_payload_bytes(
            compiled_intent,
        )
        .map_err(RadixEngineToolkitError::from)
        .and_then(|partial_transaction| partial_transaction.try_into().map(Arc::new))
    }

    pub fn partial_transaction(&self) -> Arc<PartialTransactionV2> {
        // TODO: We're creating another pointer to the partial transaction
        // object which means that this doesn't quite follow value semantics.
        // The caller will have a reference to the partial transaction which
        // means that any changes made to it will reflect in the object that
        // they get back. This isn't the first instance of this but it's my
        // first time noticing it. This might be something that we want to fix
        // or look into when rearchitecting the toolkit such that it follows the
        // value semantics perfectly. Perhaps even taking a step back, am I
        // right about the assumption that this is now a reference and that it
        // won't follow value semantics?
        self.partial_transaction.clone()
    }

    pub fn root_subintent_signatures(&self) -> Vec<SignatureWithPublicKeyV1> {
        self.root_subintent_signatures.clone()
    }

    pub fn non_root_subintent_signatures(
        &self,
    ) -> Vec<Vec<SignatureWithPublicKeyV1>> {
        self.non_root_subintent_signatures.clone()
    }

    pub fn root_subintent_hash(&self) -> Result<Arc<TransactionHash>> {
        self.partial_transaction.root_subintent_hash()
    }

    pub fn to_payload_bytes(&self) -> Result<Vec<u8>> {
        engine::SignedPartialTransactionV2::try_from(self.clone()).and_then(|intent| {
            toolkit::functions::transaction_v2::signed_partial_transaction::to_payload_bytes(
                &intent,
            )
            .map_err(Into::into)
        })
    }

    pub fn statically_validate(&self, network_id: u8) -> Result<()> {
        toolkit::functions::transaction_v2::signed_partial_transaction::statically_validate(
            &self.clone().try_into()?,
            &engine::NetworkDefinition::from_network_id(network_id),
        )
        .map_err(Into::into)
    }
}

impl TryFrom<engine::SignedPartialTransactionV2>
    for SignedPartialTransactionV2
{
    type Error = RadixEngineToolkitError;

    fn try_from(
        engine::SignedPartialTransactionV2 {
            partial_transaction,
            root_subintent_signatures,
            non_root_subintent_signatures,
        }: engine::SignedPartialTransactionV2,
    ) -> Result<Self> {
        Ok(Self {
            partial_transaction: Arc::new(partial_transaction.try_into()?),
            root_subintent_signatures: root_subintent_signatures
                .signatures
                .into_iter()
                .map(|value| value.0)
                .map(SignatureWithPublicKeyV1::from)
                .collect(),
            non_root_subintent_signatures: non_root_subintent_signatures
                .by_subintent
                .into_iter()
                .map(|value| {
                    value
                        .signatures
                        .into_iter()
                        .map(|value| value.0)
                        .map(SignatureWithPublicKeyV1::from)
                        .collect()
                })
                .collect(),
        })
    }
}

impl TryFrom<SignedPartialTransactionV2>
    for engine::SignedPartialTransactionV2
{
    type Error = RadixEngineToolkitError;

    fn try_from(
        SignedPartialTransactionV2 {
            partial_transaction,
            root_subintent_signatures,
            non_root_subintent_signatures,
        }: SignedPartialTransactionV2,
    ) -> Result<Self> {
        Ok(Self {
            partial_transaction: engine::PartialTransactionV2::try_from(
                partial_transaction.as_ref().clone(),
            )?,
            root_subintent_signatures: root_subintent_signatures
                .into_iter()
                .map(|value| {
                    engine::SignatureWithPublicKeyV1::try_from(value)
                        .map(engine::IntentSignatureV1)
                })
                .collect::<Result<_>>()
                .map(|value| engine::IntentSignaturesV2 {
                    signatures: value,
                })?,
            non_root_subintent_signatures: non_root_subintent_signatures
                .into_iter()
                .map(|value| {
                    value
                        .into_iter()
                        .map(|value| {
                            engine::SignatureWithPublicKeyV1::try_from(value)
                                .map(engine::IntentSignatureV1)
                        })
                        .collect::<Result<_>>()
                        .map(|value| engine::IntentSignaturesV2 {
                            signatures: value,
                        })
                })
                .collect::<Result<_>>()
                .map(|value| engine::NonRootSubintentSignaturesV2 {
                    by_subintent: value,
                })?,
        })
    }
}
