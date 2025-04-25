## 🔧 WebAssembly (WASM) Setup

When writing Rust code for WebAssembly or embedded systems, it's common to use the following attributes, since the standard library and runtime may not be available:

```rust
#![no_main]
#![no_std]
```

- `#![no_main]`: Tells the compiler **not** to use the usual `main()` function as the entry point. This is useful in low-level contexts like kernels, microcontrollers, or WASM modules where you define a custom entry point.
- `#![no_std]`: Prevents linking the Rust standard library. Used in constrained environments (e.g., embedded systems, bootloaders, or WASM) where `std` isn't available.

### 😱 Panic Handler

```rust
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}
```

- Required when using `#![no_std]` because the standard panic behavior from `std` is unavailable.
- This implementation simply loops forever on panic — a common pattern in embedded or low-level Rust code.

### 📺 `Display` Struct and Static Instance

```rust
pub struct Display {
    pixels: [Pixel; WIDTH * HEIGHT],
}

static mut display: Display = Display {
    pixels: [0; WIDTH * HEIGHT],
};
```

- `Display` holds an array of pixels.
- `display` is a `static mut` instance initialized with all pixels set to zero.
- Accessing or mutating static variables requires `unsafe` due to potential data races in multi-threaded environments.

### 🌐 External Interface Function

```rust
#[no_mangle]
pub fn get_display() -> *mut Display {
    unsafe {
        &mut display
    }
}
```

- `#[no_mangle]`: Prevents the compiler from renaming the function, allowing it to be accessible from other languages or systems (e.g., via WASM).
- Returns a mutable pointer to the `display` struct, likely so an external renderer or system can access and manipulate the display buffer.

### 🚀 Compiling a Rust Program to WebAssembly

```bash
rustc -C opt-level=3 --target wasm32-unknown-unknown game.rs
```

- 🦀 `rustc`: Runs the Rust compiler.

- ⚙️ `-C opt-level=3`: Applies **optimization level 3** — the highest level for faster, smaller code (great for release builds).

- 🌐 `--target wasm32-unknown-unknown`: Sets the target to WebAssembly:
  - 🧱 `wasm32`: 32-bit WebAssembly architecture.
  - ❓`unknown-unknown`: No OS, no ABI — a barebones, portable WASM target.

- 📄 `game.rs`: Your Rust source file to compile.


### 🔗 External Function Declaration

```rust
extern "C" {
    fn imported_func(x: i32) -> i32;
}
```

- Declares an external function, typically provided by the host environment when running in WASM or other embedded contexts.
- `extern "C"` specifies the C calling convention for compatibility with non-Rust code.


## Web Assembly
WebAssembly is a portable binary format and virtual machine that provides a safe, fast, and platform-independent runtime for high-level languages on the web. It was developed by major browser vendors, including Mozilla, Microsoft, Google, and Apple, and became a W3C recommendation in 2019

Unlike JavaScript, which is interpreted and dynamically typed, WebAssembly is statically typed and compiled ahead of time, resulting in faster execution. It operates within a secure sandboxed environment in the browser, ensuring safety and stability

### 🚀 **Why Use WebAssembly?**

WebAssembly (Wasm) is gaining popularity for a number of powerful reasons:

- **Blazing-Fast Performance:** Wasm runs at near-native speeds in the browser, making it ideal for performance-heavy applications like games, video editing, and complex data visualizations.

- **Cross-Language Support:** Code written in languages like C, C++, Rust, and Go can be compiled to WebAssembly, allowing developers to repurpose existing codebases and work in the language they’re most comfortable with.

- **Portability:** WebAssembly modules are designed to run on any platform with a compatible virtual machine, making it perfect for building cross-platform solutions.

- **Enhanced Security:** Its sandboxed execution environment helps isolate code, reducing vulnerabilities and creating a more secure runtime.

- **JavaScript's Perfect Partner:** Wasm doesn’t replace JavaScript—it complements it. You can use both together in the same application, taking advantage of each where it performs best.

### 📚 References

- [Precise semantics of `no_mangle` – Rust Internals](https://internals.rust-lang.org/t/precise-semantics-of-no-mangle/4098)  
- [`extern` keyword – Rust Documentation](https://doc.rust-lang.org/std/keyword.extern.html)  
- [Accessing/modifying mutable static variables – Rust Book](https://doc.rust-lang.org/book/ch20-01-unsafe-rust.html#accessing-or-modifying-a-mutable-static-variable)  
- [`no_std` and `no_main` explained – ImplRust](https://pico.implrust.com/core-concepts/no-std-main.html)
- [WebAssembly - Wikipedia](https://en.wikipedia.org/wiki/WebAssembly)
- 
