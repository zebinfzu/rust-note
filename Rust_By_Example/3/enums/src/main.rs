#![allow(dead_code)]
#![allow(unused)]
fn main() {
    enum WebEvent {
        // 一个 `enum` 可以是单元结构体（称为 `unit-like` 或 `unit`），
        PageLoad,
        PageUnload,
        // 或者一个元组结构体，
        KeyPress(char),
        Paste(String),
        // 或者一个普通的结构体。
        Click { x: i64, y: i64 },
    }

    // 此函数将一个 `WebEvent` enum 作为参数，无返回值。
    fn inspect(event: WebEvent) {
        match event {
            WebEvent::PageLoad => println!("page loaded"),
            WebEvent::PageUnload => println!("page unloaded"),
            // 从 `enum` 里解构出 `c`。
            WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
            WebEvent::Paste(s) => println!("pasted \"{}\".", s),
            // 把 `Click` 解构给 `x` and `y`。
            WebEvent::Click { x, y } => {
                println!("clicked at x={}, y={}.", x, y);
            }
        }
    }

    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` 从一个字符串切片中创建一个具有所有权的 `String`。
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    // 类型别名
    enum VeryVerboseEnumOfThingsToDoWithNumbers {
        Add,
        Subtract,
    }
    type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;
    let x = Operations::Add;

    // impl 块当中使用self就是别名
}
