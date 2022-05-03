# napi-rust

## 是什么是napi?

- Node.js原生模块开发变迁史参考：[从暴力到 NAN 再到 NAPI——Node.js 原生模块开发方式变迁](https://xcoder.in/2017/07/01/nodejs-addon-history/)

- Native Addon

> *Addons* are **dynamically-linked shared objects written in C/C++**. The [`require()`](http://nodejs.cn/api/modules.html#requireid) function can load addons as ordinary Node.js modules. Addons provide an interface between JavaScript and C/C++ libraries.

- node-api

> Node-API (formerly N-API) is **an API for building native Addons**. It is independent from the underlying JavaScript runtime (for example, V8) and is maintained as part of Node.js itself. 
>
> This API will be Application Binary Interface (ABI) stable across versions of Node.js. It is intended to insulate addons from changes in the underlying JavaScript engine and allow modules compiled for one major version to run on later major versions of Node.js without recompilation. 

- 优点
  - 以 C 的风格提供稳定 ABI 接口；
  - 消除 Node.js 版本的差异；
  - 消除 JavaScript 引擎的差异（如 Google V8、Microsoft ChakraCore 等）。


## napi-rust

- [Getting started – NAPI-RS](https://napi.rs/docs/introduction/getting-started)







Value

If you want **NAPI-RS** to convert objects from JavaScript with the same shape defined in Rust, you can use the `#[napi]` macro with the `object` attribute.

```Rust
/// #[napi(object)] requires all struct fields to be public
#[napi(object)]
struct PackageJson {
	pub name: String,
	pub version: String,
	pub dependencies: Option<HashMap<String, String>>,
	pub dev_dependencies: Option<HashMap<String, String>>,
}

#[napi]
fn log_package_name(package_json: PackageJson) {
	println!("name: {}", package_json.name);
}

#[napi]
fn read_package_json() -> PackageJson {
	// ...
}
```



(不知道对读摄像头参数有没有用)

### [Buffer#](https://napi.rs/docs/concepts/values#buffer)

```
#[napi]
fn with_buffer(buf: Buffer) {
  let buf: Vec<u8> = buf.into();
  // do something
}

#[napi]
fn read_buffer(file: String) -> Buffer {
	Buffer::from(std::fs::read(file).unwrap())
}
export function withBuffer(buf: Buffer): void
export function readBuffer(file: string): Buffer
```