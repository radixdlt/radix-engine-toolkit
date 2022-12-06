// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at

//   http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "variant")]
pub enum ResultProxy<O, E> {
    Ok { field: O },
    Err { field: E },
}

impl<O, E> From<ResultProxy<O, E>> for Result<O, E> {
    fn from(result: ResultProxy<O, E>) -> Self {
        match result {
            ResultProxy::Ok { field } => Result::Ok(field),
            ResultProxy::Err { field } => Result::Err(field),
        }
    }
}

impl<O, E> From<Result<O, E>> for ResultProxy<O, E> {
    fn from(result: Result<O, E>) -> Self {
        match result {
            Result::Ok(field) => ResultProxy::Ok { field },
            Result::Err(field) => ResultProxy::Err { field },
        }
    }
}
