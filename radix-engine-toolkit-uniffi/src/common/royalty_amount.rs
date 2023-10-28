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

use crate::prelude::*;

#[derive(Clone, Debug, Enum)]
pub enum RoyaltyAmount {
    Free,
    Xrd { value: Arc<Decimal> },
    Usd { value: Arc<Decimal> },
}

impl From<NativeRoyaltyAmount> for RoyaltyAmount {
    fn from(value: NativeRoyaltyAmount) -> Self {
        match value {
            NativeRoyaltyAmount::Free => Self::Free,
            NativeRoyaltyAmount::Xrd(amount) => Self::Xrd {
                value: Arc::new(Decimal(amount)),
            },
            NativeRoyaltyAmount::Usd(amount) => Self::Usd {
                value: Arc::new(Decimal(amount)),
            },
        }
    }
}

impl From<RoyaltyAmount> for NativeRoyaltyAmount {
    fn from(value: RoyaltyAmount) -> Self {
        match value {
            RoyaltyAmount::Free => Self::Free,
            RoyaltyAmount::Xrd { value: amount } => Self::Xrd(amount.0),
            RoyaltyAmount::Usd { value: amount } => Self::Usd(amount.0),
        }
    }
}
