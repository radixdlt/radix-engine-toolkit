use crate::error::Error;
use crate::model::value::{ManifestAstValue, ManifestAstValueKind};
use crate::model::NonFungibleGlobalId;
use crate::{EntityAddress, ManifestAstValueVisitor};

/// A value visitor whose main responsibility is to perform aliasing on all encountered values. As
/// an example, this is the main visitor responsible for turing a Tuple(ResourceAddress, NFLocalId)
/// to a NonFungibleGlobalAddress
#[derive(Debug, Default)]
pub struct ValueAliasingVisitor;

impl ManifestAstValueVisitor for ValueAliasingVisitor {
    fn visit_tuple(&mut self, value: &mut ManifestAstValue) -> crate::Result<()> {
        if let ManifestAstValue::Tuple { ref elements } = value {
            // Case: NonFungibleGlobalId - A tuple of ResourceAddress and NonFungibleLocalId
            match (elements.get(0), elements.get(1)) {
                (
                    Some(ManifestAstValue::ResourceAddress {
                        address: resource_address,
                    }),
                    Some(ManifestAstValue::NonFungibleLocalId {
                        value: non_fungible_local_id,
                    }),
                ) if elements.len() == 2 => {
                    *value = ManifestAstValue::NonFungibleGlobalId {
                        address: NonFungibleGlobalId {
                            resource_address: *resource_address,
                            non_fungible_local_id: non_fungible_local_id.clone(),
                        },
                    };
                    Ok(())
                }
                _ => Ok(()),
            }
        } else {
            Err(Error::Infallible {
                message: "Must be a tuple!".into(),
            })
        }
    }

    fn visit_array(&mut self, value: &mut ManifestAstValue) -> crate::Result<()> {
        if let ManifestAstValue::Array {
            ref elements,
            element_kind: ManifestAstValueKind::U8,
        } = value
        {
            // Case: Bytes - An array of u8
            let mut bytes = Vec::new();
            for element in elements.iter() {
                match element {
                    ManifestAstValue::U8 { value } => bytes.push(*value),
                    // If we encounter anything that is not a U8, then we stop the aliasing op and
                    // don't continue.
                    _ => return Ok(()),
                }
            }
            *value = ManifestAstValue::Bytes { value: bytes };
            Ok(())
        } else if let ManifestAstValue::Array { .. } = value {
            Ok(())
        } else {
            Err(Error::Infallible {
                message: "Must be an array!".into(),
            })
        }
    }

    fn visit_address(&mut self, value: &mut ManifestAstValue) -> crate::Result<()> {
        match value {
            ManifestAstValue::Address { address } => {
                match address {
                    EntityAddress::ComponentAddress { address } => {
                        *value = ManifestAstValue::ComponentAddress { address: *address };
                    }
                    EntityAddress::ResourceAddress { address } => {
                        *value = ManifestAstValue::ResourceAddress { address: *address };
                    }
                    EntityAddress::PackageAddress { address } => {
                        *value = ManifestAstValue::PackageAddress { address: *address };
                    }
                }

                Ok(())
            }
            _ => Err(Error::Infallible {
                message: "Must be an address!".into(),
            }),
        }
    }
}
