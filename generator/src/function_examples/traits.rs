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

use native_json_library::functions::traits::Function;
use serde::{Deserialize, Serialize};

use crate::utils;

pub trait HasExamples<'f, const N: usize>
where
    Self: Function<'f> + Sized,
{
    fn function_name() -> String {
        utils::snake_case_type_name::<Self>()
    }

    fn example_inputs() -> [Self::Input; N];

    fn example_outputs() -> [Self::Output; N] {
        Self::example_inputs().map(|input| Self::handle(input).unwrap())
    }

    fn examples() -> [FunctionExample<Self::Input, Self::Output>; N] {
        unwrap_or_panic(
            Self::example_inputs()
                .into_iter()
                .zip(Self::example_outputs().into_iter())
                .map(|(input, output)| FunctionExample { input, output })
                .collect::<Vec<_>>()
                .try_into(),
        )
    }

    fn serde_value_examples() -> [FunctionExample<serde_json::Value, serde_json::Value>; N] {
        Self::examples().map(|example| FunctionExample {
            input: serde_json::to_value(example.input).unwrap(),
            output: serde_json::to_value(example.output).unwrap(),
        })
    }
}

#[derive(Serialize, Deserialize)]
pub struct FunctionExample<I, O> {
    input: I,
    output: O,
}

fn unwrap_or_panic<O, E>(result: Result<O, E>) -> O {
    match result {
        Ok(value) => value,
        Err(_) => panic!("Unwrap error"),
    }
}
