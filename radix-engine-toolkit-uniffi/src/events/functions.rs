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

#![allow(unreachable_code)]

use crate::prelude::*;

#[uniffi::export]
pub fn sbor_decode_to_typed_native_event(
    event_type_identifier: EventTypeIdentifier,
    event_data: Vec<u8>,
    network_id: u8,
) -> Result<TypedNativeEvent> {
    core_events_sbor_decode_to_native_event(&event_type_identifier.into(), &event_data)
        .map(|typed_event| TypedNativeEvent::from_native(typed_event, network_id))
        .map_err(Into::into)
}

#[derive(Clone, Debug, Record)]
pub struct EventTypeIdentifier {
    pub emitter: Emitter,
    pub schema_hash: Arc<Hash>,
    pub local_type_index: LocalTypeIndex,
}

#[derive(Clone, Debug, Enum)]
pub enum Emitter {
    Function {
        address: Arc<Address>,
        object_module_id: ObjectModuleId,
        blueprint_name: String,
    },
    Method {
        address: Arc<Address>,
        object_module_id: ObjectModuleId,
    },
}

#[derive(Clone, Debug, Record)]
pub struct InitiateRecoveryEvent {
    pub proposer: Proposer,
    pub proposal: RecoveryProposal,
}

#[derive(Clone, Debug, Record)]
pub struct InitiateBadgeWithdrawAttemptEvent {
    pub proposer: Proposer,
}

#[derive(Clone, Debug, Record)]
pub struct RuleSetUpdateEvent {
    pub proposer: Proposer,
    pub proposal: RecoveryProposal,
}

#[derive(Clone, Debug, Record)]
pub struct BadgeWithdrawEvent {
    pub proposer: Proposer,
}

#[derive(Clone, Debug, Record)]
pub struct CancelRecoveryProposalEvent {
    pub proposer: Proposer,
}

#[derive(Clone, Debug, Record)]
pub struct CancelBadgeWithdrawAttemptEvent {
    pub proposer: Proposer,
}

#[derive(Clone, Debug, Record)]
pub struct LockPrimaryRoleEvent {
    pub placeholder_field: bool,
}

#[derive(Clone, Debug, Record)]
pub struct UnlockPrimaryRoleEvent {
    pub placeholder_field: bool,
}

#[derive(Clone, Debug, Record)]
pub struct StopTimedRecoveryEvent {
    pub placeholder_field: bool,
}

#[derive(Clone, Debug, Record)]
pub struct RoundChangeEvent {
    pub round: u64,
}

#[derive(Clone, Debug, Record)]
pub struct RegisterValidatorEvent {
    pub placeholder_field: bool,
}

#[derive(Clone, Debug, Record)]
pub struct UnregisterValidatorEvent {
    pub placeholder_field: bool,
}

#[derive(Clone, Debug, Record)]
pub struct StakeEvent {
    pub xrd_staked: Arc<Decimal>,
}

#[derive(Clone, Debug, Record)]
pub struct UnstakeEvent {
    pub stake_units: Arc<Decimal>,
}

#[derive(Clone, Debug, Record)]
pub struct ClaimXrdEvent {
    pub claimed_xrd: Arc<Decimal>,
}

#[derive(Clone, Debug, Record)]
pub struct UpdateAcceptingStakeDelegationStateEvent {
    pub accepts_delegation: bool,
}

#[derive(Clone, Debug, Record)]
pub struct ProtocolUpdateReadinessSignalEvent {
    pub protocol_version_name: String,
}

#[derive(Clone, Debug, Record)]
pub struct ValidatorEmissionAppliedEvent {
    pub epoch: u64,
    pub starting_stake_pool_xrd: Arc<Decimal>,
    pub stake_pool_added_xrd: Arc<Decimal>,
    pub total_stake_unit_supply: Arc<Decimal>,
    pub validator_fee_xrd: Arc<Decimal>,
    pub proposals_made: u64,
    pub proposals_missed: u64,
}

#[derive(Clone, Debug, Record)]
pub struct ValidatorRewardAppliedEvent {
    pub epoch: u64,
    pub amount: Arc<Decimal>,
}

#[derive(Clone, Debug, Record)]
pub struct EpochChangeEvent {
    pub epoch: u64,
    pub validator_set: HashMap<String, ValidatorInfo>,
}

#[derive(Clone, Debug, Record)]
pub struct OneResourcePoolContributionEvent {
    pub amount_of_resources_contributed: Arc<Decimal>,
    pub pool_units_minted: Arc<Decimal>,
}

#[derive(Clone, Debug, Record)]
pub struct OneResourcePoolRedemptionEvent {
    pub pool_unit_tokens_redeemed: Arc<Decimal>,
    pub redeemed_amount: Arc<Decimal>,
}

#[derive(Clone, Debug, Record)]
pub struct OneResourcePoolWithdrawEvent {
    pub amount: Arc<Decimal>,
}

#[derive(Clone, Debug, Record)]
pub struct OneResourcePoolDepositEvent {
    pub amount: Arc<Decimal>,
}

#[derive(Clone, Debug, Record)]
pub struct TwoResourcePoolContributionEvent {
    pub contributed_resources: HashMap<String, Arc<Decimal>>,
    pub pool_units_minted: Arc<Decimal>,
}

#[derive(Clone, Debug, Record)]
pub struct TwoResourcePoolRedemptionEvent {
    pub pool_unit_tokens_redeemed: Arc<Decimal>,
    pub redeemed_resources: HashMap<String, Arc<Decimal>>,
}

#[derive(Clone, Debug, Record)]
pub struct TwoResourcePoolWithdrawEvent {
    pub resource_address: Arc<Address>,
    pub amount: Arc<Decimal>,
}

#[derive(Clone, Debug, Record)]
pub struct TwoResourcePoolDepositEvent {
    pub resource_address: Arc<Address>,
    pub amount: Arc<Decimal>,
}

#[derive(Clone, Debug, Record)]
pub struct MultiResourcePoolContributionEvent {
    pub contributed_resources: HashMap<String, Arc<Decimal>>,
    pub pool_units_minted: Arc<Decimal>,
}

#[derive(Clone, Debug, Record)]
pub struct MultiResourcePoolRedemptionEvent {
    pub pool_unit_tokens_redeemed: Arc<Decimal>,
    pub redeemed_resources: HashMap<String, Arc<Decimal>>,
}

#[derive(Clone, Debug, Record)]
pub struct MultiResourcePoolWithdrawEvent {
    pub resource_address: Arc<Address>,
    pub amount: Arc<Decimal>,
}

#[derive(Clone, Debug, Record)]
pub struct MultiResourcePoolDepositEvent {
    pub resource_address: Arc<Address>,
    pub amount: Arc<Decimal>,
}

#[derive(Clone, Debug, Record)]
pub struct VaultCreationEvent {
    pub vault_id: Arc<Address>,
}

#[derive(Clone, Debug, Record)]
pub struct MintFungibleResourceEvent {
    pub amount: Arc<Decimal>,
}

#[derive(Clone, Debug, Record)]
pub struct BurnFungibleResourceEvent {
    pub amount: Arc<Decimal>,
}

#[derive(Clone, Debug, Record)]
pub struct MintNonFungibleResourceEvent {
    pub ids: Vec<NonFungibleLocalId>,
}

#[derive(Clone, Debug, Record)]
pub struct BurnNonFungibleResourceEvent {
    pub ids: Vec<NonFungibleLocalId>,
}

#[derive(Clone, Debug, Record)]
pub struct LockFeeEvent {
    pub amount: Arc<Decimal>,
}

#[derive(Clone, Debug, Enum)]
pub enum WithdrawResourceEvent {
    Amount { value: Arc<Decimal> },
    Ids { value: Vec<NonFungibleLocalId> },
}

#[derive(Clone, Debug, Enum)]
pub enum DepositResourceEvent {
    Amount { value: Arc<Decimal> },
    Ids { value: Vec<NonFungibleLocalId> },
}

#[derive(Clone, Debug, Enum)]
pub enum RecallResourceEvent {
    Amount { value: Arc<Decimal> },
    Ids { value: Vec<NonFungibleLocalId> },
}

#[derive(Clone, Debug, Record)]
pub struct SetRoleEvent {
    pub role_key: String,
    pub rule: Vec<u8>,
}

#[derive(Clone, Debug, Record)]
pub struct LockRoleEvent {
    pub role_key: String,
}

#[derive(Clone, Debug, Record)]
pub struct SetAndLockRoleEvent {
    pub role_key: String,
    pub rule: Vec<u8>,
}

#[derive(Clone, Debug, Record)]
pub struct SetOwnerRoleEvent {
    pub rule: Vec<u8>,
}

#[derive(Clone, Debug, Record)]
pub struct LockOwnerRoleEvent {
    pub placeholder_field: bool,
}

#[derive(Clone, Debug, Record)]
pub struct SetAndLockOwnerRoleEvent {
    pub rule: Vec<u8>,
}

#[derive(Clone, Debug, Record)]
pub struct SetMetadataEvent {
    pub key: String,
    pub value: MetadataValue,
}

#[derive(Clone, Debug, Record)]
pub struct RemoveMetadataEvent {
    pub key: String,
}

#[derive(Clone, Debug, Record)]
pub struct ValidatorInfo {
    pub key: Secp256k1PublicKey,
    pub stake: Arc<Decimal>,
}

#[derive(Clone, Debug, Enum)]
pub enum Role {
    Primary,
    Recovery,
    Confirmation,
}

#[derive(Clone, Debug, Enum)]
pub enum Proposer {
    Primary,
    Recovery,
}

#[derive(Clone, Debug, Record)]
pub struct RuleSet {
    pub primary_role: Vec<u8>,
    pub recovery_role: Vec<u8>,
    pub confirmation_role: Vec<u8>,
}

#[derive(Clone, Debug, Record)]
pub struct RecoveryProposal {
    pub rule_set: RuleSet,
    pub timed_recovery_delay_in_minutes: Option<u32>,
}

impl NativeConvertible for RuleSet {
    type Native = NativeRuleSet;

    fn from_native(value: NativeRuleSet) -> Self {
        Self {
            primary_role: native_scrypto_encode(&value.primary_role).unwrap(),
            recovery_role: native_scrypto_encode(&value.recovery_role).unwrap(),
            confirmation_role: native_scrypto_encode(&value.confirmation_role).unwrap(),
        }
    }
}

impl NativeConvertible for RecoveryProposal {
    type Native = NativeRecoveryProposal;

    fn from_native(value: NativeRecoveryProposal) -> Self {
        Self {
            rule_set: <RuleSet as NativeConvertible>::from_native(value.rule_set),
            timed_recovery_delay_in_minutes: value.timed_recovery_delay_in_minutes,
        }
    }
}

impl NativeConvertible for Proposer {
    type Native = NativeProposer;

    fn from_native(value: NativeProposer) -> Self {
        match value {
            NativeProposer::Primary => Self::Primary,
            NativeProposer::Recovery => Self::Recovery,
        }
    }
}

impl NativeConvertible for Role {
    type Native = NativeRole;

    fn from_native(value: NativeRole) -> Self {
        match value {
            NativeRole::Primary => Self::Primary,
            NativeRole::Recovery => Self::Recovery,
            NativeRole::Confirmation => Self::Confirmation,
        }
    }
}

impl NativeConvertible for ValidatorInfo {
    type Native = NativeValidator;

    fn from_native(value: NativeValidator) -> Self {
        Self {
            key: value.key.into(),
            stake: Arc::new(Decimal(value.stake)),
        }
    }
}

impl NativeConvertible for RecallResourceEvent {
    type Native = NativeRecallResourceEvent;

    fn from_native(value: NativeRecallResourceEvent) -> Self {
        match value {
            NativeRecallResourceEvent::Amount(value) => Self::Amount {
                value: Arc::new(Decimal(value)),
            },
            NativeRecallResourceEvent::Ids(value) => Self::Ids {
                value: value.into_iter().map(Into::into).collect(),
            },
        }
    }
}

impl NativeConvertible for DepositResourceEvent {
    type Native = NativeDepositResourceEvent;

    fn from_native(value: NativeDepositResourceEvent) -> Self {
        match value {
            NativeDepositResourceEvent::Amount(value) => Self::Amount {
                value: Arc::new(Decimal(value)),
            },
            NativeDepositResourceEvent::Ids(value) => Self::Ids {
                value: value.into_iter().map(Into::into).collect(),
            },
        }
    }
}

impl NativeConvertible for WithdrawResourceEvent {
    type Native = NativeWithdrawResourceEvent;

    fn from_native(value: NativeWithdrawResourceEvent) -> Self {
        match value {
            NativeWithdrawResourceEvent::Amount(value) => Self::Amount {
                value: Arc::new(Decimal(value)),
            },
            NativeWithdrawResourceEvent::Ids(value) => Self::Ids {
                value: value.into_iter().map(Into::into).collect(),
            },
        }
    }
}

impl NativeConvertible for LockFeeEvent {
    type Native = NativeLockFeeEvent;

    fn from_native(value: NativeLockFeeEvent) -> Self {
        Self {
            amount: Arc::new(Decimal(value.amount)),
        }
    }
}

impl NativeConvertible for MintFungibleResourceEvent {
    type Native = NativeMintFungibleResourceEvent;

    fn from_native(value: NativeMintFungibleResourceEvent) -> Self {
        Self {
            amount: Arc::new(Decimal(value.amount)),
        }
    }
}

impl NativeConvertible for BurnFungibleResourceEvent {
    type Native = NativeBurnFungibleResourceEvent;

    fn from_native(value: NativeBurnFungibleResourceEvent) -> Self {
        Self {
            amount: Arc::new(Decimal(value.amount)),
        }
    }
}

impl NativeConvertible for MintNonFungibleResourceEvent {
    type Native = NativeMintNonFungibleResourceEvent;

    fn from_native(value: NativeMintNonFungibleResourceEvent) -> Self {
        Self {
            ids: value.ids.into_iter().map(Into::into).collect(),
        }
    }
}

impl NativeConvertible for BurnNonFungibleResourceEvent {
    type Native = NativeBurnNonFungibleResourceEvent;

    fn from_native(value: NativeBurnNonFungibleResourceEvent) -> Self {
        Self {
            ids: value.ids.into_iter().map(Into::into).collect(),
        }
    }
}

impl NativeConvertibleWithNetworkContext for VaultCreationEvent {
    type Native = NativeVaultCreationEvent;

    fn from_native(native: Self::Native, network_id: u8) -> Self {
        Self {
            vault_id: Arc::new(Address(native.vault_id, network_id)),
        }
    }
}

impl NativeConvertibleWithNetworkContext for MultiResourcePoolDepositEvent {
    type Native = NativeMultiResourcePoolDepositEvent;

    fn from_native(native: Self::Native, network_id: u8) -> Self {
        Self {
            resource_address: Arc::new(Address(native.resource_address.into_node_id(), network_id)),
            amount: Arc::new(Decimal(native.amount)),
        }
    }
}

impl NativeConvertibleWithNetworkContext for MultiResourcePoolWithdrawEvent {
    type Native = NativeMultiResourcePoolWithdrawEvent;

    fn from_native(native: Self::Native, network_id: u8) -> Self {
        Self {
            resource_address: Arc::new(Address(native.resource_address.into_node_id(), network_id)),
            amount: Arc::new(Decimal(native.amount)),
        }
    }
}

impl NativeConvertibleWithNetworkContext for MultiResourcePoolRedemptionEvent {
    type Native = NativeMultiResourcePoolRedemptionEvent;

    fn from_native(native: Self::Native, network_id: u8) -> Self {
        Self {
            pool_unit_tokens_redeemed: Arc::new(Decimal(native.pool_unit_tokens_redeemed)),
            redeemed_resources: native
                .redeemed_resources
                .into_iter()
                .map(|(key, value)| {
                    (
                        Address(key.into_node_id(), network_id).as_str(),
                        Arc::new(Decimal(value)),
                    )
                })
                .collect(),
        }
    }
}

impl NativeConvertibleWithNetworkContext for MultiResourcePoolContributionEvent {
    type Native = NativeMultiResourcePoolContributionEvent;

    fn from_native(native: Self::Native, network_id: u8) -> Self {
        Self {
            pool_units_minted: Arc::new(Decimal(native.pool_units_minted)),
            contributed_resources: native
                .contributed_resources
                .into_iter()
                .map(|(key, value)| {
                    (
                        Address(key.into_node_id(), network_id).as_str(),
                        Arc::new(Decimal(value)),
                    )
                })
                .collect(),
        }
    }
}

impl NativeConvertibleWithNetworkContext for TwoResourcePoolDepositEvent {
    type Native = NativeTwoResourcePoolDepositEvent;

    fn from_native(native: Self::Native, network_id: u8) -> Self {
        Self {
            resource_address: Arc::new(Address(native.resource_address.into_node_id(), network_id)),
            amount: Arc::new(Decimal(native.amount)),
        }
    }
}

impl NativeConvertibleWithNetworkContext for TwoResourcePoolWithdrawEvent {
    type Native = NativeTwoResourcePoolWithdrawEvent;

    fn from_native(native: Self::Native, network_id: u8) -> Self {
        Self {
            resource_address: Arc::new(Address(native.resource_address.into_node_id(), network_id)),
            amount: Arc::new(Decimal(native.amount)),
        }
    }
}

impl NativeConvertibleWithNetworkContext for TwoResourcePoolRedemptionEvent {
    type Native = NativeTwoResourcePoolRedemptionEvent;

    fn from_native(native: Self::Native, network_id: u8) -> Self {
        Self {
            pool_unit_tokens_redeemed: Arc::new(Decimal(native.pool_unit_tokens_redeemed)),
            redeemed_resources: native
                .redeemed_resources
                .into_iter()
                .map(|(key, value)| {
                    (
                        Address(key.into_node_id(), network_id).as_str(),
                        Arc::new(Decimal(value)),
                    )
                })
                .collect(),
        }
    }
}

impl NativeConvertibleWithNetworkContext for TwoResourcePoolContributionEvent {
    type Native = NativeTwoResourcePoolContributionEvent;

    fn from_native(native: Self::Native, network_id: u8) -> Self {
        Self {
            pool_units_minted: Arc::new(Decimal(native.pool_units_minted)),
            contributed_resources: native
                .contributed_resources
                .into_iter()
                .map(|(key, value)| {
                    (
                        Address(key.into_node_id(), network_id).as_str(),
                        Arc::new(Decimal(value)),
                    )
                })
                .collect(),
        }
    }
}

impl NativeConvertible for OneResourcePoolContributionEvent {
    type Native = NativeOneResourcePoolContributionEvent;

    fn from_native(native: Self::Native) -> Self {
        Self {
            pool_units_minted: Arc::new(Decimal(native.pool_units_minted)),
            amount_of_resources_contributed: Arc::new(Decimal(
                native.amount_of_resources_contributed,
            )),
        }
    }
}

impl NativeConvertible for OneResourcePoolRedemptionEvent {
    type Native = NativeOneResourcePoolRedemptionEvent;

    fn from_native(native: Self::Native) -> Self {
        Self {
            pool_unit_tokens_redeemed: Arc::new(Decimal(native.pool_unit_tokens_redeemed)),
            redeemed_amount: Arc::new(Decimal(native.redeemed_amount)),
        }
    }
}

impl NativeConvertible for OneResourcePoolWithdrawEvent {
    type Native = NativeOneResourcePoolWithdrawEvent;

    fn from_native(native: Self::Native) -> Self {
        Self {
            amount: Arc::new(Decimal(native.amount)),
        }
    }
}

impl NativeConvertible for OneResourcePoolDepositEvent {
    type Native = NativeOneResourcePoolDepositEvent;

    fn from_native(native: Self::Native) -> Self {
        Self {
            amount: Arc::new(Decimal(native.amount)),
        }
    }
}

impl NativeConvertibleWithNetworkContext for EpochChangeEvent {
    type Native = NativeEpochChangeEvent;

    fn from_native(native: Self::Native, network_id: u8) -> Self {
        Self {
            epoch: native.epoch.number(),
            validator_set: native
                .validator_set
                .validators_by_stake_desc
                .into_iter()
                .map(|(key, value)| {
                    (
                        Address(key.into_node_id(), network_id).as_str(),
                        <ValidatorInfo as NativeConvertible>::from_native(value),
                    )
                })
                .collect(),
        }
    }
}

impl NativeConvertible for ValidatorRewardAppliedEvent {
    type Native = NativeValidatorRewardAppliedEvent;

    fn from_native(native: Self::Native) -> Self {
        Self {
            amount: Arc::new(Decimal(native.amount)),
            epoch: native.epoch.number(),
        }
    }
}

impl NativeConvertible for ValidatorEmissionAppliedEvent {
    type Native = NativeValidatorEmissionAppliedEvent;

    fn from_native(native: Self::Native) -> Self {
        Self {
            epoch: native.epoch.number(),
            starting_stake_pool_xrd: Arc::new(Decimal(native.starting_stake_pool_xrd)),
            stake_pool_added_xrd: Arc::new(Decimal(native.stake_pool_added_xrd)),
            total_stake_unit_supply: Arc::new(Decimal(native.total_stake_unit_supply)),
            validator_fee_xrd: Arc::new(Decimal(native.validator_fee_xrd)),
            proposals_made: native.proposals_made,
            proposals_missed: native.proposals_missed,
        }
    }
}

impl NativeConvertible for ProtocolUpdateReadinessSignalEvent {
    type Native = NativeProtocolUpdateReadinessSignalEvent;

    fn from_native(native: Self::Native) -> Self {
        Self {
            protocol_version_name: native.protocol_version_name,
        }
    }
}

impl NativeConvertible for UpdateAcceptingStakeDelegationStateEvent {
    type Native = NativeUpdateAcceptingStakeDelegationStateEvent;

    fn from_native(native: Self::Native) -> Self {
        Self {
            accepts_delegation: native.accepts_delegation,
        }
    }
}

impl NativeConvertible for ClaimXrdEvent {
    type Native = NativeClaimXrdEvent;

    fn from_native(native: Self::Native) -> Self {
        Self {
            claimed_xrd: Arc::new(Decimal(native.claimed_xrd)),
        }
    }
}

impl NativeConvertible for UnstakeEvent {
    type Native = NativeUnstakeEvent;

    fn from_native(native: Self::Native) -> Self {
        Self {
            stake_units: Arc::new(Decimal(native.stake_units)),
        }
    }
}

impl NativeConvertible for StakeEvent {
    type Native = NativeStakeEvent;

    fn from_native(native: Self::Native) -> Self {
        Self {
            xrd_staked: Arc::new(Decimal(native.xrd_staked)),
        }
    }
}

impl NativeConvertible for UnregisterValidatorEvent {
    type Native = NativeUnregisterValidatorEvent;

    fn from_native(_: Self::Native) -> Self {
        Self {
            placeholder_field: true,
        }
    }
}

impl NativeConvertible for RegisterValidatorEvent {
    type Native = NativeRegisterValidatorEvent;

    fn from_native(_: Self::Native) -> Self {
        Self {
            placeholder_field: true,
        }
    }
}

impl NativeConvertible for RoundChangeEvent {
    type Native = NativeRoundChangeEvent;

    fn from_native(native: Self::Native) -> Self {
        Self {
            round: native.round.number(),
        }
    }
}

impl NativeConvertible for StopTimedRecoveryEvent {
    type Native = NativeStopTimedRecoveryEvent;

    fn from_native(_: Self::Native) -> Self {
        Self {
            placeholder_field: true,
        }
    }
}

impl NativeConvertible for UnlockPrimaryRoleEvent {
    type Native = NativeUnlockPrimaryRoleEvent;

    fn from_native(_: Self::Native) -> Self {
        Self {
            placeholder_field: true,
        }
    }
}

impl NativeConvertible for LockPrimaryRoleEvent {
    type Native = NativeLockPrimaryRoleEvent;

    fn from_native(_: Self::Native) -> Self {
        Self {
            placeholder_field: true,
        }
    }
}

impl NativeConvertible for CancelBadgeWithdrawAttemptEvent {
    type Native = NativeCancelBadgeWithdrawAttemptEvent;

    fn from_native(native: Self::Native) -> Self {
        Self {
            proposer: <Proposer as NativeConvertible>::from_native(native.proposer),
        }
    }
}

impl NativeConvertible for CancelRecoveryProposalEvent {
    type Native = NativeCancelRecoveryProposalEvent;

    fn from_native(native: Self::Native) -> Self {
        Self {
            proposer: <Proposer as NativeConvertible>::from_native(native.proposer),
        }
    }
}

impl NativeConvertible for BadgeWithdrawEvent {
    type Native = NativeBadgeWithdrawEvent;

    fn from_native(native: Self::Native) -> Self {
        Self {
            proposer: <Proposer as NativeConvertible>::from_native(native.proposer),
        }
    }
}

impl NativeConvertible for RuleSetUpdateEvent {
    type Native = NativeRuleSetUpdateEvent;

    fn from_native(native: Self::Native) -> Self {
        Self {
            proposer: <Proposer as NativeConvertible>::from_native(native.proposer),
            proposal: <RecoveryProposal as NativeConvertible>::from_native(native.proposal),
        }
    }
}

impl NativeConvertible for InitiateBadgeWithdrawAttemptEvent {
    type Native = NativeInitiateBadgeWithdrawAttemptEvent;

    fn from_native(native: Self::Native) -> Self {
        Self {
            proposer: <Proposer as NativeConvertible>::from_native(native.proposer),
        }
    }
}

impl NativeConvertible for InitiateRecoveryEvent {
    type Native = NativeInitiateRecoveryEvent;

    fn from_native(native: Self::Native) -> Self {
        Self {
            proposer: <Proposer as NativeConvertible>::from_native(native.proposer),
            proposal: <RecoveryProposal as NativeConvertible>::from_native(native.proposal),
        }
    }
}

impl NativeConvertible for SetRoleEvent {
    type Native = NativeSetRoleEvent;

    fn from_native(native: Self::Native) -> Self {
        Self {
            role_key: native.role_key.key,
            rule: native_scrypto_encode(&native.rule).unwrap(),
        }
    }
}

impl NativeConvertible for LockRoleEvent {
    type Native = NativeLockRoleEvent;

    fn from_native(native: Self::Native) -> Self {
        Self {
            role_key: native.role_key.key,
        }
    }
}

impl NativeConvertible for SetAndLockRoleEvent {
    type Native = NativeSetAndLockRoleEvent;

    fn from_native(native: Self::Native) -> Self {
        Self {
            role_key: native.role_key.key,
            rule: native_scrypto_encode(&native.rule).unwrap(),
        }
    }
}

impl NativeConvertible for SetOwnerRoleEvent {
    type Native = NativeSetOwnerRoleEvent;

    fn from_native(native: Self::Native) -> Self {
        Self {
            rule: native_scrypto_encode(&native.rule).unwrap(),
        }
    }
}

impl NativeConvertible for SetAndLockOwnerRoleEvent {
    type Native = NativeSetAndLockOwnerRoleEvent;

    fn from_native(native: Self::Native) -> Self {
        Self {
            rule: native_scrypto_encode(&native.rule).unwrap(),
        }
    }
}

impl NativeConvertible for LockOwnerRoleEvent {
    type Native = NativeLockOwnerRoleEvent;

    fn from_native(_: Self::Native) -> Self {
        Self {
            placeholder_field: true,
        }
    }
}

impl NativeConvertibleWithNetworkContext for SetMetadataEvent {
    type Native = NativeSetMetadataEvent;

    fn from_native(native: Self::Native, network_id: u8) -> Self {
        Self {
            key: native.key,
            value: MetadataValue::from_native(&native.value, network_id),
        }
    }
}

impl NativeConvertible for RemoveMetadataEvent {
    type Native = NativeRemoveMetadataEvent;

    fn from_native(native: Self::Native) -> Self {
        Self { key: native.key }
    }
}

impl From<Emitter> for NativeEmitter {
    fn from(value: Emitter) -> Self {
        match value {
            Emitter::Function {
                address,
                object_module_id,
                blueprint_name,
            } => Self::Function(address.0, object_module_id.into(), blueprint_name),
            Emitter::Method {
                address,
                object_module_id,
            } => Self::Method(address.0, object_module_id.into()),
        }
    }
}

impl From<EventTypeIdentifier> for NativeEventTypeIdentifier {
    fn from(value: EventTypeIdentifier) -> Self {
        Self(
            value.emitter.into(),
            NativeTypePointer::Package(value.schema_hash.0, value.local_type_index.into()),
        )
    }
}

trait NativeConvertible {
    type Native;

    fn from_native(native: Self::Native) -> Self;
}

trait NativeConvertibleWithNetworkContext {
    type Native;

    fn from_native(native: Self::Native, network_id: u8) -> Self;
}

impl<T> NativeConvertibleWithNetworkContext for T
where
    T: NativeConvertible,
{
    type Native = <T as NativeConvertible>::Native;

    fn from_native(native: Self::Native, _: u8) -> Self {
        <T as NativeConvertible>::from_native(native)
    }
}

define_structure! {
    /* Native Packages */
    AccessController => {
        AccessController => [
            InitiateRecoveryEvent,
            InitiateBadgeWithdrawAttemptEvent,
            RuleSetUpdateEvent,
            BadgeWithdrawEvent,
            CancelRecoveryProposalEvent,
            CancelBadgeWithdrawAttemptEvent,
            LockPrimaryRoleEvent,
            UnlockPrimaryRoleEvent,
            StopTimedRecoveryEvent,
        ],
    },
    Account => {
        Account => []
    },
    Identity => {
        Identity => []
    },
    Package => {
        Package => []
    },
    ConsensusManager => {
        ConsensusManager => [
            RoundChangeEvent,
            EpochChangeEvent
        ],
        Validator => [
            RegisterValidatorEvent,
            UnregisterValidatorEvent,
            StakeEvent,
            UnstakeEvent,
            ClaimXrdEvent,
            UpdateAcceptingStakeDelegationStateEvent,
            ProtocolUpdateReadinessSignalEvent,
            ValidatorEmissionAppliedEvent,
            ValidatorRewardAppliedEvent,
        ],
    },
    Pool => {
        OneResourcePool => [
            OneResourcePoolContributionEvent,
            OneResourcePoolRedemptionEvent,
            OneResourcePoolWithdrawEvent,
            OneResourcePoolDepositEvent,
        ],
        TwoResourcePool => [
            TwoResourcePoolContributionEvent,
            TwoResourcePoolRedemptionEvent,
            TwoResourcePoolWithdrawEvent,
            TwoResourcePoolDepositEvent,
        ],
        MultiResourcePool => [
            MultiResourcePoolContributionEvent,
            MultiResourcePoolRedemptionEvent,
            MultiResourcePoolWithdrawEvent,
            MultiResourcePoolDepositEvent,
        ],
    },
    Resource => {
        FungibleVault => [
            LockFeeEvent,
            WithdrawResourceEvent,
            DepositResourceEvent,
            RecallResourceEvent,
        ],
        NonFungibleVault => [
            LockFeeEvent,
            WithdrawResourceEvent,
            DepositResourceEvent,
            RecallResourceEvent,
        ],
        FungibleResourceManager => [
            VaultCreationEvent,
            MintFungibleResourceEvent,
            BurnFungibleResourceEvent,
        ],
        NonFungibleResourceManager => [
            VaultCreationEvent,
            MintNonFungibleResourceEvent,
            BurnNonFungibleResourceEvent,
        ]
    },
    TransactionProcessor => {
        TransactionProcessor => []
    },
    TransactionTracker => {
        TransactionTracker => []
    },

    /* Node Module Packages */
    RoleAssignment => {
        RoleAssignment => [
            SetRoleEvent,
            LockRoleEvent,
            SetAndLockRoleEvent,
            SetOwnerRoleEvent,
            LockOwnerRoleEvent,
            SetAndLockOwnerRoleEvent,
        ]
    },
    Metadata => {
        Metadata => [
            SetMetadataEvent,
            RemoveMetadataEvent,
        ]
    },
    Royalty => {
        ComponentRoyalty => []
    },
}

/// This enum uses some special syntax to define the structure of events. This makes the code for
/// model definitions very compact, allows for very easy addition of more packages, blueprints or
/// events in the future, keeps various models all in sync, and implements various functions and
/// methods on appropriate types.
///
/// The syntax allowed for by this macro looks like the following:
/// ```no_run
/// define_structure! {
///     package_name1 => {
///         blueprint_name1 => [
///             Event1,
///             Event2,
///             Event3,
///         ],
///         blueprint_name2 => [
///             Event1,
///         ]
///     },
///     package_name2 => {
///         blueprint_name1 => [
///             Event1,
///         ],
///         blueprint_name2 => [
///             Event1,
///             Event2,
///         ]
///     }
/// }
/// ```
macro_rules! define_structure {
    (
        $(
            $package_ident: ident => {
                $(
                    $blueprint_ident: ident => [
                        $($event_ty: ty),* $(,)?
                    ]
                ),* $(,)?
            }
        ),* $(,)?
    ) => {
        paste::paste! {
            #[derive(Clone, Debug, Enum)]
            pub enum TypedNativeEvent {
                $(
                    $package_ident { value: [< Typed $package_ident PackageEvent >] },
                )*
            }

            impl NativeConvertibleWithNetworkContext for TypedNativeEvent {
                type Native = radix_engine_queries::typed_native_events::TypedNativeEvent;

                fn from_native(native: Self::Native, network_id: u8) -> Self {
                    match native {
                        $(
                            $(
                                $(
                                    radix_engine_queries::typed_native_events::TypedNativeEvent::$package_ident(
                                        radix_engine_queries::typed_native_events::[< Typed $package_ident PackageEvent >]::$blueprint_ident(
                                            radix_engine_queries::typed_native_events::[< Typed $blueprint_ident BlueprintEvent >]::$event_ty(event)
                                        )
                                    ) => TypedNativeEvent::$package_ident{
                                        value: [< Typed $package_ident PackageEvent >]::$blueprint_ident{
                                            value: [< Typed $blueprint_ident BlueprintEvent >]::[< $event_ty Value >] {
                                                value: <$event_ty as NativeConvertibleWithNetworkContext>::from_native(event, network_id)
                                            }
                                        }
                                    },
                                )*
                            )*
                        )*
                        // Can't get here!
                        _ => panic!("Can't get to this point")
                    }
                }
            }

            $(
                #[derive(Clone, Debug, Enum)]
                pub enum [< Typed $package_ident PackageEvent >] {
                    $(
                        $blueprint_ident { value: [< Typed $blueprint_ident BlueprintEvent >] },
                    )*
                }

                $(
                    #[derive(Clone, Debug, Enum)]
                    pub enum [< Typed $blueprint_ident BlueprintEvent >] {
                        $(
                            [< $event_ty Value >] { value: $event_ty },
                        )*
                    }
                )*
            )*
        }
    };
}
use define_structure;
