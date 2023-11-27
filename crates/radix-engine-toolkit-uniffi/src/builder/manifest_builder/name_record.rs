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

macro_rules! define_name_record {
    (
        $(
            {
                name: $name: ident,
                ty: $ty: ty,
                allocation_fn: $allocation_fn: ident $(,)?
            }
        ),* $(,)?
    ) => {
        paste::paste! {
            #[derive(Clone, Debug, Default)]
            pub struct NameRecord {
                id_allocator: crate::prelude::NativeManifestIdAllocator,
                $(
                    $name: crate::prelude::HashMap<String, $ty>,
                )*
            }

            impl NameRecord {
                $(
                    pub fn [< new_ $name >](&mut self, name: &str) -> Result<()> {
                        let map = &mut self.$name;
                        if map.contains_key(name) {
                            Err(NameRecordError::ObjectNameIsAlreadyTaken {
                                object: stringify!($name).to_owned(),
                                name: name.to_owned(),
                            }.into())
                        } else {
                            let object = self.id_allocator.$allocation_fn();
                            map.insert(name.to_owned(), object);
                            Ok(())
                        }
                    }

                    pub fn [< get_ $name >](&self, name: &str) -> Result<&$ty> {
                        self.$name
                            .get(name)
                            .ok_or(NameRecordError::ObjectDoesNotExist {
                                object: stringify!($name).to_owned(),
                                name: name.to_owned(),
                            }.into())
                    }
                )*
            }

        }
    };
}
define_name_record! {
    {
        name: bucket,
        ty: crate::prelude::NativeManifestBucket,
        allocation_fn: new_bucket_id
    },
    {
        name: proof,
        ty: crate::prelude::NativeManifestProof,
        allocation_fn: new_proof_id
    },
    {
        name: address_reservation,
        ty: crate::prelude::NativeManifestAddressReservation,
        allocation_fn: new_address_reservation_id
    },
    {
        name: named_address,
        ty: u32,
        allocation_fn: new_address_id
    },
}

#[derive(Clone, Debug, Enum)]
pub enum NameRecordError {
    ObjectNameIsAlreadyTaken { object: String, name: String },
    ObjectDoesNotExist { object: String, name: String },
}
