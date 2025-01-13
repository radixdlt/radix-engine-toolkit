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

use crate::internal_prelude::*;

/// A type representing the full set of invocation IOs for all instructions in
/// a manifest.
pub struct InstructionIndexedInvocationIo<'a>(
    IndexMap<&'a InstructionIndex, DefaultInvocationIo<'a>>,
);

impl<'a> InstructionIndexedInvocationIo<'a> {
    pub fn combine(
        static_information: &'a IndexMap<
            InstructionIndex,
            InvocationIo<TrackedResources>,
        >,
        dynamic_information: Option<
            &'a IndexMap<
                InstructionIndex,
                InvocationIo<
                    IndexMap<
                        &'a ResourceAddress,
                        Vec<Tracked<ResourceQuantifier<'a>>>,
                    >,
                >,
            >,
        >,
    ) -> Self {
        static STATIC_INVOCATION_IO: OnceLock<InvocationIo<TrackedResources>> =
            OnceLock::new();

        let inner_map = match dynamic_information {
            Some(dynamic_information) => dynamic_information
                .keys()
                .chain(static_information.keys())
                .unique()
                .map(|instruction_index| {
                    let static_information = static_information.get(instruction_index);
                    let dynamic_information = dynamic_information.get(instruction_index);

                    let invocation_io = match (static_information, dynamic_information) {
                        (Some(static_information), dynamic_information) => {
                            DefaultInvocationIo::combine(
                                static_information,
                                dynamic_information,
                            )
                        }
                        (None, dynamic_information) => DefaultInvocationIo::combine(
                            STATIC_INVOCATION_IO.get_or_init(|| {
                                InvocationIo::from_fn(|_| TrackedResources::new_empty())
                            }),
                            dynamic_information,
                        ),
                    };

                    (instruction_index, invocation_io)
                })
                .collect(),
            None => static_information
                .iter()
                .map(|(instruction_index, invocation_io)| {
                    (
                        instruction_index,
                        InvocationIo::<ResourceIndexedInvocationIo<'a>>::combine(
                            invocation_io,
                            None,
                        ),
                    )
                })
                .collect(),
        };
        Self(inner_map)
    }

    pub fn for_instruction(
        &'a self,
        instruction_index: &'a InstructionIndex,
    ) -> &'a DefaultInvocationIo<'a> {
        static STATIC_DEFAULT_INVOCATION_IO: OnceLock<
            DefaultInvocationIo<'static>,
        > = OnceLock::new();

        self.0.get(instruction_index).unwrap_or(
            STATIC_DEFAULT_INVOCATION_IO.get_or_init(Default::default),
        )
    }
}

pub type DefaultInvocationIo<'a> =
    InvocationIo<ResourceIndexedInvocationIo<'a>>;

structured_map! {
    /// A generic structure for the inputs and outputs of invocations.
    ///
    /// This is a structure used to define the inputs and outputs of invocations
    /// where they are generic.
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub InvocationIo => [input, output]
}

impl<'a> DefaultInvocationIo<'a> {
    fn combine(
        static_information: &'a InvocationIo<TrackedResources>,
        dynamic_information: Option<
            &'a InvocationIo<
                IndexMap<
                    &'a ResourceAddress,
                    Vec<Tracked<ResourceQuantifier<'a>>>,
                >,
            >,
        >,
    ) -> Self {
        match dynamic_information {
            Some(dynamic_information) => static_information
                .as_ref()
                .zip(dynamic_information.as_ref())
                .map(|(static_information, dynamic_information)| {
                    ResourceIndexedInvocationIo::combine(
                        static_information,
                        Some(dynamic_information),
                    )
                }),
            None => static_information.as_ref().map(|static_information| {
                ResourceIndexedInvocationIo::combine(static_information, None)
            }),
        }
    }
}

impl<'a> From<&'a InvocationStaticInformation>
    for InvocationIo<&'a TrackedResources>
{
    fn from(
        InvocationStaticInformation { input, output, .. }: &'a InvocationStaticInformation,
    ) -> Self {
        Self { input, output }
    }
}

/// A type representing the complete set of either inputs or outputs from an
/// invocation.
///
/// This type is indexed by the [`ResourceAddress`] of assets. Meaning that the
/// [`InvocationIoItem`]s stored in this type are stored in a map type where the
/// key is the [`ResourceAddress`] and the value is the [`InvocationIoItem`] of
/// this particular resource.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct ResourceIndexedInvocationIo<'a>(
    IndexMap<&'a ResourceAddress, InvocationIoItem<'a>>,
);

impl<'a> ResourceIndexedInvocationIo<'a> {
    pub fn combine(
        static_information: &'a TrackedResources,
        dynamic_information: Option<
            &'a IndexMap<
                &'a ResourceAddress,
                Vec<Tracked<ResourceQuantifier<'a>>>,
            >,
        >,
    ) -> Self {
        let static_information = static_information.specified_resources();
        let inner_map = match dynamic_information {
            Some(dynamic_information) => dynamic_information
                .keys()
                .copied()
                .chain(static_information.keys())
                .unique()
                .map(|resource_address| {
                    let static_information =
                        static_information.get(resource_address);
                    let dynamic_information =
                        dynamic_information.get(resource_address);

                    let invocation_io_item = match (
                        static_information,
                        dynamic_information,
                    ) {
                        (
                            Some(static_information),
                            Some(dynamic_information),
                        ) => InvocationIoItem::StaticAndDynamic {
                            static_information: static_information.bounds(),
                            dynamic_information,
                        },
                        (Some(static_information), None) => {
                            InvocationIoItem::Static(
                                static_information.bounds(),
                            )
                        }
                        (None, Some(dynamic_information)) => {
                            InvocationIoItem::Dynamic(dynamic_information)
                        }
                        (None, None) => unreachable!(concat!(
                            "If neither static nor dynamic info are defined ",
                            "then how did we get this resource address?",
                        )),
                    };

                    (resource_address, invocation_io_item)
                })
                .collect(),
            None => static_information
                .iter()
                .map(|(resource_address, tracked_resource)| {
                    (
                        resource_address,
                        InvocationIoItem::Static(tracked_resource.bounds()),
                    )
                })
                .collect(),
        };
        Self(inner_map)
    }

    pub fn for_resource(
        &self,
        resource_address: &ResourceAddress,
    ) -> Option<&InvocationIoItem<'a>> {
        self.0.get(resource_address)
    }

    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (&ResourceAddress, &InvocationIoItem<'_>)> {
        self.0.iter().map(|(k, v)| (*k, v))
    }
}

/// This enum defines the IO of some invocation for some resource.
///
/// Typically, this type will be used as a value in a map where the key is the
/// resource address and the value is this type.
///
/// This type captures what kind of information we have about a resource and
/// whether this information comes from static analysis, dynamic analysis, or
/// both types of analysis.
///
/// This type stores just references to the underlying types instead of storing
/// owned values to improve the performance of the invocation io logic.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InvocationIoItem<'a> {
    /// A variant for when the information about the resource invocation IO is
    /// obtained purely statically from the static analyzer. This can happen if
    /// the parts of the code that construct this enum are not given enough info
    /// to conduct dynamic analysis (e.g., they're not given the toolkit receipt
    /// that contains the worktop changes) and therefore they settle for doing
    /// everything statically.
    Static(&'a ResourceBounds),
    /// A variant for when the information about the resource invocation IO is
    /// obtained purely dynamically. This happens in a few cases where static
    /// analysis was unable to pick up on some resource (e.g. due to it being a
    /// call-method to a generic component) but the dynamic analyzer was able to
    /// pick up on the invocation IO that took place.
    Dynamic(&'a Vec<Tracked<ResourceQuantifier<'a>>>),
    /// A variant for when the information about the resource invocation IO is
    /// obtained from both the static and dynamic analyzer. This case happens
    /// quite often when both analyzers are available and a certain resource is
    /// seen by both.
    // TODO(invocation-io): Maybe I need to have a dedicated type for this case?
    // this will depend on what I find myself doing on this data and with this
    // information. So, before adding it I want to see what exactly I'm going to
    // need to use and what information I will need to extract.
    StaticAndDynamic {
        static_information: &'a ResourceBounds,
        dynamic_information: &'a Vec<Tracked<ResourceQuantifier<'a>>>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ResourceQuantifier<'a> {
    Amount(&'a Decimal),
    Ids(&'a IndexSet<NonFungibleLocalId>),
}

impl<'a> ResourceQuantifier<'a> {
    pub fn amount(&self) -> Decimal {
        match self {
            Self::Amount(amount) => **amount,
            Self::Ids(ids) => ids.len().into(),
        }
    }

    pub fn ids(&self) -> Option<&IndexSet<NonFungibleLocalId>> {
        if let Self::Ids(ids) = self {
            Some(ids)
        } else {
            None
        }
    }

    pub fn empty_static(is_fungible: bool) -> ResourceQuantifier<'static> {
        match is_fungible {
            true => Self::empty_static_amount(),
            false => Self::empty_static_ids(),
        }
    }

    pub fn empty_static_amount() -> ResourceQuantifier<'static> {
        ResourceQuantifier::Amount(&Decimal::ZERO)
    }

    pub fn empty_static_ids() -> ResourceQuantifier<'static> {
        static STATIC_IDS: OnceLock<IndexSet<NonFungibleLocalId>> =
            OnceLock::new();
        ResourceQuantifier::Ids(STATIC_IDS.get_or_init(Default::default))
    }
}

impl<'a> From<&'a ResourceSpecifier> for ResourceQuantifier<'a> {
    fn from(value: &'a ResourceSpecifier) -> Self {
        match value {
            ResourceSpecifier::Amount(_, amount) => Self::Amount(amount),
            ResourceSpecifier::Ids(_, ids) => Self::Ids(ids),
        }
    }
}

/// A data structure used to store tracked data.
///
/// This data structure stores [`Tracked`] data of a generic type [`T`] which
/// can be any type and has no bounds. The index of the instruction when the
/// data was created or obtained from is stored in the [`created_at`] field in
/// this struct as an [`InstructionIndex`].
///
/// [`created_at`]: Tracked::created_at
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Tracked<T> {
    /// The value of the tracked data.
    pub value: T,

    /// The index of the instruction that the information was obtained from.
    /// This is most commonly useful when adding assertions to the manifest and
    /// needing to know where to add them in the case of non-guaranteed deposits
    /// in the manifest.
    pub created_at: InstructionIndex,
}

impl<T> Tracked<T> {
    pub fn new(value: T, created_at: InstructionIndex) -> Self {
        Self { value, created_at }
    }

    pub fn as_ref(&self) -> Tracked<&T> {
        Tracked {
            value: &self.value,
            created_at: self.created_at,
        }
    }

    pub fn as_mut(&mut self) -> Tracked<&mut T> {
        Tracked {
            value: &mut self.value,
            created_at: self.created_at,
        }
    }

    pub fn map<O>(self, mut f: impl FnMut(T) -> O) -> Tracked<O> {
        Tracked {
            value: f(self.value),
            created_at: self.created_at,
        }
    }
}

impl<T> Tracked<&T>
where
    T: Copy,
{
    pub fn copied(&self) -> Tracked<T> {
        Tracked {
            value: *self.value,
            created_at: self.created_at,
        }
    }
}

impl<T> Tracked<&T>
where
    T: Clone,
{
    pub fn cloned(&self) -> Tracked<T> {
        Tracked {
            value: self.value.clone(),
            created_at: self.created_at,
        }
    }
}

impl<T> AsRef<T> for Tracked<T> {
    fn as_ref(&self) -> &T {
        &self.value
    }
}

impl<T> AsMut<T> for Tracked<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

impl<T> Deref for Tracked<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for Tracked<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}
