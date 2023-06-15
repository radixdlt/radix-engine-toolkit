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

macro_rules! serializable_string_wrapper {
    ($type: ty) => {
        paste::paste! {
            serializable_string_wrapper!{
                $type, [< Serializable $type:camel >]
            }
        }
    };
    ($type: ty, $name: ident) => {
        #[serde_with::serde_as]
        #[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema, Clone, Debug)]
        #[schemars(transparent)]
        #[serde(transparent)]
        pub struct $name(
            #[schemars(with = "String")]
            #[serde_as(as = "serde_with::DisplayFromStr")]
            pub $type,
        );

        impl std::ops::Deref for $name {
            type Target = $type;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl From<$type> for $name {
            fn from(value: $type) -> Self {
                Self(value)
            }
        }

        impl From<$name> for $type {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl ToString for $name {
            fn to_string(&self) -> String {
                self.0.to_string()
            }
        }
    };
}

macro_rules! define_enum_and_kind {
    (
        $(#[$meta: meta])*
        $vis: vis enum $name: ident {
            $(
                $variant_name: ident $({
                    $(
                        $field_name: ident: $field_type: ty
                    ),* $(,)?
                })?
            ),*

            $(,)?
        }
    ) => {
        paste::paste! {
            #[derive(Clone, Debug, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
            #[serde(tag = "kind")]
            $vis enum $name {
                $(
                    $variant_name $({
                        $(
                            $field_name: $field_type
                        ),*
                    })?,
                )*
            }

            #[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
            $vis enum [< $name Kind >] {
                $(
                    $variant_name,
                )*
            }
        }
    };
}

pub(crate) use define_enum_and_kind;
pub(crate) use serializable_string_wrapper;
