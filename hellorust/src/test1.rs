pub fn test1() {
    let s1 = String::from("abcs");
    let s2 = String::from("ab");

    let c = str_bj(s1.as_str(),s2.as_str());
    println!("c: {}",c);
}

fn str_bj<'a>(x:&'a str,y:&'a str) -> &'a str {
    if x.len() > y.len() {
        return x
    }
    y
}
