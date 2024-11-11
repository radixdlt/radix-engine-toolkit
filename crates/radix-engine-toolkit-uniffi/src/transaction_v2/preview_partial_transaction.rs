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
pub struct PreviewPartialTransactionV2 {
    pub partial_transaction: Arc<PartialTransactionV2>,
    pub root_subintent_signers: Vec<PublicKey>,
    pub non_root_subintent_signers: Vec<Vec<PublicKey>>,
}

#[uniffi::export]
impl PreviewPartialTransactionV2 {
    #[uniffi::constructor]
    pub fn new(
        partial_transaction: Arc<PartialTransactionV2>,
        root_subintent_signers: Vec<PublicKey>,
        non_root_subintent_signers: Vec<Vec<PublicKey>>,
    ) -> Arc<Self> {
        Arc::new(Self {
            partial_transaction,
            root_subintent_signers,
            non_root_subintent_signers,
        })
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

    pub fn root_subintent_signers(&self) -> Vec<PublicKey> {
        self.root_subintent_signers.clone()
    }

    pub fn non_root_subintent_signers(&self) -> Vec<Vec<PublicKey>> {
        self.non_root_subintent_signers.clone()
    }

    pub fn root_subintent_hash(&self) -> Result<Arc<TransactionHash>> {
        self.partial_transaction.root_subintent_hash()
    }
}
