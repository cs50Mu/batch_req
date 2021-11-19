### 交叉编译

记录下在 Mac 下编译 linux 可执行文件需要做的事情：

```
# Install the toolchain to build Linux x86_64 binaries
$ rustup target add x86_64-unknown-linux-gnu

# we need a program that links our compiled objects together
$ brew tap SergioBenitez/osxct
$ brew install x86_64-unknown-linux-gnu

# we need to tell Rust about the linker
# The official way to do this is to add a new file named `.cargo/config` in the root of your 
# project and set its content to something similar:
[target.x86_64-unknown-linux-gnu]
linker = "x86_64-linux-gnu-gcc"

# [optional] if you have any C code compiled by a Rust build script, you also have to 
# set environment variables like `TARGET_CC` to get it working:
$ export TARGET_CC=x86_64-linux-gnu-gcc

```

如果你的依赖包（比如reqwest）有用到openssl，编译的时候还是会报错。。最简单的解决办法是不依赖 openssl！
可以在`Cargo.toml`里这样设置来让 reqwest 使用 `rustls` 而不是 `openssl`：

`reqwest = { version = "0.11", features = ["rustls-tls"], default-features = false }`

#### 参考

- [A "rustup target" Example: Using a Mac to cross-compile Linux binaries](https://timryan.org/2018/07/27/cross-compiling-linux-binaries-from-macos.html)
- [Cross Compile Rust from Mac to Linux](https://github.com/chinedufn/cross-compile-rust-from-mac-to-linux)

### 性能对比

没有白费功夫，性能提升明显！

在 concurrency 设置为 5 的时候，发起请求的 qps 约为 python 版本（使用了multiprocessing）的 4 倍；

在 concurrency 设置为 10 的时候，发起请求的 qps 约为 python 版本（使用了multiprocessing）的 8 倍；
