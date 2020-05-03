# 函数和闭包

### easy test1
```rust
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
```

### 结构体中闭包 test2 
```rust
pub fn test2() {
    let mut add = Cache::new(|x| x + 12);
    let c1 = add.add(12);
    let c2 = add.add(13);
    println!("C1: {} C2: {}",c1,c2);
}

struct Cache<T>
    where T: Fn(u32) -> u32
{
    add: T,
    value: Option<u32>,
}

impl <T> Cache <T>
    where T: Fn(u32) -> u32
{
    fn new(fa: T) -> Cache<T> {
        Cache{
            add:fa,
            value:None,
        }
    }

    fn add(&mut self, i: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.add)(i);
                self.value = Some(v);
                v
            },
        }
    }
}
```


### 闭包
- Fn trait:
    - FnOnce  消费从周围作用域的变量  为了消费捕获到的变量  闭包必须获取其所有权
    - FnMut   获得可变借用 可以改变其环境
    - Fn      获得不可变借用
> 当创建一个闭包是，Rust 会更具其如何使用环境中的变量来判断我们希望如何引用环境
> 由于所有闭包 都可以被调用至少一次  因为所有闭包都实现来FnOnce   
> 没有移动被捕获变量的所有权到闭包也实现来FnMut 不需要对捕获的变量进行可变访问的闭包实现来Fn
```rust
    let x = vec![1,2,3];
                                //  把x所有权移交到闭包内
    let equal_to_x = move |z|z==x; // x会被drop
    // let equal_to_x = |z|z==x;
    let y = vec![1,2,3];
    assert!(equal_to_x(y));

    println!("{:?}",x);
```

### 迭代器
> 迭代器 负责遍历序列中的每一项 和决策何时结束的逻辑
> 创建迭代器: 迭代器是惰性的 在调用迭代器 不会有任何效果
> 所有迭代器 都实现来 iterator trait
``` 
# iterator trait定义在标准库
trait Iterator {
    type Item;
    // type Item 和 self::Item 用法 定义trait 关联类型
    fn next(&mut self) -> Option<Self::Item>;
}

// next为定义Iterator唯一方法  每next一次返回一个元素 结束时返回None
```

``` 
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
```

### 消费适配器
``` 
    // 消费适配器
    let v1 = vec![2,3,45,6,7];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum(); // 调用消费适配器sum球和

    println!("sum: {}",total);
```

### 迭代适配器
``` 
    // 迭代适配器
    let v1 = vec![1,2,3,4,5,6,7,8];
                            // .map 对每一个元素进行操作   
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    println!("v2 : {:?}",v2);

    let v2:Vec<_> = v1.into_iter().filter(|x| *x > 5).collect();
    println!("v2 : {:?}",v2);
```

### 自定义迭代器
