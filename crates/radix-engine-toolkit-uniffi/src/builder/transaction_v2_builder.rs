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
pub struct TransactionV2Builder {
    signed_children: Vec<SignedPartialTransactionV2>,
    transaction_header: Option<TransactionHeaderV2>,
    transaction_intent_header: Option<IntentHeaderV2>,
    transaction_intent_message: MessageV2,
    transaction_intent_manifest: Option<TransactionManifestV2>,
}

#[uniffi::export]
impl TransactionV2Builder {
    #[uniffi::constructor]
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            signed_children: Default::default(),
            transaction_header: Default::default(),
            transaction_intent_header: Default::default(),
            transaction_intent_message: Default::default(),
            transaction_intent_manifest: Default::default(),
        })
    }

    pub fn add_child(
        self: Arc<Self>,
        child: Arc<SignedPartialTransactionV2>,
    ) -> Arc<Self> {
        self.with_builder(|builder| {
            builder.signed_children.push(child.as_ref().clone())
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

    pub fn prepare_for_signing(
        self: Arc<Self>,
    ) -> Result<Arc<TransactionV2BuilderSignatureStep>> {
        // Deconstructing the builder.
        let TransactionV2Builder {
            signed_children,
            transaction_header: Some(transaction_header),
            transaction_intent_header: Some(header),
            transaction_intent_message: message,
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
            root_intent_core: Arc::new(IntentCoreV2 {
                header,
                blobs,
                message,
                children,
                instructions,
            }),
            non_root_subintents: signed_children
                .iter()
                .flat_map(|child| {
                    let mut subintents = Vec::new();
                    subintents
                        .push(child.partial_transaction.root_subintent.clone());
                    subintents.extend(
                        child
                            .partial_transaction
                            .non_root_subintents
                            .iter()
                            .cloned(),
                    );
                    subintents
                })
                .collect(),
        };

        // Constructing the signed transaction intent
        let signed_transaction_intent = SignedTransactionIntentV2 {
            transaction_intent: Arc::new(transaction_intent),
            transaction_intent_signatures: Default::default(),
            non_root_subintent_signatures: signed_children
                .iter()
                .flat_map(|child| {
                    let mut signatures = Vec::new();
                    signatures.push(child.root_subintent_signatures.clone());
                    signatures
                        .extend(child.non_root_subintent_signatures.clone());
                    signatures
                })
                .collect(),
        };

        // Get the signature of the subintent.
        let intent_hash = signed_transaction_intent.intent_hash()?.0;

        Ok(Arc::new(TransactionV2BuilderSignatureStep {
            intent_hash,
            signed_transaction_intent,
        }))
    }
}

impl TransactionV2Builder {
    fn with_builder(
        self: Arc<Self>,
        callback: impl FnOnce(&mut Self),
    ) -> Arc<Self> {
        let mut this = Arc::try_unwrap(self).unwrap_or_else(|x| (*x).clone());
        callback(&mut this);
        Arc::new(this)
    }
}

#[derive(Clone, Debug, Object)]
pub struct TransactionV2BuilderSignatureStep {
    intent_hash: NativeHash,
    signed_transaction_intent: SignedTransactionIntentV2,
}

#[uniffi::export]
impl TransactionV2BuilderSignatureStep {
    pub fn sign_with_private_key(
        self: Arc<Self>,
        private_key: Arc<PrivateKey>,
    ) -> Arc<Self> {
        self.with_builder(|builder| {
            builder
                .signed_transaction_intent
                .transaction_intent_signatures
                .push(SignatureWithPublicKeyV1::from(
                    private_key.0.sign_with_public_key(&builder.intent_hash),
                ));
        })
    }

    pub fn sign_with_signer(
        self: Arc<Self>,
        signer: Box<dyn Signer>,
    ) -> Arc<Self> {
        self.with_builder(|builder| {
            builder
                .signed_transaction_intent
                .transaction_intent_signatures
                .push(signer.sign_to_signature_with_public_key(Arc::new(
                    Hash(builder.intent_hash),
                )));
        })
    }

    pub fn notarize_with_private_key(
        self: Arc<Self>,
        private_key: Arc<PrivateKey>,
    ) -> Result<Arc<NotarizedTransactionV2>> {
        let signed_transaction_intent_hash =
            self.signed_transaction_intent.signed_intent_hash()?.0;
        let notary_signature = private_key
            .sign_to_signature(Arc::new(Hash(signed_transaction_intent_hash)));
        Ok(Arc::new(NotarizedTransactionV2 {
            signed_transaction_intent: Arc::new(
                self.signed_transaction_intent.clone(),
            ),
            notary_signature,
        }))
    }

    pub fn notarize_with_signer(
        self: Arc<Self>,
        private_key: Box<dyn Signer>,
    ) -> Result<Arc<NotarizedTransactionV2>> {
        let signed_transaction_intent_hash =
            self.signed_transaction_intent.signed_intent_hash()?.0;
        let notary_signature = private_key
            .sign_to_signature(Arc::new(Hash(signed_transaction_intent_hash)));
        Ok(Arc::new(NotarizedTransactionV2 {
            signed_transaction_intent: Arc::new(
                self.signed_transaction_intent.clone(),
            ),
            notary_signature,
        }))
    }
}

impl TransactionV2BuilderSignatureStep {
    fn with_builder(
        self: Arc<Self>,
        callback: impl FnOnce(&mut Self),
    ) -> Arc<Self> {
        let mut this = Arc::try_unwrap(self).unwrap_or_else(|x| (*x).clone());
        callback(&mut this);
        Arc::new(this)
    }
}
