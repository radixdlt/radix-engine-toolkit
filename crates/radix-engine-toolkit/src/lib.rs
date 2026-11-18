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

pub mod constants;
pub mod extensions;
pub mod functions;
pub mod manifest_analysis;
pub mod types;

pub(crate) mod internal_prelude {
    // Modules from the standard library and core language
    pub use core::convert::*;
    pub use core::ops::*;
    pub use std::sync::*;

    // Modules from this crate.
    pub use crate::prelude::*;

    // Modules from the Radixdlt-Scrypto Crates.
    pub use radix_common::prelude::*;
    pub use radix_engine::blueprints::consensus_manager::*;
    pub use radix_engine::system::system_modules::execution_trace::{
        ResourceSpecifier, WorktopChange,
    };
    pub use radix_engine::transaction::*;
    pub use radix_engine::utils::*;
    pub use radix_engine_interface::blueprints::access_controller::*;
    pub use radix_engine_interface::blueprints::account::*;
    pub use radix_engine_interface::blueprints::consensus_manager::*;
    pub use radix_engine_interface::blueprints::locker::*;
    pub use radix_engine_interface::blueprints::pool::*;
    pub use radix_engine_interface::blueprints::resource::*;
    pub use radix_engine_interface::prelude::*;
    pub use radix_engine_toolkit_common::receipt::{
        MetadataUpdate, RuntimeToolkitTransactionReceipt, RuntimeTypeSelector,
    };
    pub use radix_substate_store_queries::typed_native_events::*;
    pub use radix_transactions::data::*;
    pub use radix_transactions::errors::*;
    pub use radix_transactions::manifest::static_resource_movements::*;
    pub use radix_transactions::manifest::*;
    pub use radix_transactions::prelude::*;
    pub use radix_transactions::validation::*;
    pub use sbor::representations::*;
    pub use sbor::traversal::*;

    // Modules from our own crates
    pub use sbor_json::scrypto::programmatic::utils::*;
    pub use sbor_json::scrypto::programmatic::value::*;

    // Modules from other external crates.
    pub use bech32::{FromBase32, ToBase32, Variant};
    pub use extend::*;
    pub use itertools::Itertools;
    pub use paste::*;
    pub use regex::*;
}

pub mod prelude {
    pub use crate::*;

    pub use crate::constants::*;
    pub use crate::extensions::*;
    pub use crate::functions;
    pub use crate::manifest_analysis::*;
    pub use crate::types::*;
}
