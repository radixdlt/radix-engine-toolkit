use std::ops::*;

use scrypto::prelude::*;

use radix_engine::system::system_modules::execution_trace::*;
use radix_engine::transaction::*;
use radix_engine_interface::blueprints::account::*;

/// A summary of the manifest
#[derive(Clone, Debug)]
pub struct ManifestSummary {
    /// The set of the resource addresses of proofs that were presented in
    /// the manifest.
    pub presented_proofs: IndexSet<ResourceAddress>,
    /// The set of all entities encountered in the manifest - this is used by
    /// the wallet for the "using dApps" section.
    pub encountered_entities: IndexSet<NodeId>,
    /// The set of accounts encountered in the manifest where privileged
    /// methods were called.
    pub accounts_requiring_auth: IndexSet<NodeId>,
    /// The set of identities encountered in the manifest where privileged
    /// methods were called.
    pub identities_requiring_auth: IndexSet<NodeId>,
    /// The set of instructions encountered in the manifest that are reserved
    /// and can only be included in the manifest by the wallet itself.
    pub reserved_instructions: IndexSet<ReservedInstruction>,
    /// The various classifications that this manifest matched against. Note
    /// that an empty set means that the manifest is non-conforming.
    pub classification: IndexSet<ManifestClass>,
}

/// A summary of the execution of the manifest and the information that can
#[derive(Clone, Debug)]
pub struct ExecutionSummary {
    /// The withdraws done in the manifest.
    pub account_withdraws: IndexMap<ComponentAddress, Vec<ResourceIndicator>>,
    /// The deposits done in the manifest.
    pub account_deposits: IndexMap<ComponentAddress, Vec<ResourceIndicator>>,
    /// The set of the resource addresses of proofs that were presented in
    /// the manifest.
    pub presented_proofs: IndexSet<ResourceAddress>,
    /// Information on the global entities created in the transaction.
    pub new_entities: NewEntities,
    /// The set of all entities encountered in the manifest - this is used by
    /// the wallet for the "using dApps" section.
    pub encountered_entities: IndexSet<NodeId>,
    /// The set of accounts encountered in the manifest where privileged
    /// methods were called.
    pub accounts_requiring_auth: IndexSet<ComponentAddress>,
    /// The set of identities encountered in the manifest where privileged
    /// methods were called.
    pub identities_requiring_auth: IndexSet<ComponentAddress>,
    /// The set of instructions encountered in the manifest that are reserved
    /// and can only be included in the manifest by the wallet itself.
    pub reserved_instructions: IndexSet<ReservedInstruction>,
    /// The various classifications that this manifest matched against. Note
    /// that an empty set means that the manifest is non-conforming.
    pub detailed_classification: IndexSet<DetailedManifestClass>,
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

impl<'r> TransactionTypesReceipt<'r> {
    pub fn new_components(&self) -> &'r IndexSet<ComponentAddress> {
        self.commit_result.new_component_addresses()
    }

    pub fn new_resources(&self) -> &'r IndexSet<ResourceAddress> {
        self.commit_result.new_resource_addresses()
    }

    pub fn new_packages(&self) -> &'r IndexSet<PackageAddress> {
        self.commit_result.new_package_addresses()
    }

    pub fn execution_trace(&self) -> &'r TransactionExecutionTrace {
        self.execution_trace
    }
}

impl<'r> Deref for TransactionTypesReceipt<'r> {
    type Target = TransactionReceipt;

    fn deref(&self) -> &Self::Target {
        self.receipt
    }
}

/// Information on the entities created in the transaction.
#[derive(Clone, Debug)]
pub struct NewEntities {
    pub component_addresses: IndexSet<ComponentAddress>,
    pub resource_addresses: IndexSet<ResourceAddress>,
    pub package_addresses: IndexSet<PackageAddress>,
}

/// The set of instructions that is only allowed in manifests created by the
/// wallet itself.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ReservedInstruction {
    AccountLockFee,
    AccountSecurify,
    IdentitySecurify,
    AccountUpdateSettings,
    AccessControllerMethod,
}

pub enum FnRule {
    Allowed,
    Disallowed,
}

/// A struct that stores information on the methods that the general transaction
/// visitor allows and does not allow.
pub struct FnRules {
    pub allowed: &'static [&'static str],
    pub disallowed: &'static [&'static str],
    pub default: FnRule,
}

impl FnRules {
    pub fn is_fn_permitted(&self, fn_name: &str) -> bool {
        if self.allowed.contains(&fn_name) {
            true
        } else if self.disallowed.contains(&fn_name) {
            false
        } else {
            match self.default {
                FnRule::Allowed => true,
                FnRule::Disallowed => false,
            }
        }
    }

    pub fn all_allowed() -> Self {
        Self {
            allowed: Default::default(),
            disallowed: Default::default(),
            default: FnRule::Allowed,
        }
    }

    pub fn all_disallowed() -> Self {
        Self {
            allowed: Default::default(),
            disallowed: Default::default(),
            default: FnRule::Disallowed,
        }
    }
}

#[derive(Clone, Debug)]
pub enum ResourceIndicator {
    Fungible(ResourceAddress, FungibleResourceIndicator),
    NonFungible(ResourceAddress, NonFungibleResourceIndicator),
}

#[derive(Clone, Debug)]
pub enum FungibleResourceIndicator {
    Guaranteed(Decimal),
    Predicted(Predicted<Decimal>),
}

#[derive(Clone, Debug)]
pub enum NonFungibleResourceIndicator {
    ByAll {
        predicted_amount: Predicted<Decimal>,
        predicted_ids: Predicted<IndexSet<NonFungibleLocalId>>,
    },
    ByAmount {
        amount: Decimal,
        predicted_ids: Predicted<IndexSet<NonFungibleLocalId>>,
    },
    ByIds(IndexSet<NonFungibleLocalId>),
}

#[derive(Clone, Debug)]
pub struct Predicted<T> {
    pub value: T,
    pub instruction_index: usize,
}

impl<T> Deref for Predicted<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for Predicted<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl Deref for FungibleResourceIndicator {
    type Target = Decimal;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Guaranteed(value)
            | Self::Predicted(Predicted { value, .. }) => value,
        }
    }
}

impl DerefMut for FungibleResourceIndicator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::Guaranteed(value)
            | Self::Predicted(Predicted { value, .. }) => value,
        }
    }
}

impl ResourceIndicator {
    pub fn resource_address(&self) -> ResourceAddress {
        match self {
            Self::Fungible(resource_address, _)
            | Self::NonFungible(resource_address, _) => *resource_address,
        }
    }
}

impl From<ResourceIndicator> for ResourceSpecifier {
    fn from(value: ResourceIndicator) -> Self {
        match value {
            ResourceIndicator::Fungible(
                resource_address,
                FungibleResourceIndicator::Guaranteed(amount),
            )
            | ResourceIndicator::Fungible(
                resource_address,
                FungibleResourceIndicator::Predicted(Predicted {
                    value: amount,
                    ..
                }),
            ) => ResourceSpecifier::Amount(resource_address, amount),
            ResourceIndicator::NonFungible(
                resource_address,
                NonFungibleResourceIndicator::ByAll {
                    predicted_ids: Predicted { value: ids, .. },
                    ..
                }
                | NonFungibleResourceIndicator::ByAmount {
                    predicted_ids: Predicted { value: ids, .. },
                    ..
                }
                | NonFungibleResourceIndicator::ByIds(ids),
            ) => ResourceSpecifier::Ids(resource_address, ids),
        }
    }
}

#[extend::ext]
pub impl ResourceSpecifier {
    fn resource_address(&self) -> ResourceAddress {
        match self {
            Self::Amount(x, ..) | Self::Ids(x, ..) => *x,
        }
    }
}
