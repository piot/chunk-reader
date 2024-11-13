# 💾 Chunk Reader 

Chunk Reader is a Rust crate designed to facilitate asynchronous asset loading across different platforms. It 
provides a unified interface to fetch assets either from the local file system or over HTTP in 
WebAssembly (WASM) environments. Additionally, it includes a debug module to simulate asset loading delays, 
aiding in testing and development.

## ✨ Features

- **Asynchronous Asset Loading**: Load assets without blocking the main thread, ensuring smooth application performance.
- **Cross-Platform Support**: Seamlessly handle asset loading for both native applications and WASM targets.
- **Debugging Tools**: Introduce intentional delays in asset loading to test application responsiveness and error handling.

## 📦 Installation

Add chunk-reader to your project’s Cargo.toml:

```toml
[dependencies]
chunk-reader = "0.0.1"
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contents credits

- [Kenny.nl](https://kenney.nl/assets/platformer-art-deluxe) "platform art deluxe"
