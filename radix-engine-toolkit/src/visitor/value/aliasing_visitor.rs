// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use crate::error::VisitorError;
use crate::model::value::ast::{ManifestAstValue, ManifestAstValueKind};
use crate::visitor::ManifestAstValueVisitor;

/// A value visitor whose main responsibility is to perform aliasing on all encountered values. As
/// an example, this is the main visitor responsible for turing a Tuple(ResourceAddress, NFLocalId)
/// to a NonFungibleGlobalAddress
#[derive(Debug, Default)]
pub struct ValueAliasingVisitor;

impl ManifestAstValueVisitor for ValueAliasingVisitor {
    fn visit_tuple(&mut self, value: &mut ManifestAstValue) -> Result<(), VisitorError> {
        if let ManifestAstValue::Tuple {
            fields: ref elements,
        } = value
        {
            // Case: NonFungibleGlobalId - A tuple of ResourceAddress and NonFungibleLocalId
            match (elements.get(0), elements.get(1)) {
                (
                    Some(ManifestAstValue::Address { value: address }),
                    Some(ManifestAstValue::NonFungibleLocalId {
                        value: non_fungible_local_id,
                    }),
                ) if elements.len() == 2 && address.node_id().is_global_resource() => {
                    *value = ManifestAstValue::NonFungibleGlobalId {
                        resource_address: *address,
                        non_fungible_local_id: non_fungible_local_id.clone(),
                    };
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn visit_array(&mut self, value: &mut ManifestAstValue) -> Result<(), VisitorError> {
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
            *value = ManifestAstValue::Bytes { hex: bytes };
        }
        Ok(())
    }
}
