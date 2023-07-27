#!/bin/bash

library_name="ret-android"
src=$library_name/lib/src/main/kotlin
jni=$library_name/lib/src/main/jniLibs
package=com/radixdlt/ret

artifacts=../../artifacts

echo "Bootstrap project $library_name"
mkdir $library_name
mkdir -p $src/$package
mkdir -p $jni

# Copying .kt file
mv $artifacts/uniffi-bindings/$package/*.kt $src/$package/RET.kt
test -e $src/$package/RET.kt || exit 1

crate_name=radix-engine-toolkit-uniffi
jna_architectures=(
  "arm64-v8a"
  "armeabi-v7a"
)
target_triples=(
  "aarch64-linux-android"
  "armv7-linux-androideabi"
)

for (( i=0; i<${#jna_architectures[@]}; i++ ));
do
  arch_name=${jna_architectures[$i]}
  target_triple=${target_triples[$i]}

  echo "Extracting for architecture $arch_name"

  mkdir $jni/"$arch_name"
  mv $artifacts/"$crate_name"-"$target_triple"/*.so $jni/"$arch_name"/libuniffi_radix_engine_toolkit_uniffi.so
  test -e $jni/"$arch_name"/libuniffi_radix_engine_toolkit_uniffi.so || exit 1
done

# Initialise Gradle project
cp build.gradle.kts $library_name/build.gradle.kts
cp settings.gradle.kts $library_name/settings.gradle.kts
cp lib.build.gradle.kts $library_name/lib/build.gradle.kts
cp consumer-rules.pro $library_name/lib/consumer-rules.pro

# Extract version from Cargo.toml
toml=../../radix-engine-toolkit-uniffi/Cargo.toml
ret_version=$(grep -m 1 version $toml | awk -F= '{print $2}' | tr -d '" ')
commit_hash=$(git rev-parse --short HEAD)
version="$ret_version-$commit_hash"
echo -e "ret-version=$version" >> $library_name/gradle.properties