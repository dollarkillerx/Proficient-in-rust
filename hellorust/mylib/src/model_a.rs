pub mod modA {
    #[derive(Debug)]
    pub struct A {
        pub number: i32,
        name: String,
    }

    impl A {
        pub fn new_a() -> A {
            A {
                number: 1,
                name: String::from("AaA"),
            }
        }

        pub fn print_a(&self) {
            println!("a name: {}",self.name);
        }
    }
}