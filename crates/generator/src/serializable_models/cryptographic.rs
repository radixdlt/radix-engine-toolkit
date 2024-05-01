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

use radix_common::prelude::{
    Ed25519PrivateKey, PublicKey, Secp256k1PrivateKey,
};
use radix_engine_toolkit_json::prelude::*;
use radix_transactions::prelude::{SignatureV1, SignatureWithPublicKeyV1};
use scrypto::prelude::{hash, Hash};

use super::traits::HasExamples;

impl<'f> HasExamples<'f> for SerializablePublicKey {
    fn examples() -> Vec<Self> {
        vec![
            Self::Secp256k1(
                Secp256k1PrivateKey::from_u64(1)
                    .unwrap()
                    .public_key()
                    .0
                    .into(),
            ),
            Self::Ed25519(
                Ed25519PrivateKey::from_u64(1)
                    .unwrap()
                    .public_key()
                    .0
                    .into(),
            ),
        ]
    }
}

impl<'f> HasExamples<'f> for SerializableSecp256k1PublicKey {
    fn examples() -> Vec<Self> {
        vec![Secp256k1PrivateKey::from_u64(1)
            .unwrap()
            .public_key()
            .into()]
    }
}

impl<'f> HasExamples<'f> for SerializableEd25519PublicKey {
    fn examples() -> Vec<Self> {
        vec![Ed25519PrivateKey::from_u64(1).unwrap().public_key().into()]
    }
}

impl<'f> HasExamples<'f> for SerializableSignature {
    fn examples() -> Vec<Self> {
        let message = Message::PlainMessage(b"Hello World");
        private_keys()
            .into_iter()
            .map(|private_key| private_key.sign_to_signature(message).into())
            .collect()
    }
}

impl<'f> HasExamples<'f> for SerializableSignatureWithPublicKey {
    fn examples() -> Vec<Self> {
        let message = Message::PlainMessage(b"Hello World");
        private_keys()
            .into_iter()
            .map(|private_key| {
                private_key
                    .sign_to_signature_with_public_key(message)
                    .into()
            })
            .collect()
    }
}

impl<'f> HasExamples<'f> for SerializablePublicKeyHash {
    fn examples() -> Vec<Self> {
        private_keys()
            .into_iter()
            .map(|private_key| private_key.public_key().into())
            .collect()
    }
}

impl<'f> HasExamples<'f> for SerializableHash {
    fn examples() -> Vec<Self> {
        vec![b"Hello World".to_vec(), b"Hey World".to_vec()]
            .into_iter()
            .map(hash)
            .map(Into::into)
            .collect()
    }
}

fn private_keys() -> Vec<Box<dyn PrivateKey>> {
    vec![
        Box::new(Secp256k1PrivateKey::from_u64(1).unwrap()),
        Box::new(Ed25519PrivateKey::from_u64(1).unwrap()),
    ]
}

trait PrivateKey {
    fn public_key(&self) -> PublicKey;
    fn sign_to_signature(&self, message: Message) -> SignatureV1;
    fn sign_to_signature_with_public_key(
        &self,
        message: Message,
    ) -> SignatureWithPublicKeyV1;
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
enum Message<'m> {
    PlainMessage(&'m [u8]),
    Hashed(&'m Hash),
}

impl PrivateKey for Secp256k1PrivateKey {
    fn sign_to_signature(&self, message: Message) -> SignatureV1 {
        let data = match message {
            Message::Hashed(hashed) => *hashed,
            Message::PlainMessage(message) => hash(message),
        };

        let signature = self.sign(&data);

        signature.into()
    }

    fn sign_to_signature_with_public_key(
        &self,
        message: Message,
    ) -> SignatureWithPublicKeyV1 {
        let data = match message {
            Message::Hashed(hashed) => *hashed,
            Message::PlainMessage(message) => hash(message),
        };

        let signature = self.sign(&data);

        SignatureWithPublicKeyV1::Secp256k1 { signature }
    }

    fn public_key(&self) -> PublicKey {
        self.public_key().into()
    }
}

impl PrivateKey for Ed25519PrivateKey {
    fn sign_to_signature(&self, message: Message) -> SignatureV1 {
        let data = match message {
            Message::Hashed(hashed) => *hashed,
            Message::PlainMessage(message) => hash(message),
        };

        let signature = self.sign(&data);

        signature.into()
    }

    fn sign_to_signature_with_public_key(
        &self,
        message: Message,
    ) -> SignatureWithPublicKeyV1 {
        let data = match message {
            Message::Hashed(hashed) => *hashed,
            Message::PlainMessage(message) => hash(message),
        };

        let signature = self.sign(&data);
        let public_key = self.public_key();

        SignatureWithPublicKeyV1::Ed25519 {
            public_key,
            signature,
        }
    }

    fn public_key(&self) -> PublicKey {
        self.public_key().into()
    }
}
