# 高级特性 

## 不安全的RUST

### 不安全的RUST具有的超级力量
- 1. 解引用裸指针
- 2. 调用不安全的函数或则方法
- 3. 访问或修改可变静态变量
- 4. 实现不安全的trait
> 注意: unsafe 并不会关闭借用检查器 或 禁用任何其他的RUNT安全检查规则
> 只是提供上述几个不被编译器检查内存安全的功能  unsafe并不意味着这块中的代码一定就是不OK的 它是表示有程序员来确保安全


### 裸指针 test1
- 可变不可变，分别写作 `*const`,`*mut T`
- 1. 允许忽略借用规则  可以同时拥有可变和不可变引用，或则多个指向相同位置的可变指针
- 2. 不保证指向有效的内存
- 3. 允许为空
- 4. 不能实现任何自动清理的功能
``` 
let mut num = 5;

    // 创建不可变 可变的裸指针 可以在安全的代码中
    // 只是不能在不安全的代码块外解引用裸指针

    // 不可变裸指针
    let r1 = &num as *const i32;
    // 可变裸指针
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r2: {}",*r2);
        *r2 = 1212;
        println!("r1: {}",*r1);
    }
```

### 调用不安全的函数或则方法 test2
``` 
pub fn test2() {
    unsafe {
        dag_fn();
    }
}

unsafe fn dag_fn() {
    println!("ddg");
}
```

### 访问修改 可变静态变量
- 静态变量和常量的区别
    - static 变量拥有固定的内存地址 (基地址)
    - 常量则允许在任何被用到的时候复制其数据
- 静态变量是可以改变的 但是不安全的 unsafe
``` 
static mut HELLO_WORLD: &str = "Hello World";
pub fn test3() {
    unsafe {
        HELLO_WORLD = "Stc";
        println!("{}",HELLO_WORLD);
    }
}
```

### 实现不安全的trait
- 1. 当至少一个方法包含编译器不能验证的不变量是，该trait是不安全的
- 2. 在trait之前增加unsafe声明其为不安全的，同时trait的实现也必须用unsafe标记
``` 
unsafe trait Foo {
    fn foo(&self);
}

struct Bar();

unsafe impl Foo for Bar {
    fn foo(&self) {
        println!("foo");
    }
}

pub fn test4() {
    let a = Bar();
    a.foo();
}
```

## 关键类型
- 关联类型是一改将类型占位符与trait相关联的方式
- trait的实现者会针对特定类型在这个类型的位置指定相应的具体类型
> 使用泛形的时候必须标注引用类型
``` 
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}


pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;  // 如该多个类型实现来Iterator rust的特性 会不知道要返回哪一个类型
}
需要完全限定语法  带上具体的类型
let mut a = A {value: 3}
<A as Iterator<i32>>::next(&mut a);
```

## 默认泛形类型参数和运算符重载
- 1. 使用泛形类型参数时，可以为泛形指定一改默认的类型具体
- 2. 运算符重载是指在特定情况下自定义运算符行为操作
- Rust并不允许创建自定义运算符或则重载运算符
- 不过对于std::ops 中列出的运算符和相应的trait 我们可以实现运算符相关trait来重载
``` 
trait Add<RHS=Self> { // <默认类型参数>  RHS是一改泛型类型参数(right hand side)
    type Output;
    fn add(self,rhs: RHS) -> Self::Output;
}
```
## 完全限定语法
