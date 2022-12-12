#[allow(unused)]
fn main() {
    // 1. rust类型转换之as
    {
        let a = i8::MAX;
        println!("{}", a);
        let a = 300_i32 as i8; // 44
                               // 因为300超出了i8的表示范围，rust通过as做类型转换的时候表现为把高位截掉
        println!("{}", a);
    }
    // 2. 将内存地址转换成指针
    {
        let mut values: [i32; 2] = [1, 2];
        let p1: *mut i32 = values.as_mut_ptr();
        let first_address = p1 as usize; // 将p1内存地址转换位一个整数
        let second_address = first_address + 4; // 4 == std::mem::size_of::<i32>()，i32类型占用4个字节，因此将内存地址 + 4
        let p2 = second_address as *mut i32; // 访问该地址指向的下一个整数p2
        unsafe {
            *p2 += 1;
        }
        println!("{:?}", values);
    }

    // 3. TryInto 转换
    {
        let a: u8 = 10;
        let b: u16 = 1500;
        // rust中要使用特征方法就必须引入该特征，这里try_into可以使用是因为prelude引入了std::convert::TryInto
        // try_into会返回一个Result<Type, Err>
        let b_: u8 = match b.try_into() {
            Ok(b1) => b1,
            Err(e) => {
                println!("{:?}", e.to_string());
                0
            }
        };
        if a < b_ {
            println!("Ten is less than one hundred.");
        }
    }

    // 4. as 和 try_into 只能用在数值类型上面

    // 5. 通用类型转换
    {
        // 一种结构体转换为另外一种结构体的笨办法
        struct Foo {
            x: u32,
            y: u16,
        }

        struct Bar {
            a: u32,
            b: u16,
        }

        fn reinterpret(foo: Foo) -> Bar {
            let Foo { x, y } = foo;
            Bar { a: x, b: y }
        }

        // 在某些情况下，类型是可以进行隐式强制转换的，虽然这些转换弱化了 Rust 的类型系统，但是它们的存在是为了让 Rust 在大多数场景可以工作(说白了，帮助用户省事)，而不是报各种类型上的编译错误

        // 在匹配特征时，不会做任何强制转换(除了方法)。一个类型 T 可以强制转换为 U，不代表 impl T 可以强制转换为 impl U
        // 如.操作符和函数调用，都会做一些强制类型转换
    }
}
