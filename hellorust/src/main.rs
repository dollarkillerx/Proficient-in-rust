use std::hash::Hash;
use std::collections::HashMap;

fn main() {
    test1();
}

mod cat {
    pub fn hello_world(tag: &String) {
        print(&format!("Hello {}",tag));
    }

    fn print(v: &String) {
        println!("{}",v);
    }

    pub mod xi_xi {
        pub fn hello_world(tag: &String) {
            super::print(tag); // 调用父级别
        }
    }
}

fn test1() {
    crate::cat::hello_world(&String::from("PPC"));  // 相对路径
    cat::hello_world(&String::from("HhH"));         // 绝对路径

    cat::xi_xi::hello_world(&String::from("那 那 那"));
}
