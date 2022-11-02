#![allow(unused)]
fn main() {
    // char Unicode字符，均是4字节编码
    {
        let c = 'z';
        let z = 'ℤ';
        let g = '国';
        let heart_eyed_cat = '😻';
    }
    // boolean
    {
        let t = true;
        let f: bool = false;
    }
    // unit
    {
        let unit = ();
    }

}
