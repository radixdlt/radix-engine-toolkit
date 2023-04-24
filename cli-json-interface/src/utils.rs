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

use serde::Serialize;

pub fn pretty_print<S: Serialize, O: std::io::Write>(obj: &S, out: &mut O) -> Result<()> {
    let buffer = {
        let buffer = Vec::new();
        let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
        let mut serializer = serde_json::Serializer::with_formatter(buffer, formatter);
        obj.serialize(&mut serializer).unwrap();

        let mut bytes = serializer.into_inner();
        bytes.push(b'\n');
        bytes
    };
    out.write_all(&buffer)?;
    Ok(())
}
