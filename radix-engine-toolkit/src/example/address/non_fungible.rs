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

use crate::model::address::NonFungibleLocalId;

pub fn non_fungible_local_uuid() -> NonFungibleLocalId {
    NonFungibleLocalId::UUID(241008287272164729465721528295504357972)
}

pub fn non_fungible_local_integer() -> NonFungibleLocalId {
    NonFungibleLocalId::Integer(1)
}

pub fn non_fungible_local_string() -> NonFungibleLocalId {
    NonFungibleLocalId::String("Scrypto".into())
}

pub fn non_fungible_local_bytes() -> NonFungibleLocalId {
    NonFungibleLocalId::Bytes(vec![0x00, 0x01, 0x02, 0x03])
}
