rm -rf node_modules/wasm-app
mkdir -p node_modules
pwd
cp -r src/rust/pkg node_modules/wasm-app
echo "copied files"
