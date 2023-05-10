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
    BytesNonFungibleLocalId, IntegerNonFungibleLocalId, NonFungibleLocalId,
    StringNonFungibleLocalId, UUIDNonFungibleLocalId, CLOCK, FAUCET_PACKAGE, RADIX_TOKEN,
};
use scrypto::prelude::{ManifestBlobRef, ManifestExpression};

use crate::model::address::NetworkAwareNodeId;
use crate::model::value::manifest_sbor::{ManifestSborValue, ManifestSborValueKind};
use crate::model::value::scrypto_sbor::MapEntry;
use crate::utils::checked_copy_u8_slice;

pub fn value() -> ManifestSborValue {
    ManifestSborValue::Bool { value: false }
}

pub fn bool1() -> ManifestSborValue {
    ManifestSborValue::Bool { value: false }
}

pub fn bool2() -> ManifestSborValue {
    ManifestSborValue::Bool { value: true }
}

pub fn u8() -> ManifestSborValue {
    ManifestSborValue::U8 { value: 1 }
}

pub fn u16() -> ManifestSborValue {
    ManifestSborValue::U16 { value: 1 }
}

pub fn u32() -> ManifestSborValue {
    ManifestSborValue::U32 { value: 1 }
}

pub fn u64() -> ManifestSborValue {
    ManifestSborValue::U64 { value: 1 }
}

pub fn u128() -> ManifestSborValue {
    ManifestSborValue::U128 { value: 1 }
}

pub fn i8() -> ManifestSborValue {
    ManifestSborValue::I8 { value: 1 }
}

pub fn i16() -> ManifestSborValue {
    ManifestSborValue::I16 { value: 1 }
}

pub fn i32() -> ManifestSborValue {
    ManifestSborValue::I32 { value: 1 }
}

pub fn i64() -> ManifestSborValue {
    ManifestSborValue::I64 { value: 1 }
}

pub fn i128() -> ManifestSborValue {
    ManifestSborValue::I128 { value: 1 }
}

pub fn string() -> ManifestSborValue {
    ManifestSborValue::String {
        value: "Scrypto".into(),
    }
}

pub fn enum1() -> ManifestSborValue {
    ManifestSborValue::Enum {
        variant_id: 1,
        fields: vec![],
        type_name: None,
        variant_name: None,
    }
}

pub fn enum2() -> ManifestSborValue {
    ManifestSborValue::Enum {
        variant_id: 1,
        fields: vec![ManifestSborValue::U8 { value: 1 }.into()],
        type_name: None,
        variant_name: None,
    }
}

pub fn array() -> ManifestSborValue {
    ManifestSborValue::Array {
        element_kind: ManifestSborValueKind::U8,
        elements: vec![
            ManifestSborValue::U8 { value: 1 },
            ManifestSborValue::U8 { value: 2 },
            ManifestSborValue::U8 { value: 3 },
        ],
    }
}

pub fn map() -> ManifestSborValue {
    ManifestSborValue::Map {
        key_kind: ManifestSborValueKind::U8,
        value_kind: ManifestSborValueKind::String,
        entries: vec![
            MapEntry {
                key: ManifestSborValue::U8 { value: 65 },
                value: ManifestSborValue::String {
                    value: "A".to_owned(),
                },
            },
            MapEntry {
                key: ManifestSborValue::U8 { value: 66 },
                value: ManifestSborValue::String {
                    value: "B".to_owned(),
                },
            },
        ],
    }
}

pub fn tuple() -> ManifestSborValue {
    ManifestSborValue::Tuple {
        type_name: None,
        fields: vec![ManifestSborValue::Tuple {
            type_name: None,
            fields: vec![
                ManifestSborValue::U8 { value: 1 }.into(),
                ManifestSborValue::String {
                    value: "Something".to_owned(),
                }
                .into(),
            ],
        }
        .into()],
    }
}

pub fn decimal() -> ManifestSborValue {
    ManifestSborValue::Decimal {
        value: "1".parse().unwrap(),
    }
}

pub fn precise_decimal() -> ManifestSborValue {
    ManifestSborValue::PreciseDecimal {
        value: "1".parse().unwrap(),
    }
}

pub fn address1() -> ManifestSborValue {
    ManifestSborValue::Address {
        value: NetworkAwareNodeId(FAUCET_PACKAGE.as_node_id().0, 1),
    }
}

pub fn address2() -> ManifestSborValue {
    ManifestSborValue::Address {
        value: NetworkAwareNodeId(RADIX_TOKEN.as_node_id().0, 1),
    }
}

pub fn address3() -> ManifestSborValue {
    ManifestSborValue::Address {
        value: NetworkAwareNodeId(CLOCK.as_node_id().0, 1),
    }
}

pub fn bucket() -> ManifestSborValue {
    ManifestSborValue::Bucket { value: 1 }
}

pub fn proof() -> ManifestSborValue {
    ManifestSborValue::Proof { value: 1 }
}

pub fn non_fungible_local_id1() -> ManifestSborValue {
    ManifestSborValue::NonFungibleLocalId {
        value: NonFungibleLocalId::UUID(
            UUIDNonFungibleLocalId::new(241008287272164729465721528295504357972).unwrap(),
        ),
    }
}

pub fn non_fungible_local_id2() -> ManifestSborValue {
    ManifestSborValue::NonFungibleLocalId {
        value: NonFungibleLocalId::Integer(IntegerNonFungibleLocalId::new(1)),
    }
}

pub fn non_fungible_local_id3() -> ManifestSborValue {
    ManifestSborValue::NonFungibleLocalId {
        value: NonFungibleLocalId::String(
            StringNonFungibleLocalId::new("Scrypto".to_owned()).unwrap(),
        ),
    }
}

pub fn non_fungible_local_id4() -> ManifestSborValue {
    ManifestSborValue::NonFungibleLocalId {
        value: NonFungibleLocalId::Bytes(
            BytesNonFungibleLocalId::new(vec![0x01, 0x02, 0x03, 0x04]).unwrap(),
        ),
    }
}

pub fn expression1() -> ManifestSborValue {
    ManifestSborValue::Expression {
        value: ManifestExpression::EntireAuthZone,
    }
}

pub fn expression2() -> ManifestSborValue {
    ManifestSborValue::Expression {
        value: ManifestExpression::EntireWorktop,
    }
}

pub fn blob() -> ManifestSborValue {
    ManifestSborValue::Blob {
        value: ManifestBlobRef(
            checked_copy_u8_slice(
                hex::decode("d28d2c3710601fbc097000ec73455693f4861dc0eb7c90d8821f2a13f617313e")
                    .unwrap(),
            )
            .unwrap(),
        ),
    }
}
