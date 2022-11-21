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