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

use std::fmt::Debug;

use convert_case::{Case, Casing};
use serde::Serialize;

use crate::examples::{Example, ExampleData};

pub struct InMemoryExamplesBuilder {
    examples: Vec<Example>,
}

impl InMemoryExamplesBuilder {
    pub fn new() -> Self {
        Self {
            examples: Vec::new(),
        }
    }

    pub fn add_example<H, I, O>(mut self) -> Self
    where
        H: ExampleData<I, O>,
        I: Serialize + Clone + Debug,
        O: Serialize + Clone + Debug,
    {
        let example = H::to_example();
        self.examples.push(example);
        self
    }

    pub fn build(&self) -> String {
        let mut examples_markdown = Vec::<String>::new();
        for example in self.examples.iter() {
            let request_cleaned_name = example.function_name.clone();

            let title = request_cleaned_name.to_case(Case::Title);
            let function_name = request_cleaned_name.to_case(Case::Snake);
            let jni_function_name = format!(
                "Java_RadixEngineToolkitFFI_{}",
                request_cleaned_name.to_case(Case::Camel)
            );

            let example_string = format!(
                r#"## {}

| Function Name     | `{}` |
| ----------------- | :----------------- |
| JNI Function Name | `{}` |
| Functionality     | {}   |
| Required Features | {}   |

<details>
    <summary>Request Example</summary>

```json
{}
```
</details>

<details>
    <summary>Response Example</summary>

```json
{}
```
</details>
"#,
                title,
                function_name,
                jni_function_name,
                example.function_description.replace('\n', "</br>"),
                example.required_features,
                example.input,
                example.output,
            );
            examples_markdown.push(example_string);
        }
        examples_markdown.insert(
            0,
            String::from(
                r#"# Requests Examples

This document contains examples and descriptions of the different requests and responses which
the Radix Engine Toolkit may provide. As long as all of the CI test pass, then you may treat this
document as the canonical truth for the format of the different requests and as valid examples of
the payload and responses of these requests.

"#,
            ),
        );
        examples_markdown.join("\n")
    }
}
