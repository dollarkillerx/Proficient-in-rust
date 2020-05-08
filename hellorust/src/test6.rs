pub fn test6() {
    let a = 12;
    println!("pa: {}",do_twice(add_one,a));
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32,val: i32) -> i32 {
    f(val)
}

