pub fn test4() {
    run1();
}

fn run1() {
    let v1 = vec![1,2,3];
    let v1_iter = v1.iter(); // 到目前为止 不会对v1产生任何影响
    for val in v1_iter {
        println!("val: {}",val);
    }

    let mut v1_iter = v1.iter(); // 到目前为止 不会对v1产生任何影响
    // for val in v1_iter {
    //     println!("val: {}",val);
    // }

    loop {
        match v1_iter.next() {
            Some(t) => println!("data: {}",t),
            None => {
                break
            }
        }
    }
}