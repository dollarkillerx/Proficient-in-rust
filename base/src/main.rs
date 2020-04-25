use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Guess the number!!!");
    // 获取随机数
    let secret_number = rand::thread_rng().
        gen_range(1,101);
    print!("Please input you guess: ");

    loop {  // loop 无限循环
        // mut 可变的
        let mut guess = String::new();
        // 获取用户输入
        io::stdin().read_line(&mut guess).
            expect("Failed to read line");

        // 转换
            // trim()   去除空白字符串  用户输出最后会有一个\n为结束符
            // 捕获用户错误
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_)=> continue,
        };

        println!("You guessed: {}",guess);

        match guess.cmp(&secret_number)  {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!!!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}

fn test1() {
    println!("Guess the number!");

    // 获取随机数
    let secret_number = rand::thread_rng().gen_range(1,101);
    println!("The secret number is :{}",secret_number);
    println!("Please input your guess: ");

    // mut 可变的
    let mut guess = String::new();

    io::stdin().read_line(&mut guess).
        expect("Failed to read line");

    println!("You guessed: {}",guess);
}

fn test2() {
    println!("Guess the number!!!");
    // 获取随机数
    let secret_number = rand::thread_rng().
        gen_range(1,101);
    print!("Please input you guess: ");

    loop {  // loop 无限循环
        // mut 可变的
        let mut guess = String::new();
        // 获取用户输入
        io::stdin().read_line(&mut guess).
            expect("Failed to read line");

        // 转换
        // trim()   去除空白字符串  用户输出最后会有一个\n为结束符
        let guess: u32 = guess.trim().parse().
            expect("Please type a number!!!");

        println!("You guessed: {}",guess);

        match guess.cmp(&secret_number)  {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!!!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}