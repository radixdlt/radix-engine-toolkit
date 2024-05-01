#
# Update version before release
#
# How to use: edit 'old_version' and 'new_version' variables, run script from current folder.
#
# Requires cargo-edit utility, to install run command: cargo install cargo-edit
#

old_version="1.0.10"
new_version="2.0.1"

cd ..
cargo set-version $new_version
sed -i "" -e "s/$old_version/$new_version/g" crates/radix-engine-toolkit-uniffi/tests/bindings/example.kts crates/radix-engine-toolkit-uniffi/tests/bindings/example.py crates/radix-engine-toolkit-uniffi/tests/bindings/example.swift crates/radix-engine-toolkit-uniffi/tests/bindings/example_test.go
