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
    StringNonFungibleLocalId, UUIDNonFungibleLocalId, CLOCK, ECDSA_SECP256K1_TOKEN, FAUCET_PACKAGE,
    RADIX_TOKEN,
};
use scrypto::prelude::{ManifestBlobRef, ManifestExpression};

use crate::model::address::NetworkAwareNodeId;
use crate::model::value::ast::{
    BucketId, EnumDiscriminator, ManifestAstValue, ManifestAstValueKind, ProofId,
    TransientIdentifier,
};
use crate::utils::checked_copy_u8_slice;

pub fn value() -> ManifestAstValue {
    ManifestAstValue::Bool { value: false }
}

pub fn bool1() -> ManifestAstValue {
    ManifestAstValue::Bool { value: false }
}

pub fn bool2() -> ManifestAstValue {
    ManifestAstValue::Bool { value: true }
}

pub fn u8() -> ManifestAstValue {
    ManifestAstValue::U8 { value: 1 }
}

pub fn u16() -> ManifestAstValue {
    ManifestAstValue::U16 { value: 1 }
}

pub fn u32() -> ManifestAstValue {
    ManifestAstValue::U32 { value: 1 }
}

pub fn u64() -> ManifestAstValue {
    ManifestAstValue::U64 { value: 1 }
}

pub fn u128() -> ManifestAstValue {
    ManifestAstValue::U128 { value: 1 }
}

pub fn i8() -> ManifestAstValue {
    ManifestAstValue::I8 { value: 1 }
}

pub fn i16() -> ManifestAstValue {
    ManifestAstValue::I16 { value: 1 }
}

pub fn i32() -> ManifestAstValue {
    ManifestAstValue::I32 { value: 1 }
}

pub fn i64() -> ManifestAstValue {
    ManifestAstValue::I64 { value: 1 }
}

pub fn i128() -> ManifestAstValue {
    ManifestAstValue::I128 { value: 1 }
}

pub fn string() -> ManifestAstValue {
    ManifestAstValue::String {
        value: "Scrypto".into(),
    }
}

pub fn enum1() -> ManifestAstValue {
    ManifestAstValue::Enum {
        variant: EnumDiscriminator::U8 { discriminator: 1 },
        fields: None,
    }
}

pub fn enum2() -> ManifestAstValue {
    ManifestAstValue::Enum {
        variant: EnumDiscriminator::String {
            discriminator: "EnumName::Variant".into(),
        },
        fields: None,
    }
}

pub fn enum3() -> ManifestAstValue {
    ManifestAstValue::Enum {
        variant: EnumDiscriminator::U8 { discriminator: 1 },
        fields: Some(vec![ManifestAstValue::U8 { value: 1 }]),
    }
}

pub fn enum4() -> ManifestAstValue {
    ManifestAstValue::Enum {
        variant: EnumDiscriminator::String {
            discriminator: "EnumName::Variant".into(),
        },
        fields: Some(vec![ManifestAstValue::U8 { value: 1 }]),
    }
}

pub fn some() -> ManifestAstValue {
    ManifestAstValue::Some {
        value: Box::new(ManifestAstValue::U8 { value: 1 }),
    }
}

pub fn none() -> ManifestAstValue {
    ManifestAstValue::None
}

pub fn ok() -> ManifestAstValue {
    ManifestAstValue::Ok {
        value: Box::new(ManifestAstValue::U8 { value: 1 }),
    }
}

pub fn err() -> ManifestAstValue {
    ManifestAstValue::Err {
        value: Box::new(ManifestAstValue::U8 { value: 1 }),
    }
}

pub fn array() -> ManifestAstValue {
    ManifestAstValue::Array {
        element_kind: ManifestAstValueKind::U8,
        elements: vec![
            ManifestAstValue::U8 { value: 1 },
            ManifestAstValue::U8 { value: 2 },
            ManifestAstValue::U8 { value: 3 },
        ],
    }
}

pub fn map() -> ManifestAstValue {
    ManifestAstValue::Map {
        key_value_kind: ManifestAstValueKind::U8,
        value_value_kind: ManifestAstValueKind::String,
        entries: vec![
            (
                ManifestAstValue::U8 { value: 65 },
                ManifestAstValue::String {
                    value: "A".to_owned(),
                },
            ),
            (
                ManifestAstValue::U8 { value: 66 },
                ManifestAstValue::String {
                    value: "B".to_owned(),
                },
            ),
        ],
    }
}

pub fn tuple() -> ManifestAstValue {
    ManifestAstValue::Tuple {
        elements: vec![ManifestAstValue::Tuple {
            elements: vec![
                ManifestAstValue::U8 { value: 1 },
                ManifestAstValue::String {
                    value: "Something".to_owned(),
                },
            ],
        }],
    }
}

pub fn decimal() -> ManifestAstValue {
    ManifestAstValue::Decimal {
        value: "1".parse().unwrap(),
    }
}

pub fn precise_decimal() -> ManifestAstValue {
    ManifestAstValue::PreciseDecimal {
        value: "1".parse().unwrap(),
    }
}

pub fn address1() -> ManifestAstValue {
    ManifestAstValue::Address {
        address: NetworkAwareNodeId(FAUCET_PACKAGE.as_node_id().0, 1),
    }
}

pub fn address2() -> ManifestAstValue {
    ManifestAstValue::Address {
        address: NetworkAwareNodeId(RADIX_TOKEN.as_node_id().0, 1),
    }
}

pub fn address3() -> ManifestAstValue {
    ManifestAstValue::Address {
        address: NetworkAwareNodeId(CLOCK.as_node_id().0, 1),
    }
}

pub fn bucket1() -> ManifestAstValue {
    ManifestAstValue::Bucket {
        identifier: BucketId(TransientIdentifier::String {
            value: "bucket".to_owned(),
        }),
    }
}

pub fn bucket2() -> ManifestAstValue {
    ManifestAstValue::Bucket {
        identifier: BucketId(TransientIdentifier::U32 { value: 1 }),
    }
}

pub fn proof1() -> ManifestAstValue {
    ManifestAstValue::Proof {
        identifier: ProofId(TransientIdentifier::String {
            value: "proof".to_owned(),
        }),
    }
}

pub fn proof2() -> ManifestAstValue {
    ManifestAstValue::Proof {
        identifier: ProofId(TransientIdentifier::U32 { value: 1 }),
    }
}

pub fn non_fungible_local_id1() -> ManifestAstValue {
    ManifestAstValue::NonFungibleLocalId {
        value: NonFungibleLocalId::UUID(
            UUIDNonFungibleLocalId::new(241008287272164729465721528295504357972).unwrap(),
        ),
    }
}

pub fn non_fungible_local_id2() -> ManifestAstValue {
    ManifestAstValue::NonFungibleLocalId {
        value: NonFungibleLocalId::Integer(IntegerNonFungibleLocalId::new(1)),
    }
}

pub fn non_fungible_local_id3() -> ManifestAstValue {
    ManifestAstValue::NonFungibleLocalId {
        value: NonFungibleLocalId::String(
            StringNonFungibleLocalId::new("Scrypto".to_owned()).unwrap(),
        ),
    }
}

pub fn non_fungible_local_id4() -> ManifestAstValue {
    ManifestAstValue::NonFungibleLocalId {
        value: NonFungibleLocalId::Bytes(
            BytesNonFungibleLocalId::new(vec![0x01, 0x02, 0x03, 0x04]).unwrap(),
        ),
    }
}

pub fn non_fungible_global_id1() -> ManifestAstValue {
    ManifestAstValue::NonFungibleGlobalId {
        resource_address: NetworkAwareNodeId(ECDSA_SECP256K1_TOKEN.as_node_id().0, 0x01),
        non_fungible_local_id: NonFungibleLocalId::UUID(
            UUIDNonFungibleLocalId::new(241008287272164729465721528295504357972).unwrap(),
        ),
    }
}

pub fn non_fungible_global_id2() -> ManifestAstValue {
    ManifestAstValue::NonFungibleGlobalId {
        resource_address: NetworkAwareNodeId(ECDSA_SECP256K1_TOKEN.as_node_id().0, 0x01),
        non_fungible_local_id: NonFungibleLocalId::Integer(IntegerNonFungibleLocalId::new(1)),
    }
}

pub fn non_fungible_global_id3() -> ManifestAstValue {
    ManifestAstValue::NonFungibleGlobalId {
        resource_address: NetworkAwareNodeId(ECDSA_SECP256K1_TOKEN.as_node_id().0, 0x01),
        non_fungible_local_id: NonFungibleLocalId::String(
            StringNonFungibleLocalId::new("Scrypto".to_owned()).unwrap(),
        ),
    }
}

pub fn non_fungible_global_id4() -> ManifestAstValue {
    ManifestAstValue::NonFungibleGlobalId {
        resource_address: NetworkAwareNodeId(ECDSA_SECP256K1_TOKEN.as_node_id().0, 0x01),
        non_fungible_local_id: NonFungibleLocalId::Bytes(
            BytesNonFungibleLocalId::new(vec![0x01, 0x02, 0x03, 0x04]).unwrap(),
        ),
    }
}

pub fn expression1() -> ManifestAstValue {
    ManifestAstValue::Expression {
        value: ManifestExpression::EntireAuthZone,
    }
}

pub fn expression2() -> ManifestAstValue {
    ManifestAstValue::Expression {
        value: ManifestExpression::EntireWorktop,
    }
}

pub fn blob() -> ManifestAstValue {
    ManifestAstValue::Blob {
        hash: ManifestBlobRef(
            checked_copy_u8_slice(
                hex::decode("d28d2c3710601fbc097000ec73455693f4861dc0eb7c90d8821f2a13f617313e")
                    .unwrap(),
            )
            .unwrap(),
        ),
    }
}

pub fn bytes() -> ManifestAstValue {
    ManifestAstValue::Bytes {
        value: hex::decode("d28d2c3710601fbc097000ec73455693f4861dc0eb7c90d8821f2a13f617313e")
            .unwrap(),
    }
}

pub fn enum_discriminator1() -> EnumDiscriminator {
    EnumDiscriminator::String {
        discriminator: "EnumName::Variant".to_owned(),
    }
}

pub fn enum_discriminator2() -> EnumDiscriminator {
    EnumDiscriminator::U8 { discriminator: 1 }
}
