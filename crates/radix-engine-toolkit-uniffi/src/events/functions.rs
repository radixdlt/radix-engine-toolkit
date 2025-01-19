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
pub fn scrypto_sbor_decode_to_native_event(
    event_type_identifier: EventTypeIdentifier,
    event_data: Vec<u8>,
    network_id: u8,
) -> Result<TypedNativeEvent> {
    toolkit::functions::events::scrypto_sbor_decode_to_native_event(
        &event_type_identifier.try_into()?,
        &event_data,
    )
    .map(|typed_event| TypedNativeEvent::from_native(typed_event, network_id))
    .map_err(Into::into)
}

#[derive(Clone, Debug, Record)]
pub struct EventTypeIdentifier {
    pub emitter: Emitter,
    pub event_name: String,
}

#[derive(Clone, Debug, Enum)]
pub enum Emitter {
    Function {
        address: Arc<Address>,
        blueprint_name: String,
    },
    Method {
        address: Arc<Address>,
        object_module_id: ModuleId,
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
pub struct DepositRecoveryXrdEvent {
    pub amount: Arc<Decimal>,
}

#[derive(Clone, Debug, Record)]
pub struct WithdrawRecoveryXrdEvent {
    pub amount: Arc<Decimal>,
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
    pub rule: Arc<AccessRule>,
}

#[derive(Clone, Debug, Record)]
pub struct LockRoleEvent {
    pub role_key: String,
}

#[derive(Clone, Debug, Record)]
pub struct SetAndLockRoleEvent {
    pub role_key: String,
    pub rule: Arc<AccessRule>,
}

#[derive(Clone, Debug, Record)]
pub struct SetOwnerRoleEvent {
    pub rule: Arc<AccessRule>,
}

#[derive(Clone, Debug, Record)]
pub struct LockOwnerRoleEvent {
    pub placeholder_field: bool,
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

#[derive(Clone, Debug, Enum)]
pub enum AccountWithdrawEvent {
    Fungible {
        resource_address: Arc<Address>,
        amount: Arc<Decimal>,
    },
    NonFungible {
        resource_address: Arc<Address>,
        ids: Vec<NonFungibleLocalId>,
    },
}

#[derive(Clone, Debug, Enum)]
pub enum AccountDepositEvent {
    Fungible {
        resource_address: Arc<Address>,
        amount: Arc<Decimal>,
    },
    NonFungible {
        resource_address: Arc<Address>,
        ids: Vec<NonFungibleLocalId>,
    },
}

#[derive(Clone, Debug, Enum)]
pub enum AccountRejectedDepositEvent {
    Fungible {
        resource_address: Arc<Address>,
        amount: Arc<Decimal>,
    },
    NonFungible {
        resource_address: Arc<Address>,
        ids: Vec<NonFungibleLocalId>,
    },
}

#[derive(Clone, Debug, Record)]
pub struct AccountSetResourcePreferenceEvent {
    pub resource_address: Arc<Address>,
    pub preference: ResourcePreference,
}

#[derive(Clone, Debug, Record)]
pub struct AccountRemoveResourcePreferenceEvent {
    pub resource_address: Arc<Address>,
}

#[derive(Clone, Debug, Record)]
pub struct AccountSetDefaultDepositRuleEvent {
    pub default_deposit_rule: AccountDefaultDepositRule,
}

#[derive(Clone, Debug, Record)]
pub struct AccountAddAuthorizedDepositorEvent {
    pub authorized_depositor_badge: ResourceOrNonFungible,
}

#[derive(Clone, Debug, Record)]
pub struct AccountRemoveAuthorizedDepositorEvent {
    pub authorized_depositor_badge: ResourceOrNonFungible,
}

#[derive(Clone, Debug, Record)]
pub struct FungibleVaultLockFeeEvent {
    pub amount: Arc<Decimal>,
}

#[derive(Clone, Debug, Record)]
pub struct FungibleVaultWithdrawEvent {
    pub amount: Arc<Decimal>,
}

#[derive(Clone, Debug, Record)]
pub struct FungibleVaultDepositEvent {
    pub amount: Arc<Decimal>,
}

#[derive(Clone, Debug, Record)]
pub struct FungibleVaultRecallEvent {
    pub amount: Arc<Decimal>,
}

#[derive(Clone, Debug, Record)]
pub struct FungibleVaultPayFeeEvent {
    pub amount: Arc<Decimal>,
}

#[derive(Clone, Debug, Record)]
pub struct NonFungibleVaultWithdrawEvent {
    pub ids: Vec<NonFungibleLocalId>,
}

#[derive(Clone, Debug, Record)]
pub struct NonFungibleVaultDepositEvent {
    pub ids: Vec<NonFungibleLocalId>,
}

#[derive(Clone, Debug, Record)]
pub struct NonFungibleVaultRecallEvent {
    pub ids: Vec<NonFungibleLocalId>,
}

#[derive(Clone, Debug, Record)]
pub struct ClaimEvent {
    pub claimant: Arc<Address>,
    pub resource_address: Arc<Address>,
    pub resources: ResourceSpecifier,
}

#[derive(Clone, Debug, Record)]
pub struct RecoverEvent {
    pub claimant: Arc<Address>,
    pub resource_address: Arc<Address>,
    pub resources: ResourceSpecifier,
}

#[derive(Clone, Debug, Record)]
pub struct StoreEvent {
    pub claimant: Arc<Address>,
    pub resource_address: Arc<Address>,
    pub resources: ResourceSpecifier,
}

impl FromNative for RuleSet {
    type Native = engine::RuleSet;

    fn from_native(value: engine::RuleSet) -> Self {
        Self {
            primary_role: Arc::new(AccessRule(value.primary_role)),
            recovery_role: Arc::new(AccessRule(value.recovery_role)),
            confirmation_role: Arc::new(AccessRule(value.confirmation_role)),
        }
    }
}

impl ToNative for RuleSet {
    type Native = engine::RuleSet;

    fn to_native(self) -> Result<Self::Native> {
        Ok(Self::Native {
            primary_role: self.primary_role.0.clone(),
            recovery_role: self.recovery_role.0.clone(),
            confirmation_role: self.confirmation_role.0.clone(),
        })
    }
}

impl FromNative for RecoveryProposal {
    type Native = engine::RecoveryProposal;

    fn from_native(value: engine::RecoveryProposal) -> Self {
        Self {
            rule_set: <RuleSet as FromNative>::from_native(value.rule_set),
            timed_recovery_delay_in_minutes: value
                .timed_recovery_delay_in_minutes,
        }
    }
}

impl FromNative for Proposer {
    type Native = engine::Proposer;

    fn from_native(value: engine::Proposer) -> Self {
        match value {
            engine::Proposer::Primary => Self::Primary,
            engine::Proposer::Recovery => Self::Recovery,
        }
    }
}

impl FromNative for Role {
    type Native = engine::Role;

    fn from_native(value: engine::Role) -> Self {
        match value {
            engine::Role::Primary => Self::Primary,
            engine::Role::Recovery => Self::Recovery,
            engine::Role::Confirmation => Self::Confirmation,
        }
    }
}

impl FromNative for ValidatorInfo {
    type Native = engine::Validator;

    fn from_native(value: engine::Validator) -> Self {
        Self {
            key: value.key.into(),
            stake: Arc::new(Decimal(value.stake)),
        }
    }
}

impl FromNative for MintFungibleResourceEvent {
    type Native = engine::MintFungibleResourceEvent;

    fn from_native(value: engine::MintFungibleResourceEvent) -> Self {
        Self {
            amount: Arc::new(Decimal(value.amount)),
        }
    }
}

impl FromNative for BurnFungibleResourceEvent {
    type Native = engine::BurnFungibleResourceEvent;

    fn from_native(value: engine::BurnFungibleResourceEvent) -> Self {
        Self {
            amount: Arc::new(Decimal(value.amount)),
        }
    }
}

impl FromNative for MintNonFungibleResourceEvent {
    type Native = engine::MintNonFungibleResourceEvent;

    fn from_native(value: engine::MintNonFungibleResourceEvent) -> Self {
        Self {
            ids: value.ids.into_iter().map(Into::into).collect(),
        }
    }
}

impl FromNative for BurnNonFungibleResourceEvent {
    type Native = engine::BurnNonFungibleResourceEvent;

    fn from_native(value: engine::BurnNonFungibleResourceEvent) -> Self {
        Self {
            ids: value.ids.into_iter().map(Into::into).collect(),
        }
    }
}

impl FromNativeWithNetworkContext for VaultCreationEvent {
    type Native = engine::VaultCreationEvent;

    fn from_native(native: Self::Native, network_id: u8) -> Self {
        Self {
            vault_id: Arc::new(Address::from_node_id(
                engine::InternalAddress::try_from(native.vault_id)
                    .expect("Should be valid"),
                network_id,
            )),
        }
    }
}

impl FromNativeWithNetworkContext for MultiResourcePoolDepositEvent {
    type Native =
        radix_engine::blueprints::pool::v1::events::multi_resource_pool::DepositEvent;

    fn from_native(native: Self::Native, network_id: u8) -> Self {
        Self {
            resource_address: Arc::new(Address::from_node_id(
                native.resource_address,
                network_id,
            )),
            amount: Arc::new(Decimal(native.amount)),
        }
    }
}

impl FromNativeWithNetworkContext for MultiResourcePoolWithdrawEvent {
    type Native =
        radix_engine::blueprints::pool::v1::events::multi_resource_pool::WithdrawEvent;

    fn from_native(native: Self::Native, network_id: u8) -> Self {
        Self {
            resource_address: Arc::new(Address::from_node_id(
                native.resource_address,
                network_id,
            )),
            amount: Arc::new(Decimal(native.amount)),
        }
    }
}

impl FromNativeWithNetworkContext for MultiResourcePoolRedemptionEvent {
    type Native =
        radix_engine::blueprints::pool::v1::events::multi_resource_pool::RedemptionEvent;

    fn from_native(native: Self::Native, network_id: u8) -> Self {
        Self {
            pool_unit_tokens_redeemed: Arc::new(Decimal(
                native.pool_unit_tokens_redeemed,
            )),
            redeemed_resources: native
                .redeemed_resources
                .into_iter()
                .map(|(key, value)| {
                    (
                        Address::from_node_id(key, network_id).as_str(),
                        Arc::new(Decimal(value)),
                    )
                })
                .collect(),
        }
    }
}

impl FromNativeWithNetworkContext for MultiResourcePoolContributionEvent {
    type Native =
        radix_engine::blueprints::pool::v1::events::multi_resource_pool::ContributionEvent;

    fn from_native(native: Self::Native, network_id: u8) -> Self {
        Self {
            pool_units_minted: Arc::new(Decimal(native.pool_units_minted)),
            contributed_resources: native
                .contributed_resources
                .into_iter()
                .map(|(key, value)| {
                    (
                        Address::from_node_id(key, network_id).as_str(),
                        Arc::new(Decimal(value)),
                    )
                })
                .collect(),
        }
    }
}

impl FromNativeWithNetworkContext for TwoResourcePoolDepositEvent {
    type Native =
        radix_engine::blueprints::pool::v1::events::two_resource_pool::DepositEvent;

    fn from_native(native: Self::Native, network_id: u8) -> Self {
        Self {
            resource_address: Arc::new(Address::from_node_id(
                native.resource_address,
                network_id,
            )),
            amount: Arc::new(Decimal(native.amount)),
        }
    }
}

impl FromNativeWithNetworkContext for TwoResourcePoolWithdrawEvent {
    type Native =
        radix_engine::blueprints::pool::v1::events::two_resource_pool::WithdrawEvent;

    fn from_native(native: Self::Native, network_id: u8) -> Self {
        Self {
            resource_address: Arc::new(Address::from_node_id(
                native.resource_address,
                network_id,
            )),
            amount: Arc::new(Decimal(native.amount)),
        }
    }
}

impl FromNativeWithNetworkContext for TwoResourcePoolRedemptionEvent {
    type Native =
        radix_engine::blueprints::pool::v1::events::two_resource_pool::RedemptionEvent;

    fn from_native(native: Self::Native, network_id: u8) -> Self {
        Self {
            pool_unit_tokens_redeemed: Arc::new(Decimal(
                native.pool_unit_tokens_redeemed,
            )),
            redeemed_resources: native
                .redeemed_resources
                .into_iter()
                .map(|(key, value)| {
                    (
                        Address::from_node_id(key, network_id).as_str(),
                        Arc::new(Decimal(value)),
                    )
                })
                .collect(),
        }
    }
}

impl FromNativeWithNetworkContext for TwoResourcePoolContributionEvent {
    type Native =
        radix_engine::blueprints::pool::v1::events::two_resource_pool::ContributionEvent;

    fn from_native(native: Self::Native, network_id: u8) -> Self {
        Self {
            pool_units_minted: Arc::new(Decimal(native.pool_units_minted)),
            contributed_resources: native
                .contributed_resources
                .into_iter()
                .map(|(key, value)| {
                    (
                        Address::from_node_id(key, network_id).as_str(),
                        Arc::new(Decimal(value)),
                    )
                })
                .collect(),
        }
    }
}

impl FromNative for OneResourcePoolContributionEvent {
    type Native =
        radix_engine::blueprints::pool::v1::events::one_resource_pool::ContributionEvent;

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
    type Native =
        radix_engine::blueprints::pool::v1::events::one_resource_pool::RedemptionEvent;

    fn from_native(native: Self::Native) -> Self {
        Self {
            pool_unit_tokens_redeemed: Arc::new(Decimal(
                native.pool_unit_tokens_redeemed,
            )),
            redeemed_amount: Arc::new(Decimal(native.redeemed_amount)),
        }
    }
}

impl FromNative for OneResourcePoolWithdrawEvent {
    type Native =
        radix_engine::blueprints::pool::v1::events::one_resource_pool::WithdrawEvent;

    fn from_native(native: Self::Native) -> Self {
        Self {
            amount: Arc::new(Decimal(native.amount)),
        }
    }
}

impl FromNative for OneResourcePoolDepositEvent {
    type Native =
        radix_engine::blueprints::pool::v1::events::one_resource_pool::DepositEvent;

    fn from_native(native: Self::Native) -> Self {
        Self {
            amount: Arc::new(Decimal(native.amount)),
        }
    }
}

impl FromNativeWithNetworkContext for EpochChangeEvent {
    type Native = engine::EpochChangeEvent;

    fn from_native(native: Self::Native, network_id: u8) -> Self {
        Self {
            epoch: native.epoch.number(),
            validator_set: native
                .validator_set
                .validators_by_stake_desc
                .into_iter()
                .map(|(key, value)| {
                    (
                        Address::from_node_id(key, network_id).as_str(),
                        <ValidatorInfo as FromNative>::from_native(value),
                    )
                })
                .collect(),
        }
    }
}

impl FromNative for ValidatorRewardAppliedEvent {
    type Native = engine::ValidatorRewardAppliedEvent;

    fn from_native(native: Self::Native) -> Self {
        Self {
            amount: Arc::new(Decimal(native.amount)),
            epoch: native.epoch.number(),
        }
    }
}

impl FromNative for ValidatorEmissionAppliedEvent {
    type Native = engine::ValidatorEmissionAppliedEvent;

    fn from_native(native: Self::Native) -> Self {
        Self {
            epoch: native.epoch.number(),
            starting_stake_pool_xrd: Arc::new(Decimal(
                native.starting_stake_pool_xrd,
            )),
            stake_pool_added_xrd: Arc::new(Decimal(
                native.stake_pool_added_xrd,
            )),
            total_stake_unit_supply: Arc::new(Decimal(
                native.total_stake_unit_supply,
            )),
            validator_fee_xrd: Arc::new(Decimal(native.validator_fee_xrd)),
            proposals_made: native.proposals_made,
            proposals_missed: native.proposals_missed,
        }
    }
}

impl FromNative for ProtocolUpdateReadinessSignalEvent {
    type Native = engine::ProtocolUpdateReadinessSignalEvent;

    fn from_native(native: Self::Native) -> Self {
        Self {
            protocol_version_name: native.protocol_version_name,
        }
    }
}

impl FromNative for UpdateAcceptingStakeDelegationStateEvent {
    type Native = engine::UpdateAcceptingStakeDelegationStateEvent;

    fn from_native(native: Self::Native) -> Self {
        Self {
            accepts_delegation: native.accepts_delegation,
        }
    }
}

impl FromNative for ClaimXrdEvent {
    type Native = engine::ClaimXrdEvent;

    fn from_native(native: Self::Native) -> Self {
        Self {
            claimed_xrd: Arc::new(Decimal(native.claimed_xrd)),
        }
    }
}

impl FromNative for UnstakeEvent {
    type Native = engine::UnstakeEvent;

    fn from_native(native: Self::Native) -> Self {
        Self {
            stake_units: Arc::new(Decimal(native.stake_units)),
        }
    }
}

impl FromNative for StakeEvent {
    type Native = engine::StakeEvent;

    fn from_native(native: Self::Native) -> Self {
        Self {
            xrd_staked: Arc::new(Decimal(native.xrd_staked)),
        }
    }
}

impl FromNative for UnregisterValidatorEvent {
    type Native = engine::UnregisterValidatorEvent;

    fn from_native(_: Self::Native) -> Self {
        Self {
            placeholder_field: true,
        }
    }
}

impl FromNative for RegisterValidatorEvent {
    type Native = engine::RegisterValidatorEvent;

    fn from_native(_: Self::Native) -> Self {
        Self {
            placeholder_field: true,
        }
    }
}

impl FromNative for RoundChangeEvent {
    type Native = engine::RoundChangeEvent;

    fn from_native(native: Self::Native) -> Self {
        Self {
            round: native.round.number(),
        }
    }
}

impl FromNative for StopTimedRecoveryEvent {
    type Native = engine::StopTimedRecoveryEvent;

    fn from_native(_: Self::Native) -> Self {
        Self {
            placeholder_field: true,
        }
    }
}

impl FromNative for UnlockPrimaryRoleEvent {
    type Native = engine::UnlockPrimaryRoleEvent;

    fn from_native(_: Self::Native) -> Self {
        Self {
            placeholder_field: true,
        }
    }
}

impl FromNative for DepositRecoveryXrdEvent {
    type Native = engine::DepositRecoveryXrdEvent;

    fn from_native(native: Self::Native) -> Self {
        Self {
            amount: Arc::new(Decimal(native.amount)),
        }
    }
}

impl FromNative for WithdrawRecoveryXrdEvent {
    type Native = engine::WithdrawRecoveryXrdEvent;

    fn from_native(native: Self::Native) -> Self {
        Self {
            amount: Arc::new(Decimal(native.amount)),
        }
    }
}

impl FromNative for LockPrimaryRoleEvent {
    type Native = engine::LockPrimaryRoleEvent;

    fn from_native(_: Self::Native) -> Self {
        Self {
            placeholder_field: true,
        }
    }
}

impl FromNative for CancelBadgeWithdrawAttemptEvent {
    type Native = engine::CancelBadgeWithdrawAttemptEvent;

    fn from_native(native: Self::Native) -> Self {
        Self {
            proposer: <Proposer as FromNative>::from_native(native.proposer),
        }
    }
}

impl FromNative for CancelRecoveryProposalEvent {
    type Native = engine::CancelRecoveryProposalEvent;

    fn from_native(native: Self::Native) -> Self {
        Self {
            proposer: <Proposer as FromNative>::from_native(native.proposer),
        }
    }
}

impl FromNative for BadgeWithdrawEvent {
    type Native = engine::BadgeWithdrawEvent;

    fn from_native(native: Self::Native) -> Self {
        Self {
            proposer: <Proposer as FromNative>::from_native(native.proposer),
        }
    }
}

impl FromNative for RuleSetUpdateEvent {
    type Native = engine::RuleSetUpdateEvent;

    fn from_native(native: Self::Native) -> Self {
        Self {
            proposer: <Proposer as FromNative>::from_native(native.proposer),
            proposal: <RecoveryProposal as FromNative>::from_native(
                native.proposal,
            ),
        }
    }
}

impl FromNative for InitiateBadgeWithdrawAttemptEvent {
    type Native = engine::InitiateBadgeWithdrawAttemptEvent;

    fn from_native(native: Self::Native) -> Self {
        Self {
            proposer: <Proposer as FromNative>::from_native(native.proposer),
        }
    }
}

impl FromNative for InitiateRecoveryEvent {
    type Native = engine::InitiateRecoveryEvent;

    fn from_native(native: Self::Native) -> Self {
        Self {
            proposer: <Proposer as FromNative>::from_native(native.proposer),
            proposal: <RecoveryProposal as FromNative>::from_native(
                native.proposal,
            ),
        }
    }
}

impl FromNative for SetRoleEvent {
    type Native = engine::SetRoleEvent;

    fn from_native(native: Self::Native) -> Self {
        Self {
            role_key: native.role_key.key,
            rule: Arc::new(AccessRule(native.rule)),
        }
    }
}

impl FromNative for SetOwnerRoleEvent {
    type Native = engine::SetOwnerRoleEvent;

    fn from_native(native: Self::Native) -> Self {
        Self {
            rule: Arc::new(AccessRule(native.rule)),
        }
    }
}

impl FromNative for LockOwnerRoleEvent {
    type Native = engine::LockOwnerRoleEvent;

    fn from_native(_: Self::Native) -> Self {
        Self {
            placeholder_field: true,
        }
    }
}

impl FromNativeWithNetworkContext for SetMetadataEvent {
    type Native = engine::SetMetadataEvent;

    fn from_native(native: Self::Native, network_id: u8) -> Self {
        Self {
            key: native.key,
            value: MetadataValue::from_native(&native.value, network_id),
        }
    }
}

impl FromNative for RemoveMetadataEvent {
    type Native = engine::RemoveMetadataEvent;

    fn from_native(native: Self::Native) -> Self {
        Self { key: native.key }
    }
}

impl FromNativeWithNetworkContext for AccountWithdrawEvent {
    type Native = radix_engine::blueprints::account::WithdrawEvent;

    fn from_native(native: Self::Native, network_id: u8) -> Self {
        match native {
            Self::Native::Fungible(resource_address, amount) => {
                Self::Fungible {
                    resource_address: Arc::new(Address::from_node_id(
                        resource_address,
                        network_id,
                    )),
                    amount: Arc::new(Decimal(amount)),
                }
            }
            Self::Native::NonFungible(resource_address, ids) => {
                Self::NonFungible {
                    resource_address: Arc::new(Address::from_node_id(
                        resource_address,
                        network_id,
                    )),
                    ids: ids.into_iter().map(From::from).collect::<Vec<_>>(),
                }
            }
        }
    }
}

impl FromNativeWithNetworkContext for AccountDepositEvent {
    type Native = radix_engine::blueprints::account::DepositEvent;

    fn from_native(native: Self::Native, network_id: u8) -> Self {
        match native {
            Self::Native::Fungible(resource_address, amount) => {
                Self::Fungible {
                    resource_address: Arc::new(Address::from_node_id(
                        resource_address,
                        network_id,
                    )),
                    amount: Arc::new(Decimal(amount)),
                }
            }
            Self::Native::NonFungible(resource_address, ids) => {
                Self::NonFungible {
                    resource_address: Arc::new(Address::from_node_id(
                        resource_address,
                        network_id,
                    )),
                    ids: ids.into_iter().map(From::from).collect::<Vec<_>>(),
                }
            }
        }
    }
}

impl FromNativeWithNetworkContext for AccountRejectedDepositEvent {
    type Native = radix_engine::blueprints::account::RejectedDepositEvent;

    fn from_native(native: Self::Native, network_id: u8) -> Self {
        match native {
            Self::Native::Fungible(resource_address, amount) => {
                Self::Fungible {
                    resource_address: Arc::new(Address::from_node_id(
                        resource_address,
                        network_id,
                    )),
                    amount: Arc::new(Decimal(amount)),
                }
            }
            Self::Native::NonFungible(resource_address, ids) => {
                Self::NonFungible {
                    resource_address: Arc::new(Address::from_node_id(
                        resource_address,
                        network_id,
                    )),
                    ids: ids.into_iter().map(From::from).collect::<Vec<_>>(),
                }
            }
        }
    }
}

impl FromNativeWithNetworkContext for AccountSetResourcePreferenceEvent {
    type Native = radix_engine::blueprints::account::SetResourcePreferenceEvent;

    fn from_native(native: Self::Native, network_id: u8) -> Self {
        Self {
            resource_address: Arc::new(Address::from_node_id(
                native.resource_address,
                network_id,
            )),
            preference: <ResourcePreference as FromNative>::from_native(
                native.preference,
            ),
        }
    }
}

impl FromNativeWithNetworkContext for AccountRemoveResourcePreferenceEvent {
    type Native =
        radix_engine::blueprints::account::RemoveResourcePreferenceEvent;

    fn from_native(native: Self::Native, network_id: u8) -> Self {
        Self {
            resource_address: Arc::new(Address::from_node_id(
                native.resource_address,
                network_id,
            )),
        }
    }
}

impl FromNative for AccountSetDefaultDepositRuleEvent {
    type Native = radix_engine::blueprints::account::SetDefaultDepositRuleEvent;

    fn from_native(native: Self::Native) -> Self {
        Self {
            default_deposit_rule:
                <AccountDefaultDepositRule as FromNative>::from_native(
                    native.default_deposit_rule,
                ),
        }
    }
}

impl FromNativeWithNetworkContext for AccountAddAuthorizedDepositorEvent {
    type Native =
        radix_engine::blueprints::account::AddAuthorizedDepositorEvent;

    fn from_native(native: Self::Native, network_id: u8) -> Self {
        Self {
            authorized_depositor_badge:
                <ResourceOrNonFungible as FromNativeWithNetworkContext>::from_native(
                    native.authorized_depositor_badge,
                    network_id,
                ),
        }
    }
}

impl FromNativeWithNetworkContext for AccountRemoveAuthorizedDepositorEvent {
    type Native =
        radix_engine::blueprints::account::RemoveAuthorizedDepositorEvent;

    fn from_native(native: Self::Native, network_id: u8) -> Self {
        Self {
            authorized_depositor_badge:
                <ResourceOrNonFungible as FromNativeWithNetworkContext>::from_native(
                    native.authorized_depositor_badge,
                    network_id,
                ),
        }
    }
}

impl FromNative for FungibleVaultLockFeeEvent {
    type Native =
        radix_engine::blueprints::resource::fungible_vault::LockFeeEvent;

    fn from_native(native: Self::Native) -> Self {
        Self {
            amount: Arc::new(Decimal(native.amount)),
        }
    }
}

impl FromNative for FungibleVaultWithdrawEvent {
    type Native =
        radix_engine::blueprints::resource::fungible_vault::WithdrawEvent;

    fn from_native(native: Self::Native) -> Self {
        Self {
            amount: Arc::new(Decimal(native.amount)),
        }
    }
}

impl FromNative for FungibleVaultDepositEvent {
    type Native =
        radix_engine::blueprints::resource::fungible_vault::DepositEvent;

    fn from_native(native: Self::Native) -> Self {
        Self {
            amount: Arc::new(Decimal(native.amount)),
        }
    }
}

impl FromNative for FungibleVaultRecallEvent {
    type Native =
        radix_engine::blueprints::resource::fungible_vault::RecallEvent;

    fn from_native(native: Self::Native) -> Self {
        Self {
            amount: Arc::new(Decimal(native.amount)),
        }
    }
}

impl FromNative for FungibleVaultPayFeeEvent {
    type Native =
        radix_engine::blueprints::resource::fungible_vault::PayFeeEvent;

    fn from_native(native: Self::Native) -> Self {
        Self {
            amount: Arc::new(Decimal(native.amount)),
        }
    }
}

impl FromNative for NonFungibleVaultWithdrawEvent {
    type Native =
        radix_engine::blueprints::resource::non_fungible_vault::WithdrawEvent;

    fn from_native(native: Self::Native) -> Self {
        Self {
            ids: native.ids.into_iter().map(From::from).collect(),
        }
    }
}

impl FromNative for NonFungibleVaultDepositEvent {
    type Native =
        radix_engine::blueprints::resource::non_fungible_vault::DepositEvent;

    fn from_native(native: Self::Native) -> Self {
        Self {
            ids: native.ids.into_iter().map(From::from).collect(),
        }
    }
}

impl FromNative for NonFungibleVaultRecallEvent {
    type Native =
        radix_engine::blueprints::resource::non_fungible_vault::RecallEvent;

    fn from_native(native: Self::Native) -> Self {
        Self {
            ids: native.ids.into_iter().map(From::from).collect(),
        }
    }
}

impl FromNativeWithNetworkContext for ClaimEvent {
    type Native = engine::ClaimEvent;

    fn from_native(native: Self::Native, network_id: u8) -> Self {
        let address = Arc::new(Address::from_node_id(
            native.resource_address,
            network_id,
        ));
        Self {
            claimant: Arc::new(Address::from_node_id(
                native.claimant.0,
                network_id,
            )),
            resource_address: address.clone(),
            resources: ResourceSpecifier::from_native_for_locker_blueprint(
                &native.resources,
                &native.resource_address,
                network_id,
            ),
        }
    }
}

impl FromNativeWithNetworkContext for RecoverEvent {
    type Native = engine::RecoverEvent;

    fn from_native(native: Self::Native, network_id: u8) -> Self {
        let address = Arc::new(Address::from_node_id(
            native.resource_address,
            network_id,
        ));
        Self {
            claimant: Arc::new(Address::from_node_id(
                native.claimant.0,
                network_id,
            )),
            resource_address: address.clone(),
            resources: ResourceSpecifier::from_native_for_locker_blueprint(
                &native.resources,
                &native.resource_address,
                network_id,
            ),
        }
    }
}

impl FromNativeWithNetworkContext for StoreEvent {
    type Native = engine::StoreEvent;

    fn from_native(native: Self::Native, network_id: u8) -> Self {
        let address = Arc::new(Address::from_node_id(
            native.resource_address,
            network_id,
        ));
        Self {
            claimant: Arc::new(Address::from_node_id(
                native.claimant.0,
                network_id,
            )),
            resource_address: address.clone(),
            resources: ResourceSpecifier::from_native_for_locker_blueprint(
                &native.resources,
                &native.resource_address,
                network_id,
            ),
        }
    }
}

impl TryFrom<Emitter> for engine::Emitter {
    type Error = RadixEngineToolkitError;

    fn try_from(value: Emitter) -> Result<Self> {
        match value {
            Emitter::Function {
                address,
                blueprint_name,
            } => Ok(Self::Function(engine::BlueprintId::new(
                &engine::PackageAddress::try_from(*address)?,
                blueprint_name,
            ))),
            Emitter::Method {
                address,
                object_module_id,
            } => Ok(Self::Method((*address).into(), object_module_id.into())),
        }
    }
}

impl TryFrom<EventTypeIdentifier> for engine::EventTypeIdentifier {
    type Error = RadixEngineToolkitError;

    fn try_from(value: EventTypeIdentifier) -> Result<Self> {
        Ok(Self(value.emitter.try_into()?, value.event_name))
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
            DepositRecoveryXrdEvent,
            WithdrawRecoveryXrdEvent
        ],
    },
    Account => {
        Account => [
            AccountWithdrawEvent,
            AccountDepositEvent,
            AccountRejectedDepositEvent,
            AccountSetResourcePreferenceEvent,
            AccountRemoveResourcePreferenceEvent,
            AccountSetDefaultDepositRuleEvent,
            AccountAddAuthorizedDepositorEvent,
            AccountRemoveAuthorizedDepositorEvent,
        ]
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
            FungibleVaultLockFeeEvent,
            FungibleVaultWithdrawEvent,
            FungibleVaultDepositEvent,
            FungibleVaultRecallEvent,
            FungibleVaultPayFeeEvent,
        ],
        NonFungibleVault => [
            NonFungibleVaultWithdrawEvent,
            NonFungibleVaultDepositEvent,
            NonFungibleVaultRecallEvent,
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
            SetOwnerRoleEvent,
            LockOwnerRoleEvent,
        ]
    },
    Metadata => {
        Metadata => [
            SetMetadataEvent,
            RemoveMetadataEvent,
        ]
    },

    Locker => {
        AccountLocker => [
            StoreEvent,
            RecoverEvent,
            ClaimEvent
        ]
    },
}

/// This macro uses some special syntax to define the structure of events. This
/// makes the code for model definitions very compact, allows for very easy
/// addition of more packages, blueprints or events in the future, keeps various
/// models all in sync, and implements various functions and methods on
/// appropriate types.
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
                type Native = radix_substate_store_queries::typed_native_events::TypedNativeEvent;

                fn from_native(native: Self::Native, network_id: u8) -> Self {
                    match native {
                        $(
                            $(
                                $(
                                    radix_substate_store_queries::typed_native_events::TypedNativeEvent::$package_ident(
                                        radix_substate_store_queries::typed_native_events::[< Typed $package_ident PackageEvent >]::$blueprint_ident(
                                            radix_substate_store_queries::typed_native_events::[< Typed $blueprint_ident BlueprintEvent >]::$event_ty(event)
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
                        e => panic!("Can't get to this point. {:?}", e)
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
