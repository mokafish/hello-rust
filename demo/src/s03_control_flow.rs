pub fn main() {
    println!("--- s03_control_flow ---");
    
    let number = 6;

    if number % 4 == 0 {
        println!("数字可被 4 整除");
    } else if number % 3 == 0 {
        println!("数字可被 3 整除");
    } else if number % 2 == 0 {
        println!("数字可被 2 整除");
    } else {
        println!("数字不可被 4、3 或 2 整除");
    }

    // 在 let 语句中使用 if
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("number 的值为：{number}");

    // 自循环返回值
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("结果为：{result}");

    // while
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("点火！！");

    // 遍历
    let a = [10, 20, 30, 40, 50];

    for el in a {
        println!("the value is: {el}");
    }

    for number in (1..4).rev() {
        println! ("{}!", number);
    }

    println! ("LIFTOFF!!");
}
