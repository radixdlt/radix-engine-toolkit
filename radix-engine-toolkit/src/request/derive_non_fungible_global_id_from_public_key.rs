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

use crate::request::Handler;
use crate::{error::Result, NonFungibleGlobalId};
use scrypto::prelude::{FromPublicKey, PublicKey};
use toolkit_derive::serializable;

// =================
// Model Definition
// =================

/// This function derives the non-fungible global id of the virtual badge associated with the passed
/// public key.
#[serializable]
pub struct DeriveNonFungibleGlobalIdFromPublicKeyRequest {
    /// An 8 bit unsigned integer serialized as a string which represents the id of the network
    /// that the virtual badge non-fungible global id is being derived for.
    #[schemars(with = "String")]
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub network_id: u8,

    /// The public key to derive the non-fungible global id for.
    #[schemars(with = "crate::model::crypto::PublicKey")]
    #[serde_as(as = "serde_with::FromInto<crate::model::crypto::PublicKey>")]
    pub public_key: PublicKey,
}

/// The response from [`DeriveNonFungibleGlobalIdFromPublicKeyRequest`].
#[serializable]
pub struct DeriveNonFungibleGlobalIdFromPublicKeyResponse {
    /// The non-fungible global id of the virtual badge associated with the given public key. The
    /// underlying type of this is a `NonFungibleGlobalId` from the `Value` model.
    #[serde(flatten)] // TODO: Remove after betanet v2
    #[schemars(with = "crate::model::value::ManifestAstValue")]
    #[serde_as(as = "serde_with::TryFromInto<crate::model::value::ManifestAstValue>")]
    pub non_fungible_global_id: NonFungibleGlobalId,
}

// ===============
// Implementation
// ===============

pub struct DeriveNonFungibleGlobalIdFromPublicKeyHandler;

impl
    Handler<
        DeriveNonFungibleGlobalIdFromPublicKeyRequest,
        DeriveNonFungibleGlobalIdFromPublicKeyResponse,
    > for DeriveNonFungibleGlobalIdFromPublicKeyHandler
{
    fn pre_process(
        request: DeriveNonFungibleGlobalIdFromPublicKeyRequest,
    ) -> Result<DeriveNonFungibleGlobalIdFromPublicKeyRequest> {
        Ok(request)
    }

    fn handle(
        request: &DeriveNonFungibleGlobalIdFromPublicKeyRequest,
    ) -> Result<DeriveNonFungibleGlobalIdFromPublicKeyResponse> {
        let non_fungible_global_id =
            scrypto::prelude::NonFungibleGlobalId::from_public_key(&request.public_key);
        let non_fungible_global_id = NonFungibleGlobalId {
            resource_address: crate::NetworkAwareResourceAddress {
                network_id: request.network_id,
                address: non_fungible_global_id.resource_address(),
            },
            non_fungible_local_id: non_fungible_global_id.local_id().clone(),
        };
        Ok(DeriveNonFungibleGlobalIdFromPublicKeyResponse {
            non_fungible_global_id,
        })
    }

    fn post_process(
        _: &DeriveNonFungibleGlobalIdFromPublicKeyRequest,
        response: DeriveNonFungibleGlobalIdFromPublicKeyResponse,
    ) -> Result<DeriveNonFungibleGlobalIdFromPublicKeyResponse> {
        Ok(response)
    }
}
