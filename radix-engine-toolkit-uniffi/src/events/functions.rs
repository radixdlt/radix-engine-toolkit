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
    core_events_sbor_decode_to_native_event(&event_type_identifier.try_into()?, &event_data)
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
    pub primary_role: Arc<AccessRule>,
    pub recovery_role: Arc<AccessRule>,
    pub confirmation_role: Arc<AccessRule>,
}

#[derive(Clone, Debug, Record)]
pub struct RecoveryProposal {
    pub rule_set: RuleSet,
    pub timed_recovery_delay_in_minutes: Option<u32>,
}

impl FromNative for RuleSet {
    type Native = NativeRuleSet;

    fn from_native(value: NativeRuleSet) -> Self {
        Self {
            primary_role: Arc::new(AccessRule(value.primary_role)),
            recovery_role: Arc::new(AccessRule(value.recovery_role)),
            confirmation_role: Arc::new(AccessRule(value.confirmation_role)),
        }
    }
}

impl FromNative for RecoveryProposal {
    type Native = NativeRecoveryProposal;

    fn from_native(value: NativeRecoveryProposal) -> Self {
        Self {
            rule_set: <RuleSet as FromNative>::from_native(value.rule_set),
            timed_recovery_delay_in_minutes: value.timed_recovery_delay_in_minutes,
        }
    }
}

impl FromNative for Proposer {
    type Native = NativeProposer;

    fn from_native(value: NativeProposer) -> Self {
        match value {
            NativeProposer::Primary => Self::Primary,
            NativeProposer::Recovery => Self::Recovery,
        }
    }
}

impl FromNative for Role {
    type Native = NativeRole;

    fn from_native(value: NativeRole) -> Self {
        match value {
            NativeRole::Primary => Self::Primary,
            NativeRole::Recovery => Self::Recovery,
            NativeRole::Confirmation => Self::Confirmation,
        }
    }
}

impl FromNative for ValidatorInfo {
    type Native = NativeValidator;

    fn from_native(value: NativeValidator) -> Self {
        Self {
            key: value.key.into(),
            stake: Arc::new(Decimal(value.stake)),
        }
    }
}

impl FromNative for RecallResourceEvent {
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

impl FromNative for DepositResourceEvent {
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

impl FromNative for WithdrawResourceEvent {
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

impl FromNative for LockFeeEvent {
    type Native = NativeLockFeeEvent;

    fn from_native(value: NativeLockFeeEvent) -> Self {
        Self {
            amount: Arc::new(Decimal(value.amount)),
        }
    }
}

impl FromNative for MintFungibleResourceEvent {
    type Native = NativeMintFungibleResourceEvent;

    fn from_native(value: NativeMintFungibleResourceEvent) -> Self {
        Self {
            amount: Arc::new(Decimal(value.amount)),
        }
    }
}

impl FromNative for BurnFungibleResourceEvent {
    type Native = NativeBurnFungibleResourceEvent;

    fn from_native(value: NativeBurnFungibleResourceEvent) -> Self {
        Self {
            amount: Arc::new(Decimal(value.amount)),
        }
    }
}

impl FromNative for MintNonFungibleResourceEvent {
    type Native = NativeMintNonFungibleResourceEvent;

    fn from_native(value: NativeMintNonFungibleResourceEvent) -> Self {
        Self {
            ids: value.ids.into_iter().map(Into::into).collect(),
        }
    }
}

impl FromNative for BurnNonFungibleResourceEvent {
    type Native = NativeBurnNonFungibleResourceEvent;

    fn from_native(value: NativeBurnNonFungibleResourceEvent) -> Self {
        Self {
            ids: value.ids.into_iter().map(Into::into).collect(),
        }
    }
}

impl FromNativeWithNetworkContext for VaultCreationEvent {
    type Native = NativeVaultCreationEvent;

    fn from_native(native: Self::Native, network_id: u8) -> Self {
        Self {
            vault_id: Arc::new(Address(native.vault_id, network_id)),
        }
    }
}

impl FromNativeWithNetworkContext for MultiResourcePoolDepositEvent {
    type Native = NativeMultiResourcePoolDepositEvent;

    fn from_native(native: Self::Native, network_id: u8) -> Self {
        Self {
            resource_address: Arc::new(Address(native.resource_address.into_node_id(), network_id)),
            amount: Arc::new(Decimal(native.amount)),
        }
    }
}

impl FromNativeWithNetworkContext for MultiResourcePoolWithdrawEvent {
    type Native = NativeMultiResourcePoolWithdrawEvent;

    fn from_native(native: Self::Native, network_id: u8) -> Self {
        Self {
            resource_address: Arc::new(Address(native.resource_address.into_node_id(), network_id)),
            amount: Arc::new(Decimal(native.amount)),
        }
    }
}

impl FromNativeWithNetworkContext for MultiResourcePoolRedemptionEvent {
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

impl FromNativeWithNetworkContext for MultiResourcePoolContributionEvent {
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

impl FromNativeWithNetworkContext for TwoResourcePoolDepositEvent {
    type Native = NativeTwoResourcePoolDepositEvent;

    fn from_native(native: Self::Native, network_id: u8) -> Self {
        Self {
            resource_address: Arc::new(Address(native.resource_address.into_node_id(), network_id)),
            amount: Arc::new(Decimal(native.amount)),
        }
    }
}

impl FromNativeWithNetworkContext for TwoResourcePoolWithdrawEvent {
    type Native = NativeTwoResourcePoolWithdrawEvent;

    fn from_native(native: Self::Native, network_id: u8) -> Self {
        Self {
            resource_address: Arc::new(Address(native.resource_address.into_node_id(), network_id)),
            amount: Arc::new(Decimal(native.amount)),
        }
    }
}

impl FromNativeWithNetworkContext for TwoResourcePoolRedemptionEvent {
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

impl FromNativeWithNetworkContext for TwoResourcePoolContributionEvent {
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

impl FromNative for OneResourcePoolContributionEvent {
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

impl FromNative for OneResourcePoolRedemptionEvent {
    type Native = NativeOneResourcePoolRedemptionEvent;

    fn from_native(native: Self::Native) -> Self {
        Self {
            pool_unit_tokens_redeemed: Arc::new(Decimal(native.pool_unit_tokens_redeemed)),
            redeemed_amount: Arc::new(Decimal(native.redeemed_amount)),
        }
    }
}

impl FromNative for OneResourcePoolWithdrawEvent {
    type Native = NativeOneResourcePoolWithdrawEvent;

    fn from_native(native: Self::Native) -> Self {
        Self {
            amount: Arc::new(Decimal(native.amount)),
        }
    }
}

impl FromNative for OneResourcePoolDepositEvent {
    type Native = NativeOneResourcePoolDepositEvent;

    fn from_native(native: Self::Native) -> Self {
        Self {
            amount: Arc::new(Decimal(native.amount)),
        }
    }
}

impl FromNativeWithNetworkContext for EpochChangeEvent {
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
                        <ValidatorInfo as FromNative>::from_native(value),
                    )
                })
                .collect(),
        }
    }
}

impl FromNative for ValidatorRewardAppliedEvent {
    type Native = NativeValidatorRewardAppliedEvent;

    fn from_native(native: Self::Native) -> Self {
        Self {
            amount: Arc::new(Decimal(native.amount)),
            epoch: native.epoch.number(),
        }
    }
}

impl FromNative for ValidatorEmissionAppliedEvent {
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

impl FromNative for ProtocolUpdateReadinessSignalEvent {
    type Native = NativeProtocolUpdateReadinessSignalEvent;

    fn from_native(native: Self::Native) -> Self {
        Self {
            protocol_version_name: native.protocol_version_name,
        }
    }
}

impl FromNative for UpdateAcceptingStakeDelegationStateEvent {
    type Native = NativeUpdateAcceptingStakeDelegationStateEvent;

    fn from_native(native: Self::Native) -> Self {
        Self {
            accepts_delegation: native.accepts_delegation,
        }
    }
}

impl FromNative for ClaimXrdEvent {
    type Native = NativeClaimXrdEvent;

    fn from_native(native: Self::Native) -> Self {
        Self {
            claimed_xrd: Arc::new(Decimal(native.claimed_xrd)),
        }
    }
}

impl FromNative for UnstakeEvent {
    type Native = NativeUnstakeEvent;

    fn from_native(native: Self::Native) -> Self {
        Self {
            stake_units: Arc::new(Decimal(native.stake_units)),
        }
    }
}

impl FromNative for StakeEvent {
    type Native = NativeStakeEvent;

    fn from_native(native: Self::Native) -> Self {
        Self {
            xrd_staked: Arc::new(Decimal(native.xrd_staked)),
        }
    }
}

impl FromNative for UnregisterValidatorEvent {
    type Native = NativeUnregisterValidatorEvent;

    fn from_native(_: Self::Native) -> Self {
        Self {
            placeholder_field: true,
        }
    }
}

impl FromNative for RegisterValidatorEvent {
    type Native = NativeRegisterValidatorEvent;

    fn from_native(_: Self::Native) -> Self {
        Self {
            placeholder_field: true,
        }
    }
}

impl FromNative for RoundChangeEvent {
    type Native = NativeRoundChangeEvent;

    fn from_native(native: Self::Native) -> Self {
        Self {
            round: native.round.number(),
        }
    }
}

impl FromNative for StopTimedRecoveryEvent {
    type Native = NativeStopTimedRecoveryEvent;

    fn from_native(_: Self::Native) -> Self {
        Self {
            placeholder_field: true,
        }
    }
}

impl FromNative for UnlockPrimaryRoleEvent {
    type Native = NativeUnlockPrimaryRoleEvent;

    fn from_native(_: Self::Native) -> Self {
        Self {
            placeholder_field: true,
        }
    }
}

impl FromNative for LockPrimaryRoleEvent {
    type Native = NativeLockPrimaryRoleEvent;

    fn from_native(_: Self::Native) -> Self {
        Self {
            placeholder_field: true,
        }
    }
}

impl FromNative for CancelBadgeWithdrawAttemptEvent {
    type Native = NativeCancelBadgeWithdrawAttemptEvent;

    fn from_native(native: Self::Native) -> Self {
        Self {
            proposer: <Proposer as FromNative>::from_native(native.proposer),
        }
    }
}

impl FromNative for CancelRecoveryProposalEvent {
    type Native = NativeCancelRecoveryProposalEvent;

    fn from_native(native: Self::Native) -> Self {
        Self {
            proposer: <Proposer as FromNative>::from_native(native.proposer),
        }
    }
}

impl FromNative for BadgeWithdrawEvent {
    type Native = NativeBadgeWithdrawEvent;

    fn from_native(native: Self::Native) -> Self {
        Self {
            proposer: <Proposer as FromNative>::from_native(native.proposer),
        }
    }
}

impl FromNative for RuleSetUpdateEvent {
    type Native = NativeRuleSetUpdateEvent;

    fn from_native(native: Self::Native) -> Self {
        Self {
            proposer: <Proposer as FromNative>::from_native(native.proposer),
            proposal: <RecoveryProposal as FromNative>::from_native(native.proposal),
        }
    }
}

impl FromNative for InitiateBadgeWithdrawAttemptEvent {
    type Native = NativeInitiateBadgeWithdrawAttemptEvent;

    fn from_native(native: Self::Native) -> Self {
        Self {
            proposer: <Proposer as FromNative>::from_native(native.proposer),
        }
    }
}

impl FromNative for InitiateRecoveryEvent {
    type Native = NativeInitiateRecoveryEvent;

    fn from_native(native: Self::Native) -> Self {
        Self {
            proposer: <Proposer as FromNative>::from_native(native.proposer),
            proposal: <RecoveryProposal as FromNative>::from_native(native.proposal),
        }
    }
}

impl FromNative for SetRoleEvent {
    type Native = NativeSetRoleEvent;

    fn from_native(native: Self::Native) -> Self {
        Self {
            role_key: native.role_key.key,
            rule: native_scrypto_encode(&native.rule).unwrap(),
        }
    }
}

impl FromNative for LockRoleEvent {
    type Native = NativeLockRoleEvent;

    fn from_native(native: Self::Native) -> Self {
        Self {
            role_key: native.role_key.key,
        }
    }
}

impl FromNative for SetAndLockRoleEvent {
    type Native = NativeSetAndLockRoleEvent;

    fn from_native(native: Self::Native) -> Self {
        Self {
            role_key: native.role_key.key,
            rule: native_scrypto_encode(&native.rule).unwrap(),
        }
    }
}

impl FromNative for SetOwnerRoleEvent {
    type Native = NativeSetOwnerRoleEvent;

    fn from_native(native: Self::Native) -> Self {
        Self {
            rule: native_scrypto_encode(&native.rule).unwrap(),
        }
    }
}

impl FromNative for SetAndLockOwnerRoleEvent {
    type Native = NativeSetAndLockOwnerRoleEvent;

    fn from_native(native: Self::Native) -> Self {
        Self {
            rule: native_scrypto_encode(&native.rule).unwrap(),
        }
    }
}

impl FromNative for LockOwnerRoleEvent {
    type Native = NativeLockOwnerRoleEvent;

    fn from_native(_: Self::Native) -> Self {
        Self {
            placeholder_field: true,
        }
    }
}

impl FromNativeWithNetworkContext for SetMetadataEvent {
    type Native = NativeSetMetadataEvent;

    fn from_native(native: Self::Native, network_id: u8) -> Self {
        Self {
            key: native.key,
            value: MetadataValue::from_native(&native.value, network_id),
        }
    }
}

impl FromNative for RemoveMetadataEvent {
    type Native = NativeRemoveMetadataEvent;

    fn from_native(native: Self::Native) -> Self {
        Self { key: native.key }
    }
}

impl TryFrom<Emitter> for NativeEmitter {
    type Error = RadixEngineToolkitError;

    fn try_from(value: Emitter) -> Result<Self> {
        match value {
            Emitter::Function {
                address,
                blueprint_name,
            } => Ok(Self::Function(NativeBlueprintId::new(
                &NativePackageAddress::try_from(*address)?,
                blueprint_name,
            ))),
            Emitter::Method {
                address,
                object_module_id,
            } => Ok(Self::Method(address.0, object_module_id.into())),
        }
    }
}

impl TryFrom<EventTypeIdentifier> for NativeEventTypeIdentifier {
    type Error = RadixEngineToolkitError;

    fn try_from(value: EventTypeIdentifier) -> Result<Self> {
        Ok(Self(
            value.emitter.try_into()?,
            NativeTypePointer::Package(NativeTypeIdentifier(
                value.schema_hash.0,
                value.local_type_index.into(),
            )),
        ))
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

            impl FromNativeWithNetworkContext for TypedNativeEvent {
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
                                                value: <$event_ty as FromNativeWithNetworkContext>::from_native(event, network_id)
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
