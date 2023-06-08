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

mod function_examples;

use crate::function_examples::generator::generate_function_examples;
use std::path::PathBuf;

fn main() {
    let output_directory = {
        let manifest_directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        manifest_directory.join("output")
    };
    std::fs::create_dir_all(&output_directory).unwrap();

    // Generating the function examples
    {
        let output_directory = output_directory.join("function_examples");
        std::fs::create_dir_all(&output_directory).unwrap();

        let function_examples = generate_function_examples();
        for (module, examples) in function_examples {
            let output_path = output_directory.join(format!("{module}.json"));
            let serialized = serde_json::to_string_pretty(&examples).unwrap();

            std::fs::write(output_path, serialized).unwrap();
        }
    }
}
