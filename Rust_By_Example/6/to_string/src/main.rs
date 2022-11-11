use std::i32;

fn main() {
    // 把任何类型转换成string，需要实现该类型的ToString特征，但其实实现Display就好了，Display实现自动提供ToString
    {
        use std::fmt::Display;

        struct Circle {
            radius: i32,
        }

        impl Display for Circle {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "Circle of radius {}", self.radius)
            }
        }
        let circle = Circle { radius: 6 };
        println!("{}", circle.to_string());
    }
    // ToString
    {
        struct Circle {
            radius: i32,
        }

        impl ToString for Circle {
            fn to_string(&self) -> String {
                format!("Circle of radius {:?}", self.radius)
            }
        }
    }
    // 解析字符串
    {
        // 只要目标类型实现了FromStr特征就可以通过parse把字符串转换成目标类型
        let parsed: i32 = "5".parse().unwrap();
        let turbo_parsed = "10".parse::<i32>().unwrap();

        let sum = parsed + turbo_parsed;
        println! {"Sum: {:?}", sum};

        struct Circle {
            radius: i32,
        }

        use std::str::FromStr;
        impl FromStr for Circle {
            type Err = ();
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let radius = s.parse::<i32>();
                match radius {
                    Ok(radius) => Ok(Circle {radius}),
                    _ => Err(())
                }
            }
        }

        let circle = "1".parse::<Circle>().unwrap();
        println!("{}", circle.radius)
    }
}
