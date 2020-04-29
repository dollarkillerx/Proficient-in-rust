// extern crate crypto;
use std::hash::Hash;
use std::collections::HashMap;
use mylib::factory::produce_refrigerator as b;
use mylib::model_a as ma;
use crypto::digest::Digest;
use crypto::sha3::Sha3;

fn main() {
    test1();
    test2();
    test3();
}

fn test3() {
    let mut hasher = Sha3::sha3_256();
    hasher.input_str("Hello World");
    println!("hash: {}",hasher.result_str())
}

fn test2() {
    println!();
    // mylib::factory::produce_refrigerator::produce_re();
    // produce_refrigerator::produce_re();
    b::produce_re();

    let aa = ma::modA::A::new_a();
    aa.print_a();
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
