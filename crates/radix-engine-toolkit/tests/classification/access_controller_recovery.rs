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

use super::access_controller_utils::*;
use crate::prelude::*;

const RECOVERY: Option<ManifestClassification> =
    Some(ManifestClassification::AccessControllerRecovery);

/// Proposes `proposal` as `proposer` on `access_controller` and quick confirms
/// it.
fn recover(
    builder: ManifestBuilder,
    access_controller: ComponentAddress,
    proposer: Proposer,
    proposal: RecoveryProposal,
) -> ManifestBuilder {
    builder
        .initiate_recovery(access_controller, proposer, proposal.clone())
        .quick_confirm_recovery(access_controller, proposer, proposal)
}

fn addresses() -> (ComponentAddress, ComponentAddress, ComponentAddress) {
    let mut allocator = TestAddressAllocator::new();
    (
        allocator.new_account_address(),
        allocator.new_access_controller_address(),
        allocator.new_access_controller_address(),
    )
}

fn other(proposer: Proposer) -> Proposer {
    match proposer {
        Proposer::Primary => Proposer::Recovery,
        Proposer::Recovery => Proposer::Primary,
    }
}

#[test]
fn proposing_and_confirming_shows_the_rules_enacted() {
    for proposer in PROPOSERS {
        // Arrange
        let (mut ledger, account, access_controller) =
            ledger_with_access_controller();
        let manifest = recover(
            ManifestBuilder::new().lock_fee(account, 10),
            access_controller,
            proposer,
            proposal(new_rules(), Some(5)),
        )
        .build();

        // Act
        let class = detailed_access_controller_class(&mut ledger, manifest);

        // Assert
        assert_eq!(
            class,
            DetailedManifestClassification::AccessControllerRecovery(
                AccessControllerRecoveryOutput {
                    recovery_proposals: vec![
                        AccessControllerRecoveryProposal {
                            access_controller,
                            proposer,
                            rule_set: new_rules(),
                            timed_recovery_delay_in_minutes: Some(5),
                        }
                    ],
                }
            )
        );
    }
}

#[test]
fn fee_payment_is_permitted() {
    // Arrange
    let (mut ledger, account, access_controller) =
        ledger_with_access_controller();
    let manifest = recover(
        ManifestBuilder::new()
            .call_method(
                access_controller,
                ACCESS_CONTROLLER_LOCK_RECOVERY_FEE_IDENT,
                AccessControllerLockRecoveryFeeManifestInput {
                    amount: Decimal::one(),
                },
            )
            .lock_contingent_fee(account, 10),
        access_controller,
        Proposer::Recovery,
        proposal(allow_all(), None),
    )
    .build();

    // Act
    let class = detailed_access_controller_class(&mut ledger, manifest);

    // Assert
    assert!(matches!(
        class,
        DetailedManifestClassification::AccessControllerRecovery(..)
    ));
}

#[test]
fn recovering_several_access_controllers_is_recovery() {
    let (_, access_controller, other_access_controller) = addresses();
    let builder = recover(
        ManifestBuilder::new(),
        access_controller,
        Proposer::Primary,
        proposal(allow_all(), None),
    );
    let builder = recover(
        builder,
        other_access_controller,
        Proposer::Recovery,
        proposal(deny_all(), None),
    );
    assert_access_controller_class(builder.build(), RECOVERY);
}

/// A proposal left unconfirmed may never be enacted, or be enacted later
/// without the manifest's signers.
#[test]
fn leaving_a_proposal_unconfirmed_is_not_recovery() {
    let (_, access_controller, other_access_controller) = addresses();
    let builder = recover(
        ManifestBuilder::new(),
        access_controller,
        Proposer::Primary,
        proposal(allow_all(), None),
    );
    let manifest = builder
        .initiate_recovery(
            other_access_controller,
            Proposer::Primary,
            proposal(allow_all(), None),
        )
        .build();
    assert_access_controller_class(manifest, None);
}

/// The attack the class prevents: the rules enacted are confirmed before a
/// harmless proposal that would otherwise replace them in the output.
#[test]
fn confirming_a_recovery_and_proposing_another_is_not_recovery() {
    let (_, access_controller, _) = addresses();
    for proposer in PROPOSERS {
        let builder = recover(
            ManifestBuilder::new(),
            access_controller,
            proposer,
            proposal(deny_all(), None),
        );
        let manifest = builder
            .initiate_recovery(
                access_controller,
                proposer,
                proposal(allow_all(), None),
            )
            .build();
        assert_access_controller_class(manifest, None);
    }
}

#[test]
fn proposing_twice_on_the_same_access_controller_is_not_recovery() {
    let (_, access_controller, _) = addresses();
    for first in PROPOSERS {
        for second in PROPOSERS {
            let manifest = ManifestBuilder::new()
                .initiate_recovery(
                    access_controller,
                    first,
                    proposal(allow_all(), None),
                )
                .initiate_recovery(
                    access_controller,
                    second,
                    proposal(allow_all(), None),
                )
                .quick_confirm_recovery(
                    access_controller,
                    second,
                    proposal(allow_all(), None),
                )
                .build();
            assert_access_controller_class(manifest, None);
        }
    }
}

#[test]
fn confirming_anything_but_the_proposal_is_not_recovery() {
    let (_, access_controller, _) = addresses();
    let proposed = proposal(allow_all(), Some(10));
    for proposer in PROPOSERS {
        for (confirmed_proposer, confirmed) in [
            (proposer, proposal(deny_all(), Some(10))),
            (proposer, proposal(allow_all(), None)),
            (other(proposer), proposed.clone()),
        ] {
            let manifest = ManifestBuilder::new()
                .initiate_recovery(
                    access_controller,
                    proposer,
                    proposed.clone(),
                )
                .quick_confirm_recovery(
                    access_controller,
                    confirmed_proposer,
                    confirmed,
                )
                .build();
            assert_access_controller_class(manifest, None);
        }
    }
}

/// The confirmation confirms a recovery already on the ledger, not the one
/// proposed.
#[test]
fn confirming_before_proposing_is_not_recovery() {
    let (_, access_controller, _) = addresses();
    for proposer in PROPOSERS {
        let manifest = ManifestBuilder::new()
            .quick_confirm_recovery(
                access_controller,
                proposer,
                proposal(allow_all(), None),
            )
            .initiate_recovery(
                access_controller,
                proposer,
                proposal(allow_all(), None),
            )
            .build();
        assert_access_controller_class(manifest, None);
    }
}

#[test]
fn other_access_controller_calls_are_not_recovery() {
    let (_, access_controller, _) = addresses();
    let others: [fn(ManifestBuilder, ComponentAddress) -> ManifestBuilder; 6] = [
        |builder, access_controller| {
            builder.timed_confirm_recovery(
                access_controller,
                proposal(allow_all(), None),
            )
        },
        |builder, access_controller| {
            builder.stop_timed_recovery(
                access_controller,
                proposal(allow_all(), None),
            )
        },
        |builder, access_controller| {
            builder.cancel_recovery(access_controller, Proposer::Primary)
        },
        |builder, access_controller| {
            builder.cancel_recovery(access_controller, Proposer::Recovery)
        },
        |builder, access_controller| {
            builder.lock_primary_role(access_controller)
        },
        |builder, access_controller| {
            builder.unlock_primary_role(access_controller)
        },
    ];
    for other in others {
        for other_first in [true, false] {
            let mut builder = ManifestBuilder::new();
            if other_first {
                builder = other(builder, access_controller);
            }
            builder = recover(
                builder,
                access_controller,
                Proposer::Recovery,
                proposal(allow_all(), None),
            );
            if !other_first {
                builder = other(builder, access_controller);
            }
            assert_access_controller_class(builder.build(), None);
        }
    }
}

#[test]
fn topping_up_another_access_controller_is_not_recovery() {
    let (account, access_controller, other_access_controller) = addresses();
    let builder = recover(
        ManifestBuilder::new(),
        access_controller,
        Proposer::Recovery,
        proposal(allow_all(), None),
    );
    let manifest =
        top_up_recovery_fee(builder, account, other_access_controller).build();
    assert_access_controller_class(manifest, None);
}

/// A recovery of an access controller, with extra instructions added by
/// `extra`.
fn recovery_with(
    extra: impl FnOnce(ManifestBuilder, ComponentAddress) -> ManifestBuilder,
) -> TransactionManifestV1 {
    let (account, access_controller, _) = addresses();
    let builder = recover(
        ManifestBuilder::new(),
        access_controller,
        Proposer::Recovery,
        proposal(allow_all(), None),
    );
    extra(builder, account).build()
}

#[test]
fn withdrawing_xrd_without_topping_up_is_not_recovery() {
    let manifest = recovery_with(|builder, account| {
        builder
            .withdraw_from_account(account, XRD, 100)
            .try_deposit_entire_worktop_or_abort(account, None)
    });
    assert_access_controller_class(manifest, None);
}

#[test]
fn withdrawing_other_resources_is_not_recovery() {
    let manifest = recovery_with(|builder, account| {
        builder
            .withdraw_from_account(account, ACCOUNT_OWNER_BADGE, 1)
            .try_deposit_entire_worktop_or_abort(account, None)
    });
    assert_access_controller_class(manifest, None);
}

#[test]
fn removing_account_metadata_is_not_recovery() {
    let manifest = recovery_with(|builder, account| {
        builder.call_metadata_method(
            account,
            METADATA_REMOVE_IDENT,
            MetadataRemoveInput {
                key: "owner_keys".into(),
            },
        )
    });
    assert_access_controller_class(manifest, None);
}

#[test]
fn setting_resource_metadata_is_not_recovery() {
    let manifest = recovery_with(|builder, _| {
        builder.set_metadata(XRD, "name", "not important")
    });
    assert_access_controller_class(manifest, None);
}
