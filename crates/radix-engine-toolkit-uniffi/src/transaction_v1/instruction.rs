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
    pub fn from_native(native: &engine::InstructionV1, network_id: u8) -> Self {
        match native {
            engine::InstructionV1::TakeAllFromWorktop(
                engine::TakeAllFromWorktop { resource_address },
            ) => Self::TakeAllFromWorktop {
                resource_address: Arc::new(Address::from_node_id(
                    *resource_address,
                    network_id,
                )),
            },
            engine::InstructionV1::TakeFromWorktop(
                engine::TakeFromWorktop {
                    resource_address,
                    amount,
                },
            ) => Self::TakeFromWorktop {
                resource_address: Arc::new(Address::from_node_id(
                    *resource_address,
                    network_id,
                )),
                amount: Arc::new(Decimal(*amount)),
            },
            engine::InstructionV1::TakeNonFungiblesFromWorktop(
                engine::TakeNonFungiblesFromWorktop {
                    resource_address,
                    ids,
                },
            ) => Self::TakeNonFungiblesFromWorktop {
                resource_address: Arc::new(Address::from_node_id(
                    *resource_address,
                    network_id,
                )),
                ids: ids.iter().cloned().map(Into::into).collect(),
            },

            engine::InstructionV1::ReturnToWorktop(
                engine::ReturnToWorktop { bucket_id },
            ) => Self::ReturnToWorktop {
                bucket_id: (*bucket_id).into(),
            },

            engine::InstructionV1::AssertWorktopContains(
                engine::AssertWorktopContains {
                    resource_address,
                    amount,
                },
            ) => Self::AssertWorktopContains {
                resource_address: Arc::new(Address::from_node_id(
                    *resource_address,
                    network_id,
                )),
                amount: Arc::new(Decimal(*amount)),
            },
            engine::InstructionV1::AssertWorktopContainsAny(
                engine::AssertWorktopContainsAny { resource_address },
            ) => Self::AssertWorktopContainsAny {
                resource_address: Arc::new(Address::from_node_id(
                    *resource_address,
                    network_id,
                )),
            },
            engine::InstructionV1::AssertWorktopContainsNonFungibles(
                engine::AssertWorktopContainsNonFungibles {
                    resource_address,
                    ids,
                },
            ) => Self::AssertWorktopContainsNonFungibles {
                resource_address: Arc::new(Address::from_node_id(
                    *resource_address,
                    network_id,
                )),
                ids: ids.iter().cloned().map(Into::into).collect(),
            },
            engine::InstructionV1::PopFromAuthZone(..) => Self::PopFromAuthZone,
            engine::InstructionV1::PushToAuthZone(engine::PushToAuthZone {
                proof_id,
            }) => Self::PushToAuthZone {
                proof_id: (*proof_id).into(),
            },
            engine::InstructionV1::DropNamedProofs(..) => Self::DropNamedProofs,
            engine::InstructionV1::DropAuthZoneProofs(..) => {
                Self::DropAuthZoneProofs
            }
            engine::InstructionV1::DropAuthZoneRegularProofs(..) => {
                Self::DropAuthZoneRegularProofs
            }
            engine::InstructionV1::DropAuthZoneSignatureProofs(..) => {
                Self::DropAuthZoneSignatureProofs
            }
            engine::InstructionV1::CreateProofFromAuthZoneOfAll(
                engine::CreateProofFromAuthZoneOfAll { resource_address },
            ) => Self::CreateProofFromAuthZoneOfAll {
                resource_address: Arc::new(Address::from_node_id(
                    *resource_address,
                    network_id,
                )),
            },
            engine::InstructionV1::CreateProofFromAuthZoneOfAmount(
                engine::CreateProofFromAuthZoneOfAmount {
                    resource_address,
                    amount,
                },
            ) => Self::CreateProofFromAuthZoneOfAmount {
                resource_address: Arc::new(Address::from_node_id(
                    *resource_address,
                    network_id,
                )),
                amount: Arc::new(Decimal(*amount)),
            },
            engine::InstructionV1::CreateProofFromAuthZoneOfNonFungibles(
                engine::CreateProofFromAuthZoneOfNonFungibles {
                    resource_address,
                    ids,
                },
            ) => Self::CreateProofFromAuthZoneOfNonFungibles {
                resource_address: Arc::new(Address::from_node_id(
                    *resource_address,
                    network_id,
                )),
                ids: ids.iter().cloned().map(Into::into).collect(),
            },
            engine::InstructionV1::CreateProofFromBucketOfAll(
                engine::CreateProofFromBucketOfAll { bucket_id },
            ) => Self::CreateProofFromBucketOfAll {
                bucket_id: (*bucket_id).into(),
            },
            engine::InstructionV1::CreateProofFromBucketOfAmount(
                engine::CreateProofFromBucketOfAmount { bucket_id, amount },
            ) => Self::CreateProofFromBucketOfAmount {
                bucket_id: (*bucket_id).into(),
                amount: Arc::new(Decimal(*amount)),
            },
            engine::InstructionV1::CreateProofFromBucketOfNonFungibles(
                engine::CreateProofFromBucketOfNonFungibles { bucket_id, ids },
            ) => Self::CreateProofFromBucketOfNonFungibles {
                bucket_id: (*bucket_id).into(),
                ids: ids.iter().cloned().map(Into::into).collect(),
            },
            engine::InstructionV1::BurnResource(engine::BurnResource {
                bucket_id,
            }) => Self::BurnResource {
                bucket_id: (*bucket_id).into(),
            },
            engine::InstructionV1::CloneProof(engine::CloneProof {
                proof_id,
            }) => Self::CloneProof {
                proof_id: (*proof_id).into(),
            },
            engine::InstructionV1::DropProof(engine::DropProof {
                proof_id,
            }) => Self::DropProof {
                proof_id: (*proof_id).into(),
            },
            engine::InstructionV1::DropAllProofs(..) => Self::DropAllProofs,
            engine::InstructionV1::AllocateGlobalAddress(
                engine::AllocateGlobalAddress {
                    package_address,
                    blueprint_name,
                },
            ) => Self::AllocateGlobalAddress {
                package_address: Arc::new(Address::from_node_id(
                    *package_address,
                    network_id,
                )),
                blueprint_name: blueprint_name.clone(),
            },
            engine::InstructionV1::CallFunction(engine::CallFunction {
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
            engine::InstructionV1::CallMethod(engine::CallMethod {
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
            engine::InstructionV1::CallMetadataMethod(
                engine::CallMetadataMethod {
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
            engine::InstructionV1::CallRoleAssignmentMethod(
                engine::CallRoleAssignmentMethod {
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
            engine::InstructionV1::CallRoyaltyMethod(
                engine::CallRoyaltyMethod {
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
            engine::InstructionV1::CallDirectVaultMethod(
                engine::CallDirectVaultMethod {
                    address,
                    method_name,
                    args,
                },
            ) => Self::CallDirectVaultMethod {
                address: Arc::new(Address::from_node_id(
                    *address, network_id,
                )),
                method_name: method_name.to_owned(),
                args: ManifestValue::from_native(args, network_id),
            },
        }
    }

    pub fn to_native(&self) -> Result<engine::InstructionV1> {
        let value = match self {
            Self::TakeAllFromWorktop { resource_address } => {
                engine::InstructionV1::TakeAllFromWorktop(
                    engine::TakeAllFromWorktop {
                        resource_address: (*resource_address.as_ref())
                            .try_into()?,
                    },
                )
            }
            Self::TakeFromWorktop {
                resource_address,
                amount,
            } => engine::InstructionV1::TakeFromWorktop(
                engine::TakeFromWorktop {
                    resource_address: (*resource_address.as_ref())
                        .try_into()?,
                    amount: amount.0,
                },
            ),
            Self::TakeNonFungiblesFromWorktop {
                resource_address,
                ids,
            } => engine::InstructionV1::TakeNonFungiblesFromWorktop(
                engine::TakeNonFungiblesFromWorktop {
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
                engine::InstructionV1::ReturnToWorktop(
                    engine::ReturnToWorktop {
                        bucket_id: (*bucket_id).into(),
                    },
                )
            }
            Self::AssertWorktopContains {
                resource_address,
                amount,
            } => engine::InstructionV1::AssertWorktopContains(
                engine::AssertWorktopContains {
                    resource_address: (*resource_address.as_ref())
                        .try_into()?,
                    amount: amount.0,
                },
            ),
            Self::AssertWorktopContainsAny { resource_address } => {
                engine::InstructionV1::AssertWorktopContainsAny(
                    engine::AssertWorktopContainsAny {
                        resource_address: (*resource_address.as_ref())
                            .try_into()?,
                    },
                )
            }
            Self::AssertWorktopContainsNonFungibles {
                resource_address,
                ids,
            } => engine::InstructionV1::AssertWorktopContainsNonFungibles(
                engine::AssertWorktopContainsNonFungibles {
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
                engine::InstructionV1::PopFromAuthZone(engine::PopFromAuthZone)
            }
            Self::PushToAuthZone { proof_id } => {
                engine::InstructionV1::PushToAuthZone(engine::PushToAuthZone {
                    proof_id: (*proof_id).into(),
                })
            }
            Self::DropNamedProofs => {
                engine::InstructionV1::DropNamedProofs(engine::DropNamedProofs)
            }
            Self::DropAuthZoneProofs => {
                engine::InstructionV1::DropAuthZoneProofs(
                    engine::DropAuthZoneProofs,
                )
            }
            Self::DropAuthZoneRegularProofs => {
                engine::InstructionV1::DropAuthZoneRegularProofs(
                    engine::DropAuthZoneRegularProofs,
                )
            }
            Self::CreateProofFromAuthZoneOfAll { resource_address } => {
                engine::InstructionV1::CreateProofFromAuthZoneOfAll(
                    engine::CreateProofFromAuthZoneOfAll {
                        resource_address: (*resource_address.as_ref())
                            .try_into()?,
                    },
                )
            }
            Self::CreateProofFromAuthZoneOfAmount {
                resource_address,
                amount,
            } => engine::InstructionV1::CreateProofFromAuthZoneOfAmount(
                engine::CreateProofFromAuthZoneOfAmount {
                    resource_address: (*resource_address.as_ref())
                        .try_into()?,
                    amount: amount.0,
                },
            ),
            Self::CreateProofFromAuthZoneOfNonFungibles {
                resource_address,
                ids,
            } => engine::InstructionV1::CreateProofFromAuthZoneOfNonFungibles(
                engine::CreateProofFromAuthZoneOfNonFungibles {
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
                engine::InstructionV1::DropAuthZoneSignatureProofs(
                    engine::DropAuthZoneSignatureProofs,
                )
            }
            Self::CreateProofFromBucketOfAll { bucket_id } => {
                engine::InstructionV1::CreateProofFromBucketOfAll(
                    engine::CreateProofFromBucketOfAll {
                        bucket_id: (*bucket_id).into(),
                    },
                )
            }
            Self::CreateProofFromBucketOfAmount { bucket_id, amount } => {
                engine::InstructionV1::CreateProofFromBucketOfAmount(
                    engine::CreateProofFromBucketOfAmount {
                        bucket_id: (*bucket_id).into(),
                        amount: amount.0,
                    },
                )
            }
            Self::CreateProofFromBucketOfNonFungibles { bucket_id, ids } => {
                engine::InstructionV1::CreateProofFromBucketOfNonFungibles(
                    engine::CreateProofFromBucketOfNonFungibles {
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
                engine::InstructionV1::BurnResource(engine::BurnResource {
                    bucket_id: (*bucket_id).into(),
                })
            }
            Self::CloneProof { proof_id } => {
                engine::InstructionV1::CloneProof(engine::CloneProof {
                    proof_id: (*proof_id).into(),
                })
            }
            Self::DropProof { proof_id } => {
                engine::InstructionV1::DropProof(engine::DropProof {
                    proof_id: (*proof_id).into(),
                })
            }
            Self::DropAllProofs => {
                engine::InstructionV1::DropAllProofs(engine::DropAllProofs)
            }
            Self::CallFunction {
                package_address,
                blueprint_name,
                function_name,
                args,
            } => engine::InstructionV1::CallFunction(engine::CallFunction {
                package_address: package_address.clone().try_into()?,
                blueprint_name: blueprint_name.to_string(),
                function_name: function_name.to_string(),
                args: args.to_native()?,
            }),
            Self::CallMethod {
                address,
                method_name,
                args,
            } => engine::InstructionV1::CallMethod(engine::CallMethod {
                address: address.clone().try_into()?,
                method_name: method_name.to_owned(),
                args: args.to_native()?,
            }),
            Self::CallMetadataMethod {
                address,
                method_name,
                args,
            } => engine::InstructionV1::CallMetadataMethod(
                engine::CallMetadataMethod {
                    address: address.clone().try_into()?,
                    method_name: method_name.to_owned(),
                    args: args.to_native()?,
                },
            ),
            Self::CallRoleAssignmentMethod {
                address,
                method_name,
                args,
            } => engine::InstructionV1::CallRoleAssignmentMethod(
                engine::CallRoleAssignmentMethod {
                    address: address.clone().try_into()?,
                    method_name: method_name.to_owned(),
                    args: args.to_native()?,
                },
            ),
            Self::CallRoyaltyMethod {
                address,
                method_name,
                args,
            } => engine::InstructionV1::CallRoyaltyMethod(
                engine::CallRoyaltyMethod {
                    address: address.clone().try_into()?,
                    method_name: method_name.to_owned(),
                    args: args.to_native()?,
                },
            ),
            Self::CallDirectVaultMethod {
                address,
                method_name,
                args,
            } => engine::InstructionV1::CallDirectVaultMethod(
                engine::CallDirectVaultMethod {
                    address: (**address).try_into()?,
                    method_name: method_name.to_owned(),
                    args: args.to_native()?,
                },
            ),
            Self::AllocateGlobalAddress {
                package_address,
                blueprint_name,
            } => engine::InstructionV1::AllocateGlobalAddress(
                engine::AllocateGlobalAddress {
                    package_address: (**package_address).try_into()?,
                    blueprint_name: blueprint_name.to_string(),
                },
            ),
        };
        Ok(value)
    }
}
