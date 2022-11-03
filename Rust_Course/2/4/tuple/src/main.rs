fn main() {
   // 元组：长度固定，类型，顺序也固定
   {
    let _tup = (500, 5.1, 1);
    // 解构语法
    let (_x, _y, _z) = _tup;
    // 访问元组.语法
    let _first = _tup.0;
    let _second = _tup.1;
    let _third = _tup.2;
   } 
}
