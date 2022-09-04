use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::collections::HashSet;
use transaction::manifest::ast::{Instruction as AstInstruction, Value as AstValue};

use crate::error::Error;
use crate::models::value::*;

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum Instruction {
    CallFunction {
        package_address: Value,
        blueprint_name: Value,
        function_name: Value,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        arguments: Option<Vec<Value>>,
    },
    CallMethod {
        component_address: Value,
        method_name: Value,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        arguments: Option<Vec<Value>>,
    },
    CallMethodWithAllResources {
        component_address: Value,
        method: Value,
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
        package: Value,
    },
}

impl Instruction {
    pub fn validate_instruction_arguments(&self) -> Result<(), Error> {
        match self {
            Self::CallFunction {
                package_address,
                blueprint_name,
                function_name,
                arguments,
            } => {
                package_address.validate_kind(ValueKind::PackageAddress)?;
                blueprint_name.validate_kind(ValueKind::String)?;
                function_name.validate_kind(ValueKind::String)?;
                arguments
                    .clone()
                    .unwrap_or_default()
                    .iter()
                    .map(|arg| arg.validate_if_collection())
                    .collect::<Result<Vec<()>, Error>>()?;
                Ok(())
            }
            Self::CallMethod {
                component_address,
                method_name,
                arguments,
            } => {
                component_address.validate_kind(ValueKind::ComponentAddress)?;
                method_name.validate_kind(ValueKind::String)?;
                arguments
                    .clone()
                    .unwrap_or_default()
                    .iter()
                    .map(|arg| arg.validate_if_collection())
                    .collect::<Result<Vec<()>, Error>>()?;
                Ok(())
            }
            Self::CallMethodWithAllResources {
                component_address,
                method,
            } => {
                component_address.validate_kind(ValueKind::ComponentAddress)?;
                method.validate_kind(ValueKind::String)?;
                Ok(())
            }

            Self::TakeFromWorktop {
                resource_address,
                into_bucket,
            } => {
                resource_address.validate_kind(ValueKind::ResourceAddress)?;
                into_bucket.validate_kind(ValueKind::Bucket)?;
                Ok(())
            }
            Self::TakeFromWorktopByAmount {
                amount,
                resource_address,
                into_bucket,
            } => {
                amount.validate_kind(ValueKind::Decimal)?;
                resource_address.validate_kind(ValueKind::ResourceAddress)?;
                into_bucket.validate_kind(ValueKind::Bucket)?;
                Ok(())
            }
            Self::TakeFromWorktopByIds {
                ids,
                resource_address,
                into_bucket,
            } => {
                ids.iter()
                    .map(|id| id.validate_kind(ValueKind::NonFungibleId))
                    .collect::<Result<Vec<()>, _>>()?;
                resource_address.validate_kind(ValueKind::ResourceAddress)?;
                into_bucket.validate_kind(ValueKind::Bucket)?;
                Ok(())
            }
            Self::ReturnToWorktop { bucket } => {
                bucket.validate_kind(ValueKind::Bucket)?;
                Ok(())
            }

            Self::AssertWorktopContains { resource_address } => {
                resource_address.validate_kind(ValueKind::ResourceAddress)?;
                Ok(())
            }
            Self::AssertWorktopContainsByAmount {
                amount,
                resource_address,
            } => {
                amount.validate_kind(ValueKind::Decimal)?;
                resource_address.validate_kind(ValueKind::ResourceAddress)?;
                Ok(())
            }
            Self::AssertWorktopContainsByIds {
                ids,
                resource_address,
            } => {
                ids.iter()
                    .map(|id| id.validate_kind(ValueKind::NonFungibleId))
                    .collect::<Result<Vec<()>, _>>()?;
                resource_address.validate_kind(ValueKind::ResourceAddress)?;
                Ok(())
            }

            Self::PopFromAuthZone { into_proof } => {
                into_proof.validate_kind(ValueKind::Proof)?;
                Ok(())
            }
            Self::PushToAuthZone { proof } => {
                proof.validate_kind(ValueKind::Proof)?;
                Ok(())
            }
            Self::ClearAuthZone => Ok(()),

            Self::CreateProofFromAuthZone {
                resource_address,
                into_proof,
            } => {
                resource_address.validate_kind(ValueKind::ResourceAddress)?;
                into_proof.validate_kind(ValueKind::Proof)?;
                Ok(())
            }
            Self::CreateProofFromAuthZoneByAmount {
                amount,
                resource_address,
                into_proof,
            } => {
                amount.validate_kind(ValueKind::Decimal)?;
                resource_address.validate_kind(ValueKind::ResourceAddress)?;
                into_proof.validate_kind(ValueKind::Proof)?;
                Ok(())
            }
            Self::CreateProofFromAuthZoneByIds {
                ids,
                resource_address,
                into_proof,
            } => {
                ids.iter()
                    .map(|id| id.validate_kind(ValueKind::NonFungibleId))
                    .collect::<Result<Vec<()>, _>>()?;
                resource_address.validate_kind(ValueKind::ResourceAddress)?;
                into_proof.validate_kind(ValueKind::Proof)?;
                Ok(())
            }

            Self::CreateProofFromBucket { bucket, into_proof } => {
                bucket.validate_kind(ValueKind::Bucket)?;
                into_proof.validate_kind(ValueKind::Proof)?;
                Ok(())
            }

            Self::CloneProof { proof, into_proof } => {
                proof.validate_kind(ValueKind::Proof)?;
                into_proof.validate_kind(ValueKind::Proof)?;
                Ok(())
            }
            Self::DropProof { proof } => {
                proof.validate_kind(ValueKind::Proof)?;
                Ok(())
            }
            Self::DropAllProofs => Ok(()),

            Self::PublishPackage { package } => {
                package.validate_kind(ValueKind::Bytes)?;
                Ok(())
            }
        }
    }
}

// ============
// Conversions
// ============

// TODO: This function should be transaction version dependent and should require the transaction
// version of its operations.
pub fn ast_instruction_from_instruction(
    instruction: &Instruction,
    network_id: u8,
) -> Result<AstInstruction, Error> {
    let ast_instruction: AstInstruction = match instruction {
        Instruction::CallFunction {
            package_address,
            blueprint_name,
            function_name,
            arguments,
        } => AstInstruction::CallFunction {
            package_address: ast_value_from_value(package_address, network_id)?,
            blueprint_name: ast_value_from_value(blueprint_name, network_id)?,
            function: ast_value_from_value(function_name, network_id)?,
            args: arguments
                .clone()
                .unwrap_or_default()
                .iter()
                .map(|v| ast_value_from_value(v, network_id))
                .collect::<Result<Vec<AstValue>, _>>()?,
        },
        Instruction::CallMethod {
            component_address,
            method_name,
            arguments,
        } => AstInstruction::CallMethod {
            component_address: ast_value_from_value(component_address, network_id)?,
            method: ast_value_from_value(method_name, network_id)?,
            args: arguments
                .clone()
                .unwrap_or_default()
                .iter()
                .map(|v| ast_value_from_value(v, network_id))
                .collect::<Result<Vec<AstValue>, _>>()?,
        },
        Instruction::CallMethodWithAllResources {
            component_address,
            method,
        } => AstInstruction::CallMethodWithAllResources {
            component_address: ast_value_from_value(component_address, network_id)?,
            method: ast_value_from_value(method, network_id)?,
        },

        Instruction::TakeFromWorktop {
            resource_address,
            into_bucket,
        } => AstInstruction::TakeFromWorktop {
            resource_address: ast_value_from_value(resource_address, network_id)?,
            new_bucket: ast_value_from_value(into_bucket, network_id)?,
        },
        Instruction::TakeFromWorktopByAmount {
            amount,
            resource_address,
            into_bucket,
        } => AstInstruction::TakeFromWorktopByAmount {
            amount: ast_value_from_value(amount, network_id)?,
            resource_address: ast_value_from_value(resource_address, network_id)?,
            new_bucket: ast_value_from_value(into_bucket, network_id)?,
        },
        Instruction::TakeFromWorktopByIds {
            ids,
            resource_address,
            into_bucket,
        } => AstInstruction::TakeFromWorktopByIds {
            ids: {
                ast_value_from_value(
                    &Value::Set {
                        element_type: ValueKind::NonFungibleId,
                        elements: ids.clone().into_iter().collect(),
                    },
                    network_id,
                )?
            },
            resource_address: ast_value_from_value(resource_address, network_id)?,
            new_bucket: ast_value_from_value(into_bucket, network_id)?,
        },
        Instruction::ReturnToWorktop { bucket } => AstInstruction::ReturnToWorktop {
            bucket: ast_value_from_value(bucket, network_id)?,
        },

        Instruction::AssertWorktopContains { resource_address } => {
            AstInstruction::AssertWorktopContains {
                resource_address: ast_value_from_value(resource_address, network_id)?,
            }
        }
        Instruction::AssertWorktopContainsByAmount {
            amount,
            resource_address,
        } => AstInstruction::AssertWorktopContainsByAmount {
            amount: ast_value_from_value(amount, network_id)?,
            resource_address: ast_value_from_value(resource_address, network_id)?,
        },
        Instruction::AssertWorktopContainsByIds {
            ids,
            resource_address,
        } => AstInstruction::AssertWorktopContainsByIds {
            ids: {
                ast_value_from_value(
                    &Value::Set {
                        element_type: ValueKind::NonFungibleId,
                        elements: ids.clone().into_iter().collect(),
                    },
                    network_id,
                )?
            },
            resource_address: ast_value_from_value(resource_address, network_id)?,
        },

        Instruction::PopFromAuthZone { into_proof } => AstInstruction::PopFromAuthZone {
            new_proof: ast_value_from_value(into_proof, network_id)?,
        },
        Instruction::PushToAuthZone { proof } => AstInstruction::PushToAuthZone {
            proof: ast_value_from_value(proof, network_id)?,
        },
        Instruction::ClearAuthZone => AstInstruction::ClearAuthZone,

        Instruction::CreateProofFromAuthZone {
            resource_address,
            into_proof,
        } => AstInstruction::CreateProofFromAuthZone {
            resource_address: ast_value_from_value(resource_address, network_id)?,
            new_proof: ast_value_from_value(into_proof, network_id)?,
        },
        Instruction::CreateProofFromAuthZoneByAmount {
            amount,
            resource_address,
            into_proof,
        } => AstInstruction::CreateProofFromAuthZoneByAmount {
            amount: ast_value_from_value(amount, network_id)?,
            resource_address: ast_value_from_value(resource_address, network_id)?,
            new_proof: ast_value_from_value(into_proof, network_id)?,
        },
        Instruction::CreateProofFromAuthZoneByIds {
            ids,
            resource_address,
            into_proof,
        } => AstInstruction::CreateProofFromAuthZoneByIds {
            ids: {
                ast_value_from_value(
                    &Value::Set {
                        element_type: ValueKind::NonFungibleId,
                        elements: ids.clone().into_iter().collect(),
                    },
                    network_id,
                )?
            },
            resource_address: ast_value_from_value(resource_address, network_id)?,
            new_proof: ast_value_from_value(into_proof, network_id)?,
        },
        Instruction::CreateProofFromBucket { bucket, into_proof } => {
            AstInstruction::CreateProofFromBucket {
                bucket: ast_value_from_value(bucket, network_id)?,
                new_proof: ast_value_from_value(into_proof, network_id)?,
            }
        }

        Instruction::CloneProof { proof, into_proof } => AstInstruction::CloneProof {
            proof: ast_value_from_value(proof, network_id)?,
            new_proof: ast_value_from_value(into_proof, network_id)?,
        },

        Instruction::DropProof { proof } => AstInstruction::DropProof {
            proof: ast_value_from_value(proof, network_id)?,
        },
        Instruction::DropAllProofs => AstInstruction::DropAllProofs,
        Instruction::PublishPackage { package } => AstInstruction::PublishPackage {
            package: ast_value_from_value(package, network_id)?,
        },
    };
    Ok(ast_instruction)
}

// TODO: This function should be transaction version dependent and should require the transaction
// version of its operations.
// TODO: Investigate if this function should output a version-aware instruction.
pub fn instruction_from_ast_instruction(
    ast_instruction: &AstInstruction,
    network_id: u8,
) -> Result<Instruction, Error> {
    let instruction: Instruction = match ast_instruction {
        AstInstruction::CallFunction {
            package_address,
            blueprint_name,
            function,
            args,
        } => Instruction::CallFunction {
            package_address: value_from_ast_value(package_address, network_id)?,
            blueprint_name: value_from_ast_value(blueprint_name, network_id)?,
            function_name: value_from_ast_value(function, network_id)?,
            arguments: {
                let arguments: Vec<Value> = args
                    .iter()
                    .map(|v| value_from_ast_value(v, network_id))
                    .collect::<Result<Vec<Value>, _>>()?;
                match arguments.len() {
                    0 => None,
                    _ => Some(arguments),
                }
            },
        },
        AstInstruction::CallMethod {
            component_address,
            method,
            args,
        } => Instruction::CallMethod {
            component_address: value_from_ast_value(component_address, network_id)?,
            method_name: value_from_ast_value(method, network_id)?,
            arguments: {
                let arguments: Vec<Value> = args
                    .iter()
                    .map(|v| value_from_ast_value(v, network_id))
                    .collect::<Result<Vec<Value>, _>>()?;
                match arguments.len() {
                    0 => None,
                    _ => Some(arguments),
                }
            },
        },
        AstInstruction::CallMethodWithAllResources {
            component_address,
            method,
        } => Instruction::CallMethodWithAllResources {
            component_address: value_from_ast_value(component_address, network_id)?,
            method: value_from_ast_value(method, network_id)?,
        },

        AstInstruction::TakeFromWorktop {
            resource_address,
            new_bucket,
        } => Instruction::TakeFromWorktop {
            resource_address: value_from_ast_value(resource_address, network_id)?,
            into_bucket: value_from_ast_value(new_bucket, network_id)?,
        },
        AstInstruction::TakeFromWorktopByAmount {
            amount,
            resource_address,
            new_bucket,
        } => Instruction::TakeFromWorktopByAmount {
            amount: value_from_ast_value(amount, network_id)?,
            resource_address: value_from_ast_value(resource_address, network_id)?,
            into_bucket: value_from_ast_value(new_bucket, network_id)?,
        },
        AstInstruction::TakeFromWorktopByIds {
            ids,
            resource_address,
            new_bucket,
        } => Instruction::TakeFromWorktopByIds {
            ids: if let Value::Set {
                element_type: _,
                elements,
            } = value_from_ast_value(ids, network_id)?
            {
                elements.clone().into_iter().collect()
            } else {
                panic!("Expected type Set!")
            },
            resource_address: value_from_ast_value(resource_address, network_id)?,
            into_bucket: value_from_ast_value(new_bucket, network_id)?,
        },
        AstInstruction::ReturnToWorktop { bucket } => Instruction::ReturnToWorktop {
            bucket: value_from_ast_value(bucket, network_id)?,
        },

        AstInstruction::AssertWorktopContains { resource_address } => {
            Instruction::AssertWorktopContains {
                resource_address: value_from_ast_value(resource_address, network_id)?,
            }
        }
        AstInstruction::AssertWorktopContainsByAmount {
            amount,
            resource_address,
        } => Instruction::AssertWorktopContainsByAmount {
            amount: value_from_ast_value(amount, network_id)?,
            resource_address: value_from_ast_value(resource_address, network_id)?,
        },
        AstInstruction::AssertWorktopContainsByIds {
            ids,
            resource_address,
        } => Instruction::AssertWorktopContainsByIds {
            ids: if let Value::Set {
                element_type: _,
                elements,
            } = value_from_ast_value(ids, network_id)?
            {
                elements.clone().into_iter().collect()
            } else {
                panic!("Expected type Set!")
            },
            resource_address: value_from_ast_value(resource_address, network_id)?,
        },

        AstInstruction::PopFromAuthZone { new_proof } => Instruction::PopFromAuthZone {
            into_proof: value_from_ast_value(new_proof, network_id)?,
        },
        AstInstruction::PushToAuthZone { proof } => Instruction::PushToAuthZone {
            proof: value_from_ast_value(proof, network_id)?,
        },
        AstInstruction::ClearAuthZone => Instruction::ClearAuthZone,

        AstInstruction::CreateProofFromAuthZone {
            resource_address,
            new_proof,
        } => Instruction::CreateProofFromAuthZone {
            resource_address: value_from_ast_value(resource_address, network_id)?,
            into_proof: value_from_ast_value(new_proof, network_id)?,
        },
        AstInstruction::CreateProofFromAuthZoneByAmount {
            amount,
            resource_address,
            new_proof,
        } => Instruction::CreateProofFromAuthZoneByAmount {
            amount: value_from_ast_value(amount, network_id)?,
            resource_address: value_from_ast_value(resource_address, network_id)?,
            into_proof: value_from_ast_value(new_proof, network_id)?,
        },
        AstInstruction::CreateProofFromAuthZoneByIds {
            ids,
            resource_address,
            new_proof,
        } => Instruction::CreateProofFromAuthZoneByIds {
            ids: if let Value::Set {
                element_type: _,
                elements,
            } = value_from_ast_value(ids, network_id)?
            {
                elements.clone().into_iter().collect()
            } else {
                panic!("Expected type Set!")
            },
            resource_address: value_from_ast_value(resource_address, network_id)?,
            into_proof: value_from_ast_value(new_proof, network_id)?,
        },
        AstInstruction::CreateProofFromBucket { bucket, new_proof } => {
            Instruction::CreateProofFromBucket {
                bucket: value_from_ast_value(bucket, network_id)?,
                into_proof: value_from_ast_value(new_proof, network_id)?,
            }
        }

        AstInstruction::CloneProof { proof, new_proof } => Instruction::CloneProof {
            proof: value_from_ast_value(proof, network_id)?,
            into_proof: value_from_ast_value(new_proof, network_id)?,
        },
        AstInstruction::DropProof { proof } => Instruction::DropProof {
            proof: value_from_ast_value(proof, network_id)?,
        },
        AstInstruction::DropAllProofs => Instruction::DropAllProofs,
        AstInstruction::PublishPackage { package } => Instruction::PublishPackage {
            package: value_from_ast_value(package, network_id)?,
        },
    };
    Ok(instruction)
}
