pub fn main() {
    println!("--- s04_ownership ---");
    ownership_demo();
    ownership_demo2();
}

fn ownership_demo() {
    let s = String::from("hello");  // s 进到作用域

    takes_ownership(s);             // 变量 s 的值迁移到这个函数中......
    // ......进而在这里不再有效

    let x = 5;                      // x 进到作用域

    makes_copy(x);                  // x 将迁移到这个函数中，
    // 但由于 i32 实现了 `Copy` 特质，因此
    // 后面在使用变量 x 没有问题
    println! ("x = {x}");
}   // 到这里，x 超出作用域，接着是 s。但由于 s 的值已被迁移，因此
// 不会有特别的事情发生。

fn takes_ownership(some_string: String) {   // some_string 进到作用域
    println! ("{}", some_string);
}   // 这里，some_string 超出作用域，而 `drop` 方法会被调用。退回的
// 内存被释放。

fn makes_copy(some_integer: i32) {  // some_integer 进到作用域
    println! ("{}", some_integer);
}   // 这里，some_integer 超出作用域。没有特别事情发生。

fn ownership_demo2() {
    let s1 = gives_ownership(); // gives_ownership 会将其返回值
    // 迁移到 s1 中

    let s2 = String::from("hello"); // s2 进入作用域

    let s3 = takes_and_gives_bake(s2); // s2 会被迁移到 takes_and_gives_back
    // 中，其又会将他的返回值迁移到 s3 中

    println!("s1: {s1}, s3: {s3}");
} // 这里，s3 会超出作用域而被丢弃。变量 s2 已被迁移，因此什么也不会发生。而
// s1 则超出作用域而被丢弃。

fn gives_ownership() -> String {
    // gives_ownership 将把他的返回值，迁移
    // 到调用他的函数中（即 main 函数）

    let some_string = String::from("归你了"); // some_string 进到作用域

    some_string // some_string 被返回并
    // 迁出到调用函数
}

// 此函数取个 String 并会返回一个 String
fn takes_and_gives_bake(a_string: String) -> String {
    // a_string 进入作用域
    a_string // a_string 被返回，并会迁出到调用函数
}
