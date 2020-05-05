# 模式与结构
- 1. 模式是RUST中特殊的语法，模式用来匹配值的结构
- 2. 模式的组成
    - 1. 字面值
    - 2. 解构的数组，枚举，结构体，或则元祖
    - 3. 变量
    - 4. 通配符
    - 5. 占位符

#### demo1
``` 
let a = 1;
match a {
    0 => ...,
    2 => ...,
}

match a {
    0 => ...,
    2 => ...,
    _ => ..., // 匹配余下的
}
```
这样的代码rust会报错  match需要匹配一种类型的所有情况
#### demo2 if let
``` 
let color: Option<&str> = None;
let is_ok = true;
let age: Result<u8,_> = "33".parse();

if let Some(c) = color {
    println!("color: {}",c);
}else if is_ok {
    ...
}
```

#### while let 
只要模式匹配就执行循环执行
``` 
let mut stack = Vec::new();
stack.push(1);
stack.push(2);
stack.push(3);

while let Some(top) = stack.pop() {
    println!("top = {}",top);
} // 只要匹配Some(value) 成功就会一直循环执行
```

#### for
for 中模式直接跟随for关键字 例如 for x in y，x就是对应的模式
``` 
let v = vec!['a','b','c'];
for (index,value) in v.iter().enumerate() {
       println("index: {} value: {}",indx,value);
}
此处的模式是(index,value)
```
### let
``` 
let (x,y,z) = (1,2,3);
let (x,..,z) = (12,2,3);
println("x: {},z: {}",x,y); // .. 忽略中间的
```
### 函数参数
```rust
fn print_point(&(x,y):&(i32,i32)) {
    println!("x: {},y: {}",x,y);
}

fn print_point2((x,y):(i32,i32)) {
    println!("x: {},y: {}",x,y);
}

fn main() {
    let p = (3,5);
    print_point(&p);
}
// demo2: 
fn test1_pt((x,y):(String,String)) {
    println!("x: {},y: {}",x,y)
}

fn test1() {
    let a = (String::from("aa"),String::from("bb"));

    test1_pt(a);
}
```

### 模式在使用它的地方并不都是相同的，模式存在不可反驳的可反驳的
- 1. 模式有两种
    - 1. `refutable` 可反驳的
    - 2. `irrefutable` 不可反驳的
    - 能匹配任何传递的可能值称为不可返回的  对值进行匹配可能失败的模式称为可反驳的
- 2. 只能接受不可返回模式有
    - 函数
    - let语句
    - for循环
    - 原因： 应为通过不匹配的值程序无法进行有意义的工作
- 3. if let 和 while let表达式被限制为只能接受可反驳模式，应为它们定义就是为了处理可能失败的条件

### 模式匹配语法ALL
s1:
``` 
match a {
    1|2 => ...
    _ => ...
}

match a {
    1..=3 =>  // 1.2.3
    _ => 
}
```
s2:
``` 
let p = Point{x:1,y:2};
let Point{x:a,y:b} = p;
println("point x: {} y: {}",x,y);

let p = Point{x:1,y:0};
match p {
    Point{x,y:0} => println!("x axis"),
    Point{x:0,y} => println!("y axis"),
    Point{x,y} => println!("other"),
}
```
s3:
``` 
match sum {
    Some(x) if x < 5 => println!("<5"),
    Some(x) => printlb!("x: {}",x),
}
```

#### @运算符 允许我们在创建一个值存放变量的同时，测试这个变量是否匹配模式
``` 
let msg = Message::Hello {id:5};
    match msg {
        Message::Hello {id: id_va@ 3..=7} => {
            println!("id_val: {}",id_va);
        },
        Message::Hello {id: id_va@ 10..=20} => {
            println!("id_val 20 @ 20: {}",id_va);
        },
        Message::Hello {id:x} => {
            println!("x = {}",x)
        }
    }
```