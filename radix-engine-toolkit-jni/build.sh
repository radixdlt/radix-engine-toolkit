# Licensed to the Apache Software Foundation (ASF) under one
# or more contributor license agreements.  See the NOTICE file
# distributed with this work for additional information
# regarding copyright ownership.  The ASF licenses this file
# to you under the Apache License, Version 2.0 (the
# "License"); you may not use this file except in compliance
# with the License.  You may obtain a copy of the License at

#   http://www.apache.org/licenses/LICENSE-2.0

# Unless required by applicable law or agreed to in writing,
# software distributed under the License is distributed on an
# "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
# KIND, either express or implied.  See the License for the
# specific language governing permissions and limitations
# under the License.

echo "Building the library";

CARGO_TARGETS="aarch64-apple-darwin x86_64-apple-darwin"
CROSS_TARGETS="x86_64-unknown-linux-gnu x86_64-pc-windows-gnu aarch64-linux-android armv7-linux-androideabi i686-linux-android"

for target in $CARGO_TARGETS
do
    echo "Building for target '$target'"

    cargo build \
        --target $target \
        --target-dir ./target \
        --release
done

for target in $CROSS_TARGETS
do
    echo "Building for target '$target'"

    cross build \
        --target $target \
        --target-dir ./target \
        --release
done