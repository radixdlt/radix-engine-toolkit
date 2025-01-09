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

//! This module defines the instruction groups that are used by the Radix Engine
//! Toolkit for the purposes of transaction types classification.
//!
//! The [`define_instruction_groups`] macro is responsible for defining all of
//! the group types and the conversions between the types. Everything in this
//! file is compile-time checked. Meaning, if a new instructions is added in
//! the future to the [`radix_transactions`] crate, then we will get a compile
//! time error in this file as the various match statements will no longer be
//! exhaustive.
//!
//! A [`GroupedInstruction`] type is introduced by the macro expanded code which
//! can be converted from and into an [`AnyInstruction`]. This type captures
//! both the instruction itself as well as the group that it belongs to. In
//! addition to this type, various instructions groups are introduced such as
//! [`TakeFromWorktopInstructions`] and [`ProofInstructions`] which are enums
//! whose variants are the respective instructions.
//!
//! All of the types we introduced here can be converted from and into the
//! [`AnyInstruction`] type. Some of these conversions are failable such as the
//! conversion [`AnyInstruction`] -> [`TakeFromWorktopInstructions`] as the
//! instruction may not be a take from worktop instruction. This failable
//! conversion is represented through an implementation of [`TryFrom`] rather
//! than [`From`].

use crate::internal_prelude::*;

define_instruction_groups! {
    TakeFromWorktopInstructions => [
        TakeFromWorktop,
        TakeNonFungiblesFromWorktop,
        TakeAllFromWorktop,
    ],
    ReturnToWorktopInstructions => [
        ReturnToWorktop,
    ],
    AssertionInstructions => [
        AssertWorktopContainsAny,
        AssertWorktopContains,
        AssertWorktopContainsNonFungibles,
        AssertWorktopResourcesOnly,
        AssertWorktopResourcesInclude,
        AssertNextCallReturnsOnly,
        AssertNextCallReturnsInclude,
        AssertBucketContents,
    ],
    ProofInstructions => [
        CreateProofFromBucketOfAmount,
        CreateProofFromBucketOfNonFungibles,
        CreateProofFromBucketOfAll,
        CreateProofFromAuthZoneOfAmount,
        CreateProofFromAuthZoneOfNonFungibles,
        CreateProofFromAuthZoneOfAll,
        CloneProof,
        DropProof,
        DropAuthZoneProofs,
        DropAuthZoneRegularProofs,
        DropAuthZoneSignatureProofs,
        DropNamedProofs,
        DropAllProofs,
        PushToAuthZone,
        PopFromAuthZone
    ],
    InvocationInstructions => [
        CallFunction,
        CallMethod,
        CallRoyaltyMethod,
        CallMetadataMethod,
        CallRoleAssignmentMethod,
        CallDirectVaultMethod
    ],
    SubintentInstructions => [
        YieldToParent,
        YieldToChild,
        VerifyParent,
    ],
    AddressAllocationInstructions => [
        AllocateGlobalAddress
    ],
    BurnResourceInstructions => [
        BurnResource,
    ]
}

macro_rules! define_instruction_groups {
    (
        $(
            $group_ident: ident => [
                $($instruction_ident: ident),* $(,)?
            ]
        ),* $(,)?
    ) => {
        paste::paste! {
            #[derive(Debug, Clone, PartialEq, Eq, ManifestSbor, ScryptoDescribe)]
            pub enum GroupedInstruction {
                $(
                    $group_ident($group_ident)
                ),*
            }

            impl GroupedInstruction {
                pub fn effect(&self) -> ManifestInstructionEffect {
                    match self {
                        $(
                            Self::$group_ident(v) => v.effect(),
                        )*
                    }
                }

                $(
                    pub fn [< belongs_to_ $group_ident:snake >](&self) -> bool {
                        matches!(self, Self::$group_ident(..))
                    }

                    pub fn [< belongs_to_ $group_ident:snake _and >](
                        &self,
                        mut f: impl FnMut(&$group_ident) -> bool
                    ) -> bool {
                        if let Self::$group_ident(ref value) = self {
                            f(value)
                        } else {
                            false
                        }
                    }
                )*
            }

            $(
                impl From<$group_ident> for GroupedInstruction {
                    fn from(value: $group_ident) -> Self {
                        Self::$group_ident(value)
                    }
                }

                impl TryFrom<GroupedInstruction> for $group_ident {
                    type Error = ();

                    fn try_from(value: GroupedInstruction) -> Result<Self, Self::Error> {
                        if let GroupedInstruction::$group_ident(value) = value {
                            Ok(value)
                        } else {
                            Err(())
                        }
                    }
                }
            )*

            impl From<AnyInstruction> for GroupedInstruction {
                #[inline]
                fn from(value: AnyInstruction) -> Self {
                    match value {
                        $(
                            $(
                                AnyInstruction::$instruction_ident(value)
                                    => GroupedInstruction::$group_ident(
                                        $group_ident::$instruction_ident(value)
                                    ),
                            )*
                        )*
                    }
                }
            }

            impl From<GroupedInstruction> for AnyInstruction {
                #[inline]
                fn from(value: GroupedInstruction) -> Self {
                    match value {
                        $(
                            GroupedInstruction::$group_ident(value) => value.into()
                        ),*
                    }
                }
            }

            $(
                #[derive(Debug, Clone, PartialEq, Eq, ManifestSbor, ScryptoDescribe)]
                pub enum $group_ident {
                    $(
                        $instruction_ident($instruction_ident)
                    ),*
                }

                #[allow(irrefutable_let_patterns)]
                impl $group_ident {
                    pub fn effect(&self) -> ManifestInstructionEffect {
                        match self {
                            $(
                                Self::$instruction_ident(v) => v.effect(),
                            )*
                        }
                    }

                    $(
                        pub fn [< is_ $instruction_ident: snake _instruction >](&self) -> bool {
                            matches!(self, Self::$instruction_ident(..))
                        }

                        pub fn [< is_ $instruction_ident: snake _instruction_and >](
                            &self,
                            mut f: impl FnMut(&$instruction_ident) -> bool
                        ) -> bool {
                            if let Self::$instruction_ident(ref value) = self {
                                f(value)
                            } else {
                                false
                            }
                        }
                    )*
                }

                impl TryFrom<AnyInstruction> for $group_ident {
                    type Error = ();

                    #[inline]
                    fn try_from(value: AnyInstruction) -> Result<Self, Self::Error> {
                        match value {
                            $(
                                AnyInstruction::$instruction_ident(value) => Ok(
                                    Self::$instruction_ident(value)
                                ),
                            )*
                            _ => Err(())
                        }
                    }
                }

                impl From<$group_ident> for AnyInstruction {
                    #[inline]
                    fn from(value: $group_ident) -> Self {
                        match value {
                            $(
                                $group_ident::$instruction_ident(value)
                                    => Self::$instruction_ident(value),
                            )*
                        }
                    }
                }
            )*
        }
    };
}
use define_instruction_groups;
