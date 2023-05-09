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

use scrypto::prelude::{
    BytesNonFungibleLocalId, IntegerNonFungibleLocalId, NodeId, NonFungibleLocalId,
    StringNonFungibleLocalId, UUIDNonFungibleLocalId, ACCOUNT_PACKAGE,
};

use crate::model::address::NetworkAwareNodeId;
use crate::model::value::scrypto_sbor::{MapEntry, ScryptoSborValue, ScryptoSborValueKind};

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
        variant_id: 1u8,
        fields: vec![],
    }
}

pub fn enum2() -> ScryptoSborValue {
    ScryptoSborValue::Enum {
        variant_id: 1u8,
        fields: vec![ScryptoSborValue::U8 { value: 1 }],
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
        key_kind: ScryptoSborValueKind::U8,
        value_kind: ScryptoSborValueKind::String,
        entries: vec![
            MapEntry {
                key: ScryptoSborValue::U8 { value: 65 },
                value: ScryptoSborValue::String {
                    value: "A".to_owned(),
                },
            },
            MapEntry {
                key: ScryptoSborValue::U8 { value: 66 },
                value: ScryptoSborValue::String {
                    value: "B".to_owned(),
                },
            },
        ],
    }
}

pub fn tuple() -> ScryptoSborValue {
    ScryptoSborValue::Tuple {
        fields: vec![ScryptoSborValue::Tuple {
            fields: vec![
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

pub fn own() -> ScryptoSborValue {
    ScryptoSborValue::Own {
        value: NetworkAwareNodeId(ACCOUNT_PACKAGE.as_node_id().0, 1), /* TODO: Replace with
                                                                       * something that can
                                                                       * actually be owned */
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

pub fn reference() -> ScryptoSborValue {
    ScryptoSborValue::Reference {
        value: NetworkAwareNodeId([0; NodeId::LENGTH], 1),
    }
}

pub fn bytes() -> ScryptoSborValue {
    ScryptoSborValue::Bytes {
        element_kind: ScryptoSborValueKind::U8,
        hex: hex::decode("d28d2c3710601fbc097000ec73455693f4861dc0eb7c90d8821f2a13f617313e")
            .unwrap(),
    }
}
