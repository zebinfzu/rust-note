pub fn function() {
    println!("called `my::nested::function()`");
}

// 不可以被外界以mod形式引入直接访问
#[allow(dead_code)]
fn private_function() {
    println!("called `my::nested::private_function()`");
}