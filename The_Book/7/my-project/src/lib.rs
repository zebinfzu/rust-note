// lib.rs 说明这是一个和包名同名的库crate的根

// 在crate根文件中可以声明一个新模块
// 声明方式有三种
// 1. 内联：在mod name 之后跟着一个花括号而不是分号
mod inline_mod {
    // mod内的字段对外部默认都是私有的，外部要使用的内容需要使用pub关键字
    pub fn console() -> String {
        String::from("inline mod")
    }
}


// 2. 寻找文件garden.rs
// 3. 寻找文件garden/mod.rs
mod garden;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn inline_mod_test() {
        assert_eq!("inline mod", inline_mod::console().as_str());
    }
}
