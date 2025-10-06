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

use crate::internal_prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct EntitySecurifyAnalyzer(EntitySecurifyOutput);

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct EntitySecurifyOutput {
    pub securified_accounts: Vec<ManifestGlobalAddress>,
    pub securified_identities: Vec<ManifestGlobalAddress>,
}

impl ManifestStaticAnalyzer for EntitySecurifyAnalyzer {
    type Initializer = ();
    type Output = EntitySecurifyOutput;
    type PermissionState =
        CallbackPermissionState<PermissionStateStaticCallback>;

    type RequirementState =
        AllOfRequirement<(EntitySecurify, CreateAccessController)>;

    fn new(
        _: Self::Initializer,
    ) -> (Self, Self::PermissionState, Self::RequirementState) {
        (
            Default::default(),
            CallbackPermissionState::new(is_instruction_permitted),
            Default::default(),
        )
    }

    fn output(self) -> Self::Output {
        self.0
    }

    fn process_instruction(&mut self, context: InstructionContext<'_>) {
        let InstructionContext::InvocationInstruction {
            typed_native_invocation:
                Some(TypedNativeInvocation {
                    receiver: ManifestInvocationReceiver::GlobalMethod(receiver),
                    invocation,
                }),
            ..
        } = context
        else {
            return;
        };

        match invocation {
            TypedManifestNativeInvocation::AccountBlueprintInvocation(
                AccountBlueprintInvocation::Method(
                    AccountBlueprintMethod::Securify(..),
                ),
            ) => {
                self.0.securified_accounts.push(receiver.into());
            }

            TypedManifestNativeInvocation::IdentityBlueprintInvocation(
                IdentityBlueprintInvocation::Method(
                    IdentityBlueprintMethod::Securify(..),
                ),
            ) => {
                self.0.securified_identities.push(receiver.into());
            }
            _ => return,
        }
    }
}

fn is_instruction_permitted(context: InstructionContext<'_>) -> bool {
    match context.instruction() {
        GroupedInstruction::InvocationInstructions(
            InvocationInstructions::CallMethod(CallMethod {
                address,
                method_name,
                ..
            }),
        ) => {
            let grouped_entity_type = match address {
                ManifestGlobalAddress::Static(static_address) => {
                    static_address.as_node_id().entity_type()
                }
                ManifestGlobalAddress::Named(named_address) => context
                    .named_address_store()
                    .get(named_address)
                    .and_then(BlueprintId::entity_type),
            }
            .map(GroupedEntityType::from);

            match (grouped_entity_type, method_name.as_str()) {
                // Selective Permissions
                (
                    Some(GroupedEntityType::AccountEntities(..)),
                    // Allow invocation of "securify" on account
                    ACCOUNT_SECURIFY_IDENT
                    // Allow invocations related to fee payment
                    | ACCOUNT_CREATE_PROOF_OF_AMOUNT_IDENT
                    | ACCOUNT_CREATE_PROOF_OF_NON_FUNGIBLES_IDENT
                    | ACCOUNT_LOCK_FEE_IDENT
                )
                | (
                    Some(GroupedEntityType::IdentityEntities(..)),
                    // Allow invocation of "securify" on identities
                    IDENTITY_SECURIFY_IDENT
                )
                | (
                    Some(GroupedEntityType::AccessControllerEntities(..)),
                    // Normally used to create proof for fee payment if fee payer
                    // is already securified with an AC
                    ACCESS_CONTROLLER_CREATE_PROOF_IDENT,
                )
                => true,
                _ => false,
            }
        }

        // These are the instructions for creating proofs with the account badge
        // that was just put on the worktop after securifying the account.
        // It is mainly needed for setting the owner_keys metadata on the account.
        GroupedInstruction::ProofInstructions(..)
        | GroupedInstruction::TakeFromWorktopInstructions(..)
        | GroupedInstruction::InvocationInstructions(
            InvocationInstructions::CallMetadataMethod(..),
        ) => true,

        // Address allocation for the new access controller to be created
        GroupedInstruction::AddressAllocationInstructions(..) => true,

        // Create access controller function
        GroupedInstruction::InvocationInstructions(
            InvocationInstructions::CallFunction(CallFunction {
                package_address: ManifestPackageAddress::Static(package_address),
                blueprint_name,
                function_name,
                args,
            }),
        ) => {
            let blueprint_id =
                BlueprintId::new(package_address, blueprint_name);
            let typed_invocation =
                TypedManifestNativeInvocation::from_function_invocation(
                    &blueprint_id,
                    function_name,
                    args,
                );
            match typed_invocation {
                Ok(Some(
                    TypedManifestNativeInvocation::AccessControllerBlueprintInvocation(
                        AccessControllerBlueprintInvocation::Function(
                            AccessControllerBlueprintFunction::Create(..)
                        )
                    )
                ))  => true,
                _ => false
            }
        }
        _ => false,
    }
}
