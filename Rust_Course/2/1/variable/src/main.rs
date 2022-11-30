fn main() {
    // 1. rust通过let关键字定义变量
    // 2. rust编译器默认开启严格的变量检查，没使用的变量会抛出警告
    // 3. 不想被警告可以使用_开头命名变量或者使用宏#[allow(unused_variables)]
    {
        let _x = 5;
        #[allow(unused_variables)]
        let y = 10;
    }

    // 4. 变量可变性，rust变量默认不可变，一旦初始化就不允许修改
    // 5. mut关键字用于申明一个可变的变量
    {
        let mut x = 5;
        x += 10;
    }
    //  6. 变量shadow，可以重复命名同名的变量，新变量将在初始化后shadow之前的同名变量
    {
        let x = 5;
        let x = x + 10;
        println!("{}", x);
    }
    // 7. 变量解构，let表达式不仅可以用于变量绑定，也可以用于变量的解构
    {
        let (_a, mut _b): (bool, bool) = (true, false);
    }
    // 8. 解构式赋值，在rust的1.59版本之后可以使用元组、切片、结构体的解构式赋值
    #[allow(unused)]
    {
        let (a, b, c, d, e);
        (a, b) = (1, 2);
        [c, .., d, _] = [1, 2, 3, 4, 5]; // .. 表示序列，忽略中间部分，_表示一个占位符，表示最后一个不用
        struct Struct {e: i32};
        Struct {e, ..} = Struct {e: 5};
    }
    // 9. 常量自始至终不可变，使用const申明并且必须用大写字母+_，且需要注明类型
    {
        const MAX_POINTS: u32 = 100_000;
    }
}
