# 泛形
泛型是具体属性其他属性的抽象替代 ，用于减少代码重复
> 使用泛型 并不会造成程序性能上的损失。rust通过在编译时 进行泛型代码的单态化 保证效率
> 单态化 时通过填充编译时使用具体类型  将通过代码转换为特定代码的过程

demo1: 
``` 
fn test1() {
    let data_data1 = [1,2,3,4,5,6,7,89,22];
    println!("data_data1 max: {}",largest(&data_data1));

    let data_data2 = ['a','b','c','d','e','f'];
    println!("data_data2 max: {}",largest(&data_data2));
}


// <T: PartialOrd + Copy>    约束条件
// PartialOrd 可以按照顺序比较
// Copy 具有Copy的特征
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut larger = list[0];
    for &item in list.iter() {
        if item > larger {
            larger = item // 因为具有copy属性 才可以传值
        }
    }
    larger
}
```

struct demo: 
``` 
#[derive(Debug)]
// struct Hh<T: Copy> {
struct Hh<T> {
    a: T,
    b: T,
    name: String,
}

fn test2() {
    let hc = Hh{
        a: String::from("a"),
        b: String::from("b"),
        name: String::from("aaa"),
    };

    println!("S1: {:#?}",hc);

    let hc2 = Hh{
        a: 12,
        b: 33,
        name: String::from("aa")
    };

    println!("h2: {:#?}",hc2);
}

struct Ac<T,C> {
    a: T,
    b: C,
}
```

enum demo:
``` 
enum Option<T> {
    Some(T),
    None,
}

enum Result<T,E> {
    Ok(T),
    Err(E),
}
```

友好的实例:
``` 
struct Huc<T:Copy,C: Copy> {
    tt: T,
    cc: C,
}

impl <T:Copy,C: Copy> Huc<T,C> {
    fn new<A:Copy,B:Copy>(a:A,b:B) -> Huc<A,B> {
        Huc{
            tt:a,
            cc:b,
        }
    }

    fn print_tt(&self) {
        println!("tt: {}",self.tt); // T` cannot be formatted with the default formatter
    }

    fn print_cc(&self) {
        println!("cc: {}",self.cc);
    }
}


fn test3() {
    println!();
    println!();

    let c = Huc::new("a,","b");
    c.print_cc();
    c.print_tt();
} 
```