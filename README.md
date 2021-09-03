# [Asteroid WASM - Click here to here](https://roberts-ivanovs.github.io/asteroid-wasm/)

A small hobby/ school-project. Inspired by the ATARI Asteroid game, this is an
implementation in React/Rust/WebGL/Wasm. Feel free to use the code as you please.

![Tutorial on what to click](docs/game.png)

The SPA also includes some other sections but they are nowhere as interesting as the game itself.

## Main Frameworks/Libraries/Packages


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



## Note

For the school project I ahd toa dd a lot of useless other packages and functionality, like a backend, a graph database, simple data storing, etc.

All of that has been removed for this static site version. But if you wish to check that out as well -- [here's the bloated version](https://github.com/roberts-ivanovs/asteroid-wasm/tree/v1).
