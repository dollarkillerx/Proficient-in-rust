trait Hello {
    fn hello(&self);
}

struct Us1;
struct Us2;
struct Us3;

impl Hello for Us1 {
    fn hello(&self) {
        println!("user1");
    }
}

impl Hello for Us2 {
    fn hello(&self) {
        println!("user2");
    }
}

impl Hello for Us3 {
    fn hello(&self) {
        println!("user2");
    }
}

fn hello1(u: impl Hello) // impl Hello 静态分发
{
    u.hello();
}

fn hello2(c: &dyn Hello) { // dyn  动态分发
    c.hello();
}

struct Us {}
impl Us {
    fn hello(&self,c: impl Hello) {
        c.hello();
    }
}



fn main() {
    let (a,b,c) = (Us1,Us2,Us3);
    hello1(a);
    hello1(b);
    hello1(c);
    let (a,b,c) = (Us1,Us2,Us3);
    hello2(&a);
    hello2(&b);
    hello2(&c);
    let (a,b,c) = (Us1,Us1,Us3);

    // let c : Vec<Box<Hello>> = vec![Box::from(a),Box::from(b)];
    // let c : Vec<Box<Hello>> = vec![Box::from(a),Box::from(b),Box::from(c)];
    let c : Vec<Box<dyn Hello>> = vec![Box::from(a), Box::from(b), Box::from(c)]; // trait 特征类型 无法知道其大小 只有用Box 等来装
    // let c = vec![a,b,c];
    println!("c: {}",c.len());

    let (a,b,c) = (Us1,Us1,Us3);
    let u = Us{};
    u.hello(a);
    u.hello(b);
    u.hello(c);



    println!("Hello, world!");
}
