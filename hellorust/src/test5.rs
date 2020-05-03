pub fn test5() {

    // 消费适配器
    let v1 = vec![2,3,45,6,7];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum(); // 调用消费适配器sum球和

    println!("sum: {}",total);

    // 迭代适配器
    let v1 = vec![1,2,3,4,5,6,7,8];
                            // .map 对每一个元素进行操作
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    println!("v2 : {:?}",v2);

    let v2:Vec<_> = v1.into_iter().filter(|x| *x > 5).collect();
    println!("v2 : {:?}",v2);
}