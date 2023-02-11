use std::collections::BTreeSet;

use crate::{Value, ValueVisitor};

/// A value visitor whose main job is to find all of the different network IDs that the different
/// addresses use. This is typically used in operations where we wish to check for network id
/// mismatches.
#[derive(Debug, Default)]
pub struct ValueNetworkAggregatorVisitor(pub BTreeSet<u8>);

impl ValueVisitor for ValueNetworkAggregatorVisitor {
    fn visit_component_address(&mut self, value: &mut crate::Value) -> crate::Result<()> {
        if let Value::ComponentAddress { address } = value {
            self.0.insert(address.network_id);
            Ok(())
        } else {
            Err(crate::Error::Infallible {
                message: "Expected component address!".into(),
            })
        }
    }

    fn visit_resource_address(&mut self, value: &mut crate::Value) -> crate::Result<()> {
        if let Value::ResourceAddress { address } = value {
            self.0.insert(address.network_id);
            Ok(())
        } else {
            Err(crate::Error::Infallible {
                message: "Expected resource address!".into(),
            })
        }
    }

    fn visit_package_address(&mut self, value: &mut crate::Value) -> crate::Result<()> {
        if let Value::PackageAddress { address } = value {
            self.0.insert(address.network_id);
            Ok(())
        } else {
            Err(crate::Error::Infallible {
                message: "Expected package address!".into(),
            })
        }
    }

    fn visit_non_fungible_global_id(&mut self, value: &mut crate::Value) -> crate::Result<()> {
        if let Value::NonFungibleGlobalId { address } = value {
            self.0.insert(address.resource_address.network_id);
            Ok(())
        } else {
            Err(crate::Error::Infallible {
                message: "Expected non-fungible global id!".into(),
            })
        }
    }
}
