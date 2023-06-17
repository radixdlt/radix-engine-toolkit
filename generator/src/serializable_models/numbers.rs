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

use native_json_library::prelude::*;
use radix_engine_common::math::*;

macro_rules! impl_example {
    ($type: ident) => {
        impl_example!($type, $type)
    };
    ($type: ident, $underlying_type: ident) => {
        impl<'d> super::traits::HasExamples<'d> for $type {
            fn examples() -> Vec<Self> {
                vec![
                    $underlying_type::MIN.into(),
                    (($underlying_type::MIN + $underlying_type::MAX) / 2).into(),
                    $underlying_type::MAX.into(),
                ]
            }
        }
    };
}
impl_example! {SerializableU8, u8}
impl_example! {SerializableU16, u16}
impl_example! {SerializableU32, u32}
impl_example! {SerializableU64, u64}
impl_example! {SerializableU128, u128}
impl_example! {SerializableI8, i8}
impl_example! {SerializableI16, i16}
impl_example! {SerializableI32, i32}
impl_example! {SerializableI64, i64}
impl_example! {SerializableI128, i128}
impl_example! {SerializableDecimal, Decimal}
impl_example! {SerializablePreciseDecimal, PreciseDecimal}
