SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

mkdir -p $SCRIPT_DIR/generator/output/typeshare

typeshare $SCRIPT_DIR --lang=typescript --output-file=$SCRIPT_DIR/generator/output/typeshare/generated.ts
prettier --write $SCRIPT_DIR/generator/output/typeshare/*.ts
