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

use super::access_controller_utils::*;
use crate::prelude::*;

const PRIMARY_ROLE_LOCK: Option<ManifestClassification> =
    Some(ManifestClassification::AccessControllerPrimaryRoleLock);

fn access_controller() -> ComponentAddress {
    TestAddressAllocator::new().new_access_controller_address()
}

#[test]
fn locking_and_unlocking_show_the_operation() {
    // Arrange
    let (mut ledger, account, access_controller) =
        ledger_with_access_controller();
    let lock = ManifestBuilder::new()
        .lock_fee(account, 10)
        .lock_primary_role(access_controller)
        .build();
    let unlock = ManifestBuilder::new()
        .lock_fee(account, 10)
        .unlock_primary_role(access_controller)
        .build();

    // Act
    let classes = [lock, unlock].map(|manifest| {
        detailed_access_controller_class(&mut ledger, manifest)
    });

    // Assert
    assert_eq!(
        classes,
        [
            PrimaryRoleLockOperation::Lock,
            PrimaryRoleLockOperation::Unlock
        ]
        .map(|operation| {
            DetailedManifestClassification::AccessControllerPrimaryRoleLock(
                AccessControllerPrimaryRoleLockOutput {
                    operations: vec![
                        AccessControllerPrimaryRoleLockOperation {
                            access_controller,
                            operation,
                        },
                    ],
                },
            )
        })
    );
}

#[test]
fn locking_and_unlocking_different_access_controllers_is_primary_role_lock() {
    let mut allocator = TestAddressAllocator::new();
    let manifest = ManifestBuilder::new()
        .lock_primary_role(allocator.new_access_controller_address())
        .unlock_primary_role(allocator.new_access_controller_address())
        .build();
    assert_access_controller_class(manifest, PRIMARY_ROLE_LOCK);
}

#[test]
fn several_operations_on_the_same_access_controller_are_not_primary_role_lock()
{
    let access_controller = access_controller();
    let operations: [fn(ManifestBuilder, ComponentAddress) -> ManifestBuilder;
        2] = [
        |builder, access_controller| {
            builder.lock_primary_role(access_controller)
        },
        |builder, access_controller| {
            builder.unlock_primary_role(access_controller)
        },
    ];
    for first in operations {
        for second in operations {
            let builder = first(ManifestBuilder::new(), access_controller);
            let manifest = second(builder, access_controller).build();
            assert_access_controller_class(manifest, None);
        }
    }
}

#[test]
fn setting_account_metadata_is_not_primary_role_lock() {
    let mut allocator = TestAddressAllocator::new();
    let account = allocator.new_account_address();
    let manifest = ManifestBuilder::new()
        .lock_primary_role(allocator.new_access_controller_address())
        .set_metadata(account, "owner_keys", "not important")
        .build();
    assert_access_controller_class(manifest, None);
}
