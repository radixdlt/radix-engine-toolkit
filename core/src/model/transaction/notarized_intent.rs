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

use scrypto::prelude::Signature;
use serializable::serializable;

use crate::SignedTransactionIntent;

/// A notarized transaction intent which is made up of a signed transaction intent and the notary
/// intent on said signed intent.
#[serializable]
pub struct NotarizedTransactionIntent {
    /// The signed transaction intent of the transaction.
    pub signed_intent: SignedTransactionIntent,

    /// The signature of the notary on the signed transaction intent.
    #[schemars(with = "crate::model::crypto::Signature")]
    #[serde_as(as = "serde_with::FromInto<crate::model::crypto::Signature>")]
    pub notary_signature: Signature,
}
