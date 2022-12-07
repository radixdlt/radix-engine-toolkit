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

use radix_transaction::manifest::ast::{RENode as AstRENode, Value as AstValue};

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

use crate::error::Error;
use crate::model::{Identifier, NodeIdentifier};

use crate::model::value::ValueKind;

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq, Clone)]
#[serde(tag = "type", content = "identifier")]
pub enum RENode {
    Bucket(Identifier),
    Proof(Identifier),

    AuthZoneStack(#[serde_as(as = "DisplayFromStr")] u32),
    Worktop,

    Global(String),
    KeyValueStore(NodeIdentifier),
    NonFungibleStore(NodeIdentifier),
    Component(NodeIdentifier),
    EpochManager(NodeIdentifier),
    Vault(NodeIdentifier),
    ResourceManager(NodeIdentifier),
    Package(NodeIdentifier),
    Clock(NodeIdentifier),
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
pub enum RENodeKind {
    Bucket,
    Proof,

    AuthZoneStack,
    Worktop,

    Global,
    KeyValueStore,
    NonFungibleStore,
    Component,
    EpochManager,
    Vault,
    ResourceManager,
    Package,
    Clock,
}

// ============
// Conversions
// ============

pub fn ast_re_node_from_re_node(re_node: &RENode) -> AstRENode {
    match re_node {
        RENode::Bucket(identifier) => {
            let ast_value = match identifier {
                Identifier::String(string) => AstValue::String(string.clone()),
                Identifier::U32(number) => AstValue::U32(*number),
            };
            AstRENode::Bucket(ast_value)
        }
        RENode::Proof(identifier) => {
            let ast_value = match identifier {
                Identifier::String(string) => AstValue::String(string.clone()),
                Identifier::U32(number) => AstValue::U32(*number),
            };
            AstRENode::Proof(ast_value)
        }

        RENode::AuthZoneStack(auth_zone_id) => {
            let ast_value = AstValue::U32(*auth_zone_id);
            AstRENode::AuthZoneStack(ast_value)
        }
        RENode::Worktop => AstRENode::Worktop,

        RENode::Global(identifier) => {
            let ast_value = AstValue::String(identifier.to_owned());
            AstRENode::Global(ast_value)
        }
        RENode::KeyValueStore(identifier) => {
            let ast_value = AstValue::String(identifier.to_string());
            AstRENode::KeyValueStore(ast_value)
        }
        RENode::NonFungibleStore(identifier) => {
            let ast_value = AstValue::String(identifier.to_string());
            AstRENode::NonFungibleStore(ast_value)
        }
        RENode::Component(identifier) => {
            let ast_value = AstValue::String(identifier.to_string());
            AstRENode::Component(ast_value)
        }
        RENode::EpochManager(identifier) => {
            let ast_value = AstValue::String(identifier.to_string());
            AstRENode::EpochManager(ast_value)
        }
        RENode::Vault(identifier) => {
            let ast_value = AstValue::String(identifier.to_string());
            AstRENode::Vault(ast_value)
        }
        RENode::ResourceManager(identifier) => {
            let ast_value = AstValue::String(identifier.to_string());
            AstRENode::ResourceManager(ast_value)
        }
        RENode::Package(identifier) => {
            let ast_value = AstValue::String(identifier.to_string());
            AstRENode::Package(ast_value)
        }
        RENode::Clock(identifier) => {
            let ast_value = AstValue::String(identifier.to_string());
            AstRENode::Clock(ast_value)
        }
    }
}

pub fn re_node_from_ast_re_node(ast_re_node: &AstRENode) -> Result<RENode, Error> {
    let re_node = match ast_re_node {
        AstRENode::Bucket(identifier) => {
            if let AstValue::U32(identifier) = identifier {
                RENode::Bucket(Identifier::U32(*identifier))
            } else if let AstValue::String(identifier) = identifier {
                RENode::Bucket(Identifier::String(identifier.clone()))
            } else {
                Err(Error::UnexpectedReNodeContents {
                    kind_being_parsed: RENodeKind::Bucket,
                    allowed_children_kinds: vec![ValueKind::U32, ValueKind::String],
                    found_child_kind: identifier.kind().into(),
                })?
            }
        }
        AstRENode::Proof(identifier) => {
            if let AstValue::U32(identifier) = identifier {
                RENode::Proof(Identifier::U32(*identifier))
            } else if let AstValue::String(identifier) = identifier {
                RENode::Proof(Identifier::String(identifier.clone()))
            } else {
                Err(Error::UnexpectedReNodeContents {
                    kind_being_parsed: RENodeKind::Proof,
                    allowed_children_kinds: vec![ValueKind::U32, ValueKind::String],
                    found_child_kind: identifier.kind().into(),
                })?
            }
        }

        AstRENode::AuthZoneStack(identifier) => {
            if let AstValue::U32(identifier) = identifier {
                RENode::AuthZoneStack(*identifier)
            } else {
                Err(Error::UnexpectedReNodeContents {
                    kind_being_parsed: RENodeKind::AuthZoneStack,
                    allowed_children_kinds: vec![ValueKind::U32],
                    found_child_kind: identifier.kind().into(),
                })?
            }
        }

        AstRENode::Worktop => RENode::Worktop,

        AstRENode::Global(identifier) => {
            if let AstValue::String(identifier) = identifier {
                RENode::Global(identifier.clone())
            } else {
                Err(Error::UnexpectedReNodeContents {
                    kind_being_parsed: RENodeKind::Global,
                    allowed_children_kinds: vec![ValueKind::String],
                    found_child_kind: identifier.kind().into(),
                })?
            }
        }
        AstRENode::KeyValueStore(identifier) => {
            if let AstValue::String(identifier) = identifier {
                RENode::KeyValueStore(identifier.parse()?)
            } else {
                Err(Error::UnexpectedReNodeContents {
                    kind_being_parsed: RENodeKind::KeyValueStore,
                    allowed_children_kinds: vec![ValueKind::String],
                    found_child_kind: identifier.kind().into(),
                })?
            }
        }
        AstRENode::NonFungibleStore(identifier) => {
            if let AstValue::String(identifier) = identifier {
                RENode::NonFungibleStore(identifier.parse()?)
            } else {
                Err(Error::UnexpectedReNodeContents {
                    kind_being_parsed: RENodeKind::NonFungibleStore,
                    allowed_children_kinds: vec![ValueKind::String],
                    found_child_kind: identifier.kind().into(),
                })?
            }
        }
        AstRENode::Component(identifier) => {
            if let AstValue::String(identifier) = identifier {
                RENode::Component(identifier.parse()?)
            } else {
                Err(Error::UnexpectedReNodeContents {
                    kind_being_parsed: RENodeKind::Component,
                    allowed_children_kinds: vec![ValueKind::String],
                    found_child_kind: identifier.kind().into(),
                })?
            }
        }
        AstRENode::EpochManager(identifier) => {
            if let AstValue::String(identifier) = identifier {
                RENode::EpochManager(identifier.parse()?)
            } else {
                Err(Error::UnexpectedReNodeContents {
                    kind_being_parsed: RENodeKind::EpochManager,
                    allowed_children_kinds: vec![ValueKind::String],
                    found_child_kind: identifier.kind().into(),
                })?
            }
        }
        AstRENode::Vault(identifier) => {
            if let AstValue::String(identifier) = identifier {
                RENode::Vault(identifier.parse()?)
            } else {
                Err(Error::UnexpectedReNodeContents {
                    kind_being_parsed: RENodeKind::Vault,
                    allowed_children_kinds: vec![ValueKind::String],
                    found_child_kind: identifier.kind().into(),
                })?
            }
        }
        AstRENode::ResourceManager(identifier) => {
            if let AstValue::String(identifier) = identifier {
                RENode::ResourceManager(identifier.parse()?)
            } else {
                Err(Error::UnexpectedReNodeContents {
                    kind_being_parsed: RENodeKind::ResourceManager,
                    allowed_children_kinds: vec![ValueKind::String],
                    found_child_kind: identifier.kind().into(),
                })?
            }
        }
        AstRENode::Package(identifier) => {
            if let AstValue::String(identifier) = identifier {
                RENode::Package(identifier.parse()?)
            } else {
                Err(Error::UnexpectedReNodeContents {
                    kind_being_parsed: RENodeKind::Package,
                    allowed_children_kinds: vec![ValueKind::String],
                    found_child_kind: identifier.kind().into(),
                })?
            }
        }
        AstRENode::Clock(identifier) => {
            if let AstValue::String(identifier) = identifier {
                RENode::Clock(identifier.parse()?)
            } else {
                Err(Error::UnexpectedReNodeContents {
                    kind_being_parsed: RENodeKind::Clock,
                    allowed_children_kinds: vec![ValueKind::String],
                    found_child_kind: identifier.kind().into(),
                })?
            }
        }
    };
    Ok(re_node)
}
