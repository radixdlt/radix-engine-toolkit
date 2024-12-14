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
pub enum InstructionV1 {
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
}

impl InstructionV1 {
    pub fn from_native(native: &NativeInstructionV1, network_id: u8) -> Self {
        match native {
            NativeInstructionV1::TakeAllFromWorktop(
                NativeTakeAllFromWorktop { resource_address },
            ) => Self::TakeAllFromWorktop {
                resource_address: Arc::new(Address::from_typed_node_id(
                    *resource_address,
                    network_id,
                )),
            },
            NativeInstructionV1::TakeFromWorktop(NativeTakeFromWorktop {
                resource_address,
                amount,
            }) => Self::TakeFromWorktop {
                resource_address: Arc::new(Address::from_typed_node_id(
                    *resource_address,
                    network_id,
                )),
                amount: Arc::new(Decimal(*amount)),
            },
            NativeInstructionV1::TakeNonFungiblesFromWorktop(
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

            NativeInstructionV1::ReturnToWorktop(NativeReturnToWorktop {
                bucket_id,
            }) => Self::ReturnToWorktop {
                bucket_id: (*bucket_id).into(),
            },

            NativeInstructionV1::AssertWorktopContains(
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
            NativeInstructionV1::AssertWorktopContainsAny(
                NativeAssertWorktopContainsAny { resource_address },
            ) => Self::AssertWorktopContainsAny {
                resource_address: Arc::new(Address::from_typed_node_id(
                    *resource_address,
                    network_id,
                )),
            },
            NativeInstructionV1::AssertWorktopContainsNonFungibles(
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
            NativeInstructionV1::PopFromAuthZone { .. } => {
                Self::PopFromAuthZone
            }
            NativeInstructionV1::PushToAuthZone(NativePushToAuthZone {
                proof_id,
            }) => Self::PushToAuthZone {
                proof_id: (*proof_id).into(),
            },
            NativeInstructionV1::DropNamedProofs { .. } => {
                Self::DropNamedProofs
            }
            NativeInstructionV1::DropAuthZoneProofs { .. } => {
                Self::DropAuthZoneProofs
            }
            NativeInstructionV1::DropAuthZoneRegularProofs { .. } => {
                Self::DropAuthZoneRegularProofs
            }
            NativeInstructionV1::DropAuthZoneSignatureProofs { .. } => {
                Self::DropAuthZoneSignatureProofs
            }
            NativeInstructionV1::CreateProofFromAuthZoneOfAll(
                NativeCreateProofFromAuthZoneOfAll { resource_address },
            ) => Self::CreateProofFromAuthZoneOfAll {
                resource_address: Arc::new(Address::from_typed_node_id(
                    *resource_address,
                    network_id,
                )),
            },
            NativeInstructionV1::CreateProofFromAuthZoneOfAmount(
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
            NativeInstructionV1::CreateProofFromAuthZoneOfNonFungibles(
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
            NativeInstructionV1::CreateProofFromBucketOfAll(
                NativeCreateProofFromBucketOfAll { bucket_id },
            ) => Self::CreateProofFromBucketOfAll {
                bucket_id: (*bucket_id).into(),
            },
            NativeInstructionV1::CreateProofFromBucketOfAmount(
                NativeCreateProofFromBucketOfAmount { bucket_id, amount },
            ) => Self::CreateProofFromBucketOfAmount {
                bucket_id: (*bucket_id).into(),
                amount: Arc::new(Decimal(*amount)),
            },
            NativeInstructionV1::CreateProofFromBucketOfNonFungibles(
                NativeCreateProofFromBucketOfNonFungibles { bucket_id, ids },
            ) => Self::CreateProofFromBucketOfNonFungibles {
                bucket_id: (*bucket_id).into(),
                ids: ids.iter().cloned().map(Into::into).collect(),
            },
            NativeInstructionV1::BurnResource(NativeBurnResource {
                bucket_id,
            }) => Self::BurnResource {
                bucket_id: (*bucket_id).into(),
            },
            NativeInstructionV1::CloneProof(NativeCloneProof { proof_id }) => {
                Self::CloneProof {
                    proof_id: (*proof_id).into(),
                }
            }
            NativeInstructionV1::DropProof(NativeDropProof { proof_id }) => {
                Self::DropProof {
                    proof_id: (*proof_id).into(),
                }
            }
            NativeInstructionV1::DropAllProofs { .. } => Self::DropAllProofs,
            NativeInstructionV1::AllocateGlobalAddress(
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
            NativeInstructionV1::CallFunction(NativeCallFunction {
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
            NativeInstructionV1::CallMethod(NativeCallMethod {
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
            NativeInstructionV1::CallMetadataMethod(
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
            NativeInstructionV1::CallRoleAssignmentMethod(
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
            NativeInstructionV1::CallRoyaltyMethod(
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
            NativeInstructionV1::CallDirectVaultMethod(
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
        }
    }

    pub fn to_native(&self) -> Result<NativeInstructionV1> {
        let value = match self {
            Self::TakeAllFromWorktop { resource_address } => {
                NativeInstructionV1::TakeAllFromWorktop(
                    NativeTakeAllFromWorktop {
                        resource_address: (*resource_address.as_ref())
                            .try_into()?,
                    },
                )
            }
            Self::TakeFromWorktop {
                resource_address,
                amount,
            } => NativeInstructionV1::TakeFromWorktop(NativeTakeFromWorktop {
                resource_address: (*resource_address.as_ref()).try_into()?,
                amount: amount.0,
            }),
            Self::TakeNonFungiblesFromWorktop {
                resource_address,
                ids,
            } => NativeInstructionV1::TakeNonFungiblesFromWorktop(
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
                NativeInstructionV1::ReturnToWorktop(NativeReturnToWorktop {
                    bucket_id: (*bucket_id).into(),
                })
            }
            Self::AssertWorktopContains {
                resource_address,
                amount,
            } => NativeInstructionV1::AssertWorktopContains(
                NativeAssertWorktopContains {
                    resource_address: (*resource_address.as_ref())
                        .try_into()?,
                    amount: amount.0,
                },
            ),
            Self::AssertWorktopContainsAny { resource_address } => {
                NativeInstructionV1::AssertWorktopContainsAny(
                    NativeAssertWorktopContainsAny {
                        resource_address: (*resource_address.as_ref())
                            .try_into()?,
                    },
                )
            }
            Self::AssertWorktopContainsNonFungibles {
                resource_address,
                ids,
            } => NativeInstructionV1::AssertWorktopContainsNonFungibles(
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
                NativeInstructionV1::PopFromAuthZone(NativePopFromAuthZone)
            }
            Self::PushToAuthZone { proof_id } => {
                NativeInstructionV1::PushToAuthZone(NativePushToAuthZone {
                    proof_id: (*proof_id).into(),
                })
            }
            Self::DropNamedProofs => {
                NativeInstructionV1::DropNamedProofs(NativeDropNamedProofs)
            }
            Self::DropAuthZoneProofs => {
                NativeInstructionV1::DropAuthZoneProofs(
                    NativeDropAuthZoneProofs,
                )
            }
            Self::DropAuthZoneRegularProofs => {
                NativeInstructionV1::DropAuthZoneRegularProofs(
                    NativeDropAuthZoneRegularProofs,
                )
            }
            Self::CreateProofFromAuthZoneOfAll { resource_address } => {
                NativeInstructionV1::CreateProofFromAuthZoneOfAll(
                    NativeCreateProofFromAuthZoneOfAll {
                        resource_address: (*resource_address.as_ref())
                            .try_into()?,
                    },
                )
            }
            Self::CreateProofFromAuthZoneOfAmount {
                resource_address,
                amount,
            } => NativeInstructionV1::CreateProofFromAuthZoneOfAmount(
                NativeCreateProofFromAuthZoneOfAmount {
                    resource_address: (*resource_address.as_ref())
                        .try_into()?,
                    amount: amount.0,
                },
            ),
            Self::CreateProofFromAuthZoneOfNonFungibles {
                resource_address,
                ids,
            } => NativeInstructionV1::CreateProofFromAuthZoneOfNonFungibles(
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
                NativeInstructionV1::DropAuthZoneSignatureProofs(
                    NativeDropAuthZoneSignatureProofs,
                )
            }
            Self::CreateProofFromBucketOfAll { bucket_id } => {
                NativeInstructionV1::CreateProofFromBucketOfAll(
                    NativeCreateProofFromBucketOfAll {
                        bucket_id: (*bucket_id).into(),
                    },
                )
            }
            Self::CreateProofFromBucketOfAmount { bucket_id, amount } => {
                NativeInstructionV1::CreateProofFromBucketOfAmount(
                    NativeCreateProofFromBucketOfAmount {
                        bucket_id: (*bucket_id).into(),
                        amount: amount.0,
                    },
                )
            }
            Self::CreateProofFromBucketOfNonFungibles { bucket_id, ids } => {
                NativeInstructionV1::CreateProofFromBucketOfNonFungibles(
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
                NativeInstructionV1::BurnResource(NativeBurnResource {
                    bucket_id: (*bucket_id).into(),
                })
            }
            Self::CloneProof { proof_id } => {
                NativeInstructionV1::CloneProof(NativeCloneProof {
                    proof_id: (*proof_id).into(),
                })
            }
            Self::DropProof { proof_id } => {
                NativeInstructionV1::DropProof(NativeDropProof {
                    proof_id: (*proof_id).into(),
                })
            }
            Self::DropAllProofs => {
                NativeInstructionV1::DropAllProofs(NativeDropAllProofs)
            }
            Self::CallFunction {
                package_address,
                blueprint_name,
                function_name,
                args,
            } => NativeInstructionV1::CallFunction(NativeCallFunction {
                package_address: package_address.clone().try_into()?,
                blueprint_name: blueprint_name.to_string(),
                function_name: function_name.to_string(),
                args: args.to_native()?,
            }),
            Self::CallMethod {
                address,
                method_name,
                args,
            } => NativeInstructionV1::CallMethod(NativeCallMethod {
                address: address.clone().try_into()?,
                method_name: method_name.to_owned(),
                args: args.to_native()?,
            }),
            Self::CallMetadataMethod {
                address,
                method_name,
                args,
            } => NativeInstructionV1::CallMetadataMethod(
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
            } => NativeInstructionV1::CallRoleAssignmentMethod(
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
            } => NativeInstructionV1::CallRoyaltyMethod(
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
            } => NativeInstructionV1::CallDirectVaultMethod(
                NativeCallDirectVaultMethod {
                    address: (**address).try_into()?,
                    method_name: method_name.to_owned(),
                    args: args.to_native()?,
                },
            ),
            Self::AllocateGlobalAddress {
                package_address,
                blueprint_name,
            } => NativeInstructionV1::AllocateGlobalAddress(
                NativeAllocateGlobalAddress {
                    package_address: (**package_address).try_into()?,
                    blueprint_name: blueprint_name.to_string(),
                },
            ),
        };
        Ok(value)
    }
}
