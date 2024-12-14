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
pub enum InstructionV2 {
    TakeAllFromWorktop {
        resource_address: Arc<Address>,
    },

    TakeFromWorktop {
        resource_address: Arc<Address>,
        amount: Arc<Decimal>,
    },

    TakeNonFungiblesFromWorktop {
        resource_address: Arc<Address>,
        ids: Vec<NonFungibleLocalId>,
    },

    ReturnToWorktop {
        bucket_id: ManifestBucket,
    },

    AssertWorktopContains {
        resource_address: Arc<Address>,
        amount: Arc<Decimal>,
    },

    AssertWorktopContainsAny {
        resource_address: Arc<Address>,
    },

    AssertWorktopContainsNonFungibles {
        resource_address: Arc<Address>,
        ids: Vec<NonFungibleLocalId>,
    },

    PopFromAuthZone,

    PushToAuthZone {
        proof_id: ManifestProof,
    },

    CreateProofFromAuthZoneOfAmount {
        resource_address: Arc<Address>,
        amount: Arc<Decimal>,
    },

    CreateProofFromAuthZoneOfNonFungibles {
        resource_address: Arc<Address>,
        ids: Vec<NonFungibleLocalId>,
    },

    CreateProofFromAuthZoneOfAll {
        resource_address: Arc<Address>,
    },

    DropAllProofs,
    DropNamedProofs,
    DropAuthZoneProofs,
    DropAuthZoneRegularProofs,
    DropAuthZoneSignatureProofs,

    CreateProofFromBucketOfAmount {
        bucket_id: ManifestBucket,
        amount: Arc<Decimal>,
    },

    CreateProofFromBucketOfNonFungibles {
        bucket_id: ManifestBucket,
        ids: Vec<NonFungibleLocalId>,
    },

    CreateProofFromBucketOfAll {
        bucket_id: ManifestBucket,
    },

    BurnResource {
        bucket_id: ManifestBucket,
    },

    CloneProof {
        proof_id: ManifestProof,
    },

    DropProof {
        proof_id: ManifestProof,
    },

    CallFunction {
        package_address: ManifestAddress,
        blueprint_name: String,
        function_name: String,
        args: ManifestValue,
    },

    CallMethod {
        address: ManifestAddress,
        method_name: String,
        args: ManifestValue,
    },

    CallRoyaltyMethod {
        address: ManifestAddress,
        method_name: String,
        args: ManifestValue,
    },

    CallMetadataMethod {
        address: ManifestAddress,
        method_name: String,
        args: ManifestValue,
    },

    CallRoleAssignmentMethod {
        address: ManifestAddress,
        method_name: String,
        args: ManifestValue,
    },

    CallDirectVaultMethod {
        address: Arc<Address>,
        method_name: String,
        args: ManifestValue,
    },

    AllocateGlobalAddress {
        package_address: Arc<Address>,
        blueprint_name: String,
    },

    YieldToParent {
        args: ManifestValue,
    },

    YieldToChild {
        child_index: u32,
        args: ManifestValue,
    },

    VerifyParent {
        access_rule: Arc<AccessRule>,
    },

    AssertWorktopResourcesOnly {
        constraints: HashMap<String, ManifestResourceConstraint>,
    },
    AssertWorktopResourcesInclude {
        constraints: HashMap<String, ManifestResourceConstraint>,
    },
    AssertNextCallReturnsOnly {
        constraints: HashMap<String, ManifestResourceConstraint>,
    },
    AssertNextCallReturnsInclude {
        constraints: HashMap<String, ManifestResourceConstraint>,
    },
    AssertBucketContents {
        bucket_id: ManifestBucket,
        constraint: ManifestResourceConstraint,
    },
}

impl InstructionV2 {
    pub fn from_native(native: &NativeInstructionV2, network_id: u8) -> Self {
        match native {
            NativeInstructionV2::TakeAllFromWorktop(
                NativeTakeAllFromWorktop { resource_address },
            ) => Self::TakeAllFromWorktop {
                resource_address: Arc::new(Address::from_typed_node_id(
                    *resource_address,
                    network_id,
                )),
            },
            NativeInstructionV2::TakeFromWorktop(NativeTakeFromWorktop {
                resource_address,
                amount,
            }) => Self::TakeFromWorktop {
                resource_address: Arc::new(Address::from_typed_node_id(
                    *resource_address,
                    network_id,
                )),
                amount: Arc::new(Decimal(*amount)),
            },
            NativeInstructionV2::TakeNonFungiblesFromWorktop(
                NativeTakeNonFungiblesFromWorktop {
                    resource_address,
                    ids,
                },
            ) => Self::TakeNonFungiblesFromWorktop {
                resource_address: Arc::new(Address::from_typed_node_id(
                    *resource_address,
                    network_id,
                )),
                ids: ids.iter().cloned().map(Into::into).collect(),
            },

            NativeInstructionV2::ReturnToWorktop(NativeReturnToWorktop {
                bucket_id,
            }) => Self::ReturnToWorktop {
                bucket_id: (*bucket_id).into(),
            },

            NativeInstructionV2::AssertWorktopContains(
                NativeAssertWorktopContains {
                    resource_address,
                    amount,
                },
            ) => Self::AssertWorktopContains {
                resource_address: Arc::new(Address::from_typed_node_id(
                    *resource_address,
                    network_id,
                )),
                amount: Arc::new(Decimal(*amount)),
            },
            NativeInstructionV2::AssertWorktopContainsAny(
                NativeAssertWorktopContainsAny { resource_address },
            ) => Self::AssertWorktopContainsAny {
                resource_address: Arc::new(Address::from_typed_node_id(
                    *resource_address,
                    network_id,
                )),
            },
            NativeInstructionV2::AssertWorktopContainsNonFungibles(
                NativeAssertWorktopContainsNonFungibles {
                    resource_address,
                    ids,
                },
            ) => Self::AssertWorktopContainsNonFungibles {
                resource_address: Arc::new(Address::from_typed_node_id(
                    *resource_address,
                    network_id,
                )),
                ids: ids.iter().cloned().map(Into::into).collect(),
            },
            NativeInstructionV2::PopFromAuthZone { .. } => {
                Self::PopFromAuthZone
            }
            NativeInstructionV2::PushToAuthZone(NativePushToAuthZone {
                proof_id,
            }) => Self::PushToAuthZone {
                proof_id: (*proof_id).into(),
            },
            NativeInstructionV2::DropNamedProofs { .. } => {
                Self::DropNamedProofs
            }
            NativeInstructionV2::DropAuthZoneProofs { .. } => {
                Self::DropAuthZoneProofs
            }
            NativeInstructionV2::DropAuthZoneRegularProofs { .. } => {
                Self::DropAuthZoneRegularProofs
            }
            NativeInstructionV2::DropAuthZoneSignatureProofs { .. } => {
                Self::DropAuthZoneSignatureProofs
            }
            NativeInstructionV2::CreateProofFromAuthZoneOfAll(
                NativeCreateProofFromAuthZoneOfAll { resource_address },
            ) => Self::CreateProofFromAuthZoneOfAll {
                resource_address: Arc::new(Address::from_typed_node_id(
                    *resource_address,
                    network_id,
                )),
            },
            NativeInstructionV2::CreateProofFromAuthZoneOfAmount(
                NativeCreateProofFromAuthZoneOfAmount {
                    resource_address,
                    amount,
                },
            ) => Self::CreateProofFromAuthZoneOfAmount {
                resource_address: Arc::new(Address::from_typed_node_id(
                    *resource_address,
                    network_id,
                )),
                amount: Arc::new(Decimal(*amount)),
            },
            NativeInstructionV2::CreateProofFromAuthZoneOfNonFungibles(
                NativeCreateProofFromAuthZoneOfNonFungibles {
                    resource_address,
                    ids,
                },
            ) => Self::CreateProofFromAuthZoneOfNonFungibles {
                resource_address: Arc::new(Address::from_typed_node_id(
                    *resource_address,
                    network_id,
                )),
                ids: ids.iter().cloned().map(Into::into).collect(),
            },
            NativeInstructionV2::CreateProofFromBucketOfAll(
                NativeCreateProofFromBucketOfAll { bucket_id },
            ) => Self::CreateProofFromBucketOfAll {
                bucket_id: (*bucket_id).into(),
            },
            NativeInstructionV2::CreateProofFromBucketOfAmount(
                NativeCreateProofFromBucketOfAmount { bucket_id, amount },
            ) => Self::CreateProofFromBucketOfAmount {
                bucket_id: (*bucket_id).into(),
                amount: Arc::new(Decimal(*amount)),
            },
            NativeInstructionV2::CreateProofFromBucketOfNonFungibles(
                NativeCreateProofFromBucketOfNonFungibles { bucket_id, ids },
            ) => Self::CreateProofFromBucketOfNonFungibles {
                bucket_id: (*bucket_id).into(),
                ids: ids.iter().cloned().map(Into::into).collect(),
            },
            NativeInstructionV2::BurnResource(NativeBurnResource {
                bucket_id,
            }) => Self::BurnResource {
                bucket_id: (*bucket_id).into(),
            },
            NativeInstructionV2::CloneProof(NativeCloneProof { proof_id }) => {
                Self::CloneProof {
                    proof_id: (*proof_id).into(),
                }
            }
            NativeInstructionV2::DropProof(NativeDropProof { proof_id }) => {
                Self::DropProof {
                    proof_id: (*proof_id).into(),
                }
            }
            NativeInstructionV2::DropAllProofs { .. } => Self::DropAllProofs,
            NativeInstructionV2::AllocateGlobalAddress(
                NativeAllocateGlobalAddress {
                    package_address,
                    blueprint_name,
                },
            ) => Self::AllocateGlobalAddress {
                package_address: Arc::new(Address::from_typed_node_id(
                    *package_address,
                    network_id,
                )),
                blueprint_name: blueprint_name.clone(),
            },
            NativeInstructionV2::CallFunction(NativeCallFunction {
                package_address,
                blueprint_name,
                function_name,
                args,
            }) => Self::CallFunction {
                package_address: ManifestAddress::from_dynamic_package_address(
                    package_address,
                    network_id,
                ),
                blueprint_name: blueprint_name.to_owned(),
                function_name: function_name.to_owned(),
                args: ManifestValue::from_native(args, network_id),
            },
            NativeInstructionV2::CallMethod(NativeCallMethod {
                address,
                method_name,
                args,
            }) => Self::CallMethod {
                address: ManifestAddress::from_dynamic_global_address(
                    address, network_id,
                ),
                method_name: method_name.to_owned(),
                args: ManifestValue::from_native(args, network_id),
            },
            NativeInstructionV2::CallMetadataMethod(
                NativeCallMetadataMethod {
                    address,
                    method_name,
                    args,
                },
            ) => Self::CallMetadataMethod {
                address: ManifestAddress::from_dynamic_global_address(
                    address, network_id,
                ),
                method_name: method_name.to_owned(),
                args: ManifestValue::from_native(args, network_id),
            },
            NativeInstructionV2::CallRoleAssignmentMethod(
                NativeCallRoleAssignmentMethod {
                    address,
                    method_name,
                    args,
                },
            ) => Self::CallRoleAssignmentMethod {
                address: ManifestAddress::from_dynamic_global_address(
                    address, network_id,
                ),
                method_name: method_name.to_owned(),
                args: ManifestValue::from_native(args, network_id),
            },
            NativeInstructionV2::CallRoyaltyMethod(
                NativeCallRoyaltyMethod {
                    address,
                    method_name,
                    args,
                },
            ) => Self::CallRoyaltyMethod {
                address: ManifestAddress::from_dynamic_global_address(
                    address, network_id,
                ),
                method_name: method_name.to_owned(),
                args: ManifestValue::from_native(args, network_id),
            },
            NativeInstructionV2::CallDirectVaultMethod(
                NativeCallDirectVaultMethod {
                    address,
                    method_name,
                    args,
                },
            ) => Self::CallDirectVaultMethod {
                address: Arc::new(Address::from_typed_node_id(
                    *address, network_id,
                )),
                method_name: method_name.to_owned(),
                args: ManifestValue::from_native(args, network_id),
            },
            NativeInstructionV2::YieldToParent(NativeYieldToParent {
                args,
            }) => Self::YieldToParent {
                args: ManifestValue::from_native(args, network_id),
            },
            NativeInstructionV2::YieldToChild(NativeYieldToChild {
                child_index,
                args,
            }) => Self::YieldToChild {
                child_index: child_index.0,
                args: ManifestValue::from_native(args, network_id),
            },
            NativeInstructionV2::VerifyParent(NativeVerifyParent {
                access_rule,
            }) => Self::VerifyParent {
                access_rule: Arc::new(AccessRule(access_rule.clone())),
            },
            NativeInstructionV2::AssertWorktopResourcesOnly(
                assert_worktop_resources_only,
            ) => Self::AssertWorktopResourcesOnly {
                constraints: assert_worktop_resources_only
                    .constraints
                    .iter()
                    .map(|(address, constraint)| {
                        (
                            Address::unsafe_from_raw(
                                address.into_node_id(),
                                network_id,
                            )
                            .as_str(),
                            constraint.clone().into(),
                        )
                    })
                    .collect(),
            },
            NativeInstructionV2::AssertWorktopResourcesInclude(
                assert_worktop_resources_include,
            ) => Self::AssertWorktopResourcesInclude {
                constraints: assert_worktop_resources_include
                    .constraints
                    .iter()
                    .map(|(address, constraint)| {
                        (
                            Address::unsafe_from_raw(
                                address.into_node_id(),
                                network_id,
                            )
                            .as_str(),
                            constraint.clone().into(),
                        )
                    })
                    .collect(),
            },
            NativeInstructionV2::AssertNextCallReturnsOnly(
                assert_next_call_returns_only,
            ) => Self::AssertNextCallReturnsOnly {
                constraints: assert_next_call_returns_only
                    .constraints
                    .iter()
                    .map(|(address, constraint)| {
                        (
                            Address::unsafe_from_raw(
                                address.into_node_id(),
                                network_id,
                            )
                            .as_str(),
                            constraint.clone().into(),
                        )
                    })
                    .collect(),
            },
            NativeInstructionV2::AssertNextCallReturnsInclude(
                assert_next_call_returns_include,
            ) => Self::AssertNextCallReturnsInclude {
                constraints: assert_next_call_returns_include
                    .constraints
                    .iter()
                    .map(|(address, constraint)| {
                        (
                            Address::unsafe_from_raw(
                                address.into_node_id(),
                                network_id,
                            )
                            .as_str(),
                            constraint.clone().into(),
                        )
                    })
                    .collect(),
            },
            NativeInstructionV2::AssertBucketContents(
                assert_bucket_contents,
            ) => Self::AssertBucketContents {
                constraint: assert_bucket_contents.constraint.clone().into(),
                bucket_id: assert_bucket_contents.bucket_id.into(),
            },
        }
    }

    pub fn to_native(&self) -> Result<NativeInstructionV2> {
        let value = match self {
            Self::TakeAllFromWorktop { resource_address } => {
                NativeInstructionV2::TakeAllFromWorktop(
                    NativeTakeAllFromWorktop {
                        resource_address: (*resource_address.as_ref())
                            .try_into()?,
                    },
                )
            }
            Self::TakeFromWorktop {
                resource_address,
                amount,
            } => NativeInstructionV2::TakeFromWorktop(NativeTakeFromWorktop {
                resource_address: (*resource_address.as_ref()).try_into()?,
                amount: amount.0,
            }),
            Self::TakeNonFungiblesFromWorktop {
                resource_address,
                ids,
            } => NativeInstructionV2::TakeNonFungiblesFromWorktop(
                NativeTakeNonFungiblesFromWorktop {
                    resource_address: (*resource_address.as_ref())
                        .try_into()?,
                    ids: ids
                        .iter()
                        .cloned()
                        .map(TryInto::try_into)
                        .collect::<Result<_>>()?,
                },
            ),
            Self::ReturnToWorktop { bucket_id } => {
                NativeInstructionV2::ReturnToWorktop(NativeReturnToWorktop {
                    bucket_id: (*bucket_id).into(),
                })
            }
            Self::AssertWorktopContains {
                resource_address,
                amount,
            } => NativeInstructionV2::AssertWorktopContains(
                NativeAssertWorktopContains {
                    resource_address: (*resource_address.as_ref())
                        .try_into()?,
                    amount: amount.0,
                },
            ),
            Self::AssertWorktopContainsAny { resource_address } => {
                NativeInstructionV2::AssertWorktopContainsAny(
                    NativeAssertWorktopContainsAny {
                        resource_address: (*resource_address.as_ref())
                            .try_into()?,
                    },
                )
            }
            Self::AssertWorktopContainsNonFungibles {
                resource_address,
                ids,
            } => NativeInstructionV2::AssertWorktopContainsNonFungibles(
                NativeAssertWorktopContainsNonFungibles {
                    resource_address: (*resource_address.as_ref())
                        .try_into()?,
                    ids: ids
                        .iter()
                        .cloned()
                        .map(TryInto::try_into)
                        .collect::<Result<_>>()?,
                },
            ),
            Self::PopFromAuthZone => {
                NativeInstructionV2::PopFromAuthZone(NativePopFromAuthZone)
            }
            Self::PushToAuthZone { proof_id } => {
                NativeInstructionV2::PushToAuthZone(NativePushToAuthZone {
                    proof_id: (*proof_id).into(),
                })
            }
            Self::DropNamedProofs => {
                NativeInstructionV2::DropNamedProofs(NativeDropNamedProofs)
            }
            Self::DropAuthZoneProofs => {
                NativeInstructionV2::DropAuthZoneProofs(
                    NativeDropAuthZoneProofs,
                )
            }
            Self::DropAuthZoneRegularProofs => {
                NativeInstructionV2::DropAuthZoneRegularProofs(
                    NativeDropAuthZoneRegularProofs,
                )
            }
            Self::CreateProofFromAuthZoneOfAll { resource_address } => {
                NativeInstructionV2::CreateProofFromAuthZoneOfAll(
                    NativeCreateProofFromAuthZoneOfAll {
                        resource_address: (*resource_address.as_ref())
                            .try_into()?,
                    },
                )
            }
            Self::CreateProofFromAuthZoneOfAmount {
                resource_address,
                amount,
            } => NativeInstructionV2::CreateProofFromAuthZoneOfAmount(
                NativeCreateProofFromAuthZoneOfAmount {
                    resource_address: (*resource_address.as_ref())
                        .try_into()?,
                    amount: amount.0,
                },
            ),
            Self::CreateProofFromAuthZoneOfNonFungibles {
                resource_address,
                ids,
            } => NativeInstructionV2::CreateProofFromAuthZoneOfNonFungibles(
                NativeCreateProofFromAuthZoneOfNonFungibles {
                    resource_address: (*resource_address.as_ref())
                        .try_into()?,
                    ids: ids
                        .iter()
                        .cloned()
                        .map(TryInto::try_into)
                        .collect::<Result<_>>()?,
                },
            ),
            Self::DropAuthZoneSignatureProofs => {
                NativeInstructionV2::DropAuthZoneSignatureProofs(
                    NativeDropAuthZoneSignatureProofs,
                )
            }
            Self::CreateProofFromBucketOfAll { bucket_id } => {
                NativeInstructionV2::CreateProofFromBucketOfAll(
                    NativeCreateProofFromBucketOfAll {
                        bucket_id: (*bucket_id).into(),
                    },
                )
            }
            Self::CreateProofFromBucketOfAmount { bucket_id, amount } => {
                NativeInstructionV2::CreateProofFromBucketOfAmount(
                    NativeCreateProofFromBucketOfAmount {
                        bucket_id: (*bucket_id).into(),
                        amount: amount.0,
                    },
                )
            }
            Self::CreateProofFromBucketOfNonFungibles { bucket_id, ids } => {
                NativeInstructionV2::CreateProofFromBucketOfNonFungibles(
                    NativeCreateProofFromBucketOfNonFungibles {
                        bucket_id: (*bucket_id).into(),
                        ids: ids
                            .iter()
                            .cloned()
                            .map(TryInto::try_into)
                            .collect::<Result<_>>()?,
                    },
                )
            }
            Self::BurnResource { bucket_id } => {
                NativeInstructionV2::BurnResource(NativeBurnResource {
                    bucket_id: (*bucket_id).into(),
                })
            }
            Self::CloneProof { proof_id } => {
                NativeInstructionV2::CloneProof(NativeCloneProof {
                    proof_id: (*proof_id).into(),
                })
            }
            Self::DropProof { proof_id } => {
                NativeInstructionV2::DropProof(NativeDropProof {
                    proof_id: (*proof_id).into(),
                })
            }
            Self::DropAllProofs => {
                NativeInstructionV2::DropAllProofs(NativeDropAllProofs)
            }
            Self::CallFunction {
                package_address,
                blueprint_name,
                function_name,
                args,
            } => NativeInstructionV2::CallFunction(NativeCallFunction {
                package_address: package_address.clone().try_into()?,
                blueprint_name: blueprint_name.to_string(),
                function_name: function_name.to_string(),
                args: args.to_native()?,
            }),
            Self::CallMethod {
                address,
                method_name,
                args,
            } => NativeInstructionV2::CallMethod(NativeCallMethod {
                address: address.clone().try_into()?,
                method_name: method_name.to_owned(),
                args: args.to_native()?,
            }),
            Self::CallMetadataMethod {
                address,
                method_name,
                args,
            } => NativeInstructionV2::CallMetadataMethod(
                NativeCallMetadataMethod {
                    address: address.clone().try_into()?,
                    method_name: method_name.to_owned(),
                    args: args.to_native()?,
                },
            ),
            Self::CallRoleAssignmentMethod {
                address,
                method_name,
                args,
            } => NativeInstructionV2::CallRoleAssignmentMethod(
                NativeCallRoleAssignmentMethod {
                    address: address.clone().try_into()?,
                    method_name: method_name.to_owned(),
                    args: args.to_native()?,
                },
            ),
            Self::CallRoyaltyMethod {
                address,
                method_name,
                args,
            } => NativeInstructionV2::CallRoyaltyMethod(
                NativeCallRoyaltyMethod {
                    address: address.clone().try_into()?,
                    method_name: method_name.to_owned(),
                    args: args.to_native()?,
                },
            ),
            Self::CallDirectVaultMethod {
                address,
                method_name,
                args,
            } => NativeInstructionV2::CallDirectVaultMethod(
                NativeCallDirectVaultMethod {
                    address: (**address).try_into()?,
                    method_name: method_name.to_owned(),
                    args: args.to_native()?,
                },
            ),
            Self::AllocateGlobalAddress {
                package_address,
                blueprint_name,
            } => NativeInstructionV2::AllocateGlobalAddress(
                NativeAllocateGlobalAddress {
                    package_address: (**package_address).try_into()?,
                    blueprint_name: blueprint_name.to_string(),
                },
            ),
            Self::YieldToParent { args } => {
                NativeInstructionV2::YieldToParent(NativeYieldToParent {
                    args: args.to_native()?,
                })
            }
            Self::YieldToChild { child_index, args } => {
                NativeInstructionV2::YieldToChild(NativeYieldToChild {
                    child_index: NativeManifestNamedIntentIndex(*child_index),
                    args: args.to_native()?,
                })
            }
            Self::VerifyParent { access_rule } => {
                NativeInstructionV2::VerifyParent(NativeVerifyParent {
                    access_rule: access_rule.0.clone(),
                })
            }
            Self::AssertWorktopResourcesOnly { constraints } => {
                NativeInstructionV2::AssertWorktopResourcesOnly(
                    NativeAssertWorktopResourcesOnly {
                        constraints: constraints
                            .iter()
                            .map(|(address, constraint)| -> Result<_> {
                                let address = Address::new(address.to_owned())?;
                                let resource_address =
                                    NativeResourceAddress::try_from(
                                        address.as_bytes(),
                                    )?;
                                let constraint =
                                    constraint.clone().try_into()?;

                                Ok((resource_address, constraint))
                            })
                            .collect::<Result<
                                Vec<(
                                    NativeResourceAddress,
                                    NativeManifestResourceConstraint,
                                )>,
                            >>()
                            .map(|constraints| {
                                constraints.into_iter().fold(
                                    NativeManifestResourceConstraints::new(),
                                    |acc, (address, constraint)| {
                                        acc.with(address, constraint)
                                    },
                                )
                            })?,
                    },
                )
            }
            Self::AssertWorktopResourcesInclude { constraints } => {
                NativeInstructionV2::AssertWorktopResourcesInclude(
                    NativeAssertWorktopResourcesInclude {
                        constraints: constraints
                            .iter()
                            .map(|(address, constraint)| -> Result<_> {
                                let address = Address::new(address.to_owned())?;
                                let resource_address =
                                    NativeResourceAddress::try_from(
                                        address.as_bytes(),
                                    )?;
                                let constraint =
                                    constraint.clone().try_into()?;

                                Ok((resource_address, constraint))
                            })
                            .collect::<Result<
                                Vec<(
                                    NativeResourceAddress,
                                    NativeManifestResourceConstraint,
                                )>,
                            >>()
                            .map(|constraints| {
                                constraints.into_iter().fold(
                                    NativeManifestResourceConstraints::new(),
                                    |acc, (address, constraint)| {
                                        acc.with(address, constraint)
                                    },
                                )
                            })?,
                    },
                )
            }
            Self::AssertNextCallReturnsOnly { constraints } => {
                NativeInstructionV2::AssertNextCallReturnsOnly(
                    NativeAssertNextCallReturnsOnly {
                        constraints: constraints
                            .iter()
                            .map(|(address, constraint)| -> Result<_> {
                                let address = Address::new(address.to_owned())?;
                                let resource_address =
                                    NativeResourceAddress::try_from(
                                        address.as_bytes(),
                                    )?;
                                let constraint =
                                    constraint.clone().try_into()?;

                                Ok((resource_address, constraint))
                            })
                            .collect::<Result<
                                Vec<(
                                    NativeResourceAddress,
                                    NativeManifestResourceConstraint,
                                )>,
                            >>()
                            .map(|constraints| {
                                constraints.into_iter().fold(
                                    NativeManifestResourceConstraints::new(),
                                    |acc, (address, constraint)| {
                                        acc.with(address, constraint)
                                    },
                                )
                            })?,
                    },
                )
            }
            Self::AssertNextCallReturnsInclude { constraints } => {
                NativeInstructionV2::AssertNextCallReturnsInclude(
                    NativeAssertNextCallReturnsInclude {
                        constraints: constraints
                            .iter()
                            .map(|(address, constraint)| -> Result<_> {
                                let address = Address::new(address.to_owned())?;
                                let resource_address =
                                    NativeResourceAddress::try_from(
                                        address.as_bytes(),
                                    )?;
                                let constraint =
                                    constraint.clone().try_into()?;

                                Ok((resource_address, constraint))
                            })
                            .collect::<Result<
                                Vec<(
                                    NativeResourceAddress,
                                    NativeManifestResourceConstraint,
                                )>,
                            >>()
                            .map(|constraints| {
                                constraints.into_iter().fold(
                                    NativeManifestResourceConstraints::new(),
                                    |acc, (address, constraint)| {
                                        acc.with(address, constraint)
                                    },
                                )
                            })?,
                    },
                )
            }
            Self::AssertBucketContents {
                bucket_id,
                constraint,
            } => NativeInstructionV2::AssertBucketContents(
                NativeAssertBucketContents {
                    bucket_id: (*bucket_id).into(),
                    constraint: constraint.clone().try_into()?,
                },
            ),
        };
        Ok(value)
    }
}
