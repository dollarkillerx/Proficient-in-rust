trait Agc {
    fn get_name(&self) -> &String;
}

trait PrintName {
    fn print_name(&self);
}

impl <T: Agc> PrintName for T {
    fn print_name(&self) {
        println!("pr: {}",self.get_name());
    }
}

struct User {
    name: String,
}

impl Agc for User {
    fn get_name(&self) -> &String {
        &self.name
    }
}

pub fn test6() {
    let u = User{
        name: String::from("Acd"),
    };

    u.print_name();
}