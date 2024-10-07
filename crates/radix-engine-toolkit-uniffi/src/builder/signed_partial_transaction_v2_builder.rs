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
pub struct SignedPartialTransactionV2Builder {
    signed_children: Vec<SignedPartialTransactionV2>,
    root_subintent_header: Option<IntentHeaderV2>,
    root_subintent_message: MessageV2,
    root_subintent_manifest: Option<TransactionManifestV2>,
}

#[uniffi::export]
impl SignedPartialTransactionV2Builder {
    #[uniffi::constructor]
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            signed_children: Default::default(),
            root_subintent_header: Default::default(),
            root_subintent_message: Default::default(),
            root_subintent_manifest: Default::default(),
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
        self.with_builder(|builder| builder.root_subintent_message = message)
    }

    pub fn intent_header(
        self: Arc<Self>,
        intent_header: IntentHeaderV2,
    ) -> Arc<Self> {
        self.with_builder(|builder| {
            builder.root_subintent_header = Some(intent_header)
        })
    }

    pub fn manifest(
        self: Arc<Self>,
        manifest: Arc<TransactionManifestV2>,
    ) -> Arc<Self> {
        self.with_builder(|builder| {
            builder.root_subintent_manifest = Some(manifest.as_ref().clone())
        })
    }

    pub fn prepare_for_signing(
        self: Arc<Self>,
    ) -> Result<Arc<SignedPartialTransactionV2BuilderSignatureStep>> {
        // Deconstructing the builder.
        let SignedPartialTransactionV2Builder {
            signed_children,
            root_subintent_header: Some(header),
            root_subintent_message: message,
            root_subintent_manifest:
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

        // Constructing the partial transaction
        let partial_transaction = PartialTransactionV2 {
            root_subintent: Arc::new(IntentCoreV2 {
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

        // Constructing the signed partial transaction
        let signed_partial_transaction = SignedPartialTransactionV2 {
            partial_transaction: Arc::new(partial_transaction),
            root_subintent_signatures: Default::default(),
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
        let subintent_hash = NativeSignedPartialTransactionV2::try_from(
            signed_partial_transaction.clone(),
        )?
        .prepare(&NativePreparationSettings::latest())?
        .subintent_hash()
        .0;

        Ok(Arc::new(SignedPartialTransactionV2BuilderSignatureStep {
            subintent_hash,
            signed_partial_transaction,
        }))
    }
}

impl SignedPartialTransactionV2Builder {
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
pub struct SignedPartialTransactionV2BuilderSignatureStep {
    subintent_hash: NativeHash,
    signed_partial_transaction: SignedPartialTransactionV2,
}

#[uniffi::export]
impl SignedPartialTransactionV2BuilderSignatureStep {
    pub fn sign_with_private_key(
        self: Arc<Self>,
        private_key: Arc<PrivateKey>,
    ) -> Arc<Self> {
        self.with_builder(|builder| {
            builder
                .signed_partial_transaction
                .root_subintent_signatures
                .push(SignatureWithPublicKeyV1::from(
                    private_key.0.sign_with_public_key(&builder.subintent_hash),
                ));
        })
    }

    pub fn sign_with_signer(
        self: Arc<Self>,
        signer: Box<dyn Signer>,
    ) -> Arc<Self> {
        self.with_builder(|builder| {
            builder
                .signed_partial_transaction
                .root_subintent_signatures
                .push(signer.sign_to_signature_with_public_key(Arc::new(
                    Hash(builder.subintent_hash),
                )));
        })
    }

    pub fn build(self: Arc<Self>) -> Arc<SignedPartialTransactionV2> {
        Arc::new(self.as_ref().signed_partial_transaction.clone())
    }
}

impl SignedPartialTransactionV2BuilderSignatureStep {
    fn with_builder(
        self: Arc<Self>,
        callback: impl FnOnce(&mut Self),
    ) -> Arc<Self> {
        let mut this = Arc::try_unwrap(self).unwrap_or_else(|x| (*x).clone());
        callback(&mut this);
        Arc::new(this)
    }
}
