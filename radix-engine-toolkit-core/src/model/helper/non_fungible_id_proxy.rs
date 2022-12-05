// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at

//   http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use std::convert::TryFrom;

use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use scrypto::prelude::scrypto_decode;

use crate::error::Error;

#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct NonFungibleIdProxy {
    #[serde_as(as = "serde_with::hex::Hex")]
    value: Vec<u8>,
}

impl TryFrom<scrypto::prelude::NonFungibleId> for NonFungibleIdProxy {
    type Error = Error;

    fn try_from(value: scrypto::prelude::NonFungibleId) -> Result<Self, Self::Error> {
        Ok(Self {
            value: value.to_vec(),
        })
    }
}

impl TryFrom<NonFungibleIdProxy> for scrypto::prelude::NonFungibleId {
    type Error = Error;

    fn try_from(value: NonFungibleIdProxy) -> Result<Self, Self::Error> {
        Ok(scrypto_decode(&value.value)?)
    }
}
