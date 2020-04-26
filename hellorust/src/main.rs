fn main() {
    // t1();
    t2();
}

// enum Option<T> { // <T> 泛形
//     Some(T),
//     None,
// }

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

fn t1() {
    let v1 = IpAddr{
        kind:IpAddKind::V6(String::from("::1")),
    };
    let v2 = IpAddr{
        kind:IpAddKind::V4(8,8,8,8),
    };

    println!("V1: {:#?}",v1);
    println!("V2: {:#?}",v2);
    t1_t(&v1.kind);
    println!("V1: {:#?}",v1);
}

fn t1_t(ip: &IpAddKind) {
    match *ip {
        IpAddKind::V4(_,_,_,_) => println!("v4"),
        IpAddKind::V6(_) => println!("v6"),
        _ => println!("ha"),
    }
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
            // Message::Write(&s) => println!("s: {}",s),
        }
    }
}

// 等同于
// strcut QuitMessage; // 类单元结构体
// struct MoveMessage {
//     x: i32,
//     y: i32,
// }
// struct WriteMessage(String)
// struct Change(i32,i32,u32)