SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

# Increase the heapsize used by kotlinc. This is required to be able to compile
# the kotlin files.
export JAVA_OPTS="-Xmx8g"

python3 $SCRIPT_DIR/add_license.py
$SCRIPT_DIR/format.sh
$SCRIPT_DIR/test.sh
$SCRIPT_DIR/uniffi_bindgen.sh
$SCRIPT_DIR/typeshare.sh
(cd $SCRIPT_DIR/../crates/generator; cargo run)