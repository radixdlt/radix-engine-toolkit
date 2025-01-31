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

/// The resource movements that were detected in the manifest by the dynamic
/// analyzer. Some of the information here is obtained from the worktop changes
/// of the toolkit receipt. As an example, if a withdraw of non-fungibles is
/// performed by amount then we will attempt to resolve the non-fungible ids
/// from the amount we see in the worktop changes.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct AccountDynamicResourceMovementsOutput {
    /// The account withdraws that were observed by the static analyzer.
    pub account_withdraws: IndexMap<GlobalAddress, Vec<InvocationIoItem>>,
    /// The account deposits that were observed by the static analyzer.
    pub account_deposits: IndexMap<GlobalAddress, Vec<InvocationIoItem>>,
}
