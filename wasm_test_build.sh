cd wasm_test
cargo build --release --target wasm32-unknown-unknown
cd ..
cp wasm_test/target/wasm32-unknown-unknown/release/wasm_test.wasm ./out/wasm_test.wasm