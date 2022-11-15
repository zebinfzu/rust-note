mod inaccessible; // 寻找inacessilbe.rs
pub mod nested;

// 外部作用域引入可以直接访问,pub关键字
pub fn function() {
    println!("called `my::function()`");
}

fn private_function() {
    println!("called `my::private_function()`");
}

pub fn indirect_access() {
    print!("called `my::indirect_access()`, that\n> ");

    private_function();
}