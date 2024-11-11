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
use std::ops::Deref;

#[derive(Clone, Debug, Object)]
pub struct PreviewTransactionV2Builder {
    transaction_header: Option<TransactionHeaderV2>,
    transaction_intent_header: Option<IntentHeaderV2>,
    transaction_intent_message: MessageV2,
    transaction_intent_manifest: Option<TransactionManifestV2>,
    child_partial_transactions: Vec<PartialTransactionV2>,
    root_signer_public_keys: IndexSet<PublicKey>,
    non_root_subintent_signer_public_keys: Vec<Vec<PublicKey>>,
}

#[uniffi::export]
impl PreviewTransactionV2Builder {
    #[uniffi::constructor]
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            transaction_header: Default::default(),
            transaction_intent_header: Default::default(),
            transaction_intent_message: Default::default(),
            transaction_intent_manifest: Default::default(),
            child_partial_transactions: Default::default(),
            root_signer_public_keys: Default::default(),
            non_root_subintent_signer_public_keys: Default::default(),
        })
    }

    pub fn add_child(
        self: Arc<Self>,
        partial_transaction: Arc<PartialTransactionV2>,
        signers: Vec<PublicKey>,
    ) -> Arc<Self> {
        self.with_builder(|builder| {
            builder
                .child_partial_transactions
                .push(partial_transaction.deref().clone());
            builder.non_root_subintent_signer_public_keys.push(signers);
        })
    }

    pub fn message(self: Arc<Self>, message: MessageV2) -> Arc<Self> {
        self.with_builder(|builder| {
            builder.transaction_intent_message = message
        })
    }

    pub fn intent_header(
        self: Arc<Self>,
        intent_header: IntentHeaderV2,
    ) -> Arc<Self> {
        self.with_builder(|builder| {
            builder.transaction_intent_header = Some(intent_header)
        })
    }

    pub fn transaction_header(
        self: Arc<Self>,
        transaction_header: TransactionHeaderV2,
    ) -> Arc<Self> {
        self.with_builder(|builder| {
            builder.transaction_header = Some(transaction_header)
        })
    }

    pub fn manifest(
        self: Arc<Self>,
        manifest: Arc<TransactionManifestV2>,
    ) -> Arc<Self> {
        self.with_builder(|builder| {
            builder.transaction_intent_manifest =
                Some(manifest.as_ref().clone())
        })
    }

    pub fn add_root_intent_signer(
        self: Arc<Self>,
        signer: PublicKey,
    ) -> Arc<Self> {
        self.with_builder(|builder| {
            builder.root_signer_public_keys.insert(signer);
        })
    }

    pub fn build(self: Arc<Self>) -> Result<Vec<u8>> {
        // Deconstructing the builder.
        let Self {
            transaction_header: Some(transaction_header),
            transaction_intent_header: Some(header),
            transaction_intent_message: message,
            child_partial_transactions,
            root_signer_public_keys,
            non_root_subintent_signer_public_keys,
            transaction_intent_manifest:
                Some(TransactionManifestV2 {
                    instructions,
                    blobs,
                    children,
                }),
        } = Arc::try_unwrap(self).unwrap_or_else(|x| (*x).clone())
        else {
            return Err(
                RadixEngineToolkitError::NotAllBuilderItemsWereSpecified,
            );
        };

        // Constructing the transaction intent
        let transaction_intent = TransactionIntentV2 {
            transaction_header,
            root_intent_core: IntentCoreV2::new(
                header,
                blobs,
                message,
                children,
                instructions,
            ),
            non_root_subintents: child_partial_transactions
                .iter()
                .flat_map(|child| {
                    let mut subintents = Vec::new();
                    subintents.push(child.root_subintent.clone());
                    subintents
                        .extend(child.non_root_subintents.iter().cloned());
                    subintents
                })
                .collect(),
        };
        let transaction_intent =
            NativeTransactionIntentV2::try_from(transaction_intent)?;

        let preview_transaction = NativePreviewTransactionV2 {
            transaction_intent,
            root_signer_public_keys: root_signer_public_keys
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<_>>()?,
            non_root_subintent_signer_public_keys:
                non_root_subintent_signer_public_keys
                    .into_iter()
                    .map(|public_keys| {
                        public_keys
                            .into_iter()
                            .map(TryInto::try_into)
                            .collect::<Result<_>>()
                    })
                    .collect::<Result<_>>()?,
        };

        let raw_preview_transaction = preview_transaction.to_raw()?;
        Ok(raw_preview_transaction.to_vec())
    }
}

impl PreviewTransactionV2Builder {
    fn with_builder(
        self: Arc<Self>,
        callback: impl FnOnce(&mut Self),
    ) -> Arc<Self> {
        let mut this =
            Arc::try_unwrap(self).unwrap_or_else(|arc| (*arc).clone());
        callback(&mut this);
        Arc::new(this)
    }
}
