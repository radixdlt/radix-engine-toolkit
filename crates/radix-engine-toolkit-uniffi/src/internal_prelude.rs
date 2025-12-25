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

//! This internal prelude module exposes two main public modules that can be
//! imported by other modules in this crate: an [`engine`] and a [`toolkit`]
//! modules. These two modules are meant to be preludes for the engine stack
//! and toolkit respectively. This is meant to replace the old system we had
//! where we would import everything by name and then add a prefix to it, which
//! quickly turned out to be very hard to manage. Instead, once this module is
//! imported typed such as [`engine::NetworkDefinition`] become available.

#[allow(ambiguous_glob_reexports)]
pub mod engine {
    pub use radix_common::prelude::*;
    pub use radix_engine::blueprints::access_controller::latest::*;
    pub use radix_engine::blueprints::account::*;
    pub use radix_engine::blueprints::consensus_manager::*;
    pub use radix_engine::blueprints::locker::*;
    pub use radix_engine::blueprints::package::*;
    pub use radix_engine::blueprints::pool::v1::*;
    pub use radix_engine::blueprints::resource::*;
    pub use radix_engine::object_modules::metadata::*;
    pub use radix_engine::object_modules::role_assignment::*;
    pub use radix_engine::object_modules::royalty::*;
    pub use radix_engine::system::system_modules::execution_trace::{
        ResourceSpecifier, WorktopChange,
    };
    pub use radix_engine::transaction::*;
    pub use radix_engine::utils::*;
    pub use radix_engine_interface::blueprints::access_controller::*;
    pub use radix_engine_interface::blueprints::account::*;
    pub use radix_engine_interface::blueprints::component::*;
    pub use radix_engine_interface::blueprints::consensus_manager::*;
    pub use radix_engine_interface::blueprints::identity::*;
    pub use radix_engine_interface::blueprints::locker::*;
    pub use radix_engine_interface::blueprints::package::*;
    pub use radix_engine_interface::blueprints::pool::*;
    pub use radix_engine_interface::blueprints::resource::*;
    pub use radix_engine_interface::prelude::*;
    pub use radix_engine_toolkit_common::receipt::{
        MetadataUpdate, RuntimeToolkitTransactionReceipt, RuntimeTypeSelector,
    };
    pub use radix_substate_store_queries::typed_native_events::*;
    pub use radix_transactions::data::*;
    pub use radix_transactions::data::{
        from_decimal, from_non_fungible_local_id, from_precise_decimal,
        to_decimal, to_non_fungible_local_id, to_precise_decimal,
    };
    pub use radix_transactions::errors::*;
    pub use radix_transactions::manifest::static_resource_movements::*;
    pub use radix_transactions::manifest::*;
    pub use radix_transactions::prelude::*;
    pub use radix_transactions::validation::*;
    pub use sbor::representations::*;
    pub use sbor::traversal::*;
}

pub mod toolkit {
    pub use radix_engine_toolkit::prelude::*;
}

// We import a number of traits so that their methods are available to us in the
// toolkit without needing to use the fully qualified syntax. We import them as
// underscore (_) so that we don't get any name conflicts in the toolkit when
// using the methods that they define.
pub use engine::{
    FromPublicKey as _, HasNotarizedTransactionHash as _,
    HasSignedTransactionIntentHash as _, HasSubintentHash as _,
    HasTransactionIntentHash as _, Signer as _, TransactionPayload as _,
};
pub use toolkit::extensions::*;

// We must import the require functions by name since the `rule` macro expects
// a function ident and not an expression.
pub use engine::{
    require, require_all_of, require_amount, require_any_of, require_n_of,
};
