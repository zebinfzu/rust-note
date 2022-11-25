// panic默认会展开
// 如果想讲panic展开换成panic时中断，需要在cargo.toml文件的[profile]中设置panic="abort"
// 命令行输出打印的时候可以添加上RUST_BACKTRACE=1来获取backtrace打印更多信息
fn main() {
   panic!("quit")
}
