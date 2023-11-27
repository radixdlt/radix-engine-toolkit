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

use super::traits::Function;
use crate::memory::Pointer;
use crate::utils::{serialize_and_write_to_memory, serialize_to_jstring};

pub fn handle_invocation<'f, F>(input: Pointer) -> Pointer
where
    F: Function<'f>,
{
    let result =
        crate::utils::read_and_deserialize_from_memory::<F::Input>(input)
            .map_err(crate::error::Error::from)
            .and_then(|input| {
                F::handle(input).map_err(crate::error::Error::from)
            })
            .and_then(|output| {
                serialize_and_write_to_memory::<F::Output>(&output)
                    .map_err(crate::error::Error::from)
            })
            .map_err(|error| {
                serialize_and_write_to_memory::<crate::error::Error>(&error)
                    .expect("can't fail")
            });

    match result {
        Ok(ptr) => ptr,
        Err(ptr) => ptr,
    }
}

pub fn handle_jni_invocation<'f, F>(
    mut env: jni::JNIEnv,
    _: jni::objects::JClass,
    input: jni::objects::JString,
) -> jni::sys::jstring
where
    F: Function<'f>,
{
    let result =
        crate::utils::deserialize_from_jstring::<F::Input>(&mut env, &input)
            .map_err(crate::error::Error::from)
            .and_then(|input| {
                F::handle(input).map_err(crate::error::Error::from)
            })
            .and_then(|output| {
                serialize_to_jstring::<F::Output>(&env, &output)
                    .map_err(crate::error::Error::from)
            })
            .map_err(|error| {
                serialize_to_jstring::<crate::error::Error>(&env, &error)
                    .expect("can't fail")
            });

    match result {
        Ok(ptr) => ptr,
        Err(ptr) => ptr,
    }
}
