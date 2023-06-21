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

use core::cell::RefCell;
use radix_engine_common::data::manifest::*;
use radix_engine_common::types::*;
use sbor::rust::cell::Ref;
use sbor::rust::prelude::*;
use sbor::traversal::*;
use sbor::*;
use transaction::prelude::{ManifestAddress, ManifestBucket, ManifestExpression};

#[derive(Clone, PartialEq, Eq)]
pub struct IndexedManifestValue {
    bytes: Vec<u8>,
    manifest_value: RefCell<Option<ManifestValue>>,

    static_addresses: Vec<NodeId>,
    named_addresses: Vec<u32>,
    buckets: Vec<ManifestBucket>,
    expressions: Vec<ManifestExpression>,
}

impl IndexedManifestValue {
    fn new(bytes: Vec<u8>) -> Result<Self, DecodeError> {
        let mut traverser = ManifestTraverser::new(
            &bytes,
            MANIFEST_SBOR_V1_MAX_DEPTH,
            ExpectedStart::PayloadPrefix(MANIFEST_SBOR_V1_PAYLOAD_PREFIX),
            true,
        );
        let mut static_addresses = Vec::new();
        let mut named_addresses = Vec::new();
        let mut buckets = Vec::new();
        let mut expressions = Vec::new();
        loop {
            let event = traverser.next_event();
            match event.event {
                TraversalEvent::ContainerStart(_) => {}
                TraversalEvent::ContainerEnd(_) => {}
                TraversalEvent::TerminalValue(r) => {
                    if let traversal::TerminalValueRef::Custom(c) = r {
                        match c.0 {
                            ManifestCustomValue::Address(address) => match address {
                                ManifestAddress::Static(node_id) => static_addresses.push(node_id),
                                ManifestAddress::Named(id) => named_addresses.push(id),
                            },
                            ManifestCustomValue::Bucket(bucket) => buckets.push(bucket),
                            ManifestCustomValue::Expression(expression) => {
                                expressions.push(expression)
                            }
                            ManifestCustomValue::Proof(_)
                            | ManifestCustomValue::Blob(_)
                            | ManifestCustomValue::Decimal(_)
                            | ManifestCustomValue::PreciseDecimal(_)
                            | ManifestCustomValue::NonFungibleLocalId(_)
                            | ManifestCustomValue::AddressReservation(_) => {}
                        }
                    }
                }
                TraversalEvent::TerminalValueBatch(_) => {}
                TraversalEvent::End => {
                    break;
                }
                TraversalEvent::DecodeError(e) => {
                    return Err(e);
                }
            }
        }

        Ok(Self {
            bytes,
            static_addresses,
            named_addresses,
            buckets,
            expressions,
            manifest_value: RefCell::new(None),
        })
    }

    fn get_manifest_value(&self) -> Ref<ManifestValue> {
        let is_empty = { self.manifest_value.borrow().is_none() };

        if is_empty {
            *self.manifest_value.borrow_mut() = Some(
                manifest_decode::<ManifestValue>(&self.bytes)
                    .expect("Failed to decode bytes in IndexedManifestValue"),
            );
        }

        Ref::map(self.manifest_value.borrow(), |v| v.as_ref().unwrap())
    }

    pub fn unit() -> Self {
        Self::from_typed(&())
    }

    pub fn from_typed<T: ManifestEncode + ?Sized>(value: &T) -> Self {
        let bytes = manifest_encode(value).expect("Failed to encode trusted Rust value");
        Self::new(bytes).expect("Failed to index trusted Rust value")
    }

    pub fn from_manifest_value(value: &ManifestValue) -> Self {
        let bytes = manifest_encode(value).expect("Failed to encode trusted ManifestValue");
        Self::new(bytes).expect("Failed to index trusted ManifestValue")
    }

    pub fn from_slice(slice: &[u8]) -> Result<Self, DecodeError> {
        Self::new(slice.to_vec())
    }

    pub fn from_vec(vec: Vec<u8>) -> Result<Self, DecodeError> {
        Self::new(vec)
    }

    pub fn to_manifest_value(&self) -> ManifestValue {
        self.get_manifest_value().clone()
    }

    pub fn as_manifest_value(&self) -> Ref<ManifestValue> {
        self.get_manifest_value()
    }

    pub fn as_typed<T: ManifestDecode>(&self) -> Result<T, DecodeError> {
        manifest_decode(&self.bytes)
    }

    pub fn as_slice(&self) -> &[u8] {
        self.bytes.as_slice()
    }

    pub fn static_addresses(&self) -> &Vec<NodeId> {
        &self.static_addresses
    }

    pub fn named_addresses(&self) -> &Vec<u32> {
        &self.named_addresses
    }

    pub fn expressions(&self) -> &Vec<ManifestExpression> {
        &self.expressions
    }

    pub fn buckets(&self) -> &Vec<ManifestBucket> {
        &self.buckets
    }
}
