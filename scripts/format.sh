SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

cd $SCRIPT_DIR/..
cargo fmt --all -- --config max_width=120
cargo fmt --all -- --config max_width=110
cargo fmt --all -- --config max_width=100
cargo fmt --all -- --config max_width=90
cargo fmt --all -- --config max_width=80