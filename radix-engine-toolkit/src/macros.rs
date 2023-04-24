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

#[macro_export]
macro_rules! impl_from_parse_error {
    ($container_error: ty, $error_type: ty => $kind: ident) => {
        impl From<$error_type> for $container_error {
            fn from(error: $error_type) -> Self {
                Self::ParseError {
                    parsing: stringify!($kind).to_owned(),
                    message: format!("{:?}", error),
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_display_as_debug {
    ($type: ty) => {
        impl std::fmt::Display for $type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::Debug::fmt(self, f)
            }
        }
    };
}
