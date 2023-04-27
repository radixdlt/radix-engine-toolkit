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
macro_rules! define_kind_enum {
    (
        $(#[$enum_metadata:meta])*
        $vis:vis enum $enum_ident:ident {
            $(
                $(#[$variant_metadata:meta])*
                $variant_ident:ident $(
                    {
                        $(
                            $(#[$field_metadata:meta])*
                            $field_ident:ident : $field_type:ty
                        ),* $(,)?
                    }
                )?
            ),* $(,)?
        }
    ) =>
    {
        paste::item! {
            $(#[$enum_metadata])*
            $vis enum $enum_ident {
                $(
                    $(#[$variant_metadata])*
                    $variant_ident $({
                        $(
                            $(#[$field_metadata])*
                            $field_ident: $field_type
                        ),*
                    })?
                ),*
            }

            #[toolkit_derive::serializable]
            #[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Copy)]
            $vis enum [< $enum_ident Kind >] {
                $(
                    $(#[$variant_metadata])*
                    $variant_ident
                ),*
            }

            impl $enum_ident {
                pub fn kind(&self) -> [< $enum_ident Kind >] {
                    match self {
                        $(
                            Self::$variant_ident {..} => [< $enum_ident Kind >]::$variant_ident
                        ),*
                    }
                }
            }
        }
    };
}
