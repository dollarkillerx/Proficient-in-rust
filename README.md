# Slice

### 简单切片
``` 
    let hello = &s[0..idx]; // 切片
    // let hello = &s[0..=(idx-1)]; // 切片
    // let hello = &s[..=(idx-1)]; // 切片
    // let hello = &s[1..]; // 切片
    // let hello = &s[..]; // 切片

    let a_list = [1,2,3,4,5,6,7,8,9,10];
    let b = &a_list[..3];
    println!("B: {:#?}  len: {}",b,b.len());
```