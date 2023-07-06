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

#![allow(clippy::should_implement_trait)]

macro_rules! define_uniffi_decimal {
    ($type: ty) => {
        paste::paste!{
            define_uniffi_decimal!{[<$type>], $crate::prelude::[<Native $type>]}
        }
    };
    ($ident: ident, $native_type: ty) => {
        paste::paste! {
            #[derive(Clone, Debug, crate::prelude::Object, Default)]
            pub struct $ident(pub(crate) $native_type);

            #[uniffi::export]
            impl $ident {
                #[uniffi::constructor]
                pub fn new(value: String) -> $crate::prelude::Result<$crate::prelude::Arc<Self>> {
                    value.parse().map(|value| $crate::prelude::Arc::new(Self(value))).map_err(Into::into)
                }

                #[uniffi::constructor]
                pub fn max() -> crate::prelude::Arc<Self> {
                    $crate::prelude::Arc::new(Self($native_type::MAX))
                }

                #[uniffi::constructor]
                pub fn min() -> crate::prelude::Arc<Self> {
                    $crate::prelude::Arc::new(Self($native_type::MIN))
                }

                #[uniffi::constructor]
                pub fn zero() -> crate::prelude::Arc<Self> {
                    $crate::prelude::Arc::new(Self($native_type::zero()))
                }

                #[uniffi::constructor]
                pub fn one() -> crate::prelude::Arc<Self> {
                    $crate::prelude::Arc::new(Self($native_type::one()))
                }

                pub fn add(&self, other: $crate::prelude::Arc<Self>) -> $crate::prelude::Arc<Self> {
                    $crate::prelude::Arc::new(Self(self.0 + other.0))
                }

                pub fn sub(&self, other: $crate::prelude::Arc<Self>) -> $crate::prelude::Arc<Self> {
                    $crate::prelude::Arc::new(Self(self.0 - other.0))
                }

                pub fn mul(&self, other: $crate::prelude::Arc<Self>) -> $crate::prelude::Arc<Self> {
                    $crate::prelude::Arc::new(Self(self.0 * other.0))
                }

                pub fn div(&self, other: $crate::prelude::Arc<Self>) -> $crate::prelude::Arc<Self> {
                    $crate::prelude::Arc::new(Self(self.0 / other.0))
                }

                pub fn as_str(&self) -> String {
                    self.0.to_string()
                }

                pub fn is_zero(&self) -> bool {
                    self.0.is_zero()
                }

                pub fn is_positive(&self) -> bool {
                    self.0.is_positive()
                }

                pub fn is_negative(&self) -> bool {
                    self.0.is_negative()
                }

                pub fn abs(&self) -> $crate::prelude::Arc<Self> {
                    $crate::prelude::Arc::new(Self(self.0.abs()))
                }

                pub fn floor(&self) -> $crate::prelude::Arc<Self> {
                    $crate::prelude::Arc::new(Self(self.0.floor()))
                }

                pub fn ceiling(&self) -> $crate::prelude::Arc<Self> {
                    $crate::prelude::Arc::new(Self(self.0.ceiling()))
                }

                pub fn round(&self, decimal_places: i32, rounding_mode: crate::prelude::RoundingMode) -> $crate::prelude::Arc<Self> {
                    $crate::prelude::Arc::new(Self(self.0.round(decimal_places, rounding_mode.into())))
                }

                pub fn powi(&self, exp: i64) -> $crate::prelude::Arc<Self> {
                    $crate::prelude::Arc::new(Self(self.0.powi(exp)))
                }

                pub fn sqrt(&self) -> Option<$crate::prelude::Arc<Self>> {
                    self.0.sqrt().map(|value| $crate::prelude::Arc::new(Self(value)))
                }

                pub fn cbrt(&self) -> $crate::prelude::Arc<Self> {
                    $crate::prelude::Arc::new(Self(self.0.cbrt()))
                }

                pub fn nth_root(&self, n: u32) -> Option<$crate::prelude::Arc<Self>> {
                    self.0.nth_root(n).map(|value| $crate::prelude::Arc::new(Self(value)))
                }

                pub fn equal(&self, other: $crate::prelude::Arc<Self>) -> bool {
                    self.0.eq(&other.0)
                }

                pub fn not_equal(&self, other: $crate::prelude::Arc<Self>) -> bool {
                    self.0.ne(&other.0)
                }

                pub fn greater_than(&self, other: $crate::prelude::Arc<Self>) -> bool {
                    self.0.gt(&other.0)
                }

                pub fn greater_than_or_equal(&self, other: $crate::prelude::Arc<Self>) -> bool {
                    self.0.ge(&other.0)
                }

                pub fn less_than(&self, other: $crate::prelude::Arc<Self>) -> bool {
                    self.0.lt(&other.0)
                }

                pub fn less_than_or_equal(&self, other: $crate::prelude::Arc<Self>) -> bool {
                    self.0.le(&other.0)
                }
            }
        }
    }
}
define_uniffi_decimal!(Decimal);
define_uniffi_decimal!(PreciseDecimal);

#[derive(Clone, Debug, crate::prelude::Enum)]
pub enum RoundingMode {
    ToPositiveInfinity,
    ToNegativeInfinity,
    ToZero,
    AwayFromZero,
    ToNearestMidpointTowardZero,
    ToNearestMidpointAwayFromZero,
    ToNearestMidpointToEven,
}

impl From<RoundingMode> for crate::prelude::NativeRoundingMode {
    fn from(value: RoundingMode) -> Self {
        match value {
            RoundingMode::ToPositiveInfinity => {
                crate::prelude::NativeRoundingMode::ToPositiveInfinity
            }
            RoundingMode::ToNegativeInfinity => {
                crate::prelude::NativeRoundingMode::ToNegativeInfinity
            }
            RoundingMode::ToZero => crate::prelude::NativeRoundingMode::ToZero,
            RoundingMode::AwayFromZero => crate::prelude::NativeRoundingMode::AwayFromZero,
            RoundingMode::ToNearestMidpointTowardZero => {
                crate::prelude::NativeRoundingMode::ToNearestMidpointTowardZero
            }
            RoundingMode::ToNearestMidpointAwayFromZero => {
                crate::prelude::NativeRoundingMode::ToNearestMidpointAwayFromZero
            }
            RoundingMode::ToNearestMidpointToEven => {
                crate::prelude::NativeRoundingMode::ToNearestMidpointToEven
            }
        }
    }
}

impl From<crate::prelude::NativeRoundingMode> for RoundingMode {
    fn from(value: crate::prelude::NativeRoundingMode) -> Self {
        match value {
            crate::prelude::NativeRoundingMode::ToPositiveInfinity => {
                RoundingMode::ToPositiveInfinity
            }
            crate::prelude::NativeRoundingMode::ToNegativeInfinity => {
                RoundingMode::ToNegativeInfinity
            }
            crate::prelude::NativeRoundingMode::ToZero => RoundingMode::ToZero,
            crate::prelude::NativeRoundingMode::AwayFromZero => RoundingMode::AwayFromZero,
            crate::prelude::NativeRoundingMode::ToNearestMidpointTowardZero => {
                RoundingMode::ToNearestMidpointTowardZero
            }
            crate::prelude::NativeRoundingMode::ToNearestMidpointAwayFromZero => {
                RoundingMode::ToNearestMidpointAwayFromZero
            }
            crate::prelude::NativeRoundingMode::ToNearestMidpointToEven => {
                RoundingMode::ToNearestMidpointToEven
            }
        }
    }
}
