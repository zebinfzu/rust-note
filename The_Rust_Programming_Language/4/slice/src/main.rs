#![allow(unused)]
// slice 允许你引用集合中一段连续的元素序列，而不用引用整个集合。slice 是一类引用，所以它没有所有权。
fn main() {
    {
        let mut s = String::from("hello world");

        let word = first_word(&s); // word 的值为 5

        s.clear(); // 这清空了字符串，使其等于 ""

        // word 在此处的值仍然是 5，
        // 但是没有更多的字符串让我们可以有效地应用数值 5。word 的值现在完全无效！
        // word 和 s 本质上没有任何联系，可能获取到word的值之后s的内容就改变了
    }
    {
        // 字符串slice
        let s = String::from("hello world");
        // s在栈上的空间: ptr 指向堆地址 len capacity
        let hello = &s[0..5];
        let world = &s[6..11];
        // world栈上的空间: ptr 指向堆地址 len
    }
    {
        // 字符串字面值就是不可变引用，slice
        let s: &str = "tttt";
    }
    {
        let my_string = String::from("hello world");

        // `first_word` 适用于 `String`（的 slice），整体或全部
        let word = _first_word(&my_string[0..6]);
        let word = _first_word(&my_string[..]);
        // `first_word` 也适用于 `String` 的引用，
        // 这等价于整个 `String` 的 slice
        let word = _first_word(&my_string);

        let my_string_literal = "hello world";

        // `first_word` 适用于字符串字面值，整体或全部
        let word = _first_word(&my_string_literal[0..6]);
        let word = _first_word(&my_string_literal[..]);

        // 因为字符串字面值已经 **是** 字符串 slice 了，
        // 这也是适用的，无需 slice 语法！
        let word = _first_word(my_string_literal);
    }
}

// 不使用slice
fn first_word(s: &str) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// 返回字符串的slice
fn _first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
