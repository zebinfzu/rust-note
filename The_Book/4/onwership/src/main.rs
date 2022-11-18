fn main() {
    // 1. each value in rust has a variable that's called its owner
    // 2. there can only be one owner at a time
    // 3. when the owner goes out of scope, the value will be dropped.

    // primitive type -> integer float boolean char -> has Copy trait
    // copy when =
    {
        let x = 5;
        let y = 5;
        let _z = x + y;
    }

    // 复杂类型=发生的是移动 move,所有权会被移动
    {
        let s1 = String::from("test");
        let s2 = s1;
        // println!("{}", s1); 报错，在所有权移动之后使用了s1
        println!("{}", s2);
    } // s2 drop
    
}
