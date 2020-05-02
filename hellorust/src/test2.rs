pub fn test2() {
    let us = User{
      name: &String::from("aaaa"),
    };

    println!("user: {:#?}",us);
}

#[derive(Debug)]
struct User<'a> {
    name: &'a str
}