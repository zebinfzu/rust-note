#![allow(unused)]
fn main() {
    // 1. immutable
    {
        let _x = 5;
        // _x = 6; // false
    }
    // 2. mutable
    {
        let mut x = 5;
        x = 6;
    }
    // 3. const 可以在任意作用域声明，并且常量必须
    {
        const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    }
    // 4. shadow
    {
        let x = 5;
        {
            let x =  4;
            println!("x: {}", x);
        }
        println!("x: {}", x);
    }
}
