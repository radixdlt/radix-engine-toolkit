use crate::sbor::indexed_manifest_value::IndexedManifestValue;
use crate::utils::*;
use radix_engine::system::system_modules::execution_trace::ResourceSpecifier;
use radix_engine_interface::api::node_modules::royalty::*;
use radix_engine_interface::blueprints::{
    access_controller::*, account::*, consensus_manager::*, identity::*,
    package::*, pool::*,
};
use scrypto::prelude::*;
use transaction::prelude::*;

use super::TrustedWorktop;

impl TrustedWorktop {
    pub fn handle_call_functions(
        &mut self,
        package_address: &DynamicPackageAddress,
        blueprint_name: &str,
        function_name: &str,
        args: &ManifestValue,
    ) {
        // todo
    }
}
