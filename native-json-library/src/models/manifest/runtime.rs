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

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use transaction::prelude::*;

use crate::prelude::*;

define_prefixed_id!(SerializableBucketId, ManifestBucket, "bucket");
define_prefixed_id!(SerializableProofId, ManifestProof, "proof");
define_prefixed_id!(SerializableNamedAddress, ManifestNamedAddress, "address");
define_prefixed_id!(
    SerializableAddressReservation,
    ManifestAddressReservation,
    "reservation"
);

pub struct ManifestNamedAddress(u32);

#[serde_with::serde_as]
#[derive(Serialize, Deserialize, JsonSchema, Clone, Copy, Debug)]
pub enum SerializableExpression {
    EntireWorktop,
    EntireAuthZone,
}

impl From<ManifestExpression> for SerializableExpression {
    fn from(value: ManifestExpression) -> Self {
        match value {
            ManifestExpression::EntireAuthZone => Self::EntireAuthZone,
            ManifestExpression::EntireWorktop => Self::EntireWorktop,
        }
    }
}

impl From<SerializableExpression> for ManifestExpression {
    fn from(value: SerializableExpression) -> Self {
        match value {
            SerializableExpression::EntireAuthZone => Self::EntireAuthZone,
            SerializableExpression::EntireWorktop => Self::EntireWorktop,
        }
    }
}

#[serde_with::serde_as]
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
#[serde(tag = "kind")]
pub enum SerializableManifestAddress {
    Static { value: SerializableNodeId },
    Named { value: SerializableNamedAddress },
}

macro_rules! define_prefixed_id {
    ($name: ident, $underlying_type: ty, $prefix: expr) => {
        paste::paste! {
            define_prefixed_id! {
                [< $name >],
                [< $name Prefixed >],
                $underlying_type,
                $prefix
            }
        }
    };
    ($type_name: ident, $prefixed_type_name: ident, $underlying_type: ty, $prefix: expr) => {
        #[serde_with::serde_as]
        #[derive(
            serde::Serialize, serde::Deserialize, schemars::JsonSchema, Clone, Debug, Copy,
        )]
        #[schemars(transparent)]
        #[serde(transparent)]
        pub struct $type_name(
            #[schemars(with = "String")]
            #[serde_as(as = "serde_with::DisplayFromStr")]
            $prefixed_type_name,
        );

        impl $type_name {
            pub fn new(id: u32) -> Self {
                Self($prefixed_type_name(id))
            }

            pub fn value(&self) -> u32 {
                self.0 .0
            }
        }

        impl From<$type_name> for u32 {
            fn from(value: $type_name) -> Self {
                value.0 .0
            }
        }

        impl From<u32> for $type_name {
            fn from(value: u32) -> Self {
                $type_name($prefixed_type_name(value))
            }
        }

        impl From<$underlying_type> for $type_name {
            fn from(value: $underlying_type) -> Self {
                value.0.into()
            }
        }

        impl From<$type_name> for $underlying_type {
            fn from(value: $type_name) -> Self {
                Self(value.0 .0)
            }
        }

        impl std::ops::Deref for $type_name {
            type Target = u32;

            fn deref(&self) -> &Self::Target {
                &self.0 .0
            }
        }

        impl std::ops::DerefMut for $type_name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0 .0
            }
        }

        #[derive(Clone, Debug, Copy)]
        struct $prefixed_type_name(u32);

        impl $prefixed_type_name {
            const PREFIX: &'static str = $prefix;
        }

        impl std::fmt::Display for $prefixed_type_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}{}", Self::PREFIX, self.0)
            }
        }

        impl std::str::FromStr for $prefixed_type_name {
            type Err = PrefixedIdParseError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                s.replace(Self::PREFIX, "")
                    .parse()
                    .map(Self)
                    .map_err(|error| PrefixedIdParseError::FailedToParseAsU32(format!("{error:?}")))
            }
        }
    };
}
use define_prefixed_id;

#[derive(Clone, Debug)]
pub enum PrefixedIdParseError {
    FailedToParseAsU32(String),
}

impl std::fmt::Display for PrefixedIdParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self, f)
    }
}
