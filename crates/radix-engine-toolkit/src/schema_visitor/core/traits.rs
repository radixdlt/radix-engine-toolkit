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

use crate::internal_prelude::*;

#[allow(unused_variables)]
pub trait SchemaVisitor<T>
where
    T: CustomSchema,
{
    type Error: Debug + Into<super::error::SchemaVisitorError>;

    #[inline]
    fn visit_any(
        &mut self,
        local_type_id: LocalTypeId,
        schema: &Schema<T>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    #[inline]
    fn visit_bool(
        &mut self,
        local_type_id: LocalTypeId,
        schema: &Schema<T>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    #[inline]
    fn visit_i8(
        &mut self,
        local_type_id: LocalTypeId,
        schema: &Schema<T>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    #[inline]
    fn visit_i16(
        &mut self,
        local_type_id: LocalTypeId,
        schema: &Schema<T>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    #[inline]
    fn visit_i32(
        &mut self,
        local_type_id: LocalTypeId,
        schema: &Schema<T>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    #[inline]
    fn visit_i64(
        &mut self,
        local_type_id: LocalTypeId,
        schema: &Schema<T>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    #[inline]
    fn visit_i128(
        &mut self,
        local_type_id: LocalTypeId,
        schema: &Schema<T>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    #[inline]
    fn visit_u8(
        &mut self,
        local_type_id: LocalTypeId,
        schema: &Schema<T>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    #[inline]
    fn visit_u16(
        &mut self,
        local_type_id: LocalTypeId,
        schema: &Schema<T>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    #[inline]
    fn visit_u32(
        &mut self,
        local_type_id: LocalTypeId,
        schema: &Schema<T>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    #[inline]
    fn visit_u64(
        &mut self,
        local_type_id: LocalTypeId,
        schema: &Schema<T>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    #[inline]
    fn visit_u128(
        &mut self,
        local_type_id: LocalTypeId,
        schema: &Schema<T>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    #[inline]
    fn visit_string(
        &mut self,
        local_type_id: LocalTypeId,
        schema: &Schema<T>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    #[inline]
    fn visit_array(
        &mut self,
        local_type_id: LocalTypeId,
        schema: &Schema<T>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    #[inline]
    fn visit_tuple(
        &mut self,
        local_type_id: LocalTypeId,
        schema: &Schema<T>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    #[inline]
    fn visit_enum(
        &mut self,
        local_type_id: LocalTypeId,
        schema: &Schema<T>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    #[inline]
    fn visit_map(
        &mut self,
        local_type_id: LocalTypeId,
        schema: &Schema<T>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    #[inline]
    fn visit_custom(
        &mut self,
        local_type_id: LocalTypeId,
        schema: &Schema<T>,
        custom_type_kind: &T::CustomLocalTypeKind,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    #[inline]
    fn is_enabled(&self) -> bool {
        true
    }
}
