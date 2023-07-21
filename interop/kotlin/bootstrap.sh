#!/bin/bash

library_name="ret-kotlin"
src=$library_name/lib/src/main/kotlin
res=$library_name/lib/src/main/resources
package=com/radixdlt/ret

artifacts=../../artifacts

echo "Bootstrap project $library_name"
mkdir $library_name
mkdir -p $src/$package
mkdir -p $res

# Copying .kt file
mv $artifacts/uniffi-bindings/$package/*.kt $src/$package/RET.kt
test -e $src/$package/RET.kt || exit 1

crate_name=radix-engine-toolkit-uniffi
jna_architectures=(
  "darwin-aarch64"
  "darwin-x86-64"
  "linux-armel"
  "linux-x86-64"
  "win32-x86-64"
)
target_triples=(
  "aarch64-apple-darwin"
  "x86_64-apple-darwin"
  "aarch64-unknown-linux-gnu"
  "x86_64-unknown-linux-gnu"
  "x86_64-pc-windows-gnu"
)
file_names=(
  "libuniffi_radix_engine_toolkit_uniffi.dylib"
  "libuniffi_radix_engine_toolkit_uniffi.dylib"
  "libuniffi_radix_engine_toolkit_uniffi.so"
  "libuniffi_radix_engine_toolkit_uniffi.so"
  "radix_engine_toolkit_uniffi.dll"
)

for (( i=0; i<${#jna_architectures[@]}; i++ ));
do
  arch_name=${jna_architectures[$i]}
  target_triple=${target_triples[$i]}
  file_name=${file_names[$i]}

  echo "Extracting for architecture $arch_name"

  mkdir $res/"$arch_name"
  mv $artifacts/"$crate_name"-"$target_triple"/"$file_name" $res/"$arch_name"/"$file_name"
  test -e $res/"$arch_name"/"$file_name" || exit 1
done

# Initialise Gradle project
cp build.gradle.kts $library_name/lib/build.gradle.kts
cp settings.gradle.kts $library_name/settings.gradle.kts

# Extract version from Cargo.toml
toml=../../radix-engine-toolkit-uniffi/Cargo.toml
ret_version=$(grep -m 1 version $toml | awk -F= '{print $2}' | tr -d '" ')
commit_hash=$(git rev-parse --short HEAD)
version="$ret_version-$commit_hash"
echo -e "ret-version=$version" >> $library_name/gradle.properties