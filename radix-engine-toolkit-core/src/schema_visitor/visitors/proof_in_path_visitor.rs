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

use crate::schema_visitor::core::{error::SchemaVisitorError, traits::SchemaVisitor};
use radix_engine_common::prelude::{
    OwnValidation, ScryptoCustomSchema, ScryptoCustomTypeKind, ScryptoCustomTypeValidation,
};
use sbor::{LocalTypeId, Schema, TypeValidation};

#[derive(Default)]
pub struct ProofInPathVisitor(bool);

impl SchemaVisitor<ScryptoCustomSchema> for ProofInPathVisitor {
    type Error = SchemaVisitorError;

    fn visit_custom(
        &mut self,
        local_type_id: LocalTypeId,
        schema: &Schema<ScryptoCustomSchema>,
        custom_type_kind: &<ScryptoCustomSchema as sbor::CustomSchema>::CustomTypeKind<LocalTypeId>,
    ) -> Result<(), Self::Error> {
        let type_validation = schema.resolve_type_validation(local_type_id).map_or(
            Err(SchemaVisitorError::InvalidLocalTypeId(local_type_id)),
            Ok,
        )?;
        if *custom_type_kind == ScryptoCustomTypeKind::Own
            && matches!(
                type_validation,
                TypeValidation::<ScryptoCustomTypeValidation>::Custom(
                    ScryptoCustomTypeValidation::Own(OwnValidation::IsProof),
                )
            )
        {
            self.0 = true
        }

        Ok(())
    }

    fn is_enabled(&self) -> bool {
        !self.0
    }
}

impl ProofInPathVisitor {
    pub fn path_contains_proof(self) -> bool {
        self.0
    }
}
