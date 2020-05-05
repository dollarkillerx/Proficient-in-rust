pub fn test1() {
    test1_1();
    test1_2();
}

fn test1_1() {
    let mut num = 5;

    // 创建不可变 可变的裸指针 可以在安全的代码中
    // 只是不能在不安全的代码块外解引用裸指针

    // 不可变裸指针
    let r1 = &num as *const i32;
    // 可变裸指针
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r2: {}",*r2);
        *r2 = 1212;
        println!("r1: {}",*r1);
    }
}

fn  test1_2() {
    let addr = 0x12345;
    let r = addr as *const i32;

    unsafe {
        println!("rr");
        println!("r: {}",*r);
    }
}