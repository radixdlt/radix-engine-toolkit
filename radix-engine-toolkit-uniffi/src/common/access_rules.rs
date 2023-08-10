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

use radix_engine::types::FromPublicKey;

use crate::prelude::*;

#[derive(Clone, Debug, Object)]
pub struct AccessRule(pub NativeAccessRule);

#[uniffi::export]
impl AccessRule {
    #[uniffi::constructor]
    pub fn require(resource_or_non_fungible: ResourceOrNonFungible) -> Result<Arc<Self>> {
        let access_rule = NativeAccessRule::Protected(NativeAccessRuleNode::ProofRule(
            NativeProofRule::Require(resource_or_non_fungible.to_native()?),
        ));
        Ok(Arc::new(Self(access_rule)))
    }

    #[uniffi::constructor]
    pub fn require_amount(amount: Arc<Decimal>, resource: Arc<Address>) -> Result<Arc<Self>> {
        let resource_address = NativeResourceAddress::try_from(*resource)?;
        let access_rule = NativeAccessRule::Protected(NativeAccessRuleNode::ProofRule(
            NativeProofRule::AmountOf(amount.0, resource_address),
        ));
        Ok(Arc::new(Self(access_rule)))
    }

    #[uniffi::constructor]
    pub fn require_count_of(count: u8, resources: Vec<ResourceOrNonFungible>) -> Result<Arc<Self>> {
        let access_rule = NativeAccessRule::Protected(NativeAccessRuleNode::ProofRule(
            NativeProofRule::CountOf(count, resources.to_native()?),
        ));
        Ok(Arc::new(Self(access_rule)))
    }

    #[uniffi::constructor]
    pub fn require_all_of(resources: Vec<ResourceOrNonFungible>) -> Result<Arc<Self>> {
        let access_rule = NativeAccessRule::Protected(NativeAccessRuleNode::ProofRule(
            NativeProofRule::AllOf(resources.to_native()?),
        ));
        Ok(Arc::new(Self(access_rule)))
    }

    #[uniffi::constructor]
    pub fn require_any_of(resources: Vec<ResourceOrNonFungible>) -> Result<Arc<Self>> {
        let access_rule = NativeAccessRule::Protected(NativeAccessRuleNode::ProofRule(
            NativeProofRule::AnyOf(resources.to_native()?),
        ));
        Ok(Arc::new(Self(access_rule)))
    }

    #[uniffi::constructor]
    pub fn require_virtual_signature(public_key: PublicKey) -> Result<Arc<Self>> {
        let public_key = NativePublicKey::try_from(public_key)?;
        let non_fungible_global_id = NativeNonFungibleGlobalId::from_public_key(&public_key);
        let access_rule = native_rule!(native_require(non_fungible_global_id));
        Ok(Arc::new(Self(access_rule)))
    }

    #[uniffi::constructor]
    pub fn allow_all() -> Arc<Self> {
        Arc::new(Self(NativeAccessRule::AllowAll))
    }

    #[uniffi::constructor]
    pub fn deny_all() -> Arc<Self> {
        Arc::new(Self(NativeAccessRule::DenyAll))
    }

    pub fn or(&self, other: Arc<Self>) -> Arc<Self> {
        let access_rule = match (&self.0, &other.0) {
            (NativeAccessRule::AllowAll, _) | (_, NativeAccessRule::AllowAll) => {
                NativeAccessRule::AllowAll
            }
            (NativeAccessRule::Protected(rule1), NativeAccessRule::Protected(rule2)) => {
                NativeAccessRule::Protected(NativeAccessRuleNode::AnyOf(vec![
                    rule1.clone(),
                    rule2.clone(),
                ]))
            }
            (NativeAccessRule::DenyAll, r @ NativeAccessRule::Protected(_))
            | (r @ NativeAccessRule::Protected(_), NativeAccessRule::DenyAll) => r.clone(),
            (NativeAccessRule::DenyAll, NativeAccessRule::DenyAll) => NativeAccessRule::DenyAll,
        };
        Arc::new(AccessRule(access_rule))
    }

    pub fn and(&self, other: Arc<Self>) -> Arc<Self> {
        let access_rule = match (&self.0, &other.0) {
            (NativeAccessRule::AllowAll, NativeAccessRule::AllowAll) => NativeAccessRule::AllowAll,
            (NativeAccessRule::AllowAll, r @ NativeAccessRule::Protected(_))
            | (r @ NativeAccessRule::Protected(_), NativeAccessRule::AllowAll) => r.clone(),
            (NativeAccessRule::Protected(rule1), NativeAccessRule::Protected(rule2)) => {
                NativeAccessRule::Protected(NativeAccessRuleNode::AllOf(vec![
                    rule1.clone(),
                    rule2.clone(),
                ]))
            }
            (NativeAccessRule::DenyAll, _) | (_, NativeAccessRule::DenyAll) => {
                NativeAccessRule::DenyAll
            }
        };
        Arc::new(AccessRule(access_rule))
    }
}

#[derive(Clone, Debug, Enum)]
pub enum ResourceOrNonFungible {
    NonFungible { value: Arc<NonFungibleGlobalId> },
    Resource { value: Arc<Address> },
}

#[derive(Clone, Debug, Enum)]
pub enum OwnerRole {
    None,
    Fixed { value: Arc<AccessRule> },
    Updatable { value: Arc<AccessRule> },
}

impl ToNative for ResourceOrNonFungible {
    type Native = NativeResourceOrNonFungible;

    fn to_native(self) -> Result<Self::Native> {
        match self {
            // Self::Resource { value } => value.try_into().map(Self::Native::Resource),
            Self::Resource { value } => (*value).try_into().map(Self::Native::Resource),
            Self::NonFungible { value } => Ok(Self::Native::NonFungible(value.0.clone())),
        }
    }
}

impl FromNativeWithNetworkContext for ResourceOrNonFungible {
    type Native = NativeResourceOrNonFungible;

    fn from_native(native: Self::Native, network_id: u8) -> Self {
        match native {
            NativeResourceOrNonFungible::Resource(resource_address) => Self::Resource {
                value: Arc::new(Address::from_typed_node_id(resource_address, network_id)),
            },
            NativeResourceOrNonFungible::NonFungible(non_fungible_global_id) => Self::NonFungible {
                value: Arc::new(NonFungibleGlobalId(non_fungible_global_id, network_id)),
            },
        }
    }
}

impl ToNative for OwnerRole {
    type Native = NativeOwnerRole;

    fn to_native(self) -> Result<Self::Native> {
        match self {
            Self::None => Ok(Self::Native::None),
            Self::Fixed { value } => Ok(Self::Native::Fixed(value.0.clone())),
            Self::Updatable { value } => Ok(Self::Native::Updatable(value.0.clone())),
        }
    }
}
