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

#![feature(get_mut_unchecked)]

pub mod build;
pub mod builder;
pub mod common;
pub mod cryptography;
pub mod derive;
pub mod error;
pub mod events;
pub mod manifest;
pub mod sbor;
pub mod traits;
pub mod transaction;
pub mod utils;

pub(crate) mod internal_prelude;

// Everything is imported at the root of the crate. This is to support some of the needs of the
// UniFFI toolkit.
pub mod prelude {
    /* Common */
    pub use crate::common::address::*;
    pub use crate::common::decimal::*;
    pub use crate::common::entity_type::*;
    pub use crate::common::metadata::*;
    pub use crate::common::module_id::*;
    pub use crate::common::non_fungible::*;
    pub use crate::common::olympia::*;

    /* Traits */
    pub use crate::traits::*;

    /* Builders */
    pub use crate::builder::transaction_builder::*;

    /* Errors */
    pub use crate::error::*;

    /* Events */
    pub use crate::events::*;

    /* Cryptography */
    pub use crate::cryptography::curve::*;
    pub use crate::cryptography::hash::*;
    pub use crate::cryptography::private_key::*;
    pub use crate::cryptography::public_key::*;
    pub use crate::cryptography::public_key_hash::*;
    pub use crate::cryptography::signature::*;
    pub use crate::cryptography::signature_with_public_key::*;

    /* Manifest */
    pub use crate::manifest::address::*;
    pub use crate::manifest::address_reservation::*;
    pub use crate::manifest::blob::*;
    pub use crate::manifest::bucket::*;
    pub use crate::manifest::expression::*;
    pub use crate::manifest::proof::*;
    pub use crate::manifest::value::*;

    /* Transaction */
    pub use crate::transaction::hash::*;
    pub use crate::transaction::header::*;
    pub use crate::transaction::instruction::*;
    pub use crate::transaction::instructions::*;
    pub use crate::transaction::intent::*;
    pub use crate::transaction::manifest::*;
    pub use crate::transaction::message::*;
    pub use crate::transaction::notarized_transaction::*;
    pub use crate::transaction::signed_intent::*;
    pub use crate::transaction::validation::*;

    /* SBOR */
    pub use crate::sbor::functions::*;

    /* Build */
    pub use crate::build::functions::*;

    /* Utils */
    pub use crate::utils::functions::*;

    /* Derive */
    pub use crate::derive::functions::*;

    /* Internal Prelude */
    pub(crate) use crate::internal_prelude::*;

    /* Often needed */
    pub(crate) use std::collections::{BTreeMap, HashMap};
    pub(crate) use std::sync::Arc;
    pub(crate) use thiserror::Error as ThisError;
    pub(crate) use uniffi::{Enum, Error, Object, Record};
}
pub use prelude::*;

uniffi::include_scaffolding!("radix_engine_toolkit_uniffi");
