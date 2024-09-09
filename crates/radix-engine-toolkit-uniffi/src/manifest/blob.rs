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

use crate::prelude::*;

#[derive(Clone, Debug, Record)]
pub struct ManifestBlobRef {
    pub value: Arc<Hash>,
}

impl From<NativeManifestBlobRef> for ManifestBlobRef {
    fn from(value: NativeManifestBlobRef) -> Self {
        Self {
            value: Arc::new(NativeHash(value.0).into()),
        }
    }
}

impl From<ManifestBlobRef> for NativeManifestBlobRef {
    fn from(value: ManifestBlobRef) -> Self {
        Self(value.value.0.0)
    }
}
