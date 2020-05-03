pub fn test1() {
    // 闭包 定义1
    let add_s1 = |x: u32| -> u32 {
        x + 1
    };

    // 闭包 定义2
    let add_s2 = |x| {
        x + 2
    };

    // 闭包 定义3
    let add_s3 = |x| x + 3;


    // 闭包 使用
    println!("s1: {}",add_s1(12));
    println!("s2: {}",add_s2(12));
    println!("s3: {}",add_s3(12));

    // 闭包 使用 上下文环境
    let bb = 122;
    let add_s4 = |x| x + bb;
    println!("s4: {}",add_s4(12));
}