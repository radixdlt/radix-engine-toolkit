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

use radix_engine_toolkit::prelude::*;

pub fn generate_function_spec() -> OpenApi {
    open_api_spec![
        BuildInformation,
        DeriveVirtualAccountAddressFromPublicKey,
        DeriveVirtualIdentityAddressFromPublicKey,
        DeriveVirtualSignatureNonFungibleGlobalIdFromPublicKey,
        DeriveVirtualAccountAddressFromOlympiaAccountAddress,
        DeriveResourceAddressFromOlympiaResourceAddress,
        DerivePublicKeyFromOlympiaAccountAddress,
        DeriveOlympiaAccountAddressFromPublicKey,
        DeriveNodeAddressFromPublicKey,
        InstructionsHash,
        InstructionsConvert,
        InstructionsCompile,
        InstructionsDecompile,
        InstructionsStaticallyValidate,
        InstructionsExtractAddresses,
        ExecutionAnalyze,
        ManifestHash,
        ManifestCompile,
        ManifestDecompile,
        ManifestStaticallyValidate,
        IntentHash,
        IntentCompile,
        IntentDecompile,
        IntentStaticallyValidate,
        SignedIntentHash,
        SignedIntentCompile,
        SignedIntentDecompile,
        SignedIntentStaticallyValidate,
        NotarizedTransactionHash,
        NotarizedTransactionCompile,
        NotarizedTransactionDecompile,
        NotarizedTransactionStaticallyValidate,
        UtilsKnownAddress,
        ScryptoSborDecodeToString,
        ManifestSborDecodeToString
    ]
}

macro_rules! open_api_spec {
    (
        $(
            $type: ty
        ),* $(,)?
    ) => {
        {
            let mut generator = rocket_okapi::gen::OpenApiGenerator::new(&Default::default());
            $(
                generator.json_schema::<<$type as radix_engine_toolkit::functions::traits::Function>::Input>();
                generator.json_schema::<<$type as radix_engine_toolkit::functions::traits::Function>::Output>();
            )*
            generator.into_openapi()
        }
    };
}
use open_api_spec;
use rocket_okapi::okapi::openapi3::OpenApi;
