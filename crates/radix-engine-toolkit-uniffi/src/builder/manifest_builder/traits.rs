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

use crate::prelude::*;

pub trait FromWithNameRecordContext<T>
where
    Self: Sized,
{
    fn from(item: T, name_record: &NameRecord) -> Result<Self>;
}

impl<A, B> FromWithNameRecordContext<Option<A>> for Option<B>
where
    B: FromWithNameRecordContext<A>,
{
    fn from(item: Option<A>, name_record: &NameRecord) -> Result<Option<B>> {
        match item {
            Some(item) => Ok(Some(<B as FromWithNameRecordContext<A>>::from(
                item,
                name_record,
            )?)),
            None => Ok(None),
        }
    }
}

impl<A, B> FromWithNameRecordContext<Vec<A>> for Vec<B>
where
    B: FromWithNameRecordContext<A>,
{
    fn from(item: Vec<A>, name_record: &NameRecord) -> Result<Self> {
        item.into_iter()
            .map(|item| {
                <B as FromWithNameRecordContext<A>>::from(item, name_record)
            })
            .collect::<Result<_>>()
    }
}

impl<A, B> FromWithNameRecordContext<Vec<A>> for engine::IndexSet<B>
where
    B: FromWithNameRecordContext<A> + std::hash::Hash + Eq,
{
    fn from(item: Vec<A>, name_record: &NameRecord) -> Result<Self> {
        item.into_iter()
            .map(|item| {
                <B as FromWithNameRecordContext<A>>::from(item, name_record)
            })
            .collect::<Result<_>>()
    }
}

impl<A, B> FromWithNameRecordContext<engine::IndexSet<A>> for Vec<B>
where
    B: FromWithNameRecordContext<A>,
{
    fn from(
        item: engine::IndexSet<A>,
        name_record: &NameRecord,
    ) -> Result<Self> {
        item.into_iter()
            .map(|item| {
                <B as FromWithNameRecordContext<A>>::from(item, name_record)
            })
            .collect::<Result<_>>()
    }
}

impl<K1, K2, V1, V2> FromWithNameRecordContext<HashMap<K1, V1>>
    for HashMap<K2, V2>
where
    K2: FromWithNameRecordContext<K1> + std::hash::Hash + Eq,
    V2: FromWithNameRecordContext<V1>,
{
    fn from(item: HashMap<K1, V1>, name_record: &NameRecord) -> Result<Self> {
        item.into_iter()
            .map(|(key, value)| {
                Ok((
                    <K2 as FromWithNameRecordContext<K1>>::from(
                        key,
                        name_record,
                    )?,
                    <V2 as FromWithNameRecordContext<V1>>::from(
                        value,
                        name_record,
                    )?,
                ))
            })
            .collect::<Result<_>>()
    }
}

impl<K1, K2, V1, V2> FromWithNameRecordContext<HashMap<K1, V1>>
    for IndexMap<K2, V2>
where
    K2: FromWithNameRecordContext<K1> + std::hash::Hash + Eq,
    V2: FromWithNameRecordContext<V1>,
{
    fn from(item: HashMap<K1, V1>, name_record: &NameRecord) -> Result<Self> {
        item.into_iter()
            .map(|(key, value)| {
                Ok((
                    <K2 as FromWithNameRecordContext<K1>>::from(
                        key,
                        name_record,
                    )?,
                    <V2 as FromWithNameRecordContext<V1>>::from(
                        value,
                        name_record,
                    )?,
                ))
            })
            .collect::<Result<_>>()
    }
}

impl<K1, K2, V1, V2> FromWithNameRecordContext<IndexMap<K1, V1>>
    for HashMap<K2, V2>
where
    K2: FromWithNameRecordContext<K1> + std::hash::Hash + Eq,
    V2: FromWithNameRecordContext<V1>,
{
    fn from(item: IndexMap<K1, V1>, name_record: &NameRecord) -> Result<Self> {
        item.into_iter()
            .map(|(key, value)| {
                Ok((
                    <K2 as FromWithNameRecordContext<K1>>::from(
                        key,
                        name_record,
                    )?,
                    <V2 as FromWithNameRecordContext<V1>>::from(
                        value,
                        name_record,
                    )?,
                ))
            })
            .collect::<Result<_>>()
    }
}

impl<K1, K2, V1, V2> FromWithNameRecordContext<HashMap<K1, V1>>
    for BTreeMap<K2, V2>
where
    K2: FromWithNameRecordContext<K1> + Ord,
    V2: FromWithNameRecordContext<V1>,
{
    fn from(item: HashMap<K1, V1>, name_record: &NameRecord) -> Result<Self> {
        item.into_iter()
            .map(|(key, value)| {
                Ok((
                    <K2 as FromWithNameRecordContext<K1>>::from(
                        key,
                        name_record,
                    )?,
                    <V2 as FromWithNameRecordContext<V1>>::from(
                        value,
                        name_record,
                    )?,
                ))
            })
            .collect::<Result<_>>()
    }
}

impl<K1, K2, V1, V2> FromWithNameRecordContext<BTreeMap<K1, V1>>
    for HashMap<K2, V2>
where
    K2: FromWithNameRecordContext<K1> + Ord + std::hash::Hash,
    V2: FromWithNameRecordContext<V1>,
{
    fn from(item: BTreeMap<K1, V1>, name_record: &NameRecord) -> Result<Self> {
        item.into_iter()
            .map(|(key, value)| {
                Ok((
                    <K2 as FromWithNameRecordContext<K1>>::from(
                        key,
                        name_record,
                    )?,
                    <V2 as FromWithNameRecordContext<V1>>::from(
                        value,
                        name_record,
                    )?,
                ))
            })
            .collect::<Result<_>>()
    }
}

macro_rules! impl_from_with_name_record_for_address_from_arc_address {
    (
        $( $type: ty ),* $(,)?
    ) => {
        $(
            impl FromWithNameRecordContext<Arc<Address>> for $type {
                fn from(item: Arc<Address>, _: &NameRecord) -> Result<Self> {
                    Self::try_from(*item)
                }
            }
        )*
    };
}
impl_from_with_name_record_for_address_from_arc_address![
    engine::GlobalAddress,
    engine::InternalAddress,
    engine::ResourceAddress,
    engine::ComponentAddress,
    engine::PackageAddress,
    engine::DynamicGlobalAddress,
    engine::DynamicResourceAddress,
    engine::DynamicComponentAddress,
    engine::DynamicPackageAddress,
];

macro_rules! impl_from_with_name_record_for_address_from_string {
    (
        $( $type: ty ),* $(,)?
    ) => {
        $(
            impl FromWithNameRecordContext<String> for $type {
                fn from(item: String, _: &NameRecord) -> Result<Self> {
                    $crate::common::address::Address::new(item)
                        .and_then(|value| (*value).try_into())
                }
            }
        )*
    };
}
impl_from_with_name_record_for_address_from_string![
    engine::GlobalAddress,
    engine::InternalAddress,
    engine::ResourceAddress,
    engine::ComponentAddress,
    engine::PackageAddress,
    engine::DynamicGlobalAddress,
    engine::DynamicResourceAddress,
    engine::DynamicComponentAddress,
    engine::DynamicPackageAddress,
];

impl FromWithNameRecordContext<ManifestBuilderBucket>
    for engine::ManifestBucket
{
    fn from(
        item: ManifestBuilderBucket,
        name_record: &NameRecord,
    ) -> Result<Self> {
        item.to_native(name_record)
    }
}

impl FromWithNameRecordContext<ManifestBuilderProof> for engine::ManifestProof {
    fn from(
        item: ManifestBuilderProof,
        name_record: &NameRecord,
    ) -> Result<Self> {
        item.to_native(name_record)
    }
}

impl FromWithNameRecordContext<ManifestBuilderAddressReservation>
    for engine::ManifestAddressReservation
{
    fn from(
        item: ManifestBuilderAddressReservation,
        name_record: &NameRecord,
    ) -> Result<Self> {
        item.to_native(name_record)
    }
}

impl FromWithNameRecordContext<ManifestBuilderNamedAddress>
    for engine::ManifestNamedAddress
{
    fn from(
        item: ManifestBuilderNamedAddress,
        name_record: &NameRecord,
    ) -> Result<Self> {
        item.to_native(name_record)
    }
}

impl FromWithNameRecordContext<Arc<Decimal>> for engine::Decimal {
    fn from(item: Arc<Decimal>, _: &NameRecord) -> Result<Self> {
        Ok(item.0)
    }
}

impl FromWithNameRecordContext<Arc<PreciseDecimal>> for engine::PreciseDecimal {
    fn from(item: Arc<PreciseDecimal>, _: &NameRecord) -> Result<Self> {
        Ok(item.0)
    }
}

impl FromWithNameRecordContext<Arc<AccessRule>> for engine::AccessRule {
    fn from(item: Arc<AccessRule>, _: &NameRecord) -> Result<Self> {
        Ok(item.0.clone())
    }
}

impl FromWithNameRecordContext<AccountDefaultDepositRule>
    for engine::DefaultDepositRule
{
    fn from(item: AccountDefaultDepositRule, _: &NameRecord) -> Result<Self> {
        item.to_native()
    }
}

impl FromWithNameRecordContext<ResourcePreference>
    for engine::ResourcePreference
{
    fn from(item: ResourcePreference, _: &NameRecord) -> Result<Self> {
        item.to_native()
    }
}

impl FromWithNameRecordContext<ResourceOrNonFungible>
    for engine::ResourceOrNonFungible
{
    fn from(item: ResourceOrNonFungible, _: &NameRecord) -> Result<Self> {
        item.to_native()
    }
}

impl FromWithNameRecordContext<ResourceOrNonFungible>
    for engine::ManifestResourceOrNonFungible
{
    fn from(item: ResourceOrNonFungible, _: &NameRecord) -> Result<Self> {
        match item {
            ResourceOrNonFungible::Resource { value } => (*value)
                .try_into()
                .map(engine::ManifestResourceOrNonFungible::Resource),
            ResourceOrNonFungible::NonFungible { value } => {
                Ok(engine::ManifestResourceOrNonFungible::NonFungible(
                    value.0.clone(),
                ))
            }
        }
    }
}

impl FromWithNameRecordContext<OwnerRole> for engine::OwnerRole {
    fn from(item: OwnerRole, _: &NameRecord) -> Result<Self> {
        item.to_native()
    }
}

impl FromWithNameRecordContext<NonFungibleLocalId>
    for engine::NonFungibleLocalId
{
    fn from(item: NonFungibleLocalId, _: &NameRecord) -> Result<Self> {
        engine::NonFungibleLocalId::try_from(item)
    }
}

impl FromWithNameRecordContext<NonFungibleGlobalId>
    for engine::NonFungibleGlobalId
{
    fn from(item: NonFungibleGlobalId, _: &NameRecord) -> Result<Self> {
        Ok(item.0)
    }
}

impl FromWithNameRecordContext<MetadataValue> for engine::MetadataValue {
    fn from(item: MetadataValue, _: &NameRecord) -> Result<Self> {
        item.to_native()
    }
}

impl FromWithNameRecordContext<String> for engine::RoleKey {
    fn from(item: String, _: &NameRecord) -> Result<Self> {
        Ok(engine::RoleKey::new(item))
    }
}

impl FromWithNameRecordContext<ModuleId> for engine::ObjectModuleId {
    fn from(item: ModuleId, _: &NameRecord) -> Result<Self> {
        Ok(item.into())
    }
}

impl FromWithNameRecordContext<RoyaltyAmount> for engine::RoyaltyAmount {
    fn from(item: RoyaltyAmount, _: &NameRecord) -> Result<Self> {
        Ok(item.into())
    }
}

impl FromWithNameRecordContext<RuleSet> for engine::RuleSet {
    fn from(item: RuleSet, _: &NameRecord) -> Result<Self> {
        item.to_native()
    }
}

impl FromWithNameRecordContext<WithdrawStrategy> for engine::WithdrawStrategy {
    fn from(item: WithdrawStrategy, _: &NameRecord) -> Result<Self> {
        Ok(item.into())
    }
}

impl FromWithNameRecordContext<Vec<ManifestBuilderBucket>>
    for engine::ManifestBucketBatch
{
    fn from(
        item: Vec<ManifestBuilderBucket>,
        name_record: &NameRecord,
    ) -> Result<Self> {
        item.into_iter()
            .map(|value| {
                FromWithNameRecordContext::<ManifestBuilderBucket>::from(
                    value,
                    name_record,
                )
            })
            .collect::<Result<_>>()
            .map(Self::ManifestBuckets)
    }
}

impl FromWithNameRecordContext<PublicKey> for engine::Secp256k1PublicKey {
    fn from(item: PublicKey, _: &NameRecord) -> Result<Self> {
        if let engine::PublicKey::Secp256k1(public_key) =
            engine::PublicKey::try_from(item)?
        {
            Ok(public_key)
        } else {
            Err(RadixEngineToolkitError::InvalidPublicKey)
        }
    }
}

impl FromWithNameRecordContext<ResourceSpecifier>
    for radix_engine_interface::blueprints::locker::ResourceSpecifier
{
    fn from(item: ResourceSpecifier, name_record: &NameRecord) -> Result<Self> {
        match item {
            ResourceSpecifier::Amount {
                resource_address: _,
                amount,
            } => Ok(radix_engine_interface::blueprints::locker::ResourceSpecifier::Fungible(amount.0)),
            ResourceSpecifier::Ids {
                resource_address: _,
                ids,
            } => Ok(radix_engine_interface::blueprints::locker::ResourceSpecifier::NonFungible(
                FromWithNameRecordContext::<Vec<_>>::from(ids, name_record)?,
            )),
        }
    }
}

impl<A, B> FromWithNameRecordContext<Vec<A>> for (B, B)
where
    B: FromWithNameRecordContext<A>,
{
    fn from(mut item: Vec<A>, name_record: &NameRecord) -> Result<Self> {
        let len = item.len();
        if len == 2 {
            let item2 = item.pop().unwrap();
            let item1 = item.pop().unwrap();
            Ok((B::from(item1, name_record)?, B::from(item2, name_record)?))
        } else {
            Err(RadixEngineToolkitError::InvalidLength {
                expected: 2,
                actual: len as u64,
                data: vec![],
            })
        }
    }
}

macro_rules! impl_same_from_and_to {
    (
        $(
            $ty: ty
        ),* $(,)?
    ) => {
        $(
            impl FromWithNameRecordContext<$ty> for $ty {
                fn from(item: $ty, _: &NameRecord) -> Result<Self> {
                    Ok(item)
                }
            }
        )*
    };
}

impl_same_from_and_to! {
    bool,
    u8,
    u16,
    u32,
    u64,
    u128,
    i8,
    i16,
    i32,
    i64,
    i128,
    String,
    ()
}

macro_rules! impl_tuple {
    (
        $(
            $ident: tt
        ),+ $(,)?
    ) => {
        paste::paste! {
            impl<
                $(
                    [< A $ident >],[< B $ident >]
                ),+
            > FromWithNameRecordContext<( $([< A $ident >],)+ )> for ( $([< B $ident >],)+ )
            where
                $(
                    [< B $ident >]: FromWithNameRecordContext<[< A $ident >]>
                ),+
            {
                fn from(($([< a $ident >],)+): ($([< A $ident >],)+), name_record: &NameRecord) -> Result<Self> {
                    Ok((
                        $(
                            <[< B $ident >] as FromWithNameRecordContext<[< A $ident >]>>::from([< a $ident >],name_record)?,
                        )+
                    ))
                }
            }
        }
    };
}

impl_tuple! { 1 }
impl_tuple! { 1, 2 }
impl_tuple! { 1, 2, 3 }
impl_tuple! { 1, 2, 3, 4 }
impl_tuple! { 1, 2, 3, 4, 5 }
impl_tuple! { 1, 2, 3, 4, 5, 6 }
impl_tuple! { 1, 2, 3, 4, 5, 6, 7 }
impl_tuple! { 1, 2, 3, 4, 5, 6, 7, 8 }
impl_tuple! { 1, 2, 3, 4, 5, 6, 7, 8, 9 }
impl_tuple! { 1, 2, 3, 4, 5, 6, 7, 8, 9, 10 }
impl_tuple! { 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11 }
impl_tuple! { 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, }
impl_tuple! { 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13 }
