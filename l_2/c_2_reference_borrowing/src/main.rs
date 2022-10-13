fn main() {
    // 1. 引用和解引用，常规引用是一个指针类型
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // 2. 不可变引用，不允许修改值
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}", s1, len);

    // 3. 可变引用，允许修改值
    // 首先值本身必须是可变的
    let mut s = String::from("hello");
    // 其次必须是可变引用
    change(&mut s);
    // 同一个值的可变引用同时只能存在一个
    // 不可变引用和可变引用不能同时存在

    // 新版编译器引用的作用域到最后一次使用就会结束
}

fn calculate_length(s: &String) -> usize {
    s.len();
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
