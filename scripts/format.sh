SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

# Sadly, rustfmt sometimes can't format lines when they get too long. This could
# be a long variable name that make rustfmt choke, it could be a line being not
# easy to break up, or it could be any other magnitude of reasons. This is a 
# well known issue in Rustfmt and there's currently an issue open to fix it:
# https://github.com/rust-lang/rustfmt/issues/3863. In the mean time, a nice
# "hack" for this problem is to format the file multiple times with decreasing
# allowed max_width until we get to the desired max_width. 
for max_width in `seq 200 -10 80`;
do
    cargo fmt --all \
        --manifest-path="$SCRIPT_DIR/../Cargo.toml" \
        -- --config max_width=$max_width
done
