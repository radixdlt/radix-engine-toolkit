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
        component_address: &mut crate::Value,
        method_name: &mut crate::Value,
        arguments: &mut Option<Vec<crate::Value>>,
    ) -> crate::Result<()> {
        match (component_address, method_name, arguments) {
            (
                Value::ComponentAddress {
                    address: component_address,
                },
                Value::String { value: method_name },
                _,
            ) => {
                // We only care if this a component address
                match component_address.address {
                    ComponentAddress::Account(..)
                    | ComponentAddress::EcdsaSecp256k1VirtualAccount(..)
                    | ComponentAddress::EddsaEd25519VirtualAccount(..) => {
                        if Self::AUTH_REQUIRING_METHODS.contains(&method_name.as_str()) {
                            self.auth_required.insert(*component_address);
                        }
                        if Self::WITHDRAW_METHODS.contains(&method_name.as_str()) {
                            self.accounts_withdrawn_from.insert(*component_address);
                        }
                        if Self::DEPOSIT_METHODS.contains(&method_name.as_str()) {
                            self.accounts_deposited_into.insert(*component_address);
                        }

                        Ok(())
                    }
                    _ => Ok(()),
                }
            }
            _ => Err(crate::Error::Infallible {
                message: "Call Method has incorrect arguments!".into(),
            }),
        }
    }
}
