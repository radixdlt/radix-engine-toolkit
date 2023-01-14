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

use serializable::serializable;

use crate::{TransactionHeader, TransactionManifest};

/// A transaction intent which is made of the header containing the transaction metadata and a
/// manifest consisting of the instructions and blobs.
#[serializable]
pub struct TransactionIntent {
    /// A transaction header of the transaction metadata.
    pub header: TransactionHeader,

    /// A transaction manifest of the transaction instructions and blobs.
    pub manifest: TransactionManifest,
}
