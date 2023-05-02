use radix_engine_toolkit::functions::*;
use serde::{de::DeserializeOwned, Serialize};

mod ffi {
    pub type Pointer = *mut std::ffi::c_char;

    extern "C" {
        pub fn information(pointer: Pointer) -> Pointer;
        pub fn convert_manifest(pointer: Pointer) -> Pointer;
        pub fn extract_addresses_from_manifest(pointer: Pointer) -> Pointer;
        pub fn analyze_transaction_execution(pointer: Pointer) -> Pointer;
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
    fn invoke_fn<S: Serialize, D: DeserializeOwned, F>(function: F, input: S) -> Result<D, Error>
    where
        F: Fn(ffi::Pointer) -> ffi::Pointer,
    {
        let input_pointer = Self::write_object_to_memory(input);
        let output_pointer = function(input_pointer);

        let output_string = Self::read_string(output_pointer)?;
        let output = if let Ok(output) = Self::deserialize::<D, _>(&output_string) {
            Ok(output)
        } else if let Ok(output) =
            Self::deserialize::<radix_engine_toolkit::error::RETError, _>(&output_string)
        {
            println!("{:?}", output);
            Err(Error)
        } else {
            return Err(Error);
        };

        Self::free_memory(input_pointer);
        Self::free_memory(output_pointer);

        output
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

    fn invoke(input: T) -> Result<Self::Output, Error>;
}

macro_rules! impl_invoke {
    ($input:path, $output:path, $fn_ident: path) => {
        impl Invoke<$input> for RadixEngineToolkit {
            type Output = $output;

            fn invoke(input: $input) -> Result<Self::Output, Error> {
                Self::invoke_fn(|pointer| unsafe { $fn_ident(pointer) }, input)
            }
        }
    };
}

impl_invoke! { information::Input, information::Output, ffi::information }
impl_invoke! { convert_manifest::Input, convert_manifest::Output, ffi::convert_manifest }
impl_invoke! { extract_addresses_from_manifest::Input, extract_addresses_from_manifest::Output, ffi::extract_addresses_from_manifest }
impl_invoke! { analyze_transaction_execution::Input, analyze_transaction_execution::Output, ffi::analyze_transaction_execution }
impl_invoke! { compile_transaction_intent::Input, compile_transaction_intent::Output, ffi::compile_transaction_intent }
impl_invoke! { compile_signed_transaction_intent::Input, compile_signed_transaction_intent::Output, ffi::compile_signed_transaction_intent }
impl_invoke! { compile_notarized_transaction::Input, compile_notarized_transaction::Output, ffi::compile_notarized_transaction }
impl_invoke! { decompile_transaction_intent::Input, decompile_transaction_intent::Output, ffi::decompile_transaction_intent }
impl_invoke! { decompile_signed_transaction_intent::Input, decompile_signed_transaction_intent::Output, ffi::decompile_signed_transaction_intent }
impl_invoke! { decompile_notarized_transaction::Input, decompile_notarized_transaction::Output, ffi::decompile_notarized_transaction }
impl_invoke! { decompile_unknown_intent::Input, decompile_unknown_intent::Output, ffi::decompile_unknown_transaction_intent }
impl_invoke! { derive_babylon_address_from_olympia_address::Input, derive_babylon_address_from_olympia_address::Output, ffi::derive_babylon_address_from_olympia_address }
impl_invoke! { derive_virtual_account_address::Input, derive_virtual_account_address::Output, ffi::derive_virtual_account_address }
impl_invoke! { derive_virtual_identity_address::Input, derive_virtual_identity_address::Output, ffi::derive_virtual_identity_address }
impl_invoke! { encode_address::Input, encode_address::Output, ffi::encode_address }
impl_invoke! { decode_address::Input, decode_address::Output, ffi::decode_address }
impl_invoke! { sbor_encode::Input, sbor_encode::Output, ffi::sbor_encode }
impl_invoke! { sbor_decode::Input, sbor_decode::Output, ffi::sbor_decode }
impl_invoke! { known_entity_addresses::Input, known_entity_addresses::Output, ffi::known_entity_addresses }
impl_invoke! { statically_validate_transaction::Input, statically_validate_transaction::Output, ffi::statically_validate_transaction }
impl_invoke! { hash::Input, hash::Output, ffi::hash }
