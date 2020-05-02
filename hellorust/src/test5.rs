trait Hello {
    fn hello(&self) {
        println!("Hello Rust");
    }
}

trait Nc {
    fn get_name(&self) ->&String;
}

struct Ter {
    name: String,
    age: u8,
}

struct Student {
    name: String,
    age: u8,
}

struct People<T,U> {
    ter: T,
    student: U,
}

impl Nc for Student{
    fn get_name(&self) -> &String {
        &self.name
    }
}

impl Hello for Student{}

impl Nc for Ter {
    fn get_name(&self) -> &String {
        &self.name
    }
}

impl Hello for Ter{}

impl <T:Hello + Nc,U:Hello + Nc> People<T,U> {
    fn new(stu:T,te:U) -> People<T,U> {
        People{
            ter:stu,
            student:te,
        }
    }

    fn run(&self) {
        println!("A: {} B: {}",self.student.get_name(),self.ter.get_name());
        self.student.hello();
        self.ter.hello();
    }
}

pub fn test5() {
    let s1 = Ter{
        name: String::from("ter"),
        age:18,
    };

    let s2 = Student{
        name: String::from("student"),
        age:18,
    };

    let s3 = People::new(s1,s2);
    s3.run();
}