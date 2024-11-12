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
pub struct PreviewPartialTransactionV2Builder {
    children_with_signers: Vec<PreviewPartialTransactionV2>,
    root_subintent_header: Option<IntentHeaderV2>,
    root_subintent_message: MessageV2,
    root_subintent_manifest: Option<TransactionManifestV2>,
    root_subintent_signers: Vec<PublicKey>,
}

#[uniffi::export]
impl PreviewPartialTransactionV2Builder {
    #[uniffi::constructor]
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            children_with_signers: Default::default(),
            root_subintent_header: Default::default(),
            root_subintent_message: Default::default(),
            root_subintent_manifest: Default::default(),
            root_subintent_signers: Default::default(),
        })
    }

    pub fn add_child(
        self: Arc<Self>,
        child: Arc<PreviewPartialTransactionV2>,
    ) -> Arc<Self> {
        self.with_builder(|builder| {
            builder.children_with_signers.push(child.as_ref().clone())
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

    pub fn add_root_subintent_signer(
        self: Arc<Self>,
        signer: PublicKey,
    ) -> Arc<Self> {
        self.with_builder(|builder| builder.root_subintent_signers.push(signer))
    }

    pub fn build(self: Arc<Self>) -> Result<Arc<PreviewPartialTransactionV2>> {
        // Deconstructing the builder.
        let PreviewPartialTransactionV2Builder {
            children_with_signers,
            root_subintent_header: Some(header),
            root_subintent_message: message,
            root_subintent_signers,
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
            root_subintent: SubintentV2::new(IntentCoreV2::new(
                header,
                blobs,
                message,
                children,
                instructions,
            )),
            non_root_subintents: children_with_signers
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
        let preview_partial_transaction = PreviewPartialTransactionV2 {
            partial_transaction: Arc::new(partial_transaction),
            root_subintent_signers,
            non_root_subintent_signers: children_with_signers
                .iter()
                .flat_map(|child| {
                    let mut signers = Vec::new();
                    signers.push(child.root_subintent_signers.clone());
                    signers.extend(child.non_root_subintent_signers.clone());
                    signers
                })
                .collect(),
        };

        Ok(Arc::new(preview_partial_transaction))
    }
}

impl PreviewPartialTransactionV2Builder {
    fn with_builder(
        self: Arc<Self>,
        callback: impl FnOnce(&mut Self),
    ) -> Arc<Self> {
        let mut this = Arc::try_unwrap(self).unwrap_or_else(|x| (*x).clone());
        callback(&mut this);
        Arc::new(this)
    }
}
