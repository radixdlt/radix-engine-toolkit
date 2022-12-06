// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at

//   http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use crate::error::Error;
use crate::model::{Bech32Coder, NetworkAwareComponentAddress, NodeIdentifier, Value, ValueKind};
use radix_transaction::manifest::ast::{ScryptoReceiver as AstScryptoReceiver, Value as AstValue};

#[derive(Clone, Debug)]
pub enum ScryptoReceiver {
    ComponentAddress(NetworkAwareComponentAddress),
    Component(NodeIdentifier),
}

impl ScryptoReceiver {
    pub fn to_ast_scrypto_receiver(&self, bech32_coder: &Bech32Coder) -> AstScryptoReceiver {
        match self {
            Self::Component(identifier) => {
                AstScryptoReceiver::Component(AstValue::String(identifier.to_string()))
            }
            Self::ComponentAddress(address) => {
                let address_string = bech32_coder
                    .encoder
                    .encode_component_address_to_string(&address.address);
                AstScryptoReceiver::Global(AstValue::String(address_string))
            }
        }
    }

    pub fn from_ast_scrypto_receiver(
        receiver: &AstScryptoReceiver,
        bech32_coder: &Bech32Coder,
    ) -> Result<Self, Error> {
        match receiver {
            AstScryptoReceiver::Component(value) => {
                if let AstValue::String(identifier_hex) = value {
                    Ok(Self::Component(identifier_hex.parse()?))
                } else {
                    Err(Error::UnexpectedContents {
                        kind_being_parsed: ValueKind::Component,
                        allowed_children_kinds: vec![ValueKind::String],
                        found_child_kind: value.kind().into(),
                    })
                }
            }
            AstScryptoReceiver::Global(value) => {
                if let AstValue::String(address_string) = value {
                    Ok(Self::ComponentAddress(NetworkAwareComponentAddress {
                        network_id: bech32_coder.network_id(),
                        address: bech32_coder
                            .decoder
                            .validate_and_decode_component_address(address_string)?,
                    }))
                } else {
                    Err(Error::UnexpectedContents {
                        kind_being_parsed: ValueKind::ComponentAddress,
                        allowed_children_kinds: vec![ValueKind::String],
                        found_child_kind: value.kind().into(),
                    })
                }
            }
        }
    }
}

impl From<ScryptoReceiver> for Value {
    fn from(receiver: ScryptoReceiver) -> Self {
        match receiver {
            ScryptoReceiver::Component(identifier) => Value::Component { identifier },
            ScryptoReceiver::ComponentAddress(address) => Value::ComponentAddress { address },
        }
    }
}

impl TryFrom<Value> for ScryptoReceiver {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Component { identifier } => Ok(ScryptoReceiver::Component(identifier)),
            Value::ComponentAddress { address } => Ok(ScryptoReceiver::ComponentAddress(address)),
            _ => Err(Error::InvalidType {
                expected_types: vec![ValueKind::Component, ValueKind::ComponentAddress],
                actual_type: value.kind(),
            }),
        }
    }
}
