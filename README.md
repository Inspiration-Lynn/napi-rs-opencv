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

## [napi-rs](https://crates.io/crates/napi)配置

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

## napi-rs opencv binding配置

- 具体步骤：[twistedfall/opencv-rust: Rust bindings for OpenCV 3 & 4 (github.com)](https://github.com/twistedfall/opencv-rust)

### 手动编译安装opencv(含contrib)

- 参考：[OpenCV: Installation in Linux](https://docs.opencv.org/4.5.5/d7/d9f/tutorial_linux_install.html)

### 配置opencv环境变量

>[Rust OpenCV bindings](https://github.com/twistedfall/opencv-rust)要求

>Getting the OpenCV library (linux):
>
>- build OpenCV manually and set up the following environment variables prior to building the project with `opencv` crate:
>  - `PKG_CONFIG_PATH` for the location of `*.pc` files or `OpenCV_DIR` for the location of `*.cmake` files
>  - `LD_LIBRARY_PATH` for where to look for the installed `*.so` files during runtime

```bash
vim ~/.bashrc
```

在末尾写入：

```bash
export OpenCV_DIR=/usr/local/lib/cmake/opencv4
export LD_LIBRARY_PATH=/usr/local/lib
```

使环境变量生效：

```bash
source ~/.bashrc
```

> **[troubleshooting]**
>
> 参考：[unresolved imports `opencv::highgui`, `opencv::videoio` · Issue #230 · twistedfall/opencv-rust (github.com)](https://github.com/twistedfall/opencv-rust/issues/230)
>
> Specifying the OpenCV *build* directory as *install* directory falsely:
>
> ```
> OpenCV_DIR = Some("/home/rprata/Projects/opencv/build/")
> ```
>
> this is currently not supported. Install OpenCV after building and then specify the directory where it was installed to.

### 安装clang

Additionally, please make sure to install `clang` package or its derivative that contains `libclang.so` and `clang` binary.

- Debian, Ubuntu: `clang` and `libclang-dev`

```bash
 sudo apt-get update -y
 sudo apt-get install -y libclang-dev
 sudo apt install clang
```

验证安装成功：

```bash
lynn@DESKTOP-M96JUD3:~$ clang --version
clang version 10.0.0-4ubuntu1
Target: x86_64-pc-linux-gnu
Thread model: posix
InstalledDir: /usr/bin
lynn@DESKTOP-M96JUD3:~$ clang++ --version
clang version 10.0.0-4ubuntu1
Target: x86_64-pc-linux-gnu
Thread model: posix
InstalledDir: /usr/bin
```

### 安装ninja
