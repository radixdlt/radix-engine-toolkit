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
    ($type: ty, $to_inner: ident, $from_inner: ident) => {
        paste::paste! {
            define_uniffi_decimal!{
                [<$type>],
                $crate::prelude::engine::$type,
                $crate::prelude::engine::[<Inner $type>],
                $to_inner,
                $from_inner}
        }
    };
    ($ident: ident, $native_type: ty, $native_inner_type: ty, $to_inner: ident, $from_inner: ident) => {
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
                    use $crate::prelude::engine::CheckedAdd;
                    self.0.checked_add(other.0).ok_or($crate::prelude::RadixEngineToolkitError::DecimalError).map(Self).map($crate::prelude::Arc::new)
                }

                pub fn sub(&self, other: $crate::prelude::Arc<Self>) -> $crate::prelude::Result<$crate::prelude::Arc<Self>> {
                    use $crate::prelude::engine::CheckedSub;
                    self.0.checked_sub(other.0).ok_or($crate::prelude::RadixEngineToolkitError::DecimalError).map(Self).map($crate::prelude::Arc::new)
                }

                pub fn mul(&self, other: $crate::prelude::Arc<Self>) -> $crate::prelude::Result<$crate::prelude::Arc<Self>> {
                    use $crate::prelude::engine::CheckedMul;
                    self.0.checked_mul(other.0).ok_or($crate::prelude::RadixEngineToolkitError::DecimalError).map(Self).map($crate::prelude::Arc::new)
                }

                pub fn div(&self, other: $crate::prelude::Arc<Self>) -> $crate::prelude::Result<$crate::prelude::Arc<Self>> {
                    use $crate::prelude::engine::CheckedDiv;
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
                    self.0.$to_inner().to_string()
                }

                pub fn to_le_bytes(&self) -> Vec<u8> {
                    self.0.$to_inner().to_le_bytes().to_vec()
                }

                #[uniffi::constructor]
                pub fn from_le_bytes(value: &Vec<u8>) -> $crate::prelude::Arc<Self> {
                    $crate::prelude::Arc::new(Self($native_type::$from_inner($native_inner_type::from_le_bytes(
                        &value
                    ))))
                }

            }
        }
    };
}
define_uniffi_decimal!(Decimal, attos, from_attos);
define_uniffi_decimal!(PreciseDecimal, precise_subunits, from_precise_subunits);

#[derive(Clone, Debug, Enum)]
pub enum WithdrawStrategy {
    Exact,
    Rounded { rounding_mode: RoundingMode },
}

impl From<engine::WithdrawStrategy> for WithdrawStrategy {
    fn from(value: engine::WithdrawStrategy) -> Self {
        match value {
            engine::WithdrawStrategy::Exact => Self::Exact,
            engine::WithdrawStrategy::Rounded(mode) => Self::Rounded {
                rounding_mode: mode.into(),
            },
        }
    }
}

impl From<WithdrawStrategy> for engine::WithdrawStrategy {
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

impl From<RoundingMode> for crate::prelude::engine::RoundingMode {
    fn from(value: RoundingMode) -> Self {
        match value {
            RoundingMode::ToPositiveInfinity => {
                crate::prelude::engine::RoundingMode::ToPositiveInfinity
            }
            RoundingMode::ToNegativeInfinity => {
                crate::prelude::engine::RoundingMode::ToNegativeInfinity
            }
            RoundingMode::ToZero => crate::prelude::engine::RoundingMode::ToZero,
            RoundingMode::AwayFromZero => {
                crate::prelude::engine::RoundingMode::AwayFromZero
            }
            RoundingMode::ToNearestMidpointTowardZero => {
                crate::prelude::engine::RoundingMode::ToNearestMidpointTowardZero
            }
            RoundingMode::ToNearestMidpointAwayFromZero => {
                crate::prelude::engine::RoundingMode::ToNearestMidpointAwayFromZero
            }
            RoundingMode::ToNearestMidpointToEven => {
                crate::prelude::engine::RoundingMode::ToNearestMidpointToEven
            }
        }
    }
}

impl From<crate::prelude::engine::RoundingMode> for RoundingMode {
    fn from(value: crate::prelude::engine::RoundingMode) -> Self {
        match value {
            crate::prelude::engine::RoundingMode::ToPositiveInfinity => {
                RoundingMode::ToPositiveInfinity
            }
            crate::prelude::engine::RoundingMode::ToNegativeInfinity => {
                RoundingMode::ToNegativeInfinity
            }
            crate::prelude::engine::RoundingMode::ToZero => RoundingMode::ToZero,
            crate::prelude::engine::RoundingMode::AwayFromZero => {
                RoundingMode::AwayFromZero
            }
            crate::prelude::engine::RoundingMode::ToNearestMidpointTowardZero => {
                RoundingMode::ToNearestMidpointTowardZero
            }
            crate::prelude::engine::RoundingMode::ToNearestMidpointAwayFromZero => {
                RoundingMode::ToNearestMidpointAwayFromZero
            }
            crate::prelude::engine::RoundingMode::ToNearestMidpointToEven => {
                RoundingMode::ToNearestMidpointToEven
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decimal_to_le_bytes() {
        let d = Decimal::new(String::from("1234567890.123456789")).unwrap();
        assert_eq!(
            [
                0, 146, 124, 189, 145, 122, 121, 109, 235, 53, 253, 3, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0
            ],
            d.to_le_bytes().as_slice()
        );
    }

    #[test]
    fn decimal_from_le_bytes() {
        let d1 = Decimal::new(String::from("1234567890.123456789")).unwrap();
        let d2 = Decimal::from_le_bytes(
            &[
                0, 146, 124, 189, 145, 122, 121, 109, 235, 53, 253, 3, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
            ]
            .to_vec(),
        );
        assert!(d1.equal(d2));
    }
}
