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

#![allow(clippy::missing_safety_doc)]

use std::alloc::{alloc, dealloc, Layout};

pub type Pointer = *mut std::ffi::c_char;

#[no_mangle]
pub unsafe extern "C" fn toolkit_alloc(capacity: usize) -> Pointer {
    let align = std::mem::align_of::<usize>();
    let layout = Layout::from_size_align_unchecked(capacity, align);
    alloc(layout) as Pointer
}

#[no_mangle]
pub unsafe extern "C" fn toolkit_free(pointer: Pointer, capacity: usize) {
    let align = std::mem::align_of::<usize>();
    let layout = Layout::from_size_align_unchecked(capacity, align);
    dealloc(pointer as *mut _, layout);
}

#[no_mangle]
pub unsafe extern "C" fn toolkit_free_c_string(pointer: Pointer) {
    // Loading the C-String from memory to get the byte-count of the string.
    let length = std::ffi::CStr::from_ptr(pointer as *const std::ffi::c_char)
        .to_bytes()
        .len();
    toolkit_free(pointer, length);
}
