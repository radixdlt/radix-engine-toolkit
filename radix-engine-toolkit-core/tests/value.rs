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

use radix_engine_toolkit_core::model::{Value, ValueKind};

#[test]
fn value_has_expected_json_representation() {
    // Arrange
    let test_vectors = vec![
        // ================
        // Primitive Types
        // ================

        // Unit and Boolean
        ValueJsonRepresentationTestVector::new(Value::Unit, r#"{"type": "Unit"}"#),
        ValueJsonRepresentationTestVector::new(
            Value::Bool { value: true },
            r#"{"type": "Bool", "value": true}"#,
        ),
        ValueJsonRepresentationTestVector::new(
            Value::Bool { value: false },
            r#"{"type": "Bool", "value": false}"#,
        ),
        // Unsigned Integers
        ValueJsonRepresentationTestVector::new(
            Value::U8 { value: 19 },
            r#"{"type": "U8", "value": "19"}"#,
        ),
        ValueJsonRepresentationTestVector::new(
            Value::U16 { value: 19 },
            r#"{"type": "U16", "value": "19"}"#,
        ),
        ValueJsonRepresentationTestVector::new(
            Value::U32 { value: 19 },
            r#"{"type": "U32", "value": "19"}"#,
        ),
        ValueJsonRepresentationTestVector::new(
            Value::U64 { value: 19 },
            r#"{"type": "U64", "value": "19"}"#,
        ),
        ValueJsonRepresentationTestVector::new(
            Value::U128 { value: 19 },
            r#"{"type": "U128", "value": "19"}"#,
        ),
        // Signed Integers
        ValueJsonRepresentationTestVector::new(
            Value::I8 { value: 19 },
            r#"{"type": "I8", "value": "19"}"#,
        ),
        ValueJsonRepresentationTestVector::new(
            Value::I16 { value: 19 },
            r#"{"type": "I16", "value": "19"}"#,
        ),
        ValueJsonRepresentationTestVector::new(
            Value::I32 { value: 19 },
            r#"{"type": "I32", "value": "19"}"#,
        ),
        ValueJsonRepresentationTestVector::new(
            Value::I64 { value: 19 },
            r#"{"type": "I64", "value": "19"}"#,
        ),
        ValueJsonRepresentationTestVector::new(
            Value::I128 { value: 19 },
            r#"{"type": "I128", "value": "19"}"#,
        ),
        // String
        ValueJsonRepresentationTestVector::new(
            Value::String {
                value: "P2P Cash System".into(),
            },
            r#"{"type": "String", "value": "P2P Cash System"}"#,
        ),
        // Enums and Enum Aliases (Option & Result)
        ValueJsonRepresentationTestVector::new(
            Value::Enum {
                variant: "Create".into(),
                fields: Some(vec![Value::String {
                    value: "Component".into(),
                }]),
            },
            r#"{"type": "Enum", "variant": "Create", "fields": [{"type": "String", "value": "Component"}]}"#,
        ),
        ValueJsonRepresentationTestVector::new(
            Value::Option {
                value: Box::new(Some(Value::String {
                    value: "Component".into(),
                })),
            },
            r#"{"type": "Option", "variant": "Some", "field": {"type": "String", "value": "Component"}}"#,
        ),
        ValueJsonRepresentationTestVector::new(
            Value::Option {
                value: Box::new(None),
            },
            r#"{"type": "Option", "variant": "None"}"#,
        ),
        ValueJsonRepresentationTestVector::new(
            Value::Result {
                value: Box::new(Ok(Value::String {
                    value: "Component".into(),
                })),
            },
            r#"{"type": "Result", "variant": "Ok", "field": {"type": "String", "value": "Component"}}"#,
        ),
        ValueJsonRepresentationTestVector::new(
            Value::Result {
                value: Box::new(Err(Value::String {
                    value: "Component".into(),
                })),
            },
            r#"{"type": "Result", "variant": "Err", "field": {"type": "String", "value": "Component"}}"#,
        ),
        // =================
        // Collection Types
        // =================
        ValueJsonRepresentationTestVector::new(
            Value::Array {
                element_type: ValueKind::String,
                elements: vec![Value::String {
                    value: "World, Hello!".into(),
                }],
            },
            r#"{"type": "Array", "element_type": "String", "elements": [{"type": "String", "value": "World, Hello!"}]}"#,
        ),
    ];

    // Checking that the serialization of values matches
    for test_vector in test_vectors.iter() {
        // Act
        let expected_serialized_value: serde_json::Value =
            serde_json::from_str(&test_vector.json_representation)
                .expect("Failed to deserialize trusted value");
        let serialized_value =
            serde_json::to_value(&test_vector.value).expect("Failed to serialize trusted value");

        // Assert
        assert_eq!(expected_serialized_value, serialized_value);
    }

    // Checking that the deserialization of values matches
    for test_vector in test_vectors.iter() {
        // Act
        let expected_value = &test_vector.value;
        let deserialized_value: Value = serde_json::from_str(&test_vector.json_representation)
            .expect("Deserialization failed!");

        // Assert
        assert_eq!(*expected_value, deserialized_value)
    }
}

struct ValueJsonRepresentationTestVector {
    value: Value,
    json_representation: String,
}

impl ValueJsonRepresentationTestVector {
    pub fn new<S: AsRef<str>>(value: Value, json_representation: S) -> Self {
        let json_representation: &str = json_representation.as_ref();
        let json_representation: String = json_representation.into();
        Self {
            value,
            json_representation,
        }
    }
}
