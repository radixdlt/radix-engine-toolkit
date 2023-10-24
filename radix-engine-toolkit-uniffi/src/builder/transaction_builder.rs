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

use std::ops::Deref;

use crate::prelude::*;

#[derive(Clone, Copy, Debug, Object)]
pub struct TransactionBuilder;

#[derive(Clone, Debug, Object)]
pub struct TransactionBuilderHeaderStep(pub(crate) TransactionHeader);

#[derive(Clone, Debug, Object)]
pub struct TransactionBuilderMessageStep(
    pub(crate) TransactionHeader,
    pub(crate) TransactionManifest,
    pub(crate) Message,
);

#[derive(Object)]
pub struct TransactionBuilderIntentSignaturesStep(
    pub(crate) TransactionHeader,
    pub(crate) TransactionManifest,
    pub(crate) Message,
    pub(crate) Vec<Arc<PrivateKey>>,
    pub(crate) Vec<Box<dyn Signer>>,
);

#[uniffi::export]
impl TransactionBuilder {
    #[uniffi::constructor]
    pub fn new() -> Arc<Self> {
        Arc::new(Self)
    }

    pub fn header(self: Arc<Self>, header: TransactionHeader) -> Arc<TransactionBuilderHeaderStep> {
        Arc::new(TransactionBuilderHeaderStep(header))
    }
}

#[uniffi::export]
impl TransactionBuilderHeaderStep {
    pub fn manifest(
        self: Arc<Self>,
        manifest: Arc<TransactionManifest>,
    ) -> Arc<TransactionBuilderMessageStep> {
        let header = self.0.clone();
        Arc::new(TransactionBuilderMessageStep(
            header,
            manifest.as_ref().clone(),
            Message::None,
        ))
    }
}

#[uniffi::export]
impl TransactionBuilderMessageStep {
    pub fn message(
        self: Arc<Self>,
        message: Message,
    ) -> Arc<TransactionBuilderIntentSignaturesStep> {
        let header = self.0.clone();
        let manifest = self.1.clone();
        Arc::new(TransactionBuilderIntentSignaturesStep(
            header,
            manifest,
            message,
            vec![],
            vec![],
        ))
    }

    pub fn sign_with_private_key(
        self: Arc<Self>,
        private_key: Arc<PrivateKey>,
    ) -> Arc<TransactionBuilderIntentSignaturesStep> {
        let header = self.0.clone();
        let manifest = self.1.clone();
        let message = self.2.clone();
        Arc::new(TransactionBuilderIntentSignaturesStep(
            header,
            manifest,
            message,
            vec![private_key],
            vec![],
        ))
    }

    pub fn sign_with_signer(
        self: Arc<Self>,
        signer: Box<dyn Signer>,
    ) -> Arc<TransactionBuilderIntentSignaturesStep> {
        let header = self.0.clone();
        let manifest = self.1.clone();
        let message = self.2.clone();
        Arc::new(TransactionBuilderIntentSignaturesStep(
            header,
            manifest,
            message,
            vec![],
            vec![signer],
        ))
    }
}

#[uniffi::export]
impl TransactionBuilderIntentSignaturesStep {
    pub fn sign_with_private_key(mut self: Arc<Self>, private_key: Arc<PrivateKey>) -> Arc<Self> {
        let builder = unsafe { Arc::get_mut_unchecked(&mut self) };
        builder.3.push(private_key);
        self
    }

    pub fn sign_with_signer(mut self: Arc<Self>, signer: Box<dyn Signer>) -> Arc<Self> {
        let builder = unsafe { Arc::get_mut_unchecked(&mut self) };
        builder.4.push(signer);
        self
    }

    pub fn notarize_with_private_key(
        self: Arc<Self>,
        private_key: Arc<PrivateKey>,
    ) -> Result<Arc<NotarizedTransaction>> {
        self.notarize(private_key.as_ref()).map(Arc::new)
    }

    pub fn notarize_with_signer(
        self: Arc<Self>,
        signer: Box<dyn Signer>,
    ) -> Result<Arc<NotarizedTransaction>> {
        self.notarize(signer.as_ref()).map(Arc::new)
    }
}

impl TransactionBuilderIntentSignaturesStep {
    fn notarize(&self, notary: &dyn Signer) -> Result<NotarizedTransaction> {
        /* Processing the intent */
        let header = self.0.clone();
        let manifest = self.1.clone();
        let message = self.2.clone();
        let intent = Intent {
            header,
            manifest: Arc::new(manifest),
            message,
        };
        let hash = Arc::new(Hash(intent.hash()?.0));

        /* Creating the intent signatures */
        let intent_signatures = self
            .3
            .iter()
            .map(|private_key| {
                private_key
                    .deref()
                    .sign_to_signature_with_public_key(hash.clone())
            })
            .chain(self.4.iter().map(|signer| {
                signer
                    .deref()
                    .sign_to_signature_with_public_key(hash.clone())
            }))
            .map(NativeSignatureWithPublicKey::try_from)
            .map(|signature| signature.map(NativeIntentSignature))
            .collect::<Result<Vec<_>>>()?;

        /* Preparing the signed intent */
        let intent =
            NativeIntent::try_from(intent).expect("Everything about this is trusted at this point");
        let signed_intent = NativeSignedIntent {
            intent,
            intent_signatures: NativeIntentSignatures {
                signatures: intent_signatures,
            },
        };

        /* Preparing the notarized intent */
        let notarized_transaction = {
            let signed_intent = SignedIntent::from(signed_intent);
            let signed_intent_hash = Arc::new(Hash(signed_intent.hash()?.0));
            let notary_signature = notary.sign_to_signature(signed_intent_hash);
            let notarized_transaction = NotarizedTransaction {
                signed_intent: Arc::new(signed_intent),
                notary_signature,
            };
            let _ = notarized_transaction.hash()?;
            notarized_transaction
        };
        Ok(notarized_transaction)
    }
}
