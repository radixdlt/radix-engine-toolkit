#!/bin/bash

library_name="ret-kotlin"
src=$library_name/lib/src/main/kotlin
res=$library_name/lib/src/main/resources
package=com/radixdlt/ret

artifacts=../../artifacts

echo "Bootstrap project $library_name"
mkdir $library_name
mkdir extracted
mkdir -p $src/$package
mkdir -p $res

# Extracting .kt file
tar -xzf $artifacts/UniFFI-Bindings/UniFFI-Bindings.tar.gz --directory=extracted
mv extracted/output/$package/*.kt $src/$package/RET.kt

crate_name=radix-engine-toolkit-uniffi
jna_architectures=(
  "darwin-aarch64"
  "darwin-x86-64"
  "linux-armel"
  "linux-x86-64"
  "win32-x86-64"
)
ret_names=(
  "aarch64-apple-darwin"
  "x86_64-apple-darwin"
  "aarch64-unknown-linux-gnu"
  "x86_64-unknown-linux-gnu"
  "x86_64-pc-windows-gnu"
)
suffixes=(
  "dylib"
  "dylib"
  "so"
  "so"
  "dll"
)

for (( i=0; i<${#jna_architectures[@]}; i++ ));
do
  arch_name=${jna_architectures[$i]}
  ret_name=${ret_names[$i]}
  suffix=${suffixes[$i]}

  echo "Extracting for architecture $arch_name"

  mkdir extracted/"$arch_name"
  tar -xzf $artifacts/"$crate_name"-"$ret_name"/"$ret_name".tar.gz --directory=extracted/"$arch_name"
  mkdir $res/"$arch_name"
  mv extracted/"$arch_name"/*."$suffix" $res/"$arch_name"/libuniffi_radix_engine_toolkit_uniffi."$suffix"
done

rm -rf extracted

# Initialise Gradle project
cp build.gradle.kts $library_name/lib/build.gradle.kts
cp settings.gradle.kts $library_name/settings.gradle.kts

# Extract version from Cargo.toml
toml=../../radix-engine-toolkit-uniffi/Cargo.toml
ret_version=$(grep -m 1 version $toml | awk -F= '{print $2}' | tr -d '" ')
commit_hash=$(git rev-parse --short HEAD)
version="$ret_version-$commit_hash"
echo -e "ret-version=$version" >> $library_name/gradle.properties