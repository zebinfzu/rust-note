mod my; // 会寻找同文件夹下的 my.rs 或者 my/mod.rs

fn function() {
    println!("called `function()`");
}

fn main() {
    my::function();

    function();

    my::indirect_access();

    my::nested::function();
}
