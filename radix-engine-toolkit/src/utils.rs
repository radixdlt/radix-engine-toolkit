// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at

//   http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use crate::model::address::Bech32Coder;
use native_transaction::data::{format_manifest_value, ManifestDecompilationDisplayContext};
use radix_engine_common::prelude::{to_manifest_value, ManifestEncode};
use std::fmt::Debug;

pub fn checked_copy_u8_slice<T: AsRef<[u8]>, const N: usize>(slice: T) -> Option<[u8; N]> {
    let slice = slice.as_ref();
    if slice.len() != N {
        None
    } else {
        let mut bytes = [0u8; N];
        bytes.copy_from_slice(&slice[0..N]);
        Some(bytes)
    }
}

pub fn debug_string<T: Debug>(object: T) -> String {
    format!("{:?}", object)
}

pub fn manifest_string_representation<T>(value: &T, bech32_coder: &Bech32Coder) -> String
where
    T: ManifestEncode,
{
    let mut string = String::new();
    let mut context =
        ManifestDecompilationDisplayContext::with_optional_bech32(Some(bech32_coder.encoder()));
    format_manifest_value(&mut string, &to_manifest_value(value), &context, true, 0)
        .expect("Impossible case! Valid SBOR can't fail here");
    string
}
