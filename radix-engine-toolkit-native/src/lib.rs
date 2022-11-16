use serde::{Serialize};

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
        pub unsafe extern "C" fn $export_ident(
            string_pointer: radix_engine_toolkit_core::memory::Pointer
        ) -> radix_engine_toolkit_core::memory::Pointer {
            // Loading the request from a string pointer into a request object
            let request: Result<$request_type, _> = $request_type::new_from_pointer(string_pointer);
            let request: $request_type = match request {
                Ok(request) => request,
                Err(error) => {
                    return serialize_to_json_string_and_write_to_memory(
                        &error,
                    )
                    .expect("Failed to write a trusted string to memory")
                }
            };

            // Fulfilling the request and either getting back an error or a valid response
            let response: Result<_, _> = request.fulfill_request();
            match response {
                Ok(response) => {
                    serialize_to_json_string_and_write_to_memory(
                        &response,
                    )
                    .expect("Failed to write a trusted string to memory")
                }
                Err(error) => {
                    serialize_to_json_string_and_write_to_memory(&error)
                        .expect("Failed to write a trusted string to memory")
                }
            }
        }
    };
}

extern "C" {
    /// Allocates memory of a certain size through the client's memory allocator.
    ///
    /// This function allows all memory allocation for the library requests and responses to happen
    /// at the client side through the client's memory allocator. This gives us the advantage of the
    /// client now being in complete control of the request and response pointers and being able to
    /// deallocate them through their native allocator when needed. 
    pub fn client_allocate_memory(capacity: usize) -> radix_engine_toolkit_core::memory::Pointer;
}

pub unsafe fn serialize_to_json_string_and_write_to_memory<T>(
    object: &T,
) -> Result<radix_engine_toolkit_core::memory::Pointer, radix_engine_toolkit_core::error::Error>
where
    T: Serialize,
{
    let object_string: String = serde_json::to_string(object)?;
    let object_bytes: &[u8] = object_string.as_bytes();
    let byte_count: usize = object_bytes.len() + 1;
    let bytes: Vec<u8> = [object_bytes, &[0]].concat();

    let pointer: radix_engine_toolkit_core::memory::Pointer = client_allocate_memory(byte_count);
    pointer.copy_from(bytes.as_ptr(), byte_count);
    Ok(pointer)
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

export_request!(DeriveNonFungibleAddressRequest as derive_non_fungible_address);
export_request!(
    DeriveNonFungibleAddressFromPublicKeyRequest as derive_non_fungible_address_from_public_key
);

export_request!(DeriveVirtualAccountAddressRequest as derive_virtual_account_address);

export_request!(EncodeAddressRequest as encode_address);
export_request!(DecodeAddressRequest as decode_address);

export_request!(SBOREncodeRequest as sbor_encode);
export_request!(SBORDecodeRequest as sbor_decode);
