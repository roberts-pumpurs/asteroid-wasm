cd src/rust
mkdir -f pkg
wasm-pack build --release --out-dir pkg && sh ../../wasm-replace.sh && yarn install && yarn build
# yarn global add serve
# echo "about to serve"
# serve -s /srv/app/build -l 3000
