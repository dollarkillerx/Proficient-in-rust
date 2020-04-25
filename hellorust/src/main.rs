use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("猜猜看!!!");
    let caiCai = rand::thread_rng().gen_range(0,100);
   loop {
       print!("输入你所猜所想: ");
       let mut input = String::new();

       io::stdin().read_line(&mut input).
           expect("蛤 你输入的啥子???");

       // to uint32
       let inputUint: u32 = match input.trim().parse() {
           Ok(num) => num,
           Err(_) => continue,
       };

       println!("you Input: {}",inputUint);

       match inputUint.cmp(&caiCai) {
           Ordering::Less => println!("Less"),
           Ordering::Equal => {
               println!("Equal");
               break;
           },
           Ordering::Greater => println!("Greater"),
       }

   }
}