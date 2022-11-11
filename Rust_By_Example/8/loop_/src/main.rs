#![allow(unused)]
fn main() {
    // loop 可以使用continue 和 break关键字，而且可以使用'label语法来指定要break，continue哪个循环

    'outer: loop {
        println!("Entered the outer loop");
        // continue 'outer;        
        'inner: loop {
            println!("Entered the inner loop");

            // 这只是中断内部的循环
            //break;

            // 这会中断外层循环
            break 'outer;
        }

        println!("This point will never be reached");
    }
    
    println!("Exited the outer loop");

    // loop返回值
    let mut counter = 0;

    let result = 'o: loop {
        counter += 1;

        if counter == 10 {
            break 'o counter * 2;
        }
    };

    assert_eq!(result, 20);


}
