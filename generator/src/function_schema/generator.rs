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

use indexmap::{indexmap, IndexMap};
use radix_engine_toolkit::prelude::*;
use schemars::schema::RootSchema;

pub fn generate_function_schema(
) -> IndexMap<&'static str, IndexMap<String, (RootSchema, RootSchema)>> {
    indexmap!(
        "information" => function_schema![
            BuildInformation,
        ],
        "derive" => function_schema![
            DeriveVirtualAccountAddressFromPublicKey,
            DeriveVirtualIdentityAddressFromPublicKey,
            DeriveVirtualSignatureNonFungibleGlobalIdFromPublicKey,
            DeriveVirtualAccountAddressFromOlympiaAccountAddress,
            DeriveResourceAddressFromOlympiaResourceAddress,
            DerivePublicKeyFromOlympiaAccountAddress,
            DeriveOlympiaAccountAddressFromPublicKey,
            DeriveNodeAddressFromPublicKey,
        ],
        "instructions" => function_schema![
            InstructionsHash,
            InstructionsConvert,
            InstructionsCompile,
            InstructionsDecompile,
            InstructionsStaticallyValidate,
            InstructionsExtractAddresses,
        ],
        "execution" => function_schema![
            ExecutionAnalyze
        ],
        "manifest" => function_schema![
            ManifestHash,
            ManifestCompile,
            ManifestDecompile,
            ManifestStaticallyValidate,
        ],
        "intent" => function_schema![
            IntentHash,
            IntentCompile,
            IntentDecompile,
            IntentStaticallyValidate,
        ],
        "signed_intent" => function_schema![
            SignedIntentHash,
            SignedIntentCompile,
            SignedIntentDecompile,
            SignedIntentStaticallyValidate,
        ],
        "notarized_transaction" => function_schema![
            NotarizedTransactionHash,
            NotarizedTransactionCompile,
            NotarizedTransactionDecompile,
            NotarizedTransactionStaticallyValidate,
        ],
        "utils" => function_schema![
            UtilsKnownAddress
        ]
    )
}

macro_rules! function_schema {
    (
        $( $function: ident ),* $(,)?
    ) => {
        {
            let mut map = indexmap::IndexMap::new();

            $(

                let name = $crate::utils::snake_case_type_name::<$function>();

                let input_schema = schemars::schema_for!(<$function as radix_engine_toolkit::functions::traits::Function>::Input);
                let output_schema = schemars::schema_for!(<$function as radix_engine_toolkit::functions::traits::Function>::Output);

                map.insert(name, (input_schema, output_schema));
            )*

            map
        }
    };
}
use function_schema;
