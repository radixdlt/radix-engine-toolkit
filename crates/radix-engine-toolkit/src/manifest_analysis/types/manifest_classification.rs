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

/// The classification process classifies manifests into classes. The following
/// are the classes that the Radix Engine Toolkit supports. The order seen below
/// is the canonical order of specificity of the manifest transaction types.
#[derive(Clone, Debug, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ManifestClassification {
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
    Transfer,
    /// A manifest where XRD is withdrawn from one or more account(s), staked
    /// to one or more validator(s), and the LSUs deposited into one or more
    /// account(s).
    ValidatorStake,
    /// A manifest where LSUs are withdrawn from one or more account(s),
    /// unstaked from one or more validator(s), and the claim NFT(s) are
    /// deposited into one or more account(s).
    ValidatorUnstake,
    /// A manifest where claim NFT(s) are withdrawn from one or more account(s),
    /// get claimed from one or more validator(s), and then the XRD is deposited
    /// into one or more account(s).
    ValidatorClaimXrd,
    /// A manifest where fungible resources are contributed to a pool of any
    /// kind. In this class resources are withdrawn from one or more account(s),
    /// get contributed to one or more pool(s), and then the pool units get
    /// deposited into one or more account(s).
    PoolContribution,
    /// A manifest where pool units are redeemed from a pool of any kind. In
    /// this class pool units are withdrawn from one or more account(s), get
    /// contributed to one or more pool(s), and then the pool units get
    /// deposited into one or more account(s).
    PoolRedemption,
    /// A manifest where account deposit settings get updated. In this manifest
    /// class one of the account deposit settings methods are called.
    AccountDepositSettingsUpdate,
}
