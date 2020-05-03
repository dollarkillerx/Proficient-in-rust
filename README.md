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