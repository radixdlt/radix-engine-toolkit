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

use radix_engine::types::*;
use schemars::*;
use serde::*;
use transaction::prelude::*;
use transaction::validation::*;

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(tag = "kind")]
pub enum SerializableInstruction {
    TakeAllFromWorktop {
        resource_address: SerializableManifestValue,
        new_bucket: SerializableManifestValue,
    },

    TakeFromWorktop {
        resource_address: SerializableManifestValue,
        amount: SerializableManifestValue,
        new_bucket: SerializableManifestValue,
    },

    TakeNonFungiblesFromWorktop {
        resource_address: SerializableManifestValue,
        ids: SerializableManifestValue,
        new_bucket: SerializableManifestValue,
    },

    ReturnToWorktop {
        bucket_id: SerializableManifestValue,
    },

    AssertWorktopContainsAny {
        resource_address: SerializableManifestValue,
    },

    AssertWorktopContains {
        resource_address: SerializableManifestValue,
        amount: SerializableManifestValue,
    },

    AssertWorktopContainsNonFungibles {
        resource_address: SerializableManifestValue,
        ids: SerializableManifestValue,
    },

    PopFromAuthZone {
        new_proof: SerializableManifestValue,
    },

    PushToAuthZone {
        proof_id: SerializableManifestValue,
    },

    ClearAuthZone,

    CreateProofFromAuthZoneOfAmount {
        resource_address: SerializableManifestValue,
        amount: SerializableManifestValue,
        new_proof: SerializableManifestValue,
    },

    CreateProofFromAuthZoneOfNonFungibles {
        resource_address: SerializableManifestValue,
        ids: SerializableManifestValue,
        new_proof: SerializableManifestValue,
    },

    CreateProofFromAuthZoneOfAll {
        resource_address: SerializableManifestValue,
        new_proof: SerializableManifestValue,
    },

    ClearSignatureProofs,

    CreateProofFromBucketOfAmount {
        bucket_id: SerializableManifestValue,
        amount: SerializableManifestValue,
        new_proof: SerializableManifestValue,
    },

    CreateProofFromBucketOfNonFungibles {
        bucket_id: SerializableManifestValue,
        ids: SerializableManifestValue,
        new_proof: SerializableManifestValue,
    },

    CreateProofFromBucketOfAll {
        bucket_id: SerializableManifestValue,
        new_proof: SerializableManifestValue,
    },

    BurnResource {
        bucket_id: SerializableManifestValue,
    },

    CloneProof {
        proof_id: SerializableManifestValue,
        new_proof: SerializableManifestValue,
    },

    DropProof {
        proof_id: SerializableManifestValue,
    },

    CallFunction {
        package_address: SerializableManifestValue,
        blueprint_name: SerializableManifestValue,
        function_name: SerializableManifestValue,
        args: SerializableManifestValue,
    },

    CallMethod {
        address: SerializableManifestValue,
        method_name: SerializableManifestValue,
        args: SerializableManifestValue,
    },

    CallRoyaltyMethod {
        address: SerializableManifestValue,
        method_name: SerializableManifestValue,
        args: SerializableManifestValue,
    },

    CallMetadataMethod {
        address: SerializableManifestValue,
        method_name: SerializableManifestValue,
        args: SerializableManifestValue,
    },

    CallAccessRulesMethod {
        address: SerializableManifestValue,
        method_name: SerializableManifestValue,
        args: SerializableManifestValue,
    },

    CallDirectVaultMethod {
        address: SerializableManifestValue,
        method_name: SerializableManifestValue,
        args: SerializableManifestValue,
    },

    DropAllProofs,

    AllocateGlobalAddress {
        package_address: SerializableManifestValue,
        blueprint_name: SerializableManifestValue,
        address_reservation: SerializableManifestValue,
        named_address: SerializableManifestValue,
    },
}

impl SerializableInstruction {
    pub fn from_instruction(
        instruction: &InstructionV1,
        network_id: u8,
        id_allocator: &mut ManifestIdAllocator,
    ) -> Result<Self, InstructionConversionError> {
        let instruction = match instruction {
            InstructionV1::TakeFromWorktop {
                resource_address,
                amount,
            } => {
                let bucket = id_allocator.new_bucket_id();

                Self::TakeFromWorktop {
                    resource_address: SerializableManifestValue::from_typed(
                        resource_address,
                        network_id,
                    )?,
                    amount: SerializableManifestValue::from_typed(amount, network_id)?,
                    new_bucket: SerializableManifestValue::from_typed(&bucket, network_id)?,
                }
            }
            InstructionV1::TakeNonFungiblesFromWorktop {
                resource_address,
                ids,
            } => {
                let bucket = id_allocator.new_bucket_id();

                Self::TakeNonFungiblesFromWorktop {
                    ids: SerializableManifestValue::from_typed(ids, network_id)?,
                    resource_address: SerializableManifestValue::from_typed(
                        resource_address,
                        network_id,
                    )?,
                    new_bucket: SerializableManifestValue::from_typed(&bucket, network_id)?,
                }
            }
            InstructionV1::TakeAllFromWorktop { resource_address } => {
                let bucket = id_allocator.new_bucket_id();

                Self::TakeAllFromWorktop {
                    resource_address: SerializableManifestValue::from_typed(
                        resource_address,
                        network_id,
                    )?,
                    new_bucket: SerializableManifestValue::from_typed(&bucket, network_id)?,
                }
            }
            InstructionV1::ReturnToWorktop { bucket_id } => Self::ReturnToWorktop {
                bucket_id: SerializableManifestValue::from_typed(&bucket_id, network_id)?,
            },
            InstructionV1::AssertWorktopContains {
                resource_address,
                amount,
            } => Self::AssertWorktopContains {
                resource_address: SerializableManifestValue::from_typed(
                    resource_address,
                    network_id,
                )?,
                amount: SerializableManifestValue::from_typed(amount, network_id)?,
            },
            InstructionV1::AssertWorktopContainsAny { resource_address } => {
                Self::AssertWorktopContainsAny {
                    resource_address: SerializableManifestValue::from_typed(
                        resource_address,
                        network_id,
                    )?,
                }
            }
            InstructionV1::AssertWorktopContainsNonFungibles {
                resource_address,
                ids,
            } => Self::AssertWorktopContainsNonFungibles {
                resource_address: SerializableManifestValue::from_typed(
                    resource_address,
                    network_id,
                )?,
                ids: SerializableManifestValue::from_typed(ids, network_id)?,
            },
            InstructionV1::PopFromAuthZone => {
                let proof = id_allocator.new_proof_id();

                Self::PopFromAuthZone {
                    new_proof: SerializableManifestValue::from_typed(&proof, network_id)?,
                }
            }
            InstructionV1::PushToAuthZone { proof_id } => Self::PushToAuthZone {
                proof_id: SerializableManifestValue::from_typed(&proof_id, network_id)?,
            },
            InstructionV1::ClearAuthZone => Self::ClearAuthZone,
            InstructionV1::CreateProofFromAuthZoneOfAll { resource_address } => {
                let proof = id_allocator.new_proof_id();

                Self::CreateProofFromAuthZoneOfAll {
                    resource_address: SerializableManifestValue::from_typed(
                        resource_address,
                        network_id,
                    )?,
                    new_proof: SerializableManifestValue::from_typed(&proof, network_id)?,
                }
            }
            InstructionV1::CreateProofFromAuthZoneOfAmount {
                amount,
                resource_address,
            } => {
                let proof = id_allocator.new_proof_id();

                Self::CreateProofFromAuthZoneOfAmount {
                    resource_address: SerializableManifestValue::from_typed(
                        resource_address,
                        network_id,
                    )?,
                    amount: SerializableManifestValue::from_typed(amount, network_id)?,
                    new_proof: SerializableManifestValue::from_typed(&proof, network_id)?,
                }
            }
            InstructionV1::CreateProofFromAuthZoneOfNonFungibles {
                ids,
                resource_address,
            } => {
                let proof = id_allocator.new_proof_id();

                Self::CreateProofFromAuthZoneOfNonFungibles {
                    resource_address: SerializableManifestValue::from_typed(
                        resource_address,
                        network_id,
                    )?,
                    ids: SerializableManifestValue::from_typed(ids, network_id)?,
                    new_proof: SerializableManifestValue::from_typed(&proof, network_id)?,
                }
            }
            InstructionV1::ClearSignatureProofs => Self::ClearSignatureProofs,
            InstructionV1::CreateProofFromBucketOfAll { bucket_id } => {
                let proof = id_allocator.new_proof_id();

                Self::CreateProofFromBucketOfAll {
                    bucket_id: SerializableManifestValue::from_typed(bucket_id, network_id)?,
                    new_proof: SerializableManifestValue::from_typed(&proof, network_id)?,
                }
            }
            InstructionV1::CreateProofFromBucketOfAmount { amount, bucket_id } => {
                let proof = id_allocator.new_proof_id();

                Self::CreateProofFromBucketOfAmount {
                    bucket_id: SerializableManifestValue::from_typed(bucket_id, network_id)?,
                    amount: SerializableManifestValue::from_typed(amount, network_id)?,
                    new_proof: SerializableManifestValue::from_typed(&proof, network_id)?,
                }
            }
            InstructionV1::CreateProofFromBucketOfNonFungibles { ids, bucket_id } => {
                let proof = id_allocator.new_proof_id();

                Self::CreateProofFromBucketOfNonFungibles {
                    bucket_id: SerializableManifestValue::from_typed(bucket_id, network_id)?,
                    ids: SerializableManifestValue::from_typed(ids, network_id)?,
                    new_proof: SerializableManifestValue::from_typed(&proof, network_id)?,
                }
            }
            InstructionV1::BurnResource { bucket_id } => Self::BurnResource {
                bucket_id: SerializableManifestValue::from_typed(bucket_id, network_id)?,
            },
            InstructionV1::CloneProof { proof_id } => {
                let proof = id_allocator.new_proof_id();

                Self::CloneProof {
                    proof_id: SerializableManifestValue::from_typed(proof_id, network_id)?,
                    new_proof: SerializableManifestValue::from_typed(&proof, network_id)?,
                }
            }
            InstructionV1::DropProof { proof_id } => Self::DropProof {
                proof_id: SerializableManifestValue::from_typed(proof_id, network_id)?,
            },
            InstructionV1::DropAllProofs => Self::DropAllProofs,
            InstructionV1::CallFunction {
                package_address,
                blueprint_name,
                function_name,
                args,
            } => Self::CallFunction {
                package_address: SerializableManifestValue::from_typed(
                    package_address,
                    network_id,
                )?,
                blueprint_name: SerializableManifestValue::from_typed(blueprint_name, network_id)?,
                function_name: SerializableManifestValue::from_typed(function_name, network_id)?,
                args: SerializableManifestValue::from_typed(args, network_id)?,
            },
            InstructionV1::CallMethod {
                address,
                method_name,
                args,
            } => Self::CallMethod {
                address: SerializableManifestValue::from_typed(address, network_id)?,
                method_name: SerializableManifestValue::from_typed(method_name, network_id)?,
                args: SerializableManifestValue::from_typed(args, network_id)?,
            },
            InstructionV1::CallRoyaltyMethod {
                address,
                method_name,
                args,
            } => Self::CallRoyaltyMethod {
                address: SerializableManifestValue::from_typed(address, network_id)?,
                method_name: SerializableManifestValue::from_typed(method_name, network_id)?,
                args: SerializableManifestValue::from_typed(args, network_id)?,
            },
            InstructionV1::CallMetadataMethod {
                address,
                method_name,
                args,
            } => Self::CallMetadataMethod {
                address: SerializableManifestValue::from_typed(address, network_id)?,
                method_name: SerializableManifestValue::from_typed(method_name, network_id)?,
                args: SerializableManifestValue::from_typed(args, network_id)?,
            },
            InstructionV1::CallAccessRulesMethod {
                address,
                method_name,
                args,
            } => Self::CallAccessRulesMethod {
                address: SerializableManifestValue::from_typed(address, network_id)?,
                method_name: SerializableManifestValue::from_typed(method_name, network_id)?,
                args: SerializableManifestValue::from_typed(args, network_id)?,
            },
            InstructionV1::CallDirectVaultMethod {
                address,
                method_name,
                args,
            } => Self::CallDirectVaultMethod {
                address: SerializableManifestValue::from_typed(address, network_id)?,
                method_name: SerializableManifestValue::from_typed(method_name, network_id)?,
                args: SerializableManifestValue::from_typed(args, network_id)?,
            },
            InstructionV1::AllocateGlobalAddress {
                package_address,
                blueprint_name,
            } => {
                let address_reservation = id_allocator.new_address_reservation_id();
                let named_address = id_allocator.new_address_id();

                Self::AllocateGlobalAddress {
                    package_address: SerializableManifestValue::from_typed(
                        package_address,
                        network_id,
                    )?,
                    blueprint_name: SerializableManifestValue::from_typed(
                        blueprint_name,
                        network_id,
                    )?,
                    address_reservation: SerializableManifestValue::from_typed(
                        &address_reservation,
                        network_id,
                    )?,
                    named_address: SerializableManifestValue::from_typed(
                        &named_address,
                        network_id,
                    )?,
                }
            }
        };
        Ok(instruction)
    }

    pub fn to_instruction(&self) -> Result<InstructionV1, InstructionConversionError> {
        let instruction = match self {
            Self::TakeFromWorktop {
                resource_address,
                amount,
                ..
            } => InstructionV1::TakeFromWorktop {
                resource_address: resource_address.to_typed()?,
                amount: amount.to_typed()?,
            },
            Self::TakeNonFungiblesFromWorktop {
                ids,
                resource_address,
                ..
            } => InstructionV1::TakeNonFungiblesFromWorktop {
                resource_address: resource_address.to_typed()?,
                ids: ids.to_typed()?,
            },
            Self::TakeAllFromWorktop {
                resource_address, ..
            } => InstructionV1::TakeAllFromWorktop {
                resource_address: resource_address.to_typed()?,
            },
            Self::ReturnToWorktop { bucket_id } => InstructionV1::ReturnToWorktop {
                bucket_id: bucket_id.to_typed()?,
            },
            Self::AssertWorktopContains {
                resource_address,
                amount,
            } => InstructionV1::AssertWorktopContains {
                resource_address: resource_address.to_typed()?,
                amount: amount.to_typed()?,
            },
            Self::AssertWorktopContainsAny { resource_address } => {
                InstructionV1::AssertWorktopContainsAny {
                    resource_address: resource_address.to_typed()?,
                }
            }
            Self::AssertWorktopContainsNonFungibles {
                resource_address,
                ids,
            } => InstructionV1::AssertWorktopContainsNonFungibles {
                resource_address: resource_address.to_typed()?,
                ids: ids.to_typed()?,
            },
            Self::PopFromAuthZone { .. } => InstructionV1::PopFromAuthZone {},
            Self::PushToAuthZone { proof_id } => InstructionV1::PushToAuthZone {
                proof_id: proof_id.to_typed()?,
            },
            Self::ClearAuthZone => InstructionV1::ClearAuthZone,
            Self::ClearSignatureProofs => InstructionV1::ClearSignatureProofs,
            Self::CreateProofFromAuthZoneOfAll {
                resource_address, ..
            } => InstructionV1::CreateProofFromAuthZoneOfAll {
                resource_address: resource_address.to_typed()?,
            },
            Self::CreateProofFromAuthZoneOfAmount {
                resource_address,
                amount,
                ..
            } => InstructionV1::CreateProofFromAuthZoneOfAmount {
                resource_address: resource_address.to_typed()?,
                amount: amount.to_typed()?,
            },
            Self::CreateProofFromAuthZoneOfNonFungibles {
                resource_address,
                ids,
                ..
            } => InstructionV1::CreateProofFromAuthZoneOfNonFungibles {
                resource_address: resource_address.to_typed()?,
                ids: ids.to_typed()?,
            },
            Self::CreateProofFromBucketOfAll { bucket_id, .. } => {
                InstructionV1::CreateProofFromBucketOfAll {
                    bucket_id: bucket_id.to_typed()?,
                }
            }
            Self::CreateProofFromBucketOfAmount {
                bucket_id, amount, ..
            } => InstructionV1::CreateProofFromBucketOfAmount {
                bucket_id: bucket_id.to_typed()?,
                amount: amount.to_typed()?,
            },
            Self::CreateProofFromBucketOfNonFungibles { bucket_id, ids, .. } => {
                InstructionV1::CreateProofFromBucketOfNonFungibles {
                    bucket_id: bucket_id.to_typed()?,
                    ids: ids.to_typed()?,
                }
            }
            Self::BurnResource { bucket_id } => InstructionV1::BurnResource {
                bucket_id: bucket_id.to_typed()?,
            },
            Self::CloneProof { proof_id, .. } => InstructionV1::CloneProof {
                proof_id: proof_id.to_typed()?,
            },
            Self::DropProof { proof_id, .. } => InstructionV1::DropProof {
                proof_id: proof_id.to_typed()?,
            },
            Self::DropAllProofs {} => InstructionV1::DropAllProofs {},
            Self::CallFunction {
                package_address,
                blueprint_name,
                function_name,
                args,
            } => InstructionV1::CallFunction {
                package_address: package_address.to_typed()?,
                blueprint_name: blueprint_name.to_typed()?,
                function_name: function_name.to_typed()?,
                args: args.to_typed()?,
            },
            Self::CallMethod {
                address,
                method_name,
                args,
            } => InstructionV1::CallMethod {
                address: address.to_typed()?,
                method_name: method_name.to_typed()?,
                args: args.to_typed()?,
            },
            Self::CallRoyaltyMethod {
                address,
                method_name,
                args,
            } => InstructionV1::CallRoyaltyMethod {
                address: address.to_typed()?,
                method_name: method_name.to_typed()?,
                args: args.to_typed()?,
            },
            Self::CallMetadataMethod {
                address,
                method_name,
                args,
            } => InstructionV1::CallMetadataMethod {
                address: address.to_typed()?,
                method_name: method_name.to_typed()?,
                args: args.to_typed()?,
            },
            Self::CallAccessRulesMethod {
                address,
                method_name,
                args,
            } => InstructionV1::CallAccessRulesMethod {
                address: address.to_typed()?,
                method_name: method_name.to_typed()?,
                args: args.to_typed()?,
            },
            Self::CallDirectVaultMethod {
                address,
                method_name,
                args,
            } => InstructionV1::CallDirectVaultMethod {
                address: address.to_typed()?,
                method_name: method_name.to_typed()?,
                args: args.to_typed()?,
            },
            Self::AllocateGlobalAddress {
                package_address,
                blueprint_name,
                ..
            } => InstructionV1::AllocateGlobalAddress {
                package_address: package_address.to_typed()?,
                blueprint_name: blueprint_name.to_typed()?,
            },
        };
        Ok(instruction)
    }
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
#[serde(tag = "kind", content = "error")]
pub enum InstructionConversionError {
    ValueConversionError(ValueConversionError),
}

impl From<ValueConversionError> for InstructionConversionError {
    fn from(value: ValueConversionError) -> Self {
        Self::ValueConversionError(value)
    }
}

pub fn to_serializable_instructions(
    instructions: &[InstructionV1],
    network_id: u8,
) -> Result<Vec<SerializableInstruction>, LocatedInstructionConversionError> {
    let mut id_allocator = ManifestIdAllocator::default();

    instructions
        .iter()
        .enumerate()
        .map(|(instruction_index, instruction)| {
            SerializableInstruction::from_instruction(instruction, network_id, &mut id_allocator)
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
            instruction
                .to_instruction()
                .map_err(|error| LocatedInstructionConversionError {
                    instruction_index,
                    error,
                })
        })
        .collect::<Result<_, _>>()
}

#[derive(Debug, Clone)]
pub struct LocatedInstructionConversionError {
    pub instruction_index: usize,
    pub error: InstructionConversionError,
}
