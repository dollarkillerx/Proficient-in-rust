// RUST 中 在编译时 数据类型是固定的 大小是固定的 是分配在栈上的

fn main() {
    heap();
}

fn heap() {
    heap1();
    heap2();
    heap3();
    heap4();
    heap4_2();
    heap5();
    heap6();
    heap7();
}

fn heap7() {
    let mut name = String::from("dollarkiller");
    let a = &name;
    let b = &name;
    println!("a: {} b: {} ",a,b);
    let c = &mut name;
    println!("c: {}",c);
    // println!("a: {} b: {} c: {}",a,b,c);  当 存在借用的是哈     下面不能出现 引用
    // println!("a: {} b: {} ",a,b);
}

fn heap6() {
    let mut name = String::from("hello");
    heap6_modify_name(&mut name); // 借用
    heap6_print(&name);
    println!("name: {}",name);
}

fn heap6_print(name:&String) {
    println!("name: {}",name);
}

fn heap6_modify_name(name:&mut String) {
    name.push_str(", rust");
}

fn heap5() {
    let name = String::from("dollarkiller");
    heap5_print(&name); //  引用
    println!("you name: {}",name);
}

fn heap5_print(name:&String) {
    println!("you name: {}",name);
}


fn heap4() {
    let name = String::from("name");
    let name2= heap4_name(name); // 再次获得所有权
    println!("name: {}",name2);
}

fn heap4_name(name:String) -> String {
    name  // 所有权 再 交出
}

fn heap4_2() {
    let mut name = String::from("name");
    name= heap4_name(name); // 再次获得所有权
    println!("name: {}",name);
}


fn heap3() {
    let name = String::from("name");
    let name2 = name.clone();
    println!("name: {}",name);
    println!("name2: {}",name2);
}

fn heap2() {
    let name = String::from("name");
    let name2 = name;  //  此处 name 的所有权 交给 name2 了 name被移出了
    // println!("name: {}",name);
    println!("name2: {}",name2);
}

fn heap1() {
    let name = String::from("name"); // String 为不固定大小 so:  存在内存的堆中
    print_name(name);
    // println!("you name: {}",name);  // 此处name 失去来所有权  无法被调用

}

fn print_name(name:String) { //   这里name 做来浅拷贝  (rust 前拷贝 会导致所有权转移)
    println!("you name: {}",name)
}