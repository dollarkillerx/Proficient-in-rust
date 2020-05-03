pub fn test3() {
    test1();
    test2();
}

fn test2() {
    let x = vec![1,2,3];
                                //  把x所有权移交到闭包内
    let equal_to_x = move |z|z==x; // x会被drop
    // let equal_to_x = |z|z==x;
    let y = vec![1,2,3];
    assert!(equal_to_x(y));

    // println!("{:?}",x);
}

fn test1() {
    let x = String::from("aaa");

    let equal_to_x = |z: String| z == x;
    // let equal_to_x = move |z: String| z == x;

    println!("a: {}",equal_to_x(String::from("aaa")));
    println!("x === {:?}",x);
}