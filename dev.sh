cd src/rust
cargo watch -i .gitignore -i "pkg/*" -s "wasm-pack build --out-dir src/rust/pkg && sh /srv/app/wasm-replace.sh && yarn install && yarn start"
