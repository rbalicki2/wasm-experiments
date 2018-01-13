cargo build --release --target=wasm32-unknown-unknown
mkdir -p target/static
wasm-gc target/wasm32-unknown-unknown/release/wasm_minimal.wasm \
  target/static/wasm-minimal.wasm

