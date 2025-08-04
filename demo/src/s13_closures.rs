#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

pub fn main() {
    println!("--- s13_closures ---");

    let f = |x| x + 1;
    let n = f(1);
    println!("{n}");
    let mut list = vec![1, 2, 3];
    println!("在定义闭包之前的：{:?}", list);

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("在调用闭包之后：{:?}", list);

    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    list.sort_by_key(|r| {
        r.height; // fake read
        r.width
    });

    dbg!(list);

    let double = |x| x * 2;
    let result = apply(double);
    println!("5 * 2 = {}", result); // 输出：5 * 2 = 10

    let nums = vec![1, 2, 3, 4, 5];
    let even = filter_vec(nums, |&x| x % 2 == 0);
    println!("{:?}", even); // 输出：[2, 4]

    let nums = vec![1, 2, 3];
    let squares = map_vec(nums, |x| x * x);
    println!("{:?}", squares); // 输出：[1, 4, 9]

    let mut count = 0;
    invoke_twice(|| {
        count += 1;
        println!("called, count = {}", count);
    });
    // 输出：
    // called, count = 1
    // called, count = 2
    let message = String::from("hello");
    consume_and_run(move || {
        // message 的所有权被移动到闭包中
        println!("{}", message);
    });
    // 再次使用 message 会编译错误
    let result = apply_twice(3, |x| x + 10);
    println!("{}", result); // 输出：23  （3 + 10 + 10）
}

// Fn()：不可变借用环境（可多次调用）
// FnMut()：可变借用环境（可多次调用）
// FnOnce()：获取所有权（仅能调用一次）

fn apply<F>(f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(5)
}

fn filter_vec<T, F>(v: Vec<T>, predicate: F) -> Vec<T>
where
    F: Fn(&T) -> bool,
    T: Clone,
{
    v.into_iter().filter(|item| predicate(item)).collect()
}

fn map_vec<T, U, F>(v: Vec<T>, mut mapper: F) -> Vec<U>
where
    F: FnMut(T) -> U,
{
    v.into_iter().map(|x| mapper(x)).collect()
}

fn invoke_twice<F>(mut action: F)
where
    F: FnMut(),
{
    action();
    action();
}

fn consume_and_run<F>(f: F)
where
    F: FnOnce(),
{
    f();
}

fn apply_twice<T, F>(value: T, f: F) -> T
where
    F: Fn(T) -> T,
{
    let first = f(value);
    f(first)
}
