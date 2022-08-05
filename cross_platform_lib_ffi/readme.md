# build use cargo or [cross](https://github.com/cross-rs/cross) ,use FFI [cbindgen](https://github.com/eqrion/cbindgen/).
> FFI examples: https://github.com/shepmaster/rust-ffi-omnibus/tree/master/examples

> http://kmdouglass.github.io/posts/complex-data-types-and-the-rust-ffi/

> [UE4 3rd plugin Test](https://github.com/smallverse/ue4_test), The same is true for Unity 3D, but you need to build it yourself, which is easier than UE4

> [for cross,Speed up the download of container images via DockerHub](https://github.com/smallverse/imageverse)

## 1 Win11 amd64 build for Win & Android 
### windows
```shell
cargo build --release --target=x86_64-pc-windows-msvc 
```
### Android
```shell
#arm64
cross build --release --target=aarch64-linux-android 
#arm32
cross build --release --target=arm-linux-androideabi
#arm32
cross build --release --target=armv7-linux-androideabi

```
---
## 2 MacOS(M1 arrach64)  build for MacOS & IOS
### MacOS
```shell
#m1 arm64 macos
cargo build --release --target=aarch64-apple-darwin
#amd64 macos
cargo build --release --target=x86_64-apple-darwin
```


### IOS
```shell
#arm64 ios
cargo build --release --target=aarch64-apple-ios
```
---

## 3 build for Linux
> The linux build is simple enough for you to try, so I won't go over it here

