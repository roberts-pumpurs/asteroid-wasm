cd src/rust
cargo watch -i .gitignore -i "pkg/*" -s "wasm-pack build --out-dir src/rust/pkg && cd ../.. && sh wasm-replace.sh && yarn install && yarn start"
