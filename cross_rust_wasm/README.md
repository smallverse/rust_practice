# cross_rust_wasm
## 1 about

https://www.npmjs.com/package/cross_rust_wasm

---

## 2 use cross_rust_wasm
nodejs：
```shell
npm i rust_wasm@nodejs
```

webpack(bundler)：
```shell
npm i rust_wasm@bundler
```

[web:](https://rustwasm.github.io/docs/wasm-bindgen/examples/without-a-bundler.html)
```shell
npm i rust_wasm@web
```
[no-modules：](https://rustwasm.github.io/docs/wasm-bindgen/examples/without-a-bundler.html)
```shell
npm i rust_wasm@no-modules
```

---

## 3🚴 build & publish
### 2.1 build with `wasm-pack build`

[deploying-rust-and-webassembly](https://rustwasm.github.io/docs/wasm-bindgen/reference/deployment.html)

[pack-and-publish](https://rustwasm.github.io/wasm-pack/book/commands/pack-and-publish.html)
```
# default: --target bundler
wasm-pack build

# or: --target bundler
wasm-pack build --target nodejs

# or: --target web
wasm-pack build --target web

# or: --target no-modules
wasm-pack build --target no-modules

```

### 2.2 🔬 test in headless browsers with `wasm-pack test`

```
wasm-pack test --headless --firefox
```

### 2.3 🎁 publish to npm with `wasm-pack publish`
#### npm select source from nrm:https://github.com/Pana/nrm
```shell
# 
nrm ls
nrm use npm
npm adduser

# or --tag xxx
wasm-pack publish

nrm ls
#nrm use tencent
```

```
wasm-pack publish
```

## 3 build & publish by:
1 https://github.com/rustwasm/wasm-pack

2 https://developer.mozilla.org/zh-CN/docs/WebAssembly/Rust_to_wasm

---
