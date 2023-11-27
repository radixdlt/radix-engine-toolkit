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

use std::fmt::Debug;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::error::InvocationInterpretationError;
use crate::memory::Pointer;

pub fn debug_string<T: Debug>(object: T) -> String {
    format!("{:?}", object)
}

pub fn serialize_and_write_to_memory<S: Serialize>(
    object: &S,
) -> Result<Pointer, InvocationInterpretationError> {
    serde_json::to_string(object)
        .map_err(|error| {
            InvocationInterpretationError::SerializationError(debug_string(
                error,
            ))
        })
        .map(|string| {
            let object_bytes = string.as_bytes();
            let byte_count = object_bytes.len() + 1;

            unsafe {
                let pointer = crate::memory::toolkit_alloc(byte_count);
                pointer.copy_from(
                    [object_bytes, &[0]].concat().as_ptr() as Pointer,
                    byte_count,
                );

                pointer
            }
        })
}

pub fn read_and_deserialize_from_memory<'s, D: Deserialize<'s>>(
    string_pointer: Pointer,
) -> Result<D, InvocationInterpretationError> {
    unsafe {
        std::ffi::CStr::from_ptr(string_pointer as *const std::ffi::c_char)
    }
    .to_str()
    .map_err(
        |error| InvocationInterpretationError::Utf8Error(debug_string(error))
    )
    .and_then(|string| {
        serde_json::from_str(string).map_err(|error| {
            InvocationInterpretationError::DeserializationError(debug_string(
                error,
            ))
        })
    })
}

pub fn serialize_to_jstring<S: Serialize>(
    env: &jni::JNIEnv,
    object: &S,
) -> Result<jni::sys::jstring, InvocationInterpretationError> {
    serde_json::to_string(object)
        .map_err(|error| {
            InvocationInterpretationError::SerializationError(debug_string(
                error,
            ))
        })
        .and_then(|string| {
            env.new_string(string).map_err(|error| {
                InvocationInterpretationError::FailedToAllocateJniString(
                    debug_string(error),
                )
            })
        })
        .map(|string| string.into_raw())
}

pub fn deserialize_from_jstring<D: DeserializeOwned>(
    env: &mut jni::JNIEnv,
    jstring: &jni::objects::JString,
) -> Result<D, InvocationInterpretationError> {
    let java_str = env.get_string(jstring).map_err(|error| {
        InvocationInterpretationError::FailedToReadJniString(debug_string(
            error,
        ))
    })?;

    let string = String::from(java_str);
    let result = serde_json::from_str(&string).map_err(|error| {
        InvocationInterpretationError::DeserializationError(debug_string(error))
    })?;

    Ok(result)
}
