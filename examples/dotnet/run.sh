SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

(cd $SCRIPT_DIR/../..; ./build.sh) # Build transaction library
dotnet build # Build the client code
(cd $SCRIPT_DIR/bin/Debug/net6.0; ./dotnet) # Run the client code