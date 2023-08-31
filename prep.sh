BASEDIR=$(dirname "$0")
cd $BASEDIR

python3 add_license.py
./format.sh
./test.sh
./uniffi_bindgen.sh
./typeshare.sh
(cd generator; cargo run)