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

use heck::ToSnakeCase;
use indexmap::IndexMap;
use rocket_okapi::okapi::openapi3::OpenApi;
use serde_yaml::{Mapping, Value};

pub fn snake_case_type_name<T>() -> String {
    std::any::type_name::<T>()
        .split("::")
        .last()
        .unwrap()
        .to_owned()
        .to_snake_case()
}

pub fn convert_open_api_adts_to_discriminated_unions(spec: &OpenApi) -> Value {
    let mut spec = serde_yaml::to_value(spec).unwrap();

    let additions_map = {
        let mut map = if let Some(Value::Mapping(map)) = spec
            .get_mut("components")
            .and_then(|value| value.get_mut("schemas"))
        {
            map.iter_mut().collect::<IndexMap<_, _>>()
        } else {
            panic!("Failed to find the schemas")
        };

        // Handle the discriminator changes
        let mut additions_map = IndexMap::new();
        for item in map.values_mut() {
            let addition_map = traverse_and_convert_discriminator(item);
            additions_map.extend(addition_map)
        }
        additions_map
    };

    // Write the additions to the map
    if let Some(Value::Mapping(map)) = spec
        .get_mut("components")
        .and_then(|value| value.get_mut("schemas"))
    {
        map.extend(additions_map)
    } else {
        panic!("Failed to find the schemas")
    };

    spec
}

fn traverse_and_convert_discriminator(
    item: &mut Value,
) -> IndexMap<Value, Value> {
    if let Some(ref mut enum_variants) = get_enum(item) {
        // Remove the: ["required"][enum_discriminator] and the
        // ["properties"][enum_discriminator]
        for (value, enum_property) in enum_variants.values_mut() {
            // Required
            if let Some(Value::Sequence(required_sequence)) =
                value.get_mut("required")
            {
                if let Some(index) = required_sequence
                    .iter()
                    .enumerate()
                    .find_map(|(index, value)| {
                        if let Value::String(string) = value {
                            if *string == *enum_property {
                                Some(index)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    })
                {
                    required_sequence.remove(index);
                }
            }

            // Properties
            if let Some(Value::Mapping(map)) = value.get_mut("properties") {
                map.remove(Value::String(enum_property.to_string()));
            }

            // Remove required if empty
            let can_delete_required =
                value.get("required").map_or(false, |required| {
                    if let Value::Sequence(required) = required {
                        required.is_empty()
                    } else {
                        false
                    }
                });
            if can_delete_required {
                if let Value::Mapping(mapping) = value {
                    mapping.remove("required");
                }
            }
        }

        // Convert the enum variants to the required return
        let enum_discriminators = enum_variants
            .iter()
            .map(|(k, _)| k.to_owned())
            .collect::<Vec<_>>();
        let mut additions_map = enum_variants
            .iter()
            .map(|(name, (value, _))| {
                (Value::String(name.to_string()), (*value).clone())
            })
            .collect::<IndexMap<Value, Value>>();

        // Visit the children of the value type to see if any of them are
        // applicable
        let child_addition_maps = match item {
            Value::Sequence(sequence) => sequence
                .iter_mut()
                .map(traverse_and_convert_discriminator)
                .collect::<Vec<IndexMap<Value, Value>>>(),
            Value::Mapping(mapping) => mapping
                .iter_mut()
                .map(|(_, v)| v)
                .map(traverse_and_convert_discriminator)
                .collect::<Vec<IndexMap<Value, Value>>>(),
            Value::Tagged(tagged) => {
                vec![traverse_and_convert_discriminator(&mut tagged.value)]
            }
            Value::Null
            | Value::Bool(..)
            | Value::Number(..)
            | Value::String(..) => Default::default(),
        };
        for child_addition_map in child_addition_maps {
            additions_map.extend(child_addition_map);
        }

        // Change the item to be a one-of discriminator
        let item_replacement = Value::Mapping({
            let mut mapping = Mapping::new();

            // One of key
            mapping.insert(
                    Value::String("oneOf".to_string()),
                    Value::Sequence(
                        enum_discriminators
                            .iter()
                            .map(|enum_discriminator| {
                                Value::Mapping({
                                    let mut mapping = Mapping::new();

                                    mapping.insert(
                                        Value::String("$ref".to_string()),
                                        Value::String(format!(
                                            "#/components/schemas/{enum_discriminator}"
                                        )),
                                    );

                                    mapping
                                })
                            })
                            .collect(),
                    ),
                );

            // Discriminator key
            mapping.insert(
                Value::String("discriminator".to_string()),
                Value::Mapping({
                    let mut mapping = Mapping::new();

                    // TODO: Property might not be `kind`.
                    mapping.insert(
                        Value::String("propertyName".to_string()),
                        Value::String("kind".to_string()),
                    );
                    mapping.insert(
                        Value::String("mapping".to_string()),
                        Value::Mapping({
                            let mut mapping = Mapping::new();

                            for enum_discriminator in enum_discriminators {
                                let key = Value::String(
                                    enum_discriminator.to_string(),
                                );
                                let value = Value::String(format!(
                                    "#/components/schemas/{enum_discriminator}"
                                ));
                                mapping.insert(key, value);
                            }

                            mapping
                        }),
                    );

                    mapping
                }),
            );

            mapping
        });
        *item = item_replacement;

        additions_map
    } else {
        Default::default()
    }
}

fn get_enum(
    item: &mut Value,
) -> Option<IndexMap<String, (&mut Value, String)>> {
    if let Value::Mapping(ref mut map) = item {
        if let Some(Value::Sequence(values)) = map.get_mut("oneOf") {
            let mut variants_map = IndexMap::new();
            for value in values {
                // TODO: Do not rely on the `kind` key.
                if let Some(Value::Sequence(enum_discriminators)) = value
                    .get_mut("properties")
                    .and_then(|value| value.get_mut("kind"))
                    .and_then(|value| value.get_mut("enum"))
                {
                    if enum_discriminators.len() == 1 {
                        if let Some(Value::String(enum_discriminator)) =
                            enum_discriminators.first()
                        {
                            variants_map.insert(
                                enum_discriminator.to_string(),
                                (value, "kind".to_owned()),
                            );
                        } else {
                            return None;
                        }
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            }

            Some(variants_map)
        } else {
            None
        }
    } else {
        None
    }
}
