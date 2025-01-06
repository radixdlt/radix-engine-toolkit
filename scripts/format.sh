SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

cd $SCRIPT_DIR/..
cargo +nightly-2024-02-01 fmt -- --config max_width=120
cargo +nightly-2024-02-01 fmt -- --config max_width=110
cargo +nightly-2024-02-01 fmt -- --config max_width=100
cargo +nightly-2024-02-01 fmt -- --config max_width=90
cargo +nightly-2024-02-01 fmt -- --config max_width=80