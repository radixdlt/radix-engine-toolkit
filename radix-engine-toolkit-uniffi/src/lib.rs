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

pub mod build;
pub mod common;
pub mod cryptography;
pub mod derive;
pub mod error;
pub mod utils;

pub(crate) mod internal_prelude;

// Everything is imported at the root of the crate. This is to support some of the needs of the
// UniFFI toolkit.
pub mod prelude {
    /* Common */
    pub use crate::common::address::*;
    pub use crate::common::entity_type::*;
    pub use crate::common::non_fungible::*;
    pub use crate::common::olympia::*;

    /* Errors */
    pub use crate::error::*;

    /* Cryptography */
    pub use crate::cryptography::public_key::*;
    pub use crate::cryptography::public_key_hash::*;
    pub use crate::cryptography::signature::*;
    pub use crate::cryptography::signature_with_public_key::*;

    /* Build */
    pub use crate::build::functions::*;

    /* Utils */
    pub use crate::utils::functions::*;

    /* Derive */
    pub use crate::derive::functions::*;

    /* Internal Prelude */
    pub(crate) use crate::internal_prelude::*;

    /* Often needed */
    pub(crate) use std::sync::Arc;
    pub(crate) use thiserror::Error as ThisError;
    pub(crate) use uniffi::{Enum, Error, Object, Record};
}
pub use prelude::*;

uniffi::include_scaffolding!("radix_engine_toolkit_uniffi");
