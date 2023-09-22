import os

LICENSE: str ="""
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
"""

def main() -> None:
    license: str = LICENSE.strip()
    for (root_path, _, file_names) in os.walk(os.path.dirname(os.path.realpath(__file__))):
        for file_name in file_names:
            if not file_name.endswith('.kt'):
                continue

            file_path: str = os.path.join(root_path, file_name)

            with open(file_path, 'r') as file:
                content: str = file.read()

            if license not in content:
                content = license + '\n\n' + content

                with open(file_path, 'w') as file:
                    file.write(content)

if __name__ == "__main__":
    main()