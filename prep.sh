BASEDIR=$(dirname "$0")
cd $BASEDIR

python3 add_license.py
./format.sh
./test.sh
(cd generator; cargo run)