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
pub struct AccessControllerRecoveryAnalyzer(AccessControllerRecoveryOutput);

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct AccessControllerRecoveryOutput {
    pub access_controllers: Vec<ComponentAddress>,
}

impl ManifestStaticAnalyzer for AccessControllerRecoveryAnalyzer {
    type Initializer = ();
    type Output = AccessControllerRecoveryOutput;

    type PermissionState =
        CallbackPermissionState<PermissionStateStaticCallback>;

    type RequirementState = AnyOfRequirement<(
        AccessControllerInitiateRecoveryAsPrimary,
        AccessControllerInitiateRecoveryAsRecovery,
    )>;

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
                    receiver:
                     ManifestInvocationReceiver::GlobalMethod(
                            ResolvedManifestAddress::Static {
                                static_address: ac_address,
                            },
                    ),

                    invocation:
                    TypedManifestNativeInvocation::AccessControllerBlueprintInvocation(
                        AccessControllerBlueprintInvocation::Method(
                            AccessControllerBlueprintMethod::InitiateRecoveryAsPrimary(..)
                        )
                    ) | TypedManifestNativeInvocation::AccessControllerBlueprintInvocation(
                        AccessControllerBlueprintInvocation::Method(
                            AccessControllerBlueprintMethod::InitiateRecoveryAsRecovery(..)
                        )
                    ),
                }),
            ..
        } = context
        else {
            return;
        };

        let ac_address = ComponentAddress::try_from(*ac_address).expect(
            "Must succeed since the typed invocation conversion succeeded",
        );

        self.0.access_controllers.push(ac_address);
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
                // Fee Payment
                (
                    Some(GroupedEntityType::AccountEntities(..)),
                    ACCOUNT_CREATE_PROOF_OF_AMOUNT_IDENT
                    | ACCOUNT_CREATE_PROOF_OF_NON_FUNGIBLES_IDENT
                    | ACCOUNT_LOCK_FEE_IDENT,
                )
                | (
                    Some(GroupedEntityType::AccessControllerEntities(..)),
                    ACCESS_CONTROLLER_CREATE_PROOF_IDENT
                    | ACCESS_CONTROLLER_LOCK_RECOVERY_FEE_IDENT,
                ) => true,
                // Recovery idents - starting and confirming
                (
                    Some(GroupedEntityType::AccessControllerEntities(..)),
                    ACCESS_CONTROLLER_INITIATE_RECOVERY_AS_PRIMARY_IDENT
                    | ACCESS_CONTROLLER_INITIATE_RECOVERY_AS_RECOVERY_IDENT
                    | ACCESS_CONTROLLER_QUICK_CONFIRM_PRIMARY_ROLE_RECOVERY_PROPOSAL_IDENT
                    | ACCESS_CONTROLLER_QUICK_CONFIRM_RECOVERY_ROLE_RECOVERY_PROPOSAL_IDENT
                    // Allow cancelling an existing recovery proposal
                    | ACCESS_CONTROLLER_CANCEL_RECOVERY_ROLE_RECOVERY_PROPOSAL_IDENT
                    | ACCESS_CONTROLLER_STOP_TIMED_RECOVERY_IDENT,
                ) => true,
                _ => false,
            }
        }
        // Metadata updates are allowed when performing recovery
        GroupedInstruction::InvocationInstructions(
            InvocationInstructions::CallMetadataMethod(..),
        ) => true,
        _ => false,
    }
}
