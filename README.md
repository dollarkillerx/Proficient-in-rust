#  trait 再练习

### trait s1
```rust
//  PRO
trait LessPro<RHS=Self> {
    fn pro_less(self,rhs: RHS) -> RHS ;
}

impl LessPro for i32 {
    fn pro_less(self, rhs: Self) -> Self {
        self - rhs
    }
}
// pro 2
trait AddPro<RHS=Self> {
    type Output;
    fn pro_add(self,r: RHS) -> Self::Output;
}

impl AddPro for i32 {
    type Output = u32;

    fn pro_add(self, r: Self) -> Self::Output {
        (self + r) as Self::Output
    }
}
```

### Rp
```rust
impl AddPro for i32 {
    type Output = u32;

    fn pro_add(self, r: Self) -> Self::Output {
        (self + r) as Self::Output
    }
}

trait HelloWorld<T> {
    fn hello_world(&self,p: T);
}

impl HelloWorld<&str> for i32 {
    fn hello_world(&self,p: &str) {
        println!("{}",p);
    }
}

impl HelloWorld<i32> for i32 {
    fn hello_world(&self,p: i32) {
        println!("Hello World : {}",p);
    }
}
```
孤儿原则  要实现trait 需要trait 要在当前的crate中

### impl trait and dya trait 
impl 为静态分发  第一次分发后会将他单利化
dya 动态分发  (当trait以对象使用的时候  必须为动态分发  unsize)

###  Send trait and Sync trait 
- Send 安全的在线程间传递  跨线程传递所有权
- Sync 跨线程传递共享(不可变)引用


