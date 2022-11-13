//! This module defines the struct and implementation for a [`RadixEngineToolkit`] WASM wrapper that
//! uses the WasmTime runtime. This struct is mainly defined for the purpose of testing out the
//! behavior of the Radix Engine Toolkit when it is running through a WASM host.

use serde::de::DeserializeOwned;
use serde::Serialize;

use std::path::{Path, PathBuf};

use radix_engine_toolkit::error::Error;
use radix_engine_toolkit::requests::*;

use wasmtime::{AsContextMut, Engine, Instance, Linker, Memory, Module, Store, TypedFunc};

// ==========
// Type Defs
// ==========

/// A shortcut type that defines a `Result` that returns the generic `T` or a `WrapperError`.
type Result<T> = std::result::Result<T, WrapperError>;

// ================
// Library Wrapper
// ================

pub struct RadixEngineToolkit {
    engine: Engine,
    module: Module,
    linker: Linker<i32>,
    store: Store<i32>,
    instance: Instance,
    function_store: RadixEngineToolkitFunctions,
}

impl RadixEngineToolkit {
    /// Creates a new [RadixEngineToolkit] object from a given path.
    ///
    /// This function is able to instantiate a new [RadixEngineToolkit] given the path to the
    /// library's WASM module. This function reads the module and then creates a new object.
    ///
    /// # Checks
    ///
    /// 1. This function checks that the path provided is a valid path to an already existing file
    ///
    /// # Arguments
    ///
    /// - `path: AsRef<Path>`: A generic object which can be referenced as a `Path`.
    ///
    /// # Returns
    ///
    /// - `Result<Self, WrapperError>`: A new object of [RadixEngineToolkit] is returned, or a
    /// [WrapperError] is returned in the case of an error.
    pub fn new_from_module_path<T: AsRef<Path>>(path: T) -> Result<Self> {
        // Get the `Path` from the generic object.
        let path: &Path = path.as_ref();

        // Check that the path points to a file, if not, then return a FileNotFound error.
        if !path.exists() {
            return Err(WrapperError::FileNotFoundError(path.into()));
        };

        // Read the file contents and then call the bytes constructor to continue the process
        let buffer: Vec<u8> = std::fs::read(path).map_err(WrapperError::FileReadingError)?;
        Self::new_from_module_bytes(buffer)
    }

    /// Creates a new [RadixEngineToolkit] object from a module byte array.
    ///
    /// This function is able to instantiate a new [RadixEngineToolkit] given the contents of the
    /// WASM file.
    ///
    /// # Arguments
    ///
    /// - `bytes` [`AsRef<[u8]>`] - A generic object which can be referenced as a `[u8]`.
    ///
    /// # Returns
    ///
    /// - [`Result<Self, WrapperError>`]: A new object of [RadixEngineToolkit] is returned, or a
    /// [WrapperError] is returned in the case of an error.
    pub fn new_from_module_bytes<T: AsRef<[u8]>>(bytes: T) -> Result<Self> {
        // Getting the `&[u8]` from the generic object.
        let bytes: &[u8] = bytes.as_ref();

        // Creating the required WASM host objects to run the Radix Engine Toolkit WASM.
        let engine: Engine = Engine::default();
        let module: Module = Module::new(&engine, bytes).map_err(WrapperError::WasmTimeError)?;
        let linker: Linker<i32> = Linker::new(&engine);
        let mut store: Store<i32> = Store::new(&engine, 4);
        let instance: Instance = linker
            .instantiate(&mut store, &module)
            .map_err(WrapperError::WasmTimeError)?;
        let function_store: RadixEngineToolkitFunctions =
            RadixEngineToolkitFunctions::new(&instance, &mut store)?;

        let radix_engine_toolkit: Self = Self {
            engine,
            module,
            linker,
            store,
            instance,
            function_store,
        };
        Ok(radix_engine_toolkit)
    }

    /// Creates a new [RadixEngineToolkit] object from source code.
    ///
    /// This function compiles the [RadixEngineToolkit] as to a `wasm32-unknown-unknown` target and
    /// then uses the `new_from_module_path` constructor to create a new [RadixEngineToolkit] object
    ///
    /// # Returns
    ///
    /// - [`Result<Self, WrapperError>`]: A new object of [RadixEngineToolkit] is returned, or a
    /// [`WrapperError`]
    pub fn new_compile_from_source() -> Result<Self> {
        // The path to the directory containing the Cargo.toml manifest file
        let manifest_directory: PathBuf =
            std::env::current_dir().expect("Failed to get the path of the current directory");

        // Build the Radix Engine Toolkit from source - Build a wasm32-unknown-unknown binary for
        // release.
        let status: std::process::ExitStatus = std::process::Command::new("cargo")
            .current_dir(&manifest_directory)
            .args(["build", "--target", "wasm32-unknown-unknown", "--release"])
            .status()
            .expect("Compilation of WASM for tests failed");
        if !status.success() {
            panic!("Failed to compile package: {:?}", &manifest_directory);
        };

        // Building a path to the WASM file
        let wasm_module_path: PathBuf = manifest_directory
            .join("target/wasm32-unknown-unknown/release/radix_engine_toolkit.wasm");
        Self::new_from_module_path(wasm_module_path)
    }

    crate::define_request_function! {InformationRequest, InformationResponse, information}
    crate::define_request_function! {ConvertManifestRequest, ConvertManifestResponse, convert_manifest}
    crate::define_request_function! {CompileTransactionIntentRequest, CompileTransactionIntentResponse, compile_transaction_intent}
    crate::define_request_function! {DecompileTransactionIntentRequest, DecompileTransactionIntentResponse, decompile_transaction_intent}
    crate::define_request_function! {CompileSignedTransactionIntentRequest, CompileSignedTransactionIntentResponse, compile_signed_transaction_intent}
    crate::define_request_function! {DecompileSignedTransactionIntentRequest, DecompileSignedTransactionIntentResponse, decompile_signed_transaction_intent}
    crate::define_request_function! {CompileNotarizedTransactionIntentRequest, CompileNotarizedTransactionIntentResponse, compile_notarized_transaction_intent}
    crate::define_request_function! {DecompileNotarizedTransactionIntentRequest, DecompileNotarizedTransactionIntentResponse, decompile_notarized_transaction_intent}
    crate::define_request_function! {DecompileUnknownTransactionIntentRequest, DecompileUnknownTransactionIntentResponse, decompile_unknown_transaction_intent}
    crate::define_request_function! {DecodeAddressRequest, DecodeAddressResponse, decode_address}
    crate::define_request_function! {EncodeAddressRequest, EncodeAddressResponse, encode_address}
    crate::define_request_function! {SBORDecodeRequest, SBORDecodeResponse, sbor_decode}
    crate::define_request_function! {SBOREncodeRequest, SBOREncodeResponse, sbor_encode}
    crate::define_request_function! {DeriveNonFungibleAddressFromPublicKeyRequest, DeriveNonFungibleAddressFromPublicKeyResponse, derive_non_fungible_address_from_public_key}
    crate::define_request_function! {DeriveNonFungibleAddressRequest, DeriveNonFungibleAddressResponse, derive_non_fungible_address}
    crate::define_request_function! {DeriveVirtualAccountAddressRequest, DeriveVirtualAccountAddressResponse, derive_virtual_account_address}

    /// Calls a function in the WASM instance with a given request
    ///
    /// This is a high level method which is used to call functions in the WASM instance while
    /// abstracting away the memory allocation, serialization, writing of objects, and all of the
    /// other steps. This can be thought of as the main router which all requests to the transaction
    /// library go through.
    ///
    /// At a high level, this function does the following:
    ///
    /// 1. Serializes the request.
    /// 2. Allocates enough memory for the C-String representation of the serialized request.
    /// 3. Writes this request to linear memory.
    /// 4. Invokes the WASM function.
    /// 5. Reads the response string at the pointer returned by the WASM function
    /// 6. Attempts to deserialize the response as `D`. If that fails, then the response is assumed
    /// to be an [Error] response and therefore it attempts to deserialize it as such.
    /// 7. Frees up the memory allocated for the request and the response strings.
    /// 8. Returns the deserialized object.
    ///
    /// # Arguments
    ///
    /// - `function` [TypedFunc<i32, i32>] - The function to invoke on the WASM instance. This
    /// function should take an [i32] and return an [i32]. By default, the arguments and the returns
    /// are the memory offsets of the request and the response respectively in the WASM's linear
    /// memory.
    /// - `request`: [Serialize]: A generic object that implements serde's [Serialize] trait and
    /// therefore can be serialized to a string. This is the request payload that the `function`
    /// will be called with.
    ///
    /// # Returns
    ///
    /// - `Result<std::result::Result<D, Error>>`: This method has a complex return type mainly due
    /// to the nature
    fn call_wasm_function<S: Serialize, D: DeserializeOwned>(
        &mut self,
        function: TypedFunc<i32, i32>,
        request: S,
    ) -> Result<D> {
        // Write the request to the WASM's linear memory
        let request_memory_offset: i32 = self.write_object_to_memory(request)?;

        // Call the function using the provided request memory offset
        let response_memory_offset: i32 = function
            .call(&mut self.store, request_memory_offset)
            .map_err(WrapperError::WasmTimeTrapError)?;

        // The response is either of type `D` or of type `Error`. So, we attempt to decode it as
        // both
        let response_string: String = self.read_string(response_memory_offset)?;
        let response: Result<D> = if let Ok(response) = Self::deserialize::<D, _>(&response_string)
        {
            Ok(response)
        } else if let Ok(response) = Self::deserialize::<Error, _>(&response_string) {
            Err(WrapperError::LibraryError(response))
        } else {
            return Err(WrapperError::DeserializationError);
        };

        // Free up the allocated memory for the request and the response
        self.free_memory(request_memory_offset)?;
        self.free_memory(response_memory_offset)?;

        response
    }

    /// Writes an object to linear memory
    ///
    /// This is a higher level method used to serialize, allocate memory, and eventually write an
    /// object to the WASM's linear memory. This method returns of the offset at which the C-String
    /// UTF-8 encoded representation of the serialized object is stored.
    ///
    /// # Arguments
    ///
    /// - `object: Serialize`: A generic object which implements the [Serialize] trait and therefore
    /// can be serialized using serde.
    ///
    /// # Returns
    ///
    /// - `Result<i32>`: An [i32] is returned if the memory allocation is successful, otherwise, a
    /// [WrapperError] is returned.
    fn write_object_to_memory<S: Serialize>(&mut self, object: S) -> Result<i32> {
        let serialized_object: String = Self::serialize(object);
        let memory_offset: i32 = self.allocate_memory_for_string(&serialized_object)?;
        self.write_string(serialized_object, memory_offset)?;
        Ok(memory_offset)
    }

    /// Reads and deserializes an object stored in the WASM's linear memory.
    ///
    /// This is a higher level function used to read and deserialize an object stored in the WASM's
    /// linear memory as a JSON C-String encoded in UTF-8. Given the memory location, this function
    ///
    /// # Arguments
    ///
    /// - `memory_offset: i32`: A memory offset where the string is stored in the instance's linear
    /// memory
    ///
    /// # Returns:
    ///
    /// `Result<D>`: A generic object that implements [Deserialize] is returned if the reading and
    /// deserialization are successful, otherwise, a [WrapperError] is returned.
    fn read_object_from_memory<D: DeserializeOwned>(&mut self, memory_offset: i32) -> Result<D> {
        let string: String = self.read_string(memory_offset)?;
        Self::deserialize(&string)
    }

    /// Serializes an object to a JSON string
    ///
    /// # Arguments
    ///
    /// - `object` [`Serialize`] - A generic object of any type that implements the [Serialize]
    /// trait.
    ///
    /// # Returns
    ///
    /// - [`String`]: A JSON string of the serialized object
    fn serialize<S: Serialize>(object: S) -> String {
        serde_json::to_string(&object).expect("Could not serialize a trusted payload")
    }

    /// Deserializes an object from JSON string to the generic `D`.
    ///
    /// This is a generic function capable of deserializing any input string to type specified by
    /// the generic `D`.
    ///
    /// # Arguments
    ///
    /// - `string: AsRef<str>>`: Any object which can implements the `AsRef<str>` trait.
    ///
    /// # Returns
    ///
    /// - `Result<D>`: A result response containing an object of type `D` if the deserialization
    /// succeeded.
    fn deserialize<D: DeserializeOwned, S: AsRef<str>>(string: S) -> Result<D> {
        let str: &str = string.as_ref();
        serde_json::from_str(str)
            .map_err(|error| WrapperError::RadixEngineToolkitError(Error::from(error)))
    }

    /// Writes a string to the WASM's linear memory.
    ///
    /// This function, takes a string to writes its C-String representation to linear memory at the
    /// specified memory offset to write the string at.
    ///
    /// # Arguments
    ///
    /// - `string: AsRef<str>`: Any object which can implements the `AsRef<str>` trait.
    /// - `memory_offset: i32`: An `i32` of the memory offset to write the string at.
    ///
    /// # Returns
    ///
    /// - `Result<()>`: A result is returned of either unit in the case of successful writing, or
    /// a [WrapperError] in the case of the string writing failing.
    ///
    /// # Note
    ///
    /// It is assumed that memory offset value is obtained through a call to the `allocate_memory`
    /// function on this class.
    fn write_string<S: AsRef<str>>(&mut self, string: S, memory_offset: i32) -> Result<()> {
        // Converting the string to a C String and writing
        let string: &str = string.as_ref();
        let mut string_bytes: Vec<u8> = string.as_bytes().to_vec();
        string_bytes.push(0);

        self.get_memory()
            .write(&mut self.store, memory_offset as usize, &string_bytes)
            .map_err(WrapperError::MemoryAccessError)?;
        Ok(())
    }

    /// Reads a C-String from the given memory offset.
    ///
    /// This function takes a memory offset and reads a null terminated UTF-8 encoded string
    /// beginning from this memory offset.
    ///
    /// # Arguments
    ///
    /// - `memory_offset: i32`: A memory offset where the string is stored in the instance's linear
    /// memory
    ///
    /// # Returns:
    ///
    /// `Result<String>`: If the reading is successful, then a [String] is returned, otherwise, a
    /// [WrapperError] is returned.
    fn read_string(&mut self, memory_offset: i32) -> Result<String> {
        // Creating a memory buffer containing the memory contents beginning from the specified
        // memory offset.
        let memory_buffer: &[u8] = &self.get_memory().data(&self.store)[memory_offset as usize..];

        // Getting the index of the first null offset.
        if let Some(null_index) = memory_buffer.iter().position(|x| *x == 0) {
            let string_buffer: &[u8] = &memory_buffer[..null_index];
            Ok(String::from(
                std::str::from_utf8(string_buffer).map_err(WrapperError::Utf8Error)?,
            ))
        } else {
            Err(WrapperError::NullTerminatorNotFound)
        }
    }

    /// Allocates memory for a string in the instance's linear memory
    ///
    /// This method takes a string and allocates enough memory for its C-String UTF-8 encoded
    /// representation in the instance's linear memory.
    ///
    /// # Arguments
    ///
    /// - `string: AsRef<str>`: A generic object which can be referenced as a `str`.
    ///
    /// # Returns
    ///
    /// - [`Result<i32>`]: An [i32] is returned if the memory allocation is successful, otherwise, a
    /// [WrapperError] is returned.
    fn allocate_memory_for_string<S: AsRef<str>>(&mut self, string: S) -> Result<i32> {
        // Converting the string to a C-String and getting the byte count of this string
        let string: &str = string.as_ref();
        let byte_count: usize = string.len();

        // Memory allocation by capacity can now be performed.
        self.allocate_memory_by_capacity(byte_count + 1)
    }

    /// Allocates memory in the instance's linear memory
    ///
    /// This method takes a string and allocates memory in the instance's linear memory based on the
    /// capacity required.
    ///
    /// # Arguments
    ///
    /// - `capacity: usize`: The byte count of the amount of bytes to allocate.
    ///
    /// # Returns
    ///
    /// - [Result<i32>]: An [i32] is returned if the memory allocation is successful, otherwise, a
    /// [WrapperError] is returned.
    fn allocate_memory_by_capacity(&mut self, capacity: usize) -> Result<i32> {
        self.function_store
            .toolkit_alloc
            .call(&mut self.store, capacity as i32)
            .map_err(WrapperError::WasmTimeTrapError)
    }

    /// Frees up memory in the WASM's linear memory.
    ///
    /// This method frees up memory in WASM's linear memory. This is with the assumption that the
    /// memory was allocated through the library's memory allocator
    fn free_memory(&mut self, memory_offset: i32) -> Result<()> {
        self.function_store
            .toolkit_free_c_string
            .call(&mut self.store, memory_offset)
            .map_err(WrapperError::WasmTimeTrapError)
    }

    /// Gets the memory of the current WASM instance.
    ///
    /// # Returns
    ///
    /// - [Memory]: A memory object of instance's linear memory.
    fn get_memory(&mut self) -> Memory {
        self.instance
            .get_memory(&mut self.store, "memory")
            .expect("Failed to get the memory of the WASM instance")
    }
}

// ===============
// Function Store
// ===============

crate::define_function_store! {
    pub struct RadixEngineToolkitFunctions {
        pub information: TypedFunc<i32, i32>,

        pub convert_manifest: TypedFunc<i32, i32>,

        pub compile_transaction_intent: TypedFunc<i32, i32>,
        pub compile_signed_transaction_intent: TypedFunc<i32, i32>,
        pub compile_notarized_transaction_intent: TypedFunc<i32, i32>,

        pub decompile_transaction_intent: TypedFunc<i32, i32>,
        pub decompile_signed_transaction_intent: TypedFunc<i32, i32>,
        pub decompile_notarized_transaction_intent: TypedFunc<i32, i32>,
        pub decompile_unknown_transaction_intent: TypedFunc<i32, i32>,

        pub sbor_encode: TypedFunc<i32, i32>,
        pub sbor_decode: TypedFunc<i32, i32>,

        pub encode_address: TypedFunc<i32, i32>,
        pub decode_address: TypedFunc<i32, i32>,

        pub derive_non_fungible_address_from_public_key: TypedFunc<i32, i32>,
        pub derive_non_fungible_address: TypedFunc<i32, i32>,
        
        pub derive_virtual_account_address: TypedFunc<i32, i32>,

        pub toolkit_alloc: TypedFunc<i32, i32>,
        pub toolkit_free_c_string: TypedFunc<i32, ()>
    }
}

// ======
// Error
// ======

/// An enum representing errors encountered by the [RadixEngineToolkit] wrapper.
#[derive(Debug)]
pub enum WrapperError {
    /// An error emitted when a file could not be found.
    FileNotFoundError(PathBuf),

    /// An error emitted when a file could not be read.
    FileReadingError(std::io::Error),

    /// An error emitted by the WasmTime runtime.
    WasmTimeError(anyhow::Error),

    /// An error emitted when a Radix Engine Toolkit operation fails
    RadixEngineToolkitError(Error),

    /// An error emitted when trying to access the linear memory of a WASM instance.
    MemoryAccessError(wasmtime::MemoryAccessError),

    /// An error emitted when no null terminator can be found.
    NullTerminatorNotFound,

    /// An error representing the standard library's [std::str::Utf8Error] type.
    Utf8Error(std::str::Utf8Error),

    /// An error representing the standard library's [std::ffi::NulError] type.
    NulError(std::ffi::NulError),

    /// An error representing the standard library's [wasmtime::Trap] type.
    WasmTimeTrapError(wasmtime::Trap),

    /// An error emitted when the deserialization of an object fails
    DeserializationError,

    /// An error emitted during runtime in response to a request
    LibraryError(Error),
}

impl From<std::ffi::NulError> for WrapperError {
    fn from(error: std::ffi::NulError) -> Self {
        Self::NulError(error)
    }
}

impl From<anyhow::Error> for WrapperError {
    fn from(error: anyhow::Error) -> Self {
        Self::WasmTimeError(error)
    }
}

// =======
// Macros
// =======

#[macro_export]
macro_rules! define_function_store{
    (
     $vis:vis struct $struct_name:ident {
        $(
        $field_vis:vis $field_name:ident : TypedFunc<$input_type: ty, $output_type: ty>
        ),*
    }
    ) => {
        $vis struct $struct_name{
            $(
                $field_vis $field_name : TypedFunc<$input_type, $output_type>,
            )*
        }

        impl $struct_name {
            pub fn new(instance: &Instance, store: &mut Store<i32>) -> Result<Self> {
                Ok(
                    Self {
                        $(
                            $field_name: instance.get_typed_func(store.as_context_mut(), stringify!($field_name))?,
                        )*
                    }
                )
            }
        }
    }
}

#[macro_export]
macro_rules! define_request_function {
    ($request_type: ty, $response_type: ty, $function_ident: ident) => {
        pub fn $function_ident(&mut self, request: $request_type) -> Result<$response_type> {
            self.call_wasm_function(self.function_store.$function_ident, request)
        }
    };
}

// ======
// Tests
// ======

#[cfg(test)]
mod tests {
    use radix_engine_toolkit::requests::{InformationRequest, InformationResponse};

    use super::{RadixEngineToolkit, Result};

    #[test]
    pub fn test_information_request_succeeds() {
        // Arrange
        let mut radix_engine_toolkit: RadixEngineToolkit =
            RadixEngineToolkit::new_compile_from_source()
                .expect("Failed to create a new library from source");

        // Act
        let response: Result<InformationResponse> =
            radix_engine_toolkit.information(InformationRequest {});

        // Assert
        assert!(matches!(response, Ok(_)))
    }
}
