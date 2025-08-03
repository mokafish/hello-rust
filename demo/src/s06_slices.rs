pub fn main() {
    println!("--- s06_slices ---");
    let s = String::from("The quick brown fox jumps over the lazy dog.");

    // `first_word` 会在 String 的切片上工作，不管是部分还是整个 String
    let word = first_word(&s[0..6]);
    println!("The word is {}", word);

    let word = first_word(&s[..]);
    println!("The word is {}", word);

    // `first_word` 还对 String 的引用有效，这与 String 的整个切片等价
    let word = first_word(&s);
    println!("The word is {}", word);

    let s_string_literal = "hello word";

    // `first_word` 在字符串字面值上有效，不论部分还是整体
    let word = first_word(&s_string_literal[0..6]);
    println!("The word is {}", word);

    let word = first_word(&s_string_literal[..]);
    println!("The word is {}", word);

    // 由于字符串字面值已经 *是* 字符串切片，
    // 因此无需切片语法，这也会工作。
    let word = first_word(s_string_literal);
    println!("The word is {}", word);


    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq! (slice, &[2, 3]);
    println!("数组切片: {} {}", slice[0],slice[1]);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
