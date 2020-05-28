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

// 为 String 加入 form Dinosaur的实现
impl<'a> From<Dinosaur<'a>> for String {
    fn from(d: Dinosaur) -> String {
        format!("{:?}", d)
    }
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

    println!("{}", String::from(dinosaur));
}
