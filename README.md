After running the simulator:

Add to Cargo.toml:
path="src/wasm_exports.rs"

cargo add wasm-bindgen
npm install http-server
wasm-pack build --target web
http-server .