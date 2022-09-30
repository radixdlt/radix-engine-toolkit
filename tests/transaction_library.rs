//! This module defines the struct and implementation for a TransactionLibrary wrapper that uses the
//! WasmTime runtime. This struct is mainly defined for the purpose of testing out the behavior of
//! the transaction library when it is running through a WASM host.

use serde::{Deserialize, Serialize};
use std::ffi::CString;
use std::path::{Path, PathBuf};
use transaction_library::requests::*;
use wasmtime::{AsContextMut, Engine, Instance, Linker, Memory, Module, Store, TypedFunc};

// ==========
// Type Defs
// ==========

/// A shortcut type that defines a `Result` that returns the generic `T` or a `WrapperError`.
type Result<T> = std::result::Result<T, WrapperError>;

// ================
// Library Wrapper
// ================

struct TransactionLibrary {
    engine: Engine,
    module: Module,
    linker: Linker<i32>,
    store: Store<i32>,
    instance: Instance,
    function_store: TransactionLibraryFunctions,
}

impl TransactionLibrary {
    /// Creates a new [TransactionLibrary] object from a given path.
    ///
    /// This function is able to instantiate a new [TransactionLibrary] given the path to the
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
    /// - `Result<Self, WrapperError>`: A new object of [TransactionLibrary] is returned, or a
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

    /// Creates a new [TransactionLibrary] object from a module byte array.
    ///
    /// This function is able to instantiate a new [TransactionLibrary] given the contents of the
    /// WASM file.
    ///
    /// # Arguments
    ///
    /// - `bytes: AsRef<[u8]>`: A generic object which can be referenced as a `[u8]`.
    ///
    /// # Returns
    ///
    /// - `Result<Self, WrapperError>`: A new object of [TransactionLibrary] is returned, or a
    /// [WrapperError] is returned in the case of an error.
    pub fn new_from_module_bytes<T: AsRef<[u8]>>(bytes: T) -> Result<Self> {
        // Getting the `&[u8]` from the generic object.
        let bytes: &[u8] = bytes.as_ref();

        // Creating the required WASM host objects to run the transaction library WASM.
        let engine: Engine = Engine::default();
        let module: Module = Module::new(&engine, bytes).map_err(WrapperError::WasmTimeError)?;
        let linker: Linker<i32> = Linker::new(&engine);
        let mut store: Store<i32> = Store::new(&engine, 4);
        let instance: Instance = linker
            .instantiate(&mut store, &module)
            .map_err(WrapperError::WasmTimeError)?;
        let function_store: TransactionLibraryFunctions =
            TransactionLibraryFunctions::new(&instance, &mut store)?;

        let transaction_library: Self = Self {
            engine,
            module,
            linker,
            store,
            instance,
            function_store,
        };
        Ok(transaction_library)
    }

    /// Creates a new [TransactionLibrary] object from source code.
    ///
    /// This function compiles the [TransactionLibrary] as to a `wasm32-unknown-unknown` target and
    /// then uses the `new_from_module_path` constructor to create a new [TransactionLibrary] object
    ///
    /// # Returns
    ///
    /// - `Result<Self, WrapperError>`: A new object of [TransactionLibrary] is returned, or a
    pub fn new_compile_from_source() -> Result<Self> {
        // The path to the directory containing the Cargo.toml manifest file
        let manifest_directory: PathBuf =
            std::env::current_dir().expect("Failed to get the path of the current directory");

        // Build the transaction library from source
        let status: std::process::ExitStatus = std::process::Command::new("cargo")
            .current_dir(&manifest_directory)
            .args(["build", "--target", "wasm32-unknown-unknown", "--release"])
            .status()
            .unwrap();
        if !status.success() {
            panic!("Failed to compile package: {:?}", &manifest_directory);
        };

        // Building a path to the WASM file
        let wasm_module_path: PathBuf = manifest_directory
            .join("target/wasm32-unknown-unknown/release/transaction_library.wasm");
        Self::new_from_module_path(wasm_module_path)
    }

    /// Serializes an object to a JSON string
    ///
    /// # Arguments
    ///
    /// - `object: Serialize`: A generic object of any type that implements the [Serialize] trait.
    ///
    /// # Returns
    ///
    /// - `String`: A JSON string of the serialized object
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
    fn deserialize<'d, D: Deserialize<'d>, S: AsRef<str>>(string: &'d S) -> Result<D> {
        let str: &'d str = string.as_ref();
        serde_json::from_str(str).map_err(|error| {
            WrapperError::TransactionLibraryError(transaction_library::error::Error::from(error))
        })
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
        let string: CString =
            CString::new(string).expect("Failed to create CString from a trusted string");

        self.get_memory()
            .write(&mut self.store, memory_offset as usize, string.to_bytes())
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
    /// - `Result<i32>`: An [i32] is returned if the memory allocation is successful, otherwise, a
    /// [WrapperError] is returned.
    fn allocate_memory_for_string<S: AsRef<str>>(&mut self, string: S) -> Result<i32> {
        // Converting the string to a C-String and getting the byte count of this string
        let string: &str = string.as_ref();
        let string: CString = CString::new(string).map_err(WrapperError::NulError)?;
        let byte_count: usize = string.as_bytes().len();

        // Memory allocation by capacity can now be performed.
        self.allocate_memory_by_capacity(byte_count)
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
    /// - `Result<i32>`: An [i32] is returned if the memory allocation is successful, otherwise, a
    /// [WrapperError] is returned.
    fn allocate_memory_by_capacity(&mut self, capacity: usize) -> Result<i32> {
        self.function_store
            .__transaction_lib_alloc
            .call(&mut self.store, capacity as i32)
            .map_err(WrapperError::WasmTimeTrapError)
    }

    /// Gets the memory of the current WASM instance.
    ///
    /// # Returns
    ///
    /// - `Memory`: A memory object of instance's linear memory.
    fn get_memory(&mut self) -> Memory {
        self.instance
            .get_memory(&mut self.store, "memory")
            .expect("Failed to get the memory of the WASM instance")
    }
}

// ===============
// Function Store
// ===============

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
                            $field_name: instance.get_typed_func(store.as_context_mut(), "info")?,
                        )*
                    }
                )
            }
        }
    }
}

define_function_store! {
    struct TransactionLibraryFunctions {
        pub information: TypedFunc<i32, i32>,

        pub convert_manifest: TypedFunc<i32, i32>,

        pub compile_transaction_intent: TypedFunc<i32, i32>,
        pub compile_signed_transaction_intent: TypedFunc<i32, i32>,
        pub compile_notarized_transaction_intent: TypedFunc<i32, i32>,

        pub decompile_transaction_intent: TypedFunc<i32, i32>,
        pub decompile_signed_transaction_intent: TypedFunc<i32, i32>,
        pub decompile_notarized_transaction_intent: TypedFunc<i32, i32>,

        pub sbor_encode: TypedFunc<i32, i32>,
        pub sbor_decode: TypedFunc<i32, i32>,

        pub encode_address: TypedFunc<i32, i32>,
        pub decode_address: TypedFunc<i32, i32>,

        pub extract_abi: TypedFunc<i32, i32>,

        pub __transaction_lib_alloc: TypedFunc<i32, i32>,
        pub __transaction_lib_free: TypedFunc<i32, ()>
    }
}

// ======
// Error
// ======

/// An enum representing errors encountered by the [TransactionLibrary] wrapper.
#[derive(Debug)]
enum WrapperError {
    /// An error emitted when a file could not be found.
    FileNotFoundError(PathBuf),

    /// An error emitted when a file could not be read.
    FileReadingError(std::io::Error),

    /// An error emitted by the WasmTime runtime.
    WasmTimeError(anyhow::Error),

    /// An error emitted when a transaction library operation fails
    TransactionLibraryError(transaction_library::error::Error),

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
