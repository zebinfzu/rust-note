#[allow(unused)]
fn main() {
    // 1. rust的数组是基本类型，固定长度(这个固定指的是编译期就知道)
    {
        // rust数组存在栈上
        let months = [
            "January",
            "February",
            "March",
            "April",
            "May",
            "June",
            "July",
            "August",
            "September",
            "October",
            "November",
            "December",
        ];
    }
    // 2. 标识数据类型
    {
        let a: [i32; 5] = [1, 2, 3, 4, 5];
    }
    // 3. 申明数值一样的数组
    {
        let a = [3; 5];
    }
    // 4. 数组通过[]访问
    // 5. 越界访问将会导致panic
    // 6. 数组元素必须实现了Copy特征，不然编译错误
    // 7. 数组切片
    {
        let a: [i32; 5] = [1, 2, 3, 4, 5];

        let slice: &[i32] = &a[1..3];

        assert_eq!(slice, &[2, 3]);
    }
}
