use std::collections::BTreeSet;

use scrypto::prelude::ComponentAddress;
use scrypto::radix_engine_interface::blueprints::account::*;

use crate::error::Result;
use crate::model::address::{EntityAddress, NetworkAwareComponentAddress};
use crate::model::value::ast::ManifestAstValue;
use crate::visitor::InstructionVisitor;

/// A visitor whose main responsibility is determining the kind of interactions involved with
/// accounts
#[derive(Debug, Default)]
pub struct AccountInteractionsInstructionVisitor {
    pub auth_required: BTreeSet<NetworkAwareComponentAddress>,
    pub accounts_withdrawn_from: BTreeSet<NetworkAwareComponentAddress>,
    pub accounts_deposited_into: BTreeSet<NetworkAwareComponentAddress>,
}

impl AccountInteractionsInstructionVisitor {
    const AUTH_REQUIRING_METHODS: &'static [&'static str] = &[
        ACCOUNT_LOCK_FEE_IDENT,
        ACCOUNT_LOCK_CONTINGENT_FEE_IDENT,
        ACCOUNT_WITHDRAW_IDENT,
        ACCOUNT_WITHDRAW_NON_FUNGIBLES_IDENT,
        ACCOUNT_LOCK_FEE_AND_WITHDRAW_IDENT,
        ACCOUNT_LOCK_FEE_AND_WITHDRAW_NON_FUNGIBLES_IDENT,
        ACCOUNT_CREATE_PROOF_IDENT,
        ACCOUNT_CREATE_PROOF_BY_AMOUNT_IDENT,
        ACCOUNT_CREATE_PROOF_BY_IDS_IDENT,
    ];
    const WITHDRAW_METHODS: &'static [&'static str] = &[
        ACCOUNT_WITHDRAW_IDENT,
        ACCOUNT_WITHDRAW_NON_FUNGIBLES_IDENT,
        ACCOUNT_LOCK_FEE_AND_WITHDRAW_IDENT,
        ACCOUNT_LOCK_FEE_AND_WITHDRAW_NON_FUNGIBLES_IDENT,
    ];
    const DEPOSIT_METHODS: &'static [&'static str] =
        &[ACCOUNT_DEPOSIT_IDENT, ACCOUNT_DEPOSIT_BATCH_IDENT];
}

impl InstructionVisitor for AccountInteractionsInstructionVisitor {
    fn visit_call_method(
        &mut self,
        component_address: &mut ManifestAstValue,
        method_name: &mut ManifestAstValue,
        _: &mut Option<Vec<ManifestAstValue>>,
    ) -> Result<()> {
        match (component_address, method_name) {
            (
                ManifestAstValue::ComponentAddress {
                    address: component_address,
                }
                | ManifestAstValue::Address {
                    address:
                        EntityAddress::ComponentAddress {
                            address: component_address,
                        },
                },
                ManifestAstValue::String { value: method_name },
            ) if matches!(
                component_address.address,
                ComponentAddress::Account(..)
                    | ComponentAddress::EcdsaSecp256k1VirtualAccount(..)
                    | ComponentAddress::EddsaEd25519VirtualAccount(..),
            ) =>
            {
                if Self::AUTH_REQUIRING_METHODS.contains(&method_name.as_str()) {
                    self.auth_required.insert(*component_address);
                }
                if Self::WITHDRAW_METHODS.contains(&method_name.as_str()) {
                    self.accounts_withdrawn_from.insert(*component_address);
                }
                if Self::DEPOSIT_METHODS.contains(&method_name.as_str()) {
                    self.accounts_deposited_into.insert(*component_address);
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn visit_set_metadata(
        &mut self,
        entity_address: &mut ManifestAstValue,
        _: &mut ManifestAstValue,
        _: &mut ManifestAstValue,
    ) -> Result<()> {
        match entity_address {
            ManifestAstValue::ComponentAddress {
                address: component_address,
            }
            | ManifestAstValue::Address {
                address:
                    EntityAddress::ComponentAddress {
                        address: component_address,
                    },
            } if matches!(
                component_address.address,
                ComponentAddress::Account(..)
                    | ComponentAddress::EcdsaSecp256k1VirtualAccount(..)
                    | ComponentAddress::EddsaEd25519VirtualAccount(..),
            ) =>
            {
                self.auth_required.insert(*component_address);
            }
            _ => {}
        }

        Ok(())
    }

    fn visit_set_component_royalty_config(
        &mut self,
        component_address: &mut crate::model::value::ast::ManifestAstValue,
        _: &mut crate::model::value::ast::ManifestAstValue,
    ) -> Result<()> {
        match component_address {
            ManifestAstValue::ComponentAddress {
                address: component_address,
            }
            | ManifestAstValue::Address {
                address:
                    EntityAddress::ComponentAddress {
                        address: component_address,
                    },
            } if matches!(
                component_address.address,
                ComponentAddress::Account(..)
                    | ComponentAddress::EcdsaSecp256k1VirtualAccount(..)
                    | ComponentAddress::EddsaEd25519VirtualAccount(..),
            ) =>
            {
                self.auth_required.insert(*component_address);
            }
            _ => {}
        }
        Ok(())
    }

    fn visit_claim_component_royalty(
        &mut self,
        component_address: &mut crate::model::value::ast::ManifestAstValue,
    ) -> Result<()> {
        match component_address {
            ManifestAstValue::ComponentAddress {
                address: component_address,
            }
            | ManifestAstValue::Address {
                address:
                    EntityAddress::ComponentAddress {
                        address: component_address,
                    },
            } if matches!(
                component_address.address,
                ComponentAddress::Account(..)
                    | ComponentAddress::EcdsaSecp256k1VirtualAccount(..)
                    | ComponentAddress::EddsaEd25519VirtualAccount(..),
            ) =>
            {
                self.auth_required.insert(*component_address);
            }
            _ => {}
        }
        Ok(())
    }

    fn visit_set_method_access_rule(
        &mut self,
        entity_address: &mut crate::model::value::ast::ManifestAstValue,
        _: &mut crate::model::value::ast::ManifestAstValue,
        _: &mut crate::model::value::ast::ManifestAstValue,
        _: &mut crate::model::value::ast::ManifestAstValue,
    ) -> Result<()> {
        match entity_address {
            ManifestAstValue::ComponentAddress {
                address: component_address,
            }
            | ManifestAstValue::Address {
                address:
                    EntityAddress::ComponentAddress {
                        address: component_address,
                    },
            } if matches!(
                component_address.address,
                ComponentAddress::Account(..)
                    | ComponentAddress::EcdsaSecp256k1VirtualAccount(..)
                    | ComponentAddress::EddsaEd25519VirtualAccount(..),
            ) =>
            {
                self.auth_required.insert(*component_address);
            }
            _ => {}
        }
        Ok(())
    }
}
