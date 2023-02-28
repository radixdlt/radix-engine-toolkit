use std::collections::BTreeSet;

use crate::error::{Error, Result};
use crate::model::address::{
    EntityAddress, NetworkAwareComponentAddress, NetworkAwarePackageAddress,
    NetworkAwareResourceAddress,
};
use crate::model::value::ast::ManifestAstValue;
use crate::visitor::ManifestAstValueVisitor;

/// An address aggregator visitor which collects all of the encountered global entity addresses and
/// stored them in its state.
#[derive(Debug, Default)]
pub struct AddressAggregatorVisitor {
    pub component_addresses: BTreeSet<NetworkAwareComponentAddress>,
    pub resource_addresses: BTreeSet<NetworkAwareResourceAddress>,
    pub package_addresses: BTreeSet<NetworkAwarePackageAddress>,
}

impl ManifestAstValueVisitor for AddressAggregatorVisitor {
    fn visit_address(&mut self, value: &mut ManifestAstValue) -> Result<()> {
        if let ManifestAstValue::Address { address } = value {
            match address {
                EntityAddress::ComponentAddress { address } => {
                    self.component_addresses.insert(*address);
                }
                EntityAddress::ResourceAddress { address } => {
                    self.resource_addresses.insert(*address);
                }
                EntityAddress::PackageAddress { address } => {
                    self.package_addresses.insert(*address);
                }
            }
            Ok(())
        } else {
            Err(Error::Infallible {
                message: "Expected component address!".into(),
            })
        }
    }

    fn visit_component_address(&mut self, value: &mut ManifestAstValue) -> Result<()> {
        if let ManifestAstValue::ComponentAddress { address } = value {
            self.component_addresses.insert(*address);
            Ok(())
        } else {
            Err(Error::Infallible {
                message: "Expected component address!".into(),
            })
        }
    }

    fn visit_resource_address(&mut self, value: &mut ManifestAstValue) -> Result<()> {
        if let ManifestAstValue::ResourceAddress { address } = value {
            self.resource_addresses.insert(*address);
            Ok(())
        } else {
            Err(Error::Infallible {
                message: "Expected resource address!".into(),
            })
        }
    }

    fn visit_package_address(&mut self, value: &mut ManifestAstValue) -> Result<()> {
        if let ManifestAstValue::PackageAddress { address } = value {
            self.package_addresses.insert(*address);
            Ok(())
        } else {
            Err(Error::Infallible {
                message: "Expected package address!".into(),
            })
        }
    }

    fn visit_non_fungible_global_id(&mut self, value: &mut ManifestAstValue) -> Result<()> {
        if let ManifestAstValue::NonFungibleGlobalId { address } = value {
            self.resource_addresses.insert(address.resource_address);
            Ok(())
        } else {
            Err(Error::Infallible {
                message: "Expected non-fungible global id!".into(),
            })
        }
    }
}
