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
pub enum Instruction {
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

    ClearAuthZone,

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

    ClearSignatureProofs,

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

    CallAccessRulesMethod {
        address: ManifestAddress,
        method_name: String,
        args: ManifestValue,
    },

    CallDirectVaultMethod {
        address: Arc<Address>,
        method_name: String,
        args: ManifestValue,
    },

    DropAllProofs,

    AllocateGlobalAddress {
        package_address: Arc<Address>,
        blueprint_name: String,
    },
}

impl Instruction {
    pub fn from_native(native: &NativeInstruction, network_id: u8) -> Self {
        match native {
            NativeInstruction::TakeAllFromWorktop { resource_address } => {
                Self::TakeAllFromWorktop {
                    resource_address: Arc::new(Address::from_node_id(
                        *resource_address,
                        network_id,
                    )),
                }
            }
            NativeInstruction::TakeFromWorktop {
                resource_address,
                amount,
            } => Self::TakeFromWorktop {
                resource_address: Arc::new(Address::from_node_id(*resource_address, network_id)),
                amount: Arc::new(Decimal(*amount)),
            },
            NativeInstruction::TakeNonFungiblesFromWorktop {
                resource_address,
                ids,
            } => Self::TakeNonFungiblesFromWorktop {
                resource_address: Arc::new(Address::from_node_id(*resource_address, network_id)),
                ids: ids.iter().cloned().map(Into::into).collect(),
            },

            NativeInstruction::ReturnToWorktop { bucket_id } => Self::ReturnToWorktop {
                bucket_id: (*bucket_id).into(),
            },

            NativeInstruction::AssertWorktopContains {
                resource_address,
                amount,
            } => Self::AssertWorktopContains {
                resource_address: Arc::new(Address::from_node_id(*resource_address, network_id)),
                amount: Arc::new(Decimal(*amount)),
            },
            NativeInstruction::AssertWorktopContainsAny { resource_address } => {
                Self::AssertWorktopContainsAny {
                    resource_address: Arc::new(Address::from_node_id(
                        *resource_address,
                        network_id,
                    )),
                }
            }
            NativeInstruction::AssertWorktopContainsNonFungibles {
                resource_address,
                ids,
            } => Self::AssertWorktopContainsNonFungibles {
                resource_address: Arc::new(Address::from_node_id(*resource_address, network_id)),
                ids: ids.iter().cloned().map(Into::into).collect(),
            },
            NativeInstruction::PopFromAuthZone => Self::PopFromAuthZone,
            NativeInstruction::PushToAuthZone { proof_id } => Self::PushToAuthZone {
                proof_id: (*proof_id).into(),
            },
            NativeInstruction::ClearAuthZone => Self::ClearAuthZone,
            NativeInstruction::ClearSignatureProofs => Self::ClearSignatureProofs,
            NativeInstruction::CreateProofFromAuthZoneOfAll { resource_address } => {
                Self::CreateProofFromAuthZoneOfAll {
                    resource_address: Arc::new(Address::from_node_id(
                        *resource_address,
                        network_id,
                    )),
                }
            }
            NativeInstruction::CreateProofFromAuthZoneOfAmount {
                resource_address,
                amount,
            } => Self::CreateProofFromAuthZoneOfAmount {
                resource_address: Arc::new(Address::from_node_id(*resource_address, network_id)),
                amount: Arc::new(Decimal(*amount)),
            },
            NativeInstruction::CreateProofFromAuthZoneOfNonFungibles {
                resource_address,
                ids,
            } => Self::CreateProofFromAuthZoneOfNonFungibles {
                resource_address: Arc::new(Address::from_node_id(*resource_address, network_id)),
                ids: ids.iter().cloned().map(Into::into).collect(),
            },
            NativeInstruction::CreateProofFromBucketOfAll { bucket_id } => {
                Self::CreateProofFromBucketOfAll {
                    bucket_id: (*bucket_id).into(),
                }
            }
            NativeInstruction::CreateProofFromBucketOfAmount { bucket_id, amount } => {
                Self::CreateProofFromBucketOfAmount {
                    bucket_id: (*bucket_id).into(),
                    amount: Arc::new(Decimal(*amount)),
                }
            }
            NativeInstruction::CreateProofFromBucketOfNonFungibles { bucket_id, ids } => {
                Self::CreateProofFromBucketOfNonFungibles {
                    bucket_id: (*bucket_id).into(),
                    ids: ids.iter().cloned().map(Into::into).collect(),
                }
            }
            NativeInstruction::BurnResource { bucket_id } => Self::BurnResource {
                bucket_id: (*bucket_id).into(),
            },
            NativeInstruction::CloneProof { proof_id } => Self::CloneProof {
                proof_id: (*proof_id).into(),
            },
            NativeInstruction::DropProof { proof_id } => Self::DropProof {
                proof_id: (*proof_id).into(),
            },
            NativeInstruction::DropAllProofs => Self::DropAllProofs,
            NativeInstruction::AllocateGlobalAddress {
                package_address,
                blueprint_name,
            } => Self::AllocateGlobalAddress {
                package_address: Arc::new(Address::from_node_id(*package_address, network_id)),
                blueprint_name: blueprint_name.clone(),
            },
            NativeInstruction::CallFunction {
                package_address,
                blueprint_name,
                function_name,
                args,
            } => Self::CallFunction {
                package_address: ManifestAddress::from_dynamic_package_address(
                    package_address,
                    network_id,
                ),
                blueprint_name: blueprint_name.to_owned(),
                function_name: function_name.to_owned(),
                args: ManifestValue::from_native(args, network_id),
            },
            NativeInstruction::CallMethod {
                address,
                method_name,
                args,
            } => Self::CallMethod {
                address: ManifestAddress::from_dynamic_global_address(address, network_id),
                method_name: method_name.to_owned(),
                args: ManifestValue::from_native(args, network_id),
            },
            NativeInstruction::CallMetadataMethod {
                address,
                method_name,
                args,
            } => Self::CallMetadataMethod {
                address: ManifestAddress::from_dynamic_global_address(address, network_id),
                method_name: method_name.to_owned(),
                args: ManifestValue::from_native(args, network_id),
            },
            NativeInstruction::CallAccessRulesMethod {
                address,
                method_name,
                args,
            } => Self::CallAccessRulesMethod {
                address: ManifestAddress::from_dynamic_global_address(address, network_id),
                method_name: method_name.to_owned(),
                args: ManifestValue::from_native(args, network_id),
            },
            NativeInstruction::CallRoyaltyMethod {
                address,
                method_name,
                args,
            } => Self::CallRoyaltyMethod {
                address: ManifestAddress::from_dynamic_global_address(address, network_id),
                method_name: method_name.to_owned(),
                args: ManifestValue::from_native(args, network_id),
            },
            NativeInstruction::CallDirectVaultMethod {
                address,
                method_name,
                args,
            } => Self::CallDirectVaultMethod {
                address: Arc::new(Address::from_node_id(*address, network_id)),
                method_name: method_name.to_owned(),
                args: ManifestValue::from_native(args, network_id),
            },
        }
    }

    pub fn to_native(&self) -> Result<NativeInstruction> {
        let value = match self {
            Self::TakeAllFromWorktop { resource_address } => {
                NativeInstruction::TakeAllFromWorktop {
                    resource_address: (*resource_address.as_ref()).try_into()?,
                }
            }
            Self::TakeFromWorktop {
                resource_address,
                amount,
            } => NativeInstruction::TakeFromWorktop {
                resource_address: (*resource_address.as_ref()).try_into()?,
                amount: amount.0,
            },
            Self::TakeNonFungiblesFromWorktop {
                resource_address,
                ids,
            } => NativeInstruction::TakeNonFungiblesFromWorktop {
                resource_address: (*resource_address.as_ref()).try_into()?,
                ids: ids
                    .iter()
                    .cloned()
                    .map(TryInto::try_into)
                    .collect::<Result<_>>()?,
            },
            Self::ReturnToWorktop { bucket_id } => NativeInstruction::ReturnToWorktop {
                bucket_id: (*bucket_id).into(),
            },
            Self::AssertWorktopContains {
                resource_address,
                amount,
            } => NativeInstruction::AssertWorktopContains {
                resource_address: (*resource_address.as_ref()).try_into()?,
                amount: amount.0,
            },
            Self::AssertWorktopContainsAny { resource_address } => {
                NativeInstruction::AssertWorktopContainsAny {
                    resource_address: (*resource_address.as_ref()).try_into()?,
                }
            }
            Self::AssertWorktopContainsNonFungibles {
                resource_address,
                ids,
            } => NativeInstruction::AssertWorktopContainsNonFungibles {
                resource_address: (*resource_address.as_ref()).try_into()?,
                ids: ids
                    .iter()
                    .cloned()
                    .map(TryInto::try_into)
                    .collect::<Result<_>>()?,
            },
            Self::PopFromAuthZone => NativeInstruction::PopFromAuthZone,
            Self::PushToAuthZone { proof_id } => NativeInstruction::PushToAuthZone {
                proof_id: (*proof_id).into(),
            },
            Self::ClearAuthZone => NativeInstruction::ClearAuthZone,
            Self::CreateProofFromAuthZoneOfAll { resource_address } => {
                NativeInstruction::CreateProofFromAuthZoneOfAll {
                    resource_address: (*resource_address.as_ref()).try_into()?,
                }
            }
            Self::CreateProofFromAuthZoneOfAmount {
                resource_address,
                amount,
            } => NativeInstruction::CreateProofFromAuthZoneOfAmount {
                resource_address: (*resource_address.as_ref()).try_into()?,
                amount: amount.0,
            },
            Self::CreateProofFromAuthZoneOfNonFungibles {
                resource_address,
                ids,
            } => NativeInstruction::CreateProofFromAuthZoneOfNonFungibles {
                resource_address: (*resource_address.as_ref()).try_into()?,
                ids: ids
                    .iter()
                    .cloned()
                    .map(TryInto::try_into)
                    .collect::<Result<_>>()?,
            },
            Self::ClearSignatureProofs => NativeInstruction::ClearSignatureProofs,
            Self::CreateProofFromBucketOfAll { bucket_id } => {
                NativeInstruction::CreateProofFromBucketOfAll {
                    bucket_id: (*bucket_id).into(),
                }
            }
            Self::CreateProofFromBucketOfAmount { bucket_id, amount } => {
                NativeInstruction::CreateProofFromBucketOfAmount {
                    bucket_id: (*bucket_id).into(),
                    amount: amount.0,
                }
            }
            Self::CreateProofFromBucketOfNonFungibles { bucket_id, ids } => {
                NativeInstruction::CreateProofFromBucketOfNonFungibles {
                    bucket_id: (*bucket_id).into(),
                    ids: ids
                        .iter()
                        .cloned()
                        .map(TryInto::try_into)
                        .collect::<Result<_>>()?,
                }
            }
            Self::BurnResource { bucket_id } => NativeInstruction::BurnResource {
                bucket_id: (*bucket_id).into(),
            },
            Self::CloneProof { proof_id } => NativeInstruction::CloneProof {
                proof_id: (*proof_id).into(),
            },
            Self::DropProof { proof_id } => NativeInstruction::DropProof {
                proof_id: (*proof_id).into(),
            },
            Self::DropAllProofs => NativeInstruction::DropAllProofs,
            Self::CallFunction {
                package_address,
                blueprint_name,
                function_name,
                args,
            } => NativeInstruction::CallFunction {
                package_address: package_address.clone().try_into()?,
                blueprint_name: blueprint_name.to_string(),
                function_name: function_name.to_string(),
                args: args.to_native()?,
            },
            Self::CallMethod {
                address,
                method_name,
                args,
            } => NativeInstruction::CallMethod {
                address: address.clone().try_into()?,
                method_name: method_name.to_owned(),
                args: args.to_native()?,
            },
            Self::CallMetadataMethod {
                address,
                method_name,
                args,
            } => NativeInstruction::CallMetadataMethod {
                address: address.clone().try_into()?,
                method_name: method_name.to_owned(),
                args: args.to_native()?,
            },
            Self::CallAccessRulesMethod {
                address,
                method_name,
                args,
            } => NativeInstruction::CallAccessRulesMethod {
                address: address.clone().try_into()?,
                method_name: method_name.to_owned(),
                args: args.to_native()?,
            },
            Self::CallRoyaltyMethod {
                address,
                method_name,
                args,
            } => NativeInstruction::CallRoyaltyMethod {
                address: address.clone().try_into()?,
                method_name: method_name.to_owned(),
                args: args.to_native()?,
            },
            Self::CallDirectVaultMethod {
                address,
                method_name,
                args,
            } => NativeInstruction::CallDirectVaultMethod {
                address: address.as_ref().0 .0.try_into()?,
                method_name: method_name.to_owned(),
                args: args.to_native()?,
            },
            Self::AllocateGlobalAddress {
                package_address,
                blueprint_name,
            } => NativeInstruction::AllocateGlobalAddress {
                package_address: package_address.as_ref().0 .0.try_into()?,
                blueprint_name: blueprint_name.to_string(),
            },
        };
        Ok(value)
    }
}
