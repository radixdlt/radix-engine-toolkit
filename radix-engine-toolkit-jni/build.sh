echo "Building the library";

for target in "x86_64-unknown-linux-gnu" "aarch64-apple-darwin" "x86_64-apple-darwin"
do
    echo "Building for target '$target'"

    cargo build \
        --target $target \
        --target-dir ./target \
        --release
done
