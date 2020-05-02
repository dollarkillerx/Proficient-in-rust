use hellorust::*;
fn main() {
    // test1();
    // test2();
    // test3();
    // test5::test5();
    test6::test6();
}

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

impl Hello for Student {}

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