use std::io;
use rand::Rng;

fn main() {
    println!("猜数字游戏");

    println!("请输入你猜测的数字");

    let result = rand::thread_rng().gen_range(1..=100);
    let mut guess = String::new();

    loop {
        guess.clear();

        io::stdin().read_line(&mut guess).expect("Faild to read line");
        let num: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        match num.cmp(&result) {
            std::cmp::Ordering::Less => println!("输入的值太小了"),
            std::cmp::Ordering::Equal => break,
            std::cmp::Ordering::Greater => println!("输入的值太大了"),
        }
    }

    println!("答对了");
}
