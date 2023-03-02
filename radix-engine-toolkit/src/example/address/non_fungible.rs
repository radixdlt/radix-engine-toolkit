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

use crate::model::address::{NetworkAwareResourceAddress, NonFungibleGlobalId, NonFungibleLocalId};
use scrypto::prelude::{
    BytesNonFungibleLocalId, IntegerNonFungibleLocalId, StringNonFungibleLocalId,
    UUIDNonFungibleLocalId, ECDSA_SECP256K1_TOKEN,
};

pub fn non_fungible_local_uuid() -> NonFungibleLocalId {
    NonFungibleLocalId::UUID(241008287272164729465721528295504357972)
}

pub fn non_fungible_local_integer() -> NonFungibleLocalId {
    NonFungibleLocalId::Integer(1)
}

pub fn non_fungible_local_string() -> NonFungibleLocalId {
    NonFungibleLocalId::String("Scrypto".into())
}

pub fn non_fungible_local_bytes() -> NonFungibleLocalId {
    NonFungibleLocalId::Bytes(vec![0x00, 0x01, 0x02, 0x03])
}

pub fn non_fungible_global_uuid() -> NonFungibleGlobalId {
    NonFungibleGlobalId {
        resource_address: NetworkAwareResourceAddress {
            network_id: 0x01,
            address: ECDSA_SECP256K1_TOKEN,
        },
        non_fungible_local_id: scrypto::prelude::NonFungibleLocalId::UUID(
            UUIDNonFungibleLocalId::new(241008287272164729465721528295504357972).unwrap(),
        ),
    }
}

pub fn non_fungible_global_integer() -> NonFungibleGlobalId {
    NonFungibleGlobalId {
        resource_address: NetworkAwareResourceAddress {
            network_id: 0x01,
            address: ECDSA_SECP256K1_TOKEN,
        },
        non_fungible_local_id: scrypto::prelude::NonFungibleLocalId::Integer(
            IntegerNonFungibleLocalId::new(1),
        ),
    }
}

pub fn non_fungible_global_string() -> NonFungibleGlobalId {
    NonFungibleGlobalId {
        resource_address: NetworkAwareResourceAddress {
            network_id: 0x01,
            address: ECDSA_SECP256K1_TOKEN,
        },
        non_fungible_local_id: scrypto::prelude::NonFungibleLocalId::String(
            StringNonFungibleLocalId::new("Scrypto".to_owned()).unwrap(),
        ),
    }
}

pub fn non_fungible_global_bytes() -> NonFungibleGlobalId {
    NonFungibleGlobalId {
        resource_address: NetworkAwareResourceAddress {
            network_id: 0x01,
            address: ECDSA_SECP256K1_TOKEN,
        },
        non_fungible_local_id: scrypto::prelude::NonFungibleLocalId::Bytes(
            BytesNonFungibleLocalId::new(vec![0x01, 0x02, 0x03, 0x04]).unwrap(),
        ),
    }
}
