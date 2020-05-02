# 一个简单的小demo
###  学习目标
实现一个命令行工具 GREP

### 使用到的技术
- 代码组织
- vector and str
- 异常处理
- 合理使用 trait 和 生命周期
- 测试

### 技术补充
#### 获取命令行参数
- `std::env::args` 会返回一改迭代器
```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}",args);
}
```