# napi-rs-opencv-cross_compile

- 参考：[rust-cross](https://github.com/japaric/rust-cross)

## 1. Rust配置

1. 安装Rust

Windows 的 Linux 子系统（WSL）下：

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. 更换版本

troubleshooting：编译遇到

```bash
ld.lld: error: unable to find library -lgcc_s
```

因为要改变基本库（如libgcc），因此需要更换成不稳定的nightly版本

查看cargo 版本

```bash
$ cargo version
```

切换到nightly版本的cargo：

```bash
rustup default nightly
```

```bash
$ cargo version
cargo 1.60.0-nightly (95bb3c92b 2022-01-18)
```

## 2. 配置交叉编译工具链

1. 拷贝交叉编译所需工具clang/ musl/到~/env/目录下
2. 加入可执行权限

```bash
~/env/clang/ohos/linux-x86_64/llvm/bin$ sudo chmod +x *
```

```bash
~/env/clang/ohos/linux-x86_64/llvm/bin$ ls -la
```

4. 配置环境变量

```bash
vim ~/.bashrc
source ~/.bashrc
```

## 3. 配置opencv napi-rs binding 

opencv4(含contrib)交叉编译生成的动态库位置：hit@hit-tg-node-1:~/wxc/OpenCV/out/lib

将/out/lib拷贝到wsl2-/home/lynn/opencv目录下

> 将lib.tar拷贝到指定位置，然后`tar -xvf lib.tar`解压
>
> 这样动态库是包含软链接的（虽然不知道需不需要）

添加环境变量（opencv包含contrib）

```bash
vim ~/.bashrc
```

在末尾写入：

```bash
export OpenCV_DIR=/home/lynn/opencv/out/lib/cmake/opencv4
export LD_LIBRARY_PATH=/home/lynn/opencv/out/lib
```

>[Rust OpenCV bindings](https://github.com/twistedfall/opencv-rust)要求

>Getting the OpenCV library (linux):
>
>- build OpenCV manually and set up the following environment variables prior to building the project with `opencv` crate:
>  - `PKG_CONFIG_PATH` for the location of `*.pc` files or `OpenCV_DIR` for the location of `*.cmake` files
>  - `LD_LIBRARY_PATH` for where to look for the installed `*.so` files during runtime

使环境变量生效

```bash
source ~/.bashrc
```

## 4. 交叉编译

进入目标文件夹rust-opencv

1. .cargo/config.toml配置如下：

```toml
[build]
target = "aarch64-unknown-linux-musl"

[env]
CXX = "/home/lynn/env/clang/ohos/linux-x86_64/llvm/bin/clang++"
CC = "/home/lynn/env/clang/ohos/linux-x86_64/llvm/bin/clang"
CFLAGS = "--sysroot=/home/lynn/env/musl --target=aarch64-linux-ohosmusl"
CXXFLAGS = "--sysroot=/home/lynn/env/musl --target=aarch64-linux-ohosmusl"
CXXSTDLIB = "c++"

[target.aarch64-unknown-linux-musl]
linker = "/home/lynn/env/clang/ohos/linux-x86_64/llvm/bin/ld.lld"

rustflags = [
    "-Ctarget-feature=-crt-static",
    "-Clink-self-contained=off",
    "-L/home/lynn/env/musl/usr/lib/aarch64-linux-ohosmusl",
    "-L/home/lynn/env/clang/ohos/linux-x86_64/llvm/lib/aarch64-linux-ohosmusl",
    "-L/home/lynn/env/clang/ohos/linux-x86_64/llvm/lib/aarch64-linux-ohosmusl/c++",
    "-L/home/lynn/env/clang/ohos/linux-x86_64/llvm/lib"
]

# "-Cembed-bitcode=yes",
#    "-Clink-arg=--gc-sections",
#    "-Clto",

[unstable]
build-std = ["core", "compiler_builtins", "alloc", "std", "panic_unwind"]
build-std-features = ["llvm-libunwind"]

```

```bash
cargo check
```

```bash
cargo build --release
```

生成的动态库在//target/aarch64-unknown-linux-musl/release/下

```bash
lynn@DESKTOP-KIMSKHG:~/ohos-opencv/rust-opencv/target/aarch64-unknown-linux-musl/release$ file libjsopencv.so
libjsopencv.so: ELF 64-bit LSB shared object, ARM aarch64, version 1 (SYSV), dynamically linked, not stripped
```







不知道需不需要：

[安装其他目标 (target)](https://rustwiki.org/zh-CN/edition-guide/rust-2018/rustup-for-managing-rust-versions.html#安装其他目标-target)

安装目标平台所需musl-libc工具链？？

```
rustup target add aarch64-unknown-linux-musl
```

