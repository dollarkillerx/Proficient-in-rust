# 面向对象
- 对象
- 封装
- 继承

### Rust中面向对象
- 组成: struct or enum 加上impl块就是面向对象
- 继承 rust 中没有继承这个概念  但是可以通过trait来实现半个继承
```rust
struct Dog {
    name: String,
}

impl Dog {
    fn print_name(&self) {
        println!(self.name);
    }
}

fn main() {
    let w = Dog{
        name: String::from("wc"),
    };
    
    w.print_name();
} 
```

### trait 对象
- dyn 使用泛形
``` 
pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>, // trait 对象使用dyn 关键字描述
}

// 为什么需要dyn描述呢？ 直接用泛形不香吗 使用范形是编译器会获得其类型 改变为静态 所以泛形在RUST中没有额外的开销
// but 这样会定死类型
// pub struct Screnn<T: Draw> {
//     pub components: Vec<T>,
// }
```
