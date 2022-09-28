//! This module defines the struct and implementation for a TransactionLibrary wrapper that uses the
//! WasmTime runtime. This struct is mainly defined for the purpose of testing out the behavior of
//! the transaction library when it is running through a WASM host.

use wasmtime::{Engine, Instance, Linker, Module, Store};
use std::path::{Path, PathBuf};

/// A shortcut type that defines a `Result` that returns the generic `T` or a `WrapperError`.
type Result<T> = std::result::Result<T, WrapperError>;

struct TransactionLibrary {
    engine: Engine,
    module: Module,
    linker: Linker<i32>,
    store: Store<i32>,
    instance: Instance,
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

        let transaction_library: Self = Self {
            engine,
            module,
            linker,
            store,
            instance,
        };
        Ok(transaction_library)
    }

    // pub fn new_compile_from_source() -> Result<Self> {


    //     // Build the transaction library from source
    //     let status = std::process::Command::new("cargo")
    //         .current_dir(package_dir.as_ref())
    //         .args(["build", "--target", "wasm32-unknown-unknown", "--release"])
    //         .status()
    //         .unwrap();
    //     if !status.success() {
    //         panic!("Failed to compile package: {:?}", package_dir.as_ref());
    //     }
    // }
}

/// An enum representing errors encountered by the [TransactionLibrary] wrapper.
#[derive(Debug)]
enum WrapperError {
    /// An error emitted when a file could not be found.
    FileNotFoundError(PathBuf),

    /// An error emitted when a file could not be read.
    FileReadingError(std::io::Error),

    /// An error emitted by the WasmTime runtime.
    WasmTimeError(anyhow::Error),
}