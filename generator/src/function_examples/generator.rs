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

use super::traits::FunctionExample;
use indexmap::{indexmap, IndexMap};
use native_json_library::functions::derive::*;
use native_json_library::functions::information::*;
use native_json_library::functions::instructions::*;

#[allow(clippy::type_complexity)]
pub fn generate_function_examples() -> IndexMap<
    &'static str,
    IndexMap<String, Vec<FunctionExample<serde_json::Value, serde_json::Value>>>,
> {
    indexmap!(
        "information" => function_examples![
            BuildInformation
        ],
        "derive" => function_examples![
            DeriveVirtualAccountAddressFromPublicKey,
            DeriveVirtualIdentityAddressFromPublicKey,
            DeriveVirtualSignatureNonFungibleGlobalIdFromPublicKey,
            DeriveVirtualAccountAddressFromOlympiaAccountAddress,
            DeriveResourceAddressFromOlympiaResourceAddress,
            DerivePublicKeyFromOlympiaAccountAddress,
            DeriveOlympiaAccountAddressFromPublicKey,
            DeriveNodeAddressFromPublicKey,
        ],
        "instructions" => function_examples![
            InstructionsHash,
            InstructionsConvert,
            InstructionsCompile,
            InstructionsDecompile,
            InstructionsStaticallyValidate,
            InstructionsExtractAddresses,
            InstructionsTransactionType
        ]
    )
}

macro_rules! function_examples {
    (
        $( $function: ident ),* $(,)?
    ) => {
        {
            use $crate::function_examples::traits::HasExamples;

            let mut map = indexmap::IndexMap::new();

            $(
                let name = $function::function_name();
                let examples = $function::serde_value_examples().into_iter().collect::<Vec<_>>();

                map.insert(name, examples);
            )*

            map
        }
    };
}
use function_examples;
