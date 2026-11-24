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

//! Checks shared by the access controller, recovery fee, entity securify and
//! securified entity creation analyzers.

use crate::internal_prelude::*;

/// Whether the instruction is permitted when the manifest is a subintent.
///
/// * `VERIFY_PARENT` is permitted.
/// * `YIELD_TO_PARENT` is permitted as long as it doesn't pass anything back
///   to the parent. This keeps resources withdrawn in the subintent from being
///   handed to the parent.
/// * `ASSERT_WORKTOP_IS_EMPTY` (an `ASSERT_WORKTOP_RESOURCES_ONLY` without
///   constraints) is permitted, since enclosed subintents start with it. It
///   can't move resources.
///
/// Transaction manifests can't contain the first two instructions, and the
/// assertion doesn't change what they do. One that starts with it is held to
/// the `SubintentRestrictions`, though.
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
        instruction => is_assert_worktop_is_empty(instruction),
    }
}

/// Whether the instruction is an `ASSERT_WORKTOP_IS_EMPTY`: an
/// `ASSERT_WORKTOP_RESOURCES_ONLY` without constraints.
fn is_assert_worktop_is_empty(instruction: &GroupedInstruction) -> bool {
    matches!(
        instruction,
        GroupedInstruction::AssertionInstructions(
            AssertionInstructions::AssertWorktopResourcesOnly(
                AssertWorktopResourcesOnly { constraints },
            ),
        ) if constraints.specified_resources().is_empty()
    )
}

/// What a subintent of the access controller and recovery fee classes may not
/// do, on top of their permitted instructions:
///
/// * Pay the fee. The transaction a subintent is part of is paid for by
///   whoever submits it, and in a subintent a fee lock either fails, or is a
///   contingent one: a payment of up to the whole transaction's cost, which
///   the parent decides and the class's output doesn't show.
/// * Hand control back to the parent before its last instruction. The parent
///   would run its own instructions between the subintent's.
///
/// A manifest is known to be a subintent once it yields to or verifies its
/// parent, which every subintent does and a transaction manifest can't. It's
/// also taken to be one when it starts with `ASSERT_WORKTOP_IS_EMPTY`: that's
/// how an enclosed subintent starts, and all that's left to tell it by once
/// `as_enclosed` has turned it into a transaction manifest to preview, without
/// its `YIELD_TO_PARENT`. So transaction manifests that don't start with that
/// assertion, such as those the wallet builds, may pay the fee.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub(super) struct SubintentRestrictions {
    is_subintent: bool,
    pays_fee: bool,
    yielded_to_parent: bool,
    acts_after_yielding: bool,
}

impl SubintentRestrictions {
    pub(super) fn process_instruction(
        &mut self,
        context: InstructionContext<'_>,
        is_fee_payment: bool,
    ) {
        match context.instruction() {
            GroupedInstruction::SubintentInstructions(
                SubintentInstructions::YieldToParent(..),
            ) => {
                self.is_subintent = true;
                self.yielded_to_parent = true;
            }
            GroupedInstruction::SubintentInstructions(
                SubintentInstructions::VerifyParent(..),
            ) => {
                self.is_subintent = true;
                self.acts_after_yielding |= self.yielded_to_parent;
            }
            instruction => {
                self.is_subintent |= *context.instruction_index().value() == 0
                    && is_assert_worktop_is_empty(instruction);
                self.acts_after_yielding |= self.yielded_to_parent;
            }
        }
        self.pays_fee |= is_fee_payment;
    }

    /// Whether the manifest does something it may not, which nothing later in
    /// it can undo.
    pub(super) fn are_violated(&self) -> bool {
        self.is_subintent && (self.pays_fee || self.acts_after_yielding)
    }
}

/// Whether the resources are known to be only XRD.
pub(super) fn is_only_xrd(resources: &TrackedResources) -> bool {
    let specified_resources = resources.specified_resources();
    !specified_resources.is_empty()
        && specified_resources
            .keys()
            .all(|resource_address| resource_address == &XRD)
        && !resources.unspecified_resources().may_be_present()
}

/// Locking the fee, and creating the proofs it or the access controller calls
/// need: of an access controller's badge when the fee payer is securified, or
/// of badges an access controller's roles require.
pub(super) fn is_fee_payment(context: InstructionContext<'_>) -> bool {
    matches!(
        context,
        InstructionContext::InvocationInstruction {
            typed_native_invocation: Some(TypedNativeInvocation {
                receiver: ManifestInvocationReceiver::GlobalMethod(..),
                invocation:
                    TypedManifestNativeInvocation::AccountBlueprintInvocation(
                        AccountBlueprintInvocation::Method(
                            AccountBlueprintMethod::LockFee(..)
                            | AccountBlueprintMethod::LockContingentFee(..)
                            | AccountBlueprintMethod::CreateProofOfAmount(..)
                            | AccountBlueprintMethod::CreateProofOfNonFungibles(..),
                        ),
                    )
                    | TypedManifestNativeInvocation::AccessControllerBlueprintInvocation(
                        AccessControllerBlueprintInvocation::Method(
                            AccessControllerBlueprintMethod::CreateProof(..)
                            | AccessControllerBlueprintMethod::LockRecoveryFee(..),
                        ),
                    ),
            }),
            ..
        }
    )
}

/// The component at the address, if the address is known outside the
/// manifest.
pub(super) fn static_component(
    address: &ResolvedManifestAddress<GlobalAddress>,
) -> Option<ComponentAddress> {
    let ResolvedManifestAddress::Static { static_address } = address else {
        return None;
    };
    ComponentAddress::try_from(*static_address).ok()
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

/// The one account or identity owner badge in the invocation IO, unless there
/// are none or several.
pub(super) fn owner_badge_local_id(
    invocation_io_items: &InvocationIoItems,
) -> Option<&NonFungibleLocalId> {
    let mut owner_badge_ids = invocation_io_items
        .items_iter()
        .filter_map(|item| match item {
            InvocationIoItem::NonFungible(resource_address, ids)
                if resource_address == &ACCOUNT_OWNER_BADGE
                    || resource_address == &IDENTITY_OWNER_BADGE =>
            {
                Some(ids)
            }
            _ => None,
        })
        .flat_map(|ids| ids.iter());

    let owner_badge_id = owner_badge_ids.next()?;
    owner_badge_ids.next().is_none().then_some(owner_badge_id)
}

/// Whether the resources are known to be exactly one account or identity owner
/// badge, and nothing else.
pub(super) fn is_one_owner_badge(resources: &TrackedResources) -> bool {
    let mut specified_resources = resources.specified_resources().iter();
    let (Some((resource_address, resource)), None) =
        (specified_resources.next(), specified_resources.next())
    else {
        return false;
    };

    (resource_address == &ACCOUNT_OWNER_BADGE
        || resource_address == &IDENTITY_OWNER_BADGE)
        && resource.bounds().get_exact_amount() == Some(Decimal::ONE)
        && !resources.unspecified_resources().may_be_present()
}
