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
pub struct PartialTransactionV2Builder {
    child_partial_transactions: Vec<PartialTransactionV2>,
    root_subintent_header: Option<IntentHeaderV2>,
    root_subintent_message: MessageV2,
    root_subintent_manifest: Option<TransactionManifestV2>,
}

#[uniffi::export]
impl PartialTransactionV2Builder {
    #[uniffi::constructor]
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            child_partial_transactions: Default::default(),
            root_subintent_header: Default::default(),
            root_subintent_message: Default::default(),
            root_subintent_manifest: Default::default(),
        })
    }

    pub fn add_child(
        self: Arc<Self>,
        child: Arc<PartialTransactionV2>,
    ) -> Arc<Self> {
        self.with_builder(|builder| {
            builder
                .child_partial_transactions
                .push(child.as_ref().clone())
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

    pub fn build(self: Arc<Self>) -> Result<Arc<PartialTransactionV2>> {
        // Deconstructing the builder.
        let PartialTransactionV2Builder {
            child_partial_transactions,
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
            root_subintent: SubintentV2::new(IntentCoreV2::new(
                header,
                blobs,
                message,
                children,
                instructions,
            )),
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

        Ok(Arc::new(partial_transaction))
    }
}

impl PartialTransactionV2Builder {
    fn with_builder(
        self: Arc<Self>,
        callback: impl FnOnce(&mut Self),
    ) -> Arc<Self> {
        let mut this = Arc::try_unwrap(self).unwrap_or_else(|x| (*x).clone());
        callback(&mut this);
        Arc::new(this)
    }
}
