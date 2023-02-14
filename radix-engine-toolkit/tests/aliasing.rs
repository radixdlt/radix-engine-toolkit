use radix_engine_toolkit::{
    model::Value, traverse_value, NonFungibleGlobalId, ValueAliasingVisitor, ValueKind,
};
use scrypto::prelude::IntegerNonFungibleLocalId;

#[test]
fn aliasing_of_deeply_nested_structures_works() {
    // Arrange
    let mut value = Value::Map {
        key_value_kind: ValueKind::String,
        value_value_kind: ValueKind::Tuple,
        entries: vec![
            (
                Value::String {
                    value: "HelloWorld".into(),
                },
                Value::Tuple {
                    elements: vec![
                        Value::Decimal {
                            value: "12".parse().unwrap(),
                        },
                        Value::PreciseDecimal {
                            value: "12".parse().unwrap(),
                        },
                    ],
                },
            ),
            (
                Value::String {
                    value: "WorldHello".into(),
                },
                Value::Tuple {
                    elements: vec![Value::Tuple {
                        elements: vec![Value::Tuple {
                            elements: vec![Value::Array {
                                element_kind: ValueKind::Array,
                                elements: vec![Value::Array {
                                    element_kind: ValueKind::Tuple,
                                    elements: vec![Value::Tuple { elements: vec![
                                        Value::ResourceAddress { address: "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety".parse().unwrap() },
                                        Value::NonFungibleLocalId { value: scrypto::prelude::NonFungibleLocalId::Integer(IntegerNonFungibleLocalId::new(1)) } ,
                                    ] }],
                                }],
                            }],
                        }],
                    }],
                },
            ),
        ],
    };

    let expected = Value::Map {
        key_value_kind: ValueKind::String,
        value_value_kind: ValueKind::Tuple,
        entries: vec![
            (
                Value::String {
                    value: "HelloWorld".into(),
                },
                Value::Tuple {
                    elements: vec![
                        Value::Decimal {
                            value: "12".parse().unwrap(),
                        },
                        Value::PreciseDecimal {
                            value: "12".parse().unwrap(),
                        },
                    ],
                },
            ),
            (
                Value::String {
                    value: "WorldHello".into(),
                },
                Value::Tuple {
                    elements: vec![Value::Tuple {
                        elements: vec![Value::Tuple {
                            elements: vec![Value::Array {
                                element_kind: ValueKind::Array,
                                elements: vec![Value::Array {
                                    element_kind: ValueKind::Tuple,
                                    elements: vec![Value::NonFungibleGlobalId { address: NonFungibleGlobalId {
                                        resource_address: "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety".parse().unwrap(),
                                        non_fungible_local_id: scrypto::prelude::NonFungibleLocalId::Integer(IntegerNonFungibleLocalId::new(1))
                                    } }],
                                }],
                            }],
                        }],
                    }],
                },
            ),
        ],
    };

    let mut visitor = ValueAliasingVisitor::default();

    // Act
    traverse_value(&mut value, &mut [&mut visitor]).unwrap();

    // Assert
    assert_eq!(expected, value)
}
