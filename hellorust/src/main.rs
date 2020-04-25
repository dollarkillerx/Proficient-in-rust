// 1. 字符串slice 是String中的一部分值的引用
// 2. 字面值就是slice
// 3. 其它类型slice
fn main() {
    test1();
}

fn test1() {
    let s = String::from("hello world");
    let idx = t1(&s);

    let hello = &s[0..idx]; // 切片
    // let hello = &s[0..=(idx-1)]; // 切片
    // let hello = &s[..=(idx-1)]; // 切片
    // let hello = &s[1..]; // 切片
    // let hello = &s[..]; // 切片
    println!("Hello idx:{} msg:{}",idx,hello);

    let a_list = [1,2,3,4,5,6,7,8,9,10];
    let b = &a_list[..3];
    println!("B: {:#?}  len: {}",b,b.len());
}

fn t1(s:&String) -> usize {
    let bytes = s.as_bytes(); // str to slice

    for (i,&item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}