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

//! Permission checks shared by the access controller and entity securify
//! analyzers.

use crate::internal_prelude::*;

/// Whether the instruction is permitted when the manifest is a subintent.
///
/// `VERIFY_PARENT` is permitted, and so is `YIELD_TO_PARENT` as long as it
/// doesn't pass anything back to the parent. This keeps resources withdrawn in
/// the subintent from being handed to the parent. Transaction manifests can't
/// contain these instructions, so this doesn't affect them.
pub(super) fn is_permitted_subintent_instruction(
    context: InstructionContext<'_>,
) -> bool {
    match context.instruction() {
        GroupedInstruction::SubintentInstructions(
            SubintentInstructions::YieldToParent(YieldToParent { args }),
        ) => *args == ManifestValue::unit(),
        GroupedInstruction::SubintentInstructions(
            SubintentInstructions::VerifyParent(..),
        ) => true,
        _ => false,
    }
}

/// Whether the instruction sets a metadata field on an account or identity,
/// e.g. its `owner_keys`.
pub(super) fn is_account_or_identity_metadata_set(
    context: InstructionContext<'_>,
) -> bool {
    matches!(
        context,
        InstructionContext::InvocationInstruction {
            typed_native_invocation: Some(TypedNativeInvocation {
                receiver: ManifestInvocationReceiver::GlobalMethod(receiver),
                invocation:
                    TypedManifestNativeInvocation::MetadataBlueprintInvocation(
                        MetadataBlueprintInvocation::Method(
                            MetadataBlueprintMethod::Set(..),
                        ),
                    ),
            }),
            ..
        } if matches!(
            receiver.grouped_entity_type(),
            Some(
                GroupedEntityType::AccountEntities(..)
                    | GroupedEntityType::IdentityEntities(..)
            )
        )
    )
}

/// Whether the instruction withdraws XRD from an account.
pub(super) fn is_xrd_withdraw(context: InstructionContext<'_>) -> bool {
    matches!(
        context,
        InstructionContext::InvocationInstruction {
            typed_native_invocation: Some(TypedNativeInvocation {
                invocation:
                    TypedManifestNativeInvocation::AccountBlueprintInvocation(
                        AccountBlueprintInvocation::Method(
                            AccountBlueprintMethod::Withdraw(
                                AccountWithdrawManifestInput {
                                    resource_address:
                                        ManifestResourceAddress::Static(XRD),
                                    ..
                                },
                            ),
                        ),
                    ),
                ..
            }),
            ..
        }
    )
}

/// If the instruction contributes only XRD to the recovery fee vault of an
/// access controller, returns that access controller.
pub(super) fn xrd_recovery_fee_contribution_receiver<'a>(
    context: InstructionContext<'a>,
) -> Option<&'a ResolvedManifestAddress<GlobalAddress>> {
    let InstructionContext::InvocationInstruction {
        typed_native_invocation:
            Some(TypedNativeInvocation {
                receiver: ManifestInvocationReceiver::GlobalMethod(receiver),
                invocation:
                    TypedManifestNativeInvocation::AccessControllerBlueprintInvocation(
                        AccessControllerBlueprintInvocation::Method(
                            AccessControllerBlueprintMethod::ContributeRecoveryFee(..),
                        ),
                    ),
            }),
        static_analysis_invocation_io,
        ..
    } = context
    else {
        return None;
    };

    let specified_resources =
        static_analysis_invocation_io.input.specified_resources();
    let contributes_only_xrd = !specified_resources.is_empty()
        && specified_resources
            .keys()
            .all(|resource_address| resource_address == &XRD)
        && !static_analysis_invocation_io
            .input
            .unspecified_resources()
            .may_be_present();

    contributes_only_xrd.then_some(receiver)
}

/// Converts a rule set from a manifest into a typed rule set. This fails when
/// a rule refers to something that only exists in the manifest, such as a
/// named address, in which case the rule can't be shown to the user.
pub(super) fn typed_rule_set(rule_set: &ManifestRuleSet) -> Option<RuleSet> {
    let ManifestRuleSet {
        primary_role,
        recovery_role,
        confirmation_role,
    } = rule_set.clone();

    Some(RuleSet {
        primary_role: primary_role.try_into_typed().ok()?,
        recovery_role: recovery_role.try_into_typed().ok()?,
        confirmation_role: confirmation_role.try_into_typed().ok()?,
    })
}
