set -x

export target="aarch64-apple-darwin"
export target_replaced=$(echo $target | tr '[:lower:]' '[:upper:]' | sed 's/-/_/g')

export "CARGO_TARGET_"$target_replaced"_LINKER"="hello world"