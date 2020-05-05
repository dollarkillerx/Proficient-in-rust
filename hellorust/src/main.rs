fn main() {
    test1();
    test2();
    test3();
}

fn test1_pt((x,y):(String,String)) {
    println!("x: {},y: {}",x,y)
}

fn test1() {
    let a = (String::from("aa"),String::from("bb"));

    test1_pt(a);
}

enum Ac {
    Ac(String),
    Nil,
}

fn test2() {
    let a = Ac::Ac(String::from("AAAA"));
    match &a {
        Ac::Ac(c) => println!("c: {}",c),
        Ac::Nil => println!("Nil"),
    }
}

enum Message {
    Hello{id: i32},
}
fn test3() {
    let msg = Message::Hello {id:5};
    match msg {
        Message::Hello {id: id_va@ 3..=7} => {
            println!("id_val: {}",id_va);
        },
        Message::Hello {id: id_va@ 10..20} => {
            println!("id_val 20 @ 20: {}",id_va);
        },
        Message::Hello {id:x} => {
            println!("x = {}",x)
        }
    }
}
