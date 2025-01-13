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

/// A type used to represent the output of most of the transaction types
/// classifications.
///
/// In all of the visitors we have authored there are roughly three modes that
/// their output could be in. This can be more easily summarized by the table
/// that follows:
///
/// Is classification       Could we extract    
/// requirement fulfilled   more information    Variant
/// 0                       _                   Classification not fulfilled.
/// 1                       0                   Classification is fulfilled
///                                             but we couldn't extract the
///                                             typical set of visitor info
///                                             due to whatever reason.
/// 1                       1                   Classification is fulfilled and
///                                             we were able to extract the
///                                             typical set of information on
///                                             the transaction type.
///
/// An example of when this can happen is in a validator stake transaction. A
/// manifest can be detected to be a valid validator stake manifest if it
/// fulfills the set of invocations we allow and disallow but if the toolkit
/// receipt is not provided (in the case of static analysis) then we can't
/// extract more information from the manifest but it is still a validator
/// stake manifest. In this example, we can't extract more information without
/// the receipt because we can't tell what the address of the LSUs are.
///
/// This type should not escape the toolkit and should not really be exposed
/// to the toolkit's clients. However, it's kept as `pub` since we use it as
/// an associated type in the visitor implementations.
pub enum DetailedTransactionTypeOutput<T> {
    DoesNotFulfillClassificationRequirement,
    FulfillsClassificationRequirement,
    FulfillsDetailedClassificationRequirement(T),
}

impl<T> DetailedTransactionTypeOutput<T> {
    pub fn new(
        are_all_instructions_permitted: bool,
        is_classification_requirement_fulfilled: bool,
        is_detailed_classification_requirement_fulfilled: bool,
        value: T,
    ) -> Self {
        match (
            are_all_instructions_permitted,
            is_classification_requirement_fulfilled,
            is_detailed_classification_requirement_fulfilled,
        ) {
            (true, true, true) => {
                Self::FulfillsDetailedClassificationRequirement(value)
            }
            (true, true, false) => Self::FulfillsClassificationRequirement,
            (false, _, _) | (_, false, _) => {
                Self::DoesNotFulfillClassificationRequirement
            }
        }
    }

    pub fn is_classification_fulfilled(&self) -> bool {
        matches!(
            self,
            Self::FulfillsClassificationRequirement
                | Self::FulfillsDetailedClassificationRequirement(..)
        )
    }
}
