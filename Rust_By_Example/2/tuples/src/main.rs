fn main() {
    // 元组作为函数的参数和返回值
    {
        fn _reverse(pair: (i32, bool)) -> (bool, i32) {
            let (integer, boolean) = pair;
            (boolean, integer)
        }
    }
    // 练习
    {
        #[derive(Debug)]
        struct Matrix(f32, f32, f32, f32);

        let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
        println!("{:?}", matrix);
        //1. fmt::Display trait
        impl std::fmt::Display for Matrix {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "( {} {} )\n( {} {} )", self.0, self.1, self.2, self.3)
            }
        }
        println!("{}", matrix);
        // 2. 以 reverse 函数作为样板，写一个 transpose 函数，它可以接受一个 Matrix 作为参数，并返回一个右上 - 左下对角线上的两元素交换后的 Matrix
        fn transpose(matrix: &Matrix) -> Matrix {
            Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
        }
        println!("Matrix:\n{}", matrix);
        println!("Transpose:\n{}", transpose(&matrix));
    }
}
