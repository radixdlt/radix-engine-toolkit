use std::collections::HashSet;

use crate::{
    NetworkAwareComponentAddress, NetworkAwarePackageAddress, NetworkAwareResourceAddress, Value,
    ValueVisitor,
};

/// An address aggregator visitor which collects all of the encountered global entity addresses and
/// stored them in its state.
#[derive(Debug, Default)]
pub struct AddressValueAggregator {
    pub component_addresses: HashSet<NetworkAwareComponentAddress>,
    pub resource_addresses: HashSet<NetworkAwareResourceAddress>,
    pub package_addresses: HashSet<NetworkAwarePackageAddress>,
}

impl ValueVisitor for AddressValueAggregator {
    fn visit_component_address(&mut self, value: &mut crate::Value) -> crate::Result<()> {
        if let Value::ComponentAddress { address } = value {
            self.component_addresses.insert(*address);
            Ok(())
        } else {
            Err(crate::Error::Infallible {
                message: "Expected component address!".into(),
            })
        }
    }

    fn visit_resource_address(&mut self, value: &mut crate::Value) -> crate::Result<()> {
        if let Value::ResourceAddress { address } = value {
            self.resource_addresses.insert(*address);
            Ok(())
        } else {
            Err(crate::Error::Infallible {
                message: "Expected resource address!".into(),
            })
        }
    }

    fn visit_package_address(&mut self, value: &mut crate::Value) -> crate::Result<()> {
        if let Value::PackageAddress { address } = value {
            self.package_addresses.insert(*address);
            Ok(())
        } else {
            Err(crate::Error::Infallible {
                message: "Expected package address!".into(),
            })
        }
    }

    fn visit_non_fungible_global_id(&mut self, value: &mut crate::Value) -> crate::Result<()> {
        if let Value::NonFungibleGlobalId { address } = value {
            self.resource_addresses.insert(address.resource_address);
            Ok(())
        } else {
            Err(crate::Error::Infallible {
                message: "Expected non-fungible global id!".into(),
            })
        }
    }
}
