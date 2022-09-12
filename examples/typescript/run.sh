SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

(cd $SCRIPT_DIR/../..; ./build.sh) # Build transaction library
tsc # Build the client code
( # Copy required files to the out dir
    cp $SCRIPT_DIR/wasm/transaction_library.wasm $SCRIPT_DIR/out
    cp $SCRIPT_DIR/../complex.rtm $SCRIPT_DIR/out
)
(cd out; node main.js) # Run the client code