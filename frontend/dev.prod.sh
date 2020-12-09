cd /srv/app/src/rust
wasm-pack build --release --out-dir /srv/app/src/rust/pkg && sh /srv/app/wasm-replace.sh && yarn install && yarn build
yarn global add serve
echo "about to serve"
serve -s /srv/app/build -l 3000
