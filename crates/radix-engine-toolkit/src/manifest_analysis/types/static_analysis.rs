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

/// This type represents the static analysis produced by the toolkit for some
/// manifest.
///
/// # Note
///
/// Most of the addresses found in the static analysis are dynamic addresses of
/// the type [`ManifestGlobalAddress`] which means that they're either static or
/// named addresses. For added context, [`Named`] addresses are addresses that
/// belong to an address allocation that took place in the manifest itself. In
/// these cases, we can observe the address allocation but can't tell what the
/// address is going to be before the transaction is ran. [`Static`] addresses
/// are the addresses that you see everyday that are typically Bech32m encoded.
///
/// [`Named`]: ManifestGlobalAddress::Named
/// [`Static`]: ManifestGlobalAddress::Static
pub struct StaticAnalysis {
    /// A summary of all of the account interactions that could be observed in
    /// the manifest. This has the set of accounts withdrawn from, deposited
    /// into, securified, burned from, created proofs from, locked fees from
    /// and pretty much all of the information about the account interactions.
    pub account_interactions_summary: AccountInteractionsOutput,

    /// A summary of all of the account resource movements that occurred in the
    /// manifest as seen by the static resource movements analyzer.
    pub account_static_resource_movements_summary:
        AccountStaticResourceMovements,

    /// A summary of all of the proofs created from accounts that could be
    /// observed in the manifest.
    pub proofs_created_summary: PresentedProofsOutput,

    /// A summary of all of the entities encountered in the manifest while
    /// traversing it. This covers pretty much all entities whether they're
    /// seen in instructions as addresses, in arguments, or pretty much anywhere
    /// in the manifest.
    pub entities_encountered_summary: EncounteredEntitiesOutput,

    /// A summary of all of the entities that the wallet needs to provide auth
    /// for. This includes both account and identity entities.
    pub entities_requiring_auth_summary: EntitiesRequiringAuthOutput,

    /// A summary of the reserved instructions that the toolkit has found in the
    /// manifest.
    pub reserved_instructions_summary: ReservedInstructionsOutput,

    /// An ordered set of the classification(s) of the manifest based on the
    /// static analysis.
    pub manifest_classification: BTreeSet<ManifestClassification>,
}
