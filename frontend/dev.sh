cd /srv/app/src/rust
cargo watch -i .gitignore -i "pkg/*" -s "wasm-pack build --out-dir /srv/app/src/rust/pkg && sh /srv/app/wasm-replace.sh && yarn start"
