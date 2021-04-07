cargo build

cargo build --target wasm32-unknown-unknown

wasm-bindgen --target web --out-dir static --out-name wasm target/wasm32-unknown-unknown/debug/chora_io_client.wasm --no-typescript

miniserve ./static --index index.html