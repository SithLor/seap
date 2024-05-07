cd add
cargo build --release --target wasm32-unknown-unknown
cd ..
cp add/target/wasm32-unknown-unknown/release/add.wasm ./out/add.wasm