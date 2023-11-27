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

macro_rules! export_function {
    ($function: ident as $name: ident) => {
        #[no_mangle]
        pub extern "C" fn $name(
            ptr: $crate::memory::Pointer,
        ) -> $crate::memory::Pointer {
            crate::functions::handler::handle_invocation::<$function>(ptr)
        }
    };
}

macro_rules! export_jni_function {
    ($function: ident as $name: ident) => {
        paste::paste! {
            #[no_mangle]
            pub extern "system" fn [< Java_com_radixdlt_toolkit_RadixEngineToolkitFFI_ $name >](
                env: jni::JNIEnv,
                class: jni::objects::JClass,
                input: jni::objects::JString,
            ) -> jni::sys::jstring {
                crate::functions::handler::handle_jni_invocation::<$function>(env, class, input)
            }
        }
    };
}

pub(crate) use {export_function, export_jni_function};
