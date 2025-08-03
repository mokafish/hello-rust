pub fn main() {
    println!("--- s02_function ---");
    
    let x = plus_one(5);

    println! ("x 的值为：{x}");
    println! ("fib(5) = {}", fibonacci(5));
    println! ("fib(10) = {}", fibonacci(10));
    println! ("fib(20) = {}", fibonacci(20));
}

fn plus_one(x: i32) -> i32 {
     x + 1
}

fn fibonacci(n: u64) -> u64 {
    if n == 0 || n == 1 {
        return n;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}