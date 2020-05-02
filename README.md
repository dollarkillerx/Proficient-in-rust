# Trait
特征类是与Interface  (共享行为)

简单实现
```rust
// 定义trait
pub trait GetInformation {
    fn get_name(&self) -> &String;
    fn get_age(&self) -> u32;
}

// 实现
pub struct Student {
    pub name: String,
    pub age: u32,
}

impl Student {
    fn new(name:String,age:u32) -> Student {
        Student{ name,age }
    }

    fn hello(&self) {
        println!("Hello World");
    }
}

impl GetInformation for Student {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_age(&self) -> u32 {
        self.age
    }
}

// trait 作为参数  // 必须要求item 拥有GetInformation 特征
fn print_information(item: impl GetInformation) {
    println!("name = {}",item.get_name());
    println!("age = {}",item.get_age());
    // item.hello()  这个是没有的  只能用GetInformation的方法
}

fn test1() {
    let a = Student::new(String::from("aaa"),18);
    // println!("name: {}  age: {}",a.get_name(),a.get_age());
    print_information(a);
}
```

#### 默认实现
```rust
trait Hello {
    fn hello(&self) {  // 默认实现
        println!("Hello World");
    }
}

struct StA {

}

struct Stb {

}

impl Hello for StA {} // StA 采用了默认实现
impl Hello for Stb {
    fn hello(&self) {
        println!("Hello Rust"); // StB 重写来默认实现
    }
}

fn hello(item: impl Hello) {
    item.hello();
}

fn test2() {
    let a = StA{};
    let b = Stb{};

    hello(a);
    hello(b);
}
```

### trait bound 语法糖
```rust
// fn print_information(item: impl GetInformation) { // 直接作为参数
fn print_information2<T: GetInformation> (item: &T) { // 使用trait bound 语法糖
    println!("name = {}",item.get_name());
    println!("age = {}",item.get_age());
}

fn print_all<T: GetInformation + Hello> (item: &T) {
    item.hello();
    item.get_age();
    item.get_name();
}

fn print_all2<T> (item: &T)
    where T: GetInformation + Hello
{
    item.hello();
    item.get_age();
    item.get_name();
}

fn hhh() -> impl Hello { // 返回一个特征
    Student {
        name: String::from("aaa"),
        age:12,
    }
}

fn test3() {
    let a = Student::new(String::from("aaa"),18);
    // print_all(a);
    print_all(&a);
    print_all2(&a);
    print_information2(&a);
    let b = hhh();
    b.hello();
}
```
trait 作为返回值 注意点:
```rust
fn new(b:bool) -> impl Hello {
    if b {
        return S1{};
    }else {
        return S2{};
    }
}

虽然 S1 S2都实现来 Hello 但是 这个方法里面只能返回一种类型
```

### trait 有条件的实现方法
```rust
trait Hello {
    fn hello(&self) {
        println!("Hello Rust");
    }
}

trait Nc {
    fn get_name(&self) ->&String;
}

struct Ter {
    name: String,
    age: u8,
}

struct Student {
    name: String,
    age: u8,
}

struct People<T,U> {
    ter: T,
    student: U,
}

impl Nc for Student{
    fn get_name(&self) -> &String {
        &self.name
    }
}

impl Hello for Student{}

impl Nc for Ter {
    fn get_name(&self) -> &String {
        &self.name
    }
}

impl Hello for Ter{}

impl <T:Hello + Nc,U:Hello + Nc> People<T,U> {
    fn new(stu:T,te:U) -> People<T,U> {
        People{
            ter:stu,
            student:te,
        }
    }

    fn run(&self) {
        println!("A: {} B: {}",self.student.get_name(),self.ter.get_name());
        self.student.hello();
        self.ter.hello();
    }
}

pub fn test5() {
    let s1 = Ter{
        name: String::from("ter"),
        age:18,
    };

    let s2 = Student{
        name: String::from("student"),
        age:18,
    };

    let s3 = People::new(s1,s2);
    s3.run();
}
```

#### 套娃  对于实现trait 实现 特定trait
```rust
trait Agc {
    fn get_name(&self) -> &String;
}

trait PrintName {
    fn print_name(&self);
}

impl <T: Agc> PrintName for T {
    fn print_name(&self) {
        println!("pr: {}",self.get_name());
    }
}

struct User {
    name: String,
}

impl Agc for User {
    fn get_name(&self) -> &String {
        &self.name
    }
}

pub fn test6() {
    let u = User{
        name: String::from("Acd"),
    };

    u.print_name();
}
```