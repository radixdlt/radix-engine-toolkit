use std::collections::BTreeSet;

use crate::{InstructionVisitor, NetworkAwareComponentAddress, Value};
use scrypto::prelude::ComponentAddress;

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
        "lock_fee",
        "lock_contingent_fee",
        "withdraw",
        "withdraw_by_amount",
        "withdraw_by_ids",
        "lock_fee_and_withdraw",
        "lock_fee_and_withdraw_by_amount",
        "lock_fee_and_withdraw_by_ids",
        "create_proof",
        "create_proof_by_amount",
        "create_proof_by_ids",
    ];
    const WITHDRAW_METHODS: &'static [&'static str] =
        &["withdraw", "withdraw_by_amount", "withdraw_by_ids"];
    const DEPOSIT_METHODS: &'static [&'static str] = &["deposit", "deposit_batch"];
}

impl InstructionVisitor for AccountInteractionsInstructionVisitor {
    fn visit_call_method(
        &mut self,
        component_address: &mut Value,
        method_name: &mut Value,
        _: &mut Option<Vec<Value>>,
    ) -> crate::Result<()> {
        if let (
            Value::ComponentAddress {
                address:
                    component_address @ NetworkAwareComponentAddress {
                        address:
                            ComponentAddress::Account(..)
                            | ComponentAddress::EcdsaSecp256k1VirtualAccount(..)
                            | ComponentAddress::EddsaEd25519VirtualAccount(..),
                        ..
                    },
            },
            Value::String { value: method_name },
        ) = (component_address, method_name)
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
        };
        Ok(())
    }

    fn visit_set_metadata(
        &mut self,
        entity_address: &mut Value,
        _: &mut Value,
        _: &mut Value,
    ) -> crate::Result<()> {
        if let Value::ComponentAddress {
            address:
                component_address @ NetworkAwareComponentAddress {
                    address:
                        ComponentAddress::Account(..)
                        | ComponentAddress::EcdsaSecp256k1VirtualAccount(..)
                        | ComponentAddress::EddsaEd25519VirtualAccount(..),
                    ..
                },
        } = entity_address
        {
            self.auth_required.insert(*component_address);
        };
        Ok(())
    }

    fn visit_set_component_royalty_config(
        &mut self,
        component_address: &mut crate::Value,
        _: &mut crate::Value,
    ) -> crate::Result<()> {
        if let Value::ComponentAddress {
            address:
                component_address @ NetworkAwareComponentAddress {
                    address:
                        ComponentAddress::Account(..)
                        | ComponentAddress::EcdsaSecp256k1VirtualAccount(..)
                        | ComponentAddress::EddsaEd25519VirtualAccount(..),
                    ..
                },
        } = component_address
        {
            self.auth_required.insert(*component_address);
        };
        Ok(())
    }

    fn visit_claim_component_royalty(
        &mut self,
        component_address: &mut crate::Value,
    ) -> crate::Result<()> {
        if let Value::ComponentAddress {
            address:
                component_address @ NetworkAwareComponentAddress {
                    address:
                        ComponentAddress::Account(..)
                        | ComponentAddress::EcdsaSecp256k1VirtualAccount(..)
                        | ComponentAddress::EddsaEd25519VirtualAccount(..),
                    ..
                },
        } = component_address
        {
            self.auth_required.insert(*component_address);
        };
        Ok(())
    }

    fn visit_set_method_access_rule(
        &mut self,
        entity_address: &mut crate::Value,
        _: &mut crate::Value,
        _: &mut crate::Value,
        _: &mut crate::Value,
    ) -> crate::Result<()> {
        if let Value::ComponentAddress {
            address:
                component_address @ NetworkAwareComponentAddress {
                    address:
                        ComponentAddress::Account(..)
                        | ComponentAddress::EcdsaSecp256k1VirtualAccount(..)
                        | ComponentAddress::EddsaEd25519VirtualAccount(..),
                    ..
                },
        } = entity_address
        {
            self.auth_required.insert(*component_address);
        };
        Ok(())
    }
}
