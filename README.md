# 枚举和模式匹配

### 简单枚举
``` 
fn t1() {
    let v1 = IpAddr{
        kind:IpAddKind::V6(String::from("::1")),
    };
    let v2 = IpAddr{
        kind:IpAddKind::V4(8,8,8,8),
    };

    println!("V1: {:#?}",v1);
    println!("V2: {:#?}",v2);
}

#[derive(Debug)]
enum IpAddKind{
    V4(u8,u8,u8,u8),
    V6(String),
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddKind,
}
```

### 经典用法
``` 
// 经典用法
enum Message {
    Quit,
    Move{x: i32,y:i32 },
    Write(String),
    Change(i32,i32,u32),
}

impl Message {
    fn print(&self) {
        match *self {
            Message::Quit => println!("QUit"),
            Message::Move{x,y} => println!("Move x = {} y = {}",x,y),
            Message::Change(a,b,c) => println!("Change a = {} b = {} c ={} ",a,b,c),
            _ => println!("All"),
            // Message::Write(&s) => println!("s: {}",s), String 这个需要所有权 这里 取的啥* 无法打印
        }
    }
}
```


### Option 标准类型
``` 
fn t2() {
    let some_number = Some(5);
    let some_string = Some(String::from("A String"));
    let absent_number:Option<i32> = None;

    let x:i32 =5;
    let y:Option<i32> = Some(5);
    let mut tmp = 0;
    match y {
        Some(i) => tmp = i,
        None => println!("Do nothing"),
    }

    if let Some(i) = some_number {
        println!("I: {}",i);
    }else {
        println!("is null");
    }
    println!("Tmp: {}",tmp);
    println!("Hello World");
}
```
