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

use radix_engine_common::prelude::*;

use super::value::*;
use crate::common::address::*;

pub type VisitorResult = Result<(), ProgrammaticScryptoValueVisitorError>;

#[allow(unused_variables)]
pub trait ProgrammaticScryptoValueVisitor {
    #[inline]
    fn visit_bool(&mut self, value: &bool) {}

    #[inline]
    fn visit_i8(&mut self, value: &i8) {}

    #[inline]
    fn visit_i16(&mut self, value: &i16) {}

    #[inline]
    fn visit_i32(&mut self, value: &i32) {}

    #[inline]
    fn visit_i64(&mut self, value: &i64) {}

    #[inline]
    fn visit_i128(&mut self, value: &i128) {}

    #[inline]
    fn visit_u8(&mut self, value: &u8) {}

    #[inline]
    fn visit_u16(&mut self, value: &u16) {}

    #[inline]
    fn visit_u32(&mut self, value: &u32) {}

    #[inline]
    fn visit_u64(&mut self, value: &u64) {}

    #[inline]
    fn visit_u128(&mut self, value: &u128) {}

    #[inline]
    fn visit_string(&mut self, value: &str) {}

    #[inline]
    fn visit_enum(&mut self, discriminator: &u8, fields: &[ProgrammaticScryptoValue]) {}

    #[inline]
    fn visit_array(
        &mut self,
        element_value_kind: &ProgrammaticScryptoValueKind,
        elements: &[ProgrammaticScryptoValue],
    ) {
    }

    #[inline]
    fn visit_tuple(&mut self, fields: &[ProgrammaticScryptoValue]) {}

    #[inline]
    fn visit_map(
        &mut self,
        key_value_kind: &ProgrammaticScryptoValueKind,
        value_value_kind: &ProgrammaticScryptoValueKind,
        entries: &[(ProgrammaticScryptoValue, ProgrammaticScryptoValue)],
    ) {
    }

    #[inline]
    fn visit_reference(&mut self, value: &SerializableNodeId) {}

    #[inline]
    fn visit_own(&mut self, value: &SerializableNodeId) {}

    #[inline]
    fn visit_decimal(&mut self, value: &Decimal) {}

    #[inline]
    fn visit_precise_decimal(&mut self, value: &PreciseDecimal) {}

    #[inline]
    fn visit_non_fungible_local_id(&mut self, value: &NonFungibleLocalId) {}

    #[inline]
    fn visit_bytes(&mut self, element_value_kind: &ProgrammaticScryptoValueKind, value: &[u8]) {}
}

#[derive(Clone, Debug)]
pub enum ProgrammaticScryptoValueVisitorError {}

pub fn traverse(
    value: &ProgrammaticScryptoValue,
    visitors: &mut [&mut dyn ProgrammaticScryptoValueVisitor],
) {
    match value {
        ProgrammaticScryptoValue::Bool { value } => visitors
            .iter_mut()
            .for_each(|visitor| visitor.visit_bool(value)),
        ProgrammaticScryptoValue::I8 { value } => visitors
            .iter_mut()
            .for_each(|visitor| visitor.visit_i8(value)),
        ProgrammaticScryptoValue::I16 { value } => visitors
            .iter_mut()
            .for_each(|visitor| visitor.visit_i16(value)),
        ProgrammaticScryptoValue::I32 { value } => visitors
            .iter_mut()
            .for_each(|visitor| visitor.visit_i32(value)),
        ProgrammaticScryptoValue::I64 { value } => visitors
            .iter_mut()
            .for_each(|visitor| visitor.visit_i64(value)),
        ProgrammaticScryptoValue::I128 { value } => visitors
            .iter_mut()
            .for_each(|visitor| visitor.visit_i128(value)),
        ProgrammaticScryptoValue::U8 { value } => visitors
            .iter_mut()
            .for_each(|visitor| visitor.visit_u8(value)),
        ProgrammaticScryptoValue::U16 { value } => visitors
            .iter_mut()
            .for_each(|visitor| visitor.visit_u16(value)),
        ProgrammaticScryptoValue::U32 { value } => visitors
            .iter_mut()
            .for_each(|visitor| visitor.visit_u32(value)),
        ProgrammaticScryptoValue::U64 { value } => visitors
            .iter_mut()
            .for_each(|visitor| visitor.visit_u64(value)),
        ProgrammaticScryptoValue::U128 { value } => visitors
            .iter_mut()
            .for_each(|visitor| visitor.visit_u128(value)),
        ProgrammaticScryptoValue::String { value } => visitors
            .iter_mut()
            .for_each(|visitor| visitor.visit_string(value)),
        ProgrammaticScryptoValue::Enum {
            discriminator,
            fields,
        } => {
            visitors
                .iter_mut()
                .for_each(|visitor| visitor.visit_enum(discriminator, fields));
            fields.iter().for_each(|field| traverse(field, visitors));
        }
        ProgrammaticScryptoValue::Array {
            element_value_kind,
            elements,
        } => {
            visitors
                .iter_mut()
                .for_each(|visitor| visitor.visit_array(element_value_kind, elements));
            elements
                .iter()
                .for_each(|element| traverse(element, visitors));
        }
        ProgrammaticScryptoValue::Tuple { fields } => {
            visitors.iter_mut().for_each(
                |visitor: &mut &mut dyn ProgrammaticScryptoValueVisitor| {
                    visitor.visit_tuple(fields)
                },
            );
            fields.iter().for_each(|field| traverse(field, visitors));
        }
        ProgrammaticScryptoValue::Map {
            key_value_kind,
            value_value_kind,
            entries,
        } => {
            visitors
                .iter_mut()
                .for_each(|visitor| visitor.visit_map(key_value_kind, value_value_kind, entries));
            entries.iter().for_each(|(key, value)| {
                traverse(key, visitors);
                traverse(value, visitors);
            });
        }
        ProgrammaticScryptoValue::Reference { value } => visitors
            .iter_mut()
            .for_each(|visitor| visitor.visit_reference(value)),
        ProgrammaticScryptoValue::Own { value } => visitors
            .iter_mut()
            .for_each(|visitor| visitor.visit_own(value)),
        ProgrammaticScryptoValue::Decimal { value } => visitors
            .iter_mut()
            .for_each(|visitor| visitor.visit_decimal(value)),
        ProgrammaticScryptoValue::PreciseDecimal { value } => visitors
            .iter_mut()
            .for_each(|visitor| visitor.visit_precise_decimal(value)),
        ProgrammaticScryptoValue::NonFungibleLocalId { value } => visitors
            .iter_mut()
            .for_each(|visitor| visitor.visit_non_fungible_local_id(value)),
        ProgrammaticScryptoValue::Bytes {
            element_value_kind,
            value,
        } => visitors
            .iter_mut()
            .for_each(|visitor| visitor.visit_bytes(element_value_kind, value)),
    };
}

#[derive(Debug, Default, Clone)]
pub struct AddressNetworkMismatchVisitor(HashSet<u8>);

impl AddressNetworkMismatchVisitor {
    pub fn is_network_mismatch(&self) -> bool {
        self.0.len() > 1
    }
}

impl ProgrammaticScryptoValueVisitor for AddressNetworkMismatchVisitor {
    fn visit_reference(&mut self, value: &SerializableNodeId) {
        self.0.insert(value.1);
    }

    fn visit_own(&mut self, value: &SerializableNodeId) {
        self.0.insert(value.1);
    }
}
