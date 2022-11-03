#![allow(unused)]
fn main() {
    // 引用和解引用
    {
        let x = 5;
        // 引用
        let y = &x;
        assert_eq!(5, x);
        // 解引用
        assert_eq!(5, *y);
    }
    // 不可能引用和可变引用
    // 不能同时有两种引用
    // 可以同时有多个不可变引用，不可以同时又多个可变引用
    {
        let x = 1;
        let y = &x;
        let z = &x;
        let mut a = 5;
        // 可变引用
        let b = &mut a;
    }
    // 借用的规则，必须保证借用的元素生命周期大于等于当前元素
}
