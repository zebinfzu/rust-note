struct Struct {
    e: i32,
}
fn main() {
    // 1. rust默认immutable，需要手动加mut才允许可变变量
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    // 2. rust会警告未使用的变量，使用_开头的变量未使用会被忽略
    let _x = 5;
    let y = 10;

    // 3. let用于解构对象
    let (a, mut b): (bool, bool) = (true, false);
    println!("a = {:?}, b = {:?}", a, b);

    b = true;
    assert_eq!(a, b);

    // 4. rust里面声明变量可以先不绑定内存对象
    let (a_1, a_2, a_3, a_4, e);
    (a_1, a_2) = (1, 2);
    // _ 代表匹配一个值，但是我们不关心具体的值是什么，因此没有使用一个变量名而是使用了 _
    [a_3, .., a_4, _] = [1, 2, 3, 4, 5];
    Struct { e, .. } = Struct { e: 5 };
    assert_eq!([1, 2, 1, 4, 5], [a_1, a_2, a_3, a_4, e]);

    // 5. 常量使用const，而且不允许使用mut，编译完就是确定值，变量名通常使用大写
    const MAX_POINTS: u32 = 100_000;

    // 6. 变量遮蔽
    // 这是创造了新的内存对象，只是名字一样，而不是使用mut的变量修改原来内存地址的值
    let x = 5;
    // 在main函数的作用域内对之前的x进行遮蔽
    let x = x + 1;

    {
        // 在当前的花括号作用域内，对之前的x进行遮蔽
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);

    // 编译器允许
    let spaces = "    "; // 字符串类型
    let spaces = spaces.len(); // usize类型

    // 编译器不允许
    let mut spaces = "    ";
    spaces = spaces.len(); // 赋值不同类型的值，编译器不允许将usize赋值到&str
}
