cd src/rust
wasm-pack build --release --out-dir pkg

cd ../..
sh wasm-replace.sh
yarn install
yarn build
