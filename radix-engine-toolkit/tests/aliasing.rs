use radix_engine_toolkit::model::address::NonFungibleGlobalId;
use radix_engine_toolkit::model::value::ast::{ManifestAstValue, ManifestAstValueKind};
use radix_engine_toolkit::visitor::{traverse_value, ValueAliasingVisitor};
use scrypto::prelude::IntegerNonFungibleLocalId;

#[test]
fn aliasing_of_deeply_nested_structures_works() {
    // Arrange
    let mut value = ManifestAstValue::Map {
        key_value_kind: ManifestAstValueKind::String,
        value_value_kind: ManifestAstValueKind::Tuple,
        entries: vec![
            (
                ManifestAstValue::String {
                    value: "HelloWorld".into(),
                },
                ManifestAstValue::Tuple {
                    elements: vec![
                        ManifestAstValue::Decimal {
                            value: "12".parse().unwrap(),
                        },
                        ManifestAstValue::PreciseDecimal {
                            value: "12".parse().unwrap(),
                        },
                    ],
                },
            ),
            (
                ManifestAstValue::String {
                    value: "WorldHello".into(),
                },
                ManifestAstValue::Tuple {
                    elements: vec![ManifestAstValue::Tuple {
                        elements: vec![ManifestAstValue::Tuple {
                            elements: vec![ManifestAstValue::Array {
                                element_kind: ManifestAstValueKind::Array,
                                elements: vec![ManifestAstValue::Array {
                                    element_kind: ManifestAstValueKind::Tuple,
                                    elements: vec![ManifestAstValue::Tuple { elements: vec![
                                        ManifestAstValue::ResourceAddress { address: "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety".parse().unwrap() },
                                        ManifestAstValue::NonFungibleLocalId { value: scrypto::prelude::NonFungibleLocalId::Integer(IntegerNonFungibleLocalId::new(1)) } ,
                                    ] }],
                                }],
                            }],
                        }],
                    }],
                },
            ),
        ],
    };

    let expected = ManifestAstValue::Map {
        key_value_kind: ManifestAstValueKind::String,
        value_value_kind: ManifestAstValueKind::Tuple,
        entries: vec![
            (
                ManifestAstValue::String {
                    value: "HelloWorld".into(),
                },
                ManifestAstValue::Tuple {
                    elements: vec![
                        ManifestAstValue::Decimal {
                            value: "12".parse().unwrap(),
                        },
                        ManifestAstValue::PreciseDecimal {
                            value: "12".parse().unwrap(),
                        },
                    ],
                },
            ),
            (
                ManifestAstValue::String {
                    value: "WorldHello".into(),
                },
                ManifestAstValue::Tuple {
                    elements: vec![ManifestAstValue::Tuple {
                        elements: vec![ManifestAstValue::Tuple {
                            elements: vec![ManifestAstValue::Array {
                                element_kind: ManifestAstValueKind::Array,
                                elements: vec![ManifestAstValue::Array {
                                    element_kind: ManifestAstValueKind::Tuple,
                                    elements: vec![ManifestAstValue::NonFungibleGlobalId { address: NonFungibleGlobalId {
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
