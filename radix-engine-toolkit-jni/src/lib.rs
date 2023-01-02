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
        paste::paste!{
            /// This function exports a request function and builds a wrapper for it.
            #[no_mangle]
            pub extern "system" fn [< Java_com_radixdlt_toolkit_RadixEngineToolkitFFI_ $export_ident >](
                env: jni::JNIEnv,
                _class: jni::objects::JClass,
                input: jni::objects::JString,
            ) -> jni::sys::jstring {
                // Load the Java String
                let input: String = env.get_string(input)
                    .expect("Failed to load Java string!")
                    .into();

                // Deserialize the request string for the request type
                let request = serde_json::from_str(&input);
                let request: $request_type = match request {
                    Ok(request) => request,
                    Err(error) => return $crate::serialize_to_jstring!{env, radix_engine_toolkit_core::error::Error::from(error)}
                };

                // Fulfilling the request and either getting back an error or a valid response
                let response = request.fulfill_request();
                match response {
                    Ok(response) => $crate::serialize_to_jstring!{env, response},
                    Err(error) => $crate::serialize_to_jstring!{env, error},
                }
            }
        }
    };
}

#[macro_export]
macro_rules! serialize_to_jstring {
    ($env: expr, $value: expr) => {
        $env.new_string(
            serde_json::to_string(&$value).expect("Unable to serialize a trusted payload"),
        )
        .expect("Could not create a JString from a trusted payload")
        .into_raw()
    };
}

export_request!(InformationRequest as information);

export_request!(ConvertManifestRequest as convertManifest);

export_request!(CompileTransactionIntentRequest as compileTransactionIntent);
export_request!(CompileSignedTransactionIntentRequest as compileSignedTransactionIntent);
export_request!(CompileNotarizedTransactionIntentRequest as compileNotarizedTransactionIntent);

export_request!(DecompileTransactionIntentRequest as decompileTransactionIntent);
export_request!(DecompileSignedTransactionIntentRequest as decompileSignedTransactionIntent);
export_request!(DecompileNotarizedTransactionIntentRequest as decompileNotarizedTransactionIntent);
export_request!(DecompileUnknownTransactionIntentRequest as decompileUnknownTransactionIntent);

export_request!(DeriveVirtualAccountAddressRequest as deriveVirtualAccountAddress);

export_request!(EncodeAddressRequest as encodeAddress);
export_request!(DecodeAddressRequest as decodeAddress);

export_request!(SBOREncodeRequest as sborEncode);
export_request!(SBORDecodeRequest as sborDecode);

export_request!(KnownEntityAddressesRequest as KnownEntityAddresses);
