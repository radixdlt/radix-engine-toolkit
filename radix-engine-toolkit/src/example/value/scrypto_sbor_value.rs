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

use radix_engine_common::data::scrypto::model::OBJECT_ID_LENGTH;
use scrypto::prelude::{
    BytesNonFungibleLocalId, IntegerNonFungibleLocalId, NonFungibleLocalId, Own,
    StringNonFungibleLocalId, UUIDNonFungibleLocalId, FAUCET_COMPONENT, FAUCET_PACKAGE,
    RADIX_TOKEN,
};

use crate::model::address::*;
use crate::model::value::scrypto_sbor::{ScryptoSborValue, ScryptoSborValueKind};

pub fn value() -> ScryptoSborValue {
    ScryptoSborValue::Bool { value: false }
}

pub fn bool1() -> ScryptoSborValue {
    ScryptoSborValue::Bool { value: false }
}

pub fn bool2() -> ScryptoSborValue {
    ScryptoSborValue::Bool { value: true }
}

pub fn u8() -> ScryptoSborValue {
    ScryptoSborValue::U8 { value: 1 }
}

pub fn u16() -> ScryptoSborValue {
    ScryptoSborValue::U16 { value: 1 }
}

pub fn u32() -> ScryptoSborValue {
    ScryptoSborValue::U32 { value: 1 }
}

pub fn u64() -> ScryptoSborValue {
    ScryptoSborValue::U64 { value: 1 }
}

pub fn u128() -> ScryptoSborValue {
    ScryptoSborValue::U128 { value: 1 }
}

pub fn i8() -> ScryptoSborValue {
    ScryptoSborValue::I8 { value: 1 }
}

pub fn i16() -> ScryptoSborValue {
    ScryptoSborValue::I16 { value: 1 }
}

pub fn i32() -> ScryptoSborValue {
    ScryptoSborValue::I32 { value: 1 }
}

pub fn i64() -> ScryptoSborValue {
    ScryptoSborValue::I64 { value: 1 }
}

pub fn i128() -> ScryptoSborValue {
    ScryptoSborValue::I128 { value: 1 }
}

pub fn string() -> ScryptoSborValue {
    ScryptoSborValue::String {
        value: "Scrypto".into(),
    }
}

pub fn enum1() -> ScryptoSborValue {
    ScryptoSborValue::Enum {
        variant: 1u8,
        fields: None,
    }
}

pub fn enum2() -> ScryptoSborValue {
    ScryptoSborValue::Enum {
        variant: 1u8,
        fields: Some(vec![ScryptoSborValue::U8 { value: 1 }]),
    }
}

pub fn array() -> ScryptoSborValue {
    ScryptoSborValue::Array {
        element_kind: ScryptoSborValueKind::U8,
        elements: vec![
            ScryptoSborValue::U8 { value: 1 },
            ScryptoSborValue::U8 { value: 2 },
            ScryptoSborValue::U8 { value: 3 },
        ],
    }
}

pub fn map() -> ScryptoSborValue {
    ScryptoSborValue::Map {
        key_value_kind: ScryptoSborValueKind::U8,
        value_value_kind: ScryptoSborValueKind::String,
        entries: vec![
            (
                ScryptoSborValue::U8 { value: 65 },
                ScryptoSborValue::String {
                    value: "A".to_owned(),
                },
            ),
            (
                ScryptoSborValue::U8 { value: 66 },
                ScryptoSborValue::String {
                    value: "B".to_owned(),
                },
            ),
        ],
    }
}

pub fn tuple() -> ScryptoSborValue {
    ScryptoSborValue::Tuple {
        elements: vec![ScryptoSborValue::Tuple {
            elements: vec![
                ScryptoSborValue::U8 { value: 1 },
                ScryptoSborValue::String {
                    value: "Something".to_owned(),
                },
            ],
        }],
    }
}

pub fn decimal() -> ScryptoSborValue {
    ScryptoSborValue::Decimal {
        value: "1".parse().unwrap(),
    }
}

pub fn precise_decimal() -> ScryptoSborValue {
    ScryptoSborValue::PreciseDecimal {
        value: "1".parse().unwrap(),
    }
}

pub fn address1() -> ScryptoSborValue {
    ScryptoSborValue::Address {
        address: EntityAddress::ComponentAddress {
            address: NetworkAwareComponentAddress {
                network_id: 0x01,
                address: FAUCET_COMPONENT,
            },
        },
    }
}

pub fn address2() -> ScryptoSborValue {
    ScryptoSborValue::Address {
        address: EntityAddress::ResourceAddress {
            address: NetworkAwareResourceAddress {
                network_id: 0x01,
                address: RADIX_TOKEN,
            },
        },
    }
}

pub fn address3() -> ScryptoSborValue {
    ScryptoSborValue::Address {
        address: EntityAddress::PackageAddress {
            address: NetworkAwarePackageAddress {
                network_id: 0x01,
                address: FAUCET_PACKAGE,
            },
        },
    }
}

pub fn own1() -> ScryptoSborValue {
    ScryptoSborValue::Own {
        value: Own::Bucket([0; OBJECT_ID_LENGTH]),
    }
}

pub fn own2() -> ScryptoSborValue {
    ScryptoSborValue::Own {
        value: Own::Proof([0; OBJECT_ID_LENGTH]),
    }
}

pub fn own3() -> ScryptoSborValue {
    ScryptoSborValue::Own {
        value: Own::Vault([0; OBJECT_ID_LENGTH]),
    }
}

pub fn own4() -> ScryptoSborValue {
    ScryptoSborValue::Own {
        value: Own::Object([0; OBJECT_ID_LENGTH]),
    }
}

pub fn own5() -> ScryptoSborValue {
    ScryptoSborValue::Own {
        value: Own::KeyValueStore([0; OBJECT_ID_LENGTH]),
    }
}

pub fn non_fungible_local_id1() -> ScryptoSborValue {
    ScryptoSborValue::NonFungibleLocalId {
        value: NonFungibleLocalId::UUID(
            UUIDNonFungibleLocalId::new(241008287272164729465721528295504357972).unwrap(),
        ),
    }
}

pub fn non_fungible_local_id2() -> ScryptoSborValue {
    ScryptoSborValue::NonFungibleLocalId {
        value: NonFungibleLocalId::Integer(IntegerNonFungibleLocalId::new(1)),
    }
}

pub fn non_fungible_local_id3() -> ScryptoSborValue {
    ScryptoSborValue::NonFungibleLocalId {
        value: NonFungibleLocalId::String(
            StringNonFungibleLocalId::new("Scrypto".to_owned()).unwrap(),
        ),
    }
}

pub fn non_fungible_local_id4() -> ScryptoSborValue {
    ScryptoSborValue::NonFungibleLocalId {
        value: NonFungibleLocalId::Bytes(
            BytesNonFungibleLocalId::new(vec![0x01, 0x02, 0x03, 0x04]).unwrap(),
        ),
    }
}
