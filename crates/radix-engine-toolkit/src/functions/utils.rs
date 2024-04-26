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

use bech32::{FromBase32, Variant};
use radix_transactions::prelude::*;
use scrypto::prelude::*;

pub fn decode_transaction_id(
    transaction_id: &str,
    network_definition: &NetworkDefinition,
) -> Result<Hash, TransactionHashBech32DecodeError> {
    // Decode the hash string
    let (hrp, data, variant) = bech32::decode(transaction_id)
        .map_err(TransactionHashBech32DecodeError::Bech32mDecodingError)?;

    // Validate the HRP
    let hrp_set = HrpSet::from(network_definition);
    if [
        hrp_set.transaction_intent,
        hrp_set.signed_transaction_intent,
        hrp_set.notarized_transaction,
        hrp_set.round_update_transaction,
        hrp_set.system_transaction,
        hrp_set.ledger_transaction,
    ]
    .contains(&hrp)
    {
        Ok(())
    } else {
        Err(TransactionHashBech32DecodeError::InvalidHrp)
    }?;

    // Validate the Bech32 variant to ensure that is is Bech32m
    match variant {
        Variant::Bech32m => {}
        _ => {
            return Err(TransactionHashBech32DecodeError::InvalidVariant(
                variant,
            ))
        }
    };

    // Convert the data to u8 from u5.
    let data = Vec::<u8>::from_base32(&data)
        .map_err(TransactionHashBech32DecodeError::Bech32mDecodingError)?;

    // Validate the length
    let hash = data
        .try_into()
        .map(Hash)
        .map_err(|_| TransactionHashBech32DecodeError::InvalidLength)?;

    // Validation complete, return data bytes
    Ok(hash)
}
