fn main() {
    test1();
    println!("Hello, world!");
}

fn test1() {
    // let mut c = 1 ;
    // {
    //     c = 12+ 34;
    // }
    let c:i32;
    {
        c = 12 + 34; // 这样等于声明 不用mut
    }
    println!("c: {}",c);
}