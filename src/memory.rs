//! This module contains a number of functions used for memory interactions. These functions provide
//! a way to allocate, deallocate, and read memory.

use crate::error::Error;
use serde::{Deserialize, Serialize};
use std::alloc::{alloc, dealloc, Layout};

/// A type alias of the main pointer type that this module uses.
pub type Pointer = *mut u8;

/// Allocates memory of the specified capacity and returns a pointer to that memory location.
///
/// This function allocates memory based on the passed capacity and returns a pointer to that
/// memory location. This function does not make any assumptions on the data that will be stored
/// at that memory location.
///
/// # Safety
///
/// * This function makes use of pointers which is an unsafe feature.
/// * Memory allocated through this function should be deallocated through [`toolkit_free`] or any
/// function that calls [`toolkit_free`].
///
/// # Arguments
///
/// * `capacity`: [`usize`] - The capacity (in bytes) to allocate in memory
///
/// # Returns
///
/// * [`Pointer`]: A pointer to the allocated memory location.
#[no_mangle]
pub unsafe extern "C" fn toolkit_alloc(capacity: usize) -> Pointer {
    let align = std::mem::align_of::<usize>();
    let layout = Layout::from_size_align_unchecked(capacity, align);
    alloc(layout)
}

/// Fees up memory of a specific `capacity` beginning from the specified `pointer` location.
///
/// # Safety
///
/// * This function makes use of pointers which is an unsafe feature.
/// * This function assumes that the memory was allocated through the [toolkit_alloc] function.
///
/// # Arguments
///
/// * `pointer`: [`Pointer`] - A pointer to the allocated memory location
/// * `capacity`: [`usize`] - The amount of memory to deallocate
#[no_mangle]
pub unsafe extern "C" fn toolkit_free(pointer: Pointer, capacity: usize) {
    let align = std::mem::align_of::<usize>();
    let layout = Layout::from_size_align_unchecked(capacity, align);
    dealloc(pointer, layout);
}

/// Fees up memory allocated for a c-string at `pointer` location.
///
/// # Assumptions
///
/// * This function assumes that the memory location contains a null-terminated C-String which has
/// been allocated and written to memory through functions provides in the [`crate::memory`] module.
///
/// # Safety
///
/// * This function makes use of pointers which is an unsafe feature.
/// * This function assumes that the memory was allocated through the [`toolkit_alloc`] function.
///
/// # Arguments
///
/// * `pointer`: [`Pointer`] - A pointer to the allocated memory location
#[no_mangle]
pub unsafe extern "C" fn toolkit_free_c_string(pointer: Pointer) {
    // Loading the C-String from memory to get the byte-count of the string.
    let length: usize = std::ffi::CStr::from_ptr(pointer as *const i8)
        .to_bytes()
        .len();
    toolkit_free(pointer, length);
}

/// Reads and deserializes a JSON string from memory.
///
/// This function reads a JSON-encoded null-terminated UTF-8 encoded c-string from memory and
/// attempts to deserialize it from JSON as the generic type `T`.
///
/// There are two cases where this function could return a [`Result::Err`]:
///
/// 1. If no null-terminated c-string could be read from the provided memory location.
/// 2. If the string could not be deserialized to the generic type `T`.
///
/// # Safety
///
/// * This function makes use of pointers which is an unsafe feature.
///
/// # Note
///
/// This function does **NOT** take ownership of the data at the provided memory location. The
/// allocated memory still requires deallocation through one of the deallocation function defined in
/// the [`crate::memory`] module.
///
/// # Arguments
///
/// * `string_pointer`: [`Pointer`] - A pointer to the allocated memory location
///
/// # Returns
///
/// * [`Result<T, Error>`]: An object of type `T` is returned if the reading of the memory and the
/// deserialization both succeed. Otherwise, an [`Error`] is returned.
pub unsafe fn toolkit_read_and_deserialize_string_from_memory<'t, T>(
    string_pointer: Pointer,
) -> Result<T, Error>
where
    T: Deserialize<'t>,
{
    let string: &str = std::ffi::CStr::from_ptr(string_pointer as *const i8).to_str()?;
    Ok(serde_json::from_str(string)?)
}

/// Serializes and writes the serialized string to memory
///
/// This function takes an object that can be serialized using serde--an object that implements the
/// [`Serialize`] trait--serializes it to a JSON string, allocates the required memory to write this
/// string to memory as a null-terminated C-String, and writes the string to memory. This function
/// then returns a pointer to the memory location that the string is stored at.
///
/// # Safety
///
/// * This function makes use of pointers which is an unsafe feature.
///
/// # Arguments
///
/// * `object`: `T` - A generic object that implements the [`Serialize`] trait.
///
/// # Arguments
///
/// * [`Pointer`] - A pointer to the allocated memory location
pub unsafe fn toolkit_serialize_and_write_to_memory<T>(object: &T) -> Result<Pointer, Error>
where
    T: Serialize,
{
    let object_string: String = serde_json::to_string(object)?;
    let object_bytes: &[u8] = object_string.as_bytes();
    let byte_count: usize = object_bytes.len() + 1;

    let pointer: Pointer = toolkit_alloc(byte_count);
    pointer.copy_from([object_bytes, &[0]].concat().as_ptr(), byte_count);
    Ok(pointer)
}
