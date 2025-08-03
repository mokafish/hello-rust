// 在任何时候，咱们都可以有着一个可变引用，或任意数量的不可变引用；
// 引用必须始终有效。

pub fn main() {
    println!("--- s05_references ---");

    let s1 = String::from("hello");

    let length = calculate_length(&s1);

    println!("字符串 '{}' 的长度为：{}", s1, length);

    let mut s = String::from("hello");

    change(&mut s);
    println!("可变引用: {s}")
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}
