trait Ac {
    fn print(&self);
}


// |  cannot infer type for type parameter `T` declared on the trait `Age`
// |                     this method call resolves to `std::option::Option<T>`

trait Age {
    type Item;
    fn get_age(&mut self) -> Option<Self::Item>;
}


impl Age for M1 {
    type Item = u32;
    fn get_age(&mut self) -> Option<Self::Item> {
        Some(212)
    }
}
//
// impl Age for M1 {
//     type Item = i32;
//     fn get_age(&mut self) -> Option<Self::Item> {
//         Some(self.id as i32)
//     }
// }

struct M1 {
    id: u32,
}

impl Ac for M1 {
    fn print(&self) {
        println!("hc");
    }
}

struct M2 {
    id: u32,
}
impl Ac for M2 {
    fn print(&self) {
        println!("{}",self.id);
    }
}

fn test_c(item: &impl Ac) {
    item.print();
}

pub fn test5() {
    let mut s1 = M1{id:2323};
    let mut s2 = M2{id:1212};

    test_c(&s1);
    test_c(&s2);


    println!();
    println!("{:?}",s1.get_age());


    // <s1 as Age<i32>>::get_age();
    println!("{:?}",s1.get_age());

    // tc();
}

pub trait Iterator1<T> {
    fn next(&mut self) -> Option<T>;
}

struct A {
    value: i32,
}

impl Iterator1<i32> for A {
    fn next(&mut self) -> Option<i32> {
        println!("in i32");
        if self.value > 3 {
            self.value += 1;
            Some(self.value)
        } else {
            None
        }
    }
}

impl Iterator1<String> for A {
    fn next(&mut self) -> Option<String> {
        println!("in string");
        if self.value > 3 {
            self.value += 1;
            Some(String::from("hello"))
        } else {
            None
        }
    }
}

fn tc() {
    let mut a = A{value: 3};
    //a.next();
    <A as Iterator1<i32>>::next(&mut a);  //完全限定语法，带上了具体的类型
    <A as Iterator1<String>>::next(&mut a);
    println!("Hello, world!");
}