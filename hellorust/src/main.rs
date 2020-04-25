use std::io;
use std::thread;
use std::time::Duration;

fn main() {
    // definition();
    // pc();
    // tuple();
    // list();

    // function();
    // fun2();
    // ifElse();

    // v2();
    // lop();
    for1();
}

fn definition() {
    let mut x = 5; // mut 为可变的  (类型 和 数值 可变)
    let b = 12;    // 默认定义为 不可变
    println!("The value of x is {}",x);

    // 定义常量
    const MAX_POINTS: u32 = 100_000; // 定义 100,000 rust 支持_提高可读性
}

fn pc() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;

    let mut c = 12;
    let mut c = "asdasd";
    /**
    *   let x = 1;
    *   let x = x + 6;  // 对x进行重新计算 变量仍然复用
    *
    *   let mut c = 12;
    *   let mut c = "asdasd";  // 创建新的变量 复用来 c这个名字
    * */
    println!("The value of x is: {}",x);
}

fn tuple() {
    let tup:(i32,f64,u8) = (500,6.3,1);
    let (x,y,z) = tup; //  解构
    println!("x: {} y: {} z: {}",x,y,z);
    println!("x: {} y: {} z: {}",tup.0,tup.1,tup.2);
}

fn list() {
    let a = [0,1,2,3,4,5]; // a: [i32;6]

    // 数组 在 stack 上
    println!("a[0]: {}",a[0]);

    for index in 0..a.len() {
        println!("index is: {} &value is: {}",index,a[index]);
    }

    println!("Ii: {:?}",a);
}

fn function() {
    // 匿名函数
    let hello:fn(i:String) = |name| {
        println!("Hello {}",name);
    };

    let mut name = String::new();
    println!("Please input name: ");
    io::stdin().read_line(&mut name).expect("蛤");

    hello(name);
}

fn fun2() {
    let x = 5;
    let y = {  // 宏表达式
      let x = 3;
        x+1
    };

    println!("X: {} y: {}",x,y);
    // x:5 y:4

    println!("a: {}",five());
}

fn five() -> i32 {
    // return 5;
    5
}

fn ifElse() {
   let c:fn(i:u8) = |i|{
       if i >100 {
           println!(">100")
       }else if i>50 {
           println!("> 50")
       }else {
           print!("0.0");
       }
   };

    c(1);
    c(20);
    c(100);
}

fn v2() {
    let condition = true;
    let number = if condition { // 哈哈
        5
    }else {
        4
    };

    println!("Number: {}",number);
}

fn lop() {
    let mut counter = 0;
    let result = loop {
        counter +=1;
        //  当前线程 sleep
        thread::sleep(Duration::from_millis(200));
        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result,20);
}

fn for1() {
    let a = [10,20,30,40,50];
    for item in a.iter() {
        println!("this val is: {}",item);
    }
    // this val is: 10

    for index in 0..a.len() {
        println!("idx: {} val: {}",index,a[index]);
    }

    println!();

    for num in(1..4).rev() { // .rev 反转数组
        println!("{}!",num);
    }
}