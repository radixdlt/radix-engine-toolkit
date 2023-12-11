use std::ops::Deref;

use scrypto::prelude::*;

use radix_engine::transaction::*;
use radix_engine_interface::blueprints::account::*;

/// A summary of the manifest
#[derive(Clone, Debug)]
pub struct ManifestSummary {
    /// The set of the resource addresses of proofs that were presented in
    /// the manifest.
    presented_proofs: IndexSet<ResourceAddress>,
    /// The set of all entities encountered in the manifest - this is used by
    /// the wallet for the "using dApps" section.
    encountered_entities: IndexSet<NodeId>,
    /// The set of accounts encountered in the manifest where privileged
    /// methods were called.
    accounts_requiring_auth: IndexSet<NodeId>,
    /// The set of identities encountered in the manifest where privileged
    /// methods were called.
    identities_requiring_auth: IndexSet<NodeId>,
    /// The various classifications that this manifest matched against. Note
    /// that an empty set means that the manifest is non-conforming.
    classification: IndexSet<ManifestClass>,
}

/// A summary of the execution of the manifest and the information that can
#[derive(Clone, Debug)]
pub struct ExecutionSummary {
    /// The withdraws done in the manifest. This information is obtained
    /// through static analysis of the manifest and does not require any
    /// kind of info from the execution trace.
    account_withdraws: IndexMap<
        ComponentAddress,
        IndexMap<ResourceAddress, WithdrawInformation>,
    >,
    account_deposits: IndexMap<ComponentAddress, Vec<DepositInformation>>,
    /// The set of the resource addresses of proofs that were presented in
    /// the manifest.
    presented_proofs: IndexSet<ResourceAddress>,
    /// The set of all entities encountered in the manifest - this is used by
    /// the wallet for the "using dApps" section.
    encountered_entities: IndexSet<NodeId>,
    /// The set of accounts encountered in the manifest where privileged
    /// methods were called.
    accounts_requiring_auth: IndexSet<NodeId>,
    /// The set of identities encountered in the manifest where privileged
    /// methods were called.
    identities_requiring_auth: IndexSet<NodeId>,
    /// The various classifications that this manifest matched against. Note
    /// that an empty set means that the manifest is non-conforming.
    detailed_classification: IndexSet<DetailedManifestClass>,
}

/// The classification process classifies manifests into classes. The following
/// are the classes that the Radix Engine Toolkit supports.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ManifestClass {
    /// A general manifest that involves any amount of arbitrary components
    /// and packages where nothing more concrete can be said about the manifest
    /// and its nature.
    General,
    /// A manifest of a 1-to-1 transfer to a one-to-many transfer of resources.
    Transfer,
    /// A manifest that contributed some amount of resources to a liquidity
    /// pool that can be a one-resource pool, two-resource pool, or a
    /// multi-resource pool.
    PoolContribution,
    /// A manifest that redeemed resources from a liquidity pool. Similar to
    /// contributions, this can be any of the three pool blueprints available
    /// in the pool package.
    PoolRedemption,
    /// A manifest where XRD is staked to one or more validators.
    ValidatorStake,
    /// A manifest where XRD is unstaked from one or more validators.
    ValidatorUnstake,
    /// A manifest where XRD is claimed from one or more validators.
    ValidatorClaim,
    /// A manifest that updated the deposit settings of the account.
    AccountDepositSettingsUpdate,
}

/// The execution summary process not only determines the class of the manifest,
/// but also includes additional information about this class that the wallet
/// requires to display to the user.
///
/// # Note
///
/// This enum must have as many variants as the [`ManifestClass`] and there
/// must always be a valid implementation of [`Into<ManifestClass>`] for this
/// enum.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DetailedManifestClass {
    /// A general manifest that involves any amount of arbitrary components
    /// and packages where nothing more concrete can be said about the manifest
    /// and its nature.
    ///
    /// No additional information is required beyond what the execution summary
    /// will provide.
    General,
    /// A manifest of a 1-to-1 transfer to a one-to-many transfer of resources.
    Transfer {
        /// When `true`, then this is a one-to-one transfer and the wallet can
        /// regard this as a "simple transfer" and communicate this information
        /// to the ledger hardware wallet. Otherwise, if `false`, then this is
        /// not a one-to-one transfer.
        is_one_to_one: bool,
    },
    /// A manifest that contributed some amount of resources to a liquidity
    /// pool that can be a one-resource pool, two-resource pool, or a
    /// multi-resource pool.
    PoolContribution {
        /// The set of pool addresses contributed to in the manifest.
        pools_contributed_to: IndexSet<ComponentAddress>,
        /// The worth of pool units gotten back.
        /// pool_unit_resource -> (pool_unit_amount, asset -> amount)
        pool_units_worth: IndexMap<
            ResourceAddress,
            (Decimal, IndexMap<ResourceAddress, Decimal>),
        >,
    },
    /// A manifest that redeemed resources from a liquidity pool. Similar to
    /// contributions, this can be any of the three pool blueprints available
    /// in the pool package.
    PoolRedemption {
        /// The set of pool addresses redeemed from in the manifest.
        pools_redeemed_from: IndexSet<ComponentAddress>,
        /// The worth of pool units gotten back.
        /// pool_unit_resource -> (pool_unit_amount, asset -> amount)
        pool_units_worth: IndexMap<
            ResourceAddress,
            (Decimal, IndexMap<ResourceAddress, Decimal>),
        >,
    },
    /// A manifest where XRD is staked to one or more validators.
    ValidatorStake {
        /// The set of validator addresses staked to in the manifest.
        validators_staked_to: IndexSet<ComponentAddress>,
        /// The worth of the various liquid stake units seen in the manifest.
        /// lsu_resource -> (amount, xrd_worth)
        liquid_stake_units_worth: IndexMap<ResourceAddress, (Decimal, Decimal)>,
    },
    /// A manifest where XRD is unstaked from one or more validators.
    ValidatorUnstake {
        /// The set of validator addresses unstaked from in the manifest.
        validators_unstaked_from: IndexSet<ComponentAddress>,
    },
    /// A manifest where XRD is claimed from one or more validators.
    ValidatorClaim {
        /// The set of validator addresses claimed from in the manifest.
        validators_claimed_from: IndexSet<ComponentAddress>,
    },
    /// A manifest that updated the deposit settings of the account.
    AccountDepositSettingsUpdate {
        /// Changes to the account's deposit mode.
        /// account_address -> new_default_deposit_mode
        deposit_mode_changes: IndexMap<ComponentAddress, DefaultDepositRule>,
        /// Changes to the preference of particular resources in the account
        /// deposit settings.
        /// account_address -> (resource_address -> resource_preference).
        resource_preference_changes: IndexMap<
            ComponentAddress,
            IndexMap<ResourceAddress, ResourcePreference>,
        >,
    },
}

impl From<DetailedManifestClass> for ManifestClass {
    fn from(value: DetailedManifestClass) -> Self {
        match value {
            DetailedManifestClass::General => ManifestClass::General,
            DetailedManifestClass::Transfer { .. } => ManifestClass::Transfer,
            DetailedManifestClass::PoolContribution { .. } => {
                ManifestClass::PoolContribution
            }
            DetailedManifestClass::PoolRedemption { .. } => {
                ManifestClass::PoolRedemption
            }
            DetailedManifestClass::ValidatorStake { .. } => {
                ManifestClass::ValidatorStake
            }
            DetailedManifestClass::ValidatorUnstake { .. } => {
                ManifestClass::ValidatorUnstake
            }
            DetailedManifestClass::ValidatorClaim { .. } => {
                ManifestClass::ValidatorClaim
            }
            DetailedManifestClass::AccountDepositSettingsUpdate { .. } => {
                ManifestClass::AccountDepositSettingsUpdate
            }
        }
    }
}

/// A receipt used for the calculation of the execution summary. This receipt
/// must belong to a transaction that executed successfully and the execution
/// trace must be present.
#[derive(Clone, Debug)]
pub struct TransactionTypesReceipt<'r> {
    receipt: &'r TransactionReceipt,
    commit_result: &'r CommitResult,
    execution_trace: &'r TransactionExecutionTrace,
}

impl<'r> TransactionTypesReceipt<'r> {
    pub fn new(receipt: &'r TransactionReceipt) -> Option<Self> {
        if let TransactionResult::Commit(
            ref commit_result @ CommitResult {
                execution_trace: Some(ref execution_trace),
                ..
            },
        ) = &receipt.result
        {
            Some(Self {
                receipt,
                commit_result,
                execution_trace,
            })
        } else {
            None
        }
    }
}

impl<'r> Deref for TransactionTypesReceipt<'r> {
    type Target = TransactionReceipt;

    fn deref(&self) -> &Self::Target {
        &self.receipt
    }
}

#[derive(Clone, Debug)]
pub enum WithdrawInformation {
    /// Withdraws of a fungible resource - all fungible withdraws are by
    /// amount.
    Fungible(Decimal),
    /// Withdraws of a non-fungible resource where some withdraws could be by
    /// amount and others could be by ids. We persist both to retain the full
    /// information. The total withdrawn amount is the amount + len(set).
    NonFungible(Decimal, IndexSet<NonFungibleLocalId>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DepositInformation {
    /// An account deposit of a fungible resources where the amount can either
    /// be guaranteed or predicted.
    Fungible {
        resource_address: ResourceAddress,
        amount: Source<Decimal>,
    },
    /// A set of tracked non-fungible resources. In this case, the amount and
    /// ids may be guaranteed or predicted. A valid non-fungible tracker
    /// may have a guaranteed amount but a non-guaranteed set of ids.
    NonFungible {
        resource_address: ResourceAddress,
        amount: Source<Decimal>,
        ids: Source<IndexSet<NonFungibleLocalId>>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Source<T> {
    Guaranteed(T),
    Predicted(usize, T),
}

impl<T> std::ops::Deref for Source<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Guaranteed(target) | Self::Predicted(_, target) => target,
        }
    }
}

impl<T> std::ops::DerefMut for Source<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::Guaranteed(target) | Self::Predicted(_, target) => target,
        }
    }
}
