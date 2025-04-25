## Browser Game in RUST

## Quick Start
It is assumed that you are using [rustup](https://rustup.rs/) for managing your local Rust installation

1. Make sure you have `wasm32-unknown-unknown` target installed:
    ```bash
    rustup target add wasm32-unknown-unknown
    ```
2. Build the game
    ```bash
    make -B
    ```
3.  Play the game
    ```bash
    python3 -m http.server 6969
    ```

Open address http://127.0.0.1:6969 in browser

### Game Controls
- **Mouse Click**: Shoot enemies  
- **Spacebar**: Pause the game during play or restart after a game over  
- **Mouse Movement**: Move the player by moving the cursor  

### 🔗 References

- [WebAssembly Official Site](https://webassembly.org/) – Learn more about WebAssembly, its features, and use cases.  
- [Rust Official Documentation](https://doc.rust-lang.org/rustc/) – Comprehensive guide to Rust’s compiler and toolchain.  
- [Rustc Targets Overview](https://doc.rust-lang.org/rustc/targets/index.html) – Explore supported compilation targets for building cross-platform Rust applications.  
