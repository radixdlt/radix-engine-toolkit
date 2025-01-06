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

use radix_common::prelude::*;
use radix_transactions::manifest::*;
use radix_transactions::prelude::*;
use scrypto::prelude::*;
use static_resource_movements::*;

use crate::transaction_types::*;

pub fn to_payload_bytes(
    manifest: &SubintentManifestV2,
) -> Result<Vec<u8>, EncodeError> {
    manifest.clone().to_raw().map(|raw| raw.to_vec())
}

pub fn from_payload_bytes<T>(
    payload_bytes: T,
) -> Result<SubintentManifestV2, String>
where
    T: AsRef<[u8]>,
{
    SubintentManifestV2::from_raw(&payload_bytes.as_ref().to_vec().into())
}

pub fn statically_analyze(manifest: &SubintentManifestV2) -> StaticAnalysis {
    crate::transaction_types::statically_analyze(manifest)
}

pub fn statically_analyze_and_validate(
    manifest: &SubintentManifestV2,
) -> Result<StaticAnalysisWithResourceMovements, StaticResourceMovementsError> {
    crate::transaction_types::statically_analyze_and_validate(manifest)
}

pub fn as_enclosed(
    SubintentManifestV2 {
        instructions,
        blobs,
        children,
        object_names,
    }: &SubintentManifestV2,
) -> Option<TransactionManifestV2> {
    let [
        assert_worktop_empty_instruction @ InstructionV2::AssertWorktopResourcesOnly(
            AssertWorktopResourcesOnly { constraints },
        ),
        other_instructions @ ..,
        InstructionV2::YieldToParent(..),
    ] = instructions.as_slice()
    else {
        return None;
    };
    if !constraints.specified_resources().len().is_zero() {
        return None;
    }

    let is_enclosed = !other_instructions.iter().any(|instruction| {
        matches!(
            instruction,
            InstructionV2::YieldToChild(..) | InstructionV2::YieldToParent(..)
        )
    });

    if is_enclosed {
        Some(TransactionManifestV2 {
            instructions: std::iter::once(assert_worktop_empty_instruction)
                .chain(other_instructions)
                .cloned()
                .collect(),
            blobs: blobs.clone(),
            children: children.clone(),
            object_names: object_names.clone(),
        })
    } else {
        None
    }
}

pub fn statically_validate(
    manifest: &SubintentManifestV2,
) -> Result<(), ManifestValidationError> {
    pub struct Error(ManifestValidationError);
    impl From<ManifestValidationError> for Error {
        fn from(value: ManifestValidationError) -> Self {
            Self(value)
        }
    }
    impl From<Error> for ManifestValidationError {
        fn from(value: Error) -> Self {
            value.0
        }
    }

    pub struct Visitor;
    impl ManifestInterpretationVisitor for Visitor {
        type Output = Error;
    }

    let interpreter =
        StaticManifestInterpreter::new(ValidationRuleset::all(), manifest);
    interpreter.validate_and_apply_visitor(&mut Visitor)?;

    Ok(())
}
