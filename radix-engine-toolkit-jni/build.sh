echo "Building the library";

# Arm-based Macs
echo "Building for target: aarch64-apple-darwin"
cargo +nightly \
    build \
    -Z build-std=std,panic_abort \
    -Z build-std-features=panic_immediate_abort \
    --target aarch64-apple-darwin \
    --release

# Intel-based Macs
echo "Building for target: x86_64-apple-darwin"
cargo +nightly \
    build \
    -Z build-std=std,panic_abort \
    -Z build-std-features=panic_immediate_abort \
    --target x86_64-apple-darwin \
    --release

# x86 Linux

# Reminder: Before doing this, I needed to install a cross compiler from Brew. The following are the
# commands I used:
# $ brew tap SergioBenitez/osxct
# $ brew install x86_64-unknown-linux-gnu
# There is a chance that you might not need this if you are on a non-mac device. See the following 
# stack overflow answer for more information: 
# https://stackoverflow.com/questions/40424255/cross-compilation-to-x86-64-unknown-linux-gnu-fails-on-mac-osx
echo "Building for target: x86_64-unknown-linux-gnu"
cargo +nightly \
    build \
    -Z build-std=std,panic_abort \
    -Z build-std-features=panic_immediate_abort \
    --target x86_64-unknown-linux-gnu \
    --release

echo "Building for target: x86_64-pc-windows-gnu"
cargo +nightly \
    build \
    -Z build-std=std,panic_abort \
    -Z build-std-features=panic_immediate_abort \
    --target x86_64-pc-windows-gnu \
    --release