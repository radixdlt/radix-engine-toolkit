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

use crate::prelude::*;

#[derive(Clone, Debug, Enum)]
pub enum ManifestResourceConstraint {
    NonZeroAmount,
    ExactAmount { value: Arc<Decimal> },
    AtLeastAmount { value: Arc<Decimal> },
    ExactNonFungibles { value: Vec<NonFungibleLocalId> },
    AtLeastNonFungibles { value: Vec<NonFungibleLocalId> },
    General { value: GeneralResourceConstraint },
}

impl From<NativeManifestResourceConstraint> for ManifestResourceConstraint {
    fn from(value: NativeManifestResourceConstraint) -> Self {
        match value {
            NativeManifestResourceConstraint::NonZeroAmount => {
                Self::NonZeroAmount
            }
            NativeManifestResourceConstraint::ExactAmount(decimal) => {
                Self::ExactAmount {
                    value: Arc::new(Decimal(decimal)),
                }
            }
            NativeManifestResourceConstraint::AtLeastAmount(decimal) => {
                Self::AtLeastAmount {
                    value: Arc::new(Decimal(decimal)),
                }
            }
            NativeManifestResourceConstraint::ExactNonFungibles(index_set) => {
                Self::ExactNonFungibles {
                    value: index_set
                        .into_iter()
                        .map(|item| item.into())
                        .collect(),
                }
            }
            NativeManifestResourceConstraint::AtLeastNonFungibles(
                index_set,
            ) => Self::AtLeastNonFungibles {
                value: index_set.into_iter().map(|item| item.into()).collect(),
            },
            NativeManifestResourceConstraint::General(
                general_resource_constraint,
            ) => Self::General {
                value: general_resource_constraint.into(),
            },
        }
    }
}

impl TryFrom<ManifestResourceConstraint> for NativeManifestResourceConstraint {
    type Error = RadixEngineToolkitError;

    fn try_from(
        value: ManifestResourceConstraint,
    ) -> std::result::Result<Self, Self::Error> {
        match value {
            ManifestResourceConstraint::NonZeroAmount => {
                Ok(Self::NonZeroAmount)
            }
            ManifestResourceConstraint::ExactAmount { value } => {
                Ok(Self::ExactAmount(value.0))
            }
            ManifestResourceConstraint::AtLeastAmount { value } => {
                Ok(Self::AtLeastAmount(value.0))
            }
            ManifestResourceConstraint::ExactNonFungibles { value } => value
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<_>>()
                .map(Self::ExactNonFungibles),
            ManifestResourceConstraint::AtLeastNonFungibles { value } => value
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<_>>()
                .map(Self::AtLeastNonFungibles),
            ManifestResourceConstraint::General { value } => {
                value.try_into().map(Self::General)
            }
        }
    }
}

#[derive(Clone, Debug, Record)]
pub struct GeneralResourceConstraint {
    required_ids: Vec<NonFungibleLocalId>,
    lower_bound: LowerBound,
    upper_bound: UpperBound,
    allowed_ids: AllowedIds,
}

impl From<NativeGeneralResourceConstraint> for GeneralResourceConstraint {
    fn from(
        NativeGeneralResourceConstraint {
            required_ids,
            lower_bound,
            upper_bound,
            allowed_ids,
        }: NativeGeneralResourceConstraint,
    ) -> Self {
        Self {
            required_ids: required_ids
                .into_iter()
                .map(|item| item.into())
                .collect(),
            lower_bound: lower_bound.into(),
            upper_bound: upper_bound.into(),
            allowed_ids: allowed_ids.into(),
        }
    }
}

impl TryFrom<GeneralResourceConstraint> for NativeGeneralResourceConstraint {
    type Error = RadixEngineToolkitError;

    fn try_from(
        GeneralResourceConstraint {
            required_ids,
            lower_bound,
            upper_bound,
            allowed_ids,
        }: GeneralResourceConstraint,
    ) -> Result<Self> {
        Ok(Self {
            required_ids: required_ids
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<_>>()?,
            lower_bound: lower_bound.into(),
            upper_bound: upper_bound.into(),
            allowed_ids: allowed_ids.try_into()?,
        })
    }
}
