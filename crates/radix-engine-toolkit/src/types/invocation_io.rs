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

//! This module contains the invocation IO types which, as the name implies, are
//! the inputs and outputs of invocations. This module contains the logic for
//! combining the static and dynamic analysis data on the invocation inputs into
//! a single object that's understood by the toolkit's internal logic.
//!
//! The "entry point" so to speak into this module is the [`compute`] function
//! on the [`IndexedInvocationIo`] type which is used to compute the composite
//! static and dynamic invocation IOs from the manifest (used in static
//! analysis) and the dynamic invocation inputs obtained from the receipt from
//! the worktop changes.
//!
//! [`compute`]: IndexedInvocationIo::compute

use crate::internal_prelude::*;
use EitherGuaranteedOrPredicted::{Guaranteed, Predicted};

/// A type that stores the indexed invocation inputs and outputs where they're
/// indexed by the [`InstructionIndex`] of the invocation.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct IndexedInvocationIo(
    IndexMap<InstructionIndex, InvocationIo<InvocationIoItems>>,
);

impl IndexedInvocationIo {
    pub fn empty() -> Self {
        Default::default()
    }

    pub fn for_instruction(
        &self,
        instruction_index: &InstructionIndex,
    ) -> Option<&InvocationIo<InvocationIoItems>> {
        self.0.get(instruction_index)
    }

    pub fn add(
        &mut self,
        instruction_index: InstructionIndex,
        InvocationIo {
            input: other_input,
            output: other_output,
        }: InvocationIo<InvocationIoItems>,
    ) {
        let InvocationIo { input, output } =
            self.0.entry(instruction_index).or_default();
        input.combine(other_input);
        output.combine(other_output);
    }

    pub fn compute(
        manifest: &impl ReadableManifest,
        worktop_changes: &WorktopChanges<'_>,
    ) -> Result<Self, Error> {
        let mut static_analysis =
            StaticAnalysisInvocationIo::compute(manifest)?;
        let mut dynamic_analysis =
            DynamicAnalysisInvocationIo::compute(manifest, &worktop_changes);

        let instruction_indices = static_analysis
            .instruction_index_iter()
            .chain(dynamic_analysis.instruction_index_iter())
            .copied()
            .collect::<BTreeSet<_>>();

        let mut this = Self::default();
        for instruction_index in instruction_indices {
            let static_analysis = static_analysis.remove(&instruction_index);
            let dynamic_analysis = dynamic_analysis.remove(&instruction_index);

            let invocation_io = match static_analysis {
                Some(static_analysis) => static_analysis
                    .zip(dynamic_analysis)
                    .map(|(static_analysis, dynamic_analysis)| {
                        let dynamic_analysis = dynamic_analysis
                            .into_iter()
                            .map(|value| value.map(Cow::into_owned))
                            .collect::<Vec<_>>();
                        InvocationIoItems::new_from_invocation_static_and_dynamic_information(
                            static_analysis,
                            dynamic_analysis,
                        )
                    })
                    .swap_result()?,
                // No static analysis is available for this invocation and only
                // dynamic analysis is available. Therefore, we take everything
                // in the dynamic analysis as the invocation io.
                None => dynamic_analysis.map(|io| {
                    io.into_iter()
                        .map(|value| value.map(Cow::into_owned))
                        .fold(InvocationIoItems::empty(), |acc, item| {
                            acc.with_predicted_tracked_resource_specifier(item)
                        })
                }),
            };
            this.add(instruction_index, invocation_io);
        }

        Ok(this)
    }
}

/// A type representing the input and output into and from an invocation where
/// both input and output are represented through [`T`].
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct InvocationIo<T> {
    pub input: T,
    pub output: T,
}

impl<T> InvocationIo<T> {
    pub fn as_ref(&self) -> InvocationIo<&T> {
        InvocationIo {
            input: &self.input,
            output: &self.input,
        }
    }

    pub fn map<O>(self, mut f: impl FnMut(T) -> O) -> InvocationIo<O> {
        InvocationIo {
            input: f(self.input),
            output: f(self.output),
        }
    }

    pub fn zip<O>(self, other: InvocationIo<O>) -> InvocationIo<(T, O)> {
        InvocationIo {
            input: (self.input, other.input),
            output: (self.output, other.output),
        }
    }
}

impl<O, E> InvocationIo<Result<O, E>> {
    pub fn swap_result(self) -> Result<InvocationIo<O>, E> {
        let Self { input, output } = self;
        match (input, output) {
            (Ok(input), Ok(output)) => Ok(InvocationIo { input, output }),
            (Err(err), _) | (_, Err(err)) => Err(err),
        }
    }
}

impl<T> InvocationIo<T>
where
    T: Default,
{
    pub fn empty() -> Self {
        Default::default()
    }
}

/// Represents the partial or complete set of inputs into some invocation.
///
/// This type is used to represent either the complete or partial set of inputs
/// into some invocation. Its underlying type is a vector of [`InvocationIoItem`]
/// objects.
///
/// Keep in mind that [`InvocationIoItem`]s can not be combined together which
/// means that this object can have multiple [`InvocationIoItem`]s for the same
/// resource.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct InvocationIoItems(Vec<InvocationIoItem>);

impl InvocationIoItems {
    pub fn empty() -> Self {
        Default::default()
    }

    pub fn empty_static() -> &'static Self {
        use std::sync::OnceLock;
        static EMPTY: OnceLock<InvocationIoItems> = OnceLock::new();
        EMPTY.get_or_init(Default::default)
    }

    /// Combines the static and dynamic information into [`InvocationIoItems`].
    ///
    /// This function combines the static and dynamic information obtained from
    /// the two analyzers, respectively, into a single [`InvocationIoItems`]
    /// object that can be understood by the toolkit's internal systems. This
    /// function acts on a single invocation and multiple resources. Other types
    /// can be used for multiple invocations.
    pub fn new_from_invocation_static_and_dynamic_information(
        static_analysis: TrackedResources,
        dynamic_analysis: Vec<Tracked<ResourceSpecifier>>,
    ) -> Result<Self, Error> {
        // The dynamic analysis invocation inputs are hard to analyze in this
        // shape. So, we transform the vector into a map keyed by the resource
        // address to make it easier to analyze and to make it have a similar
        // shape to the invocation inputs from the static analysis.
        let mut dynamic_analysis = dynamic_analysis.into_iter().fold(
            IndexMap::<_, Vec<_>>::new(),
            |mut map, resource_specifier| {
                let resource_address = resource_specifier.resource_address();
                map.entry(*resource_address)
                    .or_default()
                    .push(resource_specifier);
                map
            },
        );
        let (mut static_analysis, _) = static_analysis.deconstruct();

        // Getting a collection of the resource addresses present in the static
        // and dynamic analysis invocation inputs. We will then use the resource
        // addresses to iterate over all of the invocation inputs.
        let resource_addresses = dynamic_analysis
            .keys()
            .chain(static_analysis.keys())
            .copied()
            .collect::<HashSet<_>>();

        let mut this = Self::default();
        for resource_address in resource_addresses.into_iter() {
            let static_information = static_analysis
                .swap_remove(&resource_address)
                .map(|tracked_resource| {
                    SimpleResourceBounds::from_bound(
                        resource_address,
                        tracked_resource.deconstruct().0,
                    )
                });
            let dynamic_information =
                dynamic_analysis.swap_remove(&resource_address);

            // The expect/unwrap here is safe to do. This is because the only
            // case where this can return `None` is when both static and dynamic
            // information are undefined.
            let other = Self::combine_resource_static_and_dynamic_information(
                resource_address,
                static_information,
                dynamic_information,
            )
            .map(|value| value.expect("This can't fail"))?;
            this.combine(other);
        }

        Ok(this)
    }

    /// Combines the static and dynamic information into [`InvocationIoItems`].
    ///
    /// Given the static and dynamic invocation IO of some resource, this
    /// function combines them into a single [`InvocationIoItems`] object that
    /// the toolkit's internal code is capable of understanding. To be clear,
    /// this function acts over a single invocation and a single resource. Other
    /// functions in this struct are capable of acting on a single invocation
    /// and multiple resources which is the more useful concept for clients.
    ///
    /// In the case of invocation inputs, the [`static_information`] argument is
    /// obtained from the static analyzer's output. The [`dynamic_information`]
    /// is obtained from the toolkit's receipt worktop changes section which is
    /// computed from the execution trace. We need to process the dynamic info
    /// first before consuming it so that we transform it from worktop changes
    /// and into invocation inputs.
    ///
    /// In the case of invocation outputs, the [`static_information`] argument
    /// is obtained from the static analyzer's output. [`dynamic_information`]
    /// is obtained from the toolkit's receipt and can be directly obtained from
    /// the worktop changes in the receipt without any need for additional
    /// processing.
    ///
    /// The logic that we follow for the combination is simple:
    /// * If the resource is only known about dynamically then we return the
    ///   dynamically known information as [`Predicted`].
    /// * If the resource is known about statically and dynamically then we run
    ///   some more analysis on what we have:
    ///     * If the simple resource bounds on the resource are exact then we
    ///       return them as [`Guaranteed`]
    ///     * Otherwise, we return them as [`Predicted`].
    ///
    /// This function returns an [`Err`] if static analysis is defined but the
    /// dynamic analysis is not. This case is considered to be an error by this
    /// function as it violates this function's assumption on dynamic and static
    /// analysis which is that the set of information known dynamically is a
    /// superset of those known statically. Therefore, since its a superset, we
    /// cant have something known statically but not dynamically.
    ///
    /// This function returns [`None`] if both the static and dynamic analysis
    /// are [`None`]. Otherwise, and aside from the case mentioned above, some
    /// thing should be returned from this function.
    ///
    /// [`new_from_invocation_static_and_dynamic_information`] differs from this
    /// function in that this function is used for a single resource from a
    /// single invocation whereas that function is used for the entire IO of the
    /// invocation.
    fn combine_resource_static_and_dynamic_information(
        resource_address: ResourceAddress,
        static_information: Option<SimpleResourceBounds>,
        dynamic_information: Option<Vec<Tracked<ResourceSpecifier>>>,
    ) -> Result<Option<Self>, InvocationIoError> {
        match (static_information, dynamic_information) {
            (
                Some(SimpleResourceBounds::Fungible(
                    SimpleFungibleResourceBounds::Exact(exact_amount),
                )),
                _,
            ) => Ok(Some(
                Self::empty()
                    .with_guaranteed_fungible(resource_address, exact_amount),
            )),
            (
                Some(SimpleResourceBounds::NonFungible(
                    SimpleNonFungibleResourceBounds::Exact {
                        certain_ids, ..
                    },
                )),
                _,
            ) => {
                Ok(Some(Self::empty().with_guaranteed_non_fungible(
                    resource_address,
                    certain_ids,
                )))
            }
            (
                Some(SimpleResourceBounds::Fungible(
                    SimpleFungibleResourceBounds::AtLeast(..)
                    | SimpleFungibleResourceBounds::AtMost(..)
                    | SimpleFungibleResourceBounds::Between(..)
                    | SimpleFungibleResourceBounds::UnknownAmount,
                )),
                Some(dynamic_information),
            )
            | (
                Some(SimpleResourceBounds::NonFungible(
                    SimpleNonFungibleResourceBounds::NotExact { .. },
                )),
                Some(dynamic_information),
            )
            | (None, Some(dynamic_information)) => {
                Ok(Some(dynamic_information.into_iter().fold(
                    Self::empty(),
                    |acc, specifier| {
                        acc.with_predicted_tracked_resource_specifier(specifier)
                    },
                )))
            }
            (Some(_), None) => {
                Err(Error::NoDynamicAnalysisWhenStaticAnalysisIsPresent)
            }
            (None, None) => Ok(None),
        }
    }

    pub fn with_guaranteed_fungible(
        mut self,
        resource_address: ResourceAddress,
        amount: Decimal,
    ) -> Self {
        self.add(InvocationIoItem::new_guaranteed_fungible(
            resource_address,
            amount,
        ));
        self
    }

    pub fn with_predicted_fungible(
        mut self,
        resource_address: ResourceAddress,
        amount: Decimal,
        created_at: InstructionIndex,
    ) -> Self {
        self.add(InvocationIoItem::new_predicted_fungible(
            resource_address,
            amount,
            created_at,
        ));
        self
    }

    pub fn with_guaranteed_non_fungible(
        mut self,
        resource_address: ResourceAddress,
        ids: impl IntoIterator<Item = NonFungibleLocalId>,
    ) -> Self {
        self.add(InvocationIoItem::new_guaranteed_non_fungible(
            resource_address,
            ids,
        ));
        self
    }

    pub fn with_predicted_non_fungible(
        mut self,
        resource_address: ResourceAddress,
        ids: impl IntoIterator<Item = NonFungibleLocalId>,
        created_at: InstructionIndex,
    ) -> Self {
        self.add(InvocationIoItem::new_predicted_non_fungible(
            resource_address,
            ids,
            created_at,
        ));
        self
    }

    pub fn with_predicted_resource_specifier(
        mut self,
        resource_specifier: ResourceSpecifier,
        created_at: InstructionIndex,
    ) -> Self {
        self.add(InvocationIoItem::new_predicted_resource_specifier(
            resource_specifier,
            created_at,
        ));
        self
    }

    pub fn with_predicted_tracked_resource_specifier(
        mut self,
        tracked_resource_specifier: Tracked<ResourceSpecifier>,
    ) -> Self {
        self.add(InvocationIoItem::new_predicted_tracked_resource_specifier(
            tracked_resource_specifier,
        ));
        self
    }

    pub fn add(&mut self, io: InvocationIoItem) {
        self.0.push(io)
    }

    pub fn combine(&mut self, other: Self) {
        self.0.extend(other)
    }

    pub fn extend(&mut self, ios: impl IntoIterator<Item = InvocationIoItem>) {
        self.0.extend(ios)
    }

    pub fn io_of_resource(
        &self,
        resource_address: ResourceAddress,
    ) -> impl Iterator<Item = &InvocationIoItem> {
        self.0
            .iter()
            .filter(move |io| *io.resource_address() == resource_address)
    }

    pub fn resource_amount(
        &self,
        resource_address: ResourceAddress,
    ) -> Decimal {
        self.io_of_resource(resource_address)
            .fold(Decimal::ZERO, |acc, io| acc + *io.amount())
    }

    pub fn resource_addresses(&self) -> IndexSet<ResourceAddress> {
        self.0
            .iter()
            .map(InvocationIoItem::resource_address)
            .copied()
            .collect()
    }

    pub fn as_slice(&self) -> &[InvocationIoItem] {
        &self.0
    }
}

impl IntoIterator for InvocationIoItems {
    type Item = <Vec<InvocationIoItem> as IntoIterator>::Item;
    type IntoIter = <Vec<InvocationIoItem> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FromIterator<InvocationIoItem> for InvocationIoItems {
    fn from_iter<T: IntoIterator<Item = InvocationIoItem>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

/// Represents a single input to or output from some invocation.
///
/// This type is used to represent a single IO of some invocation. it can either
/// be [`Fungible`] or [`NonFungible`]. In both cases, the enum takes a similar
/// shape where its a two-element tuple where the first item is the
/// [`ResourceAddress`] and the second is [`EitherGuaranteedOrPredicted`] of an
/// appropriate type which is [`Decimal`] for fungibles and in the case of non
/// fungibles it is [`IndexSet<NonFungibleLocalId>`].
///
/// Note that we can NOT combine multiple different [`InvocationIoItem`]s into a
/// single [`InvocationIoItem`] this means that an invocation might have
/// multiple invocation IOs of the same resource address. The reason for being
/// unable to combine multiple of them into a single one is due to [`Tracked`]
/// data not being possible to easily combine and therefore we say that this
/// type as a whole can't be combined.
///
/// [`Fungible`]: InvocationIoItem::Fungible
/// [`NonFungible`]: InvocationIoItem::NonFungible
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InvocationIoItem {
    Fungible(ResourceAddress, EitherGuaranteedOrPredicted<Decimal>),
    NonFungible(
        ResourceAddress,
        EitherGuaranteedOrPredicted<IndexSet<NonFungibleLocalId>>,
    ),
}

impl InvocationIoItem {
    pub fn new_predicted_tracked_resource_specifier(
        Tracked {
            value: resource_specifier,
            created_at,
        }: Tracked<ResourceSpecifier>,
    ) -> Self {
        Self::new_predicted_resource_specifier(resource_specifier, created_at)
    }

    pub fn new_predicted_resource_specifier(
        resource_specifier: ResourceSpecifier,
        created_at: InstructionIndex,
    ) -> Self {
        match resource_specifier {
            ResourceSpecifier::Amount(resource_address, amount) => {
                Self::new_predicted_fungible(
                    resource_address,
                    amount,
                    created_at,
                )
            }
            ResourceSpecifier::Ids(resource_address, ids) => {
                Self::new_predicted_non_fungible(
                    resource_address,
                    ids,
                    created_at,
                )
            }
        }
    }

    pub fn new_guaranteed_fungible(
        resource_address: ResourceAddress,
        amount: Decimal,
    ) -> Self {
        Self::Fungible(resource_address, Guaranteed(amount))
    }

    pub fn new_predicted_fungible(
        resource_address: ResourceAddress,
        amount: Decimal,
        created_at: InstructionIndex,
    ) -> Self {
        Self::Fungible(
            resource_address,
            Predicted(Tracked::new(amount, created_at)),
        )
    }

    pub fn new_guaranteed_non_fungible(
        resource_address: ResourceAddress,
        ids: impl IntoIterator<Item = NonFungibleLocalId>,
    ) -> Self {
        Self::NonFungible(
            resource_address,
            Guaranteed(ids.into_iter().collect()),
        )
    }

    pub fn new_predicted_non_fungible(
        resource_address: ResourceAddress,
        ids: impl IntoIterator<Item = NonFungibleLocalId>,
        created_at: InstructionIndex,
    ) -> Self {
        Self::NonFungible(
            resource_address,
            Predicted(Tracked::new(ids.into_iter().collect(), created_at)),
        )
    }

    pub fn resource_address(&self) -> &ResourceAddress {
        match self {
            Self::Fungible(v, ..) | Self::NonFungible(v, ..) => v,
        }
    }

    pub fn amount(&self) -> EitherGuaranteedOrPredicted<Decimal> {
        match self {
            Self::Fungible(_, amount) => *amount,
            Self::NonFungible(_, ids) => {
                ids.as_ref().map(|ids| ids.len().into())
            }
        }
    }
}

impl From<InvocationIoItem> for ResourceSpecifier {
    fn from(value: InvocationIoItem) -> Self {
        match value {
            InvocationIoItem::Fungible(
                resource_address,
                Guaranteed(amount) | Predicted(Tracked { value: amount, .. }),
            ) => ResourceSpecifier::Amount(resource_address, amount),
            InvocationIoItem::NonFungible(
                resource_address,
                Guaranteed(ids) | Predicted(Tracked { value: ids, .. }),
            ) => ResourceSpecifier::Ids(resource_address, ids),
        }
    }
}

/// An enum used to represent data and whether this data is guaranteed or
/// predicted.
///
/// If the data is [`Guaranteed`] then it's stored in the [`Guaranteed`] variant
/// of the enum as [`T`]. If [`Predicted`] then it's stored in the [`Predicted`]
/// variant of the enum as [`Tracked<T>`].
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EitherGuaranteedOrPredicted<T> {
    Guaranteed(T),
    Predicted(Tracked<T>),
}

impl<T> EitherGuaranteedOrPredicted<T> {
    pub fn new_guaranteed(value: T) -> Self {
        Guaranteed(value)
    }

    pub fn new_predicted(value: T, created_at: InstructionIndex) -> Self {
        Predicted(Tracked::new(value, created_at))
    }

    pub fn as_ref(&self) -> EitherGuaranteedOrPredicted<&T> {
        match self {
            Guaranteed(v) => Guaranteed(v),
            Predicted(v) => Predicted(v.as_ref()),
        }
    }

    pub fn as_mut(&mut self) -> EitherGuaranteedOrPredicted<&mut T> {
        match self {
            Guaranteed(v) => Guaranteed(v),
            Predicted(v) => Predicted(v.as_mut()),
        }
    }

    pub fn map<A>(
        self,
        mut f: impl FnMut(T) -> A,
    ) -> EitherGuaranteedOrPredicted<A> {
        match self {
            Guaranteed(v) => Guaranteed(f(v)),
            Predicted(tracked) => {
                Predicted(Tracked::new(f(tracked.value), tracked.created_at))
            }
        }
    }
}

impl<T> Deref for EitherGuaranteedOrPredicted<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            Guaranteed(v) | Predicted(Tracked { value: v, .. }) => v,
        }
    }
}

impl<T> DerefMut for EitherGuaranteedOrPredicted<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Guaranteed(v) | Predicted(Tracked { value: v, .. }) => v,
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

#[derive(Clone, Debug, PartialEq, Eq, Default)]
struct StaticAnalysisInvocationIo(
    IndexMap<InstructionIndex, InvocationIo<TrackedResources>>,
);

impl StaticAnalysisInvocationIo {
    pub fn compute(manifest: &impl ReadableManifest) -> Result<Self, Error> {
        // The initial worktop state is only unknown if the manifest is a
        // subintent manifest. Otherwise, in the case of a v1 or v2 manifest the
        // initial worktop state is known to be zero since they can't be used as
        // subintents and can't be yielded into.
        let initial_worktop_state_is_unknown = manifest.is_subintent();
        let interpreter = StaticManifestInterpreter::new(
            ValidationRuleset::babylon_equivalent(),
            manifest,
        );
        let mut visitor = StaticResourceMovementsVisitor::new(
            initial_worktop_state_is_unknown,
        );
        interpreter.validate_and_apply_visitor(&mut visitor)?;

        let invocation_io = visitor
            .output()
            .invocation_static_information
            .into_iter()
            .map(
                |(
                    instruction_index,
                    InvocationStaticInformation { input, output, .. },
                )| {
                    let instruction_index =
                        InstructionIndex::of(instruction_index);
                    (instruction_index, InvocationIo { input, output })
                },
            )
            .collect::<IndexMap<_, _>>();
        Ok(Self(invocation_io))
    }

    pub fn remove(
        &mut self,
        instruction_index: &InstructionIndex,
    ) -> Option<InvocationIo<TrackedResources>> {
        self.0.swap_remove(instruction_index)
    }

    pub fn instruction_index_iter(
        &self,
    ) -> impl Iterator<Item = &InstructionIndex> {
        self.0.keys()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
struct DynamicAnalysisInvocationIo<'a>(
    IndexMap<
        InstructionIndex,
        InvocationIo<Vec<Tracked<Cow<'a, ResourceSpecifier>>>>,
    >,
);

impl<'a> DynamicAnalysisInvocationIo<'a> {
    pub fn empty() -> Self {
        Default::default()
    }

    pub fn add_input(
        &mut self,
        instruction_index: InstructionIndex,
        input: Tracked<Cow<'a, ResourceSpecifier>>,
    ) {
        self.0
            .entry(instruction_index)
            .or_default()
            .input
            .push(input);
    }

    pub fn add_output(
        &mut self,
        instruction_index: InstructionIndex,
        output: Tracked<Cow<'a, ResourceSpecifier>>,
    ) {
        self.0
            .entry(instruction_index)
            .or_default()
            .output
            .push(output);
    }

    pub fn compute<'w>(
        manifest: &impl ReadableManifest,
        worktop_changes: &'w WorktopChanges<'w>,
    ) -> DynamicAnalysisInvocationIo<'w> {
        let mut this = DynamicAnalysisInvocationIo::<'w>::empty();
        let mut id_allocator = ManifestIdAllocator::new();
        let mut tracked_buckets = IndexMap::new();

        for (instruction_index, effect) in
            manifest.iter_instruction_effects().enumerate()
        {
            let instruction_index = InstructionIndex::of(instruction_index);

            match effect {
                // Bucket Creation
                ManifestInstructionEffect::CreateBucket { source_amount } => {
                    let resource_address = source_amount.resource_address();
                    let bucket = id_allocator.new_bucket_id();
                    let bucket_content = worktop_changes
                        .first_take(&instruction_index)
                        .map(Cow::Borrowed)
                        .unwrap_or(Cow::Owned(ResourceSpecifier::new_empty(
                            *resource_address,
                        )));
                    let tracked_bucket_content = Tracked {
                        value: bucket_content,
                        created_at: instruction_index,
                    };
                    tracked_buckets.insert(bucket, tracked_bucket_content);
                }
                // Bucket Consumption
                ManifestInstructionEffect::ConsumeBucket {
                    consumed_bucket,
                    destination,
                } => match destination {
                    BucketDestination::Worktop | BucketDestination::Burned => {
                        let _ = tracked_buckets.swap_remove(&consumed_bucket);
                    }
                    BucketDestination::Invocation(..) => {
                        let tracked_bucket_contents =
                            tracked_buckets.swap_remove(&consumed_bucket).expect(
                                "Can't fail, the transaction committed successfully.",
                            );
                        this.add_input(
                            instruction_index,
                            tracked_bucket_contents,
                        );
                    }
                },
                ManifestInstructionEffect::Invocation { args, .. } => {
                    // Handling the output.
                    worktop_changes
                        .put_iterator(&instruction_index)
                        .map(|resource_specifier| Tracked {
                            value: Cow::Borrowed(resource_specifier),
                            created_at: instruction_index,
                        })
                        .for_each(|output| {
                            this.add_output(instruction_index, output)
                        });

                    // Handling the input.
                    let indexed_value =
                        IndexedManifestValue::from_manifest_value(args);

                    let buckets = indexed_value.buckets();
                    let expressions = indexed_value.expressions();
                    let has_entire_worktop_expression = expressions
                        .contains(&ManifestExpression::EntireWorktop);

                    let buckets_tracked_resources = buckets.iter().map(|bucket| {
                        tracked_buckets
                            .swap_remove(bucket)
                            .expect("Can't fail, the transaction committed successfully.")
                    });
                    let expression_tracked_resources = worktop_changes
                        .take_iterator(&instruction_index)
                        .map(|resource_specifier| Tracked {
                            value: Cow::Borrowed(resource_specifier),
                            created_at: instruction_index,
                        });

                    for tracked_invocation_input in buckets_tracked_resources {
                        this.add_input(
                            instruction_index,
                            tracked_invocation_input,
                        );
                    }
                    if has_entire_worktop_expression {
                        for tracked_invocation_input in
                            expression_tracked_resources
                        {
                            this.add_input(
                                instruction_index,
                                tracked_invocation_input,
                            );
                        }
                    }
                }
                // No effect on the resource movements
                ManifestInstructionEffect::CreateProof { .. }
                | ManifestInstructionEffect::ConsumeProof { .. }
                | ManifestInstructionEffect::CloneProof { .. }
                | ManifestInstructionEffect::DropManyProofs { .. }
                | ManifestInstructionEffect::CreateAddressAndReservation {
                    ..
                }
                | ManifestInstructionEffect::ResourceAssertion { .. }
                | ManifestInstructionEffect::Verification { .. } => {}
            }
        }

        this
    }

    pub fn remove(
        &mut self,
        instruction_index: &InstructionIndex,
    ) -> InvocationIo<Vec<Tracked<Cow<'a, ResourceSpecifier>>>> {
        self.0.swap_remove(instruction_index).unwrap_or_default()
    }

    pub fn instruction_index_iter(
        &self,
    ) -> impl Iterator<Item = &InstructionIndex> {
        self.0.keys()
    }
}

#[derive(Debug)]
pub enum InvocationIoError {
    StaticResourceMovementsError(StaticResourceMovementsError),
    NoDynamicAnalysisWhenStaticAnalysisIsPresent,
}
use InvocationIoError as Error;

impl From<StaticResourceMovementsError> for InvocationIoError {
    fn from(v: StaticResourceMovementsError) -> Self {
        Self::StaticResourceMovementsError(v)
    }
}
