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

pub type MetadataInit = HashMap<String, MetadataInitEntry>;

#[derive(Debug, Clone, Record)]
pub struct MetadataInitEntry {
    pub value: Option<MetadataValue>,
    pub lock: bool,
}

#[derive(Debug, Clone, Record)]
pub struct MetadataModuleConfig {
    pub init: MetadataInit,
    pub roles: HashMap<String, Option<Arc<AccessRule>>>,
}

impl ToNative for MetadataInit {
    type Native = NativeMetadataInit;

    fn to_native(self) -> Result<Self::Native> {
        self.into_iter()
            .map(|(key, value)| {
                let metadata = match value.value.map(|value| value.to_native())
                {
                    Some(Ok(metadata)) => Some(metadata),
                    Some(Err(error)) => return Err(error),
                    None => None,
                };

                Ok((
                    key,
                    NativeKeyValueStoreInitEntry::<NativeMetadataValue> {
                        lock: value.lock,
                        value: metadata,
                    },
                ))
            })
            .collect::<Result<
                IndexMap<
                    String,
                    NativeKeyValueStoreInitEntry<NativeMetadataValue>,
                >,
            >>()
            .map(|data| NativeMetadataInit { data })
    }
}

impl ToNative for MetadataModuleConfig {
    type Native = NativeModuleConfig<NativeMetadataInit>;

    fn to_native(self) -> Result<Self::Native> {
        Ok(NativeModuleConfig::<NativeMetadataInit> {
            init: self.init.to_native()?,
            roles: NativeRoleAssignmentInit {
                data: self
                    .roles
                    .into_iter()
                    .map(|(key, value)| {
                        (
                            NativeRoleKey { key },
                            value.map(|value| value.0.clone()),
                        )
                    })
                    .collect(),
            },
        })
    }
}
