cd src/rust
wasm-pack build --release --out-dir pkg && sh ../../wasm-replace.sh && yarn install && yarn build
