use crate::sbor::indexed_manifest_value::IndexedManifestValue;
use crate::utils::*;
use radix_engine_interface::blueprints::{
    access_controller::*, account::*, consensus_manager::*, identity::*,
    pool::*,
};
use scrypto::prelude::*;
use transaction::prelude::*;

use super::TrustedWorktop;

impl TrustedWorktop {
    fn unknown_function_call(&mut self) {
        self.untrack_buckets = true;
        self.untrack_worktop_content = true;
        self.add_new_instruction(false, None);
    }

    pub fn handle_call_functions(
        &mut self,
        address: &DynamicPackageAddress,
        _blueprint_name: &str,
        function_name: &str,
        args: &ManifestValue,
    ) {
        if is_account(address) {
            match function_name {
                ACCOUNT_CREATE_ADVANCED_IDENT => {
                    self.add_new_instruction(true, None)
                }
                ACCOUNT_CREATE_IDENT => {
                    // resturns bucket with newly generated address
                    self.new_bucket_unknown_resources();
                    self.add_new_instruction(false, None);
                }
                _ => self.unknown_function_call(),
            }
        } else if is_validator(address) {
            match function_name {
                CONSENSUS_MANAGER_CREATE_IDENT => {
                    self.add_new_instruction(true, None)
                }
                _ => self.unknown_function_call(),
            }
        } else if is_identity(address) {
            match function_name {
                IDENTITY_CREATE_ADVANCED_IDENT => {
                    self.add_new_instruction(true, None)
                }
                IDENTITY_CREATE_IDENT => {
                    // resturns unknown bucket
                    self.new_bucket_unknown_resources();
                    self.add_new_instruction(false, None)
                }
                _ => self.unknown_function_call(),
            }
        } else if is_access_controller(address) {
            match function_name {
                ACCESS_CONTROLLER_CREATE_IDENT => {
                    if !self.untrack_buckets {
                        // invalidate input bucket
                        let input_args = IndexedManifestValue::from_typed(args);
                        assert_eq!(input_args.buckets().len(), 1);
                        let bucket_id = input_args
                            .buckets()
                            .first()
                            .expect("Expected bucket");
                        let resources = self
                            .bucket_consumed(bucket_id)
                            .expect("Bucket not found");
                        self.add_new_instruction(
                            resources.is_some(),
                            resources,
                        );
                    } else {
                        self.add_new_instruction(false, None);
                    }
                }
                _ => self.unknown_function_call(),
            }
        } else {
            // todo: check for global comonents
            self.unknown_function_call();
        }
    }
}
