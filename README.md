# Cargo

### 基础命令
``` 
cargo run 
cargo run --release

cargo build
cargo build --release
```

### 编译优化
``` 
[profile.dev]
opt-level = 0  #  优化级别

[profile.release]
opt-level = 3   
lto = true     # 更好的优化
```

### 导入包
``` 
[dependencies]
rand = "0.4.0"
```

### 文档注释
生成文档: `cargo doc`
``` 
//! My Crate  mod说明
//!
//! 'my_crate' is a collection of utilites to mak performing certain calcuations more convenient
//!

/// Add one to the number given  方法说明
/// #Example
///
/// ```
/// let file = 5;
///
/// assert_qe!(6,mylib::add_one(file));
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

### Crate 发布
- 1. 创建Crates.io账户: 通过Github账户注册，并通过cargo login ××× 登录
- 2. Cargo.toml添加描述
``` 
[package]
name = "package_name"
version = "0.1.0"  # version
lecense = "MIT"    # lecense
authors = ["dollarkillerx <adapawang@gmail.com>"]   # auth
edition = "2018"
description = "descript package"

[dependencies]
```
- 3. 撤回版本
``` 
cargo yank --vers 0.1.0 # 撤回到指定版本
```

### 工作空间
主目录Cargo.toml
``` 
[workspace]
members = [
    "adder",
]

cargo build  # build 该workspace 下所有项目
cargo run -p adder # 指定运行改项目下某一个子项目
```