use radix_engine_toolkit::request::*;
use serde::{de::DeserializeOwned, Serialize};

mod ffi {
    pub type Pointer = *mut std::ffi::c_char;

    extern "C" {
        pub fn information(pointer: Pointer) -> Pointer;
        pub fn convert_manifest(pointer: Pointer) -> Pointer;
        pub fn analyze_manifest(pointer: Pointer) -> Pointer;
        pub fn analyze_manifest_with_preview_context(pointer: Pointer) -> Pointer;
        pub fn compile_transaction_intent(pointer: Pointer) -> Pointer;
        pub fn compile_signed_transaction_intent(pointer: Pointer) -> Pointer;
        pub fn compile_notarized_transaction(pointer: Pointer) -> Pointer;
        pub fn decompile_transaction_intent(pointer: Pointer) -> Pointer;
        pub fn decompile_signed_transaction_intent(pointer: Pointer) -> Pointer;
        pub fn decompile_notarized_transaction(pointer: Pointer) -> Pointer;
        pub fn decompile_unknown_transaction_intent(pointer: Pointer) -> Pointer;
        pub fn derive_babylon_address_from_olympia_address(pointer: Pointer) -> Pointer;
        pub fn derive_virtual_account_address(pointer: Pointer) -> Pointer;
        pub fn derive_virtual_identity_address(pointer: Pointer) -> Pointer;
        pub fn encode_address(pointer: Pointer) -> Pointer;
        pub fn decode_address(pointer: Pointer) -> Pointer;
        pub fn sbor_encode(pointer: Pointer) -> Pointer;
        pub fn sbor_decode(pointer: Pointer) -> Pointer;
        pub fn known_entity_addresses(pointer: Pointer) -> Pointer;
        pub fn statically_validate_transaction(pointer: Pointer) -> Pointer;
        pub fn hash(pointer: Pointer) -> Pointer;

        pub fn toolkit_alloc(capacity: usize) -> Pointer;
        pub fn toolkit_free_c_string(pointer: Pointer);
    }
}

#[derive(Debug)]
pub struct Error;

pub struct RadixEngineToolkit;
impl RadixEngineToolkit {
    fn invoke_fn<S: Serialize, D: DeserializeOwned, F>(function: F, request: S) -> Result<D, Error>
    where
        F: Fn(ffi::Pointer) -> ffi::Pointer,
    {
        let request_pointer = Self::write_object_to_memory(request);
        let response_pointer = function(request_pointer);

        let response_string = Self::read_string(response_pointer)?;
        let response = if let Ok(response) = Self::deserialize::<D, _>(&response_string) {
            Ok(response)
        } else if let Ok(response) =
            Self::deserialize::<radix_engine_toolkit::error::RETError, _>(&response_string)
        {
            println!("{:?}", response);
            Err(Error)
        } else {
            return Err(Error);
        };

        Self::free_memory(request_pointer);
        Self::free_memory(response_pointer);

        response
    }

    fn write_object_to_memory<S: Serialize>(object: S) -> ffi::Pointer {
        let serialized_object = Self::serialize(object);
        let pointer = Self::allocate_memory_for_string(&serialized_object);
        Self::write_string(serialized_object, pointer);
        pointer
    }

    fn serialize<S: Serialize>(object: S) -> String {
        serde_json::to_string(&object).expect("Could not serialize a trusted payload")
    }

    fn deserialize<D: DeserializeOwned, S: AsRef<str>>(string: S) -> Result<D, Error> {
        let str = string.as_ref();
        serde_json::from_str(str).map_err(|_| Error)
    }

    fn write_string<S: AsRef<str>>(string: S, pointer: ffi::Pointer) {
        let string = string.as_ref();
        let mut string_bytes = string.as_bytes().to_vec();
        string_bytes.push(0);

        unsafe {
            pointer.copy_from(
                string_bytes.as_ptr() as radix_engine_toolkit::buffer::Pointer,
                string_bytes.len(),
            );
        }
    }

    fn read_string(pointer: ffi::Pointer) -> Result<String, Error> {
        unsafe {
            std::ffi::CStr::from_ptr(pointer as *const std::ffi::c_char)
                .to_str()
                .map_err(|_| Error)
                .map(|string| string.to_owned())
        }
    }

    fn allocate_memory_for_string<S: AsRef<str>>(string: S) -> ffi::Pointer {
        let string = string.as_ref();
        let byte_count = string.len();
        Self::allocate_memory_by_capacity(byte_count + 1)
    }

    fn allocate_memory_by_capacity(capacity: usize) -> ffi::Pointer {
        unsafe { ffi::toolkit_alloc(capacity) }
    }

    fn free_memory(pointer: ffi::Pointer) {
        unsafe { ffi::toolkit_free_c_string(pointer) }
    }
}

pub trait Invoke<T: Serialize> {
    type Output: DeserializeOwned;

    fn invoke(request: T) -> Result<Self::Output, Error>;
}

macro_rules! impl_invoke {
    ($request: path, $response: path, $fn_ident: path) => {
        impl Invoke<$request> for RadixEngineToolkit {
            type Output = $response;

            fn invoke(request: $request) -> Result<Self::Output, Error> {
                Self::invoke_fn(|pointer| unsafe { $fn_ident(pointer) }, request)
            }
        }
    };
}

impl_invoke! { InformationRequest, InformationResponse, ffi::information }
impl_invoke! { ConvertManifestRequest, ConvertManifestResponse, ffi::convert_manifest }
impl_invoke! { AnalyzeManifestRequest, AnalyzeManifestResponse, ffi::analyze_manifest }
impl_invoke! { AnalyzeManifestWithPreviewContextRequest, AnalyzeManifestWithPreviewContextResponse, ffi::analyze_manifest_with_preview_context }
impl_invoke! { CompileTransactionIntentRequest, CompileTransactionIntentResponse, ffi::compile_transaction_intent }
impl_invoke! { CompileSignedTransactionIntentRequest, CompileSignedTransactionIntentResponse, ffi::compile_signed_transaction_intent }
impl_invoke! { CompileNotarizedTransactionRequest, CompileNotarizedTransactionResponse, ffi::compile_notarized_transaction }
impl_invoke! { DecompileTransactionIntentRequest, DecompileTransactionIntentResponse, ffi::decompile_transaction_intent }
impl_invoke! { DecompileSignedTransactionIntentRequest, DecompileSignedTransactionIntentResponse, ffi::decompile_signed_transaction_intent }
impl_invoke! { DecompileNotarizedTransactionRequest, DecompileNotarizedTransactionResponse, ffi::decompile_notarized_transaction }
impl_invoke! { DecompileUnknownTransactionIntentRequest, DecompileUnknownTransactionIntentResponse, ffi::decompile_unknown_transaction_intent }
impl_invoke! { DeriveBabylonAddressFromOlympiaAddressRequest, DeriveBabylonAddressFromOlympiaAddressResponse, ffi::derive_babylon_address_from_olympia_address }
impl_invoke! { DeriveVirtualAccountAddressRequest, DeriveVirtualAccountAddressResponse, ffi::derive_virtual_account_address }
impl_invoke! { DeriveVirtualIdentityAddressRequest, DeriveVirtualIdentityAddressResponse, ffi::derive_virtual_identity_address }
impl_invoke! { EncodeAddressRequest, EncodeAddressResponse, ffi::encode_address }
impl_invoke! { DecodeAddressRequest, DecodeAddressResponse, ffi::decode_address }
impl_invoke! { SborEncodeRequest, SborEncodeResponse, ffi::sbor_encode }
impl_invoke! { SborDecodeRequest, SborDecodeResponse, ffi::sbor_decode }
impl_invoke! { KnownEntityAddressesRequest, KnownEntityAddressesResponse, ffi::known_entity_addresses }
impl_invoke! { StaticallyValidateTransactionRequest, StaticallyValidateTransactionResponse, ffi::statically_validate_transaction }
impl_invoke! { HashRequest, HashResponse, ffi::hash }
