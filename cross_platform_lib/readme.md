# build use cargo or [cross](https://github.com/cross-rs/cross)

[for cross,Speed up the download of container images via DockerHub](https://github.com/smallverse/imageverse)

## Win11 amd64 build
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
## MacOS(M1) arrach64 build
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
Using the practice：https://github.com/smallverse/ue4_test 
