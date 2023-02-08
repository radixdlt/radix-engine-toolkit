use super::ValueVisitor;
use crate::error::Error;
use crate::model::{NonFungibleGlobalId, Value};

/// A value visitor whose main responsibility is to perform aliasing on all encountered values. As
/// an example, this is the main visitor responsible for turing a Tuple(ResourceAddress, NFLocalId)
/// to a NonFungibleGlobalAddress
#[derive(Debug, Default)]
pub struct ValueAliasingVisitor;

impl ValueVisitor for ValueAliasingVisitor {
    fn visit_tuple(&mut self, value: &mut crate::Value) -> crate::Result<()> {
        if let Value::Tuple { ref elements } = value {
            // Case: NonFungibleGlobalId - A tuple of ResourceAddress and NonFungibleLocalId
            match (elements.get(0), elements.get(1)) {
                (
                    Some(Value::ResourceAddress {
                        address: resource_address,
                    }),
                    Some(Value::NonFungibleLocalId {
                        value: non_fungible_local_id,
                    }),
                ) if elements.len() == 2 => {
                    *value = Value::NonFungibleGlobalId {
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

    fn visit_array(&mut self, value: &mut crate::Value) -> crate::Result<()> {
        if let Value::Array { ref elements, .. } = value {
            // Case: Bytes - An array of u8
            let mut bytes = Vec::new();
            for element in elements.iter() {
                match element {
                    Value::U8 { value } => bytes.push(*value),
                    _ => return Ok(()),
                }
            }
            *value = Value::Bytes { value: bytes };
            Ok(())
        } else {
            Err(Error::Infallible {
                message: "Must be an array!".into(),
            })
        }
    }
}
