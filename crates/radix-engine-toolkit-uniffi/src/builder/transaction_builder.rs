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

#[derive(Clone, Object)]
pub struct TransactionBuilderIntentSignaturesStep(
    pub(crate) TransactionHeader,
    pub(crate) TransactionManifest,
    pub(crate) Message,
    pub(crate) Hash,
    pub(crate) Vec<SignatureWithPublicKey>,
);

#[uniffi::export]
impl TransactionBuilder {
    #[uniffi::constructor]
    pub fn new() -> Arc<Self> {
        Arc::new(Self)
    }

    pub fn header(
        self: Arc<Self>,
        header: TransactionHeader,
    ) -> Arc<TransactionBuilderHeaderStep> {
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
        TransactionBuilderIntentSignaturesStep::new(
            &TransactionBuilderMessageStep(
                self.0.clone(),
                self.1.clone(),
                message,
            ),
        )
    }

    pub fn sign_with_private_key(
        self: Arc<Self>,
        private_key: Arc<PrivateKey>,
    ) -> Arc<TransactionBuilderIntentSignaturesStep> {
        let builder = TransactionBuilderIntentSignaturesStep::new(&self);
        builder.sign_with_private_key(private_key)
    }

    pub fn sign_with_signer(
        self: Arc<Self>,
        signer: Box<dyn Signer>,
    ) -> Arc<TransactionBuilderIntentSignaturesStep> {
        let builder = TransactionBuilderIntentSignaturesStep::new(&self);
        builder.sign_with_signer(signer)
    }
}

#[uniffi::export]
impl TransactionBuilderIntentSignaturesStep {
    #[uniffi::constructor]
    fn new(message_step: &TransactionBuilderMessageStep) -> Arc<Self> {
        let intent = Intent {
            header: message_step.0.clone(),
            manifest: Arc::new(message_step.1.clone()),
            message: message_step.2.clone(),
        };
        let hash = Hash(intent.hash().unwrap().0);

        Arc::new(TransactionBuilderIntentSignaturesStep(
            message_step.0.clone(),
            message_step.1.clone(),
            message_step.2.clone(),
            hash,
            vec![],
        ))
    }

    pub fn sign_with_private_key(
        self: Arc<Self>,
        private_key: Arc<PrivateKey>,
    ) -> Arc<Self> {
        let signature = private_key
            .deref()
            .sign_to_signature_with_public_key(Arc::new(self.3));

        let mut this = Arc::try_unwrap(self).unwrap_or_else(|x| (*x).clone());
        this.4.push(signature);
        Arc::new(this)
    }

    pub fn sign_with_signer(
        self: Arc<Self>,
        signer: Box<dyn Signer>,
    ) -> Arc<Self> {
        let signature = signer
            .deref()
            .sign_to_signature_with_public_key(Arc::new(self.3));

        let mut this = Arc::try_unwrap(self).unwrap_or_else(|x| (*x).clone());
        this.4.push(signature);
        Arc::new(this)
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
        let intent = Intent {
            header: self.0.clone(),
            manifest: Arc::new(self.1.clone()),
            message: self.2.clone(),
        };

        /* Collecting the intent signatures */
        let intent_signatures = self
            .4
            .clone()
            .into_iter()
            .map(NativeSignatureWithPublicKey::try_from)
            .map(|signature| signature.map(NativeIntentSignature))
            .collect::<Result<Vec<_>>>()?;

        /* Preparing the signed intent */
        let intent = NativeIntent::try_from(intent)
            .expect("Everything about this is trusted at this point");
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

#[test]
fn test_sign_with_private_key_panics() {
    // Arrange
    let pub_key = PublicKey::Ed25519 { value: vec![0; 32] };
    let prv_key = PrivateKey::new_ed25519(vec![0; 32]).unwrap();

    let th = TransactionHeader {
        network_id: 0,
        start_epoch_inclusive: 1,
        end_epoch_exclusive: 2,
        nonce: 1,
        notary_public_key: pub_key,
        notary_is_signatory: false,
        tip_percentage: 0,
    };

    let manifest = TransactionManifest::new(
        Arc::new(Instructions(Vec::new(), 0)),
        Vec::new(),
    );

    // Act
    let intent_sig_step = TransactionBuilder::new()
        .header(th)
        .manifest(manifest)
        .sign_with_private_key(prv_key.clone());
    let _copy = Arc::clone(&intent_sig_step);

    let intent_signed = intent_sig_step.sign_with_private_key(prv_key);

    // Assert
    assert_eq!(Arc::strong_count(&intent_signed), 2);
}
