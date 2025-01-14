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

/// The classification process classifies manifests into classes. The following
/// are the classes that the Radix Engine Toolkit supports. The order seen below
/// is the canonical order of specificity of the manifest transaction types.
#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(clippy::large_enum_variant)]
pub enum DetailedManifestClassification {
    /// A general manifest that has a number of arbitrary package and component
    /// invocations.
    General,
    /// A general subintent manifest that has a number of arbitrary package and
    /// component invocations. This manifest is guaranteed to be subintent since
    /// we require that a yield to child is present in the manifest.
    GeneralSubintent,
    /// A manifest containing transfers between accounts only where resources
    /// are withdrawn from one or more account(s) and deposited into one or more
    /// account(s) without any calls to any other components.
    Transfer {
        /// This can be thought of as being a "sub-classification" of sorts
        /// where the analyzer notes down if the transfer is a one-to-one
        /// transfer in the format of the "simple transfers" rules from the
        /// transaction types specification.
        is_one_to_one_transfer: bool,
    },
    /* TODO: Add the remaining transaction types here */
    /// A manifest where account deposit settings get updated. In this manifest
    /// class one of the account deposit settings methods are called.
    AccountDepositSettingsUpdate(AccountSettingsUpdateOutput),
}
