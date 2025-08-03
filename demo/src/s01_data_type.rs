pub fn main() {
    println!("--- s01_data_type ---");
    f1()
}

fn f1() {
    // 加法
    let sum: i32 = 5 + 10;

    // 减法
    let difference: f64 = 95.5 - 4.3;

    // 乘法
    let product = 4 * 30;

    // 除法
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // 结果为 0

    // 余数
    let reminder = 43 % 5;

    println!(
        "
        5 + 10 = {sum},
        95.5 - 4.3 = {difference}
        4 * 30 = {product}
        56.7 / 32.2 = {quotient}
        2 / 3 = {floored}
        43 % 5 = {reminder}"
    );

    // 元组 - 不同类型的多个值
    let tup = (500, 6.4, 1);
    
    let (x, y, z) = tup;
    let xx = tup.0;

    println!("{xx} {x} {y} {z}");

    // 数组 - 固定长度, 相同类型
    let a = [1, 2, 3, 4, 5];
    let a2: [i32; 5] = [-1, 0, 1, 2, 3];
    let a3: [i32; 5] = [2; 5];

    println!("a[0] = {}; a2[0] = {}", a[0], a2[0]);
    println!("a3[1] = {}; a3[4] = {}", a3[1], a3[4]);
}
