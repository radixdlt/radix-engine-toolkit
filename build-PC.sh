echo "Building the library";

# Arm-based Macs
cargo +nightly \
    build \
    -Z build-std=std,panic_abort \
    -Z build-std-features=panic_immediate_abort \
    --target aarch64-apple-darwin \
    --features jni \
    --release

# Intel-based Macs
cargo +nightly \
    build \
    -Z build-std=std,panic_abort \
    -Z build-std-features=panic_immediate_abort \
    --target x86_64-apple-darwin \
    --features jni \
    --release

# x86 Linux

# Reminder: Before doing this, I needed to install a cross compiler from Brew. The following are the
# commands I used:
# $ brew tap SergioBenitez/osxct
# $ brew install x86_64-unknown-linux-gnu
# There is a chance that you might not need this if you are on a non-mac device.
cargo +nightly \
    build \
    -Z build-std=std,panic_abort \
    -Z build-std-features=panic_immediate_abort \
    --target x86_64-unknown-linux-gnu \
    --features jni \
    --release