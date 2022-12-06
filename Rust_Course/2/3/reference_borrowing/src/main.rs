#[allow(unused)]
fn main() {
    // 1. 引用和借用是一个意思
    {
        let s = String::from("hello world");
        let s1 = &s; // 可以说s1引用了s的值也可以说s1借用了s的值
        println!("{}", s1);
    }
    // 2. 引用和解引用
    {
        let x = 5;
        let y = &x;

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }
    // 3. 不可变引用 一个值同时可以存在多个不可变引用
    // 4. 可变引用 一个值同时只能存在一个不可变引用
    {
        let mut x = 10;
        let (x1, x2) = (&x, &x);
        let x2 = &mut x; // 合法，但是编译器判断到此之前的不可变引用就不能使用了
        *x2 = 101;
    }
    // 5. 通过引用和编译器检查，rust禁止悬垂引用
    {
        // 编译器会报错没有生命周期标识
        // fn dangle() -> &String {
        //     let s = String::from("hello");
        //     &s
        // }
    }
    // 6. 总结规则就是，同时要么拥有一个或多个不可变引用或一个可变引用，并且引用必须总是有效的
}
