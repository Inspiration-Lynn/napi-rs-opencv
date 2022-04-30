# Rust-napi

环境：

| 操作系统 | wsl2-Ubuntu20.04 |
| -------- | ---------------- |
|          |                  |
|          |                  |
|          |                  |

过程：

```bash
cargo new marker-detection
```

## [napi](https://crates.io/crates/napi)配置

[Cargo Targets - The Cargo Book (rust-lang.org)](https://doc.rust-lang.org/cargo/reference/cargo-targets.html)

## 第一个napi-rs：fibonacci

Cargo.toml

```toml
[package]
name = "marker-detection"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[lib]
crate-type = ["cdylib"]

[dependencies]
napi = "2"
napi-derive = "2"
```

src/lib.rs

```rust
use napi_derive::napi;

#[napi]
fn fibonacci(n: u32) -> u32 {
  match n {
    1 | 2 => 1,
    _ => fibonacci(n - 1) + fibonacci(n - 2),
  }
}
```
构建：
```bash
cargo check
cargo build --release
```

生成的动态库：

```bash
lynn@DESKTOP-M96JUD3:~/openharmony/marker-detection/target/release$ file libmarker_detection.so
libmarker_detection.so: ELF 64-bit LSB shared object, x86-64, version 1 (SYSV), dynamically linked, BuildID[sha1]=820f6cfcd1675b01e23d18543ad8ad9b388, with debug_info, not stripped
```

node测试：

将libmarker_detection.so移动到marker-detection-test目录下，重命名为libmarker_detection.node

node测试程序fibonacci.js：

```javascript
const { fibonacci } = require("./libmarker_detection.node");

console.log(fibonacci(11));
```

运行：

```bash
node fibonacci.js
```

重命名为libmarker_detection.node.fib，保存在lib目录下
