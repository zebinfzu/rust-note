#![allow(unused)]
fn main() {
    // 枚举
    {
        // 定义一个枚举类型
        #[derive(Debug)]
        enum PokerSuit {
            Clubs,
            Spades,
            Diamonds,
            Hearts,
        }
        // 创建枚举类型的实例
        let heart = PokerSuit::Hearts;
        let diamond = PokerSuit::Diamonds;
        fn print_suit(card: PokerSuit) {
            println!("{:?}", card);
        }
    }
    // 枚举成员关联到值
    {
        enum PokerCard {
            Clubs(u8),
            Spades(u8),
            Diamonds(char),
            Hearts(char),
        }
        #[derive(Debug)]
        enum Message {
            Quit,
            Move { x: i32, y: i32 }, // 关联到struct
            Write(String),
            ChangeColor(i32, i32, i32), // 关联到元组结构体
            Point((i32, i32, i32)),     // 关联到元组类型
        }
        let point = Message::Point((0, 255, 0));
        println!("{:?}", point);
        let color = Message::ChangeColor(100, 121, 0);
        println!("{:?}", color);
    }
    // option枚举
    {
        let some_number = Option::Some(1);
        // Option太过于有用以至于在prelude中就被导入，可以直接使用枚举成员Some()和None
        let some_string = Some("a string");
        let absent_number: Option<i32> = None;
        // Option枚举的常用方法
        let x: Option<u32> = Some(2);
        assert_eq!(x.is_some(), true);
        let x: Option<u32> = None;
        assert_eq!(x.is_none(), true);
        // 取出Some里面的值 unwrap -> None的时候panic
        x.unwrap_or(3);
        // 通常会配合match使用
    }
}

