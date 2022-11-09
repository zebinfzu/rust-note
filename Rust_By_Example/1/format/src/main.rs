fn main() {
    // 整数类型可以指定格式化的方式{:x} {:X} {:o} {:b}
    {
        let foo: i64 = 3735928559;
        println!("{}", format!("0x{:x}", foo));
        println!("{}", format!("0o{:o}", foo));
        println!("{}", format!("0b{:b}", foo));
    }
    // {} 格式化的规则
    // {0:} :之前指第几个参数，不写自动推导
    // {:04} 指占4位(包含小数长度，如果整数+小数小于该指定，小数输出多少看.之后)，长度不足用0补全
    // {:.05} 指小数占5位，长度不足用0补全
    // {0:<05} < ^ > 指左中右对齐
    //
    {
        let foo: f64 = 123.1234;
        println!("{}", foo);
        println!("{:>08.3}", foo); // 8表示整数+小数部分共8位
        println!("{:^05}", foo);
        println!("{:>.1$}", foo, 3); // N$ 需要N参数usize值表示精度
        println!("{:.*}", 2, foo); // * 需要usize值作为精度
    }
    {
        println!("-----city-----");

        use std::fmt::{self, Display, Formatter};

        struct City {
            name: &'static str,
            // 纬度
            lat: f32,
            // 经度
            lon: f32,
        }

        impl Display for City {
            // `f` 是一个缓冲区（buffer），此方法必须将格式化后的字符串写入其中
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
                let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

                // `write!` 和 `format!` 类似，但它会将格式化后的字符串写入
                // 一个缓冲区（即第一个参数f）中。
                write!(
                    f,
                    "{}: {:.3}°{} {:.3}°{}",
                    self.name,
                    self.lat.abs(),
                    lat_c,
                    self.lon.abs(),
                    lon_c
                )
            }
        }

        for city in [
            City {
                name: "Dublin",
                lat: 53.347778,
                lon: -6.259722,
            },
            City {
                name: "Oslo",
                lat: 59.95,
                lon: 10.75,
            },
            City {
                name: "Vancouver",
                lat: 49.25,
                lon: -123.1,
            },
        ]
        .iter()
        {
            println!("{}", *city);
        }
    }
    {
        println!("---------color------");
        #[derive(Debug)]
        struct Color {
            red: u8,
            green: u8,
            blue: u8,
        }

        impl std::fmt::Display for Color {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "RGB ({0:03}, {1:03}, {2:03}) 0x{0:0x}{1:0x}{2:0x}", self.red, self.green, self.blue)
            }
        }

        for color in [
            Color {
                red: 128,
                green: 255,
                blue: 90,
            },
            Color {
                red: 0,
                green: 3,
                blue: 254,
            },
            Color {
                red: 0,
                green: 0,
                blue: 0,
            },
        ]
        .iter()
        {
            // 在添加了针对 fmt::Display 的实现后，请改用 {} 检验效果。
            println!("{}", *color)
        }
    }
}
