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

/// Manifests that lock or unlock the primary role of one or more access
/// controllers.
///
/// On each access controller the manifest either locks or unlocks the primary
/// role, once. While locked, the primary role can't create proofs of the
/// controlled badge, so the entity it controls can't be used. Confirming a
/// recovery unlocks it. Besides the access controller calls, the manifest may
/// only pay the fee.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct AccessControllerPrimaryRoleLockClass;

pub type AccessControllerPrimaryRoleLockAnalyzer =
    AccessControllerAnalyzer<AccessControllerPrimaryRoleLockClass>;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct AccessControllerPrimaryRoleLockOutput {
    pub operations: Vec<AccessControllerPrimaryRoleLockOperation>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AccessControllerPrimaryRoleLockOperation {
    pub access_controller: ComponentAddress,
    pub operation: PrimaryRoleLockOperation,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PrimaryRoleLockOperation {
    Lock,
    Unlock,
}

impl AccessControllerClass for AccessControllerPrimaryRoleLockClass {
    type Output = AccessControllerPrimaryRoleLockOutput;

    fn classify(calls: &AccessControllerCalls) -> Option<Self::Output> {
        let operations = calls
            .by_access_controller()?
            .iter()
            .map(|(access_controller, calls)| {
                let operation = match calls.as_slice() {
                    [AccessControllerCall::LockPrimaryRole] => {
                        PrimaryRoleLockOperation::Lock
                    }
                    [AccessControllerCall::UnlockPrimaryRole] => {
                        PrimaryRoleLockOperation::Unlock
                    }
                    _ => return None,
                };
                Some(AccessControllerPrimaryRoleLockOperation {
                    access_controller: *access_controller,
                    operation,
                })
            })
            .collect::<Option<_>>()?;

        Some(AccessControllerPrimaryRoleLockOutput { operations })
    }
}
