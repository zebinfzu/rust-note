# primitive type

1. number value
   1. integer (i8, i16, i32, i64, isize)
   2. unsigned integer (u8, u16, u32, u64, usize)
   3. float (f32, f64)
2. string
   1. string literal
   2. &str
3. boolean
4. char unicode -> 4byte
5. unit ()

```rs
let guess = "42".parse().expect("Not a number!"); // error 不知道要推导成什么类型
let guess: i32 = "42".parse().expect("Not a number!");
let guess = "42".parse::<i32>().expect("Not a number!");
```