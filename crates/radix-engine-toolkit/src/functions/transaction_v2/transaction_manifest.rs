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

pub fn to_payload_bytes(
    manifest: &TransactionManifestV2,
) -> Result<Vec<u8>, EncodeError> {
    manifest.clone().to_raw().map(|raw| raw.to_vec())
}

pub fn from_payload_bytes<T>(
    payload_bytes: T,
) -> Result<TransactionManifestV2, String>
where
    T: AsRef<[u8]>,
{
    TransactionManifestV2::from_raw(&payload_bytes.as_ref().to_vec().into())
}

pub fn statically_analyze(
    manifest: &TransactionManifestV2,
) -> Result<StaticAnalysis, ManifestAnalysisError> {
    crate::internal_prelude::statically_analyze(manifest)
}

pub fn dynamically_analyze(
    manifest: &TransactionManifestV2,
    receipt: RuntimeToolkitTransactionReceipt,
) -> Result<DynamicAnalysis, ManifestAnalysisError> {
    crate::internal_prelude::dynamically_analyze(manifest, receipt)
}

pub fn statically_validate(
    manifest: &TransactionManifestV2,
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
