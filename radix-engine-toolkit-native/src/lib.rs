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

use radix_engine_toolkit_core::requests::*;
use radix_engine_toolkit_core::traits::Request;

/// Exports a request with the following C function signature:
///
/// ```C
/// char* function_name(char* request);
/// ```
#[macro_export]
macro_rules! export_request {
    ($request_type: ident as $export_ident: ident) => {
        /// This function exports a request function and builds a wrapper for it.
        ///
        /// # Safety
        ///
        /// This function makes use of pointers which is an unsafe feature.
        #[no_mangle]
        pub unsafe extern "C" fn $export_ident(string_pointer: radix_engine_toolkit_core::memory::Pointer) -> radix_engine_toolkit_core::memory::Pointer {
            // Loading the request from a string pointer into a request object
            let request = $request_type::new_from_pointer(string_pointer);
            let request = match request {
                Ok(request) => request,
                Err(error) => {
                    return radix_engine_toolkit_core::memory::toolkit_serialize_to_json_string_and_write_to_memory(
                        &error,
                    )
                    .expect("Failed to write a trusted string to memory")
                }
            };

            // Fulfilling the request and either getting back an error or a valid response
            let response = request.fulfill_request();
            match response {
                Ok(response) => {
                    radix_engine_toolkit_core::memory::toolkit_serialize_to_json_string_and_write_to_memory(
                        &response,
                    )
                    .expect("Failed to write a trusted string to memory")
                }
                Err(error) => {
                    radix_engine_toolkit_core::memory::toolkit_serialize_to_json_string_and_write_to_memory(&error)
                        .expect("Failed to write a trusted string to memory")
                }
            }
        }
    };
}

export_request!(InformationRequest as information);

export_request!(ConvertManifestRequest as convert_manifest);

export_request!(CompileTransactionIntentRequest as compile_transaction_intent);
export_request!(CompileSignedTransactionIntentRequest as compile_signed_transaction_intent);
export_request!(CompileNotarizedTransactionIntentRequest as compile_notarized_transaction_intent);

export_request!(DecompileTransactionIntentRequest as decompile_transaction_intent);
export_request!(DecompileSignedTransactionIntentRequest as decompile_signed_transaction_intent);
export_request!(
    DecompileNotarizedTransactionIntentRequest as decompile_notarized_transaction_intent
);
export_request!(DecompileUnknownTransactionIntentRequest as decompile_unknown_transaction_intent);

export_request!(DeriveVirtualAccountAddressRequest as derive_virtual_account_address);

export_request!(EncodeAddressRequest as encode_address);
export_request!(DecodeAddressRequest as decode_address);

export_request!(SBOREncodeRequest as sbor_encode);
export_request!(SBORDecodeRequest as sbor_decode);
