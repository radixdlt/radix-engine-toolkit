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

// TODO: Why make the fields private when this crate is not supposed to be
// consumed as a library anyway? Might as well just make them public so that
// we can use them in tests instead of adding weird constructors for tests.
// This was changed to be `pub` for the time being but more thought should be
// put into whether this is public within the crate alone or public in general.

#[derive(Object)]
pub struct PrivateKey(pub engine::PrivateKey);

#[uniffi::export]
impl PrivateKey {
    #[uniffi::constructor]
    pub fn new(bytes: Vec<u8>, curve: Curve) -> Result<Arc<Self>> {
        match curve {
            Curve::Ed25519 => Self::new_ed25519(bytes),
            Curve::Secp256k1 => Self::new_secp256k1(bytes),
        }
    }

    #[uniffi::constructor]
    pub fn new_secp256k1(bytes: Vec<u8>) -> Result<Arc<Self>> {
        engine::Secp256k1PrivateKey::from_bytes(&bytes)
            .map_err(|_| RadixEngineToolkitError::InvalidLength {
                expected: engine::Ed25519PublicKey::LENGTH as u64,
                actual: bytes.len() as u64,
                data: bytes,
            })
            .map(|value| Arc::new(Self(engine::PrivateKey::Secp256k1(value))))
    }
    #[uniffi::constructor]
    pub fn new_ed25519(bytes: Vec<u8>) -> Result<Arc<Self>> {
        engine::Ed25519PrivateKey::from_bytes(&bytes)
            .map_err(|_| RadixEngineToolkitError::InvalidLength {
                expected: engine::Ed25519PublicKey::LENGTH as u64,
                actual: bytes.len() as u64,
                data: bytes,
            })
            .map(|value| Arc::new(Self(engine::PrivateKey::Ed25519(value))))
    }

    pub fn raw(&self) -> Vec<u8> {
        match &self.0 {
            engine::PrivateKey::Ed25519(private_key) => private_key.to_bytes(),
            engine::PrivateKey::Secp256k1(private_key) => {
                private_key.to_bytes()
            }
        }
    }

    pub fn raw_hex(&self) -> String {
        hex::encode(self.raw())
    }

    pub fn curve(&self) -> Curve {
        match &self.0 {
            engine::PrivateKey::Ed25519(..) => Curve::Ed25519,
            engine::PrivateKey::Secp256k1(..) => Curve::Secp256k1,
        }
    }

    fn sign(&self, hash: Arc<Hash>) -> Vec<u8> {
        Signer::sign(self, hash)
    }

    fn sign_to_signature(&self, hash: Arc<Hash>) -> SignatureV1 {
        Signer::sign_to_signature(self, hash)
    }

    fn sign_to_signature_with_public_key(
        &self,
        hash: Arc<Hash>,
    ) -> SignatureWithPublicKeyV1 {
        Signer::sign_to_signature_with_public_key(self, hash)
    }

    fn public_key(&self) -> PublicKey {
        Signer::public_key(self)
    }

    fn public_key_bytes(&self) -> Vec<u8> {
        match self.public_key() {
            PublicKey::Secp256k1 { value } | PublicKey::Ed25519 { value } => {
                value
            }
        }
    }
}

impl Signer for PrivateKey {
    fn sign(&self, hash: Arc<Hash>) -> Vec<u8> {
        match self.sign_to_signature(hash) {
            SignatureV1::Ed25519 { value }
            | SignatureV1::Secp256k1 { value } => value,
        }
    }

    fn sign_to_signature(&self, hash: Arc<Hash>) -> SignatureV1 {
        self.0.sign_without_public_key(&hash.0).into()
    }

    fn sign_to_signature_with_public_key(
        &self,
        hash: Arc<Hash>,
    ) -> SignatureWithPublicKeyV1 {
        self.0.sign_with_public_key(&hash.0).into()
    }

    fn public_key(&self) -> PublicKey {
        self.0.public_key().into()
    }
}

#[uniffi::export(callback_interface)]
pub trait Signer: Send + Sync {
    fn sign(&self, hash: Arc<Hash>) -> Vec<u8>;
    fn sign_to_signature(&self, hash: Arc<Hash>) -> SignatureV1;
    fn sign_to_signature_with_public_key(
        &self,
        hash: Arc<Hash>,
    ) -> SignatureWithPublicKeyV1;
    fn public_key(&self) -> PublicKey;
}
