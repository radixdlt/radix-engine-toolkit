use radix_transaction::manifest::ast::{
    Instruction as AstInstruction, Receiver as AstReceiver, ScryptoReceiver as AstScryptoReceiver,
    Value as AstValue,
};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::collections::HashSet;

use crate::address::Bech32Manager;
use crate::error::Error;
use crate::models::re_node::*;
use crate::models::value::*;
use crate::models::NetworkAwareComponentAddress;
use crate::traits::ValidateWithContext;

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "instruction", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Instruction {
    CallFunction {
        package_address: Value,
        blueprint_name: Value,
        function_name: Value,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        arguments: Option<Vec<Value>>,
    },
    CallNativeFunction {
        /// An unstructured [`Value`] representing the name of the blueprint to call. This is
        /// expected to be a [`Value::String`] during validation and conversions.
        blueprint_name: Value,

        /// An unstructured [`Value`] representing the name of the function to call. This is
        /// expected to be a [`Value::String`] during validation and conversions.
        function_name: Value,

        /// An optional vector of the arguments used in the function call.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        arguments: Option<Vec<Value>>,
    },

    CallMethod {
        // TODO: With the introduction of the ScryptoReceiver, "component_address" seems like a
        // bad name to use. Something better is needed here.
        /// An unstructured [`Value`] which could be a [`Value::ComponentAddress`] or a
        /// [`Value::Component`]. During conversion, this gets translated into the appropriate
        /// [`AstScryptoReceiver`].
        component_address: Value,
        method_name: Value,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        arguments: Option<Vec<Value>>,
    },
    CallNativeMethod {
        // TODO: With the introduction of the ScryptoReceiver, "component_address" seems like a
        // bad name to use. Something better is needed here.
        /// The reason why the `component_address` on the [`Instruction::CallMethod`] can get
        /// special treatment and have automatic translation between [`AstScryptoReceiver`] and
        /// [`Value`] is because it follows very simple rules that are very easy to check and
        /// understand. If the `component_address` is a [`Value::Component`] then it gets translated
        /// to a [`AstScryptoReceiver::Component`]. If it is a [`Value::ComponentAddress`] then it
        /// gets translated to [`AstScryptoReceiver::Global`].
        ///
        /// On the other hand, with the [`Instruction::CallNativeMethod`] and the [`Receiver`] the
        /// [`Receiver::Owned`] and [`Receiver::Ref`] is disambiguated through an ampersand (`&`) in
        /// text form. Therefore, there is a need to introduce an additional type of [`Receiver`] in
        /// this library.
        receiver: RENode,

        method_name: Value,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        arguments: Option<Vec<Value>>,
    },

    TakeFromWorktop {
        resource_address: Value,
        into_bucket: Value,
    },
    TakeFromWorktopByAmount {
        amount: Value,
        resource_address: Value,
        into_bucket: Value,
    },
    TakeFromWorktopByIds {
        ids: HashSet<Value>,
        resource_address: Value,
        into_bucket: Value,
    },
    ReturnToWorktop {
        bucket: Value,
    },

    AssertWorktopContains {
        resource_address: Value,
    },
    AssertWorktopContainsByAmount {
        amount: Value,
        resource_address: Value,
    },
    AssertWorktopContainsByIds {
        ids: HashSet<Value>,
        resource_address: Value,
    },

    PopFromAuthZone {
        into_proof: Value,
    },
    PushToAuthZone {
        proof: Value,
    },
    ClearAuthZone,

    CreateProofFromAuthZone {
        resource_address: Value,
        into_proof: Value,
    },
    CreateProofFromAuthZoneByAmount {
        amount: Value,
        resource_address: Value,
        into_proof: Value,
    },
    CreateProofFromAuthZoneByIds {
        ids: HashSet<Value>,
        resource_address: Value,
        into_proof: Value,
    },

    CreateProofFromBucket {
        bucket: Value,
        into_proof: Value,
    },

    CloneProof {
        proof: Value,
        into_proof: Value,
    },
    DropProof {
        proof: Value,
    },
    DropAllProofs,

    PublishPackage {
        code: Value,
        abi: Value,
    },

    CreateResource {
        resource_type: Value,
        metadata: Value,
        access_rules: Value,
        mint_params: Value,
    },

    BurnBucket {
        bucket: Value,
    },

    MintFungible {
        resource_address: Value,
        amount: Value,
    },
}

impl Instruction {
    // ============
    // Conversions
    // ============
    pub fn to_ast_instruction(
        &self,
        bech32_manager: &Bech32Manager,
    ) -> Result<AstInstruction, Error> {
        let ast_instruction: AstInstruction = match self {
            Self::CallFunction {
                package_address,
                blueprint_name,
                function_name,
                arguments,
            } => AstInstruction::CallFunction {
                package_address: package_address.to_ast_value(bech32_manager)?,
                blueprint_name: blueprint_name.to_ast_value(bech32_manager)?,
                function_name: function_name.to_ast_value(bech32_manager)?,
                args: arguments
                    .clone()
                    .unwrap_or_default()
                    .iter()
                    .map(|v| v.to_ast_value(bech32_manager))
                    .collect::<Result<Vec<AstValue>, _>>()?,
            },
            Self::CallNativeFunction {
                blueprint_name,
                function_name,
                arguments,
            } => AstInstruction::CallNativeFunction {
                blueprint_name: blueprint_name.to_ast_value(bech32_manager)?,
                function_name: function_name.to_ast_value(bech32_manager)?,
                args: arguments
                    .clone()
                    .unwrap_or_default()
                    .iter()
                    .map(|v| v.to_ast_value(bech32_manager))
                    .collect::<Result<Vec<AstValue>, _>>()?,
            },
            Self::CallMethod {
                component_address,
                method_name,
                arguments,
            } => {
                let scrypto_receiver: AstScryptoReceiver =
                    if let Value::ComponentAddress { address } = component_address {
                        AstScryptoReceiver::Global(AstValue::String(
                            bech32_manager
                                .encoder
                                .encode_component_address_to_string(&address.address),
                        ))
                    } else if let Value::Component { identifier } = component_address {
                        AstScryptoReceiver::Component(AstValue::String(identifier.to_string()))
                    } else {
                        Err(Error::InvalidType {
                            expected_types: vec![ValueKind::Component, ValueKind::ComponentAddress],
                            actual_type: component_address.kind(),
                        })?
                    };

                AstInstruction::CallMethod {
                    receiver: scrypto_receiver,
                    method: method_name.to_ast_value(bech32_manager)?,
                    args: arguments
                        .clone()
                        .unwrap_or_default()
                        .iter()
                        .map(|v| v.to_ast_value(bech32_manager))
                        .collect::<Result<Vec<AstValue>, _>>()?,
                }
            }
            Self::CallNativeMethod {
                receiver,
                method_name,
                arguments,
            } => AstInstruction::CallNativeMethod {
                receiver: AstReceiver::Ref(ast_re_node_from_re_node(receiver)),
                method: method_name.to_ast_value(bech32_manager)?,
                args: arguments
                    .clone()
                    .unwrap_or_default()
                    .iter()
                    .map(|v| v.to_ast_value(bech32_manager))
                    .collect::<Result<Vec<AstValue>, _>>()?,
            },

            Self::TakeFromWorktop {
                resource_address,
                into_bucket,
            } => AstInstruction::TakeFromWorktop {
                resource_address: resource_address.to_ast_value(bech32_manager)?,
                new_bucket: into_bucket.to_ast_value(bech32_manager)?,
            },
            Self::TakeFromWorktopByAmount {
                amount,
                resource_address,
                into_bucket,
            } => AstInstruction::TakeFromWorktopByAmount {
                amount: amount.to_ast_value(bech32_manager)?,
                resource_address: resource_address.to_ast_value(bech32_manager)?,
                new_bucket: into_bucket.to_ast_value(bech32_manager)?,
            },
            Self::TakeFromWorktopByIds {
                ids,
                resource_address,
                into_bucket,
            } => AstInstruction::TakeFromWorktopByIds {
                ids: Value::Array {
                    element_type: ValueKind::NonFungibleId,
                    elements: ids.clone().into_iter().collect(),
                }
                .to_ast_value(bech32_manager)?,
                resource_address: resource_address.to_ast_value(bech32_manager)?,
                new_bucket: into_bucket.to_ast_value(bech32_manager)?,
            },
            Self::ReturnToWorktop { bucket } => AstInstruction::ReturnToWorktop {
                bucket: bucket.to_ast_value(bech32_manager)?,
            },

            Self::AssertWorktopContains { resource_address } => {
                AstInstruction::AssertWorktopContains {
                    resource_address: resource_address.to_ast_value(bech32_manager)?,
                }
            }
            Self::AssertWorktopContainsByAmount {
                amount,
                resource_address,
            } => AstInstruction::AssertWorktopContainsByAmount {
                amount: amount.to_ast_value(bech32_manager)?,
                resource_address: resource_address.to_ast_value(bech32_manager)?,
            },
            Self::AssertWorktopContainsByIds {
                ids,
                resource_address,
            } => AstInstruction::AssertWorktopContainsByIds {
                ids: Value::Array {
                    element_type: ValueKind::NonFungibleId,
                    elements: ids.clone().into_iter().collect(),
                }
                .to_ast_value(bech32_manager)?,
                resource_address: resource_address.to_ast_value(bech32_manager)?,
            },

            Self::PopFromAuthZone { into_proof } => AstInstruction::PopFromAuthZone {
                new_proof: into_proof.to_ast_value(bech32_manager)?,
            },
            Self::PushToAuthZone { proof } => AstInstruction::PushToAuthZone {
                proof: proof.to_ast_value(bech32_manager)?,
            },
            Self::ClearAuthZone => AstInstruction::ClearAuthZone,

            Self::CreateProofFromAuthZone {
                resource_address,
                into_proof,
            } => AstInstruction::CreateProofFromAuthZone {
                resource_address: resource_address.to_ast_value(bech32_manager)?,
                new_proof: into_proof.to_ast_value(bech32_manager)?,
            },
            Self::CreateProofFromAuthZoneByAmount {
                amount,
                resource_address,
                into_proof,
            } => AstInstruction::CreateProofFromAuthZoneByAmount {
                amount: amount.to_ast_value(bech32_manager)?,
                resource_address: resource_address.to_ast_value(bech32_manager)?,
                new_proof: into_proof.to_ast_value(bech32_manager)?,
            },
            Self::CreateProofFromAuthZoneByIds {
                ids,
                resource_address,
                into_proof,
            } => AstInstruction::CreateProofFromAuthZoneByIds {
                ids: Value::Array {
                    element_type: ValueKind::NonFungibleId,
                    elements: ids.clone().into_iter().collect(),
                }
                .to_ast_value(bech32_manager)?,
                resource_address: resource_address.to_ast_value(bech32_manager)?,
                new_proof: into_proof.to_ast_value(bech32_manager)?,
            },
            Self::CreateProofFromBucket { bucket, into_proof } => {
                AstInstruction::CreateProofFromBucket {
                    bucket: bucket.to_ast_value(bech32_manager)?,
                    new_proof: into_proof.to_ast_value(bech32_manager)?,
                }
            }

            Self::CloneProof { proof, into_proof } => AstInstruction::CloneProof {
                proof: proof.to_ast_value(bech32_manager)?,
                new_proof: into_proof.to_ast_value(bech32_manager)?,
            },

            Self::DropProof { proof } => AstInstruction::DropProof {
                proof: proof.to_ast_value(bech32_manager)?,
            },
            Self::DropAllProofs => AstInstruction::DropAllProofs,
            Self::PublishPackage { code, abi } => AstInstruction::PublishPackage {
                code: code.to_ast_value(bech32_manager)?,
                abi: abi.to_ast_value(bech32_manager)?,
            },

            Self::MintFungible {
                resource_address,
                amount,
            } => AstInstruction::MintFungible {
                resource_address: resource_address.to_ast_value(bech32_manager)?,
                amount: amount.to_ast_value(bech32_manager)?,
            },
            Self::BurnBucket { bucket } => AstInstruction::BurnBucket {
                bucket: bucket.to_ast_value(bech32_manager)?,
            },
            Self::CreateResource {
                resource_type,
                metadata,
                access_rules,
                mint_params,
            } => AstInstruction::CreateResource {
                resource_type: resource_type.to_ast_value(bech32_manager)?,
                metadata: metadata.to_ast_value(bech32_manager)?,
                access_rules: access_rules.to_ast_value(bech32_manager)?,
                mint_params: mint_params.to_ast_value(bech32_manager)?,
            },
        };
        Ok(ast_instruction)
    }

    pub fn from_ast_instruction(
        ast_instruction: &AstInstruction,
        bech32_manager: &Bech32Manager,
    ) -> Result<Self, Error> {
        let instruction: Instruction = match ast_instruction {
            AstInstruction::CallFunction {
                package_address,
                blueprint_name,
                function_name,
                args,
            } => Self::CallFunction {
                package_address: Value::from_ast_value(package_address, bech32_manager)?,
                blueprint_name: Value::from_ast_value(blueprint_name, bech32_manager)?,
                function_name: Value::from_ast_value(function_name, bech32_manager)?,
                arguments: {
                    let arguments: Vec<Value> = args
                        .iter()
                        .map(|v| Value::from_ast_value(v, bech32_manager))
                        .collect::<Result<Vec<Value>, _>>()?;
                    match arguments.len() {
                        0 => None,
                        _ => Some(arguments),
                    }
                },
            },
            AstInstruction::CallNativeFunction {
                blueprint_name,
                function_name,
                args,
            } => Self::CallNativeFunction {
                blueprint_name: Value::from_ast_value(blueprint_name, bech32_manager)?,
                function_name: Value::from_ast_value(function_name, bech32_manager)?,
                arguments: {
                    let arguments: Vec<Value> = args
                        .iter()
                        .map(|v| Value::from_ast_value(v, bech32_manager))
                        .collect::<Result<Vec<Value>, _>>()?;
                    match arguments.len() {
                        0 => None,
                        _ => Some(arguments),
                    }
                },
            },
            AstInstruction::CallMethod {
                receiver,
                method,
                args,
            } => Self::CallMethod {
                component_address: match receiver {
                    AstScryptoReceiver::Global(value) => {
                        if let Value::String { value } =
                            Value::from_ast_value(value, bech32_manager)?
                        {
                            Value::ComponentAddress {
                                address: NetworkAwareComponentAddress {
                                    network_id: bech32_manager.network_id(),
                                    address: bech32_manager
                                        .decoder
                                        .validate_and_decode_component_address(&value)?,
                                },
                            }
                        } else {
                            Err(Error::InvalidType {
                                expected_types: vec![ValueKind::String],
                                actual_type: value.kind().into(),
                            })?
                        }
                    }
                    AstScryptoReceiver::Component(value) => {
                        if let Value::String { value } =
                            Value::from_ast_value(value, bech32_manager)?
                        {
                            Value::Component {
                                identifier: value.parse()?,
                            }
                        } else {
                            Err(Error::InvalidType {
                                expected_types: vec![ValueKind::String],
                                actual_type: value.kind().into(),
                            })?
                        }
                    }
                },
                method_name: Value::from_ast_value(method, bech32_manager)?,
                arguments: {
                    let arguments: Vec<Value> = args
                        .iter()
                        .map(|v| Value::from_ast_value(v, bech32_manager))
                        .collect::<Result<Vec<Value>, _>>()?;
                    match arguments.len() {
                        0 => None,
                        _ => Some(arguments),
                    }
                },
            },
            AstInstruction::CallNativeMethod {
                receiver,
                method,
                args,
            } => Self::CallNativeMethod {
                receiver: match receiver {
                    AstReceiver::Ref(ast_re_node) => re_node_from_ast_re_node(ast_re_node)?,
                },
                method_name: Value::from_ast_value(method, bech32_manager)?,
                arguments: {
                    let arguments: Vec<Value> = args
                        .iter()
                        .map(|v| Value::from_ast_value(v, bech32_manager))
                        .collect::<Result<Vec<Value>, _>>()?;
                    match arguments.len() {
                        0 => None,
                        _ => Some(arguments),
                    }
                },
            },

            AstInstruction::TakeFromWorktop {
                resource_address,
                new_bucket,
            } => Self::TakeFromWorktop {
                resource_address: Value::from_ast_value(resource_address, bech32_manager)?,
                into_bucket: Value::from_ast_value(new_bucket, bech32_manager)?,
            },
            AstInstruction::TakeFromWorktopByAmount {
                amount,
                resource_address,
                new_bucket,
            } => Self::TakeFromWorktopByAmount {
                amount: Value::from_ast_value(amount, bech32_manager)?,
                resource_address: Value::from_ast_value(resource_address, bech32_manager)?,
                into_bucket: Value::from_ast_value(new_bucket, bech32_manager)?,
            },
            AstInstruction::TakeFromWorktopByIds {
                ids,
                resource_address,
                new_bucket,
            } => Self::TakeFromWorktopByIds {
                ids: if let Value::Array {
                    element_type: _,
                    elements,
                } = Value::from_ast_value(ids, bech32_manager)?
                {
                    elements.into_iter().collect()
                } else {
                    panic!("Expected type Array!")
                },
                resource_address: Value::from_ast_value(resource_address, bech32_manager)?,
                into_bucket: Value::from_ast_value(new_bucket, bech32_manager)?,
            },
            AstInstruction::ReturnToWorktop { bucket } => Self::ReturnToWorktop {
                bucket: Value::from_ast_value(bucket, bech32_manager)?,
            },

            AstInstruction::AssertWorktopContains { resource_address } => {
                Self::AssertWorktopContains {
                    resource_address: Value::from_ast_value(resource_address, bech32_manager)?,
                }
            }
            AstInstruction::AssertWorktopContainsByAmount {
                amount,
                resource_address,
            } => Self::AssertWorktopContainsByAmount {
                amount: Value::from_ast_value(amount, bech32_manager)?,
                resource_address: Value::from_ast_value(resource_address, bech32_manager)?,
            },
            AstInstruction::AssertWorktopContainsByIds {
                ids,
                resource_address,
            } => Self::AssertWorktopContainsByIds {
                ids: if let Value::Array {
                    element_type: _,
                    elements,
                } = Value::from_ast_value(ids, bech32_manager)?
                {
                    elements.into_iter().collect()
                } else {
                    panic!("Expected type Array!")
                },
                resource_address: Value::from_ast_value(resource_address, bech32_manager)?,
            },

            AstInstruction::PopFromAuthZone { new_proof } => Self::PopFromAuthZone {
                into_proof: Value::from_ast_value(new_proof, bech32_manager)?,
            },
            AstInstruction::PushToAuthZone { proof } => Self::PushToAuthZone {
                proof: Value::from_ast_value(proof, bech32_manager)?,
            },
            AstInstruction::ClearAuthZone => Self::ClearAuthZone,

            AstInstruction::CreateProofFromAuthZone {
                resource_address,
                new_proof,
            } => Self::CreateProofFromAuthZone {
                resource_address: Value::from_ast_value(resource_address, bech32_manager)?,
                into_proof: Value::from_ast_value(new_proof, bech32_manager)?,
            },
            AstInstruction::CreateProofFromAuthZoneByAmount {
                amount,
                resource_address,
                new_proof,
            } => Self::CreateProofFromAuthZoneByAmount {
                amount: Value::from_ast_value(amount, bech32_manager)?,
                resource_address: Value::from_ast_value(resource_address, bech32_manager)?,
                into_proof: Value::from_ast_value(new_proof, bech32_manager)?,
            },
            AstInstruction::CreateProofFromAuthZoneByIds {
                ids,
                resource_address,
                new_proof,
            } => Self::CreateProofFromAuthZoneByIds {
                ids: if let Value::Array {
                    element_type: _,
                    elements,
                } = Value::from_ast_value(ids, bech32_manager)?
                {
                    elements.into_iter().collect()
                } else {
                    panic!("Expected type Array!")
                },
                resource_address: Value::from_ast_value(resource_address, bech32_manager)?,
                into_proof: Value::from_ast_value(new_proof, bech32_manager)?,
            },
            AstInstruction::CreateProofFromBucket { bucket, new_proof } => {
                Self::CreateProofFromBucket {
                    bucket: Value::from_ast_value(bucket, bech32_manager)?,
                    into_proof: Value::from_ast_value(new_proof, bech32_manager)?,
                }
            }

            AstInstruction::CloneProof { proof, new_proof } => Self::CloneProof {
                proof: Value::from_ast_value(proof, bech32_manager)?,
                into_proof: Value::from_ast_value(new_proof, bech32_manager)?,
            },
            AstInstruction::DropProof { proof } => Self::DropProof {
                proof: Value::from_ast_value(proof, bech32_manager)?,
            },
            AstInstruction::DropAllProofs => Self::DropAllProofs,
            AstInstruction::PublishPackage { code, abi } => Self::PublishPackage {
                code: Value::from_ast_value(code, bech32_manager)?,
                abi: Value::from_ast_value(abi, bech32_manager)?,
            },
            AstInstruction::MintFungible {
                resource_address,
                amount,
            } => Self::MintFungible {
                resource_address: Value::from_ast_value(resource_address, bech32_manager)?,
                amount: Value::from_ast_value(amount, bech32_manager)?,
            },
            AstInstruction::BurnBucket { bucket } => Self::BurnBucket {
                bucket: Value::from_ast_value(bucket, bech32_manager)?,
            },
            AstInstruction::CreateResource {
                resource_type,
                metadata,
                access_rules,
                mint_params,
            } => Self::CreateResource {
                resource_type: Value::from_ast_value(resource_type, bech32_manager)?,
                metadata: Value::from_ast_value(metadata, bech32_manager)?,
                access_rules: Value::from_ast_value(access_rules, bech32_manager)?,
                mint_params: Value::from_ast_value(mint_params, bech32_manager)?,
            },
        };
        Ok(instruction)
    }
}

// ===========
// Validation
// ===========

impl ValidateWithContext<u8> for Instruction {
    fn validate(&self, network_id: u8) -> Result<(), Error> {
        match self {
            Self::CallFunction {
                package_address,
                blueprint_name,
                function_name,
                arguments,
            } => {
                package_address.validate((network_id, Some(ValueKind::PackageAddress)))?;
                blueprint_name.validate((network_id, Some(ValueKind::String)))?;
                function_name.validate((network_id, Some(ValueKind::String)))?;
                arguments
                    .clone()
                    .unwrap_or_default()
                    .iter()
                    .map(|arg| arg.validate((network_id, None)))
                    .collect::<Result<Vec<()>, Error>>()?;
                Ok(())
            }
            Self::CallNativeFunction {
                blueprint_name,
                function_name,
                arguments,
            } => {
                blueprint_name.validate((network_id, Some(ValueKind::String)))?;
                function_name.validate((network_id, Some(ValueKind::String)))?;
                arguments
                    .clone()
                    .unwrap_or_default()
                    .iter()
                    .map(|arg| arg.validate((network_id, None)))
                    .collect::<Result<Vec<()>, Error>>()?;
                Ok(())
            }

            Self::CallMethod {
                component_address,
                method_name,
                arguments,
            } => {
                component_address
                    .validate((network_id, Some(ValueKind::ComponentAddress)))
                    .or_else(|_| {
                        component_address.validate((network_id, Some(ValueKind::Component)))
                    })?;
                method_name.validate((network_id, Some(ValueKind::String)))?;
                arguments
                    .clone()
                    .unwrap_or_default()
                    .iter()
                    .map(|arg| arg.validate((network_id, None)))
                    .collect::<Result<Vec<()>, Error>>()?;
                Ok(())
            }
            Self::CallNativeMethod {
                method_name,
                arguments,
                ..
            } => {
                method_name.validate((network_id, Some(ValueKind::String)))?;
                arguments
                    .clone()
                    .unwrap_or_default()
                    .iter()
                    .map(|arg| arg.validate((network_id, None)))
                    .collect::<Result<Vec<()>, Error>>()?;
                Ok(())
            }

            Self::TakeFromWorktop {
                resource_address,
                into_bucket,
            } => {
                resource_address.validate((network_id, Some(ValueKind::ResourceAddress)))?;
                into_bucket.validate((network_id, Some(ValueKind::Bucket)))?;
                Ok(())
            }
            Self::TakeFromWorktopByAmount {
                amount,
                resource_address,
                into_bucket,
            } => {
                amount.validate((network_id, Some(ValueKind::Decimal)))?;
                resource_address.validate((network_id, Some(ValueKind::ResourceAddress)))?;
                into_bucket.validate((network_id, Some(ValueKind::Bucket)))?;
                Ok(())
            }
            Self::TakeFromWorktopByIds {
                ids,
                resource_address,
                into_bucket,
            } => {
                ids.iter()
                    .map(|id| id.validate((network_id, Some(ValueKind::NonFungibleId))))
                    .collect::<Result<Vec<()>, _>>()?;
                resource_address.validate((network_id, Some(ValueKind::ResourceAddress)))?;
                into_bucket.validate((network_id, Some(ValueKind::Bucket)))?;
                Ok(())
            }
            Self::ReturnToWorktop { bucket } => {
                bucket.validate((network_id, Some(ValueKind::Bucket)))?;
                Ok(())
            }

            Self::AssertWorktopContains { resource_address } => {
                resource_address.validate((network_id, Some(ValueKind::ResourceAddress)))?;
                Ok(())
            }
            Self::AssertWorktopContainsByAmount {
                amount,
                resource_address,
            } => {
                amount.validate((network_id, Some(ValueKind::Decimal)))?;
                resource_address.validate((network_id, Some(ValueKind::ResourceAddress)))?;
                Ok(())
            }
            Self::AssertWorktopContainsByIds {
                ids,
                resource_address,
            } => {
                ids.iter()
                    .map(|id| id.validate((network_id, Some(ValueKind::NonFungibleId))))
                    .collect::<Result<Vec<()>, _>>()?;
                resource_address.validate((network_id, Some(ValueKind::ResourceAddress)))?;
                Ok(())
            }

            Self::PopFromAuthZone { into_proof } => {
                into_proof.validate((network_id, Some(ValueKind::Proof)))?;
                Ok(())
            }
            Self::PushToAuthZone { proof } => {
                proof.validate((network_id, Some(ValueKind::Proof)))?;
                Ok(())
            }
            Self::ClearAuthZone => Ok(()),

            Self::CreateProofFromAuthZone {
                resource_address,
                into_proof,
            } => {
                resource_address.validate((network_id, Some(ValueKind::ResourceAddress)))?;
                into_proof.validate((network_id, Some(ValueKind::Proof)))?;
                Ok(())
            }
            Self::CreateProofFromAuthZoneByAmount {
                amount,
                resource_address,
                into_proof,
            } => {
                amount.validate((network_id, Some(ValueKind::Decimal)))?;
                resource_address.validate((network_id, Some(ValueKind::ResourceAddress)))?;
                into_proof.validate((network_id, Some(ValueKind::Proof)))?;
                Ok(())
            }
            Self::CreateProofFromAuthZoneByIds {
                ids,
                resource_address,
                into_proof,
            } => {
                ids.iter()
                    .map(|id| id.validate((network_id, Some(ValueKind::NonFungibleId))))
                    .collect::<Result<Vec<()>, _>>()?;
                resource_address.validate((network_id, Some(ValueKind::ResourceAddress)))?;
                into_proof.validate((network_id, Some(ValueKind::Proof)))?;
                Ok(())
            }

            Self::CreateProofFromBucket { bucket, into_proof } => {
                bucket.validate((network_id, Some(ValueKind::Bucket)))?;
                into_proof.validate((network_id, Some(ValueKind::Proof)))?;
                Ok(())
            }

            Self::CloneProof { proof, into_proof } => {
                proof.validate((network_id, Some(ValueKind::Proof)))?;
                into_proof.validate((network_id, Some(ValueKind::Proof)))?;
                Ok(())
            }
            Self::DropProof { proof } => {
                proof.validate((network_id, Some(ValueKind::Proof)))?;
                Ok(())
            }
            Self::DropAllProofs => Ok(()),

            Self::PublishPackage { code, abi } => {
                code.validate((network_id, Some(ValueKind::Blob)))?;
                abi.validate((network_id, Some(ValueKind::Blob)))?;
                Ok(())
            }

            Self::MintFungible { .. } => {
                // TODO: Add validation for this instruction
                Ok(())
            }
            Self::BurnBucket { bucket } => {
                bucket.validate((network_id, Some(ValueKind::Bucket)))?;
                Ok(())
            }
            Self::CreateResource { .. } => {
                // TODO: Add validation for this instruction
                Ok(())
            }
        }
    }
}
