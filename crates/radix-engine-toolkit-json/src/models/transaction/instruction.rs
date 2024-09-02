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

use std::ops::Deref;

use crate::prelude::*;

use radix_common::prelude::*;
use radix_transactions::prelude::manifest_instruction::*;
use radix_transactions::prelude::*;
use schemars::*;
use serde::*;

#[typeshare::typeshare]
#[derive(
    Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq, Hash,
)]
#[serde(tag = "kind", content = "value")]
pub enum SerializableInstruction {
    TakeAllFromWorktop {
        resource_address: SerializableNodeId,
    },

    TakeFromWorktop {
        resource_address: SerializableNodeId,
        amount: SerializableDecimal,
    },

    TakeNonFungiblesFromWorktop {
        resource_address: SerializableNodeId,
        ids: Vec<SerializableNonFungibleLocalId>,
    },

    ReturnToWorktop {
        bucket_id: SerializableU32,
    },

    AssertWorktopContainsAny {
        resource_address: SerializableNodeId,
    },

    AssertWorktopContains {
        resource_address: SerializableNodeId,
        amount: SerializableDecimal,
    },

    AssertWorktopContainsNonFungibles {
        resource_address: SerializableNodeId,
        ids: Vec<SerializableNonFungibleLocalId>,
    },

    PopFromAuthZone,

    PushToAuthZone {
        proof_id: SerializableU32,
    },

    CreateProofFromAuthZoneOfAmount {
        resource_address: SerializableNodeId,
        amount: SerializableDecimal,
    },

    CreateProofFromAuthZoneOfNonFungibles {
        resource_address: SerializableNodeId,
        ids: Vec<SerializableNonFungibleLocalId>,
    },

    CreateProofFromAuthZoneOfAll {
        resource_address: SerializableNodeId,
    },

    DropAllProofs,
    DropNamedProofs,
    DropAuthZoneProofs,
    DropAuthZoneRegularProofs,
    DropAuthZoneSignatureProofs,

    CreateProofFromBucketOfAmount {
        bucket_id: SerializableU32,
        amount: SerializableDecimal,
    },

    CreateProofFromBucketOfNonFungibles {
        bucket_id: SerializableU32,
        ids: Vec<SerializableNonFungibleLocalId>,
    },

    CreateProofFromBucketOfAll {
        bucket_id: SerializableU32,
    },

    BurnResource {
        bucket_id: SerializableU32,
    },

    CloneProof {
        proof_id: SerializableU32,
    },

    DropProof {
        proof_id: SerializableU32,
    },

    CallFunction {
        package_address: SerializableManifestAddress,
        blueprint_name: String,
        function_name: String,
        args: SerializableManifestValue,
    },

    CallMethod {
        address: SerializableManifestAddress,
        method_name: String,
        args: SerializableManifestValue,
    },

    CallRoyaltyMethod {
        address: SerializableManifestAddress,
        method_name: String,
        args: SerializableManifestValue,
    },

    CallMetadataMethod {
        address: SerializableManifestAddress,
        method_name: String,
        args: SerializableManifestValue,
    },

    CallRoleAssignmentMethod {
        address: SerializableManifestAddress,
        method_name: String,
        args: SerializableManifestValue,
    },

    CallDirectVaultMethod {
        address: SerializableNodeId,
        method_name: String,
        args: SerializableManifestValue,
    },

    AllocateGlobalAddress {
        package_address: SerializableNodeId,
        blueprint_name: String,
    },
}

impl SerializableInstruction {
    pub fn from_instruction(
        instruction: &InstructionV1,
        network_id: u8,
    ) -> Result<Self, InstructionConversionError> {
        let instruction = match instruction {
            InstructionV1::TakeFromWorktop(TakeFromWorktop {
                resource_address,
                amount,
            }) => Self::TakeFromWorktop {
                resource_address: SerializableNodeId::new(
                    resource_address.into_node_id(),
                    network_id,
                ),
                amount: SerializableDecimal::from(*amount),
            },
            InstructionV1::TakeNonFungiblesFromWorktop(
                TakeNonFungiblesFromWorktop {
                    resource_address,
                    ids,
                },
            ) => Self::TakeNonFungiblesFromWorktop {
                ids: ids
                    .iter()
                    .map(|local_id| {
                        SerializableNonFungibleLocalId::from(local_id.clone())
                    })
                    .collect(),
                resource_address: SerializableNodeId::new(
                    resource_address.into_node_id(),
                    network_id,
                ),
            },
            InstructionV1::TakeAllFromWorktop(TakeAllFromWorktop {
                resource_address,
            }) => Self::TakeAllFromWorktop {
                resource_address: SerializableNodeId::new(
                    resource_address.into_node_id(),
                    network_id,
                ),
            },
            InstructionV1::ReturnToWorktop(ReturnToWorktop { bucket_id }) => {
                Self::ReturnToWorktop {
                    bucket_id: SerializableU32::from(bucket_id.0),
                }
            }
            InstructionV1::AssertWorktopContains(AssertWorktopContains {
                resource_address,
                amount,
            }) => Self::AssertWorktopContains {
                resource_address: SerializableNodeId::new(
                    resource_address.into_node_id(),
                    network_id,
                ),
                amount: SerializableDecimal::from(*amount),
            },
            InstructionV1::AssertWorktopContainsAny(
                AssertWorktopContainsAny { resource_address },
            ) => Self::AssertWorktopContainsAny {
                resource_address: SerializableNodeId::new(
                    resource_address.into_node_id(),
                    network_id,
                ),
            },
            InstructionV1::AssertWorktopContainsNonFungibles(
                AssertWorktopContainsNonFungibles {
                    resource_address,
                    ids,
                },
            ) => Self::AssertWorktopContainsNonFungibles {
                resource_address: SerializableNodeId::new(
                    resource_address.into_node_id(),
                    network_id,
                ),
                ids: ids
                    .iter()
                    .map(|local_id| {
                        SerializableNonFungibleLocalId::from(local_id.clone())
                    })
                    .collect(),
            },
            InstructionV1::PopFromAuthZone => Self::PopFromAuthZone,
            InstructionV1::PushToAuthZone(PushToAuthZone { proof_id }) => {
                Self::PushToAuthZone {
                    proof_id: SerializableU32::from(proof_id.0),
                }
            }
            InstructionV1::DropAuthZoneProofs => Self::DropAuthZoneProofs,
            InstructionV1::CreateProofFromAuthZoneOfAll(
                CreateProofFromAuthZoneOfAll { resource_address },
            ) => Self::CreateProofFromAuthZoneOfAll {
                resource_address: SerializableNodeId::new(
                    resource_address.into_node_id(),
                    network_id,
                ),
            },
            InstructionV1::CreateProofFromAuthZoneOfAmount(
                CreateProofFromAuthZoneOfAmount {
                    amount,
                    resource_address,
                },
            ) => Self::CreateProofFromAuthZoneOfAmount {
                resource_address: SerializableNodeId::new(
                    resource_address.into_node_id(),
                    network_id,
                ),
                amount: SerializableDecimal::from(*amount),
            },
            InstructionV1::CreateProofFromAuthZoneOfNonFungibles(
                CreateProofFromAuthZoneOfNonFungibles {
                    ids,
                    resource_address,
                },
            ) => Self::CreateProofFromAuthZoneOfNonFungibles {
                resource_address: SerializableNodeId::new(
                    resource_address.into_node_id(),
                    network_id,
                ),
                ids: ids
                    .iter()
                    .map(|local_id| {
                        SerializableNonFungibleLocalId::from(local_id.clone())
                    })
                    .collect(),
            },
            InstructionV1::DropNamedProofs => Self::DropNamedProofs,
            InstructionV1::DropAuthZoneRegularProofs => {
                Self::DropAuthZoneRegularProofs
            }
            InstructionV1::DropAuthZoneSignatureProofs => {
                Self::DropAuthZoneSignatureProofs
            }
            InstructionV1::CreateProofFromBucketOfAll(
                CreateProofFromBucketOfAll { bucket_id },
            ) => Self::CreateProofFromBucketOfAll {
                bucket_id: SerializableU32::from(bucket_id.0),
            },
            InstructionV1::CreateProofFromBucketOfAmount(
                CreateProofFromBucketOfAmount { amount, bucket_id },
            ) => Self::CreateProofFromBucketOfAmount {
                bucket_id: SerializableU32::from(bucket_id.0),
                amount: SerializableDecimal::from(*amount),
            },
            InstructionV1::CreateProofFromBucketOfNonFungibles(
                CreateProofFromBucketOfNonFungibles { ids, bucket_id },
            ) => Self::CreateProofFromBucketOfNonFungibles {
                bucket_id: SerializableU32::from(bucket_id.0),
                ids: ids
                    .iter()
                    .map(|local_id| {
                        SerializableNonFungibleLocalId::from(local_id.clone())
                    })
                    .collect(),
            },
            InstructionV1::BurnResource(BurnResource { bucket_id }) => {
                Self::BurnResource {
                    bucket_id: SerializableU32::from(bucket_id.0),
                }
            }
            InstructionV1::CloneProof(CloneProof { proof_id }) => {
                Self::CloneProof {
                    proof_id: SerializableU32::from(proof_id.0),
                }
            }
            InstructionV1::DropProof(DropProof { proof_id }) => {
                Self::DropProof {
                    proof_id: SerializableU32::from(proof_id.0),
                }
            }
            InstructionV1::DropAllProofs => Self::DropAllProofs,
            InstructionV1::CallFunction(CallFunction {
                package_address,
                blueprint_name,
                function_name,
                args,
            }) => Self::CallFunction {
                package_address: match package_address {
                    DynamicPackageAddress::Named(named) => {
                        SerializableManifestAddress::Named(
                            SerializableU32::from(*named),
                        )
                    }
                    DynamicPackageAddress::Static(global_address) => {
                        SerializableManifestAddress::Static(
                            SerializableNodeId::new(
                                global_address.into_node_id(),
                                network_id,
                            ),
                        )
                    }
                },
                blueprint_name: blueprint_name.to_string(),
                function_name: function_name.to_string(),
                args: SerializableManifestValue::from_typed(args, network_id)?,
            },
            InstructionV1::CallMethod(CallMethod {
                address,
                method_name,
                args,
            }) => Self::CallMethod {
                address: match address {
                    DynamicGlobalAddress::Named(named) => {
                        SerializableManifestAddress::Named(
                            SerializableU32::from(*named),
                        )
                    }
                    DynamicGlobalAddress::Static(global_address) => {
                        SerializableManifestAddress::Static(
                            SerializableNodeId::new(
                                global_address.into_node_id(),
                                network_id,
                            ),
                        )
                    }
                },
                method_name: method_name.clone(),
                args: SerializableManifestValue::from_typed(args, network_id)?,
            },
            InstructionV1::CallRoyaltyMethod(CallRoyaltyMethod {
                address,
                method_name,
                args,
            }) => Self::CallRoyaltyMethod {
                address: match address {
                    DynamicGlobalAddress::Named(named) => {
                        SerializableManifestAddress::Named(
                            SerializableU32::from(*named),
                        )
                    }
                    DynamicGlobalAddress::Static(global_address) => {
                        SerializableManifestAddress::Static(
                            SerializableNodeId::new(
                                global_address.into_node_id(),
                                network_id,
                            ),
                        )
                    }
                },
                method_name: method_name.clone(),
                args: SerializableManifestValue::from_typed(args, network_id)?,
            },
            InstructionV1::CallMetadataMethod(CallMetadataMethod {
                address,
                method_name,
                args,
            }) => Self::CallMetadataMethod {
                address: match address {
                    DynamicGlobalAddress::Named(named) => {
                        SerializableManifestAddress::Named(
                            SerializableU32::from(*named),
                        )
                    }
                    DynamicGlobalAddress::Static(global_address) => {
                        SerializableManifestAddress::Static(
                            SerializableNodeId::new(
                                global_address.into_node_id(),
                                network_id,
                            ),
                        )
                    }
                },
                method_name: method_name.clone(),
                args: SerializableManifestValue::from_typed(args, network_id)?,
            },
            InstructionV1::CallRoleAssignmentMethod(
                CallRoleAssignmentMethod {
                    address,
                    method_name,
                    args,
                },
            ) => Self::CallRoleAssignmentMethod {
                address: match address {
                    DynamicGlobalAddress::Named(named) => {
                        SerializableManifestAddress::Named(
                            SerializableU32::from(*named),
                        )
                    }
                    DynamicGlobalAddress::Static(global_address) => {
                        SerializableManifestAddress::Static(
                            SerializableNodeId::new(
                                global_address.into_node_id(),
                                network_id,
                            ),
                        )
                    }
                },
                method_name: method_name.clone(),
                args: SerializableManifestValue::from_typed(args, network_id)?,
            },
            InstructionV1::CallDirectVaultMethod(CallDirectVaultMethod {
                address,
                method_name,
                args,
            }) => Self::CallDirectVaultMethod {
                address: SerializableNodeId::new(
                    address.into_node_id(),
                    network_id,
                ),
                method_name: method_name.clone(),
                args: SerializableManifestValue::from_typed(args, network_id)?,
            },
            InstructionV1::AllocateGlobalAddress(AllocateGlobalAddress {
                package_address,
                blueprint_name,
            }) => Self::AllocateGlobalAddress {
                package_address: SerializableNodeId::new(
                    package_address.into_node_id(),
                    network_id,
                ),
                blueprint_name: blueprint_name.clone(),
            },
        };
        Ok(instruction)
    }

    pub fn to_instruction(
        &self,
    ) -> Result<InstructionV1, InstructionConversionError> {
        let instruction = match self {
            Self::TakeFromWorktop {
                resource_address,
                amount,
                ..
            } => InstructionV1::TakeFromWorktop {
                resource_address: (*resource_address).try_into()?,
                amount: *amount.deref(),
            },
            Self::TakeNonFungiblesFromWorktop {
                ids,
                resource_address,
                ..
            } => InstructionV1::TakeNonFungiblesFromWorktop {
                resource_address: (*resource_address).try_into()?,
                ids: ids.iter().map(|id| id.deref().clone()).collect(),
            },
            Self::TakeAllFromWorktop {
                resource_address, ..
            } => InstructionV1::TakeAllFromWorktop {
                resource_address: (*resource_address).try_into()?,
            },
            Self::ReturnToWorktop { bucket_id } => {
                InstructionV1::ReturnToWorktop {
                    bucket_id: ManifestBucket(**bucket_id),
                }
            }
            Self::AssertWorktopContains {
                resource_address,
                amount,
            } => InstructionV1::AssertWorktopContains {
                resource_address: (*resource_address).try_into()?,
                amount: *amount.deref(),
            },
            Self::AssertWorktopContainsAny { resource_address } => {
                InstructionV1::AssertWorktopContainsAny {
                    resource_address: (*resource_address).try_into()?,
                }
            }
            Self::AssertWorktopContainsNonFungibles {
                resource_address,
                ids,
            } => InstructionV1::AssertWorktopContainsNonFungibles {
                resource_address: (*resource_address).try_into()?,
                ids: ids.iter().map(|id| id.deref().clone()).collect(),
            },
            Self::PopFromAuthZone { .. } => InstructionV1::PopFromAuthZone {},
            Self::PushToAuthZone { proof_id } => {
                InstructionV1::PushToAuthZone {
                    proof_id: ManifestProof(**proof_id),
                }
            }
            Self::DropNamedProofs => InstructionV1::DropNamedProofs,
            Self::DropAuthZoneProofs => InstructionV1::DropAuthZoneProofs,
            Self::DropAuthZoneRegularProofs => {
                InstructionV1::DropAuthZoneRegularProofs { .. }
            }
            Self::DropAuthZoneSignatureProofs => {
                InstructionV1::DropAuthZoneSignatureProofs { .. }
            }
            Self::CreateProofFromAuthZoneOfAll {
                resource_address, ..
            } => InstructionV1::CreateProofFromAuthZoneOfAll {
                resource_address: (*resource_address).try_into()?,
            },
            Self::CreateProofFromAuthZoneOfAmount {
                resource_address,
                amount,
                ..
            } => InstructionV1::CreateProofFromAuthZoneOfAmount {
                resource_address: (*resource_address).try_into()?,
                amount: *amount.deref(),
            },
            Self::CreateProofFromAuthZoneOfNonFungibles {
                resource_address,
                ids,
                ..
            } => InstructionV1::CreateProofFromAuthZoneOfNonFungibles {
                resource_address: (*resource_address).try_into()?,
                ids: ids.iter().map(|id| id.deref().clone()).collect(),
            },
            Self::CreateProofFromBucketOfAll { bucket_id, .. } => {
                InstructionV1::CreateProofFromBucketOfAll {
                    bucket_id: ManifestBucket(**bucket_id),
                }
            }
            Self::CreateProofFromBucketOfAmount {
                bucket_id, amount, ..
            } => InstructionV1::CreateProofFromBucketOfAmount {
                bucket_id: ManifestBucket(**bucket_id),
                amount: *amount.deref(),
            },
            Self::CreateProofFromBucketOfNonFungibles {
                bucket_id,
                ids,
                ..
            } => InstructionV1::CreateProofFromBucketOfNonFungibles {
                bucket_id: ManifestBucket(**bucket_id),
                ids: ids.iter().map(|id| id.deref().clone()).collect(),
            },
            Self::BurnResource { bucket_id } => InstructionV1::BurnResource {
                bucket_id: ManifestBucket(**bucket_id),
            },
            Self::CloneProof { proof_id, .. } => InstructionV1::CloneProof {
                proof_id: ManifestProof(**proof_id),
            },
            Self::DropProof { proof_id, .. } => InstructionV1::DropProof {
                proof_id: ManifestProof(**proof_id),
            },
            Self::DropAllProofs {} => InstructionV1::DropAllProofs {},
            Self::CallFunction {
                package_address,
                blueprint_name,
                function_name,
                args,
            } => InstructionV1::CallFunction {
                package_address: match package_address {
                    SerializableManifestAddress::Named(named) => {
                        DynamicPackageAddress::Named(**named)
                    }
                    SerializableManifestAddress::Static(address) => {
                        DynamicPackageAddress::Static((*address).try_into()?)
                    }
                },
                blueprint_name: blueprint_name.to_owned(),
                function_name: function_name.to_owned(),
                args: args.to_typed()?,
            },
            Self::CallMethod {
                address,
                method_name,
                args,
            } => InstructionV1::CallMethod {
                address: match address {
                    SerializableManifestAddress::Named(named) => {
                        DynamicGlobalAddress::Named(**named)
                    }
                    SerializableManifestAddress::Static(address) => {
                        DynamicGlobalAddress::Static((*address).try_into()?)
                    }
                },
                method_name: method_name.to_string(),
                args: args.to_typed()?,
            },
            Self::CallRoyaltyMethod {
                address,
                method_name,
                args,
            } => InstructionV1::CallRoyaltyMethod {
                address: match address {
                    SerializableManifestAddress::Named(named) => {
                        DynamicGlobalAddress::Named(**named)
                    }
                    SerializableManifestAddress::Static(address) => {
                        DynamicGlobalAddress::Static((*address).try_into()?)
                    }
                },
                method_name: method_name.to_string(),
                args: args.to_typed()?,
            },
            Self::CallMetadataMethod {
                address,
                method_name,
                args,
            } => InstructionV1::CallMetadataMethod {
                address: match address {
                    SerializableManifestAddress::Named(named) => {
                        DynamicGlobalAddress::Named(**named)
                    }
                    SerializableManifestAddress::Static(address) => {
                        DynamicGlobalAddress::Static((*address).try_into()?)
                    }
                },
                method_name: method_name.to_string(),
                args: args.to_typed()?,
            },
            Self::CallRoleAssignmentMethod {
                address,
                method_name,
                args,
            } => InstructionV1::CallRoleAssignmentMethod {
                address: match address {
                    SerializableManifestAddress::Named(named) => {
                        DynamicGlobalAddress::Named(**named)
                    }
                    SerializableManifestAddress::Static(address) => {
                        DynamicGlobalAddress::Static((*address).try_into()?)
                    }
                },
                method_name: method_name.to_string(),
                args: args.to_typed()?,
            },
            Self::CallDirectVaultMethod {
                address,
                method_name,
                args,
            } => InstructionV1::CallDirectVaultMethod {
                address: (*address).try_into()?,
                method_name: method_name.to_string(),
                args: args.to_typed()?,
            },
            Self::AllocateGlobalAddress {
                package_address,
                blueprint_name,
                ..
            } => InstructionV1::AllocateGlobalAddress {
                package_address: (*package_address).try_into()?,
                blueprint_name: blueprint_name.to_owned(),
            },
        };
        Ok(instruction)
    }
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
#[serde(tag = "kind", content = "error")]
pub enum InstructionConversionError {
    ValueConversionError(ValueConversionError),
    NodeIdConversionError(SerializableNodeIdError),
}

impl From<ValueConversionError> for InstructionConversionError {
    fn from(value: ValueConversionError) -> Self {
        Self::ValueConversionError(value)
    }
}

impl From<SerializableNodeIdError> for InstructionConversionError {
    fn from(value: SerializableNodeIdError) -> Self {
        Self::NodeIdConversionError(value)
    }
}

pub fn to_serializable_instructions(
    instructions: &[InstructionV1],
    network_id: u8,
) -> Result<Vec<SerializableInstruction>, LocatedInstructionConversionError> {
    instructions
        .iter()
        .enumerate()
        .map(|(instruction_index, instruction)| {
            SerializableInstruction::from_instruction(instruction, network_id)
                .map_err(|error| LocatedInstructionConversionError {
                    instruction_index,
                    error,
                })
        })
        .collect::<Result<_, _>>()
}

pub fn to_native_instructions(
    instructions: &[SerializableInstruction],
) -> Result<Vec<InstructionV1>, LocatedInstructionConversionError> {
    instructions
        .iter()
        .enumerate()
        .map(|(instruction_index, instruction)| {
            instruction.to_instruction().map_err(|error| {
                LocatedInstructionConversionError {
                    instruction_index,
                    error,
                }
            })
        })
        .collect::<Result<_, _>>()
}

#[derive(Debug, Clone)]
pub struct LocatedInstructionConversionError {
    pub instruction_index: usize,
    pub error: InstructionConversionError,
}
