# Asteroid WASM

A small hobby/ school-project. Inspired by the ATARI Asteroid game, this is an
implementation in React/Rust/WebGL/Wasm. Feel free to use the code as you please.

![Tutorial on what to click](docs/game.png)

The SPA also includes some other sections but they are nowhere as interesting as the game itself.

## Main Frameworks/Libraries/Packages

ExpressJS

- Node dev server via Docker LTS alpine image
- TypeScript/ESlint with very opinionated rules

React

- Create React App
- Hot reload
- TypeScript/ESlint with very opinionated rules
- SASS support
- WebAssembly support
- Integration with the Rust-Wasm module
- RUST INTEGRATION:
  - Compilation on code change straight to WASM with all type definitions
  - Rust 2018 edition
  - wasm-bindgen for WEB APIs



### Containers, Services and Ports

| Container | Service | Host Port | Docker Port |
| --------- | ------- | --------- | ----------- |
| dev-react | react   | 3001      | 3000        |
