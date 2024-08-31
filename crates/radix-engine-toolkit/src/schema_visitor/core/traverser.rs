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

use super::error::SchemaVisitorError;
use super::traits::SchemaVisitor;
use sbor::{CustomSchema, LocalTypeId, Schema, SchemaTypeKind};
use std::fmt::Debug;

pub fn traverse<T>(
    schema: &Schema<T>,
    local_type_id: LocalTypeId,
    visitors: &mut [&mut dyn SchemaVisitor<
        T,
        Error = impl Debug + Into<SchemaVisitorError>,
    >],
) -> Result<(), SchemaVisitorError>
where
    T: CustomSchema,
{
    let type_kind = schema
        .resolve_type_kind(local_type_id)
        .ok_or(SchemaVisitorError::InvalidLocalTypeId(local_type_id))?;

    match type_kind {
        SchemaTypeKind::<T>::Any => {
            for_each_enabled_visitor!(
                visitors,
                visit_any(local_type_id, schema)
            )
        }
        SchemaTypeKind::<T>::Bool => {
            for_each_enabled_visitor!(
                visitors,
                visit_bool(local_type_id, schema)
            )
        }
        SchemaTypeKind::<T>::I8 => {
            for_each_enabled_visitor!(visitors, visit_i8(local_type_id, schema))
        }
        SchemaTypeKind::<T>::I16 => {
            for_each_enabled_visitor!(
                visitors,
                visit_i16(local_type_id, schema)
            )
        }
        SchemaTypeKind::<T>::I32 => {
            for_each_enabled_visitor!(
                visitors,
                visit_i32(local_type_id, schema)
            )
        }
        SchemaTypeKind::<T>::I64 => {
            for_each_enabled_visitor!(
                visitors,
                visit_i64(local_type_id, schema)
            )
        }
        SchemaTypeKind::<T>::I128 => {
            for_each_enabled_visitor!(
                visitors,
                visit_i128(local_type_id, schema)
            )
        }
        SchemaTypeKind::<T>::U8 => {
            for_each_enabled_visitor!(visitors, visit_u8(local_type_id, schema))
        }
        SchemaTypeKind::<T>::U16 => {
            for_each_enabled_visitor!(
                visitors,
                visit_u16(local_type_id, schema)
            )
        }
        SchemaTypeKind::<T>::U32 => {
            for_each_enabled_visitor!(
                visitors,
                visit_u32(local_type_id, schema)
            )
        }
        SchemaTypeKind::<T>::U64 => {
            for_each_enabled_visitor!(
                visitors,
                visit_u64(local_type_id, schema)
            )
        }
        SchemaTypeKind::<T>::U128 => {
            for_each_enabled_visitor!(
                visitors,
                visit_u128(local_type_id, schema)
            )
        }
        SchemaTypeKind::<T>::String => {
            for_each_enabled_visitor!(
                visitors,
                visit_string(local_type_id, schema)
            )
        }
        SchemaTypeKind::<T>::Array { element_type } => {
            for_each_enabled_visitor!(
                visitors,
                visit_array(local_type_id, schema)
            );
            traverse(schema, *element_type, visitors)?;
        }
        SchemaTypeKind::<T>::Tuple { field_types } => {
            for_each_enabled_visitor!(
                visitors,
                visit_tuple(local_type_id, schema)
            );
            for local_type_id in field_types {
                traverse(schema, *local_type_id, visitors)?;
            }
        }
        SchemaTypeKind::<T>::Enum { variants } => {
            for_each_enabled_visitor!(
                visitors,
                visit_enum(local_type_id, schema)
            );
            for local_type_indices in variants.values() {
                for local_type_id in local_type_indices {
                    traverse(schema, *local_type_id, visitors)?;
                }
            }
        }
        SchemaTypeKind::<T>::Map {
            key_type,
            value_type,
        } => {
            for_each_enabled_visitor!(
                visitors,
                visit_map(local_type_id, schema)
            );
            traverse(schema, *key_type, visitors)?;
            traverse(schema, *value_type, visitors)?;
        }
        SchemaTypeKind::<T>::Custom(custom) => {
            for_each_enabled_visitor!(
                visitors,
                visit_custom(local_type_id, schema, custom)
            );
        }
    };

    Ok(())
}

macro_rules! for_each_enabled_visitor {
    ($visitors: expr, $method_ident: ident ( $($arg: ident),* $(,)? )) => {
        for visitor in $visitors.iter_mut() {
            if visitor.is_enabled() {
                visitor.$method_ident(
                    $($arg),*
                ).map_err(Into::into)?;
            }
        }
    };
}
use for_each_enabled_visitor;
