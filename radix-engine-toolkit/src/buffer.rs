// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

//! This module contains a number of functions used for memory interactions. These functions provide
//! a way to allocate, deallocate, and read memory.

use std::alloc::{alloc, dealloc, Layout};

/// A type alias of the main pointer type that this module uses.
pub type Pointer = *mut std::ffi::c_char;

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
    alloc(layout) as Pointer
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
    dealloc(pointer as *mut _, layout);
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
    let length = std::ffi::CStr::from_ptr(pointer as *const std::ffi::c_char)
        .to_bytes()
        .len();
    toolkit_free(pointer, length);
}
