// 结构体
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    // rust当中，char是Unicode编码，占4个字节，但字符串String是UTF-8编码，占(1-4)字节
    // &str 字符串字面量，是切片
    let my_name = "Pascal";

    // 切片Slice
    let s = String::from("hello world!");
    let hello = &s[0..5];
    let world = &s[6..11];
    // 从头获取切片
    let slice = &s[..2]; // [0, 2)
                         // 获取到结尾
    let slice = &s[4..]; // [4, length - 1)
                         // 截取完整的切片
    let slice = &s[..];
    // 字符串切片的标识类型就算&str

    let a = [1, 2, 3, 4, 5];
    // 该数组的切片类型是&[i32]
    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);

    // String -> &str 取引用即可
    // &str -> String String::from("字符串字面量")
    // String的底层储存格式其实是[u8]，因此能使用index索引

    // 因此字符串切片是危险的操作，因为无法保证索引的字节正好落在字符的边界上
    let hello = "中国人";

    let s = &hello[0..2]; // 导致程序崩溃，因为切片2个字节的长度，但中文的utf-8编码占据4个字节

    // 操作字符串的方法
    // 1. push
    // 2. insert(index, char | &str)
    // 3. replace，适用于String和&str，该方法返回一个新的字符串

    // 元组
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // 适用匹配解构元组
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    // 用.来访问元组
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    // 结构体
    // 初始化的时候所有字段都必须初始化
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    // rust不支持设置结构体字段的可变性，只能设置实例的可变性
    // 结构体更新语法，..语法表明凡是我们没有显式声明的字段，全部从 user1 中自动获取，必须在尾部使用
    // 注意更新语法和=是类似的，也就是说user1.username拥有的String值的所有权会被转移给user2.username
    // 之后user1不能使用username字段，但任然可以使用其他字段
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    // 元组结构体，结构体必须有名称，但结构体里面的字段不一定要有名称
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // 单元结构体，如果你定义一个类型，但是不关心该类型的内容, 只关心它的行为时，就可以使用 单元结构体
    struct AlwaysEqual;

    let subject = AlwaysEqual;

    // 我们不关心 AlwaysEqual 的字段数据，只关心它的行为，因此将它声明为单元结构体，然后再为它实现某个特征
    impl SomeTrait for AlwaysEqual {}

    // 在结构体里面使用引用类型必须加上生命周期，否则会报错

    // 使用#[derive(Debug)]标记的结构体才允许使用println!("{:?}", s); 的方式对其进行打印输出
    struct Rectangle {
        width: u32;
        height: u32;
    }
    let rect = Rectangle {
        width: 30,
        height: 50
    }
    println!("{}", rect); // 报错，没有实现display特征
    println!("{:?}", rect); // 报错，没有实现Debug特征
    // rust默认不会实现debug特征，要么手动实现要么使用标记#[derive(Debug)]

    // 枚举
    //  枚举类型是一个类型，它会包含所有可能的枚举成员, 而枚举值是该类型中的具体某个成员的实例
    enum PokerSuit {
        Clubs,
        Spades,
        Diamonds,
        Hearts,
      }
    let heart = PokerSuit::Hearts;
    let diamond = PokerSuit::Diamonds;

    // 数组array定长，vector动态长度
    // rust中所谓数组是指array，长度固定，存储在栈上
    let a = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    // 为数组声明类型[类型; 长度]
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // 初始化一个值重复出现多次的数组[初始值; 长度]
    let a = [3; 5]; // [3, 3, 3, 3, 3]
}


fn print_suit(card: PokerSuit) {
    println!("{:?}",card);
}