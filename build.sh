echo "Building the library";
cargo build --target wasm32-unknown-unknown --release

echo "Generating the OpenAPI models for the examples";
python3 ./scripts/generate-openapi-models.py

echo "Done!"