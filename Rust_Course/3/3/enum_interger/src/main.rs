#[allow(unused)]
fn main() {
    // 1. rust把枚举转换成整数很容易，反过来就很难
    // c 里面不管安全，很容易做到枚举-整数双向转换
    {
        enum MyEnum {
            A = 1,
            B,
            C,
        }
        // 枚举转整数，as即可
        let x = MyEnum::C as i32;
        // 整数转枚举，失败
        // 直接报错类型不匹配
        // match x {
        //     MyEnum::A => {}
        //     MyEnum::B => {}
        //     MyEnum::C => {}
        //     _ => {}
        // }
    }

    // 2. 使用第三方库解决这个问题num-traits num-derives
    {
        use num_derive::FromPrimitive;
        use num_traits::FromPrimitive;

        #[derive(FromPrimitive)]
        enum MyEnum {
            A = 1,
            B,
            C,
        }

        let x = 2;

        match FromPrimitive::from_i32(x) {
            Some(MyEnum::A) => println!("Got A"),
            Some(MyEnum::B) => println!("Got B"),
            Some(MyEnum::C) => println!("Got C"),
            None => println!("Couldn't convert {}", x),
        }
    }

    // 3. 使用TryFrom宏解决整数转枚举
    {
        use std::convert::TryFrom;
        enum MyEnum {
            A = 1,
            B,
            C,
        }
        impl TryFrom<i32> for MyEnum {
            type Error = ();
            fn try_from(v: i32) -> Result<Self, Self::Error> {
                match v {
                    x if x == MyEnum::A as i32 => Ok(MyEnum::A),
                    x if x == MyEnum::B as i32 => Ok(MyEnum::B),
                    x if x == MyEnum::C as i32 => Ok(MyEnum::C),
                    _ => Err(()),
                }
            }
        }

        let x = MyEnum::C as i32;

        match x.try_into() {
            Ok(MyEnum::A) => println!("a"),
            Ok(MyEnum::B) => println!("b"),
            Ok(MyEnum::C) => println!("c"),
            Err(_) => eprintln!("unknown number"),
        }
    }

    // 4. 使用std::mem::transmute
    // 这个很危险，但确实有使用场景
    {
        // 使用#[repr]控制底层类型的大小
        #[repr(i32)]
        enum MyEnum {
            A = 1,
            B,
            C,
        }
        let x = MyEnum::C;
        let y = x as i32;
        // unsafe块直接转类型
        let z: MyEnum = unsafe { std::mem::transmute(y) };
    }
}
