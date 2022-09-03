use scrypto::prelude::{Decimal, Hash, NonFungibleAddress, NonFungibleId, PreciseDecimal};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

use crate::error::Error;
use crate::models::serde::*;

// ======
// Value
// ======

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq, Clone)]
#[serde(tag = "type")]

pub enum Value {
    Unit,
    Bool {
        value: bool,
    },

    U8 {
        #[serde_as(as = "DisplayFromStr")]
        value: u8,
    },
    U16 {
        #[serde_as(as = "DisplayFromStr")]
        value: u16,
    },
    U32 {
        #[serde_as(as = "DisplayFromStr")]
        value: u32,
    },
    U64 {
        #[serde_as(as = "DisplayFromStr")]
        value: u64,
    },
    U128 {
        #[serde_as(as = "DisplayFromStr")]
        value: u128,
    },

    I8 {
        #[serde_as(as = "DisplayFromStr")]
        value: i8,
    },
    I16 {
        #[serde_as(as = "DisplayFromStr")]
        value: i16,
    },
    I32 {
        #[serde_as(as = "DisplayFromStr")]
        value: i32,
    },
    I64 {
        #[serde_as(as = "DisplayFromStr")]
        value: i64,
    },
    I128 {
        #[serde_as(as = "DisplayFromStr")]
        value: i128,
    },

    String {
        value: String,
    },

    Struct {
        fields: Vec<Value>,
    },
    Enum {
        variant_name: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        fields: Option<Vec<Value>>,
    },
    Option {
        #[serde(default)]
        value: Box<Option<Value>>,
    },
    Result {
        value: Box<Result<Value, Value>>,
    },

    Array {
        element_type: ValueKind,
        elements: Vec<Value>,
    },
    Tuple {
        elements: Vec<Value>,
    },

    List {
        element_type: ValueKind,
        elements: Vec<Value>,
    },
    Set {
        element_type: ValueKind,
        elements: Vec<Value>,
    },
    Map {
        key_type: ValueKind,
        value_type: ValueKind,
        elements: Vec<Value>,
    },

    Decimal {
        #[serde_as(as = "DisplayFromStr")]
        value: Decimal,
    },
    PreciseDecimal {
        #[serde_as(as = "DisplayFromStr")]
        value: PreciseDecimal,
    },

    ComponentAddress {
        address: NetworkAwareComponentAddress,
    },
    ResourceAddress {
        address: NetworkAwareResourceAddress,
    },
    PackageAddress {
        address: NetworkAwarePackageAddress,
    },

    Hash {
        #[serde(with = "HashDef")]
        value: Hash,
    },
    Bucket {
        identifier: Identifier,
    },
    Proof {
        identifier: Identifier,
    },
    NonFungibleId {
        #[serde_as(as = "DisplayFromStr")]
        value: NonFungibleId,
    },
    NonFungibleAddress {
        #[serde_as(as = "DisplayFromStr")]
        address: NonFungibleAddress,
    },

    Bytes {
        #[serde(with = "hex::serde")]
        value: Vec<u8>,
    },
}

impl Value {
    pub fn kind(&self) -> ValueKind {
        match self {
            Self::Unit => ValueKind::Unit,
            Self::Bool { .. } => ValueKind::Bool,

            Self::I8 { .. } => ValueKind::I8,
            Self::I16 { .. } => ValueKind::I16,
            Self::I32 { .. } => ValueKind::I32,
            Self::I64 { .. } => ValueKind::I64,
            Self::I128 { .. } => ValueKind::I128,

            Self::U8 { .. } => ValueKind::U8,
            Self::U16 { .. } => ValueKind::U16,
            Self::U32 { .. } => ValueKind::U32,
            Self::U64 { .. } => ValueKind::U64,
            Self::U128 { .. } => ValueKind::U128,

            Self::String { .. } => ValueKind::String,

            Self::Struct { .. } => ValueKind::Struct,
            Self::Enum { .. } => ValueKind::Enum,

            Self::Option { .. } => ValueKind::Option,
            Self::Array { .. } => ValueKind::Array,
            Self::Tuple { .. } => ValueKind::Tuple,
            Self::Result { .. } => ValueKind::Result,

            Self::List { .. } => ValueKind::List,
            Self::Set { .. } => ValueKind::Set,
            Self::Map { .. } => ValueKind::Map,

            Self::Decimal { .. } => ValueKind::Decimal,
            Self::PreciseDecimal { .. } => ValueKind::PreciseDecimal,

            Self::PackageAddress { .. } => ValueKind::PackageAddress,
            Self::ComponentAddress { .. } => ValueKind::ComponentAddress,
            Self::ResourceAddress { .. } => ValueKind::ResourceAddress,

            Self::Hash { .. } => ValueKind::Hash,

            Self::Bucket { .. } => ValueKind::Bucket,
            Self::Proof { .. } => ValueKind::Proof,

            Self::NonFungibleId { .. } => ValueKind::NonFungibleId,
            Self::NonFungibleAddress { .. } => ValueKind::NonFungibleAddress,

            Self::Bytes { .. } => ValueKind::Bytes,
        }
    }

    pub fn validate_kind(&self, expected_kind: ValueKind) -> Result<(), Error> {
        if self.kind() == expected_kind {
            Ok(())
        } else {
            Err(Error::InvalidType {
                expected_type: expected_kind,
                actual_type: self.kind(),
            })
        }
    }

    pub fn validate_if_collection(&self) -> Result<(), Error> {
        match self {
            Self::Array {
                element_type,
                elements,
            }
            | Self::List {
                element_type,
                elements,
            }
            | Self::Set {
                element_type,
                elements,
            } => {
                elements
                    .iter()
                    .map(|item| match item.validate_if_collection() {
                        Ok(_) => item.validate_kind(*element_type),
                        Err(error) => Err(error),
                    })
                    .collect::<Result<Vec<()>, _>>()?;
                Ok(())
            }
            Self::Map {
                key_type,
                value_type,
                elements,
            } => {
                elements
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| i % 2 == 0)
                    .map(|(_, item)| match item.validate_if_collection() {
                        Ok(_) => item.validate_kind(*key_type),
                        Err(error) => Err(error),
                    })
                    .collect::<Result<Vec<()>, _>>()?;
                elements
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| i % 2 != 0)
                    .map(|(_, item)| match item.validate_if_collection() {
                        Ok(_) => item.validate_kind(*value_type),
                        Err(error) => Err(error),
                    })
                    .collect::<Result<Vec<()>, _>>()?;
                Ok(())
            }
            // Not a collection. No validation required.
            _ => Ok(()),
        }
    }
}

// ===============================
// ValueKind Type and Conversions
// ===============================

macro_rules! define_value_kind{
    (
     $(#[$meta:meta])*
     $vis:vis enum $enum_ident:ident {
        $(
            $(#[$variant_metadata:meta])*
            $variant_ident:ident
        ),*$(,)*
    }
    ) => {
        $(#[$meta])*
        $vis enum $enum_ident {
            $(
                $(#[$variant_metadata])*
                $variant_ident,
            )*
        }

        impl Into<transaction::manifest::ast::Type> for $enum_ident {
            fn into(self) -> transaction::manifest::ast::Type {
                match self {
                    $(
                        Self::$variant_ident => transaction::manifest::ast::Type::$variant_ident,
                    )*
                }
            }
        }

        impl From<transaction::manifest::ast::Type> for $enum_ident {
            fn from(value: transaction::manifest::ast::Type) -> $enum_ident {
                match value {
                    $(
                        transaction::manifest::ast::Type::$variant_ident => Self::$variant_ident,
                    )*
                }
            }
        }
    }
}

define_value_kind! {
    #[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub enum ValueKind {
        Unit,
        Bool,

        I8,
        I16,
        I32,
        I64,
        I128,

        U8,
        U16,
        U32,
        U64,
        U128,

        String,

        Struct,
        Enum,

        Option,
        Array,
        Tuple,
        Result,

        List,
        Set,
        Map,

        Decimal,
        PreciseDecimal,

        PackageAddress,
        ComponentAddress,
        ResourceAddress,

        Hash,

        Bucket,
        Proof,

        NonFungibleId,
        NonFungibleAddress,

        Bytes,
    }
}

// ===========
// Unit Tests
// ===========

#[cfg(test)]
mod tests {
    use super::{Value, ValueKind};
    use crate::models::serde::{
        NetworkAwareComponentAddress, 
        NetworkAwareResourceAddress,
        NetworkAwarePackageAddress, 
    };
    use scrypto::prelude::*;

    macro_rules! test_value {
        ($string: expr, $value: expr) => {
            assert_serialization_matches($string, $value);
            assert_deserialization_matches($string, $value);
        };
    }

    fn assert_serialization_matches(string: &str, value: Value) {
        let serialized_string: String =
            serde_json::to_string(&value).expect("Serialization of trusted value failed");

        let string = string.replace("\n", "").replace(" ", "");
        let serialized_string = serialized_string.replace("\n", "").replace(" ", "");
        assert_eq!(string, serialized_string);
    }

    fn assert_deserialization_matches(string: &str, value: Value) {
        let deserialized_value: Value = serde_json::from_str(string).unwrap();
        assert_eq!(value, deserialized_value);
    }

    #[test]
    fn test_primitive_types() {
        test_value! {
            r#"
            {
                "type": "U8",
                "value": "192"
            }
            "#,
            Value::U8 { value: 192 }
        };
        test_value! {
            r#"
            {
                "type": "U16",
                "value": "18947"
            }
            "#,
            Value::U16 { value: 18947 }
        };
        test_value! {
            r#"
            {
                "type": "U32",
                "value": "1144418947"
            }
            "#,
            Value::U32 { value: 1144418947 }
        };
        test_value! {
            r#"
            {
                "type": "U64",
                "value": "114441894733333"
            }
            "#,
            Value::U64 {
                value: 114441894733333,
            }
        };
        test_value! {
            r#"
            {
                "type": "U128",
                "value": "11444189334733333"
            }
            "#,
            Value::U128 {
                value: 11444189334733333,
            }
        };

        test_value! {
            r#"
            {
                "type": "I8",
                "value": "-100"
            }
            "#,
            Value::I8 { value: -100 }
        };
        test_value! {
            r#"
            {
                "type": "I16",
                "value": "-18947"
            }
            "#,
            Value::I16 { value: -18947 }
        };
        test_value! {
            r#"
            {
                "type": "I32",
                "value": "-1144418947"
            }
            "#,
            Value::I32 { value: -1144418947 }
        };
        test_value! {
            r#"
            {
                "type": "I64",
                "value": "-114441894733333"
            }
            "#,
            Value::I64 {
                value: -114441894733333,
            }
        };
        test_value! {
            r#"
            {
                "type": "I128",
                "value": "-11444189334733333"
            }
            "#,
            Value::I128 {
                value: -11444189334733333,
            }
        };

        test_value! {
            r#"
            {
                "type": "String",
                "value": "Hello World!"
            }
            "#,
            Value::String {
                value: "Hello World!".into(),
            }
        };

        test_value! {
            r#"
            {
                "type": "Struct",
                "fields": [
                    {
                        "type": "String",
                        "value": "Hello World!"
                    },
                    {
                        "type": "U8",
                        "value": "179"
                    }
                ]
            }
            "#,
            Value::Struct {
                fields: vec![
                    Value::String {
                        value: "Hello World!".into()
                    },
                    Value::U8 {
                        value: 179
                    }
                ]
            }
        };
        test_value! {
            r#"
            {
                "type": "Enum",
                "variant_name": "Component"
            }
            "#,
            Value::Enum {
                variant_name: "Component".into(),
                fields: None,
            }
        };
        test_value! {
            r#"
            {
                "type": "Enum",
                "variant_name": "Component",
                "fields": [
                    {
                        "type": "String",
                        "value": "Account"
                    }
                ]
            }
            "#,
            Value::Enum {
                variant_name: "Component".into(),
                fields: Some(vec![
                    Value::String { value: "Account".into() }
                ])
            }
        };

        // TODO: I'm really unhappy with the way that this is serialized. I want something that
        // looks more like the enum type. Something like:
        // {
        //      "type": "Option",
        //      "variant_name": "None",
        // }
        // Need to look into how this can be done with Serde.
        test_value! {
            r#"
            {
                "type": "Option",
                "value": null
            }
            "#,
            Value::Option {
                value: Box::new(None),
            }
        };

        // TODO: I'm really unhappy with the way that this is serialized. I want something that
        // looks more like the enum type. Something like:
        // {
        //      "type": "Option",
        //      "variant_name": "Some",
        //      "field": {
        //          "type": "String"
        //          "value": "Hello World!"
        //      }
        // }
        // Need to look into how this can be done with Serde.
        test_value! {
            r#"
            {
                "type": "Option",
                "value": {
                    "type": "String",
                    "value": "Hello World!"
                }
            }
            "#,
            Value::Option {
                value: Box::new(Some(Value::String {
                    value: "Hello World!".into(),
                })),
            }
        };

        // TODO: I'm really unhappy with the way that this is serialized. I want something that
        // looks more like the enum type. Something like:
        // {
        //      "type": "Result",
        //      "variant_name": "Ok",
        //      "field": {
        //          "type": "String"
        //          "value": "Hello World!"
        //      }
        // }
        // Need to look into how this can be done with Serde.
        test_value! {
            r#"
            {
                "type": "Result",
                "value": {
                    "Ok": {
                        "type": "String",
                        "value": "This is ok"
                    }
                }
            }
            "#,
            Value::Result {
                value: Box::new(Ok(Value::String {
                    value: "This is ok".into(),
                })),
            }
        };

        test_value! {
            r#"
            {
                "type": "Array",
                "element_type": "Decimal",
                "elements": [
                    {
                        "type": "Decimal",
                        "value": "192.38"
                    },
                    {
                        "type": "Decimal",
                        "value": "10012"
                    }
                ]
            }
            "#,
            Value::Array {
                element_type: ValueKind::Decimal,
                elements: vec![
                    Value::Decimal {
                        value: dec!("192.38"),
                    },
                    Value::Decimal {
                        value: dec!("10012"),
                    },
                ],
            }
        };

        test_value! {
            r#"
            {
                "type": "Tuple",
                "elements": [
                    {
                        "type": "Decimal",
                        "value": "192.38"
                    },
                    {
                        "type": "NonFungibleId",
                        "value": "3007100000000b3ce8b6056e62b902e029623df6df5c"
                    },
                    {
                        "type": "Bucket",
                        "identifier": "my_xrd_bucket"
                    }
                ]
            }
            "#,
            Value::Tuple {
                elements: vec![
                    Value::Decimal {
                        value: dec!("192.38")
                    },
                    Value::NonFungibleId {
                        value: NonFungibleId::from_str("3007100000000b3ce8b6056e62b902e029623df6df5c").unwrap()
                    },
                    Value::Bucket {
                        identifier: crate::models::serde::Identifier::String("my_xrd_bucket".into())
                    }
                ]
            }
        };

        test_value! {
            r#"
            {
                "type": "List",
                "element_type": "Decimal",
                "elements": [
                    {
                        "type": "Decimal",
                        "value": "192.38"
                    },
                    {
                        "type": "Decimal",
                        "value": "10012"
                    }
                ]
            }
            "#,
            Value::List {
                element_type: ValueKind::Decimal,
                elements: vec![
                    Value::Decimal {
                        value: dec!("192.38"),
                    },
                    Value::Decimal {
                        value: dec!("10012"),
                    },
                ],
            }
        };
        
        test_value! {
            r#"
            {
                "type": "Set",
                "element_type": "Decimal",
                "elements": [
                    {
                        "type": "Decimal",
                        "value": "192.38"
                    },
                    {
                        "type": "Decimal",
                        "value": "10012"
                    }
                ]
            }
            "#,
            Value::Set {
                element_type: ValueKind::Decimal,
                elements: vec![
                    Value::Decimal {
                        value: dec!("192.38"),
                    },
                    Value::Decimal {
                        value: dec!("10012"),
                    },
                ],
            }
        };
        
        test_value! {
            r#"
            {
                "type": "Map",
                "key_type": "String",
                "value_type": "Decimal",
                "elements": [
                    {
                        "type": "String",
                        "value": "Toyota Camry"
                    },
                    {
                        "type": "Decimal",
                        "value": "80000"
                    },
                    
                    {
                        "type": "String",
                        "value": "Ford Raptor"
                    },
                    {
                        "type": "Decimal",
                        "value": "170000"
                    }
                ]
            }
            "#,
            Value::Map {
                key_type: ValueKind::String,
                value_type: ValueKind::Decimal,
                elements: vec![
                    Value::String { value: "Toyota Camry".into() },
                    Value::Decimal { value: dec!("80000") },
                    
                    Value::String { value: "Ford Raptor".into() },
                    Value::Decimal { value: dec!("170000") },
                ]
            }
        };
        
        test_value! {
            r#"
            {
                "type": "Decimal",
                "value": "100"
            }
            "#,
            Value::Decimal { 
                value: dec!("100")
            }
        };
        test_value! {
            r#"
            {
                "type": "PreciseDecimal",
                "value": "100"
            }
            "#,
            Value::PreciseDecimal { 
                value: pdec!("100")
            }
        };

        test_value! {
            r#"
            {
                "type": "ComponentAddress",
                "address": "account_sim1qwssnwt0yzhzjydxj7u9uvnljtgaug23re8p32jrjecqajtsvr"
            }
            "#,
            Value::ComponentAddress { 
                address: NetworkAwareComponentAddress {
                    network_id: 0xf2,
                    address: scrypto::address::Bech32Decoder::new(&NetworkDefinition::local_simulator())
                        .validate_and_decode_component_address("account_sim1qwssnwt0yzhzjydxj7u9uvnljtgaug23re8p32jrjecqajtsvr")
                        .unwrap()
                }    
            }
        };
        test_value! {
            r#"
            {
                "type": "PackageAddress",
                "address": "package_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqsnznk7n"
            }
            "#,
            Value::PackageAddress { 
                address: NetworkAwarePackageAddress {
                    network_id: 0xf2,
                    address: scrypto::address::Bech32Decoder::new(&NetworkDefinition::local_simulator())
                        .validate_and_decode_package_address("package_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqsnznk7n")
                        .unwrap()
                }    
            }
        };
        test_value! {
            r#"
            {
                "type": "ResourceAddress",
                "address": "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqshxgp7h"
            }
            "#,
            Value::ResourceAddress { 
                address: NetworkAwareResourceAddress {
                    network_id: 0xf2,
                    address: scrypto::address::Bech32Decoder::new(&NetworkDefinition::local_simulator())
                        .validate_and_decode_resource_address("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqshxgp7h")
                        .unwrap()
                }    
            }
        };
        
        test_value! {
            r#"
            {
                "type": "Hash",
                "value": "910edb2dabf107c7628ecdb9126535676d61bc39a843475f3057d809bfd2d65d"
            }
            "#,
            Value::Hash { 
                value: Hash::from_str("910edb2dabf107c7628ecdb9126535676d61bc39a843475f3057d809bfd2d65d").unwrap()
            }
        };
        
        test_value! {
            r#"
            {
                "type": "Bucket",
                "identifier": 192
            }
            "#,
            Value::Bucket { 
                identifier: crate::models::serde::Identifier::U32(192)
            }
        };
        test_value! {
            r#"
            {
                "type": "Bucket",
                "identifier": "HelloBucket"
            }
            "#,
            Value::Bucket { 
                identifier: crate::models::serde::Identifier::String("HelloBucket".into())
            }
        };
        
        test_value! {
            r#"
            {
                "type": "Proof",
                "identifier": 192
            }
            "#,
            Value::Proof { 
                identifier: crate::models::serde::Identifier::U32(192)
            }
        };
        test_value! {
            r#"
            {
                "type": "Proof",
                "identifier": "HelloProof"
            }
            "#,
            Value::Proof { 
                identifier: crate::models::serde::Identifier::String("HelloProof".into())
            }
        };

        test_value! {
            r#"
            {
                "type": "NonFungibleId",
                "value": "3007100000000b3ce8b6056e62b902e029623df6df5c"
            }
            "#,
            Value::NonFungibleId {
                value: NonFungibleId::from_str("3007100000000b3ce8b6056e62b902e029623df6df5c").unwrap()
            }
        };
        
        test_value! {
            r#"
            {
                "type": "Bytes",
                "value": "0307100000000b3ce8b6056e62b902e029623df6df5c0307100000000b3ce8b6056e62b902e029623df6df5c"
            }
            "#,
            Value::Bytes {
                value: hex::decode("0307100000000b3ce8b6056e62b902e029623df6df5c0307100000000b3ce8b6056e62b902e029623df6df5c").unwrap()
            }
        };
    }
}
