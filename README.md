# 生命周期
有注释的生命周期与没有写注释的生命周期的工作方式是相同的。它们只是帮助编译器澄清生命周期所涉及的上下文的标记。
### demo1
``` 
// 地球
#[derive(Debug)]
struct Earth {
    location: String,
}

// 恐龙法官
#[derive(Debug)]
struct Dinosaur<'a> {  // 注释生命周期帮助 编译器理解  注意Dinosaur的生命周期不能>Earth 防止悬空指针
    location: &'a Earth,
    name: String,
}

fn main() {
    let earth = Earth{
        location: "dc".to_string(),
    };
    let dinosaur = Dinosaur{
        location: &earth,
        name: "hc".to_string(),
    };

    println!("Earth: {:?}",earth);
    println!("Dinosaur: {:?}",dinosaur);
    println!("Hello, world!");
}
```