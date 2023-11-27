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
mod function_schema;
mod function_spec;
mod serializable_models;
mod utils;

use crate::function_examples::generator::generate_function_examples;
use function_schema::generator::generate_function_schema;
use function_spec::generator::generate_function_spec;
use serializable_models::generator::generate_serializable_model_examples;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use utils::convert_open_api_adts_to_discriminated_unions;

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

    // Generating the function JSON schema
    {
        let output_directory = output_directory.join("function_json_schema");
        std::fs::create_dir_all(&output_directory).unwrap();

        let function_examples = generate_function_schema();
        for (module, functions) in function_examples {
            let output_directory = output_directory.join(module);

            for (function_name, (input_schema, output_schema)) in functions {
                let output_directory = output_directory.join(function_name);
                std::fs::create_dir_all(&output_directory).unwrap();

                {
                    let output_path = output_directory.join("Input.json");
                    let serialized =
                        serde_json::to_string_pretty(&input_schema).unwrap();
                    std::fs::write(output_path, serialized).unwrap();
                }

                {
                    let output_path = output_directory.join("Output.json");
                    let serialized =
                        serde_json::to_string_pretty(&output_schema).unwrap();
                    std::fs::write(output_path, serialized).unwrap();
                }
            }
        }
    }

    // Generating the model examples
    {
        let output_directory =
            output_directory.join("serializable_model_examples");
        std::fs::create_dir_all(&output_directory).unwrap();

        let serializable_models_examples =
            generate_serializable_model_examples();
        for (path_extension, examples) in serializable_models_examples {
            let output_directory = output_directory.join(path_extension);
            std::fs::create_dir_all(&output_directory).unwrap();

            for (file_name, examples) in examples {
                let output_path =
                    output_directory.join(format!("{file_name}.json"));
                let serialized =
                    serde_json::to_string_pretty(&examples).unwrap();
                std::fs::write(output_path, serialized).unwrap();
            }
        }
    }

    // Generating the manifest test vectors
    {
        let output_directory = output_directory.join("manifests");
        std::fs::create_dir_all(&output_directory).unwrap();

        let manifest_directory =
            Path::new("../radix-engine-toolkit-json/tests/manifests")
                .canonicalize()
                .unwrap();

        for entry in walkdir::WalkDir::new(&manifest_directory) {
            let path = entry.unwrap().path().canonicalize().unwrap();
            if path.extension().and_then(|str| str.to_str()) != Some("rtm") {
                continue;
            }

            let output_directory = PathBuf::from_str(
                &path.parent().unwrap().to_str().unwrap().replace(
                    manifest_directory.to_str().unwrap(),
                    output_directory.to_str().unwrap(),
                ),
            )
            .unwrap();
            std::fs::create_dir_all(&output_directory).unwrap();

            let output_path = output_directory.join(path.file_name().unwrap());
            std::fs::copy(path, output_path).unwrap();
        }
    }

    // Generating the OpenAPI spec
    {
        let output_directory = output_directory.join("function_spec");
        std::fs::create_dir_all(&output_directory).unwrap();

        let mut spec = generate_function_spec();
        spec.info.title = "Radix Engine Toolkit".to_string();

        let output_path = output_directory.join("spec.yaml");
        let serialized = serde_yaml::to_string(
            &convert_open_api_adts_to_discriminated_unions(&spec),
        )
        .unwrap();
        std::fs::write(output_path, serialized).unwrap();
    }
}
