# 编程基础
``` 
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
    */
    println!("The value of x is: {}",x);
```

### 数据类型
rust 每一个值都属于某一个数据类型
- 标量
- 复合
``` 
# 编译时 必须指定 类型  (rust编译器会自动推导)
let guess: u32 = "42".parse().expect("Not a number!");
parse() 是类型转换   需要告诉它要转换为的数据类型
```

|len|有符号|无符号|
|-|-|-|
| 8-bit | i8 | u8 |
| 16-bit | i16 | u16 |
| 32-bit | i32 | u32 |
| 64-bit | i64 | u64 |
| arch | isize | usize| 

isize usize 基于系统 位数 改变

### 原生复合类型
- tuple 元祖 ` let tup:(i32,f64,u8) = (500,6.3,1);`
- array 数组  `let a = [1,2,3,5];`
``` 
tuple:
    let tup:(i32,f64,u8) = (500,6.3,1);
    let (x,y,z) = tup; //  解构
    println!("x: {} y: {} z: {}",x,y,z);
    println!("x: {} y: {} z: {}",tup.0,tup.1,tup.2);

array:
    let a = [0,1,2,3,4,5]; // a: [i32;6]

    // 数组 在 stack 上
    println!("a[0]: {}",a[0]);

    for index in 0..a.len() {
        println!("index is: {} &value is: {}",index,a[index]);
    }

    println!("Ii: {:?}",a);
```

### 函数
- 匿名函数
``` 
    // 匿名函数
    let hello:fn(i:String) = |name| {
        println!("Hello {}",name);
    };

    let mut name = String::new();
    println!("Please input name: ");
    io::stdin().read_line(&mut name).expect("蛤");

    hello(name);
```
- 宏的一点用
```
    let x = 5;
    let y = {  // 宏表达式
      let x = 3;
        x+1
    };

    println!("X: {} y: {}",x,y);
    // x:5 y:4

    // 返回值
    fn five() -> i32 {
        5
    }

    fn five() -> i32 {
        return 5;
    }
```

###  有趣的变量声明
``` 
    let number = if condition { // 哈哈
        5
    }else {
        4
    };
```

###  循环
loop:
``` 
let mut c = 0;
let a = loop {
    c += 1;
    if c == 200 {
        break c;
    }
};
```
for
``` 
    for index in 0..10 {
        println!("{}",index);
    }

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
```
