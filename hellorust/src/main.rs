use std::collections::HashMap;
fn main() {
    test1();
    test2();
    test3();
    test_str_1();
    test_hash_1();
    test_hash_2();
    test_hash_3();
}

fn test_hash_3() {
    println!();
    println!("=== hash3 ===");

    let mut c = HashMap::new();
    c.insert(String::from("a"),10101);
    c.insert(String::from("b"),10102);
    c.insert(String::from("c"),10103);

    if let None = c.get(&String::from("a")) {
        println!("a 不存在");
    }

    c.entry(String::from("c")).or_insert(10010); // 如果不存在就插入  [default = 0 如果存在 返回的是当前元素的可变引用]
    c.entry(String::from("d")).or_insert(10010);

    for (k,v) in c {
        println!("K: {} V: {}",k,v);
    }

    let mut ssc = HashMap::new();
    if let Some(i) = ssc.insert(String::from("a"),12) {
        println!("Ssc: {}",i);
    }else {
        println!("key = a 不存在  进行插入");
    }
    if let Some(i) = ssc.insert(String::from("a"),13) {
        println!("Ssc: {}",i);   // 如果存在 就插入新的返回旧的数据
    }else {
        println!("key = a 不存在  进行插入");
    }

    let text = "hello world wonderful world";
    let mut new_db = HashMap::new();
    for i in text.split_whitespace() {
        let count = new_db.entry(i).or_insert(0);
        *count +=1;
    }

    for (k,v) in new_db {
        println!("key: {}  val: {}",k,v);
    }
}

fn test_hash_2() {
    println!();
    println!("=== hash2 ===");
    let mut m1 = HashMap::new();

    m1.insert(String::from("name"),String::from("dollarkiller"));
    m1.insert(String::from("age"),String::from("18"));

    for (k,v) in &m1 {
        println!("K: {}  V: {}",k,v);
    }

    println!();
    let key = String::from("Name");
    let name = String::from("DollarKiller");
    let mut s1:HashMap<&String,&String> = HashMap::new();

    s1.insert(&key,&name);
    for (k,v) in s1 {
        println!("S1 K: {} V: {}",k,v);
    }
}

fn test_hash_1() {
    println!();
    println!("=== Hash ===");

    // 创建
    let mut sores = HashMap::new();
    if let None = sores.insert("H",12) {
        println!("Success");
    }

    let keys = vec![String::from("Blue"),String::from("Red")];
    let values= vec![20,10];

    // let scores:HashMap<&String,&i32> = keys.iter().zip(values.iter()).collect();
    let scores:HashMap<_,_> = keys.iter().zip(values.iter()).collect();
    for (k,v) in &scores {
        println!("K: {} V:{}",k,v);
    }

    println!("{:#?}",scores);
}

fn test_str_1() {
    println!();
    println!("=== Str ===");

    let mut z0 = String::new();
    z0.push_str("hello");

    let s1 = String::from("init some thing");
    println!("s1: {}",s1);

    let mut he = "hello rust".to_string();
    he.push_str(&s1);
    println!("{}",he);
    println!("{}",s1);

    he.push(' ');
    he.push('_');
    he.push('a');
    he.push('b');
    he.push('c');
    he.push('_');

    println!("{}",he);

    let ss = he + &s1; // 注意用 + he 的所有权会被移交
    println!("{}",ss);

    let s11 = String::from("s11");
    let s22 = String::from("s22");
    let s33 = String::from("s22");
    let s44 = format!("{} - {} - {} ",s11,s22,s33);
    println!("{}",s44);

    let s44 = 123;
    println!("{}",s44);

    let s44 = String::from("你好RUST");
    for s in s44.bytes() {
        println!("{}",s);
    }

    for s in s44.chars() {
        println!("{}",s);
    }
}

fn test3() {
    println!();
    println!("====================");

    let mut a = Vec::new();
    a.push("a");
    a.push("b");
    a.push("c");

    a.pop();
    a.pop();


    println!("{}",a.len());

    // a.push("b");
}

fn test2() {
    enum Spc {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        Spc::Int(3),
        Spc::Text(String::from("blue")),
        Spc::Float(10.2),
    ];

    for i in &row {
        match i {
            Spc::Int(x) => println!("x:{}",x),
            Spc::Float(f) => println!("f: {}",f),
            Spc::Text(str) => println!("Str: {}",str),
        }
    }


}

fn test1() {
    // 声明 vec i32 可变数组
    // let v: Vec<i32> = Vec::new();

    // let c = vec![1,2,3]; // 使用宏申明

    let mut v = Vec::new();
    v.push(5);
    v.push(4);
    v.push(3);
    v.push(2);
    v.push(1);
    v.push(0); // 添加值
    // 当v 离开作用域时  就会被丢弃

    // 依次读取
    let third: &i32 = &v[2];
    println!("this third element is {}",third);

    match v.get(5) {
        Some(c) => println!("this third element is {}",c),
        None => println!("this is no third element."),
    }

    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("A: {}",i);
    }

}