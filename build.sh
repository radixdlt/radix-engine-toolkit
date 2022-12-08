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

# The directory where this script lives. Used to provide an absolute path ot the crates to build
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

# The directory that all of the builds should go to
BUILD_DIR="$SCRIPT_DIR/build"
rm -rf $BUILD_DIR
mkdir $BUILD_DIR

# This is a list of all of all of the crates that we would like to build. 
CRATES_TO_BUILD="radix-engine-toolkit-native radix-engine-toolkit-wasm radix-engine-toolkit-jni"
for CRATE_NAME in $CRATES_TO_BUILD
do
    CRATE_DIR="$SCRIPT_DIR/$CRATE_NAME"
    echo "Building $CRATE_NAME"
    cd $CRATE_DIR
    ./build.sh 

    for TARGET_NAME in `find "$CRATE_DIR/target" -name "*-*" -maxdepth 1 -exec basename {} \;`; do
        for EXTENSION in "dylib" "a" "dll" "so" "wasm"; do
            for MATCHING_FILE_PATH in `find "$CRATE_DIR/target/$TARGET_NAME/release" -name "*.$EXTENSION" -maxdepth 1`; do 
                cp $MATCHING_FILE_PATH "$BUILD_DIR/$CRATE_NAME-$TARGET_NAME.$EXTENSION"
            done
        done
    done
done

cp -r $SCRIPT_DIR/radix-engine-toolkit-native/target/iOS/RadixEngineToolkit.xcframework $BUILD_DIR/RadixEngineToolkit.xcframework