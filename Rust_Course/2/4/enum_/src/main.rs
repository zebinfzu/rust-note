#[allow(unused)]
fn main() {
    // 1. Rust枚举
    {
        // 枚举类型 enum关键字 + 类型名，枚举值 -> 括号里面的字段
        #[derive(Debug)]
        enum PokerSuit {
            Clubs,
            Spades,
            Diamonds,
            Hearts,
        }
        // 枚举值
        let heart = PokerSuit::Hearts;
        let diamond = PokerSuit::Diamonds;
        fn print_suit(card: PokerSuit) {
            println!("{:?}", card);
        }
        print_suit(heart);
        print_suit(diamond);
    }
    // 2. Rust枚举值可以绑定值
    {
        // 枚举值字段本身不带有值，那么要表示所有扑克牌就需要使用structure
        {
            enum PokerSuit {
                Clubs,
                Spades,
                Diamonds,
                Hearts,
            }

            struct PokerCard {
                suit: PokerSuit,
                value: u8,
            }
            // 实例化一张红心1
            let h1 = PokerCard {
                suit: PokerSuit::Hearts,
                value: 1
            };
        }
        // 枚举值字段绑定到值，简化扑克牌的表示
        {
            enum PokerCard {
                Clubs(u8),
                Spades(u8),
                Diamonds(u8),
                Hearts(u8),
            }
            let h1 = PokerCard::Hearts(1); // 实例化一张红心1
        }
    }
    // 3. 枚举值可以绑定的值是任意数据类型
    {
        enum Message {
            Quit, // 枚举值不关联值
            Move { x: i32, y: i32 }, // 枚举值绑定匿名结构体
            Write(String), // 枚举值绑定一个String
            ChangeColor(i32, i32, i32), // 枚举值绑定3个i32
        }
    }
    // 4. Option枚举用来处理空值
    {
        // 标准库直接提供
        enum Option<T> {
            Some(T),
            None
        }
    }
}
