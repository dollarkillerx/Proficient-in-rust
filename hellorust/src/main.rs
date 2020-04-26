fn main() {
    // test1();
    // test2();
    // test3();
    // test4();
    test5();
}

fn test5() {
    let a = Dco::new(String::from("aaa"),12);
    a.print_age();
    a.print_name();
}
struct Dco {
    name:String,
    age:u8,
}
impl Dco {
    fn new(name:String,age:u8) -> Dco {
        Dco{name,age}
    }
    fn print_age(&self) {
        println!("age: {}",self.age);
    }
}
impl Dco {
    fn print_name(&self) {
        println!("name: {}",self.name);
    }
}

fn test4() {
    let go = Skill{
        name: String::from("go"),
        weights:8,
    };

    let rust = Skill{
        name: String::from("rust"),
        weights:3,
    };

    println!("My skill proficiency go vs rust {}",go.ko(&rust));
}

struct Skill {
    name: String,
    weights:u8,
}
impl Skill {
    // 关联函数 调用方式 Lan::new()
    fn new(name:String,weights:u8) ->Skill {
        Skill{name,weights}
    }
    // 结构体方法
    fn ko(&self,sk: &Skill) -> bool {
        if self.weights > sk.weights {
            return true
        }
        return false
    }
}

// struct Dog{
//     name: &String, // 结构体需要获得 结构体下 所有元素的 所有权
// }

// 类单元结构体（unit-like structs）
struct Success();

fn test3() {
    let ca1 = Cat(String::from("f5"),18);
    let ca2 = ca1;
    // println!("ca1: {:#?}",ca1);  ca1 所有权转移
    println!("ca2: {:#?}",ca2);
}

#[derive(Debug)]
struct Cat(String,u8); // 元组 结构体

fn test2() {
    let u1 = User {
        username: String::from("U1"),
        sex:Sex::Wman,
        age:16,
    };

    let u2 = User{
        age:18,
        ..u1  // 发生所有权转移  (username age)
    };

    let u3 = User{
        username: String::from("U2"),
        sex:Sex::Man,
        ..u1  // 此处没有发送所有权转移 (age 是在栈上)
    };

    println!("U2: {:#?}",u2);
    // println!("U1: {:#?}",u1);
    println!("U3: {:#?}",u3);
}

fn test1() {
    let mut dk = User{
        username: String::from("DollarKiller"),
        sex:Sex::Man,
        age:18,
    };

    test1_modify_username(&mut dk);
    println!("{:#?}",dk); // {:#?}  #代表格式化打印
    // println!("age: {}",dk.age);
}

fn test1_modify_username(dk: &mut User) {
    // 此处 如果 dk.username; username的所有权就会被移出
    // let a = &dk.username; // 引用
    let a = &mut dk.username; // 借用

    println!("you name: {}",a);
    &a.push_str("  MMM ");
}

#[derive(Debug)]
enum Sex {
    Man,
    Wman,
}

#[derive(Debug)]
struct User {
    username: String,
    sex:Sex,
    age:u8,
}

