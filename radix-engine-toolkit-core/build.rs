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

use cargo_toml::{Dependency, Manifest};
use std::env;
use std::path::Path;

fn main() {
    let manifest_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
    let manifest = Manifest::from_path(manifest_path).unwrap();

    let scrypto_dependency = manifest.dependencies.get("scrypto").unwrap();
    let string = match scrypto_dependency {
        Dependency::Simple(version) => format!("version={version}"),
        Dependency::Inherited(_) => panic!("Inherited dependency is not supported"),
        Dependency::Detailed(detailed) => {
            if let Some(ref version) = detailed.version {
                format!("version={version}")
            } else if let Some(ref branch) = detailed.branch {
                format!("branch={branch}")
            } else if let Some(ref tag) = detailed.tag {
                format!("tag={tag}")
            } else if let Some(ref rev) = detailed.rev {
                format!("rev={rev}")
            } else {
                panic!("Can't find version of Scrypto dependency")
            }
        }
    };
    println!("cargo:rustc-env=SCRYPTO_DEPENDENCY={}", string);
}
