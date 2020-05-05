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

### Trait 对象动态分发
- 1. trait对象动态分发
    - 1. 上述栗子中 ，对泛形类型使用trait bound编译器进行的方式是单态化处理，单态化的代码进行是静态分发 (就是编译器在编译的时候就知道调用来那些方法)
    - 2. 使用trait对象时，Rust必须使用动态分发  编译器无法知晓所有可能用于trait对象代码类型，所以它也不知道应该调用那些方法实现。为此RUST在运行时使用trait对象中的指针来知晓需要调用那个方法
- 2. trait对象要求对象安全
    - 只要对象安全(object safe)的trait才可以组成trait对象  trait的方法满足一下两个条件才是对象安全的
        - 1. 返回值类型不为Self
        - 2. 方法没有任何泛型类型参数