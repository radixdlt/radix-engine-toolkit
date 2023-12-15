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

use crate::prelude::*;

macro_rules! define_uniffi_decimal {
    ($type: ty) => {
        paste::paste!{
            define_uniffi_decimal!{[<$type>],$crate::prelude::[<Native $type>]}
        }
    };
    ($ident: ident, $native_type: ty) => {
        paste::paste! {
            #[derive(Clone, Debug, $crate::prelude::Object, Default)]
            pub struct $ident(pub(crate) $native_type);

            #[uniffi::export]
            impl $ident {
                #[uniffi::constructor]
                pub fn new(value: String) -> $crate::prelude::Result<$crate::prelude::Arc<Self>> {
                    value.parse().map(|value| $crate::prelude::Arc::new(Self(value))).map_err(Into::into)
                }

                #[uniffi::constructor]
                pub fn max() -> $crate::prelude::Arc<Self> {
                    $crate::prelude::Arc::new(Self($native_type::MAX))
                }

                #[uniffi::constructor]
                pub fn min() -> $crate::prelude::Arc<Self> {
                    $crate::prelude::Arc::new(Self($native_type::MIN))
                }

                #[uniffi::constructor]
                pub fn zero() -> $crate::prelude::Arc<Self> {
                    $crate::prelude::Arc::new(Self($native_type::zero()))
                }

                #[uniffi::constructor]
                pub fn one() -> $crate::prelude::Arc<Self> {
                    $crate::prelude::Arc::new(Self($native_type::one()))
                }

                pub fn add(&self, other: $crate::prelude::Arc<Self>) -> $crate::prelude::Result<$crate::prelude::Arc<Self>> {
                    use $crate::prelude::NativeCheckedAdd;
                    self.0.checked_add(other.0).ok_or($crate::prelude::RadixEngineToolkitError::DecimalError).map(Self).map($crate::prelude::Arc::new)
                }

                pub fn sub(&self, other: $crate::prelude::Arc<Self>) -> $crate::prelude::Result<$crate::prelude::Arc<Self>> {
                    use $crate::prelude::NativeCheckedSub;
                    self.0.checked_sub(other.0).ok_or($crate::prelude::RadixEngineToolkitError::DecimalError).map(Self).map($crate::prelude::Arc::new)
                }

                pub fn mul(&self, other: $crate::prelude::Arc<Self>) -> $crate::prelude::Result<$crate::prelude::Arc<Self>> {
                    use $crate::prelude::NativeCheckedMul;
                    self.0.checked_mul(other.0).ok_or($crate::prelude::RadixEngineToolkitError::DecimalError).map(Self).map($crate::prelude::Arc::new)
                }

                pub fn div(&self, other: $crate::prelude::Arc<Self>) -> $crate::prelude::Result<$crate::prelude::Arc<Self>> {
                    use $crate::prelude::NativeCheckedDiv;
                    self.0.checked_div(other.0).ok_or($crate::prelude::RadixEngineToolkitError::DecimalError).map(Self).map($crate::prelude::Arc::new)
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

                pub fn abs(&self) -> $crate::error::Result<$crate::prelude::Arc<Self>> {
                    self.0
                        .checked_abs()
                        .ok_or($crate::prelude::RadixEngineToolkitError::DecimalError)
                        .map(Self)
                        .map($crate::prelude::Arc::new)
                }

                pub fn floor(&self) -> $crate::error::Result<$crate::prelude::Arc<Self>> {
                    self.0
                        .checked_floor()
                        .ok_or($crate::prelude::RadixEngineToolkitError::DecimalError)
                        .map(Self)
                        .map($crate::prelude::Arc::new)
                }

                pub fn ceiling(&self) -> $crate::error::Result<$crate::prelude::Arc<Self>> {
                    self.0
                        .checked_ceiling()
                        .ok_or($crate::prelude::RadixEngineToolkitError::DecimalError)
                        .map(Self)
                        .map($crate::prelude::Arc::new)
                }

                pub fn round(&self, decimal_places: i32, rounding_mode: $crate::prelude::RoundingMode) -> $crate::error::Result<$crate::prelude::Arc<Self>> {
                    self.0
                        .checked_round(decimal_places, rounding_mode.into())
                        .ok_or($crate::prelude::RadixEngineToolkitError::DecimalError)
                        .map(Self)
                        .map($crate::prelude::Arc::new)
                }

                pub fn powi(&self, exp: i64) -> $crate::error::Result<$crate::prelude::Arc<Self>> {
                    self.0
                        .checked_powi(exp)
                        .map(Self)
                        .map(crate::prelude::Arc::new)
                        .ok_or($crate::prelude::RadixEngineToolkitError::DecimalError)
                }

                pub fn sqrt(&self) -> Option<$crate::prelude::Arc<Self>> {
                    self.0.checked_sqrt().map(|value| $crate::prelude::Arc::new(Self(value)))
                }

                pub fn cbrt(&self) -> $crate::error::Result<$crate::prelude::Arc<Self>> {
                    self.0
                        .checked_cbrt()
                        .map(Self)
                        .map(crate::prelude::Arc::new)
                        .ok_or($crate::prelude::RadixEngineToolkitError::DecimalError)
                }

                pub fn nth_root(&self, n: u32) -> Option<$crate::prelude::Arc<Self>> {
                    self.0.checked_nth_root(n).map(|value| $crate::prelude::Arc::new(Self(value)))
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

                pub fn mantissa(&self) -> String {
                    self.0.0.to_string()
                }
            }
        }
    }
}
define_uniffi_decimal!(Decimal);
define_uniffi_decimal!(PreciseDecimal);

#[derive(Clone, Debug, Enum)]
pub enum WithdrawStrategy {
    Exact,
    Rounded { rounding_mode: RoundingMode },
}

impl From<NativeWithdrawStrategy> for WithdrawStrategy {
    fn from(value: NativeWithdrawStrategy) -> Self {
        match value {
            NativeWithdrawStrategy::Exact => Self::Exact,
            NativeWithdrawStrategy::Rounded(mode) => Self::Rounded {
                rounding_mode: mode.into(),
            },
        }
    }
}

impl From<WithdrawStrategy> for NativeWithdrawStrategy {
    fn from(value: WithdrawStrategy) -> Self {
        match value {
            WithdrawStrategy::Exact => Self::Exact,
            WithdrawStrategy::Rounded { rounding_mode } => {
                Self::Rounded(rounding_mode.into())
            }
        }
    }
}

#[derive(Clone, Debug, Enum)]
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
