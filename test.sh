# The release flag is required when testing the Radix Engine Toolkit since some of the tests require
# using the WASM Wrapper which should be using the release builds of the toolkit for the tests.
cargo test --release