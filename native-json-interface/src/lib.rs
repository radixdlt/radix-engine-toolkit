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

#![allow(clippy::missing_safety_doc)]
#![allow(non_snake_case)]

pub mod native {
    use radix_engine_toolkit::error::Result;
    use radix_engine_toolkit::request::*;
    use serde::{Deserialize, Serialize};

    pub unsafe fn deserialize_from_memory<'a, T: Deserialize<'a>>(
        string_pointer: radix_engine_toolkit::buffer::Pointer,
    ) -> Result<T> {
        std::ffi::CStr::from_ptr(string_pointer as *const std::ffi::c_char)
            .to_str()
            .map_err(radix_engine_toolkit::error::Error::from)
            .and_then(|string| {
                serde_json::from_str(string).map_err(|error| {
                    radix_engine_toolkit::error::Error::InvalidRequestString {
                        message: format!("{:?}", error),
                    }
                })
            })
    }

    pub unsafe fn write_serializable_to_memory<T: Serialize>(
        object: &T,
    ) -> Result<radix_engine_toolkit::buffer::Pointer> {
        serde_json::to_string(object)
            .map_err(
                |error| radix_engine_toolkit::error::Error::InvalidRequestString {
                    message: format!("{:?}", error),
                },
            )
            .map(|string| {
                let object_bytes = string.as_bytes();
                let byte_count = object_bytes.len() + 1;

                let pointer = radix_engine_toolkit::buffer::toolkit_alloc(byte_count);
                pointer.copy_from(
                    [object_bytes, &[0]].concat().as_ptr() as radix_engine_toolkit::buffer::Pointer,
                    byte_count,
                );

                pointer
            })
    }

    macro_rules! export_handler {
        ($handler: ident as $handler_ident: ident) => {
            #[no_mangle]
            pub unsafe extern "C" fn $handler_ident(
                string_pointer: radix_engine_toolkit::buffer::Pointer,
            ) -> radix_engine_toolkit::buffer::Pointer {
                let result_pointers = deserialize_from_memory(string_pointer)
                    .and_then($handler::fulfill)
                    .and_then(|response| write_serializable_to_memory(&response))
                    .map_err(|error| {
                        write_serializable_to_memory(&error)
                            .expect("Failed to serialize error which is a trusted object")
                    });
                match result_pointers {
                    Ok(pointer) => pointer,
                    Err(pointer) => pointer,
                }
            }
        };
    }

    export_handler!(InformationHandler as information);

    export_handler!(ConvertManifestHandler as convert_manifest);
    export_handler!(AnalyzeManifestHandler as analyze_manifest);
    #[cfg(feature = "radix-engine")]
    export_handler!(
        AnalyzeManifestWithPreviewContextHandler as analyze_manifest_with_preview_context
    );

    export_handler!(CompileTransactionIntentHandler as compile_transaction_intent);
    export_handler!(CompileSignedTransactionIntentHandler as compile_signed_transaction_intent);
    export_handler!(CompileNotarizedTransactionHandler as compile_notarized_transaction);

    export_handler!(DecompileTransactionIntentHandler as decompile_transaction_intent);
    export_handler!(DecompileSignedTransactionIntentHandler as decompile_signed_transaction_intent);
    export_handler!(DecompileNotarizedTransactionHandler as decompile_notarized_transaction);
    export_handler!(
        DecompileUnknownTransactionIntentHandler as decompile_unknown_transaction_intent
    );

    export_handler!(
        DeriveBabylonAddressFromOlympiaAddressHandler
            as derive_babylon_address_from_olympia_address
    );
    export_handler!(DeriveVirtualAccountAddressHandler as derive_virtual_account_address);
    export_handler!(DeriveVirtualIdentityAddressHandler as derive_virtual_identity_address);
    export_handler!(
        DeriveNonFungibleGlobalIdFromPublicKeyHandler
            as derive_non_fungible_global_id_from_public_key
    );

    export_handler!(EncodeAddressHandler as encode_address);
    export_handler!(DecodeAddressHandler as decode_address);

    export_handler!(SborEncodeHandler as sbor_encode);
    export_handler!(SborDecodeHandler as sbor_decode);

    export_handler!(KnownEntityAddressesHandler as known_entity_addresses);
    export_handler!(StaticallyValidateTransactionHandler as statically_validate_transaction);

    export_handler!(HashHandler as hash);
}

#[cfg(feature = "jni")]
pub mod jni {
    use radix_engine_toolkit::error::Result;
    use radix_engine_toolkit::request::*;
    use serde::Serialize;

    fn serialize_to_jstring<T: Serialize>(
        env: jni::JNIEnv,
        object: &T,
    ) -> Result<jni::sys::jstring> {
        serde_json::to_string(object)
            .map_err(
                |error| radix_engine_toolkit::error::Error::InvalidRequestString {
                    message: format!("{:?}", error),
                },
            )
            .and_then(|string| {
                env.new_string(&string).map_err(|error| {
                    radix_engine_toolkit::error::Error::InvalidRequestString {
                        message: format!("{:?}", error),
                    }
                })
            })
            .map(|object| object.into_raw())
    }

    macro_rules! export_handler {
        ($handler: ident as $handler_ident: ident) => {
            #[no_mangle]
            pub unsafe extern "system" fn $handler_ident(
                env: jni::JNIEnv,
                _: jni::objects::JClass,
                input: jni::objects::JString,
            ) -> jni::sys::jstring {
                let result_strings = env
                    .get_string(input)
                    .map_err(
                        |error| radix_engine_toolkit::error::Error::InvalidRequestString {
                            message: format!("{:?}", error),
                        },
                    )
                    .and_then(|string_object| {
                        serde_json::from_str(&String::from(string_object)).map_err(|error| {
                            radix_engine_toolkit::error::Error::InvalidRequestString {
                                message: format!("{:?}", error),
                            }
                        })
                    })
                    .and_then($handler::fulfill)
                    .and_then(|response| serialize_to_jstring(env, &response))
                    .map_err(|error| {
                        serialize_to_jstring(env, &error)
                            .expect("Failed to convert a trusted payload to jstring")
                    });

                match result_strings {
                    Ok(string) => string,
                    Err(string) => string,
                }
            }
        };
    }

    export_handler!(
        InformationHandler as Java_com_radixdlt_toolkit_RadixEngineToolkitFFI_information
    );

    export_handler!(
        ConvertManifestHandler as Java_com_radixdlt_toolkit_RadixEngineToolkitFFI_convertManifest
    );
    export_handler!(
        AnalyzeManifestHandler as Java_com_radixdlt_toolkit_RadixEngineToolkitFFI_analyzeManifest
    );
    #[cfg(feature = "radix-engine")]
    export_handler!(
        AnalyzeManifestWithPreviewContextHandler
            as Java_com_radixdlt_toolkit_RadixEngineToolkitFFI_analyzeManifestWithPreviewContext
    );

    export_handler!(
        CompileTransactionIntentHandler
            as Java_com_radixdlt_toolkit_RadixEngineToolkitFFI_compileTransactionIntent
    );
    export_handler!(
        CompileSignedTransactionIntentHandler
            as Java_com_radixdlt_toolkit_RadixEngineToolkitFFI_compileSignedTransactionIntent
    );
    export_handler!(
        CompileNotarizedTransactionHandler
            as Java_com_radixdlt_toolkit_RadixEngineToolkitFFI_compileNotarizedTransaction
    );

    export_handler!(
        DecompileTransactionIntentHandler
            as Java_com_radixdlt_toolkit_RadixEngineToolkitFFI_decompileTransactionIntent
    );
    export_handler!(
        DecompileSignedTransactionIntentHandler
            as Java_com_radixdlt_toolkit_RadixEngineToolkitFFI_decompileSignedTransactionIntent
    );
    export_handler!(
        DecompileNotarizedTransactionHandler
            as Java_com_radixdlt_toolkit_RadixEngineToolkitFFI_decompileNotarizedTransaction
    );
    export_handler!(
        DecompileUnknownTransactionIntentHandler
            as Java_com_radixdlt_toolkit_RadixEngineToolkitFFI_decompileUnknownTransactionIntent
    );

    export_handler!(
        DeriveBabylonAddressFromOlympiaAddressHandler
            as Java_com_radixdlt_toolkit_RadixEngineToolkitFFI_deriveBabylonAddressFromOlympiaAddress
    );
    export_handler!(
        DeriveVirtualAccountAddressHandler
            as Java_com_radixdlt_toolkit_RadixEngineToolkitFFI_deriveVirtualAccountAddress
    );
    export_handler!(
        DeriveVirtualIdentityAddressHandler
            as Java_com_radixdlt_toolkit_RadixEngineToolkitFFI_deriveVirtualIdentityAddress
    );
    export_handler!(DeriveNonFungibleGlobalIdFromPublicKeyHandler as Java_com_radixdlt_toolkit_RadixEngineToolkitFFI_deriveNonFungibleGlobalIdFromPublicKey);

    export_handler!(
        EncodeAddressHandler as Java_com_radixdlt_toolkit_RadixEngineToolkitFFI_encodeAddress
    );
    export_handler!(
        DecodeAddressHandler as Java_com_radixdlt_toolkit_RadixEngineToolkitFFI_decodeAddress
    );

    export_handler!(
        SborEncodeHandler as Java_com_radixdlt_toolkit_RadixEngineToolkitFFI_sborEncode
    );
    export_handler!(
        SborDecodeHandler as Java_com_radixdlt_toolkit_RadixEngineToolkitFFI_sborDecode
    );

    export_handler!(
        KnownEntityAddressesHandler
            as Java_com_radixdlt_toolkit_RadixEngineToolkitFFI_knownEntityAddresses
    );
    export_handler!(
        StaticallyValidateTransactionHandler
            as Java_com_radixdlt_toolkit_RadixEngineToolkitFFI_staticallyValidateTransaction
    );

    export_handler!(HashHandler as Java_com_radixdlt_toolkit_RadixEngineToolkitFFI_hash);
}
