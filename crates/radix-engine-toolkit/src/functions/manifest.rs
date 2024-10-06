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

use native_radix_engine_toolkit::receipt::RuntimeToolkitTransactionReceipt;
use radix_common::prelude::*;
use radix_engine_interface::blueprints::access_controller::*;
use radix_engine_interface::blueprints::account::*;
use radix_transactions::errors::*;
use radix_transactions::prelude::*;
use radix_transactions::validation::*;

use crate::transaction_types::*;

pub fn hash(manifest: &TransactionManifestV1) -> Result<Hash, EncodeError> {
    compile(manifest).map(scrypto::prelude::hash)
}

pub fn compile(
    manifest: &TransactionManifestV1,
) -> Result<Vec<u8>, EncodeError> {
    manifest_encode(manifest)
}

pub fn decompile<T>(
    payload_bytes: T,
) -> Result<TransactionManifestV1, DecodeError>
where
    T: AsRef<[u8]>,
{
    manifest_decode(payload_bytes.as_ref())
}

pub fn statically_validate(
    manifest: &TransactionManifestV1,
) -> Result<(), TransactionValidationError> {
    NotarizedTransactionValidator::validate_instructions_v1(
        &manifest.instructions,
    )
}

pub fn modify(
    manifest: &TransactionManifestV1,
    mut modifications: TransactionManifestModifications,
) -> Result<TransactionManifestV1, ManifestModificationError> {
    // The modifications made to the manifest are done in the following order:
    // 1. Adding Assertions.
    // 2. Adding the lock fee instructions.
    // 3. Adding the access controller instructions
    let mut instructions = manifest.instructions.clone();

    // We sort the assertions array in descending order according to the
    // instruction index so that we avoid worrying about the instruction shift
    // as we insert them.
    modifications
        .add_assertions
        .sort_by(|(b, _), (a, _)| a.cmp(b));

    // Vec::insert panics if the insertion index is larger than the length of
    // the vector. So, we check for this and handle is gracefully.
    modifications
        .add_assertions
        .first()
        .map_or(Ok(()), |(index, _)| {
            if *index > instructions.len() {
                Err(ManifestModificationError::AssertionIndexOutOfBounds {
                    assertion_index: *index,
                    instructions_length: instructions.len(),
                })
            } else {
                Ok(())
            }
        })?;

    // Adding the assertions.
    for (instruction_index, assertion) in
        modifications.add_assertions.into_iter()
    {
        let assertion_instruction = InstructionV1::from(assertion);
        instructions.insert(instruction_index, assertion_instruction);
    }

    // Adding the lock fee instruction. This depends if the first instruction in
    // the manifest is a call to withdraw from an account or not. If yes,
    // then we transform the call to `withdraw` to
    // a `lock_fee_and_withdraw`. Same thing happens for the non-fungible
    // variant of the method. If the first method is not a withdraw method,
    // then we just insert a new instruction that locks a fee against said
    // account.
    if let Some((lock_fee_account, lock_fee_amount)) =
        modifications.add_lock_fee
    {
        match instructions.first_mut() {
            Some(InstructionV1::CallMethod {
                address: DynamicGlobalAddress::Static(address),
                method_name,
                args,
            }) if *address == GlobalAddress::from(lock_fee_account)
                && (method_name == ACCOUNT_WITHDRAW_IDENT
                    || method_name == ACCOUNT_WITHDRAW_NON_FUNGIBLES_IDENT) =>
            {
                match method_name.as_str() {
                    ACCOUNT_WITHDRAW_IDENT => {
                        let AccountWithdrawInput {
                            amount: withdraw_amount,
                            resource_address: withdraw_resource_address,
                        } = manifest_encode(args)
                            .map_err(|error| ManifestModificationError::SborEncodeError {
                                value: args.clone(),
                                error,
                            })
                            .and_then(|encoded| {
                                manifest_decode(&encoded).map_err(|_| {
                                    ManifestModificationError::InvalidArguments {
                                        method_name: method_name.to_owned(),
                                        arguments: args.clone(),
                                    }
                                })
                            })?;

                        *method_name =
                            ACCOUNT_LOCK_FEE_AND_WITHDRAW_IDENT.to_owned();
                        *args = to_manifest_value_and_unwrap!(
                            &AccountLockFeeAndWithdrawInput {
                                amount_to_lock: lock_fee_amount,
                                resource_address: withdraw_resource_address,
                                amount: withdraw_amount
                            }
                        );
                    }
                    ACCOUNT_WITHDRAW_NON_FUNGIBLES_IDENT => {
                        let AccountWithdrawNonFungiblesInput {
                            ids: withdraw_ids,
                            resource_address: withdraw_resource_address,
                        } = manifest_encode(args)
                            .map_err(|error| ManifestModificationError::SborEncodeError {
                                value: args.clone(),
                                error,
                            })
                            .and_then(|encoded| {
                                manifest_decode(&encoded).map_err(|_| {
                                    ManifestModificationError::InvalidArguments {
                                        method_name: method_name.to_owned(),
                                        arguments: args.clone(),
                                    }
                                })
                            })?;

                        *method_name =
                            ACCOUNT_LOCK_FEE_AND_WITHDRAW_NON_FUNGIBLES_IDENT
                                .to_owned();
                        *args = to_manifest_value_and_unwrap!(
                            &AccountLockFeeAndWithdrawNonFungiblesInput {
                                amount_to_lock: lock_fee_amount,
                                resource_address: withdraw_resource_address,
                                ids: withdraw_ids
                            }
                        );
                    }
                    _ => unreachable!(),
                }
            }
            _ => {
                // There are a few ways for us to get to this point here:
                // 1. There is no first instruction.
                // 2. There is a first instruction but it's not a CallMethod.
                // 3. There is a call method instruction but it's not to the
                //    same account that we want to lock fees against.
                // 4. There is a call method instruction to the account that we
                //    want to lock fees against but it's not to the account
                //    withdraw methods.
                instructions.insert(
                    0,
                    InstructionV1::CallMethod {
                        address: DynamicGlobalAddress::Static(
                            lock_fee_account.into(),
                        ),
                        method_name: ACCOUNT_LOCK_FEE_IDENT.to_owned(),
                        args: to_manifest_value_and_unwrap!(
                            &AccountLockFeeInput {
                                amount: lock_fee_amount
                            }
                        ),
                    },
                )
            }
        };
    };

    // Adding the access controller proofs.
    let instructions = modifications
        .add_access_controller_proofs
        .into_iter()
        .map(|component_address| InstructionV1::CallMethod {
            address: DynamicGlobalAddress::Static(component_address.into()),
            method_name: ACCESS_CONTROLLER_CREATE_PROOF_IDENT.to_owned(),
            args: to_manifest_value_and_unwrap!(
                &AccessControllerCreateProofInput {}
            ),
        })
        .chain(instructions)
        .collect::<Vec<_>>();

    Ok(TransactionManifestV1 {
        instructions,
        blobs: manifest.blobs.clone(),
    })
}

pub fn summary(manifest: &TransactionManifestV1) -> ManifestSummary {
    crate::transaction_types::summary(manifest)
}

pub fn execution_summary(
    manifest: &TransactionManifestV1,
    receipt: &RuntimeToolkitTransactionReceipt,
) -> Result<ExecutionSummary, TransactionTypesError> {
    crate::transaction_types::execution_summary(manifest, receipt)
}

#[derive(Clone, Debug)]
pub struct TransactionManifestModifications {
    /// The [`ComponentAddress`]es of the access controllers to add create
    /// proof instructions in the manifest for.
    pub add_access_controller_proofs: Vec<ComponentAddress>,

    /// The account to lock a fee against and the amount of fee to lock.
    pub add_lock_fee: Option<(ComponentAddress, Decimal)>,

    /// A vector of the assertions to add to the manifest.
    pub add_assertions: Vec<(usize, Assertion)>,
}

#[derive(Clone, Debug)]
pub enum Assertion {
    Amount {
        resource_address: ResourceAddress,
        amount: Decimal,
    },
    Ids {
        resource_address: ResourceAddress,
        ids: BTreeSet<NonFungibleLocalId>,
    },
}

impl From<Assertion> for InstructionV1 {
    fn from(value: Assertion) -> Self {
        match value {
            Assertion::Amount {
                resource_address,
                amount,
            } => InstructionV1::AssertWorktopContains {
                resource_address,
                amount,
            },
            Assertion::Ids {
                resource_address,
                ids,
            } => InstructionV1::AssertWorktopContainsNonFungibles {
                resource_address,
                ids: ids.into_iter().collect(),
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ManifestModificationError {
    InvalidArguments {
        method_name: String,
        arguments: ManifestValue,
    },
    AssertionIndexOutOfBounds {
        assertion_index: usize,
        instructions_length: usize,
    },
    SborEncodeError {
        value: ManifestValue,
        error: EncodeError,
    },
}
