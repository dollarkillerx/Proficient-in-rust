### Rust 国内安装
#### 配置加速镜像:
```
export RUSTUP_DIST_SERVER=https://mirrors.sjtug.sjtu.edu.cn/rust-static
export RUSTUP_UPDATE_ROOT=https://mirrors.sjtug.sjtu.edu.cn/rust-static/rustup
```
#### 安装
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
#### 配置 crate.io 镜像  `$HOME/.cargo/config `
```
[source.crates-io]
registry = "https://github.com/rust-lang/crates.io-index"
replace-with = 'ustc'
[source.ustc]
registry = "git://mirrors.ustc.edu.cn/crates.io-index"
```
