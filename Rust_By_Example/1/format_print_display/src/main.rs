fn main() {
    // display特征需要手动实现
    {
        // 导入Display
        use std::fmt::Display;

        struct Structure(i32);

        impl Display for Structure {
            // 该特征要求实现fmt方法
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    }
    // 任何泛型容器都没有实现fmt::Display，但任何非泛型容器都可以实现fmt::Display
    {
        use std::fmt::Display;

        #[derive(Debug)]
        struct MinMax(i64, i64);

        // 实现MinMax的Display特征
        impl Display for MinMax {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "({}, {})", self.0, self.1)
            }
        }

        #[derive(Debug)]
        struct Point2D {
            x: f64,
            y: f64,
        }

        impl Display for Point2D {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "x: {}, y: {}", self.x, self.y)
            }
        }

        let minmax = MinMax(0, 14);

        println!("Compare structures:");
        println!("Display: {}", minmax);
        println!("Debug: {:?}", minmax);

        let big_range = MinMax(-300, 300);
        let small_range = MinMax(-3, 3);

        println!(
            "The big range is {big} and the small is {small}",
            small = small_range,
            big = big_range
        );

        let point = Point2D { x: 3.3, y: 7.2 };

        println!("Compare points:");
        println!("Display: {}", point);
        println!("Debug: {:?}", point);
    }
    {
        use std::fmt;
        #[derive(Debug)]
        struct Complex {
            real: f64,
            imag: f64,
        }

        impl fmt::Display for Complex {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{} + {}i", self.real, self.imag)
            }
        }

        let complex = Complex {
            real: 3.3,
            imag: 7.2,
        };
        println!("{:?}", complex);
        println!("{}", complex);
    }
}
