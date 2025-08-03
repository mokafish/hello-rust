use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("--- 猜数游戏 ---");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println! ("秘密数字为: {secret_number}");
    let mut attempts = 0;
    loop {
        println!("请输入你的猜数:");

        let mut guess: String = String::new();

        io::stdin().read_line(&mut guess).expect("读取行失败......");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("请输入一个数字！");
                continue;
            }
        };

        attempts += 1;

        println!("你猜的数为：{guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("小了！"),
            Ordering::Greater => println!("大了！"),
            Ordering::Equal => {
                println!("对了！ 你猜了 {attempts} 次。");
                break;
            }
        }
    }
}
