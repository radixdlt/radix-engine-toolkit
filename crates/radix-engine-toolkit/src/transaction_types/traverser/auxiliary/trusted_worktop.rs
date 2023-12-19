use std::default::Default;
use super::super::traits::*;
use radix_engine_interface::blueprints::account::ACCOUNT_WITHDRAW_IDENT;
use scrypto::prelude::*;
use transaction::prelude::*;


pub struct TrustedWorktop {
    trusted: bool
}

impl TrustedWorktop {
    
}

impl Default for TrustedWorktop {
    fn default() -> Self { 
        TrustedWorktop {
            trusted: true
        }
    }
}

impl ManifestSummaryCallback for TrustedWorktop {

    fn on_instruction(
        &mut self,
        instruction: &InstructionV1,
        _instruction_index: usize,
    ) {
        if !self.trusted { 
            return
        }
        match instruction {
            InstructionV1::CallMethod { method_name, address, .. } => {
                if address.is_static_global_package() {
                    // trusted methods
                    self.trusted = method_name == ACCOUNT_WITHDRAW_IDENT; 
                } else {
                    self.trusted = false;
                }
            },
            _ => self.trusted = false
        }
    }

}