fn main() {
    // 1. rust variable is must know the type at compile time.
    {
        let _guess: u32 = "42".parse().expect("Not a number!");
    }
    // scalar types
    // represent a single value
    // integers: i8 i16 i32(default) i64 i128 isize u8 u16 u32 u64 u128 usize
    {
        let _num_1 = 98_222;
        let _num_2: u8 = 0xf;
        let _num_3 = 0o77;
        let _num_4 = 0b1111_0000;
        // byte u8 only
        let _num_5 = b'A';
    }
    // 整数溢出，debug模式下会panic，release模式下不会检查，默认按照c一样的补码循环的方式处理溢出

    // float: f32 f64(default)
    {
        let _num_1 = 0.0;
        let _num_2: f32 = 0.1;
    }
    // 2. rust所有数字类型i,f都支持加减乘除和取模，整数除法向下取整
    {
        let remainder = 43.1 % 3.4;
        println!("{}", remainder);
    }

    // boolean true false

    // char unicode 字符，都是4个byte U+0000~U+D7FF 和 U+E000~10FFFF 均是合法范围

    // compound types
    // 3. rust have two primitive compound types: tuples and arrays
    {
        let _tup = (500, 6.4, 1);
        // 解构赋值
        let (_x, _y, _z) = _tup;
        // 使用 . 访问
        let x: (i32, f64, u8) = (1000, 0.5, 0xff);
        println!("{} {} {}", x.0, x.1, x.2);
    }
    // 4. 元组空会当作一种特殊的类型 unit 单元类型，该类型只有一个值()，如果表达式不返回值，就默认返回单元值
    {
        let _f: () = {
            let _a = 1; // 语句没有返回值，因此{}表达式返回()
        };
    }

    // 5. 数组和元组的区别在于数组的元素必须是同一类型的值
    {
        let _a = [1, 2, 3, 4, 5];
        let _months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
        // 快速生成所有值一样的数组
        let _a = [3;5];
        // rust 的数组是栈上已知大小的单元块，越界访问会panic
    }

}
