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

use crate::model::value::ast::{ManifestAstValue, ManifestAstValueKind};

macro_rules! define_value_visitor {
    (
        $(#[$meta:meta])*
        $vis: vis trait $trait_ident: ident {
            $($method_ident: ident),*
        }
    ) => {
        $(#[$meta])*
        $vis trait $trait_ident {
            $(
                fn $method_ident(
                    &mut self,
                    _value: &mut $crate::model::value::ast::ManifestAstValue
                ) -> Result<(), $crate::error::VisitorError> {
                    Ok(())
                }
            )*
        }
    };
}

macro_rules! visit {
    ($visitors: expr, $method: ident, $value: expr) => {
        $visitors
            .iter_mut()
            .map(|visitor| visitor.$method($value))
            .collect::<Result<Vec<_>, _>>()
    };
}

define_value_visitor! {
    /// A trait which defines a [`crate::model::value::ast::ManifestAstValue`] visitor which operates on unstructured values, this
    /// choice is made to allow the visitor to work with aliasing an dealiasing operations which
    /// involves the visitor changing the value variant.
    pub trait ManifestAstValueVisitor {
        visit_bool,

        visit_u8,
        visit_u16,
        visit_u32,
        visit_u64,
        visit_u128,

        visit_i8,
        visit_i16,
        visit_i32,
        visit_i64,
        visit_i128,

        visit_string,

        visit_enum,
        visit_some,
        visit_none,
        visit_ok,
        visit_err,

        visit_array,
        visit_map,
        visit_tuple,

        visit_decimal,
        visit_precise_decimal,

        visit_address,

        visit_bucket,
        visit_proof,

        visit_non_fungible_global_id,
        visit_non_fungible_local_id,

        visit_expression,
        visit_blob,
        visit_bytes
    }
}

pub fn traverse_value(
    value: &mut crate::model::value::ast::ManifestAstValue,
    visitors: &mut [&mut dyn ManifestAstValueVisitor],
) -> Result<(), crate::error::VisitorError> {
    // Visit the top level value parts
    match value.kind() {
        ManifestAstValueKind::Bool => visit!(visitors, visit_bool, value)?,

        ManifestAstValueKind::U8 => visit!(visitors, visit_u8, value)?,
        ManifestAstValueKind::U16 => visit!(visitors, visit_u16, value)?,
        ManifestAstValueKind::U32 => visit!(visitors, visit_u32, value)?,
        ManifestAstValueKind::U64 => visit!(visitors, visit_u64, value)?,
        ManifestAstValueKind::U128 => visit!(visitors, visit_u128, value)?,

        ManifestAstValueKind::I8 => visit!(visitors, visit_i8, value)?,
        ManifestAstValueKind::I16 => visit!(visitors, visit_i16, value)?,
        ManifestAstValueKind::I32 => visit!(visitors, visit_i32, value)?,
        ManifestAstValueKind::I64 => visit!(visitors, visit_i64, value)?,
        ManifestAstValueKind::I128 => visit!(visitors, visit_i128, value)?,

        ManifestAstValueKind::String => visit!(visitors, visit_string, value)?,

        ManifestAstValueKind::Enum => visit!(visitors, visit_enum, value)?,

        ManifestAstValueKind::Some => visit!(visitors, visit_some, value)?,
        ManifestAstValueKind::None => visit!(visitors, visit_none, value)?,
        ManifestAstValueKind::Ok => visit!(visitors, visit_ok, value)?,
        ManifestAstValueKind::Err => visit!(visitors, visit_err, value)?,

        ManifestAstValueKind::Map => visit!(visitors, visit_map, value)?,
        ManifestAstValueKind::Array => visit!(visitors, visit_array, value)?,
        ManifestAstValueKind::Tuple => visit!(visitors, visit_tuple, value)?,

        ManifestAstValueKind::Decimal => visit!(visitors, visit_decimal, value)?,
        ManifestAstValueKind::PreciseDecimal => visit!(visitors, visit_precise_decimal, value)?,

        ManifestAstValueKind::Address => visit!(visitors, visit_address, value)?,

        ManifestAstValueKind::Bucket => visit!(visitors, visit_bucket, value)?,
        ManifestAstValueKind::Proof => visit!(visitors, visit_proof, value)?,

        ManifestAstValueKind::NonFungibleLocalId => {
            visit!(visitors, visit_non_fungible_local_id, value)?
        }
        ManifestAstValueKind::NonFungibleGlobalId => {
            visit!(visitors, visit_non_fungible_global_id, value)?
        }

        ManifestAstValueKind::Expression => visit!(visitors, visit_expression, value)?,
        ManifestAstValueKind::Blob => visit!(visitors, visit_blob, value)?,
        ManifestAstValueKind::Bytes => visit!(visitors, visit_bytes, value)?,
    };

    // Attempt to continue traversal on the value children (contained nested values). For future
    // reference, any variant that has a `ManifestAstValue` inside of it should go here.
    match value {
        ManifestAstValue::Map {
            entries: values, ..
        } => {
            values
                .iter_mut()
                .flat_map(|(x, y)| [x, y])
                .map(|value| traverse_value(value, visitors))
                .collect::<Result<Vec<_>, _>>()?;
        }
        ManifestAstValue::Enum { fields: values, .. }
        | ManifestAstValue::Array {
            elements: values, ..
        }
        | ManifestAstValue::Tuple { fields: values, .. } => {
            values
                .iter_mut()
                .map(|value| traverse_value(value, visitors))
                .collect::<Result<Vec<_>, _>>()?;
        }
        ManifestAstValue::Some { value }
        | ManifestAstValue::Ok { value }
        | ManifestAstValue::Err { value } => {
            traverse_value(value, visitors)?;
        }
        ManifestAstValue::Bool { .. }
        | ManifestAstValue::U8 { .. }
        | ManifestAstValue::U16 { .. }
        | ManifestAstValue::U32 { .. }
        | ManifestAstValue::U64 { .. }
        | ManifestAstValue::U128 { .. }
        | ManifestAstValue::I8 { .. }
        | ManifestAstValue::I16 { .. }
        | ManifestAstValue::I32 { .. }
        | ManifestAstValue::I64 { .. }
        | ManifestAstValue::I128 { .. }
        | ManifestAstValue::String { .. }
        | ManifestAstValue::None { .. }
        | ManifestAstValue::Decimal { .. }
        | ManifestAstValue::PreciseDecimal { .. }
        | ManifestAstValue::Address { .. }
        | ManifestAstValue::Bucket { .. }
        | ManifestAstValue::Proof { .. }
        | ManifestAstValue::NonFungibleLocalId { .. }
        | ManifestAstValue::NonFungibleGlobalId { .. }
        | ManifestAstValue::Expression { .. }
        | ManifestAstValue::Blob { .. }
        | ManifestAstValue::Bytes { .. } => { /* No OP. Doesn't contain a ManifestAstValue */ }
    };

    Ok(())
}
