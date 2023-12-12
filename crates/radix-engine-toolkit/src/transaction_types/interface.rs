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

//! Functions that expose the transaction types functionality without exposing
//! any of the implementation details of how the module finds and determines
//! the transaction types.

use radix_engine::prelude::*;
use radix_engine::transaction::*;
use transaction::prelude::*;

use super::error::*;
use super::types::*;

pub fn summary(_manifest: &TransactionManifestV1) -> IndexSet<ManifestSummary> {
    todo!()
}

pub fn execution_summary(
    _manifest: &TransactionManifestV1,
    _receipt: &TransactionReceipt,
) -> Result<ExecutionSummary, TransactionTypesError> {
    todo!()
}
