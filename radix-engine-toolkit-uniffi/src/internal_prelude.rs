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

//! This is an internal prelude module meant to only be public within this crate which re-exports
//! types and functions from the core toolkit and Scrypto crates with a prefix. This is done to
//! distinguish between the core toolkit types, scrypto types, and types defined in this crate.
//! We choose the types in this crate to have short un-prefixed names since those are the names that
//! the types will be generated with when using UniFFI

mod core {
    /* Modules */
    pub use radix_engine_toolkit_core::functions::information::{
        information as core_information, BuildInformation as CoreBuildInformation,
        DependencyInformation as CoreDependencyInformation,
    };
}

mod native {
    pub use transaction::prelude::{
        Ed25519PrivateKey as NativeEd25519PrivateKey, Ed25519PublicKey as NativeEd25519PublicKey,
        Ed25519PublicKeyHash as NativeEd25519PublicKeyHash,
        Ed25519Signature as NativeEd25519Signature, HasPublicKeyHash as NativeHasPublicKeyHash,
        PublicKey as NativePublicKey, PublicKeyHash as NativePublicKeyHash,
        Secp256k1PrivateKey as NativeSecp256k1PrivateKey,
        Secp256k1PublicKey as NativeSecp256k1PublicKey,
        Secp256k1PublicKeyHash as NativeSecp256k1PublicKeyHash,
        Secp256k1Signature as NativeSecp256k1Signature, SignatureV1 as NativeSignature,
        SignatureWithPublicKeyV1 as NativeSignatureWithPublicKey,
    };

    pub use scrypto::prelude::NodeId as NativeNodeId;
}

pub use self::core::*;
pub use self::native::*;
